//! `archr` — headless ArchiMate 3.2 engine CLI.
//!
//! Subcommands: validate, generate, parse, diff, query, project.

use archr_core::io::yaml::SchemaError;
use archr_core::{
    dependencies, diff::ModelDiffAnalyzer, impact, io::xml::XmlError, io::{xml, yaml},
    layout::LayoutResolver, model_to_yaml, orphans, path, project, project_view, validate::validate_model,
    Model, QueryError, RefId, Viewpoint,
};
use clap::{Parser, Subcommand};
use std::collections::HashMap;
use std::fs;
use std::process::ExitCode;

#[derive(Parser)]
#[command(
    name = "archr",
    version = env!("CARGO_PKG_VERSION"),
    about = "Headless ArchiMate 3.2 engine"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate an ArchiMate model (YAML input).
    Validate {
        /// Input YAML file path.
        #[arg(long)]
        input: String,
    },
    /// Generate Open Exchange XML from a YAML model.
    Generate {
        /// Input YAML file path.
        #[arg(long)]
        input: String,
        /// Output XML (.archimate) file path.
        #[arg(long)]
        output: String,
    },
    /// Parse Open Exchange XML into YAML.
    Parse {
        /// Input XML (.archimate) file path.
        #[arg(long)]
        input: String,
        /// Output YAML file path.
        #[arg(long)]
        output: String,
    },
    /// Diff an existing XML model against a new YAML model.
    Diff {
        /// Existing model (XML).
        #[arg(long)]
        old: String,
        /// New model (YAML).
        #[arg(long)]
        new: String,
    },
    /// Query the model graph (impact, dependencies, paths, orphans).
    Query {
        /// Input YAML file path.
        #[arg(long)]
        input: String,
        #[command(subcommand)]
        query: QueryKind,
    },
    /// Project the model onto a viewpoint or saved view, as YAML.
    View {
        /// Input YAML file path.
        #[arg(long)]
        input: String,
        /// Viewpoint name (e.g. "Business", "Motivation") — exactly one of
        /// --viewpoint/--view.
        #[arg(long)]
        viewpoint: Option<String>,
        /// Saved view id (a model viewpoint definition).
        #[arg(long)]
        view: Option<String>,
        /// Output YAML file path (stdout when omitted).
        #[arg(long)]
        output: Option<String>,
    },
}

#[derive(Subcommand)]
enum QueryKind {
    /// Elements transitively affected if --element changes (outgoing edges).
    Impact {
        /// Element id.
        #[arg(long)]
        element: String,
    },
    /// Elements --element transitively relies on (incoming edges).
    Deps {
        /// Element id.
        #[arg(long)]
        element: String,
    },
    /// Shortest directed path between two elements.
    Path {
        /// Start element id.
        #[arg(long)]
        from: String,
        /// End element id.
        #[arg(long)]
        to: String,
    },
    /// Elements without any relationship.
    Orphans,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.command {
        Commands::Validate { input } => run_validate(&input),
        Commands::Generate { input, output } => run_generate(&input, &output),
        Commands::Parse { input, output } => run_parse(&input, &output),
        Commands::Diff { old, new } => run_diff(&old, &new),
        Commands::Query { input, query } => run_query(&input, query),
        Commands::View {
            input,
            viewpoint,
            view,
            output,
        } => run_view(&input, viewpoint, view, output),
    }
}

// ---------------------------------------------------------------------------
// Model-loading pipeline — one interface, every subcommand
// ---------------------------------------------------------------------------

/// Why a model could not be loaded from a file.
enum LoadError {
    /// The file could not be read.
    Read {
        path: String,
        source: std::io::Error,
    },
    /// YAML failed schema validation.
    Yaml(Vec<SchemaError>),
    /// The Archi XML could not be parsed.
    Xml(XmlError),
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadError::Read { path, source } => write!(f, "cannot read {path}: {source}"),
            LoadError::Yaml(errors) => {
                let msgs: Vec<String> = errors.iter().map(yaml::schema_error_message).collect();
                write!(f, "schema validation failed: {}", msgs.join("; "))
            }
            LoadError::Xml(e) => write!(f, "XML parse failed: {e}"),
        }
    }
}

