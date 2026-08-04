# Getting Started

## Step 1: Define a model

```yaml
model:
  name: Simple CRM
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

## Step 2: Validate

```bash
archr validate --input model.yaml
```

## Step 3: Generate

```bash
archr generate --input model.yaml --output model.archimate
```