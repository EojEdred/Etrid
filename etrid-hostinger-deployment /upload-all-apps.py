#!/usr/bin/env python3
"""
Upload All ËTRID Apps to Hostinger
Uploads validator, masterchef, wallet, telemetry, governance, explorer, bridge apps
"""

import ftplib
import os
import sys
from pathlib import Path
from getpass import getpass

# Configuration
SCRIPT_DIR = Path(__file__).parent
APPS_DIR = SCRIPT_DIR / "apps"

# Apps to deploy (with remote directory names)
APPS = [
    {"name": "Validator Dashboard", "local": "validator", "remote": "validator"},
    {"name": "MasterChef Dashboard", "local": "masterchef", "remote": "masterchef"},
    {"name": "Wallet", "local": "wallet", "remote": "wallet"},
    {"name": "Telemetry", "local": "telemetry", "remote": "telemetry"},
    {"name": "Governance", "local": "governance", "remote": "governance"},
    {"name": "Explorer", "local": "explorer", "remote": "explorer"},
    {"name": "Bridge", "local": "bridge", "remote": "bridge"},
]

def upload_directory_ftp(ftp, local_path, remote_path):
    """Recursively upload a directory"""
    try:
        ftp.mkd(remote_path)
    except:
        pass  # Directory might already exist

    for item in local_path.iterdir():
        local_item = local_path / item.name
        remote_item = f"{remote_path}/{item.name}"

        if item.is_file():
            # Upload file
            with open(local_item, 'rb') as f:
                ftp.storbinary(f'STOR {remote_item}', f)
            print(f"    ✓ {item.name}")
        elif item.is_dir():
            # Recursively upload directory
            upload_directory_ftp(ftp, local_item, remote_item)

def upload_app_ftp(host, username, password, app_info):
    """Upload a single app"""
    local_path = APPS_DIR / app_info['local']
    remote_dir = f"public_html/{app_info['remote']}"

    if not local_path.exists():
        print(f"✗ {app_info['name']}: Local folder not found at {local_path}")
        return False

    try:
        print(f"\n📤 Uploading {app_info['name']}...")
        print(f"   Local:  {local_path}")
        print(f"   Remote: {remote_dir}")

        ftp = ftplib.FTP(host, timeout=120)
        ftp.login(username, password)

        # Navigate to public_html
        try:
            ftp.cwd('/public_html')
        except:
            try:
                ftp.cwd('public_html')
            except:
                print(f"   ✗ Could not find public_html directory")
                ftp.quit()
                return False

        # Upload the app directory
        upload_directory_ftp(ftp, local_path, app_info['remote'])

        ftp.quit()
        print(f"   ✓ {app_info['name']} uploaded successfully!")
        return True

    except Exception as e:
        print(f"   ✗ Upload failed: {e}")
        return False

def main():
    print("=" * 70)
    print("ËTRID APPS - BULK UPLOAD TO HOSTINGER")
    print("=" * 70)
    print()

    # Check all apps exist
    print("Checking local apps...")
    all_exist = True
    for app in APPS:
        local_path = APPS_DIR / app['local']
        if local_path.exists():
            # Count files
            file_count = sum(1 for _ in local_path.rglob('*') if _.is_file())
            print(f"  ✓ {app['name']}: {file_count} files")
        else:
            print(f"  ✗ Missing: {app['name']}")
            all_exist = False

    if not all_exist:
        print("\n✗ Some apps are missing. Cannot proceed.")
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

    # Ask which apps to upload
    print("Which apps do you want to upload?")
    print("1. All apps (recommended)")
    print("2. Select individual apps")
    choice = input("Choice (1 or 2): ").strip()

    apps_to_upload = APPS.copy()

    if choice == "2":
        print("\nSelect apps to upload (y/n for each):")
        apps_to_upload = []
        for app in APPS:
            answer = input(f"  Upload {app['name']}? (y/n): ").strip().lower()
            if answer == 'y':
                apps_to_upload.append(app)

    if not apps_to_upload:
        print("\n✗ No apps selected.")
        sys.exit(0)

    print()
    print("=" * 70)
    print(f"UPLOADING {len(apps_to_upload)} APPS")
    print("=" * 70)

    # Upload each app
    results = []
    for app in apps_to_upload:
        success = upload_app_ftp(host, username, password, app)
        results.append((app['name'], success))

    print()
    print("=" * 70)
    print("UPLOAD SUMMARY")
    print("=" * 70)
    print()

    all_success = all(success for _, success in results)

    for name, success in results:
        status = "✓" if success else "✗"
        print(f"  {status} {name}")

    print()

    if all_success:
        print("=" * 70)
        print("SUCCESS! 🎉")
        print("=" * 70)
        print()
        print("All apps are now live!")
        print()
        print("Test your apps:")
        for app in apps_to_upload:
            print(f"  • https://etrid.org/{app['remote']}/")
        print()
        print("Remember to clear browser cache (Cmd+Shift+R) if pages don't update")
    else:
        print("=" * 70)
        print("SOME UPLOADS FAILED")
        print("=" * 70)
        print()
        print("Troubleshooting:")
        print("1. Check your FTP credentials are correct")
        print("2. Verify you have write permissions")
        print("3. Check if public_html directory exists")
        print("4. Try uploading failed apps individually with FileZilla")

    print()

if __name__ == "__main__":
    main()
