# DigitalOcean Setup for FlareChain Validators

**Coming Soon**

See [Master VM Provider Setup](./MASTER_VM_PROVIDER_SETUP.md) for DigitalOcean-specific instructions.

**Key Points:**
- Developer-friendly interface
- Cloud Firewall available (optional)
- Simple pricing (~$18-36/month)

**Quick Setup:**
\`\`\`bash
# If using Cloud Firewall
# → Networking → Firewalls → Add Inbound Rule
# Type: Custom, Protocol: TCP, Port: 30333

# On the VM (if not using Cloud Firewall)
sudo ufw allow 30333/tcp
sudo ufw enable
\`\`\`
