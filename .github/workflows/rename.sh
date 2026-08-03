#!/bin/bash
set -e

RUNNER_OS="${1}"
ARTIFACT_NAME="${2}"
TARGET_PATH="${3}"

if [[ "$RUNNER_OS" == "Windows" ]]; then
  # PowerShell para Windows
  # Listar arquivos para debug
  dir "$TARGET_PATH/release" | findstr archr
  # Criar arquivo de script temporário
  echo "Write-Host \"Target path: $TARGET_PATH\"" > /tmp/rename.ps1
  echo "Write-Host \"Artifact name: archr-windows-x86_64.exe\"" >> /tmp/rename.ps1
  echo "[bool](Test-Path '$TARGET_PATH/release/archr.exe')" >> /tmp/rename.ps1
  echo "if (Test-Path '$TARGET_PATH/release/archr.exe') { Rename-Item -LiteralPath '$TARGET_PATH/release/archr.exe' -NewName 'archr-windows-x86_64.exe' -Force }" >> /tmp/rename.ps1
  pwsh -ExecutionPolicy Bypass -File /tmp/rename.ps1
else
  # Bash para Linux e macOS
  find "$TARGET_PATH/release" -maxdepth 1 -name "archr" -type f -exec mv {} "$ARTIFACT_NAME" \;
fi
