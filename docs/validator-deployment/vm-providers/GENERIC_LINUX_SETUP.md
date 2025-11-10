# Generic Linux VPS Setup

**Works with:** Linode, Vultr, Hetzner, OVH, or any Linux VPS

## Firewall Setup

\`\`\`bash
# Using iptables
sudo iptables -I INPUT 1 -p tcp --dport 30333 -j ACCEPT
sudo apt-get install -y iptables-persistent
sudo netfilter-persistent save

# OR using ufw
sudo ufw allow 30333/tcp
sudo ufw enable
\`\`\`

## Deployment

\`\`\`bash
# Use the automated script
cd ~/Desktop/etrid
./docs/validator-deployment/scripts/deploy-new-validator.sh <number> <ip> "<name>"
\`\`\`

See [Complete Setup Checklist](../guides/COMPLETE_SETUP_CHECKLIST.md) for manual steps.
