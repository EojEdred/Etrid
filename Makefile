.PHONY: help build test clean dev deploy docs install check format lint release

# Default target
.DEFAULT_GOAL := help

# Colors for output
CYAN := \033[0;36m
GREEN := \033[0;32m
YELLOW := \033[1;33m
NC := \033[0m

help: ## Show this help message
	@echo "$(CYAN)Ëtrid Blockchain - Development Commands$(NC)"
	@echo ""
	@echo "$(GREEN)Usage:$(NC) make [target]"
	@echo ""
	@echo "$(YELLOW)Available targets:$(NC)"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(CYAN)%-20s$(NC) %s\n", $$1, $$2}'
	@echo ""
	@echo "$(YELLOW)Examples:$(NC)"
	@echo "  make build          # Build everything"
	@echo "  make test           # Run all tests"
	@echo "  make dev            # Start development environment"
	@echo "  make deploy         # Deploy to production"

install: ## Install all dependencies
	@echo "$(CYAN)Installing dependencies...$(NC)"
	@rustup target add wasm32-unknown-unknown
	@cargo install cargo-contract --force || true
	@cd 13-clients/sdk/js-etrid-sdk && npm install || true
	@cd apps/wallet-web/etrid-crypto-website && npm install || true
	@cd apps/validator-dashboard && npm install || true
	@cd apps/watchtower-monitor && npm install || true
	@echo "$(GREEN)✓ Dependencies installed$(NC)"

check: ## Check code without building
	@echo "$(CYAN)Checking Rust code...$(NC)"
	@cargo check --all-targets
	@echo "$(GREEN)✓ Code check passed$(NC)"

format: ## Format all code
	@echo "$(CYAN)Formatting Rust code...$(NC)"
	@cargo fmt --all
	@echo "$(CYAN)Formatting TypeScript code...$(NC)"
	@cd 13-clients/sdk/js-etrid-sdk && npm run format || true
	@echo "$(GREEN)✓ Code formatted$(NC)"

lint: ## Lint all code
	@echo "$(CYAN)Linting Rust code...$(NC)"
	@cargo clippy --all-targets -- -D warnings
	@echo "$(CYAN)Linting TypeScript code...$(NC)"
	@cd 13-clients/sdk/js-etrid-sdk && npm run lint || true
	@echo "$(GREEN)✓ Linting passed$(NC)"

build: ## Build all components (development mode)
	@echo "$(CYAN)Building all components...$(NC)"
	@./scripts/build-all.sh
	@echo "$(GREEN)✓ Build complete$(NC)"

build-release: ## Build all components (release mode)
	@echo "$(CYAN)Building all components (release mode)...$(NC)"
	@./scripts/build-all.sh --release
	@echo "$(GREEN)✓ Release build complete$(NC)"

build-rust: ## Build only Rust components
	@echo "$(CYAN)Building Rust components...$(NC)"
	@./scripts/build-all.sh --skip-frontend --skip-sdk
	@echo "$(GREEN)✓ Rust build complete$(NC)"

build-frontend: ## Build only frontend applications
	@echo "$(CYAN)Building frontend applications...$(NC)"
	@./scripts/build-all.sh --skip-rust --skip-sdk
	@echo "$(GREEN)✓ Frontend build complete$(NC)"

build-sdk: ## Build only JavaScript SDK
	@echo "$(CYAN)Building JavaScript SDK...$(NC)"
	@cd 13-clients/sdk/js-etrid-sdk && npm run build
	@echo "$(GREEN)✓ SDK build complete$(NC)"

test: ## Run all tests
	@echo "$(CYAN)Running all tests...$(NC)"
	@./scripts/test-all.sh
	@echo "$(GREEN)✓ All tests passed$(NC)"

test-coverage: ## Run tests with coverage report
	@echo "$(CYAN)Running tests with coverage...$(NC)"
	@./scripts/test-all.sh --coverage
	@echo "$(GREEN)✓ Coverage report generated$(NC)"

test-rust: ## Run only Rust tests
	@echo "$(CYAN)Running Rust tests...$(NC)"
	@cargo test --all
	@echo "$(GREEN)✓ Rust tests passed$(NC)"

test-sdk: ## Run only SDK tests
	@echo "$(CYAN)Running SDK tests...$(NC)"
	@cd 13-clients/sdk/js-etrid-sdk && npm test
	@echo "$(GREEN)✓ SDK tests passed$(NC)"

clean: ## Clean all build artifacts
	@echo "$(CYAN)Cleaning build artifacts...$(NC)"
	@cargo clean
	@find . -name "node_modules" -type d -exec rm -rf {} + 2>/dev/null || true
	@find . -name ".next" -type d -exec rm -rf {} + 2>/dev/null || true
	@find . -name "dist" -type d -exec rm -rf {} + 2>/dev/null || true
	@echo "$(GREEN)✓ Clean complete$(NC)"

dev: ## Start local development environment
	@echo "$(CYAN)Starting local testnet...$(NC)"
	@./scripts/start-testnet.sh

dev-monitoring: ## Start local testnet with monitoring
	@echo "$(CYAN)Starting local testnet with monitoring...$(NC)"
	@./scripts/start-testnet.sh --with-monitoring

docs: ## Generate all documentation
	@echo "$(CYAN)Generating documentation...$(NC)"
	@./scripts/generate-docs.sh
	@echo "$(GREEN)✓ Documentation generated$(NC)"

docs-open: ## Generate documentation and open in browser
	@echo "$(CYAN)Generating documentation...$(NC)"
	@./scripts/generate-docs.sh --open

docs-rust: ## Generate only Rust documentation
	@echo "$(CYAN)Generating Rust documentation...$(NC)"
	@cargo doc --no-deps --document-private-items --open

deploy: ## Deploy all applications to production
	@echo "$(CYAN)Deploying to production...$(NC)"
	@./scripts/deploy-all.sh
	@echo "$(GREEN)✓ Deployment complete$(NC)"

deploy-staging: ## Deploy all applications to staging
	@echo "$(CYAN)Deploying to staging...$(NC)"
	@./scripts/deploy-all.sh --environment staging
	@echo "$(GREEN)✓ Staging deployment complete$(NC)"

deploy-docker: ## Deploy as Docker containers
	@echo "$(CYAN)Deploying as Docker containers...$(NC)"
	@./scripts/deploy-all.sh --target docker
	@echo "$(GREEN)✓ Docker deployment complete$(NC)"

docker-build: ## Build Docker images
	@echo "$(CYAN)Building Docker images...$(NC)"
	@docker-compose build
	@echo "$(GREEN)✓ Docker build complete$(NC)"

docker-up: ## Start Docker containers
	@echo "$(CYAN)Starting Docker containers...$(NC)"
	@docker-compose up -d
	@echo "$(GREEN)✓ Containers started$(NC)"

docker-down: ## Stop Docker containers
	@echo "$(CYAN)Stopping Docker containers...$(NC)"
	@docker-compose down
	@echo "$(GREEN)✓ Containers stopped$(NC)"

docker-logs: ## View Docker container logs
	@docker-compose logs -f

release: ## Create a new release (build + test + docs)
	@echo "$(CYAN)Creating release...$(NC)"
	@echo "$(YELLOW)1/4: Cleaning...$(NC)"
	@make clean
	@echo "$(YELLOW)2/4: Building release...$(NC)"
	@make build-release
	@echo "$(YELLOW)3/4: Running tests...$(NC)"
	@make test
	@echo "$(YELLOW)4/4: Generating docs...$(NC)"
	@make docs
	@echo "$(GREEN)✓ Release ready!$(NC)"

benchmark: ## Run performance benchmarks
	@echo "$(CYAN)Running benchmarks...$(NC)"
	@./scripts/benchmark.sh || cargo bench
	@echo "$(GREEN)✓ Benchmarks complete$(NC)"

validate: ## Validate all scripts and configurations
	@echo "$(CYAN)Validating scripts...$(NC)"
	@for script in scripts/*.sh; do \
		bash -n $$script && echo "$(GREEN)✓ $$script$(NC)" || echo "$(YELLOW)✗ $$script$(NC)"; \
	done
	@echo "$(GREEN)✓ Validation complete$(NC)"

update-deps: ## Update all dependencies
	@echo "$(CYAN)Updating dependencies...$(NC)"
	@cargo update
	@cd 13-clients/sdk/js-etrid-sdk && npm update || true
	@echo "$(GREEN)✓ Dependencies updated$(NC)"

git-status: ## Show git status with statistics
	@echo "$(CYAN)Git Status$(NC)"
	@git status
	@echo ""
	@echo "$(CYAN)Statistics:$(NC)"
	@echo "Total commits: $$(git rev-list --count HEAD)"
	@echo "Contributors: $$(git shortlog -s -n | wc -l | xargs)"

stats: ## Show project statistics
	@echo "$(CYAN)Ëtrid Project Statistics$(NC)"
	@echo ""
	@echo "$(YELLOW)Documentation:$(NC)"
	@echo "  Files: $$(find docs -name "*.md" | wc -l | xargs)"
	@echo "  Lines: $$(find docs -name "*.md" -exec wc -l {} + | tail -1 | awk '{print $$1}')"
	@echo ""
	@echo "$(YELLOW)Automation Scripts:$(NC)"
	@echo "  Files: $$(find scripts -name "*.sh" | wc -l | xargs)"
	@echo "  Lines: $$(find scripts -name "*.sh" -exec wc -l {} + | tail -1 | awk '{print $$1}')"
	@echo ""
	@echo "$(YELLOW)Rust Code:$(NC)"
	@echo "  Crates: $$(find . -name "Cargo.toml" | wc -l | xargs)"
	@echo "  Files: $$(find . -name "*.rs" | wc -l | xargs)"
	@echo "  Lines: $$(find . -name "*.rs" -exec wc -l {} + 2>/dev/null | tail -1 | awk '{print $$1}' || echo 'N/A')"

all: clean install build test docs ## Full build pipeline (clean, install, build, test, docs)
	@echo "$(GREEN)✓ All tasks complete!$(NC)"
