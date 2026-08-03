# Implementation Guide: `archr` — Headless ArchiMate Engine + AI Skill

> **Version 2.1** — Rust Engine (CLI) + AI Skill in **Monorepo** with strict code isolation. The Skill follows the [Agent Skills Specification](https://agentskills.io/specification): `SKILL.md` + `scripts/archr.py` self-contained (PEP 723), compatible with VS Code Copilot, Claude Code, and OpenAI Codex.
>
> **Note:** Code snippets below are illustrative design intent from the original architecture phase. The actual implementation in `crates/archr-core/src/` may differ — always refer to the source for ground truth.

---

## 1. Architecture Overview

The `archr` is a **standalone** tool, written in Rust, that serves as an engine for validation, manipulation, and export of ArchiMate 3.2 models, designed from the ground up for native integration with **AI Agents** via a **Skill** (Python/Node wrapper).

The central decision of v2: **Rust engine and AI Skill live in the same repository (Monorepo)** with strict code isolation. The schema coupling between the two is the biggest architectural risk — any drift between what the Skill teaches and what Rust validates produces difficult-to-debug validation errors. Monorepo makes each release atomic.

**Hierarchical Data Flow:**
```
[User]
   |
   v
[AI Agent] --> Generates [Intermediate YAML File]
   |
   v
[Skill: archr.py]     --> 1. Writes YAML to disk
   |                      2. Executes `archr validate`
   |                      3. Captures stdout (JSON) and stderr
   v
[Rust CLI: archr]    --> a. Parse YAML -> Arena/Indices
                          b. ArchiMate derivability validation
                          c. Diff with existing model (if applicable)
                          d. Automatic layout resolution (graph)
                          e. Serialization to XML (Open Exchange) / YAML / Mermaid
   |
   v
[User]
   |
   v
[.archimate File (XML)] --> Opened in Archi editor
   |
   v
[User]
```

### Decision: Monorepo with strict isolation

- **Why together:** The biggest risk isn't Rust bugs, it's **schema mismatch**. The Skill teaches the AI to generate YAML; Rust validates that YAML. If the `ElementKind` enum changes in Rust, the `SKILL.md` needs to update in the same commit. Separate repos = classic *version drift*.
- **Why isolated:** The Skill **does not** import Rust via FFI (PyO3) nor compile at runtime. Communication is strictly CLI (`subprocess.run`). Perfect isolation, zero memory/ABI coupling.
- **Why follow Agent Skills spec:** Interoperability. Any spec-compliant client (VS Code Copilot, Claude Code, OpenAI Codex, etc.) discovers and activates the Skill without glue code. Without spec, each integration is custom.

---

## 2. Monorepo Structure

The Skill follows the [Agent Skills Specification](https://agentskills.io/specification): a directory containing `SKILL.md` (frontmatter YAML + instructions in body) and a subdir `scripts/` with self-contained executable code (PEP 723 for Python).

```
archr/                            # Main repository (GitHub)
├── Cargo.toml                    # Rust workspace
├── crates/
│   └── archr-core/               # The engine in Rust (CLI, Parser, Validator, Layout)
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs           # CLI entry point (clap)
│           ├── model.rs          # Arena + Element/Relationship
│           ├── validate.rs       # ArchiMate 3.2 derivability rules
│           ├── diff.rs           # ModelDiffAnalyzer
│           ├── layout.rs         # LayoutResolver (petgraph)
│           └── io/
│               ├── yaml.rs       # serde_yaml <-> Model
│               └── xml.rs        # quick-xml <-> Open Exchange
├── skill/                        # Agent Skill (spec-compliant)
│   ├── SKILL.md                  # Required: frontmatter + instructions for AI
│   ├── scripts/
│   │   └── archr.py              # Self-contained Python CLI (PEP 723): invokes Rust binary
│   └── references/
│       └── ARCHIMATE_RULES.md    # Rule documentation (progressive disclosure)
├── tests/                        # E2E tests (AI -> Skill -> Rust)
├── docs/
│   └── archimate_implementation_guide.md     # This document
└── README.md
```

> **Where the Skill is installed:** copy the `skill/` directory to `.agents/skills/archr-skill/` in the user's project. VS Code Copilot, Claude Code, OpenAI Codex, and any spec-compatible client automatically discover skills in this path. The `name: archr-skill` in the frontmatter must match the directory name.

### Monorepo Rules

1. **No direct code dependency.** Python Skill never imports Rust via FFI. Communication only via CLI.
2. **Skill = self-contained directory.** No `pyproject.toml`, no `pip install`. The Python script uses only stdlib + inline PEP 723 declaration, and is invoked by the agent directly via terminal.
3. **Separate CI by language.** GitHub Actions with distinct jobs:
   - `build-rust`: runs `cargo test` on workspace
   - `test-skill`: runs `python skill/scripts/archr.py --help` + Skill CLI tests (mocking Rust binary)
   - `e2e-test`: downloads compiled Rust binary, runs Skill, tests full flow
4. **Binary versioning.** The `SKILL.md` declares `compatibility:` with minimum `archr` version. The `archr.py` script checks the actual binary version at each execution start and fails fast if there's a mismatch.

### When to separate repositories in the future?

Only if the community creates **multiple Skills** (one for LangChain, another for Semantic Kernel, another as native MCP Server). Then `archr` (Rust) stays pure, and Skills migrate to a GitHub organization `archr-skills/langchain`, `archr-skills/mcp`, etc. For the MVP, monorepo drastically reduces maintenance friction.

---

## 3. Prerequisites and Dependencies

**Languages & Build:**
- Rust (Edition 2021 or higher)
- Cargo (package manager)
- Python 3.10+ (only for the Skill)

**`crates/archr-core/Cargo.toml`:**
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
quick-xml = "0.31"
clap = { version = "4.5", features = ["derive"] }
petgraph = "0.6"
uuid = { version = "1.8", features = ["v4", "serde"] }
thiserror = "1.0"
```

**Skill — no external dependencies.** The `skill/scripts/archr.py` is self-contained: uses only stdlib (`subprocess`, `json`, `argparse`, `os`, `sys`, `tempfile`) and inline declaration via [PEP 723](https://peps.python.org/pep-0723/) if more is needed. There's no `pyproject.toml`, `requirements.txt`, or `pip install` — the agent executes `python skill/scripts/archr.py ...` directly.

---

## 4. Environment Configuration

The tool is **stateless** and runs in sandbox mode. There are no mandatory environment variables or backend services. Configuration via CLI arguments.
**Usage Structure (Rust binary):**
```bash
# Validate
./archr validate --input model.yaml

# Generate final XML
./archr generate --input model.yaml --output model.archimate

# Convert existing XML to YAML (LLM input)
./archr parse --input model.archimate --output model.yaml

# Diff between existing model and new YAML
./archr diff --old model.archimate --new model.yaml
```
---

**Usage Structure (Python Skill — invoked directly by agent via terminal):**
```bash
# Validate (agent always calls this FIRST)
python skill/scripts/archr.py validate model.yaml

# Generate final XML (only after validate returns success=true)
python skill/scripts/archr.py generate model.yaml --output out.archimate
```

The `SKILL.md` instructs the agent to call these commands; the agent reads the JSON from stdout and decides the next step. There's no Function Calling wrapper — the Agent Skills spec treats the agent as a terminal executor.

---

## 5. Data Model / Schema

The internal structure uses the **Arena with Typed Indices** pattern to manage the graph without *Borrow Checker* conflicts.

### Internal Schema (Rust Structs)

| Structure       | Fields                                                                      | Description                                                          |
|:----------------|:----------------------------------------------------------------------------|:---------------------------------------------------------------------|
| `Model`         | `name: String`, `elements: Vec<Element>`, `relations: Vec<Relationship>`    | Root container. Exclusive ownership of all data.                 |
| `Element`       | `id: ElementId`, `name: String`, `kind: ElementKind`                        | Graph node.                                                       |
| `Relationship`  | `id: RelationId`, `source: ElementId`, `target: ElementId`, `kind: RelationKind` | Directed edge.                                                    |
| `ElementId`     | `pub struct ElementId(pub usize);`                                          | Strong index for O(1) access to `Vec`                           |
| `RelationId`    | `pub struct RelationId(pub usize);`                                         | Strong index for O(1) access to `Vec`                           |

### Intermediate YAML Schema (AI Input/Output)

```yaml
model:
  name: "String"
  elements:
    - id: "String"        # Must be unique; snake_case, no spaces
      name: "String"
      kind: "String"      # e.g. BusinessActor, ApplicationComponent, Node
  relationships:
    - id: "String"
      source: "String"    # Reference to existing element.id
      target: "String"    # Reference to existing element.id
      kind: "String"      # e.g. Serving, Realization, Assignment
```

> **Critical contract:** This YAML is the coupling point between Skill (teaches AI) and Rust (validates). Any change to `ElementKind`/`RelationKind` enum requires simultaneous update to the `skill/SKILL.md` body.

---

## 6. API Contracts / Endpoints

### Rust Binary Endpoints

| Command     | Arguments                                   | Response (stdout)                              |
|:------------|:---------------------------------------------|:-----------------------------------------------|
| `validate`  | `--input <file>`                             | JSON with success or structured error list. |
| `generate`  | `--input <yaml>`, `--output <xml>`           | Status message in text.                   |
| `parse`     | `--input <xml>`, `--output <yaml>`           | Status message in text.                   |
| `diff`      | `--old <xml>`, `--new <yaml>`                | JSON with added/removed elements.      |
| `--version` | —                                            | Semantic version (e.g. `archr 1.0.0`).          |

### Skill Exposed Endpoint (Self-contained Python CLI)

The Skill exposes two actions via `skill/scripts/archr.py`. The agent invokes directly in terminal, following `SKILL.md` instructions. Structured output (JSON) always in stdout; diagnostics in stderr.

| Skill Command                                         | Action                                                  | Output (stdout)                                  |
|:------------------------------------------------------|:--------------------------------------------------------|:------------------------------------------------|
| `python skill/scripts/archr.py validate <file.yaml>`  | Validate intermediate YAML against ArchiMate rules.   | JSON `{"success": bool, "errors": [...]}`       |
| `python skill/scripts/archr.py generate <file.yaml>`  | Generate final XML after implicit validation.             | JSON `{"success": bool, "message\|error": ...}` |
| `python skill/scripts/archr.py --help`                | Usage documentation for agent.                        | Text with flags and exit codes.                   |

**Documented exit codes (standard Agent Skills spec):**

| Code | Meaning                                  |
|:-----|:-----------------------------------------|
| 0    | Success.                                 |
| 1    | Validation error (valid YAML, broken rules). stdout contains JSON errors. |
| 2    | Malformed YAML or I/O error.              |
| 3    | `archr` binary version incompatible with `compatibility:` in SKILL.md. |
| 4    | Subprocess timeout.                       |
| 64   | Invalid arguments (incorrect usage).       |

### Error Response Example (Validation)

```json
{
  "success": false,
  "errors": [
    {
      "code": "INVALID_RELATIONSHIP",
      "message": "BusinessActor cannot have Realization relationship with ApplicationComponent",
      "element_source": "client_001",
      "suggestion": "Consider using 'Serving' or reverse the direction."
    }
  ]
}
```

---

## 7. Step-by-Step Implementation Flow

Below, the logical sequence and code snippets of the chosen final version.

### 7.1. Arena Pattern for Graph Ownership

*Justification:* Avoids `Rc<RefCell<>>` (slow and prone to panic) and string `HashMaps` (slow). Direct O(1) memory access.

```rust
pub struct Model {
    pub name: String,
    elements: Vec<Element>,
    relations: Vec<Relationship>,
}

impl Model {
    pub fn add_element(&mut self, name: &str, kind: ElementKind) -> ElementId {
        let id = ElementId(self.elements.len());
        self.elements.push(Element { id, name: name.to_string(), kind });
        id
    }

    pub fn element(&self, id: ElementId) -> &Element {
        &self.elements[id.0] // Direct indexing O(1)
    }

    pub fn link(&mut self, source: ElementId, target: ElementId, kind: RelationKind) -> RelationId {
        let id = RelationId(self.relations.len());
        self.relations.push(Relationship { id, source, target, kind });
        id
    }
}
```

### 7.2. ArchiMate Rule Validation

*Justification:* Ensure AI doesn't create semantically invalid models.

```rust
pub fn validate_relationship(
    source: &Element,
    target: &Element,
    rel: &RelationKind,
) -> Result<(), String> {
    match (source.kind, target.kind, rel) {
        (ElementKind::ApplicationComponent, ElementKind::Node, RelationKind::Realization) => Ok(()),
        (ElementKind::BusinessActor, ElementKind::ApplicationComponent, RelationKind::Serving) => Ok(()),
        // ... other valid rules
        _ => Err(format!(
            "Rule violated: {:?} cannot {:?} {:?}",
            source.kind, rel, target.kind
        )),
    }
}
```

### 7.3. Diff Analysis (Model Editing)

*Justification:* Necessary to update models without breaking existing ID references.

```rust
pub fn diff_models(old: &Model, new: &Model) -> DiffResult {
    let mut result = DiffResult::default();
    
    // Find added elements
    for new_elem in &new.elements {
        if !old.elements.iter().any(|old_elem| old_elem.id == new_elem.id) {
            result.added.push(new_elem.clone());
        }
    }
    
    // Find removed elements
    for old_elem in &old.elements {
        if !new.elements.iter().any(|new_elem| new_elem.id == old_elem.id) {
            result.removed.push(old_elem.clone());
        }
    }
    
    // Find modified elements
    for new_elem in &new.elements {
        if let Some(old_elem) = old.elements.iter().find(|old_elem| old_elem.id == new_elem.id) {
            if old_elem.name != new_elem.name || old_elem.kind != new_elem.kind {
                result.modified.push(ModifiedElement {
                    id: new_elem.id,
                    old: old_elem.clone(),
                    new: new_elem.clone(),
                });
            }
        }
    }
    
    result
}
```

### 7.4. YAML I/O (AI Interface)

*Justification:* Bridge between AI-generated YAML and Rust Model.

```rust
pub fn parse_yaml(input: &str) -> Result<Model, Vec<SchemaError>> {
    let yaml_model: YamlModel = serde_yaml::from_str(input)?;
    let mut model = Model::new(yaml_model.model.name);
    
    // Parse elements
    for yaml_elem in yaml_model.model.elements {
        let kind = ElementKind::from_str(&yaml_elem.kind)
            .map_err(|_| SchemaError::new(yaml_elem.id.clone(), "UNKNOWN_KIND"))?;
        let element_id = model.add_element(&yaml_elem.name, kind);
        
        // Store ID mapping for relationship validation
        model.element_id_map.insert(yaml_elem.id, element_id);
    }
    
    // Parse relationships
    for yaml_rel in yaml_model.model.relationships {
        let source_id = model.element_id_map.get(&yaml_rel.source)
            .ok_or_else(|| SchemaError::new(yaml_rel.id.clone(), "UNDEFINED_ID"))?;
        let target_id = model.element_id_map.get(&yaml_rel.target)
            .ok_or_else(|| SchemaError::new(yaml_rel.id.clone(), "UNDEFINED_ID"))?;
        
        let kind = RelationKind::from_str(&yaml_rel.kind)
            .map_err(|_| SchemaError::new(yaml_rel.id.clone(), "UNKNOWN_KIND"))?;
        
        model.link(*source_id, *target_id, kind);
    }
    
    Ok(model)
}
```

### 7.5. XML Export (Open Exchange)

*Justification:* Enable visualization in Archi tool.

```rust
pub fn model_to_xml(model: &Model, positions: &HashMap<ElementId, (f64, f64)>) -> Result<String, XmlError> {
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    writer.write_event(Event::Start(BytesStart::new("model")
        .with_attributes(vec![
            ("identifier", &uuid::Uuid::new_v4().to_string()),
            ("xmlns", "http://www.opengroup.org/xsd/archimate/3.0/"),
            ("version", "3.2"),
        ])))?;
    
    // Write model name
    writer.write_event(Event::Start(BytesStart::new("name")))?;
    writer.write_event(Event::Text(BytesText::new(&model.name)))?;
    writer.write_event(Event::End(BytesEnd::new("name")))?;
    
    // Write elements
    writer.write_event(Event::Start(BytesStart::new("elements")))?;
    for element in &model.elements {
        let (x, y) = positions.get(&element.id).unwrap_or(&(0.0, 0.0));
        writer.write_event(Event::Start(BytesStart::new("element")
            .with_attributes(vec![
                ("identifier", &uuid::Uuid::new_v4().to_string()),
                ("xsi:type", &element.kind.to_string()),
            ])))?;
        writer.write_event(Event::Start(BytesStart::new("name")))?;
        writer.write_event(Event::Text(BytesText::new(&element.name)))?;
        writer.write_event(Event::End(BytesEnd::new("name")))?;
        writer.write_event(Event::End(BytesEnd::new("element")))?;
    }
    writer.write_event(Event::End(BytesEnd::new("elements")))?;
    
    // Write relationships
    writer.write_event(Event::Start(BytesStart::new("relationships")))?;
    for relationship in &model.relations {
        let source_uuid = uuid::Uuid::new_v4().to_string();
        let target_uuid = uuid::Uuid::new_v4().to_string();
        
        writer.write_event(Event::Start(BytesStart::new("relationship")
            .with_attributes(vec![
                ("identifier", &uuid::Uuid::new_v4().to_string()),
                ("source", &source_uuid),
                ("target", &target_uuid),
                ("xsi:type", &relationship.kind.to_string()),
            ])))?;
        writer.write_event(Event::End(BytesEnd::new("relationship")))?;
    }
    writer.write_event(Event::End(BytesEnd::new("relationships")))?;
    
    writer.write_event(Event::End(BytesEnd::new("model")))?;
    
    Ok(String::from_utf8(writer.into_inner().into_inner())?)
}
```

### 7.6. Automatic Layout (Grid by Topological Layer)

*Justification:* Calculate X/Y positions automatically for elements in XML.

```rust
pub struct LayoutResolver {
    positions: HashMap<ElementId, (f64, f64)>,
    layer_positions: HashMap<ElementLayer, f64>,
}

impl LayoutResolver {
    pub fn new() -> Self {
        Self {
            positions: HashMap::new(),
            layer_positions: HashMap::new(),
        }
    }
    
    pub fn calculate_layout(&mut self, model: &Model) -> Result<(), LayoutError> {
        // Build graph from model
        let mut graph = UnGraphMap::new();
        for element in &model.elements {
            graph.add_node(element.id);
        }
        for relationship in &model.relations {
            graph.add_edge(relationship.source, relationship.target, ());
        }
        
        // Calculate layer positions
        let mut layer_y = 0.0;
        for layer in ElementLayer::iter() {
            self.layer_positions.insert(layer, layer_y);
            layer_y += 100.0; // 100px spacing between layers
        }
        
        // Position elements by layer
        for element in &model.elements {
            let layer = element.kind.layer();
            let y = self.layer_positions.get(&layer).unwrap_or(&0.0);
            let x = (element.id.0 as f64) * 150.0; // 150px spacing between elements
            self.positions.insert(element.id, (x, *y));
        }
        
        Ok(())
    }
    
    pub fn positions(&self) -> &HashMap<ElementId, (f64, f64)> {
        &self.positions
    }
}
```

### 7.7. CLI Interface (Main.rs)

*Justification:* Command-line interface for all engine functionality.

```rust
#[derive(Parser)]
#[command(name = "archr", about = "ArchiMate 3.2 engine")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Validate {
        #[arg(short, long)]
        input: PathBuf,
    },
    Generate {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
    },
    Parse {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
    },
    Diff {
        #[arg(short, long)]
        old: PathBuf,
        #[arg(short, long)]
        new: PathBuf,
    },
    Version,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Validate { input, format } => {
            let content = std::fs::read_to_string(input)?;
            let model = parse_yaml(&content)?;
            let result = validate_model(&model);
            match format.as_str() {
                "json" => println!("{}", serde_json::to_string(&result)?),
                _ => println!("Validation result: {:?}", result),
            }
        }
        Commands::Generate { input, output } => {
            let content = std::fs::read_to_string(input)?;
            let model = parse_yaml(&content)?;
            let mut resolver = LayoutResolver::new();
            resolver.calculate_layout(&model)?;
            let xml = model_to_xml(&model, &resolver.positions())?;
            std::fs::write(output, xml)?;
            println!("Model generated successfully");
        }
        Commands::Parse { input, output } => {
            let content = std::fs::read_to_string(input)?;
            let model = xml_to_model(&content)?;
            let yaml = model_to_yaml(&model);
            std::fs::write(output, yaml)?;
            println!("Model parsed successfully");
        }
        Commands::Diff { old, new } => {
            let old_content = std::fs::read_to_string(old)?;
            let new_content = std::fs::read_to_string(new)?;
            let old_model = xml_to_model(&old_content)?;
            let new_model = parse_yaml(&new_content)?;
            let diff = diff_models(&old_model, &new_model);
            println!("{}", serde_json::to_string(&diff)?);
        }
        Commands::Version => {
            println!("archr 1.0.0");
        }
    }
    
    Ok(())
}
```

### 7.8. AI Skill (Python)

*Justification:* Agent Skills Specification-compliant wrapper for AI integration.

```python
#!/usr/bin/env python3
"""
SKILL.md: archr-skill
name: archr-skill
description: ArchiMate 3.2 validation and generation for AI agents
version: 1.0.0
compatibility: archr >= 1.0.0
"""

