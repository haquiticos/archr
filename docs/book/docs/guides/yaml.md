# Define a Model (YAML)

```yaml
model:
  name: My Model
  elements:
    - id: actor_001
      name: Customer
      kind: BusinessActor
  relationships:
    - id: rel_001
      source: actor_001
      target: app_001
      kind: Serving
```