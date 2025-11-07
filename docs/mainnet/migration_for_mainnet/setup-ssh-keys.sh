#!/bin/bash
# Copy SSH keys to all Contabo VMs

VMS=(
    "85.239.239.194"
    "85.239.239.193"
    "85.239.239.190"
    "85.239.239.189"
    "85.239.239.188"
    "80.190.82.186"
    "80.190.82.185"
    "80.190.82.184"
    "80.190.82.183"
    "158.220.83.146"
    "158.220.83.66"
)

PASSWORD="G1zziPwr2025"
PUBKEY=$(cat ~/.ssh/contabo-validators.pub)

echo "Copying SSH keys to all VMs..."
echo ""

for ip in "${VMS[@]}"; do
    echo "Setting up $ip..."

    expect <<EOF
spawn ssh -o StrictHostKeyChecking=no root@$ip "mkdir -p ~/.ssh && chmod 700 ~/.ssh && echo '$PUBKEY' >> ~/.ssh/authorized_keys && chmod 600 ~/.ssh/authorized_keys && echo 'Key installed'"
expect "password:"
send "$PASSWORD\r"
expect eof
EOF

    echo ""
done

echo "SSH keys installed on all VMs!"
echo "Testing passwordless access..."
echo ""

for ip in "${VMS[@]}"; do
    echo -n "$ip: "
    ssh -i ~/.ssh/contabo-validators -o ConnectTimeout=5 root@$ip "echo 'OK'" 2>&1
done
