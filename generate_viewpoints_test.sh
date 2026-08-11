#!/bin/bash

set -e

echo "Testing archr generate command..."
echo ""

# Testar geração de XML com viewpoint múltiplo
echo "1. Running: archr generate model_archimate_full.yaml model_archimate_output.archimate"
cargo run --bin archr generate model_archimate_full.yaml model_archimate_output.archimate 2>&1 | tail -10

echo ""
echo "2. Checking output file..."
if [ -f "model_archimate_output.archimate" ]; then
    echo "✓ Output file created"
    echo ""
    echo "3. Analyzing XML content..."
    echo ""
    
    # Contar quantos ArchimateDiagramModel foram gerados
    diagram_count=$(grep -c "ArchimateDiagramModel" model_archimate_output.archimate || true)
    echo "   ArchimateDiagramModel count: $diagram_count"
    
    # Listar os nomes dos diagramas
    echo ""
    echo "   Diagram names:"
    grep "name=" model_archimate_output.archimate | grep -o 'name="[^"]*"' | head -5
    echo ""
    
    # Verificar se há mais de um diagrama
    if [ "$diagram_count" -lt 6 ]; then
        echo "   ⚠ WARNING: Expected 6 viewpoints but only found $diagram_count diagrams"
        echo "   This confirms the bug: viewpoints are not being generated"
    else
        echo "   ✓ All viewpoints generated correctly"
    fi
else
    echo "✗ Output file not created"
fi

echo ""
echo "4. Cleanup..."
rm -f model_archimate_output.archimate

echo ""
echo "Done."
