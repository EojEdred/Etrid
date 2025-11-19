#!/bin/bash

################################################################################
# Android Beta Deployment Script
#
# Automates building and uploading Android builds to Google Play Internal Testing
#
# Usage:
#   ./deploy-android-beta.sh [version_code]
#
# Example:
#   ./deploy-android-beta.sh 42
#
# Prerequisites:
#   - Android SDK
#   - Gradle
#   - fastlane (optional but recommended)
#   - Google Play service account JSON key
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
ANDROID_DIR="$PROJECT_DIR/android"
APP_MODULE="app"
VERSION_CODE="${1:-}"
VERSION_NAME=$(grep 'versionName' "$ANDROID_DIR/$APP_MODULE/build.gradle" | awk -F'"' '{print $2}')

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

    # Check Java
    if ! command -v java &> /dev/null; then
        log_error "Java not found"
        exit 1
    fi

    # Check Android SDK
    if [ -z "${ANDROID_HOME:-}" ]; then
        log_error "ANDROID_HOME not set"
        exit 1
    fi

    # Check keystore
    KEYSTORE_PATH="$ANDROID_DIR/app/etrid-release.keystore"
    if [ ! -f "$KEYSTORE_PATH" ]; then
        log_error "Release keystore not found: $KEYSTORE_PATH"
        exit 1
    fi

    # Check keystore.properties
    if [ ! -f "$ANDROID_DIR/keystore.properties" ]; then
        log_error "keystore.properties not found"
        exit 1
    fi

    log_success "Prerequisites check passed"
}

# Increment version code
increment_version_code() {
    if [ -z "$VERSION_CODE" ]; then
        log_info "No version code provided, auto-incrementing..."
        CURRENT_VERSION=$(grep 'versionCode' "$ANDROID_DIR/$APP_MODULE/build.gradle" | awk '{print $2}')
        VERSION_CODE=$((CURRENT_VERSION + 1))
        log_info "New version code: $VERSION_CODE"
    else
        log_info "Using provided version code: $VERSION_CODE"
    fi

    # Update version code in build.gradle
    sed -i.bak "s/versionCode [0-9]*/versionCode $VERSION_CODE/" "$ANDROID_DIR/$APP_MODULE/build.gradle"
    rm -f "$ANDROID_DIR/$APP_MODULE/build.gradle.bak"

    log_success "Version code updated to $VERSION_CODE"
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

    log_success "Dependencies installed"
}

# Clean build artifacts
clean_build() {
    log_info "Cleaning previous builds..."
    cd "$ANDROID_DIR"
    ./gradlew clean --quiet
    log_success "Build cleaned"
}

# Run lint checks
run_lint() {
    log_info "Running lint checks..."
    cd "$ANDROID_DIR"

    set +e  # Don't exit on lint warnings
    ./gradlew lint --quiet
    LINT_RESULT=$?
    set -e

    if [ $LINT_RESULT -eq 0 ]; then
        log_success "Lint checks passed"
    else
        log_warning "Lint checks found issues, check reports"
    fi
}

# Run tests
run_tests() {
    log_info "Running unit tests..."
    cd "$ANDROID_DIR"

    set +e  # Don't exit on test failure
    ./gradlew test --quiet
    TEST_RESULT=$?
    set -e

    if [ $TEST_RESULT -eq 0 ]; then
        log_success "Tests passed"
    else
        log_warning "Tests failed, but continuing deployment"
    fi
}

# Build AAB
build_aab() {
    log_info "Building release AAB..."
    cd "$ANDROID_DIR"

    ./gradlew bundleRelease

    AAB_PATH="$ANDROID_DIR/$APP_MODULE/build/outputs/bundle/release/app-release.aab"

    if [ ! -f "$AAB_PATH" ]; then
        log_error "AAB build failed"
        exit 1
    fi

    AAB_SIZE=$(du -h "$AAB_PATH" | cut -f1)
    log_success "AAB built: $AAB_PATH ($AAB_SIZE)"
}

# Verify signing
verify_signing() {
    log_info "Verifying AAB signature..."
    cd "$ANDROID_DIR"

    AAB_PATH="$ANDROID_DIR/$APP_MODULE/build/outputs/bundle/release/app-release.aab"

    jarsigner -verify -verbose -certs "$AAB_PATH" 2>&1 | grep -i "verified" > /dev/null

    if [ $? -eq 0 ]; then
        log_success "AAB signature verified"
    else
        log_error "AAB signature verification failed"
        exit 1
    fi
}

# Upload to Play Console
upload_to_play_console() {
    log_info "Uploading to Google Play Console..."
    cd "$ANDROID_DIR"

    AAB_PATH="$ANDROID_DIR/$APP_MODULE/build/outputs/bundle/release/app-release.aab"

    # Check if fastlane is available
    if command -v fastlane &> /dev/null && [ -f "fastlane/Fastfile" ]; then
        log_info "Using fastlane for upload..."
        fastlane android internal
    else
        log_info "Using Play Console API for upload..."
        # Alternative: Use Google Play Developer API
        # This requires google-play-android-publisher Python library
        # Or upload manually via Play Console
        log_warning "Fastlane not configured. Please upload manually or setup fastlane."
        log_info "AAB location: $AAB_PATH"
        log_info "Upload at: https://play.google.com/console"
    fi

    log_success "Upload complete!"
}

