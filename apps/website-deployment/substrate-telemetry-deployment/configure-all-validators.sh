#!/bin/bash
# Configure all validators for telemetry

TELEMETRY_URL="ws://98.71.91.84/submit 0"

# Read validator IPs
VALIDATOR_FILE="/Users/macbook/Desktop/etrid/infrastructure/config/validator-ips.json"

echo "🚀 Configuring All ËTRID Validators for Telemetry"
echo "=================================================="
echo ""
echo "📡 Telemetry URL: $TELEMETRY_URL"
echo ""

# Parse JSON and configure each validator
python3 << 'PYEOF'
import json
import subprocess
import sys

with open('/Users/macbook/Desktop/etrid/infrastructure/config/validator-ips.json', 'r') as f:
    data = json.load(f)

telemetry_url = "ws://98.71.91.84/submit 0"
successful = []
failed = []

for validator in data['validators']:
    num = validator['number']
    name = validator['name']
    ip = validator['ip']
    username = validator['aiDevId']
    
    print(f"\n{'='*60}")
    print(f"🔧 Validator #{num}: {name}")
    print(f"📍 IP: {ip} | Username: {username}")
    print(f"{'='*60}")
    
    # Skip the BUILD VM (validator #10)
    if 'BUILD VM' in validator.get('notes', ''):
        print("⏭️  Skipping BUILD VM")
        continue
    
    # SSH and configure
    ssh_cmd = f"""
    ssh -i ~/.ssh/gizzi-validator -o ConnectTimeout=10 -o StrictHostKeyChecking=no {username}@{ip} '
        echo "Stopping validator..."
        sudo systemctl stop flarechain 2>/dev/null || true
        
        if [ -f /etc/systemd/system/flarechain.service ]; then
            echo "Backing up service file..."
            sudo cp /etc/systemd/system/flarechain.service /etc/systemd/system/flarechain.service.backup-$(date +%Y%m%d-%H%M%S)
            
            echo "Updating service file..."
            if grep -q "telemetry-url" /etc/systemd/system/flarechain.service; then
                echo "  Updating existing telemetry URL..."
                sudo sed -i "s|--telemetry-url.*0.*|--telemetry-url '"'"'{telemetry_url}'"'"' \\\\\\\\|g" /etc/systemd/system/flarechain.service
            else
                echo "  Adding telemetry URL..."
                sudo sed -i "/--validator/a \\  --telemetry-url '"'"'{telemetry_url}'"'"' \\\\\\\\" /etc/systemd/system/flarechain.service
            fi
            
            echo "Reloading systemd..."
            sudo systemctl daemon-reload
            
            echo "Starting validator..."
            sudo systemctl start flarechain
            
            echo "Waiting for service to start..."
            sleep 5
            
            echo "Checking logs..."
            sudo journalctl -u flarechain -n 20 | grep -i telemetry || echo "  (No telemetry messages yet - check in a minute)"
            
            echo "✅ Configured!"
        else
            echo "⚠️  Service file not found"
            exit 1
        fi
    '
    """
    
    result = subprocess.run(ssh_cmd, shell=True, capture_output=True, text=True)
    
    if result.returncode == 0:
        print(f"✅ Successfully configured {name}")
        successful.append(f"#{num} {name}")
    else:
        print(f"❌ Failed to configure {name}")
        print(f"Error: {result.stderr[:200]}")
        failed.append(f"#{num} {name}: {result.stderr[:100]}")

print(f"\n{'='*60}")
print("📊 CONFIGURATION SUMMARY")
print(f"{'='*60}")
print(f"✅ Successful: {len(successful)}")
for v in successful:
    print(f"   {v}")

print(f"\n❌ Failed: {len(failed)}")
for v in failed:
    print(f"   {v}")

print(f"\n🌐 View telemetry: http://98.71.91.84/")
print(f"🌐 Website: https://etrid.org/telemetry/")
PYEOF
