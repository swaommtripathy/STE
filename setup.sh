#!/bin/bash

set -e

echo "🚀 Setting up Solana Transfers Substreams on macOS..."

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "📦 Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo "✅ Rust already installed: $(rustc --version)"
fi

# Add wasm32 target
echo "📦 Adding wasm32-unknown-unknown target..."
rustup target add wasm32-unknown-unknown

# Check if substreams is installed
if ! command -v substreams &> /dev/null; then
    echo "📦 Installing Substreams CLI..."
    
    # Detect architecture
    ARCH=$(uname -m)
    if [ "$ARCH" = "arm64" ]; then
        DOWNLOAD_URL="https://github.com/streamingfast/substreams/releases/download/v1.10.5/substreams_darwin_arm64.tar.gz"
    else
        DOWNLOAD_URL="https://github.com/streamingfast/substreams/releases/download/v1.10.5/substreams_darwin_amd64.tar.gz"
    fi
    
    curl -L $DOWNLOAD_URL | tar -xz
    sudo mv substreams /usr/local/bin/
    echo "✅ Substreams CLI installed"
else
    echo "✅ Substreams CLI already installed: $(substreams version)"
fi

# Check for API token
if [ -z "$SUBSTREAMS_API_TOKEN" ]; then
    echo ""
    echo "⚠️  API Token not set!"
    echo "Please follow these steps:"
    echo "1. Visit: https://solana.substreams.pinax.network/"
    echo "2. Login and get your free API key"
    echo "3. Run: export SUBSTREAMS_API_TOKEN='your-api-key-here'"
    echo ""
else
    echo "✅ API token is set"
fi

echo ""
echo "✅ Setup complete!"
echo ""
echo "Next steps:"
echo "1. If you haven't set SUBSTREAMS_API_TOKEN, get your API key from:"
echo "   https://solana.substreams.pinax.network/"
echo ""
echo "2. Build and run:"
echo "   make build"
echo "   make package"
echo "   make run"
echo ""
echo "Or simply run: make run"
