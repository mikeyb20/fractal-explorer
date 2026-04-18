#!/usr/bin/env bash
# Stop hook: validates the workspace builds and tests pass after each Claude turn.
# Runs from the repo root.
set -euo pipefail

cd "$(dirname "$0")/.."

# Ensure cargo is on PATH even when the hook runs in a non-interactive shell.
if [ -f "$HOME/.cargo/env" ]; then
  # shellcheck disable=SC1091
  . "$HOME/.cargo/env"
fi

if ! cargo build --workspace --quiet 2>&1; then
  echo "BUILD FAILED — fix before continuing"
  exit 1
fi

if ! cargo test --workspace --quiet 2>&1; then
  echo "TESTS FAILED — fix before continuing"
  exit 1
fi

echo "Build and tests passing."
