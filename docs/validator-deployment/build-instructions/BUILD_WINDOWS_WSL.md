# Building FlareChain Node on Windows WSL

## Requirements

- Windows 10/11 with WSL2
- Ubuntu 22.04 LTS (recommended WSL distro)

## Setup WSL

\`\`\`powershell
# In PowerShell (as Administrator)
wsl --install -d Ubuntu-22.04
\`\`\`

## Build in WSL

\`\`\`bash
# Inside Ubuntu WSL terminal
sudo apt-get update && sudo apt-get upgrade -y

# Install dependencies
sudo apt-get install -y curl git build-essential clang libssl-dev pkg-config protobuf-compiler

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env

# Setup Rust
rustup default stable
rustup target add wasm32-unknown-unknown

# Build
cd /mnt/c/Users/YourName/Desktop/etrid
cargo build --release
\`\`\`

Binary will be at: \`./target/release/flarechain-node\`

See [Linux Build Guide](./BUILD_LINUX.md) for detailed instructions.
