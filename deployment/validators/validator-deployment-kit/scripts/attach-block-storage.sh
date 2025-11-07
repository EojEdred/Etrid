#!/bin/bash
# attach-block-storage.sh
# Helper script to attach and configure block storage for validators
# Usage: ./attach-block-storage.sh <validator-name> <provider>
# Example: ./attach-block-storage.sh validator-04 hetzner

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Configuration
VALIDATOR_NAME="$1"
PROVIDER="$2"

if [ -z "$VALIDATOR_NAME" ] || [ -z "$PROVIDER" ]; then
    echo "Usage: $0 <validator-name> <provider>"
    echo ""
    echo "Providers: hetzner, vultr, digitalocean"
    echo ""
    echo "Example: $0 validator-04 hetzner"
    exit 1
fi

PROVIDER=$(echo "$PROVIDER" | tr '[:upper:]' '[:lower:]')

echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║         Attach Block Storage to Validator                 ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo "Validator: $VALIDATOR_NAME"
echo "Provider: $PROVIDER"
echo ""

# Provider-specific instructions and costs
case "$PROVIDER" in
    hetzner)
        echo "Hetzner Volume Configuration:"
        echo "  Size: 500 GB (recommended)"
        echo "  Cost: €23.80/month (~\$25/month)"
        echo "  Performance: NVMe-backed, 10,000 IOPS"
        echo ""
        echo "Step 1: Create volume via Hetzner Cloud CLI"
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        echo ""
        echo "Run:"
        echo -e "${GREEN}hcloud volume create \\${NC}"
        echo -e "${GREEN}  --name ${VALIDATOR_NAME}-archive \\${NC}"
        echo -e "${GREEN}  --size 500 \\${NC}"
        echo -e "${GREEN}  --location fsn1${NC}"
        echo ""
        read -p "Press Enter after creating volume..."
        echo ""
        echo "Step 2: Attach volume to server"
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        echo ""
        echo "Run:"
        echo -e "${GREEN}hcloud volume attach ${VALIDATOR_NAME}-archive $VALIDATOR_NAME${NC}"
        echo ""
        read -p "Press Enter after attaching volume..."
        DEVICE="/dev/sdb"
        ;;

    vultr)
        echo "Vultr Block Storage Configuration:"
        echo "  Size: 500 GB (recommended)"
        echo "  Cost: \$50/month"
        echo "  Performance: NVMe, high IOPS"
        echo ""
        echo "Step 1: Create block storage via Vultr CLI"
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        echo ""
        echo "Run:"
        echo -e "${GREEN}vultr-cli block-storage create \\${NC}"
        echo -e "${GREEN}  --label ${VALIDATOR_NAME}-archive \\${NC}"
        echo -e "${GREEN}  --size 500 \\${NC}"
        echo -e "${GREEN}  --region <your-region>${NC}"
        echo ""
        read -p "Press Enter after creating block storage..."
        echo ""
        echo "Step 2: Attach to instance via Vultr console"
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        echo "Go to: https://my.vultr.com/storage/"
        echo "Click 'Attach' next to the block storage you created"
        echo ""
        read -p "Press Enter after attaching..."
        DEVICE="/dev/vdb"
        ;;

    digitalocean)
        echo "DigitalOcean Volume Configuration:"
        echo "  Size: 500 GB (recommended)"
        echo "  Cost: \$50/month"
        echo "  Performance: SSD, 7,500 IOPS"
        echo ""
        echo "Step 1: Create volume via doctl"
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        echo ""
        echo "Run:"
        echo -e "${GREEN}doctl compute volume create ${VALIDATOR_NAME}-archive \\${NC}"
        echo -e "${GREEN}  --size 500GiB \\${NC}"
        echo -e "${GREEN}  --region <your-region>${NC}"
        echo ""
        read -p "Press Enter after creating volume..."
        echo ""
        echo "Step 2: Attach to droplet"
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        echo ""
        echo "Get droplet ID:"
        echo -e "${GREEN}doctl compute droplet list | grep $VALIDATOR_NAME${NC}"
        echo ""
        read -p "Enter droplet ID: " DROPLET_ID
        echo ""
        echo "Attach volume:"
        echo -e "${GREEN}doctl compute volume-action attach ${VALIDATOR_NAME}-archive $DROPLET_ID${NC}"
        echo ""
        read -p "Press Enter after attaching..."
        DEVICE="/dev/disk/by-id/scsi-0DO_Volume_${VALIDATOR_NAME}-archive"
        ;;

    *)
        echo -e "${RED}Error: Unknown provider '$PROVIDER'${NC}"
        echo "Supported providers: hetzner, vultr, digitalocean"
        exit 1
        ;;
