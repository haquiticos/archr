#!/usr/bin/env bash
# archr installer — Linux & macOS
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/haquiticos/archr/main/install/install.sh | bash
#   curl -fsSL https://raw.githubusercontent.com/haquiticos/archr/main/install/install.sh | bash -s "v1.0.0"
#
# Installs the prebuilt `archr` binary into $ARCHR_INSTALL/bin (default $HOME/.archr/bin)
# and updates shell rc files to add it to PATH.

set -euo pipefail

ARCHR_REPO="haquiticos/archr"
ARCHR_INSTALL="${ARCHR_INSTALL:-$HOME/.archr}"
ARCHR_VERSION="${1:-}"

err()  { printf "\033[31merror:\033[0m %s\n" "$*" >&2; }
info() { printf "\033[32m==>\033[0m %s\n" "$*" >&2; }

# --- platform / arch detection -----------------------------------------------
detect_target() {
  local os arch
  os="$(uname -s)"
  arch="$(uname -m)"
  case "$os" in
    Linux)  os="linux" ;;
    Darwin)  os="macos" ;;
    *) err "unsupported OS: $os"; exit 1 ;;
  esac
  case "$arch" in
    x86_64|amd64)  arch="x86_64" ;;
    arm64|aarch64) arch="arm64" ;;
    *) err "unsupported arch: $arch"; exit 1 ;;
  esac
  # only targets we ship in release.yml
  if [ "$os" = "macos" ] && [ "$arch" = "arm64" ]; then arch="arm64"; fi
  if [ "$os" = "linux" ] && [ "$arch" = "arm64" ]; then
    err "linux arm64 build not published yet; see https://github.com/haquiticos/archr/releases"
    exit 1
  fi
  echo "archr-${os}-${arch}"
}

# --- download -----------------------------------------------------------------
download() {
  local target="$1" version="$2" out="$3"
  local url
  if [ -z "$version" ]; then
    url="https://github.com/${ARCHR_REPO}/releases/latest/download/${target}"
  else
    # accept both "v1.0.0" and "1.0.0"
    [ "${version#v}" = "$version" ] && version="v${version}"
    url="https://github.com/${ARCHR_REPO}/releases/download/${version}/${target}"
  fi
  info "downloading $url"
  if command -v curl >/dev/null 2>&1; then
    curl -fsSL "$url" -o "$out"
  elif command -v wget >/dev/null 2>&1; then
    wget -qO "$out" "$url"
  else
    err "need curl or wget"; exit 1
  fi
}

# --- PATH ---------------------------------------------------------------------
rc_for_shell() {
  case "$1" in
    bash)  echo "$HOME/.bashrc" ;;
    zsh)   echo "$HOME/.zshrc" ;;
    fish)  echo "$HOME/.config/fish/config.fish" ;;
    *)     return 1 ;;
  esac
}

add_path_line() {
  local rc="$1" shell="$2"
  local marker="# archr"
  if [ -f "$rc" ] && grep -q "$marker" "$rc" 2>/dev/null; then
    return 0
  fi
  info "adding $ARCHR_INSTALL/bin to PATH in $rc"
  if [ "$shell" = "fish" ]; then
    printf 'set -gx PATH %s/bin $PATH %s\n' "$ARCHR_INSTALL" "$marker" >> "$rc"
  else
    printf 'export PATH="%s/bin:$PATH" %s\n' "$ARCHR_INSTALL" "$marker" >> "$rc"
  fi
}

maybe_update_path() {
  local shell rc
  shell="$(basename "${SHELL:-}")"
  if [ -z "$shell" ]; then return 0; fi
  if rc="$(rc_for_shell "$shell")" 2>/dev/null; then
    [ -f "$(dirname "$rc")" ] || mkdir -p "$(dirname "$rc")"
    touch "$rc" 2>/dev/null || true
    add_path_line "$rc" "$shell"
  fi
}

# --- main ---------------------------------------------------------------------
main() {
  local target bindir binpath tmp
  target="$(detect_target)"
  bindir="$ARCHR_INSTALL/bin"
  binpath="$bindir/archr"
  mkdir -p "$bindir"

  tmp="$(mktemp -d)"
  trap 'rm -rf "$tmp"' EXIT

  download "$target" "$ARCHR_VERSION" "$tmp/$target"
  chmod +x "$tmp/$target"
  mv "$tmp/$target" "$binpath"
  info "installed archr to $binpath"

  maybe_update_path

  if "$binpath" --version >/dev/null 2>&1; then
    info "archr $("$binpath" --version)"
  else
    err "binary installed but failed to execute: $binpath"
    err "this may be a glibc/arch mismatch; check https://github.com/haquiticos/archr/releases"
    exit 1
  fi

  printf '\n'
  info "Done. Restart your shell or run:"
  printf '    export PATH="%s/bin:$PATH"\n' "$ARCHR_INSTALL"
  printf 'then `archr --version`.\n'
}

main "$@"
