#!/bin/bash
set -e

RUNNER_OS="${1}"
ARTIFACT_NAME="${2}"

if [[ "$RUNNER_OS" == "Windows" ]]; then
  # PowerShell para Windows
  # Encontrar o arquivo e renomear
  dir target/release | findstr "archr" | awk '{print $NF}' | xargs -I {} move "target/release/{}" "$ARTIFACT_NAME"
else
  # Bash para Linux e macOS
  mv target/release/archr* "$ARTIFACT_NAME"
fi
