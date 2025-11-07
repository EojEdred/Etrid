#!/usr/bin/env python3
"""
Setup telemetry.etrid.org subdomain
"""

import ftplib
import os

FTP_HOST = "ftp.etrid.org"
FTP_USER = "u724092535"
FTP_PASS = "Fullashit13!"

def create_remote_dir(ftp, path):
    """Create directory if it doesn't exist"""
    dirs = path.split('/')
    current = ""
    for d in dirs:
        if d:
            current += "/" + d
            try:
                ftp.mkd(current)
                print(f"   ✅ Created: {current}")
            except:
                pass  # Already exists

try:
    ftp = ftplib.FTP(FTP_HOST)
    ftp.login(FTP_USER, FTP_PASS)

    print("=" * 70)
    print("🌐 Setting up telemetry.etrid.org subdomain")
    print("=" * 70)
    print()

    # Check if subdomain directory exists
    subdomain_root = "/domains/telemetry.etrid.org/public_html"

    print("📁 Creating subdomain directory structure...")
    create_remote_dir(ftp, subdomain_root)

    # Upload telemetry files to subdomain
    print("\n📤 Uploading telemetry files to subdomain...")

    files_to_upload = [
        ("apps/telemetry/index.html", f"{subdomain_root}/index.html"),
        ("apps/telemetry/app-telemetry-feed.js", f"{subdomain_root}/app-telemetry-feed.js"),
    ]

    for local_path, remote_path in files_to_upload:
        try:
            with open(local_path, 'rb') as f:
                file_size = os.path.getsize(local_path)
                print(f"\n   Uploading {local_path}")
                print(f"   → {remote_path}")
                ftp.storbinary(f'STOR {remote_path}', f)

                uploaded_size = ftp.size(remote_path)
                if uploaded_size == file_size:
                    print(f"   ✅ Success ({uploaded_size:,} bytes)")
                else:
                    print(f"   ⚠️  Size mismatch")
        except Exception as e:
            print(f"   ❌ Failed: {e}")

    print("\n" + "=" * 70)
    print("✅ Subdomain setup complete!")
    print()
    print("🌐 URLs to test:")
    print("   • https://etrid.org")
    print("   • https://etrid.org/telemetry")
    print("   • https://telemetry.etrid.org")
    print()
    print("⚠️  Note: If telemetry.etrid.org doesn't work, you may need to:")
    print("   1. Add the subdomain in Hostinger control panel")
    print("   2. Point it to /domains/telemetry.etrid.org/public_html")
    print("   3. Wait for DNS propagation (5-30 minutes)")
    print("=" * 70)

    ftp.quit()

except Exception as e:
    print(f"❌ Error: {e}")
