#!/usr/bin/env python3
"""
Upload Files to Correct Locations - PROPERLY
"""

import ftplib
import os
import sys

FTP_HOST = "ftp.etrid.org"
FTP_USER = "u724092535"
FTP_PASS = "Fullashit13!"

# Correct file mappings
FILES_TO_UPLOAD = [
    # Main website index.html
    ("website/index.html", "/domains/etrid.org/public_html/index.html"),

    # Telemetry page files
    ("apps/telemetry/index.html", "/domains/etrid.org/public_html/telemetry/index.html"),
    ("apps/telemetry/app-telemetry-feed.js", "/domains/etrid.org/public_html/telemetry/app-telemetry-feed.js"),
]

def upload_file(ftp, local_path, remote_path):
    """Upload a single file"""
    try:
        with open(local_path, 'rb') as f:
            file_size = os.path.getsize(local_path)
            print(f"📤 Uploading {local_path}")
            print(f"   → {remote_path}")
            print(f"   Size: {file_size:,} bytes")

            ftp.storbinary(f'STOR {remote_path}', f)

            # Verify
            uploaded_size = ftp.size(remote_path)
            if uploaded_size == file_size:
                print(f"   ✅ Success! Verified {uploaded_size:,} bytes\n")
                return True
            else:
                print(f"   ⚠️  Size mismatch: {uploaded_size} != {file_size}\n")
                return False
    except Exception as e:
        print(f"   ❌ Failed: {e}\n")
        return False

def main():
    print("=" * 70)
    print("🚀 PROPER FILE UPLOAD TO ËTRID")
    print("=" * 70)
    print()

    script_dir = os.path.dirname(os.path.abspath(__file__))
    os.chdir(script_dir)

    try:
        ftp = ftplib.FTP(FTP_HOST)
        ftp.login(FTP_USER, FTP_PASS)
        print(f"✅ Connected to {FTP_HOST}\n")

        successful = 0
        failed = 0

        for local_path, remote_path in FILES_TO_UPLOAD:
            if os.path.exists(local_path):
                if upload_file(ftp, local_path, remote_path):
                    successful += 1
                else:
                    failed += 1
            else:
                print(f"❌ File not found: {local_path}\n")
                failed += 1

        print("=" * 70)
        print(f"📊 Upload Summary:")
        print(f"   ✅ Successful: {successful}")
        print(f"   ❌ Failed: {failed}")
        print()

        if failed == 0:
            print("🎉 All files uploaded successfully!")
            print()
            print("🌐 Live URLs:")
            print("   • Main site: https://etrid.org")
            print("   • Telemetry: https://etrid.org/telemetry")
            print()
            print("⚠️  Note: telemetry.etrid.org subdomain may need")
            print("    DNS configuration in Hostinger control panel")
        else:
            print("⚠️  Some uploads failed")
            sys.exit(1)

        print("=" * 70)
        ftp.quit()

    except Exception as e:
        print(f"❌ Error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
