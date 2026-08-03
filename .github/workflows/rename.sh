#!/bin/bash
set -e

RUNNER_OS="${1}"
ARTIFACT_NAME="${2}"
TARGET_PATH="${3}"

if [[ "$RUNNER_OS" == "Windows" ]]; then
  # PowerShell para Windows
  # Renomear o binário específico
  pwsh -Command "if (Test-Path '$TARGET_PATH/release/archr.exe') { Rename-Item -Path '$TARGET_PATH/release/archr.exe' -NewName '$ARTIFACT_NAME' }"
else
  # Bash para Linux e macOS
  find "$TARGET_PATH/release" -maxdepth 1 -name "archr" -type f -exec mv {} "$ARTIFACT_NAME" \;
fi