/// Loads a YAML model: path → Model, or one structured error.
fn load_yaml(path: &str) -> Result<Model, LoadError> {
    let s = fs::read_to_string(path).map_err(|source| LoadError::Read {
        path: path.into(),
        source,
    })?;
    yaml::parse_yaml(&s).map_err(LoadError::Yaml)
}

/// Loads an Archi XML model: path → Model, or one structured error.
fn load_xml(path: &str) -> Result<Model, LoadError> {
    let s = fs::read_to_string(path).map_err(|source| LoadError::Read {
        path: path.into(),
        source,
    })?;
    xml::xml_to_model(&s).map_err(LoadError::Xml)
}

/// The uniform failure path: one diagnostic line on stderr, exit code 2.
fn fail(error: &LoadError) -> ExitCode {
    eprintln!("error: {error}");
    ExitCode::from(2)
}

/// Writes a generated artifact to disk.
fn store(path: &str, content: &str) -> Result<(), String> {
    fs::write(path, content).map_err(|e| format!("cannot write {path}: {e}"))
}
// ---------------------------------------------------------------------------
// validate
// ---------------------------------------------------------------------------

fn run_validate(input_path: &str) -> ExitCode {
    let model = match load_yaml(input_path) {
        Ok(m) => m,
        Err(e @ (LoadError::Read { .. } | LoadError::Xml(_))) => return fail(&e),
        Err(LoadError::Yaml(schema_errors)) => {
            // Schema errors → success=false with structured errors.
            // If YAML is malformed (MalformedYaml), filter out schema errors.
            let has_malformed_yaml = schema_errors
                .iter()
                .any(|e| matches!(e, SchemaError::MalformedYaml(_)));

            let errors: Vec<serde_json::Value> = if has_malformed_yaml {
                // Show only MalformedYaml errors, drop all schema errors
                schema_errors
                    .iter()
                    .filter_map(|e| {
                        if let SchemaError::MalformedYaml(msg) = e {
                            Some(serde_json::json!({
                                "code": "MalformedYaml",
                                "message": format!("YAML parsing error: {}", msg),
                            }))
                        } else {
                            None
                        }
                    })
                    .collect()
            } else {
                // Show all schema errors normally
                schema_errors
                    .iter()
                    .map(|e| {
                        serde_json::json!({
                            "code": format!("{:?}", e),
                            "message": format!("Schema validation error: {}", yaml::schema_error_message(e)),
                        })
                    })
                    .collect()
            };

            let result = serde_json::json!({
                "success": false,
                "errors": errors,
            });
            println!("{}", serde_json::to_string_pretty(&result).unwrap());
            return ExitCode::from(1);
        }
    };
    let vr = validate_model(&model);
    let json = serde_json::to_string_pretty(&vr).unwrap_or_else(|_| "{}".to_string());
    println!("{}", json);

    if vr.success {
        ExitCode::from(0)
    } else {
        ExitCode::from(1)
    }
}

// ---------------------------------------------------------------------------
// generate
// ---------------------------------------------------------------------------

fn run_generate(input_path: &str, output_path: &str) -> ExitCode {
    let model = match load_yaml(input_path) {
        Ok(m) => m,
        Err(e) => return fail(&e),
    };

    // Calculate layout positions.
    let mut resolver = LayoutResolver::default();
    if let Err(e) = resolver.calculate_layout(&model) {
        eprintln!("error: layout calculation failed: {e}");
        return ExitCode::from(2);
    }

    // Convert (x, y) to (x, y, width, height) with default dimensions.
    let positions = resolver
        .positions()
        .iter()
        .map(|(&id, pos)| (id, (pos.0, pos.1, 120.0, 55.0)))
        .collect::<HashMap<_, _>>();

    let xml = match xml::model_to_xml(&model, &positions) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("error: XML serialization failed: {e}");
            return ExitCode::from(2);
        }
    };

    if let Err(msg) = store(output_path, &xml) {
        eprintln!("error: {msg}");
        return ExitCode::from(2);
    }

    eprintln!(
        "Generated {output_path} ({} elements, {} relationships)",
        model.element_count(),
        model.relation_count()
    );
    ExitCode::from(0)
}

// ---------------------------------------------------------------------------
// parse
// ---------------------------------------------------------------------------

