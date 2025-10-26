#!/usr/bin/env bash
# Create release package
set -e

VERSION=${1:-"1.0.0"}
RELEASE_DIR="release-v${VERSION}"

echo "Creating release package v${VERSION}..."

# Create release directory
mkdir -p "${RELEASE_DIR}"

# Copy essential files
cp -r docs "${RELEASE_DIR}/"
cp -r scripts "${RELEASE_DIR}/"
cp README.md CHANGELOG.md LICENSE "${RELEASE_DIR}/"

# Create tarball
tar -czf "etrid-v${VERSION}.tar.gz" "${RELEASE_DIR}"

echo "✓ Release package created: etrid-v${VERSION}.tar.gz"
