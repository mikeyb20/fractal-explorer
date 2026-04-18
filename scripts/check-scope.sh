#!/usr/bin/env bash
# PreToolUse hook: enforces per-workstream scope during parallel Tier 2 work.
# Reads allowed file paths from .claude/scope.txt (one per line).
# If the file doesn't exist, this is a no-op (no restrictions).
# Hook is opt-in — add to .claude/settings.json when starting a parallel workstream.
set -euo pipefail

FILE_PATH="${1:-}"
SCOPE_FILE=".claude/scope.txt"

if [ ! -f "$SCOPE_FILE" ]; then
  exit 0
fi

if [ -z "$FILE_PATH" ]; then
  exit 0
fi

if ! grep -qxF "$FILE_PATH" "$SCOPE_FILE"; then
  echo "SCOPE VIOLATION: $FILE_PATH is not in the allowed scope"
  echo "Allowed files are listed in $SCOPE_FILE"
  exit 1
fi
