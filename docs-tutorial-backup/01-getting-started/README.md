# Tutorial 1: Getting Started

**Self-Modeling Series**: `archr` models its own architecture as the worked example.

**Estimated Time**: 30 minutes

**Prerequisites**:
- Complete [Shared Setup](../shared/01-setup.md) (install `archr`, verify version)

**Overview**:
Your first ArchiMate model with archr. You'll create a simple model (~30 elements), validate it, generate XML, parse it back, and use diff to detect changes.

## Table of Contents

1. [Introduction](#introduction)
2. [Create Your First Model](#create-your-first-model)
3. [Validate the Model](#validate-the-model)
4. [Generate Open Exchange XML](#generate-open-exchange-xml)
5. [Parse XML Back to YAML](#parse-xml-back-to-yaml)
6. [Diff Two Models](#diff-two-models)
7. [Next Steps](#next-steps)

## Introduction

This tutorial demonstrates the full workflow of archr: from YAML specification to validated, rendered models. You'll build a small subsystem of archr itself, applying the same principles the tool uses to process its own architecture.

## Create Your First Model

Create a file named `model.yaml`:

```yaml
model:
  name: archr-subsystem
  elements:
    # Motivation layer
    - id: goal_001
      name: "Model Self-Architecture"
      kind: Goal

    # Business layer
    - id: actor_001
      name: "Developer"
      kind: BusinessActor
    - id: process_001
      name: "Write YAML"
      kind: BusinessProcess
    - id: service_001
      name: "Validate"
      kind: BusinessService

    # Application layer
    - id: app_001
      name: "ArchiMate Validator"
      kind: ApplicationComponent
    - id: app_002
      name: "XML Generator"
      kind: ApplicationComponent
    - id: app_003
      name: "YAML Parser"
      kind: ApplicationComponent

    # Technology layer
    - id: node_001
      name: "Rust Engine"
      kind: Node
    - id: device_001
      name: "Server"
      kind: Device
    - id: artifact_001
      name: "Binary"
      kind: Artifact

    # Implementation layer
    - id: wp_001
      name: "Implement Validation"
      kind: WorkPackage
    - id: plateau_001
      name: "Release 1.0.0"
      kind: Plateau
    - id: gap_001
      name: "Enhancement Feature"
      kind: Gap

  relationships:
    # Goal to Business - use Association (any layer to any layer)
    - id: rel_001
      source: goal_001
      target: actor_001
      kind: Association

    # Business relationships - Assignment is same-layer only
    - id: rel_002
      source: actor_001
      target: process_001
      kind: Association
    - id: rel_003
      source: process_001
      target: service_001
      kind: Realization  # Same-layer: BusinessProcess → BusinessService

    # Application relationships - BusinessActor does not serve ApplicationComponent
    # (Serving direction: Strategy ← Business ← Application ← Technology ← Physical)
    - id: rel_004
      source: process_001
      target: app_001
      kind: Association  # BusinessProcess → ApplicationComponent (different layers, use Association)
    - id: rel_005
      source: app_001
      target: app_002
      kind: Composition  # Same-layer: ApplicationComponent → ApplicationComponent
    - id: rel_006
      source: app_002
      target: app_003
      kind: Realization  # Same-layer: ApplicationComponent → ApplicationComponent

    # Technology relationships - Assignment is same-layer only
    - id: rel_007
      source: app_003
      target: node_001
      kind: Access  # Application → Technology
    - id: rel_008
      source: node_001
      target: device_001
      kind: Composition  # Same-layer: Node → Device
    - id: rel_009
      source: node_001
      target: artifact_001
      kind: Realization  # Same-layer: Node → Artifact

    # Implementation relationships - Assignment is same-layer only
    - id: rel_010
      source: process_001
      target: wp_001
      kind: Association  # BusinessProcess → WorkPackage (different layers, use Association)
    - id: rel_011
      source: wp_001
      target: plateau_001
      kind: Assignment  # Same-layer: WorkPackage → Plateau
    - id: rel_012
      source: gap_001
      target: app_001
      kind: Association  # Gap → ApplicationComponent (different layers, use Association)
```

**Notes on the model**:
- ArchiMate relationships have layer constraints. `Assignment`, `Composition`, `Aggregation`, `Realization` (across same layer), `Triggering`, `Flow`, and `Specialization` are same-layer-only. `Serving` descends: Technology → Application → Business → Strategy (never the reverse).
- Cross-layer links use `Association` (or `Influence`), which are allowed between any two layers. For example, `BusinessProcess → WorkPackage` crosses Business → Implementation, so it uses `Association`, not `Assignment`.
- The file declares 13 elements and 12 relationships — enough to exercise validate, generate, parse, and diff without any validation error.

## Validate the Model

Run the validator:

```bash
archr validate --input model.yaml
```

**Expected Output**:
```json
{
  "success": true,
  "errors": []
}
```

Exit code `0` = valid, `1` = validation errors, `2` = I/O or malformed YAML.

If you see errors, check the relationship `kind` against the layer rules above — the most common mistake is using `Assignment` or `Serving` across layers.

## Generate Open Exchange XML

Generate an `.archimate` file:

```bash
archr generate --input model.yaml --output model.archimate
```

The generated file includes:
- UUIDs for all elements
- Layout coordinates (single column, topological order)
- Default diagram view
- Ready to import into [Archi](https://www.archimatetool.com)

**Note**: The layout is topological, not aesthetic. You'll see all elements stacked in a single column (`col=0`). For visually arranged diagrams, open the file in Archi and drag elements manually.

## Parse XML Back to YAML

Parse the generated XML back to YAML:

```bash
archr parse --input model.archimate --output model_parsed.yaml
```

Compare the original and parsed files:

```bash
diff model.yaml model_parsed.yaml
```

**Expected Result**: Semantically identical. The round-trip preserves all element names, types, and relationships, though formatting (indentation, order) may differ. Comments are removed, and elements are sorted alphabetically in the parsed file. The diff will therefore show many textual changes but no semantic additions or removals.

## Diff Two Models

Create a modified version:

```bash
cp model.yaml model_modified.yaml
# Edit model_modified.yaml: rename "ArchiMate Validator" to "ArchiMate Validator (NEW)"
```

Run diff:

```bash
archr diff --old model.archimate --new model_modified.yaml
```

**Expected Output**:
```json
{
  "added": ["ArchiMate Validator (NEW)"],
  "removed": ["ArchiMate Validator"],
  "modified": []
}
```

**Note**: `diff` compares elements by **name**, not ID. If you have duplicate element names in different subsystems, the diff may report false "no diff" when topology changes. See [issue #9](https://github.com/haquiticos/archr/issues/9) for details.

## Next Steps

- Continue to [Tutorial 2: Implementation and Benchmarking](../02-implementation-and-benchmark/README.md)
- Explore the [archr Core](https://github.com/haquiticos/archr/blob/main/crates/archr-core) source
- Try the [Agent Skill wrapper](../skill/SKILL.md)
