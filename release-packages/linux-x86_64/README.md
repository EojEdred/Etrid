# Linux x86-64 Binary

## Download Binary

The binary was built via GitHub Actions and can be downloaded in two ways:

### Option 1: Download from GitHub Actions Artifact
```bash
# Requires gh CLI installed and authenticated
cd ~/etrid
gh run download 19018648993 -n flarechain-node-linux-x86_64 -D /tmp/ -R EojEdred/Etrid
mkdir -p ~/etrid/target/release
cp /tmp/flarechain-node ~/etrid/target/release/
chmod +x ~/etrid/target/release/flarechain-node
```

### Option 2: Use the Binary Locally Available
The binary is available at `/tmp/flarechain-node` on the build machine and can be distributed via scp:

```bash
# From build machine
bash /tmp/distribute_github_binary.sh
```

## Binary Details
- **Size**: 75MB (unstripped)
- **Architecture**: ELF 64-bit LSB pie executable, x86-64
- **Built from**: Commit f252d416
- **GitHub Actions Run**: https://github.com/EojEdred/Etrid/actions/runs/19018648993
- **Features**: Genesis committee of 21 validators embedded

## Installation on Each VM

```bash
# Once binary is available
mkdir -p ~/etrid/target/release
mv flarechain-node ~/etrid/target/release/
chmod +x ~/etrid/target/release/flarechain-node
```

## Next Steps

After binary is distributed to all 16 validators:

1. Start all validators:
   ```bash
   bash /tmp/start_all_validators.sh
   ```

2. Monitor logs:
   ```bash
   ssh -i ~/.ssh/etrid_vm1 audit-dev01@<IP> 'tail -50 ~/validator.log'
   ```
