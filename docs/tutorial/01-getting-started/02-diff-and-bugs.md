# Chapter 2: Diff and Bugs — Demonstration

**Part of Tutorial 1: Getting Started — Self-Modeling archr**

This chapter demonstrates the **`diff` command** and shows you exactly how a specific bug appears in archr's source: `Serving Implementation→Application` is rejected. You'll see the agent use `diff` to discover, diagnose, and fix the bug.

Estimated time: 20 minutes

## Overview

In Tutorial 1, the agent will produce a valid self-model of archr. In this chapter, we intentionally introduce a **bug** in the model to show how `diff` and the error messages work.

The bug:
- The agent mistakenly uses `Serving` for `WorkPackage(Implementation) → ApplicationService(Application)`.
- The agent then iterates on `validate` errors until clean.

You'll learn:
- How `diff` compares two YAML models and highlights differences.
- How to read `INVALID_RELATIONSHIP` errors from `validate`.
- How to consult the **Derivability Rules** table to fix the bug.

## Quick primer: the `diff` command

`diff` compares two YAML models (or two XML files) and outputs the set of differences.

### Usage

```bash
archr diff --old model_v1.yaml --new model_v2.yaml
```

**Requirements**:
- Both files must be valid YAML (or valid XML).
- `diff` parses the models into a canonical internal representation and then prints:
  - Added elements/relationships
  - Removed elements/relationships
  - Changed elements/relationships (differences in `id`, `name`, or `kind`)

### Example

Assume `model_v1.yaml` is a "wrong" model (with the Serving bug):

```yaml
model:
  name: Broken Model
  elements:
    - id: wp_001
      name: archr v1.0.0 Release
      kind: WorkPackage
    - id: svc_001
      name: Validate CLI
      kind: ApplicationService
    - id: app_002
      name: Validate Module
      kind: ApplicationComponent
  relationships:
    - id: rel_wrong
      source: wp_001
      target: svc_001
      kind: Serving          # ← BUG: Serving cannot descend from Implementation to Application
```

Now let `model_v2.yaml` be a "fixed" model:

```yaml
model:
  name: Fixed Model
  elements:
    - id: wp_001
      name: archr v1.0.0 Release
      kind: WorkPackage
    - id: svc_001
      name: Validate CLI
      kind: ApplicationService
    - id: app_002
      name: Validate Module
      kind: ApplicationComponent
  relationships:
    - id: rel_wrong
      source: wp_001
      target: svc_001
      kind: Realization      # ← FIX: Use Realization for Implementation→Application
```

Run diff:

```bash
archr diff --old model_v1.yaml --new model_v2.yaml
```

Output (simplified):

```diff
- (none)
+ [Added relationships]
  + rel_wrong (wp_001 → svc_001, kind: Realization) [changed]
- [Removed relationships]
  - rel_wrong (wp_001 → svc_001, kind: Serving) [removed]
```

You can see exactly that one relationship changed its kind from `Serving` to `Realization`.

## The Serving bug in archr's source

This bug is **not a user error** — it's a bug in the Rust implementation: `validate.rs::ALLOWED` (const slice, ~line 25) has **NO Serving triple with Implementation as source**.

The allowed triples (relevant parts):

```rust
// Physical→Technology
(Physical, Technology, Serving)

// Technology→{Application, Business}
(Technology, Application, Serving),
(Technology, Business, Serving),

// Application→{Business, Strategy}
(Application, Business, Serving),
(Application, Strategy, Serving),

// Business→Strategy
(Business, Strategy, Serving)
```

Missing:
- **(Implementation, Application, Serving)** ← This is the bug.

Therefore:
- `Serving` can go from `Technology` to `Application` or `Application` to `Strategy`.
- `Serving` **cannot** go from `Implementation` to `Application`.

That means if you have a `WorkPackage` (Implementation layer) trying to serve an `ApplicationService` (Application layer), `validate` will reject it with:

```json
{
  "success": false,
  "errors": [
    {
      "code": "InvalidRelationship",
      "message": "Cannot use Serving relationship between WorkPackage and ApplicationService — Serving only descends (Physical→Technology→{Application,Business}→{Strategy})",
      "source": "wp_001",
      "target": "svc_001",
      "kind": "Serving"
    }
  ]
}
```

The message explicitly states the constraint: **Serving only descends**.

## Demonstrating the bug

1. Create `model_broken.yaml` with the Serving relationship:

