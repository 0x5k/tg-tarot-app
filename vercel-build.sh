#!/bin/bash
set -e

# Source cargo environment if rustup is in PATH
if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
fi

# Build with trunk
trunk build --release --public-url .

