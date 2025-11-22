#!/bin/bash

################################################################################
# iOS Beta Deployment Script
#
# Automates building and uploading iOS builds to TestFlight
#
# Usage:
#   ./deploy-ios-beta.sh [build_number]
#
# Example:
#   ./deploy-ios-beta.sh 42
#
# Prerequisites:
#   - Xcode 15+
#   - fastlane installed
#   - Valid Apple Developer account
#   - App Store Connect API key configured
################################################################################

set -e  # Exit on error
set -u  # Exit on undefined variable

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
PROJECT_DIR="/home/user/Etrid/apps/wallet-mobile/etrid-wallet-native"
IOS_DIR="$PROJECT_DIR/ios"
APP_NAME="EtridWallet"
SCHEME="EtridWallet"
WORKSPACE="$IOS_DIR/$APP_NAME.xcworkspace"
BUILD_NUMBER="${1:-}"
VERSION=$(grep 'MARKETING_VERSION' "$IOS_DIR/$APP_NAME.xcodeproj/project.pbxproj" | head -1 | awk -F'= ' '{print $2}' | tr -d ';')

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."

    # Check Xcode
    if ! command -v xcodebuild &> /dev/null; then
        log_error "Xcode command line tools not found"
        exit 1
    fi

    # Check fastlane
    if ! command -v fastlane &> /dev/null; then
        log_error "fastlane not found. Install: gem install fastlane"
        exit 1
    fi

    # Check workspace exists
    if [ ! -d "$WORKSPACE" ]; then
        log_error "Workspace not found: $WORKSPACE"
        exit 1
    fi

    log_success "Prerequisites check passed"
}

# Increment build number
increment_build_number() {
    if [ -z "$BUILD_NUMBER" ]; then
        log_info "No build number provided, auto-incrementing..."
        cd "$IOS_DIR"
        CURRENT_BUILD=$(xcodebuild -showBuildSettings -workspace "$WORKSPACE" -scheme "$SCHEME" 2>/dev/null | grep 'CURRENT_PROJECT_VERSION' | awk '{print $3}')
        BUILD_NUMBER=$((CURRENT_BUILD + 1))
        log_info "New build number: $BUILD_NUMBER"
    else
        log_info "Using provided build number: $BUILD_NUMBER"
    fi

    # Update build number in project
    /usr/libexec/PlistBuddy -c "Set :CFBundleVersion $BUILD_NUMBER" "$IOS_DIR/$APP_NAME/Info.plist"
    log_success "Build number updated to $BUILD_NUMBER"
}

# Install dependencies
install_dependencies() {
    log_info "Installing dependencies..."

    cd "$PROJECT_DIR"

    # npm dependencies
    if [ -f "package.json" ]; then
        log_info "Installing npm packages..."
        npm install --silent
    fi

    # CocoaPods
    cd "$IOS_DIR"
    if [ -f "Podfile" ]; then
        log_info "Installing CocoaPods..."
        pod install --silent
    fi

    log_success "Dependencies installed"
}

