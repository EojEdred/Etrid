#!/usr/bin/env python3
"""
Upload Network Monitoring Page to Hostinger
Uploads the network monitoring dashboard and updated homepage
"""

import ftplib
import os
import sys
from pathlib import Path
from getpass import getpass

# Configuration
SCRIPT_DIR = Path(__file__).parent
FILES_TO_UPLOAD = [
    {
        'local': SCRIPT_DIR / "website" / "network" / "index.html",
        'remote_dir': "public_html/network",
        'remote_name': "index.html",
        'description': "Network monitoring dashboard page"
    },
    {
        'local': SCRIPT_DIR / "website" / "index.html",
        'remote_dir': "public_html",
        'remote_name': "index.html",
        'description': "Homepage (updated navigation)"
    }
]

def ensure_remote_directory(ftp, remote_dir):
    """Create remote directory if it doesn't exist"""
    parts = remote_dir.strip('/').split('/')
    current = ''

    for part in parts:
        current = f"{current}/{part}" if current else part
        try:
            ftp.cwd(f"/{current}")
        except:
            try:
                ftp.mkd(f"/{current}")
                print(f"  ✓ Created directory: /{current}")
                ftp.cwd(f"/{current}")
            except Exception as e:
                print(f"  ✗ Could not create directory /{current}: {e}")
                return False
    return True

def upload_file_ftp(host, username, password, file_info):
    """Upload a single file via FTP"""
    local_path = file_info['local']
    remote_dir = file_info['remote_dir']
    remote_name = file_info['remote_name']
    description = file_info['description']

    if not local_path.exists():
        print(f"✗ File not found: {local_path}")
        return False

    try:
        print(f"\n📤 Uploading {description}...")
        print(f"   Local:  {local_path.name}")
        print(f"   Remote: {remote_dir}/{remote_name}")

        ftp = ftplib.FTP(host, timeout=60)
        ftp.login(username, password)

        # Navigate to or create remote directory
        try:
            ftp.cwd(f"/{remote_dir}")
        except:
            print(f"   Creating remote directory structure...")
            if not ensure_remote_directory(ftp, remote_dir):
                print(f"   ✗ Failed to create directory")
                ftp.quit()
                return False

        # Upload file in BINARY mode
        local_size = local_path.stat().st_size
        with open(local_path, 'rb') as f:
            ftp.storbinary(f'STOR {remote_name}', f, blocksize=8192)

        # Verify
        try:
            remote_size = ftp.size(remote_name)
            if local_size == remote_size:
                print(f"   ✓ Upload successful ({remote_size:,} bytes)")
                success = True
            else:
                print(f"   ⚠️  Size mismatch: local={local_size:,}, remote={remote_size:,}")
                success = False
        except:
            print(f"   ✓ Uploaded (size verification not available)")
            success = True

        ftp.quit()
        return success

    except Exception as e:
        print(f"   ✗ Upload failed: {e}")
        return False

def main():
    print("=" * 70)
    print("NETWORK MONITORING PAGE - FTP UPLOAD")
    print("=" * 70)
    print()

    # Check all files exist
    print("Checking local files...")
    all_exist = True
    for file_info in FILES_TO_UPLOAD:
        if file_info['local'].exists():
            size = file_info['local'].stat().st_size
            print(f"  ✓ {file_info['description']}: {size:,} bytes")
        else:
            print(f"  ✗ Missing: {file_info['local']}")
            all_exist = False

    if not all_exist:
        print("\n✗ Some files are missing. Cannot proceed.")
        sys.exit(1)

    print()

    # Get FTP credentials
    print("Enter your Hostinger FTP credentials:")
    print("(Find these in: Hostinger → Hosting → Manage → FTP Accounts)")
    print()

    host = input("FTP Host (e.g., ftp.etrid.org): ").strip()
    if not host:
        print("✗ Host is required")
        sys.exit(1)

    host = host.replace('ftp://', '').replace('ftps://', '')

    username = input("FTP Username: ").strip()
    if not username:
        print("✗ Username is required")
        sys.exit(1)

    password = getpass("FTP Password (hidden): ")
    if not password:
        print("✗ Password is required")
        sys.exit(1)

    print()
    print("=" * 70)
    print("UPLOADING FILES")
    print("=" * 70)

    # Upload each file
    results = []
    for file_info in FILES_TO_UPLOAD:
        success = upload_file_ftp(host, username, password, file_info)
        results.append((file_info['description'], success))

    print()
    print("=" * 70)
    print("UPLOAD SUMMARY")
    print("=" * 70)
    print()

    all_success = all(success for _, success in results)

    for description, success in results:
        status = "✓" if success else "✗"
        print(f"  {status} {description}")

    print()

    if all_success:
        print("=" * 70)
        print("SUCCESS! 🎉")
        print("=" * 70)
        print()
        print("Your network monitoring page is now live!")
        print()
        print("Next steps:")
        print("1. Visit: https://etrid.org/")
        print("   → Click 'Network' in the navigation menu")
        print()
        print("2. Or directly: https://etrid.org/network/")
        print()
        print("3. Clear browser cache (Cmd+Shift+R) if page doesn't update")
        print()
        print("Your Grafana dashboard should be embedded and auto-refreshing!")
    else:
        print("=" * 70)
        print("SOME UPLOADS FAILED")
        print("=" * 70)
        print()
        print("Troubleshooting:")
        print("1. Check your FTP credentials are correct")
        print("2. Verify you have write permissions")
        print("3. Try uploading manually with FileZilla")
        print("4. Check NETWORK_MONITORING_READY.md for detailed instructions")

    print()

if __name__ == "__main__":
    main()
