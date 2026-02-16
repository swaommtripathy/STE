# 🚀 Quick Start Guide - Solana Transfers Substreams Assignment

## What This Project Does
Extracts transfer actions from Solana blockchain blocks 385870151-385870156 and outputs them in JSON format.

## Installation & Setup (5 minutes)

### Step 1: Navigate to the project
```bash
cd solana-transfers
```

### Step 2: Run the automated setup
```bash
./setup.sh
```

This script will:
- Install Rust (if needed)
- Add wasm32 target for WASM compilation
- Install Substreams CLI
- Check for API token

### Step 3: Get Your API Key
1. Visit: https://solana.substreams.pinax.network/
2. Click "Login" and sign in (free account)
3. Copy your API key
4. Set it in your terminal:
```bash
export SUBSTREAMS_API_TOKEN='your-api-key-here'
```

💡 **Tip**: Add this to your `~/.zshrc` or `~/.bash_profile` to make it permanent:
```bash
echo 'export SUBSTREAMS_API_TOKEN="your-api-key-here"' >> ~/.zshrc
source ~/.zshrc
```

## Building & Running

### Option 1: One Command (Easiest)
```bash
make run
```

This will automatically:
1. Build the Rust code to WASM
2. Package it into an SPKG file
3. Run it against blocks 385870151-385870156

### Option 2: Step by Step
```bash
# 1. Build
make build

# 2. Package
make package

# 3. Run
make run
```

### Option 3: Manual Commands
```bash
# Build the Rust code to WASM
cargo build --target wasm32-unknown-unknown --release

# Package into SPKG
substreams pack substreams.yaml

# Run against Solana blocks
substreams run -e solana.substreams.pinax.network:443 \
  solana-transfers-v0.1.0.spkg \
  map_transfers \
  --start-block 385870151 \
  --stop-block 385870157
```

## Expected Output

You should see transfers in this format:
```json
{"from": "Eq1c3uNugn3FRpJ99K2q1ZyQrora3jntUqvkMnGroKkB", "to": "4XpSNfoNfireSgdg8hY6Ng3dWrHjewZhmCancypjbcCN", "amount": 325253}
{"from": "SendAddress111111111111111111111111111", "to": "RecvAddress222222222222222222222222222", "amount": 1000000}
```

## How It Works

The Substreams module:
1. **Processes each block** (385870151 to 385870156)
2. **Iterates through all transactions** in the block
3. **Checks both regular and inner instructions**
4. **Filters for System Program** (ID: 11111111111111111111111111111111)
5. **Parses Transfer instructions** (instruction type 2)
6. **Extracts**:
   - `from`: Sender's wallet address (base58)
   - `to`: Recipient's wallet address (base58)
   - `amount`: Transfer amount in lamports (u64)
7. **Outputs JSON** for each transfer found

## Project Files

```
solana-transfers/
├── src/
│   ├── lib.rs              # Main Rust code - extracts transfers
│   └── pb/
│       └── mod.rs          # Generated protobuf code
├── proto/
│   └── transfers.proto     # Protobuf schema definition
├── Cargo.toml              # Rust dependencies
├── substreams.yaml         # Substreams manifest
├── Makefile                # Build automation
├── setup.sh                # Automated setup script
├── README.md               # Detailed documentation
└── QUICKSTART.md           # This file
```

## Troubleshooting

### "command not found: rustc"
Run: `source $HOME/.cargo/env`

### "command not found: substreams"
The setup script should install it. If not, manually run:
```bash
brew install streamingfast/tap/substreams
```

### "authentication error"
Make sure you've set your API token:
```bash
export SUBSTREAMS_API_TOKEN='your-api-key-here'
```

### Build errors
Try cleaning and rebuilding:
```bash
make clean
make build
```

### Network errors
The endpoint might be temporarily unavailable. Try again in a few moments.

## What Makes This Assignment Challenging?

1. **New Technology Stack**: Rust + WASM + Substreams + Blockchain
2. **Solana Complexity**: Understanding transactions, instructions, accounts
3. **System Program**: Identifying and parsing transfer instructions
4. **Binary Parsing**: Decoding instruction data (u32 + u64 little endian)
5. **Inner Instructions**: Checking both regular and inner instructions

## Success Criteria ✅

- ✅ Valid SPKG file compiles without errors
- ✅ Runs successfully against blocks 385870151-385870156
- ✅ Outputs correct JSON format: `{"from": "...", "to": "...", "amount": 123456}`

## Need Help?

- **Substreams Docs**: https://docs.substreams.dev/
- **Solana Docs**: https://solana.com/docs
- **Use AI Tools**: ChatGPT, Claude, etc. to debug errors

## Final Step: Submitting

Once you've successfully run the substreams and verified the output:

1. The SPKG file is: `solana-transfers-v0.1.0.spkg`
2. Share this file along with your output logs
3. Explain any challenges you faced and how you solved them

Good luck! 🚀
