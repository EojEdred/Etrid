#!/bin/bash

# Ëtrid Wallet - Android Build Script
# This script automates the Android app build process

set -e  # Exit on error

echo "🚀 Ëtrid Wallet - Android Build Script"
echo "========================================"
echo ""

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if we're in the right directory
if [ ! -f "package.json" ]; then
    echo -e "${RED}❌ Error: Must run from etrid-wallet-native directory${NC}"
    exit 1
fi

# Check Node.js
echo -e "${BLUE}Checking Node.js...${NC}"
if ! command -v node &> /dev/null; then
    echo -e "${RED}❌ Node.js not found. Please install Node.js 18+${NC}"
    exit 1
fi
NODE_VERSION=$(node -v)
echo -e "${GREEN}✓ Node.js: $NODE_VERSION${NC}"

# Check npm
if ! command -v npm &> /dev/null; then
    echo -e "${RED}❌ npm not found${NC}"
    exit 1
fi
NPM_VERSION=$(npm -v)
echo -e "${GREEN}✓ npm: $NPM_VERSION${NC}"

# Check Java
echo -e "${BLUE}Checking Java...${NC}"
if ! command -v java &> /dev/null; then
    echo -e "${RED}❌ Java not found. Please install JDK 17+${NC}"
    exit 1
fi
JAVA_VERSION=$(java -version 2>&1 | head -1)
echo -e "${GREEN}✓ Java: $JAVA_VERSION${NC}"

# Check ANDROID_HOME
echo -e "${BLUE}Checking Android SDK...${NC}"
if [ -z "$ANDROID_HOME" ]; then
    echo -e "${YELLOW}⚠ ANDROID_HOME not set${NC}"
    echo -e "${YELLOW}Please set ANDROID_HOME to your Android SDK path:${NC}"
    echo -e "${YELLOW}export ANDROID_HOME=\$HOME/Android/Sdk${NC}"
    echo -e "${YELLOW}export PATH=\$PATH:\$ANDROID_HOME/platform-tools${NC}"
    echo ""
    read -p "Continue anyway? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
else
    echo -e "${GREEN}✓ ANDROID_HOME: $ANDROID_HOME${NC}"
fi

echo ""
echo -e "${BLUE}Step 1: Installing npm dependencies...${NC}"
if [ ! -d "node_modules" ]; then
    npm install --legacy-peer-deps
    echo -e "${GREEN}✓ Dependencies installed${NC}"
else
    echo -e "${YELLOW}⚠ node_modules exists, skipping install${NC}"
    read -p "Reinstall dependencies? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        rm -rf node_modules package-lock.json
        npm install --legacy-peer-deps
        echo -e "${GREEN}✓ Dependencies reinstalled${NC}"
    fi
fi

echo ""
echo -e "${BLUE}Step 2: Setting up Gradle wrapper...${NC}"
cd android
if [ ! -f "gradlew" ]; then
    gradle wrapper --gradle-version=8.10.2
    chmod +x gradlew
    echo -e "${GREEN}✓ Gradle wrapper created${NC}"
else
    echo -e "${GREEN}✓ Gradle wrapper exists${NC}"
fi

echo ""
echo -e "${BLUE}Step 3: Cleaning previous builds...${NC}"
./gradlew clean
echo -e "${GREEN}✓ Clean complete${NC}"

echo ""
echo "Select build type:"
echo "1) Debug APK (for testing)"
echo "2) Release APK (production, unsigned)"
echo "3) Release APK (production, signed)"
echo "4) Release AAB (for Play Store)"
read -p "Enter choice (1-4): " BUILD_CHOICE

