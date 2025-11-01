#!/usr/bin/env python3
"""
Automated FTP Upload for Ivory Papers Viewer
Uploads viewer-standalone.html to Hostinger via FTP in BINARY mode
"""

import ftplib
import os
import sys
from pathlib import Path
from getpass import getpass

# Configuration
SCRIPT_DIR = Path(__file__).parent
LOCAL_FILE = SCRIPT_DIR / "whitepaper 2" / "viewer-standalone.html"
REMOTE_DIR = "public_html/whitepaper"  # Adjust if your path is different
REMOTE_FILENAME = "viewer-standalone.html"

# Check file integrity helper
CHECK_FILE = SCRIPT_DIR / "website" / "whitepaper" / "check-file-integrity.html"

def upload_file_ftp(host, username, password, local_path, remote_dir, remote_filename):
    """Upload file via FTP in BINARY mode"""
    try:
        print(f"Connecting to {host}...")
        ftp = ftplib.FTP(host, timeout=60)

        print(f"Logging in as {username}...")
        ftp.login(username, password)

        print(f"Connected successfully!")
        print(f"Current directory: {ftp.pwd()}")

        # Navigate to remote directory
        try:
            print(f"\nChanging to directory: {remote_dir}")
            ftp.cwd(remote_dir)
        except ftplib.error_perm:
            print(f"Directory {remote_dir} not found. Trying alternative paths...")
            # Try alternative paths
            alternatives = [
                "public_html/whitepaper",
                "httpdocs/whitepaper",
                "www/whitepaper",
                "whitepaper"
            ]
            found = False
            for alt in alternatives:
                try:
                    ftp.cwd(f"/{alt}")
                    print(f"✓ Found directory: /{alt}")
                    found = True
                    break
                except:
                    continue

            if not found:
                print("\nError: Could not find whitepaper directory.")
                print("Available directories:")
                ftp.retrlines('LIST')
                ftp.quit()
                return False

        # Get file size
        local_size = local_path.stat().st_size
        print(f"\nLocal file: {local_path.name}")
        print(f"Local size: {local_size:,} bytes ({local_size / 1024:.1f} KB)")

        # Upload in BINARY mode (critical!)
        print(f"\nUploading {remote_filename} in BINARY mode...")
        with open(local_path, 'rb') as f:
            ftp.storbinary(f'STOR {remote_filename}', f, blocksize=8192)

        print(f"✓ Upload complete!")

        # Verify file size on server
        try:
            remote_size = ftp.size(remote_filename)
            print(f"\nVerification:")
            print(f"  Local size:  {local_size:,} bytes")
            print(f"  Remote size: {remote_size:,} bytes")

            if local_size == remote_size:
                print(f"  ✓ File sizes match! Upload successful!")
                success = True
            else:
                print(f"  ✗ WARNING: File sizes don't match!")
                print(f"  ✗ Upload may have been incomplete.")
                success = False
        except:
            print(f"  Note: Could not verify remote file size (not all FTP servers support this)")
            success = True  # Assume success if we can't verify

        # Upload check file if it exists
        if CHECK_FILE.exists():
            print(f"\nUploading check-file-integrity.html...")
            with open(CHECK_FILE, 'rb') as f:
                ftp.storbinary(f'STOR check-file-integrity.html', f)
            print(f"✓ Check file uploaded")

        ftp.quit()
        return success

    except ftplib.error_perm as e:
        print(f"\n✗ FTP Permission Error: {e}")
        print("  Check your username and password.")
        return False
    except ftplib.error_temp as e:
        print(f"\n✗ FTP Temporary Error: {e}")
        print("  Server may be busy, try again in a moment.")
        return False
    except Exception as e:
        print(f"\n✗ Error: {e}")
        return False

def main():
    print("=" * 70)
    print("IVORY PAPERS VIEWER - FTP UPLOAD")
    print("=" * 70)
    print()

    # Check if local file exists
    if not LOCAL_FILE.exists():
        print(f"✗ Error: Local file not found: {LOCAL_FILE}")
        print(f"\nRun update-ivory-papers-viewer.py first to generate the file.")
        sys.exit(1)

    file_size = LOCAL_FILE.stat().st_size
    print(f"Local file found: {LOCAL_FILE.name}")
    print(f"File size: {file_size:,} bytes ({file_size / 1024:.1f} KB)")
    print()

    # Get FTP credentials
    print("Enter your Hostinger FTP credentials:")
    print("(Find these in: Hostinger → Hosting → Manage → FTP Accounts)")
    print()

    host = input("FTP Host (e.g., ftp.etrid.org): ").strip()
    if not host:
        print("✗ Host is required")
        sys.exit(1)

    # Remove ftp:// prefix if user included it
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
    print("Starting upload...")
    print("-" * 70)

    # Upload
    success = upload_file_ftp(host, username, password, LOCAL_FILE, REMOTE_DIR, REMOTE_FILENAME)

    print("-" * 70)
    print()

    if success:
        print("=" * 70)
        print("SUCCESS! 🎉")
        print("=" * 70)
        print()
        print("Next steps:")
        print("1. Clear your browser cache (Cmd+Shift+R on Mac)")
        print("2. Visit: https://etrid.org/whitepaper/viewer-standalone.html")
        print("3. Verify with: https://etrid.org/whitepaper/check-file-integrity.html")
        print()
        print("Your Ivory Papers viewer should now be working!")
    else:
        print("=" * 70)
        print("UPLOAD FAILED")
        print("=" * 70)
        print()
        print("Troubleshooting:")
        print("1. Double-check your FTP credentials")
        print("2. Make sure your FTP account has write access")
        print("3. Try using FileZilla manually (see FIX_NOW.md)")
        print("4. Contact Hostinger support if issues persist")

    print()

if __name__ == "__main__":
    main()
