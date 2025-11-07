#!/usr/bin/env python3
"""
Verify and Fix ËTRID Deployments
Checks what's actually on the server and fixes issues
"""

import ftplib
import os
import sys

# FTP Configuration
FTP_HOST = "ftp.etrid.org"
FTP_USER = "u724092535"
FTP_PASS = "Fullashit13!"

def list_remote_files(ftp, path):
    """List files in a remote directory"""
    try:
        files = []
        ftp.cwd(path)
        ftp.retrlines('LIST', files.append)
        return files
    except Exception as e:
        return None

def check_file_exists(ftp, path):
    """Check if a file exists on the server"""
    try:
        ftp.size(path)
        return True
    except:
        return False

def main():
    print("=" * 70)
    print("🔍 ËTRID Deployment Verification and Fix")
    print("=" * 70)
    print()

    try:
        # Connect to FTP
        print(f"🔌 Connecting to {FTP_HOST}...")
        ftp = ftplib.FTP(FTP_HOST)
        ftp.login(FTP_USER, FTP_PASS)
        print(f"✅ Connected as {FTP_USER}\n")

        # Check main site
        print("📋 Checking Main Site (etrid.org):")
        print("-" * 70)

        main_index = "domains/etrid.org/public_html/index.html"
        if check_file_exists(ftp, main_index):
            size = ftp.size(main_index)
            print(f"  ✅ index.html exists ({size:,} bytes)")
        else:
            print(f"  ❌ index.html NOT FOUND")
        print()

        # Check telemetry path
        print("📋 Checking Telemetry Path (etrid.org/telemetry):")
        print("-" * 70)

        telemetry_path = "domains/etrid.org/public_html/telemetry"
        try:
            files = list_remote_files(ftp, telemetry_path)
            if files:
                print(f"  ✅ Directory exists with {len(files)} items")
                for f in files:
                    print(f"     {f}")
            else:
                print(f"  ❌ Directory NOT FOUND or EMPTY")
        except Exception as e:
            print(f"  ❌ Error: {e}")
        print()

        # Check telemetry subdomain
        print("📋 Checking Telemetry Subdomain (telemetry.etrid.org):")
        print("-" * 70)

        subdomain_path = "domains/telemetry.etrid.org/public_html"
        try:
            files = list_remote_files(ftp, subdomain_path)
            if files:
                print(f"  ✅ Directory exists with {len(files)} items")
                for f in files:
                    print(f"     {f}")
            else:
                print(f"  ❌ Directory NOT FOUND or EMPTY")
        except Exception as e:
            print(f"  ❌ Error: {e}")
        print()

        # List all domains
        print("📋 Listing All Domains:")
        print("-" * 70)
        try:
            domains = list_remote_files(ftp, "domains")
            if domains:
                for d in domains:
                    print(f"  {d}")
            else:
                print("  ❌ No domains found")
        except Exception as e:
            print(f"  ❌ Error: {e}")

        print("=" * 70)
        ftp.quit()

    except Exception as e:
        print(f"❌ FTP Error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
