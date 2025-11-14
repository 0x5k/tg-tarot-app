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

# Install trunk - use version 0.16.0 which is known to compile reliably
# This avoids dependency compatibility issues with newer versions
echo "Installing trunk version 0.16.0..."
if ! cargo install trunk --version 0.16.0 --locked; then
    echo "Cargo install failed, trying to download pre-built binary..."
    # Download pre-built trunk binary for Linux x86_64
    mkdir -p "$HOME/.cargo/bin"
    TRUNK_VERSION="v0.16.0"
    ARCH="x86_64-unknown-linux-musl"
    curl -L "https://github.com/thedodd/trunk/releases/download/${TRUNK_VERSION}/trunk-${ARCH}.tar.gz" | \
        tar -xz -C "$HOME/.cargo/bin"
    chmod +x "$HOME/.cargo/bin/trunk" || \
    { echo "Failed to install trunk via cargo or binary download" && exit 1; }
fi

