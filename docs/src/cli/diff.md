# diff

Diff an existing XML model against a new YAML model. Detects added, removed, and modified elements (matched by element name).

```bash
archr diff --old existing.archimate --new updated.yaml
```

## Result

JSON on stdout:

```json
{
  "added": ["NewService"],
  "removed": ["LegacyApp"],
  "modified": ["Customer"]
}
```

## Exit codes

| Code | Meaning |
|------|---------|
| `0` | diff produced successfully (model may differ) |
| `2` | I/O error or malformed input |
