#!/bin/bash

# Path to your Rust projects folder
PROJECTS_DIR="$HOME/Documents/code/rust/journey"

# Check if the folder exists
if [ ! -d "$PROJECTS_DIR" ]; then
    echo "❌ Error: Directory '$PROJECTS_DIR' does not exist."
    exit 1
fi

echo "🧹 Starting cargo clean for projects in: $PROJECTS_DIR"
echo "------------------------------------------------------"

# Track how many were cleaned
CLEAN_COUNT=0
SKIP_COUNT=0

# Loop through each subfolder
for dir in "$PROJECTS_DIR"/*/; do
    PROJECT_NAME=$(basename "$dir")

    # Check if Cargo.toml exists
    if [ -f "$dir/Cargo.toml" ]; then
        echo "📦 Found Rust project: $PROJECT_NAME"
        echo "   → Running 'cargo clean'..."
        (cd "$dir" && cargo clean)
        echo "   ✅ Cleaned: $PROJECT_NAME"
        CLEAN_COUNT=$((CLEAN_COUNT + 1))
    else
        echo "⚠️ Skipping '$PROJECT_NAME' — not a Rust project (no Cargo.toml found)."
        SKIP_COUNT=$((SKIP_COUNT + 1))
    fi

    echo "------------------------------------------------------"
done

echo "✨ Cleaning complete!"
echo "✅ Projects cleaned: $CLEAN_COUNT"
echo "⚠️ Non-Rust folders skipped: $SKIP_COUNT"
