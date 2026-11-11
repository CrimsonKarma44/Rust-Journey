#!/usr/bin/env bash
#
# Run `cargo clean` on every Cargo project in this repo that has a target/ folder.
#
# Usage:
#   ./clean_rust_projects.sh            # clean everything under the repo
#   ./clean_rust_projects.sh -n         # dry run: show what would be cleaned
#   ./clean_rust_projects.sh some/dir   # clean only under some/dir
#
# Cargo writes build output to target/ next to the workspace root's Cargo.toml,
# so a target/ folder that sits beside a Cargo.toml marks something to clean.
# Workspace members share their root's target/, so they are covered without
# being listed one by one. Nested projects (e.g. works/*) are found too.

set -euo pipefail

DRY_RUN=false
if [[ "${1:-}" == "-n" || "${1:-}" == "--dry-run" ]]; then
    DRY_RUN=true
    shift
fi

# Default to the folder this script lives in, wherever the repo is cloned.
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="${1:-$SCRIPT_DIR}"

if [[ ! -d "$ROOT" ]]; then
    echo "❌ Error: directory '$ROOT' does not exist." >&2
    exit 1
fi
if ! command -v cargo >/dev/null 2>&1; then
    echo "❌ Error: cargo not found on PATH. Install Rust from https://rustup.rs" >&2
    exit 1
fi

ROOT="$(cd "$ROOT" && pwd)"
echo "🧹 Looking for Cargo build output under: $ROOT"
$DRY_RUN && echo "   (dry run: nothing will be deleted)"
echo "------------------------------------------------------"

CLEAN_COUNT=0
FAIL_COUNT=0

# -prune stops find from descending into target/ or .git, which is both faster
# and avoids matching Cargo.toml files that Cargo copied into build output.
# -print0 / read -d '' keeps paths with spaces (e.g. "13_ Functional_...") intact.
while IFS= read -r -d '' target_dir; do
    project_dir="$(dirname "$target_dir")"
    [[ -f "$project_dir/Cargo.toml" ]] || continue

    rel="${project_dir#"$ROOT"/}"
    [[ "$project_dir" == "$ROOT" ]] && rel="."
    size="$(du -sh "$target_dir" 2>/dev/null | cut -f1)"

    if $DRY_RUN; then
        echo "📦 would clean: $rel ($size)"
        CLEAN_COUNT=$((CLEAN_COUNT + 1))
    elif cargo clean --quiet --manifest-path "$project_dir/Cargo.toml"; then
        echo "✅ cleaned: $rel ($size)"
        CLEAN_COUNT=$((CLEAN_COUNT + 1))
    else
        echo "⚠️  failed:  $rel" >&2
        FAIL_COUNT=$((FAIL_COUNT + 1))
    fi
done < <(find "$ROOT" \( -name .git -prune \) -o \( -type d -name target -prune -print0 \))

echo "------------------------------------------------------"
if [[ $CLEAN_COUNT -eq 0 && $FAIL_COUNT -eq 0 ]]; then
    echo "✨ Nothing to clean: no target/ folders found."
else
    $DRY_RUN && verb="would be cleaned" || verb="cleaned"
    echo "✨ Done. Projects $verb: $CLEAN_COUNT"
    [[ $FAIL_COUNT -gt 0 ]] && echo "⚠️  Failed: $FAIL_COUNT"
fi
[[ $FAIL_COUNT -eq 0 ]]
