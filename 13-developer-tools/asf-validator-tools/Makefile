# ËTRID ASF Validator Tools - Makefile
# ═══════════════════════════════════════════════════════════════════════════════

.PHONY: all build install clean test help

# Default target
all: build

# ═══════════════════════════════════════════════════════════════════════════════
# BUILD TARGETS
# ═══════════════════════════════════════════════════════════════════════════════

build: build-keygen build-monitor build-health build-stake
	@echo "✓ All validator tools built successfully!"

build-keygen:
	@echo "Building asf-keygen..."
	@cd asf-keygen && cargo build --release

build-monitor:
	@echo "Building asf-monitor..."
	@cd asf-monitor && cargo build --release

build-health:
	@echo "Building asf-health..."
	@cd asf-health && cargo build --release

build-stake:
	@echo "Building asf-stake..."
	@cd asf-stake && cargo build --release

# ═══════════════════════════════════════════════════════════════════════════════
# INSTALLATION
# ═══════════════════════════════════════════════════════════════════════════════

install: build
	@echo "Installing validator tools to /usr/local/bin..."
	@cp asf-keygen/target/release/asf-keygen /usr/local/bin/
	@cp asf-monitor/target/release/asf-monitor /usr/local/bin/
	@cp asf-health/target/release/asf-health /usr/local/bin/
	@cp asf-stake/target/release/asf-stake /usr/local/bin/
	@echo "✓ Installation complete!"

install-local: build
	@echo "Installing validator tools to ~/.local/bin..."
	@mkdir -p ~/.local/bin
	@cp asf-keygen/target/release/asf-keygen ~/.local/bin/
	@cp asf-monitor/target/release/asf-monitor ~/.local/bin/
	@cp asf-health/target/release/asf-health ~/.local/bin/
	@cp asf-stake/target/release/asf-stake ~/.local/bin/
	@echo "✓ Local installation complete!"
	@echo "Make sure ~/.local/bin is in your PATH"

# ═══════════════════════════════════════════════════════════════════════════════
# TESTING
# ═══════════════════════════════════════════════════════════════════════════════

test: test-keygen test-monitor test-health test-stake
	@echo "✓ All tests passed!"

test-keygen:
	@echo "Testing asf-keygen..."
	@cd asf-keygen && cargo test

test-monitor:
	@echo "Testing asf-monitor..."
	@cd asf-monitor && cargo test

test-health:
	@echo "Testing asf-health..."
	@cd asf-health && cargo test

test-stake:
	@echo "Testing asf-stake..."
	@cd asf-stake && cargo test

# ═══════════════════════════════════════════════════════════════════════════════
# DEVELOPMENT
# ═══════════════════════════════════════════════════════════════════════════════

dev-keygen:
	@cd asf-keygen && cargo run -- --help

dev-monitor:
	@cd asf-monitor && cargo run -- --help

dev-health:
	@cd asf-health && cargo run -- --help

dev-stake:
	@cd asf-stake && cargo run -- --help

check:
	@echo "Checking all projects..."
	@cd asf-keygen && cargo check
	@cd asf-monitor && cargo check
	@cd asf-health && cargo check
	@cd asf-stake && cargo check
	@echo "✓ All checks passed!"

clippy:
	@echo "Running clippy on all projects..."
	@cd asf-keygen && cargo clippy -- -D warnings
	@cd asf-monitor && cargo clippy -- -D warnings
	@cd asf-health && cargo clippy -- -D warnings
	@cd asf-stake && cargo clippy -- -D warnings

fmt:
	@echo "Formatting all projects..."
	@cd asf-keygen && cargo fmt
	@cd asf-monitor && cargo fmt
	@cd asf-health && cargo fmt
	@cd asf-stake && cargo fmt
	@echo "✓ Formatting complete!"

# ═══════════════════════════════════════════════════════════════════════════════
# CLEANUP
# ═══════════════════════════════════════════════════════════════════════════════

clean:
	@echo "Cleaning all build artifacts..."
	@cd asf-keygen && cargo clean
	@cd asf-monitor && cargo clean
	@cd asf-health && cargo clean
	@cd asf-stake && cargo clean
	@echo "✓ Cleanup complete!"

