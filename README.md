# archr
<p align="center">
  <a href="https://github.com/haquiticos/archr/actions/workflows/build-rust.yml">
    <img src="https://img.shields.io/github/actions/workflow/status/haquiticos/archr/build-rust.yml?branch=main&label=CI" alt="CI Status">
  </a>
  <a href="https://crates.io/crates/archr-core">
    <img src="https://img.shields.io/crates/v/archr-core.svg" alt="Crates.io Version">
  </a>
  <a href="https://github.com/haquiticos/archr/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License">
  </a>
  <a href="https://crates.io/crates/archr-core">
    <img src="https://img.shields.io/badge/rust-1.74+-blue.svg" alt="Rust MSRV">
  </a>
</p>

A headless Rust engine for ArchiMate 3.2 — validate, manipulate, and export architecture models via CLI and YAML. Designed for AI agent integration.

## Features

- **Validate** ArchiMate 3.2 models against the full derivability ruleset (62 element types, 11 relationship types, 8 layers)
- **Generate** Open Exchange XML (`.archimate`) from human-friendly YAML
- **Parse** existing `.archimate` XML back into YAML
- **Diff** two models to detect added, removed, and modified elements
- **Automatic layout** — grid placement by topological layer (no Sugiyama)
- **Agent Skill** — PEP 723 Python wrapper for Claude Code, Copilot, and Codex

## Documentation