```yaml
model:
  name: Bug Demonstration
  elements:
    - id: wp_001
      name: archr v1.0.0 Release
      kind: WorkPackage
    - id: svc_001
      name: Validate CLI
      kind: ApplicationService
    - id: app_002
      name: Validate Module
      kind: ApplicationComponent
  relationships:
    - id: rel_wrong
      source: wp_001
      target: svc_001
      kind: Serving
```

2. Validate it:

```bash
archr validate --input model_broken.yaml
```

3. You'll see:

```json
{
  "success": false,
  "errors": [
    {
      "code": "InvalidRelationship",
      "message": "Cannot use Serving relationship between WorkPackage and ApplicationService — Serving only descends (Physical→Technology→{Application,Business}→{Strategy})",
      "source": "wp_001",
      "target": "svc_001",
      "kind": "Serving"
    }
  ]
}
```

4. Fix it by changing `kind: Serving` to `kind: Realization`:

```yaml
relationships:
  - id: rel_wrong
    source: wp_001
    target: svc_001
    kind: Realization
```

5. Re-validate:

```bash
archr validate --input model_fixed.yaml
```

6. Now you get:

```json
{"success": true, "errors": []}
```

## How the agent uses this

In Tutorial 1, the agent will:
1. Inspect `crates/archr-core/src/` and discover that:
   - `WorkPackage` is in `validate.rs` (Implementation layer).
   - `ApplicationService` is a CLI subcommand (Application layer).
2. Introduce a `Serving` relationship by default (thinking that "serves" means "supports").
3. Run `validate`, get the `INVALID_RELATIONSHIP` error.
4. Consult the **Derivability Rules** table in `SKILL.md`:
   - Find that `Serving` only descends from lower to higher layers.
   - Switch to `Realization` (Implementation→Application is allowed).
5. Re-validate and confirm success.

This pattern repeats for other invalid relationships (e.g., `Aggregation` between same-layer elements is rejected because aggregation only exists for lower layers; cross-layer aggregations are handled differently in ArchiMate 3.2).

## Additional demo: same-layer invalid relationships

Another common error is using `Aggregation` between **same-layer** elements. In ArchiMate, `Aggregation` (and `Composition`) only exist for lower layers (Physical, Technology, Business). They do **not** exist for the top layer (Strategy) or the Application layer.

### Example

```yaml
relationships:
  - id: rel_agg
    source: app_001
    target: app_002
    kind: Aggregation    # ← BUG: Aggregation cannot exist at Application layer
```

Re-validate:

```bash
archr validate --input model.yaml
```

You'll get:

```json
{
  "success": false,
  "errors": [
    {
      "code": "InvalidRelationship",
      "message": "Cannot use Aggregation relationship between ApplicationComponent and ApplicationComponent — Aggregation only exists for lower layers (Physical, Technology, Business)",
      "source": "app_001",
      "target": "app_002",
      "kind": "Aggregation"
    }
  ]
}
```

### Fix

Use a same-layer valid relationship instead:
- `Association` (any two elements, same or different layers)
- `Realization` (for Application→Application when one implements another)
- `Composition` (only if you have a lower-layer aggregation pattern)

## Summary

- **`diff`** is the command for comparing two models.
- **`validate`** is the safety gate that rejects invalid relationships, kinds, references, and duplicates.
- **The Serving bug** demonstrates that `Serving` is descending-only and cannot be used from Implementation to Application.
- **Derivability Rules** table in `SKILL.md` is your reference for fixing `InvalidRelationship` errors.

## Troubleshooting

### `archr: command not found`

```bash
export ARCHR_BIN=/path/to/archr
archr diff --old model_v1.yaml --new model_v2.yaml
```

### `diff` reports empty set of differences

Ensure both YAML files are valid and contain models (i.e., have an `elements` list). Empty files or files with only metadata will produce no diff.

### `validate` returns `INVALID_RELATIONSHIP`

1. Check the error message for the layers involved (e.g., "between WorkPackage and ApplicationService").
2. Consult the **Derivability Rules** table in `SKILL.md`.
3. Pick a valid relationship kind for those layers:
   - Cross-layer: `Assignment`, `Realization`, `Association`, `Serving` (if lower→higher).
   - Same-layer: `Association`, `Realization`, `Composition` (if valid for that layer).

### Same-layer `Aggregation` error

Aggregation is only valid for Physical, Technology, and Business layers. For Application, use `Association` or `Realization`.
