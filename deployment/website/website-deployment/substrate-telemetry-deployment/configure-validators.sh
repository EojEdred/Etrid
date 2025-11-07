#!/bin/bash
# Configure all ËTRID validators to report to telemetry server

set -e

echo "=========================================="
echo "Configure Validators for Telemetry"
echo "=========================================="
echo ""

# Telemetry endpoint
TELEMETRY_URL="wss://telemetry.etrid.org/submit/ 0"

echo "📡 Telemetry Endpoint: $TELEMETRY_URL"
echo ""

# Function to configure a single validator
configure_validator() {
    local name=$1
    local ip=$2
    
    echo "🔧 Configuring: $name ($ip)"
    
    # SSH command to update systemd service
    ssh -i ~/.ssh/gizzi-validator ubuntu@$ip << SSHEOF
        # Stop the validator
        sudo systemctl stop flarechain
        
        # Backup current service file
        sudo cp /etc/systemd/system/flarechain.service /etc/systemd/system/flarechain.service.backup
        
        # Update service file to include telemetry
        sudo sed -i 's|--validator|--validator --telemetry-url '"'"'$TELEMETRY_URL'"'"'|g' /etc/systemd/system/flarechain.service
        
        # Reload systemd and restart
        sudo systemctl daemon-reload
        sudo systemctl start flarechain
        
        echo "   ✅ $name configured and restarted"
SSHEOF
    
    if [ $? -eq 0 ]; then
        echo "   ✅ $name configured successfully"
    else
        echo "   ❌ Failed to configure $name"
    fi
    echo ""
}

# Configure all validators
# Add all 21 validator IPs here

echo "📋 Configuring primary validator..."
configure_validator "Validator-01" "98.71.91.84"

echo "📋 Configuring AuditDev validator..."
configure_validator "AuditDev" "129.80.122.34"

# Add remaining validators here
# configure_validator "Validator-03" "IP_ADDRESS"
# configure_validator "Validator-04" "IP_ADDRESS"
# ...continue for all 21 validators

echo ""
echo "=========================================="
echo "✅ Validator Configuration Complete!"
echo "=========================================="
echo ""
echo "📊 Check telemetry at: https://telemetry.etrid.org"
echo ""
echo "All validators should appear within 30 seconds."
echo ""
