#!/usr/bin/env bash
# Validate companion apps use service-specific tab icons (not a shared/stale SVG).
#
# Usage:
#   ./scripts/check-app-icons.sh              # all apps under ../
#   ./scripts/check-app-icons.sh ../mark
#
# Rules:
# 1. assets/favicon.png must exist
# 2. assets/icon.png should match favicon.png (brand asset)
# 3. index.html must prefer PNG as primary rel=icon (not SVG-first)
# 4. Reject the known legacy red-check SVG (todo scaffold leftover)
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PARENT="$(cd "$ROOT/.." && pwd)"
RED_CHECK_MD5="b38828f8820f79d0865ef3d530567fc0"
fail=0

check_one() {
  local app="$1"
  local name
  name="$(basename "$app")"
  local fav=""
  local icon="$app/assets/icon.png"
  local svg="$app/assets/favicon.svg"
  local index="$app/frontend/index.html"

  if [ ! -d "$app/frontend" ] && [ ! -d "$app/src/dashboard" ]; then
    echo "skip $name (no web UI)"
    return 0
  fi

  for cand in favicon.png favicon.jpg icon.png; do
    if [ -f "$app/assets/$cand" ]; then
      fav="$app/assets/$cand"
      break
    fi
  done

  if [ -z "$fav" ]; then
    echo "FAIL $name: missing assets/favicon.png (or .jpg / icon.png)"
    fail=1
    return
  fi

  if [ -f "$icon" ] && [[ "$fav" == *.png ]]; then
    local hf hi
    hf=$(md5sum "$fav" | awk '{print $1}')
    hi=$(md5sum "$icon" | awk '{print $1}')
    if [ "$hf" != "$hi" ]; then
      echo "WARN $name: favicon.png != icon.png (tab icon should match brand icon)"
    fi
  fi

  if [ -f "$svg" ]; then
    local hs
    hs=$(md5sum "$svg" | awk '{print $1}')
    if [ "$hs" = "$RED_CHECK_MD5" ]; then
      echo "FAIL $name: assets/favicon.svg is the legacy red-check (todo) icon"
      fail=1
    fi
  fi

  if [ -f "$index" ]; then
    if grep -q 'type="image/svg+xml".*favicon\|favicon\.svg.*rel="icon"\|rel="icon".*svg' "$index"; then
      # SVG listed as primary icon is OK only if not red-check; still prefer PNG
      if grep -q 'rel="icon"[^>]*image/svg+xml\|type="image/svg+xml"[^>]*rel="icon"' "$index"; then
        local first
        first=$(grep -n 'rel="icon"' "$index" | head -1 || true)
        if echo "$first" | grep -qi svg; then
          echo "FAIL $name: index.html prefers SVG favicon (browsers will ignore PNG brand icon)"
          fail=1
        fi
      fi
    fi
    if ! grep -q 'favicon\.png' "$index"; then
      echo "FAIL $name: index.html does not reference favicon.png"
      fail=1
    fi
  fi

  echo "ok   $name"
}

if [ $# -gt 0 ]; then
  for a in "$@"; do check_one "$(cd "$a" && pwd)"; done
else
  for d in "$PARENT"/*/; do
    name="$(basename "$d")"
    case "$name" in
      shared-assets|studio2201.github.io|.github) continue ;;
    esac
    check_one "$d"
  done
fi

exit "$fail"
