#!/bin/bash
set -e

if [[ "${{ runner.os }}" == "Windows" ]]; then
  # PowerShell para Windows
  # Encontrar o arquivo e renomear
  findstr "archr" target/${{ matrix.target }}/release | awk '{print $NF}' | while read -r old_name; do
    mv "target/${{ matrix.target }}/release/$old_name" "${{ matrix.artifact }}"
  done
else
  # Bash para Linux e macOS
  find target/${{ matrix.target }}/release -maxdepth 1 -name "archr*" -exec mv {} ${{ matrix.artifact }} \;
fi
