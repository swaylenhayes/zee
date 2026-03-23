#!/usr/bin/env bash
# zee-sync.sh — Sync zee fork with upstream Zed, rebase, and rebuild.
#
# Usage:
#   ./script/zee-sync.sh              # full sync + rebuild
#   ./script/zee-sync.sh --no-build   # sync + rebase only, skip rebuild
#
# What it does:
#   1. Fetches upstream
#   2. Fast-forwards main to upstream/main
#   3. Pushes main to origin
#   4. Rebases zee onto updated main
#   5. Force-pushes zee to origin (--force-with-lease)
#   6. Rebuilds (cargo build --release) unless --no-build
#
# Safe to run anytime. Stops on conflicts or errors.

set -euo pipefail

FORK_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$FORK_DIR"

NO_BUILD=false
for arg in "$@"; do
  case "$arg" in
    --no-build) NO_BUILD=true ;;
    *) echo "Unknown arg: $arg"; exit 1 ;;
  esac
done

echo "=== zee-sync: fetching upstream ==="
git fetch upstream

BEHIND=$(git rev-list --count main..upstream/main)
if [ "$BEHIND" -eq 0 ]; then
  echo "=== main is already up to date with upstream ==="
else
  echo "=== main is $BEHIND commits behind upstream ==="

  # Check for upstream changes in our modified files
  echo ""
  echo "--- Upstream changes in zee-modified files ---"
  WATCH_FILES=(
    "crates/project_panel/src/project_panel.rs"
    "crates/workspace/src/workspace.rs"
    "crates/workspace/src/dock.rs"
    "crates/workspace/src/workspace_settings.rs"
    "crates/settings_content/src/workspace.rs"
    "assets/settings/default.json"
    "assets/keymaps/default-macos.json"
  )
  TOUCHED=false
  for f in "${WATCH_FILES[@]}"; do
    COUNT=$(git rev-list --count main..upstream/main -- "$f")
    if [ "$COUNT" -gt 0 ]; then
      echo "  ⚠  $f ($COUNT commits)"
      TOUCHED=true
    fi
  done
  if [ "$TOUCHED" = false ]; then
    echo "  (none — clean rebase expected)"
  fi
  echo ""

  echo "=== fast-forwarding main ==="
  git checkout main --quiet
  git merge upstream/main --ff-only

  echo "=== pushing main to origin ==="
  git push origin main
fi

echo "=== rebasing zee onto main ==="
git checkout zee --quiet
git rebase main

echo "=== pushing zee to origin ==="
git push origin zee --force-with-lease

if [ "$NO_BUILD" = true ]; then
  echo ""
  echo "=== zee-sync complete (build skipped) ==="
  echo "Run 'cargo build --release' when ready."
else
  echo ""
  echo "=== building release ==="
  cargo build --release
  echo ""
  echo "=== zee-sync complete ==="
  echo "Restart Zee Dev.app to pick up the new build."
fi