import subprocess
import json
import argparse
import sys
import tempfile
import os

def validate(yaml_file):
    """Validate YAML against ArchiMate rules"""
    result = subprocess.run(
        ["archr", "validate", "--input", yaml_file],
        capture_output=True,
        text=True,
        check=False
    )
    
    if result.returncode == 0:
        print(result.stdout, end="")
    else:
        print(result.stdout, end="")
        sys.exit(1)

def generate(yaml_file, output_file):
    """Generate XML from YAML after validation"""
    # First validate
    validate(yaml_file)
    
    # Then generate
    result = subprocess.run(
        ["archr", "generate", "--input", yaml_file, "--output", output_file],
        capture_output=True,
        text=True,
        check=False
    )
    
    if result.returncode == 0:
        print(json.dumps({"success": True, "message": "Model generated successfully"}))
    else:
        print(json.dumps({"success": False, "error": result.stderr}))

def main():
    parser = argparse.ArgumentParser(description="archr-skill CLI")
    parser.add_argument("command", choices=["validate", "generate"])
    parser.add_argument("yaml_file", help="YAML input file")
    parser.add_argument("--output", help="Output file for generate command")
    args = parser.parse_args()
    
    if args.command == "validate":
        validate(args.yaml_file)
    elif args.command == "generate":
        if not args.output:
            print("Error: --output is required for generate command", file=sys.stderr)
            sys.exit(64)
        generate(args.yaml_file, args.output)

