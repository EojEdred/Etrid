#!/bin/bash
# Manual Deployment Script for ËTRID Substrate Telemetry
# Run this script to deploy everything step-by-step

set -e

echo "╔════════════════════════════════════════════════════════╗"
echo "║   ËTRID Substrate Telemetry Manual Deployment         ║"
echo "╚════════════════════════════════════════════════════════╝"
echo ""

# Get script directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR"

echo "📦 Deployment files location: $SCRIPT_DIR"
echo ""

# Check if we have all required files
echo "🔍 Checking required files..."
REQUIRED_FILES=(
    "docker-compose.yml"
    "nginx-telemetry.conf"
    "deploy-telemetry.sh"
    "configure-validators.sh"
    "telemetry-feed-integration.js"
)

for file in "${REQUIRED_FILES[@]}"; do
    if [ -f "$file" ]; then
        echo "  ✅ $file"
    else
        echo "  ❌ Missing: $file"
        exit 1
    fi
done

echo ""
echo "════════════════════════════════════════════════════════"
echo " DEPLOYMENT OPTIONS"
echo "════════════════════════════════════════════════════════"
echo ""
echo "Choose deployment method:"
echo ""
echo "  1) Transfer files to server and deploy remotely (requires SSH)"
echo "  2) Show manual deployment commands (copy/paste on server)"
echo "  3) Create deployment tarball for manual transfer"
echo ""
read -p "Enter choice [1-3]: " choice

case $choice in
    1)
        echo ""
        read -p "Enter server IP [98.71.91.84]: " SERVER_IP
        SERVER_IP=${SERVER_IP:-98.71.91.84}
        
        read -p "Enter SSH key path [~/.ssh/gizzi-validator]: " SSH_KEY
        SSH_KEY=${SSH_KEY:-~/.ssh/gizzi-validator}
        
        read -p "Enter SSH user [ubuntu]: " SSH_USER
        SSH_USER=${SSH_USER:-ubuntu}
        
        echo ""
        echo "📤 Transferring files to $SSH_USER@$SERVER_IP..."
        
        scp -i "$SSH_KEY" \
            docker-compose.yml \
            nginx-telemetry.conf \
            deploy-telemetry.sh \
            "$SSH_USER@$SERVER_IP:~/"
        
        echo ""
        echo "✅ Files transferred!"
        echo ""
        echo "Now SSH to the server and run:"
        echo "  ssh -i $SSH_KEY $SSH_USER@$SERVER_IP"
        echo "  chmod +x deploy-telemetry.sh"
        echo "  ./deploy-telemetry.sh"
        ;;
        
    2)
        echo ""
        echo "════════════════════════════════════════════════════════"
        echo " MANUAL DEPLOYMENT COMMANDS"
        echo "════════════════════════════════════════════════════════"
        echo ""
        echo "1️⃣  SSH to your server:"
        echo ""
        echo "    ssh -i ~/.ssh/YOUR_KEY ubuntu@98.71.91.84"
        echo ""
        echo "2️⃣  Install Docker and Docker Compose:"
        echo ""
        cat << 'CMDEOF'
sudo apt update
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh
sudo usermod -aG docker $USER
sudo curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose
CMDEOF
        echo ""
        echo "3️⃣  Install Nginx and Certbot:"
        echo ""
        echo "    sudo apt install -y nginx certbot python3-certbot-nginx"
        echo ""
        echo "4️⃣  Create deployment directory and files:"
        echo ""
        cat << 'CMDEOF'
sudo mkdir -p /opt/substrate-telemetry
sudo chown $USER:$USER /opt/substrate-telemetry
cd /opt/substrate-telemetry
CMDEOF
        echo ""
        echo "5️⃣  Create docker-compose.yml (copy this entire block):"
        echo ""
        cat docker-compose.yml
        echo ""
        echo "6️⃣  Start telemetry containers:"
        echo ""
        echo "    docker-compose up -d"
        echo ""
        echo "7️⃣  Configure Nginx (copy this entire block):"
        echo ""
        cat nginx-telemetry.conf
        echo ""
        echo "8️⃣  Enable Nginx site:"
        echo ""
        cat << 'CMDEOF'
sudo cp nginx-telemetry.conf /etc/nginx/sites-available/telemetry
sudo rm -f /etc/nginx/sites-enabled/default
sudo ln -s /etc/nginx/sites-available/telemetry /etc/nginx/sites-enabled/
sudo nginx -t
CMDEOF
        echo ""
        echo "9️⃣  Create DNS record:"
        echo ""
        echo "    telemetry.etrid.org → $(curl -s ifconfig.me 2>/dev/null || echo '98.71.91.84')"
        echo ""
        echo "🔟  Get SSL certificate (after DNS propagates):"
        echo ""
        echo "    sudo certbot certonly --nginx -d telemetry.etrid.org"
        echo "    sudo systemctl reload nginx"
        echo ""
        echo "════════════════════════════════════════════════════════"
        ;;
        
    3)
        TARBALL="etrid-telemetry-deployment-$(date +%Y%m%d-%H%M%S).tar.gz"
        echo ""
        echo "📦 Creating deployment tarball..."
        
        tar -czf "$TARBALL" \
            docker-compose.yml \
            nginx-telemetry.conf \
            deploy-telemetry.sh \
            configure-validators.sh \
            telemetry-feed-integration.js \
            README.md \
            DEPLOYMENT_GUIDE.md \
            INTEGRATION_SUMMARY.md
        
        echo ""
        echo "✅ Created: $TARBALL"
        echo ""
        echo "Transfer this file to your server:"
        echo "  scp -i ~/.ssh/YOUR_KEY $TARBALL ubuntu@98.71.91.84:~/"
        echo ""
        echo "Then on the server:"
        echo "  tar -xzf $TARBALL"
        echo "  chmod +x deploy-telemetry.sh"
        echo "  ./deploy-telemetry.sh"
        ;;
        
    *)
        echo "Invalid choice. Exiting."
        exit 1
        ;;
esac

echo ""
echo "════════════════════════════════════════════════════════"
echo " NEXT STEPS"
echo "════════════════════════════════════════════════════════"
echo ""
echo "After deploying to server:"
echo ""
echo "1. Create DNS record: telemetry.etrid.org → 98.71.91.84"
echo "2. Wait 5-10 minutes for DNS propagation"
echo "3. Get SSL: sudo certbot certonly --nginx -d telemetry.etrid.org"
echo "4. Reload Nginx: sudo systemctl reload nginx"
echo "5. Configure validators (edit configure-validators.sh with IPs)"
echo "6. Update website (copy telemetry-feed-integration.js)"
echo ""
echo "📖 See DEPLOYMENT_GUIDE.md for complete instructions"
echo ""
