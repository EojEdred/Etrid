# Deployment Documentation

Complete deployment guides and documentation for ËTRID monitoring and validator infrastructure.

## Quick Start

**New to deployment?** Start here:
1. Read [START_HERE.md](./START_HERE.md) - Main entry point
2. Read [EXECUTIVE_SUMMARY.md](./EXECUTIVE_SUMMARY.md) - High-level overview
3. Follow [QUICK_DEPLOYMENT_REFERENCE.md](./QUICK_DEPLOYMENT_REFERENCE.md) - Quick commands

## Documentation Files

### Entry Points
- **[START_HERE.md](./START_HERE.md)** - Main deployment guide entry point
- **[EXECUTIVE_SUMMARY.md](./EXECUTIVE_SUMMARY.md)** - High-level deployment overview

### Complete Guides
- **[AGENT_DEPLOYMENT_GUIDE.md](./AGENT_DEPLOYMENT_GUIDE.md)** - Complete AI monitoring agent deployment guide
- **[DEPLOYMENT_PACKAGE_README.md](./DEPLOYMENT_PACKAGE_README.md)** - Deployment package overview

### Reference Documents
- **[QUICK_DEPLOYMENT_REFERENCE.md](./QUICK_DEPLOYMENT_REFERENCE.md)** - Quick command reference
- **[DEPLOYMENT_CHECKLIST.md](./DEPLOYMENT_CHECKLIST.md)** - Step-by-step verification checklist
- **[DEPLOYMENT_PACKAGE_CONTENTS.txt](./DEPLOYMENT_PACKAGE_CONTENTS.txt)** - Complete package inventory

## Deployment Scripts

Main deployment scripts are located in the repository root:
- `deploy-monitoring-agents-parallel.sh` - Deploy monitoring agents to all VMs
- `deploy-complete-ai-system.sh` - Deploy complete AI monitoring system
- `deploy-monitoring-infrastructure.sh` - Deploy Prometheus/Grafana
- `deploy-node-exporters-fixed.sh` - Deploy node exporters
- `insert-validator-keys-accessible.sh` - Insert validator keys
- `install-etrid-monitoring.sh` - Universal installer

## Related Documentation

- **DevNet:** [../devnet/](../devnet/) - Development network documentation
- **Historical:** [../deployment-archive/](../deployment-archive/) - Previous deployment reports
- **Repository:** [../REPO_STRUCTURE.md](../REPO_STRUCTURE.md) - Repository organization

## Deployment Paths

### Path A: Quick Deployment (15 minutes)
1. Read [EXECUTIVE_SUMMARY.md](./EXECUTIVE_SUMMARY.md)
2. Run deployment script
3. Verify using [QUICK_DEPLOYMENT_REFERENCE.md](./QUICK_DEPLOYMENT_REFERENCE.md)

### Path B: Standard Deployment (1-2 hours)
1. Read [START_HERE.md](./START_HERE.md)
2. Read [DEPLOYMENT_PACKAGE_README.md](./DEPLOYMENT_PACKAGE_README.md)
3. Follow [DEPLOYMENT_CHECKLIST.md](./DEPLOYMENT_CHECKLIST.md)
4. Run deployment scripts
5. Complete verification

### Path C: Complete Documentation (2+ hours)
1. Read all documentation in order
2. Review [AGENT_DEPLOYMENT_GUIDE.md](./AGENT_DEPLOYMENT_GUIDE.md)
3. Follow detailed checklist
4. Complete all verification steps

---

**Last Updated:** 2025-11-01
