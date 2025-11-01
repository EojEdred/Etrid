#!/bin/bash
# Configure a single validator to report to ËTRID telemetry

if [ $# -lt 2 ]; then
    echo "Usage: $0 <validator_name> <validator_ip>"
    echo "Example: $0 Validator-01 98.71.91.84"
    exit 1
fi

VALIDATOR_NAME=$1
VALIDATOR_IP=$2
TELEMETRY_URL="ws://98.71.91.84/submit 0"

echo "🔧 Configuring $VALIDATOR_NAME ($VALIDATOR_IP)"
echo "📡 Telemetry: $TELEMETRY_URL"
echo ""

ssh -i ~/.ssh/gizzi-validator ubuntu@$VALIDATOR_IP << SSHEOF
    echo "Stopping validator..."
    sudo systemctl stop flarechain 2>/dev/null || true
    
    echo "Backing up service file..."
    sudo cp /etc/systemd/system/flarechain.service /etc/systemd/system/flarechain.service.backup-\$(date +%Y%m%d-%H%M%S)
    
    echo "Updating service file..."
    # Check if telemetry URL already exists
    if grep -q "telemetry-url" /etc/systemd/system/flarechain.service; then
        echo "  Telemetry already configured, updating..."
        sudo sed -i 's|--telemetry-url.*0.*|--telemetry-url '"'"'$TELEMETRY_URL'"'"' \\\\|g' /etc/systemd/system/flarechain.service
    else
        echo "  Adding telemetry configuration..."
        sudo sed -i '/--validator/a \  --telemetry-url '"'"'$TELEMETRY_URL'"'"' \\\\' /etc/systemd/system/flarechain.service
    fi
    
    echo "Reloading systemd..."
    sudo systemctl daemon-reload
    
    echo "Starting validator..."
    sudo systemctl start flarechain
    
    echo ""
    echo "Waiting 5 seconds..."
    sleep 5
    
    echo "Checking logs for telemetry connection..."
    sudo journalctl -u flarechain -n 50 | grep -i telemetry || echo "  (No telemetry messages yet - check in a minute)"
    
    echo ""
    echo "✅ $VALIDATOR_NAME configured!"
SSHEOF

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Successfully configured $VALIDATOR_NAME"
else
    echo ""
    echo "❌ Failed to configure $VALIDATOR_NAME"
fi
