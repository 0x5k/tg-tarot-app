#!/bin/bash
set -e

# Install Rust if not already installed
if ! command -v rustc &> /dev/null; then
    curl https://sh.rustup.rs -sSf | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Add Rust to PATH if cargo/env exists
if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
fi

# Install WASM target
rustup target add wasm32-unknown-unknown

# Install trunk
cargo install trunk --version 0.19.0

