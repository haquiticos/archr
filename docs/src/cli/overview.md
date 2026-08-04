# CLI Overview

`archr` exposes four subcommands. Data goes to **stdout**; diagnostics go to **stderr**.

| Command | Args | Exit codes |
|---------|------|------------|
| `validate` | `--input <yaml>` | `0` valid, `1` validation errors, `2` I/O or malformed YAML |
| `generate` | `--input <yaml> --output <xml>` | `0` success, `2` error |
| `parse`    | `--input <xml> --output <yaml>` | `0` success, `2` error |
| `diff`     | `--old <xml> --new <yaml>` | `0` success, `2` error |
| `--version` | — | `archr 1.0.0` |

## Global help

```bash
archr --help
archr <command> --help
```

## Conventions

- **stdout** carries the structured result (JSON for validate/diff; file output for generate/parse).
- **stderr** carries human-readable diagnostics.
- Exit code `0` = success, `1` = business-level validation errors, `2` = I/O or malformed input.
