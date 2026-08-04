# archr installer — Windows
#
# Usage (PowerShell):
#   powershell -c "irm https://raw.githubusercontent.com/haquiticos/archr/main/install/install.ps1 | iex"
#   iex "& {$(irm https://raw.githubusercontent.com/haquiticos/archr/main/install/install.ps1)} -Version v1.0.0"
#
# Installs the prebuilt archr.exe into $env:ARCHR_INSTALL\bin (default $env:USERPROFILE\.archr\bin)
# and appends it to the user's PATH.

[CmdletBinding()]
param(
  [string]$Version = ""
)

$ErrorActionPreference = "Stop"
$Repo = "haquiticos/archr"
$InstallDir = if ($env:ARCHR_INSTALL) { $env:ARCHR_INSTALL } else { Join-Path $env:USERPROFILE ".archr" }
$BinDir = Join-Path $InstallDir "bin"
$BinPath = Join-Path $BinDir "archr.exe"

function Write-Info($m) { Write-Host "==> " -ForegroundColor Green -NoNewline; Write-Host $m }

# --- platform / arch detection ------------------------------------------------
function Detect-Target {
  $os = "windows"
  $arch = [System.Runtime.InteropServices.RuntimeInformation]::ProcessArchitecture
  switch ($arch) {
    "X86"  { $a = "x86" }
    "X64"  { $a = "x86_64" }
    "Arm64"{ $a = "arm64" }
    "Arm"  { $a = "arm" }
    default { throw "unsupported arch: $arch" }
  }
  if ($a -ne "x86_64") {
    throw "windows $a build not published yet; see https://github.com/$Repo/releases"
  }
  return "archr-$os-$a"
}

function Get-Url([string]$target, [string]$version) {
  if ($version -eq "") {
    return "https://github.com/$Repo/releases/latest/download/$target.exe"
  }
  if ($version -notmatch "^v") { $version = "v$version" }
  return "https://github.com/$Repo/releases/download/$version/$target.exe"
}

function Update-Path {
  $userPath = [System.Environment]::GetEnvironmentVariable("Path", "User")
  if (($userPath -split ";") -notcontains $BinDir) {
    Write-Info "adding $BinDir to user PATH"
    [System.Environment]::SetEnvironmentVariable(
      "Path",
      $userPath.TrimEnd(";") + ";$BinDir",
      [System.EnvironmentVariableTarget]::User
    )
  }
}

# --- main ---------------------------------------------------------------------
$target = Detect-Target
$url = Get-Url -target $target -version $Version
New-Item -ItemType Directory -Force -Path $BinDir | Out-Null

Write-Info "downloading $url"
Invoke-WebRequest -Uri $url -OutFile $BinPath -UseBasicParsing

if (& $BinPath --version) {
  $v = (& $BinPath --version) -replace "`r`n",""
  Write-Info "archr $v"
} else {
  throw "binary installed but failed to execute: $BinPath"
}

Update-Path

Write-Host ""
Write-Info "Done. Restart your terminal (or open a new one) and run:"
Write-Host "    archr --version"
