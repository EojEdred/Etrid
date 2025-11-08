#!/usr/bin/env python3
"""
Upload DeFi Hub to Hostinger (etrid.org)
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
    # Updated main site with DeFi Hub navigation
    ("website/index.html", "domains/etrid.org/public_html/index.html"),

    # DeFi Hub directory
    ("website/defi-hub/index.html", "domains/etrid.org/public_html/defi-hub/index.html"),
]

def create_directory(ftp, path):
    """Create directory if it doesn't exist"""
    try:
        ftp.mkd(path)
        print(f"📁 Created directory: {path}")
    except ftplib.error_perm:
        # Directory already exists
        pass

def upload_file(ftp, local_path, remote_path):
    """Upload a single file via FTP"""
    try:
        # Ensure directory exists
        remote_dir = os.path.dirname(remote_path)
        if remote_dir:
            try:
                ftp.cwd('/')
                for part in remote_dir.split('/'):
                    if part:
                        try:
                            ftp.cwd(part)
                        except:
                            ftp.mkd(part)
                            ftp.cwd(part)
            except Exception as e:
                print(f"   ⚠️  Directory creation: {e}")

        ftp.cwd('/')

        with open(local_path, 'rb') as f:
            file_size = os.path.getsize(local_path)
            print(f"📤 Uploading {local_path}")
            print(f"   Size: {file_size:,} bytes")
            print(f"   → {remote_path}")
            ftp.storbinary(f'STOR {remote_path}', f)
            print(f"   ✅ Success!\n")
            return True
    except Exception as e:
        print(f"   ❌ Failed: {e}\n")
        return False

def main():
    print("=" * 80)
    print("🏦 ËTRID DeFi Hub Deployment")
    print("=" * 80)
    print()

    # Check if files exist
    print("🔍 Checking files...")
    for local_path, _ in FILES_TO_UPLOAD:
        if not os.path.exists(local_path):
            print(f"❌ File not found: {local_path}")
            sys.exit(1)
        else:
            print(f"✅ Found: {local_path}")
    print()

    try:
        # Connect to FTP
        print(f"🔌 Connecting to {FTP_HOST}...")
        ftp = ftplib.FTP(FTP_HOST)
        ftp.login(FTP_USER, FTP_PASS)
        print(f"✅ Connected as {FTP_USER}\n")

        # Upload files
        success_count = 0
        for local_path, remote_path in FILES_TO_UPLOAD:
            if upload_file(ftp, local_path, remote_path):
                success_count += 1

        # Summary
        print("=" * 80)
        if success_count == len(FILES_TO_UPLOAD):
            print("🎉 DeFi Hub deployed successfully!")
            print()
            print("✨ What's New:")
            print("  ✅ DeFi Hub live at /defi-hub/")
            print("  ✅ Updated navigation with 'Apps' dropdown")
            print("  ✅ Operations Center with 9 functional links")
            print("  ✅ Validator network monitoring")
            print("  ✅ AI-powered analytics")
            print("  ✅ Configuration modal for editing validators")
            print()
            print("🌐 Access Now:")
            print("  • Main Site: https://etrid.org")
            print("  • DeFi Hub: https://etrid.org/defi-hub/")
            print()
            print("📊 Features:")
            print("  • Real-time validator health monitoring")
            print("  • Operations Center with all app links")
            print("  • Editable validator configuration")
            print("  • AI recommendations and analytics")
        else:
            print(f"⚠️  Partial upload: {success_count}/{len(FILES_TO_UPLOAD)} files uploaded")
            print("Check errors above for details.")
            sys.exit(1)

        print("=" * 80)
        ftp.quit()

    except Exception as e:
        print(f"❌ FTP Error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
