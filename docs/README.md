# Ëtrid FlareChain Documentation

Welcome to the Ëtrid FlareChain documentation. This directory contains comprehensive guides for deploying, securing, and troubleshooting the FlareChain validator network.

## 📚 Documentation Structure

### 🌐 Networking
Documentation related to network architecture, security, and P2P configuration.

- **[PRODUCTION_ARCHITECTURE.md](networking/PRODUCTION_ARCHITECTURE.md)** - Complete production network architecture with 4-tier security model (Bootstrap, Validators, AIDIDs, Directors)
- **[SECURITY_SETUP.md](networking/SECURITY_SETUP.md)** - Azure NSG rules, firewall configuration, and security best practices

### 🔧 Troubleshooting
Common issues and their solutions.

- **[P2P_DEBUGGING.md](troubleshooting/P2P_DEBUGGING.md)** - Complete P2P peer discovery fix documentation, root cause analysis, and code changes

## 🎯 Quick Links

### For New Validators
1. Read: [Production Architecture](networking/PRODUCTION_ARCHITECTURE.md)
2. Setup: [Security Configuration](networking/SECURITY_SETUP.md)
3. Deploy: See `infrastructure/deployment-scripts/`

### For Existing Validators
- **P2P Issues**: [P2P Debugging Guide](troubleshooting/P2P_DEBUGGING.md)
- **Security Audit**: [Security Setup](networking/SECURITY_SETUP.md)
- **Network Expansion**: [Production Architecture](networking/PRODUCTION_ARCHITECTURE.md)

## 📋 Key Concepts

### Network Tiers
FlareChain uses a 4-tier security architecture:

1. **Bootstrap Nodes** (5 nodes) - Public entry points, port 30334 open
2. **Regular Validators** (6+) - Whitelisted connections only
3. **AIDID Nodes** - Private VNet, no public access
4. **Director Nodes** - Maximum security, VNet only

### Two P2P Networks
Every validator runs TWO networks simultaneously:

- **Port 30333** - Substrate P2P (blocks, GRANDPA finality)
- **Port 30334** - DETR P2P (ASF finality, Kademlia DHT)

Both must be configured for full functionality.

## 🛠️ Related Resources

- **Generic Validator Script**: `scripts/one-command-validator.sh`
- **Script Documentation**: `scripts/README.md`
- **Chain Specifications**: `infrastructure/chain-specs/`
- **Deployment Scripts**: `infrastructure/deployment-scripts/`

---

**Last Updated**: October 29, 2025
