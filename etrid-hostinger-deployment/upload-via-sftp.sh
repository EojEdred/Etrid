#!/bin/bash
# Upload fixes via SFTP

HOST="157.173.214.206"
USER="u724092535"
PASS="Fullashit13!"

echo "🚀 Uploading fixes via SFTP..."

# Create SFTP batch file
cat > /tmp/sftp_commands.txt << 'EOF'
cd public_html/whitepaper
put "/Users/macbook/Desktop/etrid/etrid-hostinger-deployment /website/whitepaper/viewer-standalone.html" viewer-standalone.html
cd ../telemetry
put "/Users/macbook/Desktop/etrid/etrid-hostinger-deployment /apps/telemetry/app.js" app.js
cd ../explorer
put "/Users/macbook/Desktop/etrid/etrid-hostinger-deployment /apps/explorer/index.html" index.html
quit
EOF

# Upload via SFTP
sshpass -p "$PASS" sftp -oBatchMode=no -b /tmp/sftp_commands.txt "$USER@$HOST"

echo "✅ Upload complete!"
rm /tmp/sftp_commands.txt
