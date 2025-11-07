#!/usr/bin/env python3
import ftplib
import os

FTP_HOST = "ftp.etrid.org"
FTP_USER = "u724092535"
FTP_PASS = "Fullashit13!"

print("🚀 Uploading updated index.html with feature modals...\n")

try:
    ftp = ftplib.FTP(FTP_HOST, timeout=30)
    ftp.login(FTP_USER, FTP_PASS)
    print("✅ Connected to server\n")
    
    local_file = "website/index.html"
    remote_file = "/domains/etrid.org/public_html/index.html"
    
    file_size = os.path.getsize(local_file)
    print(f"📤 Uploading: {local_file} ({file_size:,} bytes)")
    
    with open(local_file, 'rb') as f:
        ftp.storbinary(f'STOR {remote_file}', f)
    
    print(f"✅ Upload complete!\n")
    print("="*60)
    print("🎉 Feature modals are now live!")
    print("="*60)
    print("\n📋 What's New:")
    print("   ✓ All 6 core feature cards are now clickable")
    print("   ✓ Click any feature to see detailed information:")
    print("     • ASF Consensus")
    print("     • FlareChain")
    print("     • Lightning-Bloc Layer 2")
    print("     • Oracle Network")
    print("     • ËtwasmVM")
    print("     • Multichain Architecture")
    print("\n🖱️  Features:")
    print("   • Click feature card to open modal")
    print("   • Click outside modal to close")
    print("   • Press ESC key to close")
    print("   • Close button in top-right corner")
    print("\n🌐 Test it: https://etrid.org/#features")
    
    ftp.quit()
    
except Exception as e:
    print(f"❌ Error: {str(e)}")
    exit(1)
