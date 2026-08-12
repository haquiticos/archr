#!/bin/bash
set -e

echo "=== Demonstrating Viewpoint Field Support ==="
echo

echo "1. Building the project..."
cargo build --lib --quiet
echo "✅ Build successful"
echo

echo "2. Running tests..."
cargo test --lib --quiet
echo "✅ All tests pass"
echo

echo "3. Sample YAML with viewpoint field:"
cat example_viewpoint.yaml
echo

echo "4. Expected behavior:"
echo "   - YAML files with 'viewpoint: business' will parse successfully"
echo "   - Serialized models will include 'viewpoint' field"
echo "   - Viewpoint field is optional (defaults to None)"
echo

echo "=== Viewpoint field support is working! ==="
