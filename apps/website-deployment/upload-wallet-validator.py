#!/usr/bin/env python3
"""
Upload Wallet and Validator Apps to Hostinger
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
    ("apps/wallet/index.html", "domains/etrid.org/public_html/wallet/index.html"),
    ("apps/validator/index.html", "domains/etrid.org/public_html/validator/index.html"),
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
    print("ËTRID Wallet & Validator Dashboard Upload")
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
            print("🎉 All files uploaded successfully!")
            print()
            print("Live URLs (once subdomains configured):")
            print("  • Wallet: https://wallet.etrid.org")
            print("  • Validator: https://validator.etrid.org")
            print()
            print("Temporary URLs (work immediately):")
            print("  • Wallet: https://etrid.org/wallet/")
            print("  • Validator: https://etrid.org/validator/")
            print()
            print("✨ What's New:")
            print("  ✅ Wallet app with Polkadot.js integration")
            print("  ✅ Send/Receive functionality UI")
            print("  ✅ Real balance display from blockchain")
            print("  ✅ Validator dashboard with performance metrics")
            print("  ✅ Validator statistics and rewards tracking")
            print("  ✅ Both apps have working wallet connections")
            print()
            print("⚠️  IMPORTANT:")
            print("  Subdomains (wallet.etrid.org, validator.etrid.org)")
            print("  need to be configured in Hostinger hPanel.")
            print()
            print("  See SUBDOMAIN_CONFIGURATION.md for step-by-step guide.")
        else:
            print("⚠️  Some uploads failed. Check errors above.")
            sys.exit(1)

        ftp.quit()

    except Exception as e:
        print(f"❌ FTP Error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
