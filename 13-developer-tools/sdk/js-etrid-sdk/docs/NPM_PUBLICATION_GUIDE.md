# NPM Publication Guide

## Prerequisites

1. **NPM Account**: Create at [npmjs.com](https://www.npmjs.com/signup)
2. **Access Rights**: Request @etrid scope access from maintainers
3. **2FA Enabled**: Required for scoped packages

## Pre-Publication Checklist

### 1. Version Check

```bash
# Update version in package.json
npm version patch  # 0.1.0 → 0.1.1
npm version minor  # 0.1.1 → 0.2.0
npm version major  # 0.2.0 → 1.0.0
```

### 2. Build & Test

```bash
# Clean build
rm -rf dist node_modules
npm install

# Run full test suite
npm test

# Check test coverage
npm run test:coverage

# Lint code
npm run lint

# Generate documentation
npm run docs
```

### 3. Build Package

```bash
# Compile TypeScript
npm run build

# Verify dist/ folder
ls -la dist/
```

### 4. Test Package Locally

```bash
# Create tarball
npm pack

# Install in test project
cd ../test-project
npm install ../js-etrid-sdk/etrid-sdk-0.1.0.tgz

# Test imports
node -e "const sdk = require('@etrid/sdk'); console.log(sdk)"
```

## Publication Steps

### 1. Login to NPM

```bash
npm login
# Enter credentials + 2FA code
```

### 2. Dry Run

```bash
# Test publication without publishing
npm publish --dry-run

# Check what will be published
npm pack --dry-run
```

### 3. Publish

#### Alpha Release (First Time)

```bash
# Publish as alpha
npm publish --tag alpha --access public

# Users install with:
# npm install @etrid/sdk@alpha
```

#### Beta Release

```bash
npm publish --tag beta --access public

# Users install with:
# npm install @etrid/sdk@beta
```

#### Production Release

```bash
# Final release
npm publish --access public

# Users install with:
# npm install @etrid/sdk
```

### 4. Verify Publication

```bash
# Check on NPM
npm view @etrid/sdk

# Install in fresh project
mkdir test-install
cd test-install
npm init -y
npm install @etrid/sdk

# Test
node -e "console.log(require('@etrid/sdk'))"
```

## Post-Publication

### 1. Create Git Tag

```bash
git tag v0.1.0
git push origin v0.1.0
```

### 2. GitHub Release

1. Go to https://github.com/etrid/etrid-protocol/releases
2. Click "Draft a new release"
3. Select tag v0.1.0
4. Title: "Ëtrid SDK v0.1.0"
5. Add release notes
6. Publish release

### 3. Update Documentation

```bash
# Update docs site
npm run docs
git add docs/
git commit -m "Update docs for v0.1.0"
git push
```

### 4. Announce Release

- Post on Discord
- Tweet from @EtridProtocol
- Update website
- Email subscribers

## Package.json Configuration

Ensure your package.json has:

```json
{
  "name": "@etrid/sdk",
  "version": "0.1.0",
  "description": "JavaScript/TypeScript SDK for Ëtrid Protocol",
  "main": "dist/index.js",
  "types": "dist/index.d.ts",
  "files": [
    "dist",
    "README.md",
    "LICENSE"
  ],
  "publishConfig": {
    "access": "public"
  },
  "repository": {
    "type": "git",
    "url": "https://github.com/etrid/etrid-protocol"
  },
  "keywords": [
    "etrid",
    "blockchain",
    "substrate",
    "sdk",
    "web3",
    "crypto"
  ],
  "author": "Ëtrid Foundation",
  "license": "Apache-2.0"
}
```

## .npmignore Configuration

Create `.npmignore`:

```
# Source files
src/
tests/
examples/

# Config files
tsconfig.json
jest.config.js
.eslintrc.js
typedoc.json

# Development
node_modules/
.git/
.github/
.vscode/

# Documentation source
docs/tutorials/

# Only include built files
!dist/
```

## CI/CD Automation

### GitHub Actions Workflow

`.github/workflows/publish.yml`:

```yaml
name: Publish to NPM

on:
  release:
    types: [created]

jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - uses: actions/setup-node@v3
        with:
          node-version: '18'
          registry-url: 'https://registry.npmjs.org'
      
      - run: npm ci
      - run: npm test
      - run: npm run build
      - run: npm run docs
      
      - run: npm publish --access public
        env:
          NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}
```

## Troubleshooting

### "403 Forbidden"

```bash
# Check login status
npm whoami

# Re-login
npm logout
npm login
```

### "Package name taken"

```bash
# Use scoped name
# Change "etrid-sdk" to "@etrid/sdk"
```

### "Need 2FA code"

```bash
# Enable 2FA in NPM settings
# Use app like Google Authenticator
npm publish --otp=123456
```

### "Files not included"

```bash
# Check .npmignore
# Verify with:
npm pack --dry-run
```

## Version Management

### Semantic Versioning

- **Patch** (0.1.0 → 0.1.1): Bug fixes
- **Minor** (0.1.0 → 0.2.0): New features, backwards compatible
- **Major** (0.1.0 → 1.0.0): Breaking changes

### Pre-release Versions

```bash
# Alpha
npm version prerelease --preid=alpha
# 0.1.0 → 0.1.1-alpha.0

# Beta
npm version prerelease --preid=beta
# 0.1.1-alpha.0 → 0.1.1-beta.0

# Release candidate
npm version prerelease --preid=rc
# 0.1.1-beta.0 → 0.1.1-rc.0
```

## Deprecating Versions

```bash
# Deprecate old version
npm deprecate @etrid/sdk@0.0.1 "Please upgrade to 0.1.0"

# Unpublish (only within 72 hours)
npm unpublish @etrid/sdk@0.0.1
```

## Monitoring

### NPM Stats

```bash
# View download stats
npm view @etrid/sdk

# Check versions
npm view @etrid/sdk versions
```

### Analytics Tools

- [npmjs.com/package/@etrid/sdk](https://www.npmjs.com/package/@etrid/sdk)
- [npm-stat.com](https://npm-stat.com/)
- [npmtrends.com](https://npmtrends.com/)

## Best Practices

1. ✅ **Always test before publishing**
2. ✅ **Use semantic versioning**
3. ✅ **Tag releases in git**
4. ✅ **Write clear changelogs**
5. ✅ **Deprecate old versions gradually**
6. ✅ **Monitor download stats**
7. ✅ **Respond to issues promptly**
8. ✅ **Keep dependencies updated**

## Emergency Unpublish

Only if absolutely necessary within 72 hours:

```bash
# Unpublish specific version
npm unpublish @etrid/sdk@0.1.0

# Unpublish entire package (DANGEROUS!)
npm unpublish @etrid/sdk --force
```

⚠️ **Note**: Unpublishing is permanent and discouraged. Use `npm deprecate` instead.

## Release Checklist

- [ ] Version bumped in package.json
- [ ] Changelog updated
- [ ] Tests passing (npm test)
- [ ] Code linted (npm run lint)
- [ ] Documentation generated (npm run docs)
- [ ] Built successfully (npm run build)
- [ ] Local install tested
- [ ] Git tagged
- [ ] Published to NPM
- [ ] GitHub release created
- [ ] Documentation updated
- [ ] Announcement posted

---

**Ready to publish?** Run through this checklist and execute:

```bash
npm run build && npm test && npm publish --access public
```
