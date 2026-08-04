# generate

Generate Open Exchange XML (`.archimate`) from a human-friendly YAML model.

```bash
archr generate --input model.yaml --output model.archimate
```

## Output

A `.archimate` file with:

- Stable UUIDs for every element and relationship
- Topological grid layout coordinates
- A default diagram view

The file imports directly into [Archi](https://www.archimatetool.com).

## Exit codes

| Code | Meaning |
|------|---------|
| `0` | XML written successfully |
| `2` | I/O error or invalid YAML |
