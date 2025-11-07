#!/bin/bash
# deploy-landing-page.sh
# Automated deployment script for ÉTRID Lightning landing page

echo "🚀 ÉTRID Lightning Landing Page Deployment"
echo "=========================================="
echo ""

# Step 1: Build the landing page
echo "📦 Step 1: Building Next.js application..."
cd /home/user/Etrid/lightning-landing || exit 1

if [ ! -d "node_modules" ]; then
    echo "📥 Installing dependencies..."
    npm install
fi

echo "🔨 Building production bundle..."
npm run build

if [ $? -ne 0 ]; then
    echo "❌ Build failed! Please check for errors above."
    exit 1
fi

echo "✅ Build complete! Static files are in the 'out/' directory"
echo ""

# Step 2: FTP Configuration
echo "📋 Step 2: FTP Deployment Configuration"
echo "========================================"
echo ""
echo "To deploy to etrid.org/lightning, you have 3 options:"
echo ""
echo "OPTION A: Manual FTP Upload (Easiest)"
echo "--------------------------------------"
echo "1. Open your FTP client (FileZilla, Cyberduck, etc.)"
echo "2. Connect to: ftp.etrid.org"
echo "3. Navigate to: /public_html/lightning"
echo "4. Upload all files from: /home/user/Etrid/lightning-landing/out/"
echo ""
echo "OPTION B: Command-Line FTP (lftp)"
echo "----------------------------------"
echo "Run this command (replace credentials):"
echo ""
echo "lftp -c \""
echo "  set ftp:ssl-allow yes"
echo "  set ssl:verify-certificate no"
echo "  open -u YOUR_USERNAME,YOUR_PASSWORD ftp.etrid.org"
echo "  lcd /home/user/Etrid/lightning-landing/out"
echo "  cd /public_html/lightning"
echo "  mirror --reverse --delete --verbose"
echo "  bye"
echo "\""
echo ""
echo "OPTION C: Automated Script (requires credentials)"
echo "---------------------------------------------------"
echo "Create a file: deploy-ftp-credentials.sh"
echo "Add your FTP credentials and run the deploy script"
echo ""

# Check if out directory exists
if [ -d "out" ]; then
    echo "✅ Files ready for deployment in: /home/user/Etrid/lightning-landing/out/"
    echo ""
    echo "📊 Deployment Package Size:"
    du -sh out/
    echo ""
    echo "📁 Files to upload:"
    find out -type f | wc -l
    echo " files found"
else
    echo "❌ Build directory not found!"
    exit 1
fi

echo ""
echo "🎯 Next Steps:"
echo "1. Choose deployment option above"
echo "2. Upload files to your FTP server"
echo "3. Visit: https://etrid.org/lightning"
echo "4. Done! 🎉"
echo ""
