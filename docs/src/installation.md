# Installation

> Install `archr` with the install script, `cargo`, or download the binary directly on macOS, Linux, and Windows.

`archr` ships as a single, dependency-free executable. No runtime, no package manager required — just the binary.

After installation, verify with:

```bash
archr --version
```

## Installation

### Install script (macOS & Linux)

The fastest path — downloads the latest prebuilt binary for your platform, installs it to `$HOME/.archr/bin`, and wires it into your shell's `PATH`.

```bash
curl -fsSL https://raw.githubusercontent.com/haquiticos/archr/main/install/install.sh | bash
```

Pin a specific version:

```bash
curl -fsSL https://raw.githubusercontent.com/haquiticos/archr/main/install/install.sh | bash -s "v1.0.0"
```

**Requirements.** `curl` or `wget`; glibc ≥ 2.17 on Linux; macOS 11 or later. Kernel 5.6 or newer is recommended.

### Install script (Windows)

PowerShell 5.1+ (Windows 10 1809 or later):

```powershell
powershell -c "irm https://raw.githubusercontent.com/haquiticos/archr/main/install/install.ps1 | iex"
```

Pin a version:

```powershell
iex "& {$(irm https://raw.githubusercontent.com/haquiticos/archr/main/install/install.ps1)} -Version v1.0.0"
```

### cargo

If you have a Rust toolchain:

```bash
cargo install archr-core
archr --version
```

### Build from source

```bash
git clone https://github.com/haquiticos/archr.git
cd archr
cargo build --release
# Binary at target/release/archr
```

For_contributors: cross-compile with `cargo build --release --target <triple>` (see the release matrix for the supported targets).

### Verify

Open a new terminal and run:

```bash
archr --version
# archr 1.0.0
```

## Add archr to your PATH

The install scripts attempt to update `PATH` automatically. If you see `command not found: archr`, add the install directory manually.

### macOS & Linux

Determine which shell you're using:

```bash
echo $SHELL
# /bin/zsh  or  /bin/bash  or  /bin/fish
```

Open the matching rc file and append:

```bash
# bash / zsh
export PATH="$HOME/.archr/bin:$PATH"

# fish (in ~/.config/fish/config.fish)
fish_add_path $HOME/.archr/bin
```

Then reload:

```bash
source ~/.bashrc   # or ~/.zshrc
```

### Windows

PowerShell (user-level PATH):

```powershell
[System.Environment]::SetEnvironmentVariable(
  "Path",
  [System.Environment]::GetEnvironmentVariable("Path", "User") + ";$env:USERPROFILE\.archr\bin",
  [System.EnvironmentVariableTarget]::User
)
```

Restart your terminal and run `archr --version`.

## Direct downloads

Visit the [releases page on GitHub](https://github.com/haquiticos/archr/releases) for all artifacts, including older versions.

| Platform          | Asset                                  |
|-------------------|----------------------------------------|
| Linux x86_64       | `archr-linux-x86_64`                   |
| macOS Apple Silicon | `archr-macos-arm64`                  |
| macOS Intel        | `archr-macos-x86_64`                  |
| Windows x86_64    | `archr-windows-x86_64.exe`            |

Grab the latest directly:

```bash
# Linux x86_64
curl -fsSL -o archr https://github.com/haquiticos/archr/releases/latest/download/archr-linux-x86_64
chmod +x archr && ./archr --version
```

## Install a specific version

Re-run the install script with a git tag (`v1.0.0`) — the script downloads that release instead of `latest`:

```bash
# Linux & macOS
curl -fsSL https://raw.githubusercontent.com/haquiticos/archr/main/install/install.sh | bash -s "v1.0.0"

# Windows
iex "& {$(irm https://raw.githubusercontent.com/haquiticos/archr/main/install/install.ps1)} -Version v1.0.0"
```

Via `cargo`:

```bash
cargo install archr-core --version 1.0.0
```

## CPU & OS requirements

| Platform | Requirement                          |
|----------|--------------------------------------|
| Linux    | glibc ≥ 2.17; kernel ≥ 3.10 (5.6+ recommended) |
| macOS    | 11 (Big Sur) or later                |
| Windows  | Windows 10 1809 or later (PowerShell 5.1+) |

No AVX2 / SSE requirements — `archr` uses no SIMD intrinsics.
