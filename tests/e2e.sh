#!/usr/bin/env bash
# End-to-end test suite for archr.
# Tests all CLI subcommands against fixture files.
set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
FIXTURES="$SCRIPT_DIR/fixtures"

# Build the binary first.
echo "==> Building archr (release)..."
cd "$PROJECT_ROOT"
cargo build --release 2>/dev/null
BINARY="$PROJECT_ROOT/target/release/archr"

PASS=0
FAIL=0

assert_exit() {
    local expected="$1"
    local actual="$2"
    local label="$3"
    if [ "$expected" = "$actual" ]; then
        echo "  PASS: $label (exit $actual)"
        PASS=$((PASS + 1))
    else
        echo "  FAIL: $label (expected exit $expected, got $actual)"
        FAIL=$((FAIL + 1))
    fi
}

assert_contains() {
    local output="$1"
    local pattern="$2"
    local label="$3"
    if echo "$output" | grep -q "$pattern"; then
        echo "  PASS: $label"
        PASS=$((PASS + 1))
    else
        echo "  FAIL: $label (expected '$pattern' in output)"
        FAIL=$((FAIL + 1))
    fi
}

echo "==> Testing validate command..."

# valid.yaml → exit 0, success:true
OUTPUT=$("$BINARY" validate --input "$FIXTURES/valid.yaml" 2>/dev/null); RC=$?
assert_exit 0 "$RC" "validate valid.yaml"
assert_contains "$OUTPUT" '"success": true' "valid.yaml has success:true"

# orphan_id.yaml → exit 1, success:false
OUTPUT=$("$BINARY" validate --input "$FIXTURES/orphan_id.yaml" 2>/dev/null); RC=$?
assert_exit 1 "$RC" "validate orphan_id.yaml"
assert_contains "$OUTPUT" '"success": false' "orphan_id.yaml has success:false"

# invalid_rel.yaml → exit 1, INVALID_RELATIONSHIP
OUTPUT=$("$BINARY" validate --input "$FIXTURES/invalid_rel.yaml" 2>/dev/null); RC=$?
assert_exit 1 "$RC" "validate invalid_rel.yaml"
assert_contains "$OUTPUT" '"success": false' "invalid_rel.yaml has success:false"

# duplicate_id.yaml → exit 1
OUTPUT=$("$BINARY" validate --input "$FIXTURES/duplicate_id.yaml" 2>/dev/null); RC=$?
assert_exit 1 "$RC" "validate duplicate_id.yaml"

# self_loop.yaml → exit 0 (Association self-loop is valid)
OUTPUT=$("$BINARY" validate --input "$FIXTURES/self_loop.yaml" 2>/dev/null); RC=$?
assert_exit 0 "$RC" "validate self_loop.yaml"

# empty.yaml → exit 0, success:true
OUTPUT=$("$BINARY" validate --input "$FIXTURES/empty.yaml" 2>/dev/null); RC=$?
assert_exit 0 "$RC" "validate empty.yaml"

# cyclic.yaml → exit 0 (cycles are valid in validation)
OUTPUT=$("$BINARY" validate --input "$FIXTURES/cyclic.yaml" 2>/dev/null); RC=$?
assert_exit 0 "$RC" "validate cyclic.yaml"

echo "==> Testing generate + parse round-trip..."

TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

# generate valid.yaml → XML
"$BINARY" generate --input "$FIXTURES/valid.yaml" --output "$TMPDIR/out.archimate" 2>/dev/null; RC=$?
assert_exit 0 "$RC" "generate valid.yaml"
assert_contains "$(cat "$TMPDIR/out.archimate")" 'xmlns:archimate="http://www.archimatetool.com/archimate"' "XML has Archi native namespace"
assert_contains "$(cat "$TMPDIR/out.archimate")" 'xsi:type="archimate:BusinessActor"' "XML has archimate:BusinessActor type"

# parse XML → YAML
"$BINARY" parse --input "$TMPDIR/out.archimate" --output "$TMPDIR/out.yaml" 2>/dev/null; RC=$?
assert_exit 0 "$RC" "parse out.archimate"

# diff between generated XML and original YAML → empty diff
OUTPUT=$("$BINARY" diff --old "$TMPDIR/out.archimate" --new "$FIXTURES/valid.yaml" 2>/dev/null); RC=$?
assert_exit 0 "$RC" "diff valid round-trip"
assert_contains "$OUTPUT" '"added": \[\]' "diff has no additions"

echo "==> Testing --version..."
OUTPUT=$("$BINARY" --version 2>/dev/null)
assert_contains "$OUTPUT" "archr 1.0.0" "version is 1.0.0"

echo "==> Testing Python skill wrapper..."
if command -v python3 &>/dev/null; then
    OUTPUT=$(python3 "$PROJECT_ROOT/skill/scripts/archr.py" --help 2>&1); RC=$?
    assert_exit 0 "$RC" "archr.py --help"
    assert_contains "$OUTPUT" "usage:" "archr.py --help shows usage"
fi

echo ""
echo "==============================="
echo "Results: $PASS passed, $FAIL failed"
if [ "$FAIL" -eq 0 ]; then
    echo "ALL TESTS PASSED"
    exit 0
else
    echo "SOME TESTS FAILED"
    exit 1
fi
