# CI/CD Pipeline Guide

**Date:** October 25, 2025
**Platform:** GitHub Actions
**Purpose:** Automated testing, building, and deployment for Ëtrid

---

## Overview

This guide documents the complete CI/CD pipeline for Ëtrid, including:
- Continuous Integration (CI): Automated testing on every commit
- Continuous Deployment (CD): Automated deployment to testnet/mainnet
- Pre-deployment validation
- Monitoring and alerts

---

## Table of Contents

1. [Pipeline Architecture](#pipeline-architecture)
2. [GitHub Actions Workflows](#github-actions-workflows)
3. [Deployment Workflows](#deployment-workflows)
4. [Local Development Workflow](#local-development-workflow)
5. [Security & Secrets Management](#security--secrets-management)
6. [Troubleshooting](#troubleshooting)

---

## Pipeline Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Developer Workflow                       │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                      Git Push / PR                           │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                   CI Pipeline (GitHub Actions)               │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐            │
│  │ Format     │  │ Clippy     │  │ Security   │            │
│  │ Check      │  │ Lints      │  │ Audit      │            │
│  └────────────┘  └────────────┘  └────────────┘            │
│                                                               │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐            │
│  │ Build      │  │ Unit       │  │ Integration│            │
│  │ Runtime    │  │ Tests      │  │ Tests      │            │
│  └────────────┘  └────────────┘  └────────────┘            │
│                                                               │
│  ┌────────────┐  ┌────────────┐                            │
│  │ Genesis    │  │ Docker     │                            │
│  │ Validation │  │ Build      │                            │
│  └────────────┘  └────────────┘                            │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                   Deployment Pipeline                        │
│                                                               │
│  Manual Approval (for mainnet)                              │
│            ▼                                                  │
│  ┌─────────────────┐         ┌─────────────────┐           │
│  │   Testnet       │         │    Mainnet      │           │
│  │   Deployment    │────────▶│   Deployment    │           │
│  └─────────────────┘         └─────────────────┘           │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│               Post-Deployment Monitoring                     │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐            │
│  │ Prometheus │  │ Grafana    │  │ Alerting   │            │
│  └────────────┘  └────────────┘  └────────────┘            │
└─────────────────────────────────────────────────────────────┘
```

---

## GitHub Actions Workflows

### 1. Runtime Build & Test (`runtime-build-test.yml`)

**Triggers:**
- Push to `main` or `develop` branches
- Pull requests to `main` or `develop`
- Changes to runtime or pallet code

**Jobs:**

1. **check-formatting**
   - Runs `cargo fmt --check`
   - Ensures consistent code style
   - Fast fail if formatting is incorrect

2. **clippy**
   - Runs `cargo clippy` with all warnings as errors
   - Catches common mistakes and anti-patterns
   - Enforces best practices

3. **build-runtime**
   - Builds release binary
   - Builds WASM runtime
   - Uploads artifacts for downstream jobs
   - **Duration:** ~10-15 minutes

4. **unit-tests**
   - Runs all `cargo test` in workspace
   - Generates test coverage report
   - Uploads test results
   - **Duration:** ~5-10 minutes

5. **integration-tests**
   - Starts local node
   - Runs integration tests
   - Verifies RPC endpoints
   - **Duration:** ~3-5 minutes

6. **security-audit**
   - Runs `cargo audit`
   - Checks for known vulnerabilities in dependencies
   - **Duration:** ~1-2 minutes

7. **check-genesis**
   - Validates JSON syntax
   - Checks for placeholder addresses (mainnet only)
   - Verifies treasury allocation
   - **Duration:** <1 minute

8. **docker-build** (PR only)
   - Tests Docker image build
   - Verifies container runs correctly
   - **Duration:** ~5-10 minutes

**Caching Strategy:**
- Cargo registry cached by `Cargo.lock` hash
- Build artifacts cached for reuse between jobs
- Significantly reduces build time on subsequent runs

**Example Run:**

```bash
# Manually trigger locally (for testing)
act -j build-runtime  # Using 'act' to test GitHub Actions locally
```

---

## Deployment Workflows

### 2. Deploy to Testnet (`deploy-testnet.yml`)

**Triggers:**
- Manual dispatch (workflow_dispatch)
- Push to `develop` branch (optional)

**Jobs:**

1. **pre-deployment-checks**
   - Runs pre-deployment test suite
   - Validates configuration
   - Ensures all tests pass

2. **build-and-push-docker**
   - Builds Docker image
   - Tags with commit SHA and `latest`
   - Pushes to container registry (GitHub Container Registry or Docker Hub)

3. **deploy-to-testnet**
   - SSH into testnet server
   - Pulls latest Docker image
   - Runs database migrations (if needed)
   - Restarts node with new runtime
   - Verifies deployment

4. **smoke-tests**
   - Checks RPC endpoints
   - Verifies block production
   - Tests treasury proposals
   - Validates bridge functionality

5. **notify-deployment**
   - Posts to Discord/Slack
   - Sends email notification
   - Updates deployment dashboard

**Usage:**

```bash
# Manually trigger via GitHub UI:
# Actions → Deploy to Testnet → Run workflow

# Or via GitHub CLI:
gh workflow run deploy-testnet.yml
```

---

### 3. Deploy to Mainnet (`deploy-mainnet.yml`)

**Triggers:**
- Manual dispatch only (requires approval)

**Additional Safety Checks:**
- Requires manual approval from 2+ maintainers
- Runs full pre-deployment test suite
- Validates mainnet genesis has no placeholders
- Creates backup before deployment
- Gradual rollout (if multiple validators)

**Jobs:**

1. **request-approval**
   - Creates GitHub issue for approval
   - Requires 2+ maintainer approvals
   - Includes deployment checklist

2. **create-backup**
   - Backs up current state
   - Uploads to S3
   - Verifies backup integrity

3. **pre-mainnet-checks**
   - Runs comprehensive test suite
   - Validates mainnet configuration
   - Checks validator keys

4. **deploy-mainnet**
   - Blue-green deployment strategy
   - Deploys to canary node first
   - Monitors for 10 minutes
   - Rolls out to remaining validators

5. **post-deployment-validation**
   - Verifies all nodes online
   - Checks block finality
   - Validates RPC endpoints
   - Monitors for errors

6. **notify-mainnet-deployment**
   - Posts announcement
   - Updates status page
   - Sends notifications to community

**Usage:**

```bash
# Via GitHub CLI (requires maintainer permissions):
gh workflow run deploy-mainnet.yml
```

---

### 4. BSC Contract Deployment (`bsc-ci.yml`)

**Triggers:**
- Changes to `contracts/ethereum/**`
- Manual dispatch

**Jobs:**

1. **test-contracts**
   - Runs Hardhat tests
   - Checks gas usage
   - Validates contract code

2. **deploy-to-bsc-testnet**
   - Deploys to BSC testnet
   - Verifies deployment
   - Creates liquidity pool

3. **verify-contracts**
   - Verifies on BscScan
   - Publishes source code
   - Updates documentation

**Environment Variables Required:**

```bash
BSC_PRIVATE_KEY=your_deployer_private_key
BSC_RPC_URL=https://bsc-testnet.publicnode.com
BSCSCAN_API_KEY=your_bscscan_api_key
```

---

## Local Development Workflow

### Pre-commit Checks

Create `.git/hooks/pre-commit`:

```bash
#!/bin/bash
# Pre-commit hook for Ëtrid

set -e

echo "Running pre-commit checks..."

# Format check
echo "Checking formatting..."
cargo fmt --all -- --check

# Clippy
echo "Running clippy..."
cargo clippy --workspace --all-targets -- -D warnings

# Quick tests
echo "Running quick tests..."
cargo test --workspace --lib

echo "✅ Pre-commit checks passed!"
```

Make executable:
```bash
chmod +x .git/hooks/pre-commit
```

### Local CI Simulation

Test GitHub Actions locally using [act](https://github.com/nektos/act):

```bash
# Install act
brew install act  # macOS
# or
curl https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash  # Linux

# Run all jobs
act

# Run specific job
act -j build-runtime

# Run with secrets
act --secret-file .secrets
```

### Pre-deployment Test Script

Before deploying, always run:

```bash
./scripts/pre-deployment-tests.sh --testnet
```

For mainnet:

```bash
./scripts/pre-deployment-tests.sh --mainnet
```

---

## Security & Secrets Management

### GitHub Secrets

Required secrets (configure in Settings → Secrets):

**For Runtime Deployment:**
```
SSH_PRIVATE_KEY          # SSH key for server access
VALIDATOR_HOST           # Server hostname
VALIDATOR_USER           # SSH username
```

**For BSC Deployment:**
```
BSC_PRIVATE_KEY          # Deployer wallet private key
BSC_RPC_URL              # BSC RPC endpoint
BSCSCAN_API_KEY          # For contract verification
```

**For Notifications:**
```
DISCORD_WEBHOOK_URL      # Discord notifications
SLACK_WEBHOOK_URL        # Slack notifications
SENDGRID_API_KEY         # Email notifications
```

**For Docker Registry:**
```
DOCKER_USERNAME          # Docker Hub username
DOCKER_PASSWORD          # Docker Hub password (or token)
# Or for GHCR:
GITHUB_TOKEN             # Automatically provided by GitHub
```

### Secrets Best Practices

1. **Never commit secrets to Git**
   - Use `.env` files locally (add to `.gitignore`)
   - Use GitHub Secrets for CI/CD
   - Rotate secrets regularly

2. **Principle of least privilege**
   - Testnet keys should differ from mainnet
   - Use separate keys for each environment
   - Limit key permissions

3. **Secret rotation**
   - Rotate BSC deployer key after mainnet deployment
   - Rotate SSH keys quarterly
   - Rotate API keys annually

4. **Audit trail**
   - Log all secret usage
   - Monitor for unauthorized access
   - Alert on suspicious activity

---

## Caching Strategy

### Cargo Caching

```yaml
- name: Cache cargo registry
  uses: actions/cache@v3
  with:
    path: ~/.cargo/registry
    key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}

- name: Cache cargo index
  uses: actions/cache@v3
  with:
    path: ~/.cargo/git
    key: ${{ runner.os }}-cargo-git-${{ hashFiles('**/Cargo.lock') }}

- name: Cache cargo build
  uses: actions/cache@v3
  with:
    path: target
    key: ${{ runner.os }}-cargo-build-${{ hashFiles('**/Cargo.lock') }}
```

**Benefits:**
- Reduces build time from 15 minutes to 5 minutes
- Saves GitHub Actions minutes
- Faster feedback for developers

**Cache Invalidation:**
- Automatic when `Cargo.lock` changes
- Manual: Delete cache in Actions settings

---

## Monitoring CI/CD Pipeline

### Metrics to Track

1. **Build Success Rate**
   - Target: >95% on main branch
   - Alert if <90%

2. **Build Duration**
   - Target: <15 minutes
   - Alert if >20 minutes

3. **Test Pass Rate**
   - Target: 100%
   - Alert on any failures

4. **Deployment Frequency**
   - Testnet: Daily
   - Mainnet: Weekly (after stable period)

5. **Time to Recovery**
   - Target: <1 hour for rollback
   - Alert if >2 hours

### Grafana Dashboard

Create dashboard with:
- Build success/failure trends
- Build duration over time
- Test coverage percentage
- Deployment frequency
- Failed build reasons

---

## Troubleshooting

### Common Issues

#### 1. Build Timeout (>1 hour)

**Cause:** Cargo cache not working

**Solution:**
```yaml
# Add to workflow
env:
  CARGO_INCREMENTAL: 0  # Disable incremental compilation in CI
```

#### 2. Flaky Tests

**Cause:** Race conditions or timing issues

**Solution:**
```rust
// Add retries to flaky tests
#[tokio::test(flavor = "multi_thread")]
async fn flaky_test() {
    for attempt in 0..3 {
        match run_test().await {
            Ok(_) => return,
            Err(e) if attempt < 2 => continue,
            Err(e) => panic!("Test failed: {}", e),
        }
    }
}
```

#### 3. Out of Disk Space

**Cause:** Build artifacts filling disk

**Solution:**
```yaml
# Add cleanup step
- name: Clean up
  if: always()
  run: |
    cargo clean
    docker system prune -af
```

#### 4. SSH Connection Failed (Deployment)

**Cause:** SSH key not configured correctly

**Solution:**
```bash
# Test SSH connection locally
ssh -i ~/.ssh/etrid_deploy_key validator@your-server.com

# In GitHub Actions, check secret is set correctly
echo "${{ secrets.SSH_PRIVATE_KEY }}" | wc -c  # Should be >0
```

#### 5. Docker Build Failed

**Cause:** Missing dependencies or incorrect Dockerfile

**Solution:**
```dockerfile
# Ensure all dependencies installed
FROM rust:1.70 as builder

RUN apt-get update && apt-get install -y \
    clang \
    libclang-dev \
    cmake \
    build-essential

# Test locally first
docker build -t etrid:test .
docker run --rm etrid:test --version
```

---

## Best Practices

### 1. Branch Strategy

```
main          ← Production (mainnet)
  ↑
develop       ← Staging (testnet)
  ↑
feature/      ← Feature branches
bugfix/       ← Bug fixes
hotfix/       ← Emergency fixes (directly to main)
```

### 2. Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add pallet-treasury to runtime
fix: resolve vesting schedule calculation bug
docs: update deployment guide
chore: upgrade substrate to polkadot-stable2509
test: add integration tests for treasury
```

### 3. Pull Request Checklist

Before merging:
- [ ] All CI checks pass
- [ ] Code reviewed by 2+ developers
- [ ] Tests added for new features
- [ ] Documentation updated
- [ ] CHANGELOG updated (if applicable)
- [ ] No merge conflicts

### 4. Deployment Checklist

Before mainnet deployment:
- [ ] Deployed to testnet successfully
- [ ] Tested on testnet for 24+ hours
- [ ] No critical bugs reported
- [ ] Security audit passed
- [ ] Community notified
- [ ] Rollback plan prepared
- [ ] Backup created

---

## Continuous Improvement

### Metrics Review (Monthly)

1. Review build times (optimize if >15 min)
2. Review test coverage (target 80%+)
3. Review deployment frequency
4. Review incident response times
5. Update pipeline based on learnings

### Pipeline Evolution

**Current State (v1.0):**
- Basic CI with formatting, lints, build, tests
- Manual deployment to testnet/mainnet
- Basic notifications

**Future Enhancements (v2.0):**
- Automated testnet deployment on merge to develop
- Canary deployments for mainnet
- Automatic rollback on errors
- Advanced monitoring with APM
- Performance regression detection

---

## Summary

**CI/CD Pipeline Components:**
- ✅ GitHub Actions workflows (3 workflows)
- ✅ Pre-deployment test suite
- ✅ Docker build automation
- ✅ Genesis validation
- ✅ Security audits
- ✅ Testnet deployment workflow
- ✅ Mainnet deployment workflow (with approvals)

**Estimated Setup Time:** 2-3 hours
**Maintenance:** 1-2 hours/week
**Cost:** $0-50/month (GitHub Actions free tier + small overages)

**Pipeline Performance:**
- CI build time: ~15 minutes
- Full test suite: ~20 minutes
- Deployment time: ~5 minutes
- Total PR feedback: ~25 minutes

**Status:** ✅ CI/CD pipeline configuration complete and production-ready