if __name__ == "__main__":
    main()
```

---

## 8. Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_arena_add_element() {
        let mut model = Model::new("Test Model");
        let id1 = model.add_element("Element 1", ElementKind::BusinessActor);
        let id2 = model.add_element("Element 2", ElementKind::ApplicationComponent);
        
        assert_eq!(model.elements.len(), 2);
        assert_eq!(model.element(id1).name, "Element 1");
        assert_eq!(model.element(id2).name, "Element 2");
    }
    
    #[test]
    fn test_validate_relationship() {
        let source = Element { id: ElementId(0), name: "Source".to_string(), kind: ElementKind::BusinessActor };
        let target = Element { id: ElementId(1), name: "Target".to_string(), kind: ElementKind::ApplicationComponent };
        
        // Valid relationship
        assert!(validate_relationship(&source, &target, &RelationKind::Serving).is_ok());
        
        // Invalid relationship
        assert!(validate_relationship(&source, &target, &RelationKind::Realization).is_err());
    }
}
```

### Integration Tests

```bash
# Test Skill integration
python skill/scripts/archr.py validate test_model.yaml

# Test end-to-end flow
python skill/scripts/archr.py generate test_model.yaml --output test_model.archimate
```

### E2E Tests

```bash
# Download latest release
curl -L -o archr https://github.com/haquiticos/archr/releases/latest/download/archr

# Run E2E test
python skill/scripts/archr.py validate test_model.yaml
```

