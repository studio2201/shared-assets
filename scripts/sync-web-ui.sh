#!/usr/bin/env bash
# Sync shared-assets web UI styles into companion app trees.
#
# Vendors **styles only** under assets/shared-assets/styles/.
# Do NOT copy shared-rust into apps — Cargo pulls crates from the
# git tag (e.g. v3.3.1). Session/cookie stay app-local.
#
# Usage:
#   ./scripts/sync-web-ui.sh                  # all apps under ../
#   ./scripts/sync-web-ui.sh /path/to/beam    # one app
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
STYLES_SRC="$ROOT/styles"

sync_one() {
  local app="$1"
  local dest=""
  if [ -d "$app/assets/shared-assets" ] || [ -d "$app/frontend" ] || [ -d "$app/src/dashboard" ]; then
    # Canonical path for every studio2201 app (Yew Trunk + Maud).
    dest="$app/assets/shared-assets/styles"
  else
    echo "skip (no UI tree): $app"
    return 0
  fi
  mkdir -p "$dest"
  rsync -a --delete \
    --exclude '.git' \
    "$STYLES_SRC/" "$dest/"
  echo "synced -> $dest"
}

if [ $# -gt 0 ]; then
  for a in "$@"; do sync_one "$(cd "$a" && pwd)"; done
  exit 0
fi

PARENT="$(cd "$ROOT/.." && pwd)"
for d in "$PARENT"/*/; do
  name="$(basename "$d")"
  case "$name" in
    shared-assets|studio2201.github.io|.github) continue ;;
  esac
  sync_one "$d"
done
