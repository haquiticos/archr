#!/bin/bash
set -e

RUNNER_OS="${1}"
ARTIFACT_NAME="${2}"
TARGET_PATH="${3}"

if [[ "$RUNNER_OS" == "Windows" ]]; then
  # PowerShell para Windows
  # Encontrar o arquivo e renomear usando PowerShell
  pwsh -Command "Get-ChildItem -Path '$TARGET_PATH/release' -Filter 'archr*' | ForEach-Object { Rename-Item -Path \$_.FullName -NewName '$ARTIFACT_NAME' }"
else
  # Bash para Linux e macOS
  find "$TARGET_PATH/release" -maxdepth 1 -name "archr" -type f -exec mv {} "$ARTIFACT_NAME" \;
fi
