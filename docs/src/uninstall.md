# Uninstall

Remove the `archr` binary and its install directory. PATH entries can be left in place (harmless) or cleaned up manually.

## macOS & Linux

```bash
rm -rf "$HOME/.archr"
```

To also drop the PATH line from your shell rc, edit `~/.bashrc` (or `~/.zshrc`) and remove the `# archr` marker line:

```bash
export PATH="$HOME/.archr/bin:$PATH" # archr
```

Reload your shell:

```bash
source ~/.bashrc   # or ~/.zshrc
```

For fish, remove the line tagged `# archr` from `~/.config/fish/config.fish`.

## Windows

Remove the install directory:

```powershell
Remove-Item -Recurse -Force "$env:USERPROFILE\.archr"
```

Optionally remove `archr` from the user `PATH`:

```powershell
$current = [System.Environment]::GetEnvironmentVariable("Path", "User")
$cleaned = ($current -split ";" | Where-Object { $_ -notmatch "\.archr\\bin$" }) -join ";"
[System.Environment]::SetEnvironmentVariable("Path", $cleaned, [System.EnvironmentVariableTarget]::User)
```

## Via cargo

```bash
cargo uninstall archr-core
```

Note: `cargo uninstall` removes the binary from `~/.cargo/bin`, not a `curl`-installed `~/.archr`.