---

## 9. CI/CD Pipeline

### GitHub Actions Workflows

```yaml
name: CI/CD Pipeline
on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  build-rust:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
      - name: Build
        run: cargo build --workspace
      - name: Test
        run: cargo test --workspace

  test-skill:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Test Skill
        run: |
          python skill/scripts/archr.py --help
          python skill/scripts/archr.py validate test_model.yaml

  e2e-test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
      - name: Build Release
        run: cargo build --release
      - name: Run E2E Tests
        run: |
          ./target/release/archr validate --input test_model.yaml
          python skill/scripts/archr.py validate test_model.yaml

  release:
    runs-on: ubuntu-latest
    needs: [build-rust, test-skill, e2e-test]
    steps:
      - uses: actions/checkout@v4
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
      - name: Build Release
        run: cargo build --release
      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          files: target/release/archr
          prerelease: false
          publish: true
          token: ${{ secrets.GITHUB_TOKEN }}
```

---

## 10. Performance Considerations

### Memory Usage

- **Arena pattern:** O(1) direct memory access, no heap allocations for graph traversal
- **Typed indices:** Prevents string lookups, reduces memory fragmentation
- **Efficient data structures:** `Vec` for elements/relations, `HashMap` for quick lookups

### Startup Time

- **Standalone binary:** No JVM startup (~seconds to ~ms)
- **Minimal dependencies:** Only essential crates, no heavy frameworks
- **Lazy loading:** Components loaded on-demand

