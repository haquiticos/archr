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
<?xml version="1.0" encoding="UTF-8"?>
<root>
  <element id="el-1" type="BusinessActor">
    <name>Customer Service</name>
  </element>
  <element id="el-2" type="ApplicationComponent">
    <name>CRM System</name>
  </element>
  <element id="el-3" type="BusinessFunction">
    <name>Process Order</name>
  </element>
  <relation id="rel-1" type="Assignment">
    <source idref="el-1"/>
    <target idref="el-3"/>
  </relation>
  <relation id="rel-2" type="Realization">
    <source idref="el-2"/>
    <target idref="el-3"/>
  </relation>
</root>
```

## Relationship Rules (ArchiMate 3.2)

**Note**: These rules are derived from the implementation. See [docs/SPEC.md](../docs/SPEC.md) for authoritative documentation.

### Layers (8)
1. **Motivation** - Goals, requirements, drivers
2. **Strategy** - Roadmaps, principles, KPIs
3. **Business** - Business processes, functions, actors
4. **Application** - Application components, interfaces
5. **Technology** - Infrastructure, runtime, network
6. **Physical** - Hardware, locations, facilities
7. **Implementation** - Projects, deliverables, migration
8. **Other** - Concepts, principles

### Relationship Types (11)
- **Structural** (4): Composition, Aggregation, Assignment, Realization
- **Dependency** (4): Serving, Access, Influence, Association
- **Dynamic** (2): Triggering, Flow
- **Other** (1): Specialization

### Key Constraints (From Implementation)

| Layer Pair | Allowed Relations |
|------------|-------------------|
| Same Layer | All 11 types (except limitations apply per type) |
| Same Layer → Same Layer | All 11 types with restrictions |
| Same Layer → Different Layer | Limited by relationship type |

### Detailed Rules

See [docs/SPEC.md](../docs/SPEC.md) for complete derivability rules:

- **Composition**: Any layer → Any layer (composite element source)
- **Aggregation**: Same layer only
- **Assignment**: BusinessActor → BusinessFunction only
- **Realization**: ApplicationComponent → BusinessFunction, BusinessProcess
- **Serving**: BusinessService → BusinessFunction
- **Access**: ApplicationComponent → DataObject
- **Influence**: Motivation → Same layer only
- **Association**: Any layer → Any layer
- **Triggering**: Same layer only
- **Flow**: Same layer only
- **Specialization**: Any layer → Any layer
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
