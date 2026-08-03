# Issue #8 Fix: Malformed YAML No Longer Misreported as InvalidId

## Problem
When YAML files had malformed syntax (e.g., incorrect indentation), the archr validation command would return both:
- `MalformedYaml` error (correct - indicates YAML parsing error)
- `InvalidId` and other schema errors (incorrect - schema validation errors should only appear when YAML is valid)

## Root Cause
The `run_validate` function in `crates/archr-core/src/main.rs` was unconditionally processing all errors from `yaml::parse_yaml()`, including schema validation errors when the YAML itself was malformed.

## Solution
Modified the error handling logic in `run_validate` to:
1. Check if any error in the result set is a `MalformedYaml` error
2. If yes, filter the error list to show ONLY the `MalformedYaml` errors (dropping all schema errors)
3. If no `MalformedYaml` error, show all schema errors normally

## Changes Made

### File: `crates/archr-core/src/main.rs`

1. **Added import** (line 11):
   ```rust
   use archr_core::io::yaml::SchemaError;
   ```

2. **Modified error handling** (lines 92-131):
   - Added check for `MalformedYaml` error presence
   - Implemented conditional error filtering:
     - Show only `MalformedYaml` errors when YAML is malformed
     - Show all schema errors when YAML is valid but schema validation fails

## Test Results

### Manual Testing
```bash
$ ./target/release/archr validate --input tests/fixtures/malformed.yaml --format json
{
  "errors": [
    {
      "code": "MalformedYaml",
      "message": "YAML parsing error: mapping values are not allowed in this context at line 6 column 13"
    }
  ],
  "success": false
}
```

### E2E Tests (23 tests)
```
Results: 23 passed, 0 failed
ALL TESTS PASSED
```

Key test: `malformed.yaml is not misreported as InvalidId` ✅

### Unit Tests (60 tests)
```
test result: ok. 60 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

All tests pass, confirming the fix doesn't break existing functionality.

## Verification
- ✅ Malformed YAML now only reports `MalformedYaml` errors
- ✅ Valid YAML with schema errors still reports schema errors correctly
- ✅ All 23 e2e tests pass
- ✅ All 60 unit tests pass
- ✅ No breaking changes to existing functionality
