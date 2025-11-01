#!/usr/bin/env python3
"""Final upload attempt with exact credentials provided"""

import ftplib
import os
import sys

# Exact credentials from user
HOST = "157.173.214.206"
USER = "u724092535"
PASS = "Fullashit13!"
PORT = 21

FILES = [
    {
        'local': '/Users/macbook/Desktop/etrid/etrid-hostinger-deployment /website/whitepaper/viewer-standalone.html',
        'remote': 'public_html/whitepaper/viewer-standalone.html',
        'desc': 'Whitepaper (NO AOS - 401 KB)'
    },
    {
        'local': '/Users/macbook/Desktop/etrid/etrid-hostinger-deployment /apps/telemetry/app.js',
        'remote': 'public_html/telemetry/app.js',
        'desc': 'Telemetry (RPC: ws://98.71.91.84:9944)'
    },
    {
        'local': '/Users/macbook/Desktop/etrid/etrid-hostinger-deployment /apps/explorer/index.html',
        'remote': 'public_html/explorer/index.html',
        'desc': 'Explorer (endpoints updated)'
    }
]

print("=" * 70)
print("🚀 FINAL FTP UPLOAD ATTEMPT")
print("=" * 70)
print(f"Host: {HOST}")
print(f"Port: {PORT}")
print(f"User: {USER}")
print(f"Pass: {'*' * len(PASS)}")
print("=" * 70)

try:
    # Connect
    print(f"\n🔌 Connecting to {HOST}:{PORT}...")
    ftp = ftplib.FTP()
    ftp.connect(HOST, PORT, timeout=30)
    print(f"✅ Connected!")
    print(f"Server says: {ftp.getwelcome()}")

    # Login
    print(f"\n🔐 Logging in as {USER}...")
    ftp.login(USER, PASS)
    print(f"✅ LOGIN SUCCESSFUL!")

    # Check directory
    pwd = ftp.pwd()
    print(f"\n📂 Current directory: {pwd}")

    # Set passive mode
    ftp.set_pasv(True)
    print(f"✅ Passive mode enabled")

    # Upload each file
    success_count = 0
    for file_info in FILES:
        local = file_info['local']
        remote = file_info['remote']
        desc = file_info['desc']

        if not os.path.exists(local):
            print(f"\n❌ File not found: {local}")
            continue

        size = os.path.getsize(local)
        print(f"\n{'='*70}")
        print(f"📤 Uploading: {desc}")
        print(f"   Local:  {os.path.basename(local)}")
        print(f"   Size:   {size:,} bytes ({size/1024:.1f} KB)")
        print(f"   Remote: {remote}")

        try:
            # Navigate to remote directory
            remote_dir = os.path.dirname(remote)
            remote_file = os.path.basename(remote)

            # Try to change to directory
            ftp.cwd('/')
            print(f"   📁 Changed to root")

            ftp.cwd(remote_dir)
            print(f"   📁 Changed to {remote_dir}")

            # Upload in binary mode
            with open(local, 'rb') as f:
                ftp.storbinary(f'STOR {remote_file}', f, blocksize=8192)

            print(f"   ✅ UPLOADED SUCCESSFULLY!")
            success_count += 1

        except Exception as e:
            print(f"   ❌ Upload failed: {e}")

    # Close connection
    ftp.quit()
    print(f"\n{'='*70}")

    if success_count == len(FILES):
        print("🎉 SUCCESS! All files uploaded!")
        print("=" * 70)
        print("\n✅ Deployed files:")
        for f in FILES:
            print(f"   • {f['desc']}")
        print("\n🔍 Test your fixes:")
        print("   • https://etrid.org/whitepaper/viewer-standalone.html")
        print("   • https://etrid.org/telemetry/")
        print("   • https://etrid.org/explorer/")
        print("\n💡 Hard refresh (Ctrl+Shift+R or Cmd+Shift+R) to see changes!")
        sys.exit(0)
    else:
        print(f"⚠️  Partial success: {success_count}/{len(FILES)} uploaded")
        sys.exit(1)

except ftplib.error_perm as e:
    print(f"\n❌ FTP ERROR: {e}")
    print("\n💡 This error means:")
    print("   • Password is incorrect")
    print("   • FTP account doesn't exist")
    print("   • FTP is disabled in Hostinger")
    print("\n🔧 Solutions:")
    print("   1. Login to hpanel.hostinger.com")
    print("   2. Go to Files → FTP Accounts")
    print("   3. Reset password OR create new FTP account")
    print("   4. OR use File Manager to upload manually")
    sys.exit(1)

except Exception as e:
    print(f"\n❌ Connection failed: {e}")
    sys.exit(1)
