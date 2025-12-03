#!/bin/bash

# Ëtrid Wallet - iOS Build Script
# This script automates the iOS app build process
# REQUIRES: macOS with Xcode installed

set -e  # Exit on error

echo "🍎 Ëtrid Wallet - iOS Build Script"
echo "==================================="
echo ""

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if running on macOS
if [[ "$OSTYPE" != "darwin"* ]]; then
    echo -e "${RED}❌ Error: iOS builds require macOS${NC}"
    echo "Please run this script on a Mac with Xcode installed"
    exit 1
fi

# Check if we're in the right directory
if [ ! -f "package.json" ]; then
    echo -e "${RED}❌ Error: Must run from etrid-wallet-native directory${NC}"
    exit 1
fi

# Check Xcode
echo -e "${BLUE}Checking Xcode...${NC}"
if ! command -v xcodebuild &> /dev/null; then
    echo -e "${RED}❌ Xcode not found${NC}"
    echo "Please install Xcode from the App Store"
    exit 1
fi
XCODE_VERSION=$(xcodebuild -version | head -1)
echo -e "${GREEN}✓ $XCODE_VERSION${NC}"

# Check Node.js
echo -e "${BLUE}Checking Node.js...${NC}"
if ! command -v node &> /dev/null; then
    echo -e "${RED}❌ Node.js not found. Please install Node.js 18+${NC}"
    exit 1
fi
NODE_VERSION=$(node -v)
echo -e "${GREEN}✓ Node.js: $NODE_VERSION${NC}"

# Check CocoaPods
echo -e "${BLUE}Checking CocoaPods...${NC}"
if ! command -v pod &> /dev/null; then
    echo -e "${YELLOW}⚠ CocoaPods not found${NC}"
    echo "Installing CocoaPods..."
    sudo gem install cocoapods
    echo -e "${GREEN}✓ CocoaPods installed${NC}"
else
    POD_VERSION=$(pod --version)
    echo -e "${GREEN}✓ CocoaPods: $POD_VERSION${NC}"
fi

echo ""
echo -e "${BLUE}Step 1: Installing npm dependencies...${NC}"
if [ ! -d "node_modules" ]; then
    npm install --legacy-peer-deps
    echo -e "${GREEN}✓ Dependencies installed${NC}"
else
    echo -e "${YELLOW}⚠ node_modules exists${NC}"
    read -p "Reinstall dependencies? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        rm -rf node_modules package-lock.json
        npm install --legacy-peer-deps
        echo -e "${GREEN}✓ Dependencies reinstalled${NC}"
    fi
fi

echo ""
echo -e "${BLUE}Step 2: Installing iOS pods...${NC}"
cd ios
if [ ! -d "Pods" ]; then
    pod install
    echo -e "${GREEN}✓ Pods installed${NC}"
else
    echo -e "${YELLOW}⚠ Pods directory exists${NC}"
    read -p "Reinstall pods? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        pod cache clean --all
        pod deintegrate
        pod install
        echo -e "${GREEN}✓ Pods reinstalled${NC}"
    fi
fi
cd ..

echo ""
echo "Select build type:"
echo "1) Debug (for Simulator)"
echo "2) Debug (for Physical Device)"
echo "3) Release Archive (for App Store/TestFlight)"
echo "4) Open in Xcode (manual build)"
read -p "Enter choice (1-4): " BUILD_CHOICE

case $BUILD_CHOICE in
    1)
        echo ""
        echo -e "${BLUE}Step 3: Building for iOS Simulator...${NC}"
        echo "Available simulators:"
        xcrun simctl list devices | grep "iPhone"
        echo ""
        read -p "Enter device name (e.g., 'iPhone 15 Pro'): " DEVICE_NAME

        cd ios
        xcodebuild -workspace EtridWallet.xcworkspace \
            -scheme EtridWallet \
            -configuration Debug \
            -sdk iphonesimulator \
            -destination "platform=iOS Simulator,name=$DEVICE_NAME" \
            build

        echo ""
        echo -e "${GREEN}✅ SUCCESS! Debug build for simulator complete${NC}"
        echo ""
        echo "To run:"
        echo "  1. Open Simulator app"
        echo "  2. Select '$DEVICE_NAME'"
        echo "  3. Drag the app from build/Debug-iphonesimulator/ to simulator"
        cd ..
        ;;

    2)
        echo ""
        echo -e "${BLUE}Step 3: Building for Physical Device...${NC}"
        echo "Connect your iPhone/iPad via USB"
        read -p "Press Enter when device is connected..."

        # List connected devices
        echo "Connected devices:"
        xcrun xctrace list devices | grep "iPhone\|iPad"

        cd ios
        xcodebuild -workspace EtridWallet.xcworkspace \
            -scheme EtridWallet \
            -configuration Debug \
            -sdk iphoneos \
            -allowProvisioningUpdates \
            build

        echo ""
        echo -e "${GREEN}✅ SUCCESS! Debug build for device complete${NC}"
        echo ""
        echo "The app should now be installed on your device"
        cd ..
        ;;

    3)
        echo ""
        echo -e "${BLUE}Step 3: Creating Release Archive...${NC}"
        echo ""
        echo -e "${YELLOW}⚠ Make sure you have:${NC}"
        echo "  • Apple Developer account"
        echo "  • Signing certificate configured in Xcode"
        echo "  • Provisioning profile set up"
        echo ""
        read -p "Continue? (y/n) " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            exit 1
        fi

        cd ios

        # Clean build folder
        echo "Cleaning build folder..."
        xcodebuild clean -workspace EtridWallet.xcworkspace -scheme EtridWallet

        # Create archive
        echo "Creating archive..."
        xcodebuild -workspace EtridWallet.xcworkspace \
            -scheme EtridWallet \
            -configuration Release \
            -archivePath ../build/EtridWallet.xcarchive \
            -allowProvisioningUpdates \
            archive

        if [ -d "../build/EtridWallet.xcarchive" ]; then
            echo ""
            echo -e "${GREEN}✅ SUCCESS! Archive created${NC}"
            echo -e "${GREEN}📁 Location: build/EtridWallet.xcarchive${NC}"
            echo ""
            echo "Next steps:"
            echo "  1. Open Xcode"
            echo "  2. Window → Organizer"
            echo "  3. Select the archive"
            echo "  4. Click 'Distribute App'"
            echo "  5. Choose 'App Store Connect' for App Store/TestFlight"
            echo "  6. OR choose 'Development' for internal testing"
        else
            echo -e "${RED}❌ Archive creation failed${NC}"
            exit 1
        fi
        cd ..
        ;;

    4)
        echo ""
        echo -e "${BLUE}Opening in Xcode...${NC}"
        open ios/EtridWallet.xcworkspace
        echo ""
        echo -e "${GREEN}✅ Xcode opened${NC}"
        echo ""
        echo "In Xcode:"
        echo "  • Select your target device"
        echo "  • Press ⌘R to run"
        echo "  • Product → Archive for release builds"
        ;;

    *)
        echo -e "${RED}Invalid choice${NC}"
        exit 1
        ;;
esac

echo ""
echo -e "${GREEN}🎉 iOS build script complete!${NC}"
echo ""
