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

### Key Constraints
| Layer Pair | Allowed Relations |
|------------|-------------------|
| Same Layer | Composition, Aggregation, Assignment, Realization |
| Motivation → Core (Business/App/Technology) | Only Association |
| Core → Same Layer (Business/App/Technology) | All 11 types |
| Serving | Serves Core (downward) |
| Access | Accesses Infrastructure (downward) |
| Influence | Motivation/Core only (downward) |
| Association | Any layer pair |

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Validation error (JSON output on stdout) |
| 2 | File I/O / YAML malformed / binary not found |
| 3 | Binary version incompatible (requires ≥1.0.0) |
| 4 | Subprocess timeout |
| 64 | Invalid arguments |