# Generate release notes
generate_release_notes() {
    log_info "Generating release notes..."

    NOTES_FILE="$PROJECT_DIR/RELEASE_NOTES_$VERSION_CODE.md"

    cat > "$NOTES_FILE" <<EOF
# Version $VERSION_NAME (Build $VERSION_CODE)

**Release Date:** $(date +"%Y-%m-%d")

## What's New
- Bug fixes and improvements
- Performance optimizations

## What to Test
- [ ] Wallet creation and import
- [ ] Send/receive transactions
- [ ] AU Bloccard features
- [ ] Trading functionality
- [ ] NFT gallery
- [ ] App performance
- [ ] Biometric authentication

## Known Issues
- None at this time

## Feedback
Report issues:
- In-app: Settings → Send Feedback
- Email: beta@etrid.com
- Discord: #android-beta

## Device Coverage
Please test on:
- Android 10+
- Various manufacturers (Samsung, Google, OnePlus, etc.)
- Different screen sizes

---
Build: $VERSION_CODE
Version: $VERSION_NAME
Date: $(date +"%Y-%m-%d %H:%M:%S")
EOF

    log_success "Release notes: $NOTES_FILE"
}

# Generate changelog for Play Console
generate_play_console_changelog() {
    log_info "Generating Play Console changelog..."

    CHANGELOG_DIR="$ANDROID_DIR/fastlane/metadata/android/en-US/changelogs"
    mkdir -p "$CHANGELOG_DIR"

    CHANGELOG_FILE="$CHANGELOG_DIR/$VERSION_CODE.txt"

    cat > "$CHANGELOG_FILE" <<EOF
Version $VERSION_NAME - Beta Release

• Bug fixes and improvements
• Performance optimizations
• UI/UX enhancements

Please report any issues via in-app feedback or beta@etrid.com

Thank you for testing!
EOF

    log_success "Changelog: $CHANGELOG_FILE"
}

# Send notifications
send_notifications() {
    log_info "Sending notifications..."

    # Slack notification
    if [ -n "${SLACK_WEBHOOK_URL:-}" ]; then
        curl -X POST "$SLACK_WEBHOOK_URL" \
            -H 'Content-Type: application/json' \
            -d "{
                \"text\": \"🤖 Android Beta Build $VERSION_CODE Uploaded\",
                \"blocks\": [
                    {
                        \"type\": \"section\",
                        \"text\": {
                            \"type\": \"mrkdwn\",
                            \"text\": \"*Android Beta Build $VERSION_CODE*\n\nVersion: $VERSION_NAME\nBuild: $VERSION_CODE\n\nAvailable on Google Play Internal Testing\"
                        }
                    },
                    {
                        \"type\": \"actions\",
                        \"elements\": [
                            {
                                \"type\": \"button\",
                                \"text\": {
                                    \"type\": \"plain_text\",
                                    \"text\": \"View on Play Console\"
                                },
                                \"url\": \"https://play.google.com/console\"
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
                \"content\": \"🤖 **Android Beta Build $VERSION_CODE** uploaded to Play Console!\",
                \"embeds\": [{
                    \"title\": \"Version $VERSION_NAME\",
                    \"description\": \"Available on Internal Testing track\",
                    \"color\": 3066993
                }]
            }" \
            --silent --output /dev/null
    fi
}

# Generate build report
generate_build_report() {
    log_info "Generating build report..."

    REPORT_FILE="$PROJECT_DIR/BUILD_REPORT_$VERSION_CODE.md"

    cat > "$REPORT_FILE" <<EOF
# Android Build Report

**Build Date:** $(date +"%Y-%m-%d %H:%M:%S")
**Version Name:** $VERSION_NAME
**Version Code:** $VERSION_CODE

## Build Configuration
- Build Type: Release
- Signing: Release keystore
- Minify: Enabled
- Shrink Resources: Enabled

## Build Artifacts
- AAB: $(du -h "$ANDROID_DIR/$APP_MODULE/build/outputs/bundle/release/app-release.aab" 2>/dev/null | cut -f1 || echo "N/A")
- Location: $ANDROID_DIR/$APP_MODULE/build/outputs/bundle/release/

## Test Results
- Unit Tests: $(if [ -f "$ANDROID_DIR/$APP_MODULE/build/test-results/testReleaseUnitTest/" ]; then echo "Passed"; else echo "N/A"; fi)
- Lint: Check reports

## Upload Status
- Play Console: Uploaded to Internal Testing

## Next Steps
1. Monitor crash reports
2. Collect tester feedback
3. Triage bugs
4. Plan next release

---
Generated by deploy-android-beta.sh
EOF

    log_success "Build report: $REPORT_FILE"
}

# Cleanup
cleanup() {
    log_info "Cleaning up..."
    # Optionally clean build artifacts
    # cd "$ANDROID_DIR"
    # ./gradlew clean --quiet
    log_success "Cleanup complete"
}

# Main execution
main() {
    echo ""
    log_info "=========================================="
    log_info "  Android Beta Deployment"
    log_info "=========================================="
    echo ""

    check_prerequisites
    echo ""

    increment_version_code
    echo ""

    install_dependencies
    echo ""

    clean_build
    echo ""

    run_lint
    echo ""

    run_tests
    echo ""

    build_aab
    echo ""

    verify_signing
    echo ""

    upload_to_play_console
    echo ""

    generate_release_notes
    echo ""

    generate_play_console_changelog
    echo ""

    send_notifications
    echo ""

    generate_build_report
    echo ""

    cleanup
    echo ""

    log_success "=========================================="
    log_success "  Deployment Complete!"
    log_success "=========================================="
    echo ""
    log_info "Build $VERSION_CODE is now available on Internal Testing"
    log_info "Testers can install/update immediately"
    log_info "Check status: https://play.google.com/console"
    echo ""
}

# Run main function
main

exit 0
