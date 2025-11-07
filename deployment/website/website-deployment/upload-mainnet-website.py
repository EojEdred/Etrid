#!/usr/bin/env python3
"""
Upload Updated Mainnet Website to Hostinger
"""

import ftplib
import os
import sys

# FTP Configuration
FTP_HOST = "ftp.etrid.org"
FTP_USER = "u724092535"
FTP_PASS = "Fullashit13!"

# File to upload
FILE_TO_UPLOAD = ("website/index.html", "domains/etrid.org/public_html/index.html")

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
    print("ËTRID Mainnet Website Update")
    print("=" * 70)
    print()

    try:
        # Connect to FTP
        print(f"🔌 Connecting to {FTP_HOST}...")
        ftp = ftplib.FTP(FTP_HOST)
        ftp.login(FTP_USER, FTP_PASS)
        print(f"✅ Connected as {FTP_USER}\n")

        # Upload file
        local_path, remote_path = FILE_TO_UPLOAD
        success = upload_file(ftp, local_path, remote_path)

        # Summary
        print("=" * 70)
        if success:
            print("🎉 Website updated successfully!")
            print()
            print("✨ What's New:")
            print("  ✅ Mainnet launch announcement banner")
            print("  ✅ '🟢 Mainnet Live' status in hero section")
            print("  ✅ Updated stats showing 21 active validators")
            print("  ✅ 'LIVE' mainnet status indicator")
            print()
            print("🌐 Live at: https://etrid.org")
            print()
            print("📊 Mainnet Info:")
            print("  • 21 validators running")
            print("  • FlareChain mainnet operational")
            print("  • Telemetry: https://telemetry.etrid.org")
        else:
            print("⚠️  Upload failed. Check errors above.")
            sys.exit(1)

        print("=" * 70)
        ftp.quit()

    except Exception as e:
        print(f"❌ FTP Error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
