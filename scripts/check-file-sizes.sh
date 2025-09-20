#!/bin/bash

# File size enforcement script for leptos-next-metadata
# This script checks that all Rust files are under 300 lines

set -e

echo "🔍 Checking file sizes..."

# Find all Rust files and check their line counts
oversized_files=()

# Check src/ directory (exclude backup files)
while IFS= read -r -d '' file; do
  # Skip backup files
  if [[ "$file" == *"_old.rs" ]]; then
    continue
  fi
  line_count=$(wc -l < "$file")
  if [ "$line_count" -gt 300 ]; then
    oversized_files+=("$file:$line_count")
  fi
done < <(find src/ -name "*.rs" -type f -print0)

# Check macros/ directory
while IFS= read -r -d '' file; do
  line_count=$(wc -l < "$file")
  if [ "$line_count" -gt 300 ]; then
    oversized_files+=("$file:$line_count")
  fi
done < <(find macros/ -name "*.rs" -type f -print0)

if [ ${#oversized_files[@]} -gt 0 ]; then
  echo "❌ Found files exceeding 300 lines:"
  for file_info in "${oversized_files[@]}"; do
    file=$(echo "$file_info" | cut -d: -f1)
    lines=$(echo "$file_info" | cut -d: -f2)
    echo "  - $file: $lines lines"
  done
  echo ""
  echo "Please refactor these files to be under 300 lines each."
  echo "Consider splitting them into smaller, focused modules."
  echo ""
  echo "See docs/remediation/FILE_REFACTORING_PLAN.md for guidance."
  exit 1
else
  echo "✅ All files are under 300 lines"
fi
