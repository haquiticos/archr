#!/bin/bash
set -e

RUNNER_OS="${1}"
ARTIFACT_NAME="${2}"
TARGET_PATH="${3}"

if [[ "$RUNNER_OS" == "Windows" ]]; then
  # PowerShell para Windows
  # Criar arquivo de script temporário
  echo 'if (Test-Path "$PSScriptRoot/release/archr.exe") { Rename-Item -LiteralPath "$PSScriptRoot/release/archr.exe" -NewName "archr-windows-x86_64.exe" -Force }' > /tmp/rename.ps1
  pwsh -ExecutionPolicy Bypass -File /tmp/rename.ps1
else
  # Bash para Linux e macOS
  find "$TARGET_PATH/release" -maxdepth 1 -name "archr" -type f -exec mv {} "$ARTIFACT_NAME" \;
fi
