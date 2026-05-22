#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"

# 1. Build the Rust CLI
echo "==> Building accelerate CLI …"
cargo build -p cli --bin accelerate

# 2. Set env so Electron can find the CLI binary
export ACCELERATE_PATH="${ROOT}/target/debug/accelerate"

# 3. Install desktop deps (only if needed)
if [ ! -d "${ROOT}/desktop/node_modules" ]; then
  echo "==> Installing desktop dependencies …"
  (cd "${ROOT}/desktop" && npm install)
fi

# 4. Start desktop in dev mode
echo "==> Starting RCM Desktop …"
(cd "${ROOT}/desktop" && npm run dev)
