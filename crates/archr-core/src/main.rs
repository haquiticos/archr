//! `archr` — headless ArchiMate 3.2 engine CLI.
//!
//! Subcommands: validate, generate, parse, diff.

use archr_core::{
    diff::ModelDiffAnalyzer,
    io::{xml, yaml},
    layout::LayoutResolver,
    validate::validate_model,
};
use clap::{Parser, Subcommand};
use std::collections::HashMap;
use std::fs;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "archr", version = "1.0.0", about = "Headless ArchiMate 3.2 engine")]
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
        /// Output format.
        #[arg(long, default_value = "json")]
        format: String,
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
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.command {
        Commands::Validate { input, format } => run_validate(&input, &format),
        Commands::Generate { input, output } => run_generate(&input, &output),
        Commands::Parse { input, output } => run_parse(&input, &output),
        Commands::Diff { old, new } => run_diff(&old, &new),
    }
}

// ---------------------------------------------------------------------------
// validate
// ---------------------------------------------------------------------------

fn run_validate(input_path: &str, _format: &str) -> ExitCode {
    let yaml_str = match fs::read_to_string(input_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: cannot read {input_path}: {e}");
            return ExitCode::from(2);
        }
    };

    let model = match yaml::parse_yaml(&yaml_str) {
        Ok(m) => m,
        Err(schema_errors) => {
            // Schema errors → success=false with structured errors.
            let errors: Vec<serde_json::Value> = schema_errors
                .iter()
                .map(|e| {
                    serde_json::json!({
                        "code": format!("{:?}", e),
                        "message": format!("Schema validation error: {:?}", e),
                    })
                })
                .collect();
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
    let yaml_str = match fs::read_to_string(input_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: cannot read {input_path}: {e}");
            return ExitCode::from(2);
        }
    };

    let model = match yaml::parse_yaml(&yaml_str) {
        Ok(m) => m,
        Err(errors) => {
            eprintln!("error: schema validation failed: {:?}", errors);
            return ExitCode::from(2);
        }
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

    if let Err(e) = fs::write(output_path, &xml) {
        eprintln!("error: cannot write {output_path}: {e}");
        return ExitCode::from(2);
    }

    eprintln!("Generated {output_path} ({} elements, {} relationships)",
        model.element_count(),
        model.relation_count());
    ExitCode::from(0)
}

// ---------------------------------------------------------------------------
// parse
// ---------------------------------------------------------------------------

fn run_parse(input_path: &str, output_path: &str) -> ExitCode {
    let xml_str = match fs::read_to_string(input_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: cannot read {input_path}: {e}");
            return ExitCode::from(2);
        }
    };

    let model = match xml::xml_to_model(&xml_str) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("error: XML parse failed: {e}");
            return ExitCode::from(2);
        }
    };

    let yaml_out = yaml::model_to_yaml(&model);
    if let Err(e) = fs::write(output_path, &yaml_out) {
        eprintln!("error: cannot write {output_path}: {e}");
        return ExitCode::from(2);
    }

    eprintln!("Parsed {input_path} -> {output_path} ({} elements, {} relationships)",
        model.element_count(),
        model.relation_count());
    ExitCode::from(0)
}

// ---------------------------------------------------------------------------
// diff
// ---------------------------------------------------------------------------

fn run_diff(old_path: &str, new_path: &str) -> ExitCode {
    // Read existing model (XML).
    let xml_str = match fs::read_to_string(old_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: cannot read {old_path}: {e}");
            return ExitCode::from(2);
        }
    };
    let existing = match xml::xml_to_model(&xml_str) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("error: cannot parse {old_path}: {e}");
            return ExitCode::from(2);
        }
    };

    // Read new model (YAML).
    let yaml_str = match fs::read_to_string(new_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: cannot read {new_path}: {e}");
            return ExitCode::from(2);
        }
    };
    let new = match yaml::parse_yaml(&yaml_str) {
        Ok(m) => m,
        Err(errors) => {
            eprintln!("error: schema validation failed: {:?}", errors);
            return ExitCode::from(2);
        }
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
