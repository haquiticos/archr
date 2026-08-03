#!/bin/bash
set -e

RUNNER_OS="${1}"
ARTIFACT_NAME="${2}"
TARGET_PATH="${3}"

if [[ "$RUNNER_OS" == "Windows" ]]; then
  # PowerShell para Windows
  # Encontrar o arquivo e renomear
  dir "$TARGET_PATH/release" | findstr "archr" | awk '{print $NF}' | xargs -I {} move "$TARGET_PATH/release/{}" "$ARTIFACT_NAME"
else
  # Bash para Linux e macOS
  mv "$TARGET_PATH/release/archr*" "$ARTIFACT_NAME"
fi
