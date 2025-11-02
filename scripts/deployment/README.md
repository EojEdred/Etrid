# Deployment Scripts

All deployment scripts for ËTRID infrastructure, monitoring, and validators.

## Available Scripts

### AI Monitoring Deployment
- **`deploy-complete-ai-system.sh`** - Deploy complete AI monitoring system
- **`deploy-monitoring-agents-parallel.sh`** - Deploy monitoring agents to all VMs in parallel
- **`deploy-monitoring-infrastructure.sh`** - Deploy Prometheus/Grafana infrastructure
- **`install-etrid-monitoring.sh`** - Universal installer for monitoring stack

### Node Monitoring
- **`deploy-node-exporters-fixed.sh`** - Deploy node exporters for system metrics

### Validator Management
- **`insert-validator-keys-accessible.sh`** - Insert validator keys to accessible VMs

## Usage

All scripts are executable. Run from repository root:

```bash
# Example: Deploy monitoring agents
./scripts/deployment/deploy-monitoring-agents-parallel.sh

# Example: Install monitoring stack on a VM
./scripts/deployment/install-etrid-monitoring.sh
```

## Documentation

For complete deployment documentation, see:
- [docs/deployment/START_HERE.md](../../docs/deployment/START_HERE.md) - Deployment guide
- [docs/deployment/AGENT_DEPLOYMENT_GUIDE.md](../../docs/deployment/AGENT_DEPLOYMENT_GUIDE.md) - AI monitoring deployment

## Related Directories

- **DevNet Scripts:** [../devnet/](../devnet/) - Development network scripts
- **Archived Scripts:** [../archive/](../archive/) - Old script versions

---

*Last Updated: 2025-11-01*