uninstall:
	@echo "Uninstalling validator tools..."
	@rm -f /usr/local/bin/asf-keygen
	@rm -f /usr/local/bin/asf-monitor
	@rm -f /usr/local/bin/asf-health
	@rm -f /usr/local/bin/asf-stake
	@echo "✓ Uninstallation complete!"

uninstall-local:
	@echo "Uninstalling validator tools from ~/.local/bin..."
	@rm -f ~/.local/bin/asf-keygen
	@rm -f ~/.local/bin/asf-monitor
	@rm -f ~/.local/bin/asf-health
	@rm -f ~/.local/bin/asf-stake
	@echo "✓ Local uninstallation complete!"

# ═══════════════════════════════════════════════════════════════════════════════
# DOCUMENTATION
# ═══════════════════════════════════════════════════════════════════════════════

docs:
	@echo "Generating documentation..."
	@cd asf-keygen && cargo doc --no-deps
	@cd asf-monitor && cargo doc --no-deps
	@cd asf-health && cargo doc --no-deps
	@cd asf-stake && cargo doc --no-deps
	@echo "✓ Documentation generated!"
	@echo "Open target/doc/index.html to view"

# ═══════════════════════════════════════════════════════════════════════════════
# RELEASE
# ═══════════════════════════════════════════════════════════════════════════════

release: test
	@echo "Building release binaries..."
	@cd asf-keygen && cargo build --release
	@cd asf-monitor && cargo build --release
	@cd asf-health && cargo build --release
	@cd asf-stake && cargo build --release
	@echo "✓ Release binaries built!"
	@echo "Binaries are in */target/release/"

package: release
	@echo "Creating release package..."
	@mkdir -p dist
	@cp asf-keygen/target/release/asf-keygen dist/
	@cp asf-monitor/target/release/asf-monitor dist/
	@cp asf-health/target/release/asf-health dist/
	@cp asf-stake/target/release/asf-stake dist/
	@cp README.md dist/
	@cp VALIDATOR_GUIDE.md dist/
	@tar -czf asf-validator-tools.tar.gz dist/
	@echo "✓ Release package created: asf-validator-tools.tar.gz"

# ═══════════════════════════════════════════════════════════════════════════════
# HELP
# ═══════════════════════════════════════════════════════════════════════════════

help:
	@echo "ËTRID ASF Validator Tools - Build System"
	@echo "════════════════════════════════════════════════════════════════════"
	@echo ""
	@echo "Build Targets:"
	@echo "  make build          - Build all tools in release mode"
	@echo "  make build-keygen   - Build only asf-keygen"
	@echo "  make build-monitor  - Build only asf-monitor"
	@echo "  make build-health   - Build only asf-health"
	@echo "  make build-stake    - Build only asf-stake"
	@echo ""
	@echo "Installation:"
	@echo "  make install        - Install to /usr/local/bin (requires sudo)"
	@echo "  make install-local  - Install to ~/.local/bin"
	@echo "  make uninstall      - Remove from /usr/local/bin"
	@echo "  make uninstall-local- Remove from ~/.local/bin"
	@echo ""
	@echo "Testing:"
	@echo "  make test           - Run all tests"
	@echo "  make check          - Check all projects compile"
	@echo "  make clippy         - Run clippy linter"
	@echo "  make fmt            - Format all code"
	@echo ""
	@echo "Development:"
	@echo "  make dev-keygen     - Run asf-keygen in dev mode"
	@echo "  make dev-monitor    - Run asf-monitor in dev mode"
	@echo "  make dev-health     - Run asf-health in dev mode"
	@echo "  make dev-stake      - Run asf-stake in dev mode"
	@echo ""
	@echo "Cleanup:"
	@echo "  make clean          - Remove build artifacts"
	@echo ""
	@echo "Documentation:"
	@echo "  make docs           - Generate API documentation"
	@echo ""
	@echo "Release:"
	@echo "  make release        - Build optimized release binaries"
	@echo "  make package        - Create release package"
	@echo ""
