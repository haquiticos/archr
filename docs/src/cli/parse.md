# parse

Convert an existing Open Exchange XML (`.archimate`) back into human-friendly YAML.

```bash
archr parse --input model.archimate --output model.yaml
```

Use this to round-trip an Archi-authored model through `archr`, or to bootstrap YAML from an existing repository of `.archimate` files.

## Exit codes

| Code | Meaning |
|------|---------|
| `0` | YAML written successfully |
| `2` | I/O error or malformed XML |