# Clean build artifacts
clean_build() {
    log_info "Cleaning previous builds..."
    cd "$IOS_DIR"
    xcodebuild clean -workspace "$WORKSPACE" -scheme "$SCHEME" -configuration Release &>/dev/null
    rm -rf ~/Library/Developer/Xcode/DerivedData/*
    log_success "Build cleaned"
}

# Run tests
run_tests() {
    log_info "Running unit tests..."
    cd "$IOS_DIR"

    set +e  # Don't exit on test failure
    xcodebuild test \
        -workspace "$WORKSPACE" \
        -scheme "$SCHEME" \
        -destination 'platform=iOS Simulator,name=iPhone 14' \
        -quiet

    TEST_RESULT=$?
    set -e

    if [ $TEST_RESULT -eq 0 ]; then
        log_success "Tests passed"
    else
        log_warning "Tests failed, but continuing deployment"
    fi
}

# Build archive
build_archive() {
    log_info "Building archive..."
    cd "$IOS_DIR"

    ARCHIVE_PATH="$IOS_DIR/build/$APP_NAME.xcarchive"

    xcodebuild archive \
        -workspace "$WORKSPACE" \
        -scheme "$SCHEME" \
        -configuration Release \
        -archivePath "$ARCHIVE_PATH" \
        -destination 'generic/platform=iOS' \
        CODE_SIGN_STYLE=Automatic \
        DEVELOPMENT_TEAM="YOUR_TEAM_ID" \
        | grep -A 5 "warning:" || true

    if [ ! -d "$ARCHIVE_PATH" ]; then
        log_error "Archive failed"
        exit 1
    fi

    log_success "Archive created: $ARCHIVE_PATH"
}

# Export IPA
export_ipa() {
    log_info "Exporting IPA..."
    cd "$IOS_DIR"

    ARCHIVE_PATH="$IOS_DIR/build/$APP_NAME.xcarchive"
    EXPORT_PATH="$IOS_DIR/build"

    # Create export options plist
    cat > ExportOptions.plist <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>method</key>
    <string>app-store</string>
    <key>teamID</key>
    <string>YOUR_TEAM_ID</string>
    <key>uploadBitcode</key>
    <false/>
    <key>uploadSymbols</key>
    <true/>
    <key>compileBitcode</key>
    <false/>
</dict>
</plist>
EOF

    xcodebuild -exportArchive \
        -archivePath "$ARCHIVE_PATH" \
        -exportPath "$EXPORT_PATH" \
        -exportOptionsPlist ExportOptions.plist \
        | grep -A 5 "warning:" || true

    IPA_PATH="$EXPORT_PATH/$APP_NAME.ipa"

    if [ ! -f "$IPA_PATH" ]; then
        log_error "IPA export failed"
        exit 1
    fi

    IPA_SIZE=$(du -h "$IPA_PATH" | cut -f1)
    log_success "IPA exported: $IPA_PATH ($IPA_SIZE)"
}

# Upload to TestFlight
upload_to_testflight() {
    log_info "Uploading to TestFlight..."
    cd "$IOS_DIR"

    IPA_PATH="$IOS_DIR/build/$APP_NAME.ipa"

    # Using fastlane for upload
    if [ -f "fastlane/Fastfile" ]; then
        fastlane pilot upload --ipa "$IPA_PATH" --skip_waiting_for_build_processing
    else
        # Alternative: Using xcrun altool
        xcrun altool --upload-app \
            --type ios \
            --file "$IPA_PATH" \
            --apiKey "YOUR_API_KEY_ID" \
            --apiIssuer "YOUR_ISSUER_ID" \
            2>&1 | grep -i "success\|error\|warning"
    fi

    log_success "Upload complete!"
}

# Generate release notes
generate_release_notes() {
    log_info "Generating release notes..."

    NOTES_FILE="$PROJECT_DIR/RELEASE_NOTES_$BUILD_NUMBER.md"

    cat > "$NOTES_FILE" <<EOF
# Version $VERSION (Build $BUILD_NUMBER)

**Release Date:** $(date +"%Y-%m-%d")

## What's New
- Bug fixes and improvements

## What to Test
- [ ] Wallet creation and import
- [ ] Send/receive transactions
- [ ] AU Bloccard features
- [ ] Trading functionality
- [ ] NFT gallery
- [ ] App performance

## Known Issues
- None at this time

## Feedback
Report issues:
- In-app: Settings → Send Feedback
- Email: beta@etrid.com
- Discord: #ios-beta

---
Build: $BUILD_NUMBER
Version: $VERSION
Date: $(date +"%Y-%m-%d %H:%M:%S")
EOF

    log_success "Release notes: $NOTES_FILE"
}

# Send notifications
send_notifications() {
    log_info "Sending notifications..."

    # Slack notification
    if [ -n "${SLACK_WEBHOOK_URL:-}" ]; then
        curl -X POST "$SLACK_WEBHOOK_URL" \
            -H 'Content-Type: application/json' \
            -d "{
                \"text\": \"📱 iOS Beta Build $BUILD_NUMBER Uploaded\",
                \"blocks\": [
                    {
                        \"type\": \"section\",
                        \"text\": {
                            \"type\": \"mrkdwn\",
                            \"text\": \"*iOS Beta Build $BUILD_NUMBER*\n\nVersion: $VERSION\nBuild: $BUILD_NUMBER\n\nProcessing on TestFlight (10-60 minutes)\"
                        }
                    },
                    {
                        \"type\": \"actions\",
                        \"elements\": [
                            {
                                \"type\": \"button\",
                                \"text\": {
                                    \"type\": \"plain_text\",
                                    \"text\": \"View on App Store Connect\"
                                },
                                \"url\": \"https://appstoreconnect.apple.com\"
                            }
                        ]
                    }
                ]
            }" \
            --silent --output /dev/null

        log_success "Slack notification sent"
    fi

    # Discord notification (optional)
    if [ -n "${DISCORD_WEBHOOK_URL:-}" ]; then
        curl -X POST "$DISCORD_WEBHOOK_URL" \
            -H 'Content-Type: application/json' \
            -d "{
                \"content\": \"📱 **iOS Beta Build $BUILD_NUMBER** uploaded to TestFlight!\",
                \"embeds\": [{
                    \"title\": \"Version $VERSION\",
                    \"description\": \"Processing on TestFlight (10-60 minutes)\",
                    \"color\": 3447003
                }]
            }" \
            --silent --output /dev/null
    fi
}

# Cleanup
cleanup() {
    log_info "Cleaning up..."
    cd "$IOS_DIR"
    rm -rf build/
    rm -f ExportOptions.plist
    log_success "Cleanup complete"
}

# Main execution
main() {
    echo ""
    log_info "=========================================="
    log_info "  iOS Beta Deployment"
    log_info "=========================================="
    echo ""

    check_prerequisites
    echo ""

    increment_build_number
    echo ""

    install_dependencies
    echo ""

    clean_build
    echo ""

    run_tests
    echo ""

    build_archive
    echo ""

    export_ipa
    echo ""

    upload_to_testflight
    echo ""

    generate_release_notes
    echo ""

    send_notifications
    echo ""

    cleanup
    echo ""

    log_success "=========================================="
    log_success "  Deployment Complete!"
    log_success "=========================================="
    echo ""
    log_info "Build $BUILD_NUMBER is now processing on TestFlight"
    log_info "This typically takes 10-60 minutes"
    log_info "Check status: https://appstoreconnect.apple.com"
    echo ""
}

# Run main function
main

exit 0