fn run_parse(input_path: &str, output_path: &str) -> ExitCode {
    let model = match load_xml(input_path) {
        Ok(m) => m,
        Err(e) => return fail(&e),
    };
    let yaml_out = yaml::model_to_yaml(&model);
    if let Err(msg) = store(output_path, &yaml_out) {
        eprintln!("error: {msg}");
        return ExitCode::from(2);
    }

    eprintln!(
        "Parsed {input_path} -> {output_path} ({} elements, {} relationships)",
        model.element_count(),
        model.relation_count()
    );
    ExitCode::from(0)
}

// ---------------------------------------------------------------------------
// diff
// ---------------------------------------------------------------------------

fn run_diff(old_path: &str, new_path: &str) -> ExitCode {
    // Existing model (XML) and new model (YAML), through one load interface.
    let existing = match load_xml(old_path) {
        Ok(m) => m,
        Err(e) => return fail(&e),
    };
    let new = match load_yaml(new_path) {
        Ok(m) => m,
        Err(e) => return fail(&e),
    };
    let analyzer = ModelDiffAnalyzer::from_existing(&existing);
    match analyzer.analyze_update(&new) {
        Ok(report) => {
            let json = serde_json::to_string_pretty(&report).unwrap_or_else(|_| "{}".to_string());
            println!("{}", json);
            ExitCode::from(0)
        }
        Err(errors) => {
            let json = serde_json::to_string_pretty(&errors).unwrap_or_else(|_| "[]".to_string());
            eprintln!("error: reference errors: {json}");
            ExitCode::from(2)
        }
    }
}

// ---------------------------------------------------------------------------
// query
// ---------------------------------------------------------------------------

fn run_query(input_path: &str, query: QueryKind) -> ExitCode {
    let model = match load_yaml(input_path) {
        Ok(m) => m,
        Err(e) => return fail(&e),
    };
    let ref_of = |s: &str| RefId::new(s);
    let result: Result<serde_json::Value, QueryError> = match query {
        QueryKind::Impact { element } => {
            impact(&model, &ref_of(&element)).map(|ids| serde_json::json!({ "element": element, "impact": ids }))
        }
        QueryKind::Deps { element } => {
            dependencies(&model, &ref_of(&element)).map(|ids| serde_json::json!({ "element": element, "dependencies": ids }))
        }
        QueryKind::Path { from, to } => path(&model, &ref_of(&from), &ref_of(&to)).map(|p| {
            serde_json::json!({ "from": from, "to": to, "path": p })
        }),
        QueryKind::Orphans => Ok(serde_json::json!({ "orphans": orphans(&model) })),
    };
    match result {
        Ok(json) => {
            println!("{}", serde_json::to_string_pretty(&json).unwrap_or_else(|_| "{}".to_string()));
            ExitCode::from(0)
        }
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(2)
        }
    }
}

// ---------------------------------------------------------------------------
// view
// ---------------------------------------------------------------------------

fn run_view(
    input_path: &str,
    viewpoint: Option<String>,
    view: Option<String>,
    output_path: Option<String>,
) -> ExitCode {
    let model = match load_yaml(input_path) {
        Ok(m) => m,
        Err(e) => return fail(&e),
    };
    let projected = match (viewpoint, view) {
        (Some(name), _) => match Viewpoint::from_yaml_viewpoint_name(&name) {
            Some(vp) => match project(&model, vp) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("error: {e}");
                    return ExitCode::from(2);
                }
            },
            None => {
                eprintln!("error: unknown viewpoint `{name}`");
                return ExitCode::from(2);
            }
        },
        (_, Some(id)) => match project_view(&model, &RefId::new(&id)) {
            Some(m) => m,
            None => {
                eprintln!("error: unknown view `{id}`");
                return ExitCode::from(2);
            }
        },
        (None, None) => {
            eprintln!("error: exactly one of --viewpoint/--view is required");
            return ExitCode::from(2);
        }
    };
    let yaml = model_to_yaml(&projected);
    match output_path {
        Some(p) => {
            if let Err(msg) = store(&p, &yaml) {
                eprintln!("error: {msg}");
                return ExitCode::from(2);
            }
            eprintln!(
                "Projected {input_path} -> {p} ({} elements, {} relationships)",
                projected.element_count(),
                projected.relation_count()
            );
        }
        None => print!("{yaml}"),
    }
    ExitCode::from(0)
}
