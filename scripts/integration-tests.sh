#!/usr/bin/env bash
# Integration test suite
set -e

echo "Running integration tests..."

# Test 1: Build pipeline
echo "Test 1: Build pipeline"
./scripts/build-all.sh --skip-frontend || exit 1

# Test 2: All tests pass
echo "Test 2: Test suite"
./scripts/test-all.sh --skip-frontend || exit 1

# Test 3: Documentation generation
echo "Test 3: Documentation generation"
./scripts/generate-docs.sh --rust-only || exit 1

# Test 4: Scripts are executable
echo "Test 4: Script permissions"
for script in scripts/*.sh; do
    [[ -x "$script" ]] || { echo "✗ $script not executable"; exit 1; }
done

echo "✓ All integration tests passed!"
