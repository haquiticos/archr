# Shared Setup

This setup covers the common prerequisites for all three tutorials in the Self-Modeling series (T1, T2, T3). Referenced by each tutorial's README to avoid duplication.

## Installation

Choose one of the three installation paths below:

### Option 1: Prebuilt Binary (Recommended)

Download the latest release from [GitHub Releases](https://github.com/haquiticos/archr/releases) and run:

#### Linux (x86_64)
```bash
./archr-linux-x86_64 --version
```

#### macOS (Intel)
```bash
chmod +x archr-macos-x86_64
./archr-macos-x86_64 --version
```

#### macOS (Apple Silicon)
```bash
chmod +x archr-macos-arm64
./archr-macos-arm64 --version
```

#### Windows (x86_64)
```bash
archr-windows-x86_64.exe --version
```

**Verification**:
```bash
archr --version
# Expected output: archr 1.0.0
```

### Option 2: Cargo Install

```bash
cargo install archr-core
archr --version
```

**Verification**:
```bash
archr --version
# Expected output: archr 1.0.0
```

### Option 3: Build from Source

```bash
# Clone the repository
git clone https://github.com/haquiticos/archr.git
cd archr

# Build for all targets
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-msvc
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# Binary is at target/release/archr
```

**Verification**:
```bash
./target/release/archr --version
# Expected output: archr 1.0.0
```

## Environment Variable: `ARCHR_BIN`

The `archr` binary can be specified via the `ARCHR_BIN` environment variable. This is useful if you have multiple `archr` installations and want to control which one is used by the Agent Skill wrapper.

```bash
export ARCHR_BIN=/path/to/archr
python3 skill/scripts/archr.py --version
```

If `ARCHR_BIN` is not set, the wrapper falls back to `archr` in `PATH`.

## Optional Prerequisites

These tools are **not required** to complete the tutorials, but they enhance certain chapters:

### hyperfine (T3 Benchmark Chapter Only)

A command-line benchmarking tool for comparing execution time of commands.

```bash
cargo install hyperfine
```

**Usage in T3**:
```bash
hyperfine --runs=10 "archr validate --input model.yaml"
```

### Archi GUI

[ArchiMatetool.com](https://www.archimatetool.com) — the official editor for ArchiMate models. Use it to open the generated `.archimate` files and visualize the diagram.

**Usage in T2**:
```bash
archr generate --input model.yaml --output model.archimate
# Then open model.archimate in Archi
```

### Python Agent Skill Wrapper

The `archr` Python wrapper demonstrates integration with AI agents (Claude Code, VS Code Copilot, OpenAI Codex).

```bash
python3 skill/scripts/archr.py --version
```

**Usage**:
```bash
# Validate through the wrapper
python3 skill/scripts/archr.py validate model.yaml

# Generate XML
python3 skill/scripts/archr.py generate model.yaml --output model.archimate

# Parse XML back to YAML
python3 skill/scripts/archr.py parse --input model.archimate --output model.yaml
```

## Troubleshooting

### `archr: command not found`

- If using the prebuilt binary, ensure it's in your `PATH` or use `ARCHR_BIN` to point to it explicitly.
- If using `cargo install`, verify installation: `cargo install --list | grep archr-core`.
- If building from source, run the binary from `target/release/`.

### Version mismatch

```bash
archr --version
# Expected: archr 1.0.0
```

If the version is incorrect, re-install or rebuild from source using the steps above.

### Permission denied (Linux/macOS)

```bash
chmod +x archr-binary-name
```