esac

echo ""
echo "Step 3: Configure on VM"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Get VM IP
INVENTORY_FILE="validator-inventory.txt"
if [ -f "$INVENTORY_FILE" ]; then
    VM_IP=$(grep "$VALIDATOR_NAME" "$INVENTORY_FILE" | awk '{print $2}' | head -1)
else
    read -p "Enter VM IP address: " VM_IP
fi

if [ -z "$VM_IP" ]; then
    echo -e "${RED}Error: Could not determine VM IP${NC}"
    exit 1
fi

echo "VM IP: $VM_IP"
echo ""

# Create remote setup script
SETUP_SCRIPT=$(cat <<'EOF'
#!/bin/bash
set -e

DEVICE=$1
MOUNT_PATH="/mnt/blockchain-archive"

echo "Setting up block storage..."
echo ""

# Check if device exists
if [ ! -b "$DEVICE" ]; then
    echo "Error: Device $DEVICE not found"
    echo "Available devices:"
    lsblk
    exit 1
fi

echo "Device found: $DEVICE"
echo ""

# Check if already formatted
if blkid "$DEVICE" > /dev/null 2>&1; then
    echo "Device is already formatted:"
    blkid "$DEVICE"
    echo ""
    read -p "Reformat? This will ERASE all data! (yes/no): " confirm
    if [ "$confirm" != "yes" ]; then
        echo "Skipping format..."
    else
        echo "Formatting $DEVICE as ext4..."
        mkfs.ext4 -F "$DEVICE"
    fi
else
    echo "Formatting $DEVICE as ext4..."
    mkfs.ext4 "$DEVICE"
fi

echo ""

# Create mount point
if [ ! -d "$MOUNT_PATH" ]; then
    echo "Creating mount point: $MOUNT_PATH"
    mkdir -p "$MOUNT_PATH"
fi

# Mount
echo "Mounting $DEVICE to $MOUNT_PATH..."
mount "$DEVICE" "$MOUNT_PATH"

# Add to fstab
UUID=$(blkid -s UUID -o value "$DEVICE")
FSTAB_ENTRY="UUID=$UUID $MOUNT_PATH ext4 defaults,nofail 0 2"

if ! grep -q "$UUID" /etc/fstab; then
    echo "Adding to /etc/fstab..."
    echo "$FSTAB_ENTRY" >> /etc/fstab
else
    echo "Already in /etc/fstab"
fi

# Set permissions
chown -R root:root "$MOUNT_PATH"
chmod 755 "$MOUNT_PATH"

echo ""
echo "✓ Block storage configured successfully"
echo ""
df -h "$MOUNT_PATH"
EOF
)

echo "Connecting to VM and configuring storage..."
echo ""

# Send setup script to VM and execute
echo "$SETUP_SCRIPT" | ssh -o StrictHostKeyChecking=no root@$VM_IP "cat > /tmp/setup-storage.sh && chmod +x /tmp/setup-storage.sh && /tmp/setup-storage.sh $DEVICE"

if [ $? -eq 0 ]; then
    echo ""
    echo -e "${GREEN}✓ Block storage attached and mounted successfully!${NC}"
    echo ""
    echo "Storage is now available at: /mnt/blockchain-archive"
    echo ""
    echo "Next steps:"
    echo "  1. Run auto-tier-storage.sh to move old data"
    echo "  2. Monitor with monitor-validator-storage.sh"
    echo "  3. Restart validator if needed"
else
    echo ""
    echo -e "${RED}✗ Failed to configure storage${NC}"
    echo "Please check the VM manually"
    exit 1
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Configuration Summary:"
echo ""
echo "  Validator: $VALIDATOR_NAME"
echo "  Provider: $PROVIDER"
echo "  Device: $DEVICE"
echo "  Mount: /mnt/blockchain-archive"
echo "  Cost: Added to monthly bill"
case "$PROVIDER" in
    hetzner) echo "        Hetzner: +\$25/month" ;;
    vultr) echo "        Vultr: +\$50/month" ;;
    digitalocean) echo "        DigitalOcean: +\$50/month" ;;
esac
echo ""
