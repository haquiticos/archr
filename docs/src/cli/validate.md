# validate

Validate an ArchiMate 3.2 model against the full derivability ruleset.

```bash
archr validate --input model.yaml
```

## Result

JSON on stdout:

```json
{
  "success": true,
  "errors": []
}
```

On a failing model:

```json
{
  "success": false,
  "errors": [
    "Relationship rel_001 (Serving): layer mismatch Business -> Technology"
  ]
}
```

## Exit codes

| Code | Meaning |
|------|---------|
| `0` | model is valid |
| `1` | model has validation errors (errors printed in JSON) |
| `2` | I/O error or malformed YAML (diagnostic on stderr) |
