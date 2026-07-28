#!/usr/bin/env bash
# Assert companion apps wire AppShell header/footer GH links.
#
# Required for Yew apps (frontend/**/*.rs):
#   1. Uses shared_frontend AppShell
#   2. HeaderProps.repo: Some("<app>")  (title → GH repo)
#   3. FooterProps.repo: Some("<app>")  (version → release tag)
#   4. Footer show_version + version value
#
# StateSync (Maud): title href to GH repo + version footer → release tag.
#
# Usage:
#   ./scripts/check-appshell-links.sh
#   ./scripts/check-appshell-links.sh ../mark
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PARENT="$(cd "$ROOT/.." && pwd)"
fail=0

rg_fe() {
  # $1=app $2=pattern — ripgrep if available, else grep -R
  local app="$1" pat="$2"
  if command -v rg >/dev/null 2>&1; then
    rg -q --glob '*.rs' -e "$pat" "$app/frontend/src"
  else
    grep -R -q --include='*.rs' -E "$pat" "$app/frontend/src"
  fi
}

count_fe() {
  local app="$1" pat="$2"
  if command -v rg >/dev/null 2>&1; then
    rg --glob '*.rs' -e "$pat" "$app/frontend/src" 2>/dev/null | wc -l
  else
    grep -R --include='*.rs' -E "$pat" "$app/frontend/src" 2>/dev/null | wc -l
  fi
}

check_yew() {
  local app="$1"
  local name
  name="$(basename "$app")"
  local fe="$app/frontend/src"
  if [ ! -d "$fe" ]; then
    return 1
  fi

  local local_fail=0

  if ! rg_fe "$app" 'AppShell'; then
    echo "FAIL $name: no AppShell usage"
    local_fail=1
  fi

  local repo_hits
  repo_hits=$(count_fe "$app" "repo: Some\\(\"$name\"")
  repo_hits=${repo_hits// /}
  if [ "${repo_hits:-0}" -lt 2 ]; then
    echo "FAIL $name: expected Header+Footer repo: Some(\"$name\") (found ${repo_hits:-0})"
    local_fail=1
  fi

  if ! rg_fe "$app" 'show_version:'; then
    echo "FAIL $name: missing show_version in FooterProps"
    local_fail=1
  fi

  # Footer version: "version: foo" or Rust shorthand "version," after show_version
  if ! rg_fe "$app" 'version:\s*(Some\(|self\.|env!|version)' \
    && ! rg_fe "$app" '^\s+version,'; then
    echo "FAIL $name: FooterProps missing version value"
    local_fail=1
  fi

  if [ "$local_fail" -eq 0 ]; then
    echo "ok   $name (AppShell, repo×${repo_hits})"
  else
    fail=1
  fi
}

check_statesync() {
  local app="$1"
  local local_fail=0
  local page="$app/src/dashboard/dashboard_page.rs"
  local js="$app/src/dashboard/scripts_actions.rs"
  if [ ! -f "$page" ]; then
    echo "FAIL statesync: missing dashboard_page.rs"
    fail=1
    return
  fi
  if ! grep -q 'github.com/studio2201/statesync' "$page"; then
    echo "FAIL statesync: header title missing GH repo link"
    local_fail=1
  fi
  if [ -f "$js" ] && ! grep -q 'releases/tag/v' "$js"; then
    echo "FAIL statesync: version footer missing release tag link"
    local_fail=1
  fi
  if [ "$local_fail" -eq 0 ]; then
    echo "ok   statesync (Maud title + version release link)"
  else
    fail=1
  fi
}

check_one() {
  local app="$1"
  local name
  name="$(basename "$app")"
  case "$name" in
    statesync) check_statesync "$app" ;;
    *)
      if [ -d "$app/frontend/src" ]; then
        check_yew "$app"
      elif [ -d "$app/src/dashboard" ]; then
        check_statesync "$app"
      else
        echo "skip $name (no web UI)"
      fi
      ;;
  esac
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
