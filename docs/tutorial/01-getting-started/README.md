# Tutorial 1: Getting Started — Self-Modeling archr

**Part of the Self-Modeling series** — teaches you to model archr's own architecture using omp and the archr skill.

Estimated time: 30 minutes

## Prerequisites

Complete [Shared Setup](../shared/01-setup.md) before starting:
- Install the `archr` binary (one of the three installation options)
- Install omp (https://omp.sh/)
- Install the archr skill via `bash skill/install.sh`

## Overview

This tutorial runs **omp** inside the archr repository. The agent, using the loaded archr skill, will:
1. Read `crates/archr-core/src/` to understand the codebase
2. Draft a self-model `model.yaml` following the skill's YAML schema
3. Run the skill's `validate` and iterate on JSON errors until validation passes (`{"success": true, "errors": []}`)

The YAML is **agent-generated**, not hand-written. Success is measured by the validate JSON, not by matching a reference file.

## Manual primer: what validate does

Before letting the agent do the work, run `archr validate` yourself to understand the command.

Create a tiny `model.yaml` (save as `model.yaml`):

```yaml
model:
  name: Test Architecture
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

Run validation:

```bash
archr validate --input model.yaml
```

If valid, you see:
```json
{"success": true, "errors": []}
```

Now introduce an error — change `fn_001`'s kind to an invalid name:

```yaml
kind: InvalidKind123
```

Re-run validation:

```bash
archr validate --input model.yaml
```

You'll get:
```json
{
  "success": false,
  "errors": [
    {
      "code": "UnknownKind",
      "message": "element fn_001 has invalid kind InvalidKind123",
      "element_source": "fn_001",
      "suggestion": "Use a valid ArchiMate 3.2 kind: BusinessActor, BusinessProcess, BusinessFunction, ApplicationComponent, ApplicationInterface, etc."
    }
  ]
}
```

That's the format the agent will parse and fix automatically in the next section.

## Install the skill

If you haven't already, install the archr skill so omp can discover it:

```bash
bash skill/install.sh          # project-level: .agents/skills/archr-skill
# or
bash skill/install.sh --user   # user-level:    ~/.omp/agent/skills/archr-skill
```

Confirm discovery:

```bash
read skill://archr-skill/SKILL.md
```

The output should be the contents of `skill/SKILL.md`.

## Self-model with omp

Run omp **inside the archr repository** (this is "passing the repo to omp" as a working directory).

```bash
omp
```

Once inside omp, give the agent the following prompt:

> Using the archr skill, model archr's own architecture from `crates/archr-core/src/` into `model.yaml`, then run the skill's validate and fix every error until validation passes.

### What the agent will do

The agent follows the **Self-Modeling Workflow** section in `SKILL.md`:

1. **Read the source** — it inspects `crates/archr-core/src/` to identify real elements:
   - `ApplicationComponent` for each module (model, validate, io, layout, diff, cli)
   - `ApplicationService` for each CLI subcommand (validate, generate, parse, diff)
   - `DataObject` for the model graph, the `ALLOWED` matrix, and the YAML I/O structures
   - `Plateau` for the current release (v1.0.0)
   - `WorkPackage` for release deliverables
   - `Deliverable` for the binary artifacts

2. **Draft `model.yaml`** — it creates a YAML model per the schema in SKILL.md (each element needs a unique `id`, `name`, and valid `kind`; relationships need `source`, `target`, and a valid `kind`).

3. **Validate** — it runs:
   ```bash
   python3 .agents/skills/archr-skill/scripts/archr.py validate model.yaml
   ```
   (or `archr validate --input model.yaml` if the binary is in PATH) and parses the JSON on stdout.

4. **Fix errors iteratively** — for each error, it consults the **Derivability Rules** table in SKILL.md:
   - `INVALID_RELATIONSHIP` — the message is `"<source> cannot <rel> <target>"`. The agent picks a valid kind for those two element layers (e.g., cross-layer `WorkPackage(Implementation) → ApplicationService(Application)` cannot be `Serving`; it switches to `Realization` or `Association`).
   - `UnknownKind` — corrects the element/relationship `kind` to a valid ArchiMate 3.2 name.
   - `UndefinedId` — fixes the missing `source`/`target` reference.
   - `DuplicateId` — makes IDs unique.
   - `InvalidId` — removes spaces or empty IDs.
   - `MalformedYaml` — fixes YAML indentation/structure.

5. **Repeat** until exit code 0 and `{"success": true, "errors": []}`.

### Expected teaching moment

A natural model has a release `WorkPackage` (Implementation layer) deliver the `validate` `ApplicationService` (Application layer). The obvious relation is `Serving`, but `Serving` only descends (Physical→Technology→{Application,Business}→{Strategy}). `WorkPackage Serving ApplicationService` is rejected with `INVALID_RELATIONSHIP`. The agent fixes it to `Realization` (Implementation→Application is permitted) or `Association`.

You'll see the agent produce an error, then edit `model.yaml` to fix the relationship kind, re-validate, and continue until clean.

### Success criterion

The tutorial is complete when the agent returns:
```json
{"success": true, "errors": []}
```

The exit code of `archr validate --input model.yaml` is 0, and there are no errors in the JSON. No reference file is checked in; the YAML is the agent's output.

## Optional: visualize

Once validation passes, generate an ArchiMate diagram and open it in Archi:

```bash
archr generate --input model.yaml --output model.archimate
```

Then open `model.archimate` in [Archi](https://www.archimatetool.com) to visualize the architecture.

## Next steps

- Review the `model.yaml` the agent produced.
- Read the self-modeling loop in `skill/SKILL.md` → `## Self-Modeling Workflow`.
- Explore other tutorials in the Self-Modeling series:
  - [Tutorial 2: Diff and Bugs](../02-diff-and-bugs/README.md)
  - [Tutorial 3: Benchmarking](../03-benchmarking/README.md)
