# archr

A headless Rust engine for ArchiMate 3.2 — validate, manipulate, and export architecture models via CLI and YAML. Designed for AI agent integration.

## Features

- **Validate** ArchiMate 3.2 models against the full derivability ruleset (61 element types, 11 relationship types, 8 layers)
- **Generate** Open Exchange XML (`.archimate`) from human-friendly YAML
- **Parse** existing `.archimate` XML back into YAML
- **Diff** two models to detect added, removed, and modified elements
- **Automatic layout** — grid placement by topological layer (no Sugiyama)
- **Agent Skill** — PEP 723 Python wrapper for Claude Code, Copilot, and Codex

## Quick Start

### Build

```bash
cargo build --release
# Binary at target/release/archr
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
      source: actor_001
      target: app_001
      kind: Serving
```

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

Produces an Open Exchange 3.0 XML file with UUIDs, layout coordinates, and a default diagram view — ready to import into [Archi](https://www.archimatetool.com).

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
| `validate` | `--input <yaml> [--format json]` | 0 valid, 1 invalid, 2 I/O |
| `generate` | `--input <yaml> --output <xml>` | 0 success, 2 error |
| `parse` | `--input <xml> --output <yaml>` | 0 success, 2 error |
| `diff` | `--old <xml> --new <yaml>` | 0 success, 2 error |
| `--version` | — | `archr 1.0.0` |

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
│           ├── model.rs          # Arena-based graph (61 element + 11 relation types)
│           ├── main.rs           # clap CLI (validate/generate/parse/diff)
│           ├── validate.rs       # Data-driven derivability matrix
│           ├── diff.rs           # Model diff by element name
│           ├── layout.rs         # Topological grid layout (petgraph)
│           └── io/
│               ├── yaml.rs       # YAML parse + serialize + schema validation
│               └── xml.rs        # Open Exchange XML bidirectional I/O
├── skill/
│   ├── SKILL.md                  # Agent Skill spec (frontmatter + instructions)
│   ├── scripts/
│   │   └── archr.py              # PEP 723 Python wrapper (stdlib only)
│   └── references/
│       └── ARCHIMATE_RULES.md    # Derivability rules reference
├── tests/
│   ├── fixtures/                 # 8 YAML test scenarios
│   └── e2e.sh                     # End-to-end test suite
└── .github/workflows/            # CI: build-rust, test-skill, e2e-test
```

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

## Testing

```bash
# Unit tests (41 tests)
cargo test -p archr-core --lib

# End-to-end suite (19 tests, builds release binary)
bash tests/e2e.sh
```

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
