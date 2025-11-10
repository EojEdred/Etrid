# Oracle Cloud Setup for FlareChain Validators

**Coming Soon**

See [Master VM Provider Setup](./MASTER_VM_PROVIDER_SETUP.md) for Oracle Cloud-specific instructions.

**Key Points for Oracle Cloud:**
- Free tier available (2 VMs with 1GB RAM each)
- Requires Network Security Group configuration
- Both NSG and VM iptables need port 30333 open

**Quick Fix:**
\`\`\`bash
# On the VM
sudo iptables -I INPUT 1 -p tcp --dport 30333 -j ACCEPT
sudo apt-get install -y iptables-persistent
sudo netfilter-persistent save
\`\`\`

**In Oracle Cloud Console:**
- Virtual Cloud Networks → Security Lists → Add Ingress Rule
- Source: 0.0.0.0/0, Protocol: TCP, Port: 30333

See existing Oracle Cloud validators:
- Gizzi (Director-1): 64.181.215.19
- AuditDev (Director-5): 129.80.122.34
