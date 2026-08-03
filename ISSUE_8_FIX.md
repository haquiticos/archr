# Issue #8 Fix: `validate --format` flag removed (was a no-op)

## Problem
The `--format` argument on `archr validate` was accepted by the CLI but never
used. `run_validate` received it as `_format` and unconditionally emitted JSON:

```rust
// main.rs
fn run_validate(input_path: &str, _format: &str) -> ExitCode { ... }
```

The README documented `--format json` as if multiple formats existed, yet JSON
was the only output path. Passing `--format sarif` (or any value) was silently
accepted and produced JSON anyway. The Python wrapper always passed
`--format json`, masking the dead flag.

## Root Cause
The flag was declared as a free-form `String` field on the `Validate` clap
subcommand with `default_value = "json"`, but the value was discarded in
`run_validate` (binding named `_format`).

## Decision
Per the issue's suggested fix, **remove the `--format` flag entirely** rather
than implement additional formats. JSON is the only output today, so exposing a
flag that implies a choice is misleading. Every CLI surface, the Python wrapper,
and the docs have been updated to drop the flag.

Should a second format (e.g. `text`, `sarif`) be added later, reintroduce
`--format` as a typed `enum` rather than a free-form `String` so that invalid
values are rejected by clap at parse time.

## Changes Made

### `crates/archr-core/src/main.rs`
- Removed the `format: String` field and its `#[arg(...)]` attribute from the
  `Validate` variant of the `Commands` enum.
- Updated the `match` in `main()` to destructure only `input`.
- Changed `run_validate` signature from `(input_path: &str, _format: &str)` to
  `(input_path: &str)`.

### `skill/scripts/archr.py`
- `cmd_validate` no longer passes `--format json` to the archr subprocess
  (JSON remains the sole output).

### `README.md`
- CLI reference table: `validate` row now lists only `--input <yaml>`.

### `docs/archimate_implementation_guide.md`
- Removed `--format json` from the flow diagram, the usage examples, the API
  contract table, the illustrative Rust `Cli` enum, and the Python example.
- Removed the `format` field + `match format.as_str()` branch from the
  illustrative Rust snippet.

## Verification

### Build
```bash
$ cargo build --release
   Finished release
```

### Flag is now rejected
```bash
$ ./target/release/archr validate --input tests/fixtures/valid.yaml --format json
error: unexpected argument '--format' found
```
(JSON is still produced when the flag is omitted.)

### E2E suite
```
Results: N passed, 0 failed
ALL TESTS PASSED
```
(validate cases pass without `--format`.)

### Manual
```bash
$ ./target/release/archr validate --input tests/fixtures/valid.yaml
{
  "success": true,
  "errors": []
}
```

## Verification
- ✅ `--format` is rejected by clap (no silent no-op)
- ✅ JSON output unchanged when the flag is omitted
- ✅ Python wrapper still validates successfully (stdout is JSON)
- ✅ All e2e tests pass
- ✅ No breaking changes to the documented happy path (`validate --input <yaml>`)
