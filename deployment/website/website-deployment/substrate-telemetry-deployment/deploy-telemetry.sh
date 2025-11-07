#!/bin/bash
# Deploy Substrate Telemetry Server on ËTRID Infrastructure
# Run this on the server: 98.71.91.84 (or dedicated telemetry server)

set -e

echo "=========================================="
echo "ËTRID Substrate Telemetry Deployment"
echo "=========================================="
echo ""

# Check if running as root
if [ "$EUID" -eq 0 ]; then 
    echo "⚠️  Please run as normal user with sudo privileges, not root"
    exit 1
fi

# Update system
echo "📦 Updating system packages..."
sudo apt update

# Install Docker if not present
if ! command -v docker &> /dev/null; then
    echo "🐳 Installing Docker..."
    curl -fsSL https://get.docker.com -o get-docker.sh
    sudo sh get-docker.sh
    sudo usermod -aG docker $USER
    rm get-docker.sh
    echo "✅ Docker installed"
else
    echo "✅ Docker already installed"
fi

# Install Docker Compose if not present
if ! command -v docker-compose &> /dev/null; then
    echo "🐳 Installing Docker Compose..."
    sudo curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
    sudo chmod +x /usr/local/bin/docker-compose
    echo "✅ Docker Compose installed"
else
    echo "✅ Docker Compose already installed"
fi

# Install Nginx if not present
if ! command -v nginx &> /dev/null; then
    echo "🌐 Installing Nginx..."
    sudo apt install -y nginx
    echo "✅ Nginx installed"
else
    echo "✅ Nginx already installed"
fi

# Install Certbot if not present
if ! command -v certbot &> /dev/null; then
    echo "🔒 Installing Certbot..."
    sudo apt install -y certbot python3-certbot-nginx
    echo "✅ Certbot installed"
else
    echo "✅ Certbot already installed"
fi

# Create deployment directory
DEPLOY_DIR="/opt/substrate-telemetry"
echo ""
echo "📁 Creating deployment directory: $DEPLOY_DIR"
sudo mkdir -p $DEPLOY_DIR
sudo chown $USER:$USER $DEPLOY_DIR

# Copy docker-compose.yml
echo "📄 Installing docker-compose.yml..."
cp docker-compose.yml $DEPLOY_DIR/

# Deploy telemetry containers
echo ""
echo "🚀 Starting Substrate Telemetry containers..."
cd $DEPLOY_DIR
docker-compose pull
docker-compose up -d

# Wait for containers to start
echo "⏳ Waiting for containers to initialize..."
sleep 10

# Check container status
echo ""
echo "📊 Container status:"
docker-compose ps

# Configure Nginx
echo ""
echo "🌐 Configuring Nginx..."
sudo cp nginx-telemetry.conf /etc/nginx/sites-available/telemetry

# Remove default site if it exists
if [ -f /etc/nginx/sites-enabled/default ]; then
    echo "   Removing default Nginx site..."
    sudo rm /etc/nginx/sites-enabled/default
fi

# Enable telemetry site (but don't reload yet - need SSL first)
if [ ! -f /etc/nginx/sites-enabled/telemetry ]; then
    sudo ln -s /etc/nginx/sites-available/telemetry /etc/nginx/sites-enabled/
fi

# Test Nginx config
echo "   Testing Nginx configuration..."
sudo nginx -t

echo ""
echo "=========================================="
echo "✅ Telemetry Server Deployed!"
echo "=========================================="
echo ""
echo "📋 Next Steps:"
echo ""
echo "1. Create DNS A record:"
echo "   telemetry.etrid.org → $(curl -s ifconfig.me)"
echo ""
echo "2. Wait for DNS propagation (5-10 minutes), then get SSL certificate:"
echo "   sudo certbot certonly --nginx -d telemetry.etrid.org"
echo ""
echo "3. Reload Nginx:"
echo "   sudo systemctl reload nginx"
echo ""
echo "4. Configure validators to report:"
echo "   --telemetry-url 'wss://telemetry.etrid.org/submit/ 0'"
echo ""
echo "5. Access telemetry UI:"
echo "   https://telemetry.etrid.org"
echo ""
echo "🔍 Container logs:"
echo "   docker-compose logs -f backend"
echo "   docker-compose logs -f frontend"
echo "   docker-compose logs -f shard"
echo ""