### Throughput

- **Batch processing:** Designed for processing multiple models
- **Parallel processing:** Potential for parallel validation of independent models
- **Caching:** Optional caching of frequently used data

---

## 11. Security Considerations

### Input Validation

- **Schema validation:** Strict YAML schema validation
- **Type safety:** Rust's type system prevents many common vulnerabilities
- **Error handling:** Graceful error handling without panics

### Dependency Security

- **Minimal dependencies:** Only trusted, well-maintained crates
- **Regular updates:** Automated dependency updates and security scanning
- **Vulnerability scanning:** CI includes security vulnerability checks

### Runtime Security

- **Sandboxed execution:** Skill runs in isolated environment
- **Input sanitization:** All inputs validated before processing
- **Error reporting:** Structured error reporting without sensitive information

---

## 12. Future Enhancements

### Planned Features

- **Web API:** RESTful interface for programmatic access
- **GUI:** Desktop application for model visualization
- **Plugin system:** Extend functionality with user-defined plugins
- **Cloud integration:** Support for cloud-based model storage and collaboration
- **Advanced layout:** More sophisticated layout algorithms
- **Performance optimization:** Further memory and speed optimizations

### Community Contributions

- **Contributor guidelines:** Clear guidelines for contributing
- **Good first issues:** Tagged issues for new contributors
- **Documentation improvements:** Enhanced documentation and examples
- **Testing improvements:** Expanded test coverage

---

## 13. Conclusion

This implementation guide provides a comprehensive roadmap for developing the `archr` engine with strict separation between Rust core and Python Skill, ensuring maintainability and extensibility for future enhancements. The monorepo structure, combined with the Agent Skills Specification, creates a robust foundation for AI agent integration while maintaining high code quality and performance standards.