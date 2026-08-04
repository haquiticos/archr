# Upgrade

`archr` does not (yet) self-upgrade. Re-run the installer or bump the `cargo` version.

## Via install script (Linux & macOS)

Re-running the install script overwrites the binary in place with the latest release:

```bash
curl -fsSL https://raw.githubusercontent.com/haquiticos/archr/main/install/install.sh | bash
```

To move to a specific version, pass the tag:

```bash
curl -fsSL https://raw.githubusercontent.com/haquiticos/archr/main/install/install.sh | bash -s "v1.1.0"
```

## Via PowerShell (Windows)

```powershell
powershell -c "irm https://raw.githubusercontent.com/haquiticos/archr/main/install/install.ps1 | iex"
```

Pin a version:

```powershell
iex "& {$(irm https://raw.githubusercontent.com/haquiticos/archr/main/install/install.ps1)} -Version v1.1.0"
```

## Via cargo

```bash
cargo install archr-core
```

`cargo install` reinstalls the latest-compatible version published to crates.io.

## From source

```bash
git pull
cargo build --release
# binary at target/release/archr
```

## Canary builds

Not yet published. Watch the [releases page](https://github.com/haquiticos/archr/releases) for new tags.
