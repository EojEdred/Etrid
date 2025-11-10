# AWS Setup for FlareChain Validators

**Coming Soon**

See [Master VM Provider Setup](./MASTER_VM_PROVIDER_SETUP.md) for AWS-specific instructions.

**Key Points:**
- Maximum flexibility
- Security Group configuration required
- AWS CLI commands available

**Quick Setup:**
\`\`\`bash
# Add Security Group rule
aws ec2 authorize-security-group-ingress \\
  --group-id <security-group-id> \\
  --protocol tcp \\
  --port 30333 \\
  --cidr 0.0.0.0/0

# On the VM
sudo iptables -I INPUT 1 -p tcp --dport 30333 -j ACCEPT
\`\`\`
