# Configuration Files

Configuration files for ËTRID development and deployment.

## Files

### Build Configuration
- **`Cross.toml`** - Cross-compilation settings for Rust

### Environment Configuration
- **`.env.example`** - Example environment variables template
  - Copy to `.env` and fill in your values
  - Used by various scripts and services

### Infrastructure Configuration
- **`validator-ips.json`** - Validator network configuration
  - Contains IP addresses, SSH users, and metadata for all 21 validators
  - Used by deployment scripts and AI monitoring system

## Usage

### Environment Variables

```bash
# Copy example and customize
cp config/.env.example .env
# Edit .env with your API keys and settings
```

### Cross-Compilation

```bash
# Cross.toml is automatically used by cross command
cross build --release --target x86_64-unknown-linux-gnu
```

### Validator Configuration

The `validator-ips.json` file is used by:
- Deployment scripts in `scripts/deployment/`
- AI monitoring orchestrator
- Network testing tools

**Format:**
```json
{
  "validators": [
    {
      "number": 1,
      "name": "Gizzi",
      "ip": "20.186.91.207",
      "ssh_user": "etrid-validator-01",
      "region": "Azure West US",
      "role": "Director/Bootstrap"
    }
  ]
}
```

## Security Note

⚠️ **Never commit `.env` files with real credentials!**

All `.env` files (except `.env.example`) are excluded via `.gitignore`.

## Related Documentation

- [docs/deployment/](../docs/deployment/) - Deployment guides
- [README.md](../README.md) - Main project documentation

---

*Last Updated: 2025-11-01*
