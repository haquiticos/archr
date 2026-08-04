# Define a model (YAML)

A model is a single YAML file with a `model` root key: `name`, `elements`, and `relationships`.

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

## Elements

Each element has:

- `id` — stable identifier, unique within the model
- `name` — human-readable label
- `kind` — one of the 62 ArchiMate 3.2 element kinds (see [Rules](./../reference/rules.md))

## Relationships

Each relationship has:

- `id` — stable identifier
- `source`, `target` — element `id`s
- `kind` — one of the 11 relationship types

The validator enforces the derivability matrix: which layers a relationship can connect. See [ArchiMate 3.2 Rules](./../reference/rules.md).

## Validate then generate

```bash
archr validate --input model.yaml
archr generate --input model.yaml --output model.archimate
```
