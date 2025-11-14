#!/bin/bash
set -euo pipefail

export PATH="$HOME/.cargo/bin:$PATH"

if ! command -v rustup >/dev/null; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
fi

rustup target add wasm32-unknown-unknown >/dev/null 2>&1 || true
cargo install trunk --locked >/dev/null 2>&1 || true

trunk build --release --public-url .