- [docs/schema.yaml](docs/schema.yaml) — YAML reference schema (62 element kinds, 11 relationship kinds, 6 viewpoint kinds)
- [docs/viewpoints.md](docs/viewpoints.md) — Viewpoints guide
- [haquiticos.github.io/archr](https://haquiticos.github.io/archr) — full installation, upgrade, and CLI reference

## Installation

### Install script (macOS & Linux)

```bash
curl -fsSL https://raw.githubusercontent.com/haquiticos/archr/main/install/install.sh | bash
```

Pin a version:

```bash
curl -fsSL https://raw.githubusercontent.com/haquiticos/archr/main/install/install.sh | bash -s "v0.5.1"
```

### Install script (Windows)

```powershell
powershell -c "irm https://raw.githubusercontent.com/haquiticos/archr/main/install/install.ps1 | iex"
```

### Cargo Install

```bash
cargo install archr-core
archr --version
```

### Build from Source

```bash
git clone https://github.com/haquiticos/archr.git
cd archr
cargo build --release
# Binary at target/release/archr
```

### Verify

```bash
archr --version
# archr 1.1.0
```

### Define a model (YAML)

```yaml
model:
  name: My Architecture
  elements:
    - id: actor_001
      name: Customer
      kind: BusinessActor
    - id: app_001
      name: CRM
      kind: ApplicationComponent
  relationships:
    - id: rel_001
      source: app_001
      target: actor_001
      kind: Serving
  viewpoints:
    - id: vp_business
      name: Business Viewpoint
      kind: business
      elements:
        - actor_001
      relationships: []
```

> **Schema:** See [`docs/schema.yaml`](docs/schema.yaml) for the full YAML reference — all 62 element kinds, 11 relationship kinds, 6 viewpoint kinds, and validation rules.
> **Viewpoints:** See [`docs/viewpoints.md`](docs/viewpoints.md) for the viewpoints guide.

### Validate

```bash
archr validate --input model.yaml
```

Output (JSON):
```json
{
  "success": true,
  "errors": []
}
```

Exit code `0` = valid, `1` = validation errors, `2` = I/O or malformed YAML.

### Generate XML

```bash
archr generate --input model.yaml --output model.archimate
```

Produces an Open Exchange 3.0 XML file with UUIDs, layout coordinates, and one diagram view per viewpoint (or a single "Default View" when no viewpoints are defined) — ready to import into [Archi](https://www.archimatetool.com).

### Parse XML back

```bash
archr parse --input model.archimate --output model.yaml
```

### Diff models

```bash
archr diff --old existing.archimate --new updated.yaml
```

Output (JSON):
```json
{
  "added": ["NewService"],
  "removed": ["LegacyApp"],
  "modified": ["Customer"]
}
```

## CLI Reference

| Command | Args | Exit Codes |
|---------|------|------------|
| `validate` | `--input <yaml>` | 0 valid, 1 invalid, 2 I/O |
| `generate` | `--input <yaml> --output <xml>` | 0 success, 2 error |
| `parse` | `--input <xml> --output <yaml>` | 0 success, 2 error |
| `diff` | `--old <xml> --new <yaml>` | 0 success, 2 error |
| `--version` | — | `archr 1.1.0` |

**Conventions:** Data goes to stdout. Diagnostics go to stderr.

## ArchiMate 3.2 Rules

### Layers (8)

| Layer | Example Elements |
|-------|-----------------|
| Motivation | Goal, Requirement, Driver, Stakeholder |
| Strategy | Resource, Capability, ValueStream, CourseOfAction |
| Business | BusinessActor, BusinessProcess, BusinessService, Product |
| Application | ApplicationComponent, ApplicationService, DataObject |
| Technology | Node, Device, Artifact, CommunicationNetwork |
| Physical | Equipment, Facility, Material, DistributionNetwork |
| Implementation | WorkPackage, Deliverable, Plateau, Gap |
| Other | Grouping, Location, Junctions |

### Relationship Constraints

| Relationship | Rule |
|-------------|------|
| Composition, Aggregation, Assignment | Same layer |
| Realization | Same layer + cross-layer (App→Business, Tech→App, etc.) |
| Serving | Descending: Tech→App→Business→Strategy |
| Access | Application ↔ Technology, Application ↔ Business |
| Association, Influence | Any layer to any layer |
| Triggering, Flow | Same layer |
| Specialization | Same layer |
| Motivation → Core | Only Association |

Validation is **data-driven** — no hardcoded `match` statements. Rules live in a `const ALLOWED: &[(ElementLayer, RelationKind, ElementLayer)]` table.

## Project Structure

```
archr/
├── Cargo.toml                    # Workspace root
├── crates/
│   └── archr-core/
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs            # Module re-exports
│           ├── model.rs          # Arena-based graph (62 element + 11 relation types)
│           ├── main.rs           # clap CLI (validate/generate/parse/diff)
│           ├── validate.rs       # Data-driven derivability matrix
│           ├── diff.rs           # Model diff by element name
│           ├── layout.rs         # Topological grid layout (petgraph)
│           └── io/
│               ├── yaml.rs       # YAML parse + serialize + schema validation
│               └── xml.rs        # Open Exchange XML bidirectional I/O
├── docs/
│   ├── schema.yaml              # YAML reference schema
│   └── viewpoints.md            # Viewpoints guide
├── skill/
│   ├── SKILL.md                  # Agent Skill spec (frontmatter + instructions)
│   ├── scripts/
│   │   └── archr.py              # PEP 723 Python wrapper (stdlib only)
│   └── references/
│       └── ARCHIMATE_RULES.md    # Derivability rules reference
├── tests/
│   ├── fixtures/                 # YAML test scenarios
│   └── e2e.sh                     # End-to-end test suite
└── .github/workflows/            # CI: build-rust, test-skill, e2e-test
```

## Roadmap

### Status
**v1.0.0 core complete** — Full ArchiMate 3.2 validation, XML/YAML I/O, model diff, and CLI.

### Next Moves
- **LLM YAML robustness**: Improve schema validation for LLM-generated YAML (better error messages, partial parsing)
- **Layout algorithm correctness**: Refine grid placement for complex diagrams
- **Real-world XML parser resilience**: Enhance XML parsing for malformed but valid `.archimate` files
- **Performance optimization**: Reduce memory usage and improve startup time
- **Documentation improvements**: More examples, tutorials, and best practices

### Future Features
- **Web API**: RESTful interface for programmatic access
- **GUI**: Desktop application for model visualization
- **Plugin system**: Extend functionality with user-defined plugins
- **Cloud integration**: Support for cloud-based model storage and collaboration

### Community Feedback
We welcome feature requests and feedback. Please open an issue or join our [Discussions](https://github.com/haquiticos/archr/discussions).

## Agent Skill
The `skill/` directory contains an Agent Skill for AI assistants (Claude Code, VS Code Copilot, OpenAI Codex). The Python wrapper (`archr.py`) is self-contained — stdlib only, no pip install.

```bash
# Check the binary version is compatible
python3 skill/scripts/archr.py --version

# Validate through the wrapper
python3 skill/scripts/archr.py validate model.yaml

# Generate XML
python3 skill/scripts/archr.py generate model.yaml --output model.archimate
```

Set `ARCHR_BIN` to point to a custom binary location if `archr` isn't in `PATH`.



## Dependencies

| Crate | Purpose |
|-------|---------|
| serde + serde_yaml | YAML (de)serialization |
| serde_json | JSON output for validation/diff |
| quick-xml | Open Exchange XML I/O |
| clap | CLI argument parsing |
| petgraph | Topological sort + connected components |
| uuid | v4 UUIDs for XML identifiers |
| thiserror | Error types |

## License

MIT