case $BUILD_CHOICE in
    1)
        echo ""
        echo -e "${BLUE}Step 4: Building DEBUG APK...${NC}"
        ./gradlew assembleDebug

        APK_PATH="app/build/outputs/apk/debug/app-debug.apk"
        if [ -f "$APK_PATH" ]; then
            APK_SIZE=$(du -h "$APK_PATH" | cut -f1)
            echo ""
            echo -e "${GREEN}✅ SUCCESS! Debug APK built${NC}"
            echo -e "${GREEN}📱 Location: android/$APK_PATH${NC}"
            echo -e "${GREEN}📦 Size: $APK_SIZE${NC}"
            echo ""
            echo "To install on device:"
            echo "  adb install $APK_PATH"
        else
            echo -e "${RED}❌ Build failed - APK not found${NC}"
            exit 1
        fi
        ;;

    2)
        echo ""
        echo -e "${BLUE}Step 4: Building RELEASE APK (unsigned)...${NC}"
        ./gradlew assembleRelease

        APK_PATH="app/build/outputs/apk/release/app-release-unsigned.apk"
        if [ -f "$APK_PATH" ]; then
            APK_SIZE=$(du -h "$APK_PATH" | cut -f1)
            echo ""
            echo -e "${GREEN}✅ SUCCESS! Release APK built (unsigned)${NC}"
            echo -e "${GREEN}📱 Location: android/$APK_PATH${NC}"
            echo -e "${GREEN}📦 Size: $APK_SIZE${NC}"
            echo ""
            echo -e "${YELLOW}⚠ Note: This APK is UNSIGNED and cannot be installed${NC}"
            echo -e "${YELLOW}   Use option 3 to create a signed APK${NC}"
        else
            echo -e "${RED}❌ Build failed - APK not found${NC}"
            exit 1
        fi
        ;;

    3)
        echo ""
        echo -e "${BLUE}Step 4: Building SIGNED Release APK...${NC}"

        # Check for keystore
        if [ ! -f "app/etrid-wallet.keystore" ]; then
            echo -e "${YELLOW}⚠ Keystore not found. Creating new keystore...${NC}"
            echo ""
            echo "Enter keystore details:"
            read -p "Store password: " -s STORE_PASS
            echo ""
            read -p "Key password: " -s KEY_PASS
            echo ""
            read -p "Your name: " DEV_NAME
            read -p "Organization: " DEV_ORG
            read -p "City: " DEV_CITY
            read -p "State: " DEV_STATE
            read -p "Country code (US): " DEV_COUNTRY

            keytool -genkey -v \
                -keystore app/etrid-wallet.keystore \
                -alias etrid-wallet-key \
                -keyalg RSA \
                -keysize 2048 \
                -validity 10000 \
                -storepass "$STORE_PASS" \
                -keypass "$KEY_PASS" \
                -dname "CN=$DEV_NAME, OU=$DEV_ORG, O=$DEV_ORG, L=$DEV_CITY, ST=$DEV_STATE, C=$DEV_COUNTRY"

            echo ""
            echo -e "${GREEN}✓ Keystore created${NC}"
            echo -e "${RED}IMPORTANT: Backup app/etrid-wallet.keystore - you'll need it for updates!${NC}"

            # Create gradle.properties
            cat > gradle.properties << EOF
MYAPP_UPLOAD_STORE_FILE=etrid-wallet.keystore
MYAPP_UPLOAD_KEY_ALIAS=etrid-wallet-key
MYAPP_UPLOAD_STORE_PASSWORD=$STORE_PASS
MYAPP_UPLOAD_KEY_PASSWORD=$KEY_PASS
EOF
            echo -e "${GREEN}✓ Signing configuration saved${NC}"
        fi

        ./gradlew assembleRelease

        APK_PATH="app/build/outputs/apk/release/app-release.apk"
        if [ -f "$APK_PATH" ]; then
            APK_SIZE=$(du -h "$APK_PATH" | cut -f1)
            echo ""
            echo -e "${GREEN}✅ SUCCESS! Signed Release APK built${NC}"
            echo -e "${GREEN}📱 Location: android/$APK_PATH${NC}"
            echo -e "${GREEN}📦 Size: $APK_SIZE${NC}"
            echo ""
            echo "Ready to distribute! You can:"
            echo "  • Share this APK directly with users"
            echo "  • Upload to third-party app stores"
            echo "  • Install: adb install $APK_PATH"
        else
            echo -e "${RED}❌ Build failed - APK not found${NC}"
            exit 1
        fi
        ;;

    4)
        echo ""
        echo -e "${BLUE}Step 4: Building Release AAB (for Play Store)...${NC}"

        if [ ! -f "gradle.properties" ]; then
            echo -e "${RED}❌ Signing not configured. Run option 3 first to create keystore${NC}"
            exit 1
        fi

        ./gradlew bundleRelease

        AAB_PATH="app/build/outputs/bundle/release/app-release.aab"
        if [ -f "$AAB_PATH" ]; then
            AAB_SIZE=$(du -h "$AAB_PATH" | cut -f1)
            echo ""
            echo -e "${GREEN}✅ SUCCESS! Release AAB built${NC}"
            echo -e "${GREEN}📱 Location: android/$AAB_PATH${NC}"
            echo -e "${GREEN}📦 Size: $AAB_SIZE${NC}"
            echo ""
            echo "Ready for Google Play Store upload!"
            echo "  1. Go to Google Play Console"
            echo "  2. Create new release"
            echo "  3. Upload this AAB file"
        else
            echo -e "${RED}❌ Build failed - AAB not found${NC}"
            exit 1
        fi
        ;;

    *)
        echo -e "${RED}Invalid choice${NC}"
        exit 1
        ;;
esac

cd ..

echo ""
echo -e "${GREEN}🎉 Build complete!${NC}"
echo ""
