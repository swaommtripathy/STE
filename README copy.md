# Solana Transfers Substreams

This project extracts transfer actions from Solana blockchain blocks 385870151-385870156.

## Prerequisites

1. **Install Rust** (if not already installed):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

2. **Add wasm32 target**:
```bash
rustup target add wasm32-unknown-unknown
```

3. **Install Substreams CLI**:
```bash
brew install streamingfast/tap/substreams
```

Or download directly:
```bash
# For macOS ARM (M1/M2/M3)
curl -L https://github.com/streamingfast/substreams/releases/download/v1.10.5/substreams_darwin_arm64.tar.gz | tar -xz
sudo mv substreams /usr/local/bin/

# For macOS Intel
curl -L https://github.com/streamingfast/substreams/releases/download/v1.10.5/substreams_darwin_amd64.tar.gz | tar -xz
sudo mv substreams /usr/local/bin/
```

4. **Get API Key**:
   - Visit: https://solana.substreams.pinax.network/
   - Login and get your free Pinax API key
   - Set it as an environment variable:
```bash
export SUBSTREAMS_API_TOKEN="your-api-key-here"
```

## Build and Run

### Option 1: Use Makefile

```bash
# Build the project
make build

# Package into SPKG
make package

# Run against the specified blocks
make run
```

### Option 2: Manual commands

```bash
# Build
cargo build --target wasm32-unknown-unknown --release

# Package
substreams pack substreams.yaml

# Run
substreams run -e solana.substreams.pinax.network:443 \
  solana-transfers-v0.1.0.spkg \
  map_transfers \
  --start-block 385870151 \
  --stop-block 385870157
```

## Output Format

Each transfer is outputted in JSON format:
```json
{"from": "Eq1c3uNugn3FRpJ99K2q1ZyQrora3jntUqvkMnGroKkB", "to": "4XpSNfoNfireSgdg8hY6Ng3dWrHjewZhmCancypjbcCN", "amount": 325253}
```

## Project Structure

- `src/lib.rs` - Main Rust module that extracts transfers
- `proto/transfers.proto` - Protobuf schema definition
- `substreams.yaml` - Substreams manifest
- `Cargo.toml` - Rust dependencies
- `Makefile` - Build automation

## How it Works

The module:
1. Iterates through all transactions in each block
2. Checks both regular and inner instructions
3. Filters for System Program (11111111111111111111111111111111) instructions
4. Parses Transfer instructions (type 2)
5. Extracts from address, to address, and amount (in lamports)
6. Outputs in the required JSON format
