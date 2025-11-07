#!/bin/bash
# deploy-to-ftp.sh
# TEMPLATE - Replace YOUR_* values with your actual FTP credentials

# ⚠️ IMPORTANT: Fill in your FTP credentials below
FTP_HOST="ftp.etrid.org"
FTP_USER="YOUR_FTP_USERNAME"     # ← CHANGE THIS
FTP_PASS="YOUR_FTP_PASSWORD"     # ← CHANGE THIS
FTP_DIR="/public_html/lightning" # ← Verify this path

echo "🚀 Deploying to FTP..."

# Check if lftp is installed
if ! command -v lftp &> /dev/null; then
    echo "❌ lftp not found. Installing..."
    # Uncomment your OS:
    # sudo apt-get install -y lftp  # Ubuntu/Debian
    # sudo yum install -y lftp      # CentOS/RHEL
    # brew install lftp             # macOS
    exit 1
fi

# Build the site first
cd /home/user/Etrid/lightning-landing
npm run build

# Deploy via FTP
echo "📤 Uploading to $FTP_HOST..."
lftp -c "
    set ftp:ssl-allow yes
    set ssl:verify-certificate no
    open -u $FTP_USER,$FTP_PASS $FTP_HOST
    lcd out
    cd $FTP_DIR
    mirror --reverse --delete --verbose --exclude .git/
    bye
"

if [ $? -eq 0 ]; then
    echo "✅ Deployment successful!"
    echo "🌐 Visit: https://etrid.org/lightning"
else
    echo "❌ Deployment failed. Check credentials and network connection."
fi
