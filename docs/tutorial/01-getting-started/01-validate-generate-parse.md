# Chapter 1: Model Validation, Generation, and Parsing

**Part of Tutorial 1: Getting Started — Self-Modeling archr**

This chapter deep-dives into the three core archr commands: `validate`, `generate`, and `parse`. You'll learn how they operate on YAML models and why `validate` is the foundation of the self-modeling workflow.

Estimated time: 15 minutes

## Overview

Before the agent models archr's architecture, you must understand the commands it will use:

- **`validate`** — verifies that a YAML model adheres to the YAML schema and ArchiMate 3.2 rules. Returns JSON on stdout with `success` and `errors`.
- **`generate`** — converts a valid YAML model into an ArchiMate 3.0 XML file. Used to visualize the model in Archi.
- **`parse`** — reads an ArchiMate XML file and outputs a canonical YAML model. Useful for round-tripping and diffing.

## What `validate` does (primer)

`validate` is the **safety gate** for every model. It runs both static schema checks and dynamic ArchiMate rules.

### Example: valid model

Create a `model.yaml` with a valid simple model:

```yaml
model:
  name: Simple Model
  elements:
    - id: actor_001
      name: Customer
      kind: BusinessActor
    - id: app_001
      name: Order Processing
      kind: ApplicationComponent
    - id: fn_001
      name: Create Order
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

Run validation:

```bash
archr validate --input model.yaml
```

Output (exit code 0):
```json
{"success": true, "errors": []}
```

### Example: invalid relationship

Introduce an invalid relationship:

```yaml
relationships:
  - id: rel_invalid
    source: actor_001
    target: app_001
    kind: Serving
```

`Serving` is a **descending-only** relationship. You cannot have `BusinessActor Serving ApplicationComponent` — that's an upward movement, which violates ArchiMate.

Re-run validation:

```bash
archr validate --input model.yaml
```

Output (exit code 1):
```json
{
  "success": false,
  "errors": [
    {
      "code": "InvalidRelationship",
      "message": "Cannot use Serving relationship between BusinessActor and ApplicationComponent — Serving only descends (Physical→Technology→{Application,Business}→{Strategy})",
      "source": "actor_001",
      "target": "app_001",
      "kind": "Serving"
    }
  ]
}
```

The agent will read this error, consult the **Derivability Rules** table in `skill/SKILL.md`, and pick a valid relationship type for those layers. Common choices:
- `Assignment` (Actor→Function) — valid
- `Association` (Actor→Component) — valid
- `Realization` (Component→Function) — valid

### Example: unknown element kind

Change the element kind to something invalid:

```yaml
kind: InvalidKind123
```

Re-run validation:

```bash
archr validate --input model.yaml
```

Output:
```json
{
  "success": false,
  "errors": [
    {
      "code": "UnknownKind",
      "message": "element actor_001 has invalid kind InvalidKind123",
      "element_source": "actor_001",
      "suggestion": "Use a valid ArchiMate 3.2 kind: BusinessActor, BusinessProcess, BusinessFunction, ApplicationComponent, ApplicationInterface, etc."
    }
  ]
}
```

The agent fixes this by choosing a valid kind like `BusinessActor` or `BusinessFunction`.

### Example: undefined reference

Reference a non-existent element:

```yaml
relationships:
  - id: rel_missing
    source: actor_001
    target: missing_element_123
    kind: Assignment
```

Re-run validation:

```bash
archr validate --input model.yaml
```

Output:
```json
{
  "success": false,
  "errors": [
    {
      "code": "UndefinedId",
      "message": "element rel_missing references undefined target 'missing_element_123'",
      "target": "missing_element_123"
    }
  ]
}
```

The agent fixes this by:
1. Defining a new element with that ID, or
2. Correcting the `target` to an existing element ID.

## What `generate` does

`generate` **produces output** — it writes an ArchiMate 3.0 XML file that can be opened in Archi (https://www.archimatetool.com).

### Usage

```bash
archr generate --input model.yaml --output model.archimate
```

**Requirements**:
- `model.yaml` must pass `validate` (exit code 0).
- The YAML must follow the schema (elements need `id`, `name`, `kind`; relationships need `id`, `source`, `target`, `kind`).

### Expected output

`model.archimate` will be an XML file like:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<archimate:model xmlns:archimate="http://www.archimatetool.com/archimate" name="Simple Model" version="3.0">
  <archimate:businessActor id="actor_001" name="Customer">
    <archimate:assignment id="rel_001" name="Assignment">
      <archimate:businessFunction id="fn_001" name="Create Order"/>
    </archimate:assignment>
  </archimate:businessActor>
  <archimate:applicationComponent id="app_001" name="Order Processing"/>
  <archimate:businessFunction id="fn_001" name="Create Order">
    <archimate:realization id="rel_002" name="Realization">
      <archimate:applicationComponent id="app_001" name="Order Processing"/>
    </archimate:realization>
  </archimate:businessFunction>
</archimate:model>
```

> **Tip**: You can also use the skill wrapper:
> ```bash
> python3 skill/scripts/archr.py generate model.yaml --output model.archimate
> ```

## What `parse` does

`parse` **converts XML to YAML**. It's useful for:
- Round-tripping (YAML → XML → YAML) to confirm schema preservation.
- Running `diff` on two XML files to see what changed (parsed into YAML for readability).

### Usage

```bash
archr parse --input model.archimate --output model.yaml
```

**Requirements**:
- `model.archimate` must be a valid ArchiMate 3.0 XML file.

### Expected output

`model.yaml` will be a YAML model identical to the input (modulo whitespace and order).

> **Note**: The agent's self-model workflow does not use `parse` directly, but it may use it indirectly to verify round-tripping during diff/bug-fixing in later tutorials.

## Summary of agent expectations

When the agent models archr's architecture in Tutorial 1, it will:
1. **Generate `model.yaml`** based on the source code inspection.
2. **Run `validate`** and parse the JSON output.
3. **Iterate on errors** until `{"success": true, "errors": []}`.
4. **If the tutorial optional section is selected**, run `generate` and open the output in Archi for visualization.

The key learning is that `validate` is both a validator and a **error signal**. The agent will treat each error as a concrete instruction on what to fix (kind, reference, uniqueness).

## Troubleshooting

### `archr: command not found`

Ensure the `archr` binary is in `PATH` or use `ARCHR_BIN`:

```bash
export ARCHR_BIN=/path/to/archr
archr validate --input model.yaml
```

### `validate` returns `{"success": false, "errors": [...]}`

Check the error message for the `code` field:
- `UnknownKind` — fix the element or relationship `kind`.
- `InvalidRelationship` — switch to a valid kind for those layers (see Derivability Rules table).
- `UndefinedId` — add the missing element or fix the reference.
- `DuplicateId` or `InvalidId` — ensure all IDs are unique and valid (alphanumeric, no spaces).

### `generate` fails

`generate` will only succeed if `validate` succeeds. Fix any errors in the YAML before generating XML.

### `parse` fails

Ensure the XML file is valid ArchiMate 3.0. Check for:
- Missing root `<archimate:model>` tag.
- Invalid namespaces or `xmlns` attributes.
- Malformed XML (unmatched tags, missing closing quotes).
