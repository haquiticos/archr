# archr

> A headless Rust engine for ArchiMate 3.2 — validate, manipulate, and export architecture models via CLI and YAML. Designed for AI agent integration.

`archr` ships as a single, dependency-free executable. Install it with the install script, `cargo`, or download the binary directly on macOS, Linux, and Windows.

```bash
archr --version
# archr 1.0.0
```

## What it does

- **Validate** ArchiMate 3.2 models against the full derivability ruleset (62 element types, 11 relationship types, 8 layers)
- **Generate** Open Exchange XML (`.archimate`) from human-friendly YAML
- **Parse** existing `.archimate` XML back into YAML
- **Diff** two models to detect added, removed, and modified elements
- **Automatic layout** — grid placement by topological layer

## Highlights

- Single binary, zero runtime dependencies
- Data goes to stdout, diagnostics to stderr — scriptable and AI-friendly
- PEP 723 Python wrapper for Claude Code, Copilot, and Codex (stdlib only)

Ready? Head to [Installation](./installation.md).
