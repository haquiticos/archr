#!/bin/sh
# Install the archr skill into an omp-discoverable skills root.
# Usage: skill/install.sh           # project-level  -> .agents/skills/archr-skill
#        skill/install.sh --user    # user-level     -> ~/.omp/agent/skills/archr-skill
set -eu

NAME="archr-skill"
SRC="$(cd "$(dirname "$0")" && pwd)"

if [ "${1:-}" = "--user" ]; then
  DEST="$HOME/.omp/agent/skills/$NAME"
else
  DEST="$(pwd)/.agents/skills/$NAME"
fi

mkdir -p "$(dirname "$DEST")"
rm -rf "$DEST"
cp -R "$SRC" "$DEST"
rm -rf "$DEST/scripts/__pycache__"
echo "Installed $NAME -> $DEST"
