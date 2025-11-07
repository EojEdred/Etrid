#!/usr/bin/env python3
"""
Upload ËTRID website integration changes to Hostinger
Uploads: Network monitoring page, updated telemetry, updated explorer
"""

import ftplib
import os
from pathlib import Path

# FTP Configuration
FTP_HOST = '157.173.214.206'
FTP_USER = 'u724092535'
FTP_PASS = 'Fullashit13!'
FTP_PORT = 21
REMOTE_BASE = '/domains/etrid.org/public_html'

# Files to upload
FILES_TO_UPLOAD = [
    {
        'local': 'website/network/index.html',
        'remote': '/network/index.html'
    },
    {
        'local': 'apps/telemetry/app.js',
        'remote': '/telemetry/app.js'
    },
    {
        'local': 'apps/explorer/index.html',
        'remote': '/explorer/index.html'
    }
]

def upload_file(ftp, local_path, remote_path):
    """Upload a single file via FTP"""
    print(f"📤 Uploading {local_path} -> {remote_path}")
    
    # Ensure remote directory exists
    remote_dir = os.path.dirname(remote_path)
    if remote_dir and remote_dir != '/':
        try:
            ftp.mkd(REMOTE_BASE + remote_dir)
        except:
            pass  # Directory might already exist
    
    # Upload file
    with open(local_path, 'rb') as f:
        ftp.storbinary(f'STOR {REMOTE_BASE}{remote_path}', f)
    
    print(f"   ✅ Uploaded successfully")

def main():
    print("=" * 60)
    print("ËTRID Website Integration Deployment")
    print("=" * 60)
    
    # Get base directory
    base_dir = Path(__file__).parent
    
    # Connect to FTP
    print(f"\n🔌 Connecting to {FTP_HOST}...")
    try:
        ftp = ftplib.FTP()
        ftp.connect(FTP_HOST, FTP_PORT)
        ftp.login(FTP_USER, FTP_PASS)
        print("✅ Connected successfully")
        
        # Upload each file
        print(f"\n📦 Uploading {len(FILES_TO_UPLOAD)} files...\n")
        
        for file_info in FILES_TO_UPLOAD:
            local_path = base_dir / file_info['local']
            remote_path = file_info['remote']
            
            if not local_path.exists():
                print(f"⚠️  File not found: {local_path}")
                continue
            
            upload_file(ftp, str(local_path), remote_path)
        
        # Close connection
        ftp.quit()
        
        print("\n" + "=" * 60)
        print("✅ DEPLOYMENT COMPLETE!")
        print("=" * 60)
        print("\nDeployed pages:")
        print("  • Network Monitor: https://etrid.org/network/")
        print("  • Telemetry (updated): https://etrid.org/telemetry/")
        print("  • Explorer (updated): https://etrid.org/explorer/")
        print("\n🎉 All apps now connected to real blockchain data!")
        
    except Exception as e:
        print(f"\n❌ Error: {e}")
        return 1
    
    return 0

if __name__ == '__main__':
    exit(main())
