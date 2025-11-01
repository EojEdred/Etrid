#!/usr/bin/env python3
"""Upload with correct Hostinger path structure"""

import ftplib
import os
import sys

HOST = "157.173.214.206"
USER = "u724092535"
PASS = "Fullashit13!"

# Correct paths for Hostinger
FILES = [
    {
        'local': '/Users/macbook/Desktop/etrid/etrid-hostinger-deployment /website/whitepaper/viewer-standalone.html',
        'remote': '/domains/etrid.org/public_html/whitepaper/viewer-standalone.html',
        'desc': 'Whitepaper Viewer (AOS removed - 401 KB)'
    },
    {
        'local': '/Users/macbook/Desktop/etrid/etrid-hostinger-deployment /apps/telemetry/app.js',
        'remote': '/domains/etrid.org/public_html/telemetry/app.js',
        'desc': 'Telemetry App (ws://98.71.91.84:9944)'
    },
    {
        'local': '/Users/macbook/Desktop/etrid/etrid-hostinger-deployment /apps/explorer/index.html',
        'remote': '/domains/etrid.org/public_html/explorer/index.html',
        'desc': 'Explorer (RPC endpoints updated)'
    }
]

print("=" * 70)
print("🚀 UPLOADING FIXES TO ETRID.ORG")
print("=" * 70)

try:
    # Connect and login
    print(f"\n🔌 Connecting to {HOST}...")
    ftp = ftplib.FTP()
    ftp.connect(HOST, 21, timeout=30)
    ftp.login(USER, PASS)
    ftp.set_pasv(True)
    print(f"✅ Connected and logged in!")

    # Upload each file
    success_count = 0
    total_bytes = 0

    for file_info in FILES:
        local = file_info['local']
        remote = file_info['remote']
        desc = file_info['desc']

        if not os.path.exists(local):
            print(f"\n❌ File not found: {local}")
            continue

        size = os.path.getsize(local)
        total_bytes += size

        print(f"\n{'='*70}")
        print(f"📤 {desc}")
        print(f"   File: {os.path.basename(local)}")
        print(f"   Size: {size:,} bytes ({size/1024:.1f} KB)")
        print(f"   Path: {remote}")

        try:
            # Navigate to directory
            remote_dir = os.path.dirname(remote)
            remote_file = os.path.basename(remote)

            # Change to directory
            ftp.cwd(remote_dir)
            print(f"   📁 In directory: {remote_dir}")

            # Upload with progress
            with open(local, 'rb') as f:
                print(f"   ⏳ Uploading...")
                ftp.storbinary(f'STOR {remote_file}', f, blocksize=8192)

            print(f"   ✅ UPLOADED!")
            success_count += 1

        except Exception as e:
            print(f"   ❌ Failed: {e}")

    # Close
    ftp.quit()

    # Summary
    print(f"\n{'='*70}")
    print(f"📊 UPLOAD SUMMARY")
    print(f"{'='*70}")
    print(f"✅ Successful: {success_count}/{len(FILES)}")
    print(f"📦 Total uploaded: {total_bytes:,} bytes ({total_bytes/1024:.1f} KB)")

    if success_count == len(FILES):
        print(f"\n🎉 ALL FILES DEPLOYED SUCCESSFULLY!")
        print(f"\n✅ What's been fixed:")
        print(f"   • Whitepaper: AOS library removed (CSP issue solved)")
        print(f"   • Telemetry: Connected to ws://98.71.91.84:9944")
        print(f"   • Explorer: Shows correct RPC endpoints")
        print(f"\n🔍 Test your site now:")
        print(f"   • https://etrid.org/whitepaper/viewer-standalone.html")
        print(f"   • https://etrid.org/telemetry/")
        print(f"   • https://etrid.org/explorer/")
        print(f"\n💡 Hard refresh (Ctrl+Shift+R or Cmd+Shift+R) to clear cache!")
        sys.exit(0)
    else:
        print(f"\n⚠️  Some uploads failed")
        sys.exit(1)

except Exception as e:
    print(f"\n❌ Error: {e}")
    sys.exit(1)
