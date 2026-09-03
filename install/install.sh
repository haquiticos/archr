#!/usr/bin/env bash
# archr installer — Linux & macOS
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/haquiticos/archr/main/install/install.sh | bash
#   curl -fsSL https://raw.githubusercontent.com/haquiticos/archr/main/install/install.sh | bash -s "v0.5.1"
#
# Installs the prebuilt `archr` binary into $ARCHR_INSTALL/bin (default $HOME/.archr/bin)
# and updates shell rc files to add it to PATH. Also installs the archr skill
# into $ARCHR_SKILLS_DIR (default $HOME/.agents/skills/archr-skill) from the
# matching source tag.

set -euo pipefail

ARCHR_REPO="haquiticos/archr"
ARCHR_INSTALL="${ARCHR_INSTALL:-$HOME/.archr}"
ARCHR_SKILLS_DIR="${ARCHR_SKILLS_DIR:-$HOME/.agents/skills}"
ARCHR_VERSION="${1:-v0.5.1}"

err()  { printf "\033[31merror:\033[0m %s\n" "$*" >&2; }
info() { printf "\033[32m==>\033[0m %s\n" "$*" >&2; }

fetch_url() {
  local url="$1" out="$2"
  if command -v curl >/dev/null 2>&1; then
    curl -fsSL "$url" -o "$out"
  elif command -v wget >/dev/null 2>&1; then
    wget -qO "$out" "$url"
  else
    err "need curl or wget"; exit 1
  fi
}

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
  echo "archr"
}

# --- download -----------------------------------------------------------------
download() {
  local target="$1" version="$2" out="$3"
  local url
  if [ -z "$version" ]; then
    url="https://github.com/${ARCHR_REPO}/releases/latest/download/${target}"
  else
    [ "${version#v}" = "$version" ] && version="v${version}"
    url="https://github.com/${ARCHR_REPO}/releases/download/${version}/${target}"
  fi
  info "downloading $url"
  fetch_url "$url" "$out"
}

# --- skill --------------------------------------------------------------------
# Resolve the release tag for skill assets: the version arg if given, otherwise
# the latest published release tag.
resolve_tag() {
  if [ -n "$ARCHR_VERSION" ]; then
    local v="$ARCHR_VERSION"
    [ "${v#v}" = "$v" ] && v="v${v}"
    echo "$v"
  else
    local api="https://api.github.com/repos/${ARCHR_REPO}/releases/latest"
    fetch_url "$api" - 2>/dev/null \
      | grep '"tag_name"' | head -1 \
      | sed -E 's/.*"tag_name"[[:space:]]*:[[:space:]]*"([^"]+)".*/\1/'
  fi
}

# Install the skill tree (skill/) for the given release tag into the skills dir.
install_skill() {
  local tag dest tmp tgz src
  tag="$(resolve_tag)"
  [ -n "$tag" ] || { err "could not resolve archr version for skill"; return 1; }
  dest="$ARCHR_SKILLS_DIR/archr-skill"
  tmp="$(mktemp -d)"
  tgz="$tmp/archr-src.tar.gz"
  info "downloading skill source @ ${tag}"
  fetch_url "https://codeload.github.com/${ARCHR_REPO}/tar.gz/refs/tags/${tag}" "$tgz"
  tar -xzf "$tgz" -C "$tmp"
  src="$(find "$tmp" -maxdepth 2 -type d -path '*/archr-*/skill' | head -1)"
  [ -n "$src" ] || { err "skill/ not found in source tarball @ ${tag}"; rm -rf "$tmp"; return 1; }
  rm -rf "$dest"
  mkdir -p "$ARCHR_SKILLS_DIR"
  cp -R "$src" "$dest"
  rm -rf "$dest/scripts/__pycache__"
  find "$dest" -name '*.pyc' -delete
  rm -rf "$tmp"
  info "installed skill to $dest"
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

  if download "$target" "$ARCHR_VERSION" "$tmp/$target"; then
    chmod +x "$tmp/$target"
    mv "$tmp/$target" "$binpath"
    info "installed archr to $binpath"

    maybe_update_path

    if "$binpath" --version >/dev/null 2>&1; then
      info "archr $("$binpath" --version)"
    else
      err "binary installed but failed to execute: $binpath"
      err "this may be a glibc/arch mismatch; check https://github.com/haquiticos/archr/releases"
    fi

    # Install skill (best effort)
    install_skill
  else
    err "failed to download archr binary for $target"
  fi

  printf '\n'
  info "Done. Restart your shell or run:"
  printf '    export PATH="%s/bin:$PATH"\n' "$ARCHR_INSTALL"
  printf 'then `archr --version`.\n'
}

main "$@"
