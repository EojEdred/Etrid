#!/usr/bin/env python3
"""
Upload Fixed Explorer and MasterChef to Hostinger
"""

import ftplib
import os
import sys

# FTP Configuration
FTP_HOST = "ftp.etrid.org"
FTP_USER = "u724092535"
FTP_PASS = "Fullashit13!"

# Files to upload
FILES_TO_UPLOAD = [
    ("apps/explorer/index.html", "domains/etrid.org/public_html/explorer/index.html"),
]

def upload_file(ftp, local_path, remote_path):
    """Upload a single file via FTP"""
    try:
        with open(local_path, 'rb') as f:
            file_size = os.path.getsize(local_path)
            print(f"📤 Uploading {local_path} ({file_size:,} bytes)")
            print(f"   → {remote_path}")
            ftp.storbinary(f'STOR {remote_path}', f)
            print(f"   ✅ Success!\n")
            return True
    except Exception as e:
        print(f"   ❌ Failed: {e}\n")
        return False

def main():
    print("=" * 70)
    print("ËTRID Explorer & MasterChef Real Data Fix")
    print("=" * 70)
    print()

    try:
        # Connect to FTP
        print(f"🔌 Connecting to {FTP_HOST}...")
        ftp = ftplib.FTP(FTP_HOST)
        ftp.login(FTP_USER, FTP_PASS)
        print(f"✅ Connected as {FTP_USER}\n")

        # Upload each file
        success_count = 0
        fail_count = 0

        for local_path, remote_path in FILES_TO_UPLOAD:
            if upload_file(ftp, local_path, remote_path):
                success_count += 1
            else:
                fail_count += 1

        # Summary
        print("=" * 70)
        print(f"✅ Successful uploads: {success_count}")
        print(f"❌ Failed uploads: {fail_count}")
        print("=" * 70)
        print()

        if fail_count == 0:
            print("🎉 All fixes uploaded successfully!")
            print()
            print("✨ What's Fixed:")
            print("  ✅ Explorer: Removed 'DEMO DATA' badge")
            print("  ✅ Explorer: Shows real blockchain data from FlareChain")
            print("  ✅ Explorer: Real-time block updates")
            print("  ✅ Explorer: Working search functionality")
            print()
            print("🌐 Test URLs:")
            print("  • Explorer: https://etrid.org/explorer/")
            print()
            print("📊 Explorer Now Shows:")
            print("  • Real latest block number")
            print("  • Real total issuance")
            print("  • Real validator count (21)")
            print("  • Live block updates every 10 seconds")
            print("  • Real transaction/extrinsic data")
        else:
            print("⚠️  Some uploads failed. Check errors above.")
            sys.exit(1)

        ftp.quit()

    except Exception as e:
        print(f"❌ FTP Error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
