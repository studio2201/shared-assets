#!/usr/bin/env bash
# Ensure companion apps use the service-specific brand icon as the browser tab favicon.
#
# What this does (in-place):
#   1. assets/favicon.png := assets/icon.png  (when icon.png exists)
#   2. Remove legacy red-check favicon.svg (todo scaffold leftover)
#   3. Make frontend/index.html prefer PNG as primary rel=icon
#
# Usage:
#   ./scripts/ensure-app-icons.sh              # all apps under ../
#   ./scripts/ensure-app-icons.sh ../mark
#
# Validate afterwards with:
#   ./scripts/check-app-icons.sh
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PARENT="$(cd "$ROOT/.." && pwd)"
RED_CHECK_MD5="b38828f8820f79d0865ef3d530567fc0"

ensure_one() {
  local app="$1"
  local name
  name="$(basename "$app")"
  local assets="$app/assets"
  local icon="$assets/icon.png"
  local fav="$assets/favicon.png"
  local svg="$assets/favicon.svg"
  local index="$app/frontend/index.html"
  local changed=0

  if [ ! -d "$app/frontend" ] && [ ! -d "$app/src/dashboard" ]; then
    echo "skip $name (no web UI)"
    return 0
  fi

  if [ ! -d "$assets" ]; then
    echo "skip $name (no assets/)"
    return 0
  fi

  # 1) favicon.png must match brand icon.png when both exist
  if [ -f "$icon" ]; then
    if [ ! -f "$fav" ] || ! cmp -s "$icon" "$fav"; then
      cp -f "$icon" "$fav"
      echo "  $name: synced assets/favicon.png <- icon.png"
      changed=1
    fi
  elif [ ! -f "$fav" ] && [ ! -f "$assets/favicon.jpg" ]; then
    echo "WARN $name: missing assets/icon.png and assets/favicon.png"
  fi

  # 2) drop legacy red-check SVG (or any favicon.svg when we ship PNG brand)
  if [ -f "$svg" ]; then
    local hs
    hs=$(md5sum "$svg" | awk '{print $1}')
    if [ "$hs" = "$RED_CHECK_MD5" ]; then
      rm -f "$svg"
      echo "  $name: removed legacy red-check favicon.svg"
      changed=1
    elif [ -f "$fav" ]; then
      # Prefer PNG brand; keep service-specific SVGs only if they differ from red-check
      # and are not the sole icon. We still strip SVG from index.html below.
      :
    fi
  fi

  # 3) index.html — PNG primary, no SVG-first icon links
  if [ -f "$index" ]; then
    local before
    before=$(md5sum "$index" | awk '{print $1}')

    # Remove SVG favicon link tags and "alternate icon" PNG fallbacks
    # that only exist because SVG was primary.
    if grep -q 'favicon\.svg\|type="image/svg+xml".*icon\|rel="icon"[^>]*svg' "$index"; then
      # portable in-place sed: write temp then mv
      local tmp
      tmp=$(mktemp)
      # drop lines that reference favicon.svg or svg+xml as icon
      grep -v -E 'favicon\.svg|type="image/svg\+xml"[^>]*rel="icon"|rel="icon"[^>]*type="image/svg\+xml"|rel="alternate icon"' "$index" >"$tmp" || true
      # if grep -v emptied the file somehow, abort
      if [ ! -s "$tmp" ]; then
        rm -f "$tmp"
        echo "FAIL $name: refused to empty index.html"
        return 1
      fi
      mv "$tmp" "$index"
    fi

    # Ensure PNG copy-file for Trunk (Yew apps)
    if ! grep -q 'data-trunk rel="copy-file"[^>]*favicon\.png\|copy-file" href="\.\./assets/favicon\.png"' "$index"; then
      if grep -q 'data-trunk' "$index"; then
        # insert copy-file before first rel=icon or before </head>
        if grep -q 'rel="icon"' "$index"; then
          local tmp2
          tmp2=$(mktemp)
          awk '
            /rel="icon"/ && !done {
              print "<link data-trunk rel=\"copy-file\" href=\"../assets/favicon.png\" />"
              done=1
            }
            { print }
          ' "$index" >"$tmp2"
          mv "$tmp2" "$index"
        fi
      fi
    fi

    # Ensure primary PNG rel=icon
    if ! grep -q 'rel="icon"[^>]*favicon\.png\|href="favicon\.png"[^>]*rel="icon"' "$index" \
      && ! grep -q 'type="image/png"[^>]*href="favicon\.png"' "$index"; then
      local tmp3
      tmp3=$(mktemp)
      if grep -q '</title>' "$index"; then
        sed 's#</title>#</title>\n    <link rel="icon" type="image/png" href="favicon.png" />\n    <link rel="apple-touch-icon" href="favicon.png" />#' "$index" >"$tmp3"
      else
        sed 's#</head>#    <link rel="icon" type="image/png" href="favicon.png" />\n    <link rel="apple-touch-icon" href="favicon.png" />\n</head>#' "$index" >"$tmp3"
      fi
      mv "$tmp3" "$index"
    fi

    # Ensure apple-touch-icon once
    if ! grep -q 'apple-touch-icon' "$index"; then
      local tmp4
      tmp4=$(mktemp)
      sed 's#rel="icon" type="image/png" href="favicon.png" */>#&\n    <link rel="apple-touch-icon" href="favicon.png" />#' "$index" >"$tmp4"
      mv "$tmp4" "$index"
    fi

    local after
    after=$(md5sum "$index" | awk '{print $1}')
    if [ "$before" != "$after" ]; then
      echo "  $name: updated frontend/index.html icon links (PNG primary)"
      changed=1
    fi
  fi

  if [ "$changed" -eq 0 ]; then
    echo "ok   $name (already compliant)"
  else
    echo "fixed $name"
  fi
}

if [ $# -gt 0 ]; then
  for a in "$@"; do ensure_one "$(cd "$a" && pwd)"; done
else
  for d in "$PARENT"/*/; do
    name="$(basename "$d")"
    case "$name" in
      shared-assets|studio2201.github.io|.github) continue ;;
    esac
    ensure_one "$d"
  done
fi
