---
name: archr-skill
description: >
  Validate, generate, and manipulate ArchiMate 3.2 architecture models via a headless Rust engine.
  Use when the user asks to create, validate, or edit ArchiMate models, export to Open Exchange XML,
  or check element relationship derivability. Triggers: "archimate", "archi", "model validation",
  "generate .archimate", "architecture diagram".
compatibility: >
  Requires the `archr` Rust binary (v1.0.0+) in PATH or set via ARCHR_BIN env var.
  Python >= 3.10 for the wrapper script. No pip dependencies.
---

# archr Agent Skill

Validate, generate, and manipulate ArchiMate 3.2 architecture models via the `archr` Rust binary.

## Workflow

1. **Validate** (optional) - Check YAML syntax and ArchiMate rules before generation
2. **Generate** - Convert validated YAML to Open Exchange XML `.archimate` format
3. **Parse** (available via `archr` binary directly) - Reverse-engineer XML to YAML

## Example YAML Schema

```yaml
model:
  name: My Architecture
  elements:
    - id: actor_001
      name: Customer Service
      kind: BusinessActor
    - id: app_001
      name: CRM System
      kind: ApplicationComponent
    - id: fn_001
      name: Process Order
      kind: BusinessFunction
  relationships:
    - id: rel_001
      source: actor_001
      target: fn_001
      kind: Assignment
    - id: rel_002
      source: app_001
      target: fn_001
      kind: Realization
```

## Relationship Rules (ArchiMate 3.2)

**Note**: These rules are derived from the implementation. See [docs/SPEC.md](../docs/SPEC.md) for authoritative documentation.

### Layers (8)
1. **Motivation** - Goals, requirements, drivers
2. **Strategy** - Capabilities, resources, value streams
3. **Business** - Business processes, functions, actors
4. **Application** - Application components, interfaces
5. **Technology** - Infrastructure, runtime, network
6. **Physical** - Equipment, facilities, materials
7. **Implementation** - Projects, deliverables, migration
8. **Other** - Grouping, location, junctions

### Relationship Types (11)
- **Structural** (4): Composition, Aggregation, Assignment, Realization
- **Dependency** (4): Serving, Access, Influence, Association
- **Dynamic** (2): Triggering, Flow
- **Other** (1): Specialization

### Derivability Rules (from `validate.rs::ALLOWED`)

| Relationship | Allowed source → target |
|-------------|------------------------|
| Composition | Same layer only |
| Aggregation | Same layer only |
| Assignment | Same layer only |
| Realization | Same layer; upward crossing: Implementation→{Strategy,Business,App,Tech,Physical}, Technology→{Application,Business}, Application→Business |
| Serving | Descending chain: Physical→Technology, Technology→{Application,Business}, Application→{Business,Strategy}, Business→Strategy |
| Access | Bidirectional: Application↔Technology, Application↔Business, Application↔Application |
| Influence | Any layer → any layer |
| Association | Any layer → any layer |
| Triggering | Same layer only |
| Flow | Same layer only |
| Specialization | Same layer only |

For the full `ALLOWED` matrix (203 triples), see [docs/SPEC.md](../docs/SPEC.md).

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Validation error (JSON output on stdout) |
| 2 | File I/O / YAML malformed / binary not found |
| 3 | Binary version incompatible (requires ≥1.0.0) |
| 4 | Subprocess timeout |
| 64 | Invalid arguments |

## References

- **[docs/SPEC.md](../docs/SPEC.md)** — Authoritative, auto-generated spec
- **[validate.rs](../../crates/archr-core/src/validate.rs)** — Implementation details
- **[model.rs](../../crates/archr-core/src/model.rs)** — Element definitions
- **[archimate.ecore](./references/archimate.ecore)** — Archi metamodel (MIT license)
