# Building FlareChain Node on macOS

## Requirements

- macOS 10.15+ (Catalina or later)
- Xcode Command Line Tools
- Homebrew (recommended)

## Quick Build

\`\`\`bash
# Install Xcode Command Line Tools
xcode-select --install

# Install dependencies via Homebrew
brew install openssl cmake protobuf

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env

# Setup Rust
rustup default stable
rustup update
rustup target add wasm32-unknown-unknown

# Build
cd ~/Desktop/etrid
cargo build --release

# Binary will be at: ./target/release/flarechain-node
\`\`\`

## Apple Silicon (M1/M2/M3)

Works natively on Apple Silicon. No special configuration needed.

## Intel Macs

Same build process as Apple Silicon.

See [Linux Build Guide](./BUILD_LINUX.md) for detailed troubleshooting.
