#!/bin/bash
#
# Etrid Operations Center - One-Click Installer
# Installs Pinokio and Etrid Ops Dashboard
#

set -e

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  Etrid Operations Center Installer"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Detect OS
OS="unknown"
ARCH="unknown"

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    OS="linux"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    OS="mac"
else
    echo -e "${RED}❌ Unsupported OS: $OSTYPE${NC}"
    exit 1
fi

ARCH=$(uname -m)
if [[ "$ARCH" == "x86_64" ]]; then
    ARCH="x64"
elif [[ "$ARCH" == "arm64" ]] || [[ "$ARCH" == "aarch64" ]]; then
    ARCH="arm64"
else
    echo -e "${RED}❌ Unsupported architecture: $ARCH${NC}"
    exit 1
fi

echo -e "${BLUE}Detected: $OS-$ARCH${NC}"
echo ""

# Check if Pinokio is already installed
PINOKIO_DIR="$HOME/pinokio"
PINOKIO_INSTALLED=false

if [ -d "$PINOKIO_DIR" ]; then
    echo -e "${GREEN}✓ Pinokio directory found at $PINOKIO_DIR${NC}"
    PINOKIO_INSTALLED=true
else
    echo -e "${YELLOW}! Pinokio not found - will install${NC}"
fi

# Install Pinokio if needed
if [ "$PINOKIO_INSTALLED" = false ]; then
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "  Installing Pinokio"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""

    # Get latest Pinokio version
    echo -e "${BLUE}Fetching latest Pinokio release...${NC}"

    PINOKIO_VERSION=$(curl -s https://api.github.com/repos/pinokiocomputer/pinokio/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

    if [ -z "$PINOKIO_VERSION" ]; then
        echo -e "${YELLOW}! Could not fetch latest version, using default${NC}"
        PINOKIO_VERSION="3.0.0"
    fi

    echo -e "${GREEN}Latest version: $PINOKIO_VERSION${NC}"

    # Download Pinokio
    DOWNLOAD_URL=""
    if [ "$OS" = "linux" ]; then
        DOWNLOAD_URL="https://github.com/pinokiocomputer/pinokio/releases/download/$PINOKIO_VERSION/Pinokio-$PINOKIO_VERSION.AppImage"
    elif [ "$OS" = "mac" ]; then
        if [ "$ARCH" = "arm64" ]; then
            DOWNLOAD_URL="https://github.com/pinokiocomputer/pinokio/releases/download/$PINOKIO_VERSION/Pinokio-$PINOKIO_VERSION-arm64.dmg"
        else
            DOWNLOAD_URL="https://github.com/pinokiocomputer/pinokio/releases/download/$PINOKIO_VERSION/Pinokio-$PINOKIO_VERSION.dmg"
        fi
    fi

    echo ""
    echo -e "${BLUE}Downloading Pinokio from:${NC}"
    echo "$DOWNLOAD_URL"
    echo ""

    DOWNLOAD_PATH="/tmp/pinokio-installer"
    curl -L -o "$DOWNLOAD_PATH" "$DOWNLOAD_URL"

    if [ ! -f "$DOWNLOAD_PATH" ]; then
        echo -e "${RED}❌ Download failed${NC}"
        exit 1
    fi

    echo -e "${GREEN}✓ Downloaded successfully${NC}"

    # Install based on OS
    if [ "$OS" = "linux" ]; then
        echo ""
        echo -e "${YELLOW}⚠️  Please install Pinokio manually:${NC}"
        echo "  1. chmod +x $DOWNLOAD_PATH"
        echo "  2. ./$DOWNLOAD_PATH"
        echo "  3. Follow the installation wizard"
        echo ""
        echo -e "${YELLOW}After installation, run this script again.${NC}"
        exit 0
    elif [ "$OS" = "mac" ]; then
        echo ""
        echo -e "${YELLOW}⚠️  Please install Pinokio manually:${NC}"
        echo "  1. Open $DOWNLOAD_PATH"
        echo "  2. Drag Pinokio to Applications"
        echo "  3. Launch Pinokio at least once"
        echo ""
        echo -e "${YELLOW}After installation, run this script again.${NC}"
        exit 0
    fi
fi

# Install Etrid Operations Center
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  Installing Etrid Operations Center"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Create directories
mkdir -p "$PINOKIO_DIR/api/etrid"
mkdir -p "$PINOKIO_DIR/etrid-dashboard"
mkdir -p "$PINOKIO_DIR/etrid-scripts"

echo -e "${BLUE}Installing API...${NC}"

# Download or copy API files
if [ -d "$(pwd)/api/etrid" ]; then
    echo -e "${BLUE}Copying local files...${NC}"
    cp -r "$(pwd)/api/etrid/"* "$PINOKIO_DIR/api/etrid/"
else
    echo -e "${BLUE}Downloading from GitHub...${NC}"
    REPO_URL="https://github.com/EojEdred/Etrid/tree/main/pinokio-etrid-ops"

    # Clone just the ops directory
    git clone --depth 1 --filter=blob:none --sparse \
        https://github.com/EojEdred/Etrid.git /tmp/etrid-ops

    cd /tmp/etrid-ops
    git sparse-checkout set pinokio-etrid-ops

    cp -r pinokio-etrid-ops/api/etrid/* "$PINOKIO_DIR/api/etrid/"
    cp -r pinokio-etrid-ops/dashboard/* "$PINOKIO_DIR/etrid-dashboard/"
    cp -r pinokio-etrid-ops/scripts/* "$PINOKIO_DIR/etrid-scripts/"

    cd -
    rm -rf /tmp/etrid-ops
fi

echo -e "${GREEN}✓ Files copied${NC}"

# Install Node.js dependencies
echo ""
echo -e "${BLUE}Installing dependencies...${NC}"

cd "$PINOKIO_DIR/api/etrid"
if command -v npm &> /dev/null; then
    npm install
    echo -e "${GREEN}✓ API dependencies installed${NC}"
else
    echo -e "${YELLOW}! npm not found - please install Node.js${NC}"
    echo "  Visit: https://nodejs.org/"
fi

cd "$PINOKIO_DIR/etrid-dashboard"
if command -v npm &> /dev/null; then
    npm install
    echo -e "${GREEN}✓ Dashboard dependencies installed${NC}"
fi

# Create default config if it doesn't exist
if [ ! -f "$PINOKIO_DIR/api/etrid/config.json" ]; then
    echo ""
    echo -e "${BLUE}Creating default configuration...${NC}"

    cat > "$PINOKIO_DIR/api/etrid/config.json" <<'EOF'
{
  "chains": {
    "flarechain": {
      "name": "FlareChain Mainnet",
      "type": "main",
      "nodes": []
    },
    "pbcs": [
      {"name": "BTC-PBC", "ticker": "BTC", "nodes": []},
      {"name": "ETH-PBC", "ticker": "ETH", "nodes": []},
      {"name": "SOL-PBC", "ticker": "SOL", "nodes": []},
      {"name": "BNB-PBC", "ticker": "BNB", "nodes": []},
      {"name": "ADA-PBC", "ticker": "ADA", "nodes": []},
      {"name": "DOGE-PBC", "ticker": "DOGE", "nodes": []},
      {"name": "LINK-PBC", "ticker": "LINK", "nodes": []},
      {"name": "MATIC-PBC", "ticker": "MATIC", "nodes": []},
      {"name": "SC-USDT-PBC", "ticker": "USDT", "nodes": []},
      {"name": "TRX-PBC", "ticker": "TRX", "nodes": []},
      {"name": "XLM-PBC", "ticker": "XLM", "nodes": []},
      {"name": "XRP-PBC", "ticker": "XRP", "nodes": []}
    ]
  },
  "clouds": {
    "aws": {"credentials": "~/.aws/credentials", "nodes": []},
    "gcp": {"keyfile": "~/.gcp/key.json", "nodes": []},
    "azure": {"credentials": "~/.azure/credentials", "nodes": []},
    "digitalocean": {"token": "~/.digitalocean/token", "nodes": []}
  }
}
EOF

    echo -e "${GREEN}✓ Default config created${NC}"
fi

# Success!
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  Installation Complete! 🚀"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo -e "${GREEN}✓ Etrid Operations Center installed successfully${NC}"
echo ""
echo "📝 Next steps:"
echo ""
echo "1. Configure your nodes:"
echo "   ${BLUE}Edit: $PINOKIO_DIR/api/etrid/config.json${NC}"
echo ""
echo "2. Launch Pinokio application"
echo ""
echo "3. In Pinokio, navigate to:"
echo "   ${BLUE}Discover → Local → etrid-dashboard${NC}"
echo ""
echo "4. Click 'Start' to launch the dashboard"
echo ""
echo "5. Access dashboard at:"
echo "   ${BLUE}http://localhost:8080${NC}"
echo ""
echo "6. Enable remote access:"
echo "   ${BLUE}Click 'Share' in Pinokio → Select 'Internet' or 'WiFi'${NC}"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📚 Documentation:"
echo "  - Setup Guide: $PINOKIO_DIR/etrid-ops/README.md"
echo "  - Remote Access: $PINOKIO_DIR/etrid-ops/REMOTE_ACCESS_SETUP.md"
echo "  - Validator Guide: $PINOKIO_DIR/etrid-ops/VALIDATOR_PACKAGE.md"
echo ""
echo "💬 Support:"
echo "  - GitHub: https://github.com/EojEdred/Etrid"
echo "  - Discord: [Your Discord link]"
echo "  - Telegram: [Your Telegram link]"
echo ""
echo -e "${GREEN}Happy validating! 🎉${NC}"
echo ""
