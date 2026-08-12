#!/bin/bash

echo "=========================================="
echo "Teste de Viewpoints no Archr"
echo "=========================================="

# Test 1: Modelo com único viewpoint
echo ""
echo "Test 1: Modelo com único viewpoint"
echo "------------------------------------"
cargo run -- generate --input example_viewpoint.yaml --output output_test1.xml
echo "✓ Gerado com sucesso: output_test1.xml"
echo ""

# Test 2: Modelo com múltiplos viewpoints
echo "Test 2: Modelo com múltiplos viewpoints"
echo "----------------------------------------"
cargo run -- generate --input test_multiple_viewpoints.yaml --output output_test2.xml
echo "✓ Gerado com sucesso: output_test2.xml"
echo ""

# Test 3: Modelo sem viewpoints (para comparação)
echo "Test 3: Modelo sem viewpoints"
echo "------------------------------"
cargo run -- generate --input model_minimal.yaml --output output_test3.xml
echo "✓ Gerado com sucesso: output_test3.xml"
echo ""

echo "=========================================="
echo "Todos os testes foram executados!"
echo "=========================================="
