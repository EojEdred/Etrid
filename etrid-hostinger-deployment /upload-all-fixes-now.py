#!/usr/bin/env python3
"""
Upload all fixes to Hostinger via FTP
- Whitepaper viewer (no AOS)
- Telemetry app with real RPC
- Explorer with real endpoints
"""

import ftplib
import os
import sys

# FTP Configuration
FTP_HOST = "157.173.214.206"
FTP_USER = "u724092535"
FTP_PASS = "Fullashit13!"

# Files to upload
UPLOADS = [
    {
        'local': '/Users/macbook/Desktop/etrid/etrid-hostinger-deployment /website/whitepaper/viewer-standalone-no-aos.html',
        'remote': 'public_html/whitepaper/viewer-standalone.html',
        'description': 'Whitepaper Viewer (CSP fix)'
    },
    {
        'local': '/Users/macbook/Desktop/etrid/etrid-hostinger-deployment /apps/telemetry/app.js',
        'remote': 'public_html/telemetry/app.js',
        'description': 'Telemetry App (RPC endpoint fix)'
    },
    {
        'local': '/Users/macbook/Desktop/etrid/etrid-hostinger-deployment /apps/explorer/index.html',
        'remote': 'public_html/explorer/index.html',
        'description': 'Explorer (RPC endpoint display)'
    }
]

def upload_file(ftp, local_path, remote_path, description):
    """Upload a single file via FTP"""
    try:
        # Check if file exists
        if not os.path.exists(local_path):
            print(f"❌ File not found: {local_path}")
            return False

        file_size = os.path.getsize(local_path)
        print(f"\n📤 Uploading: {description}")
        print(f"   Local:  {local_path}")
        print(f"   Remote: {remote_path}")
        print(f"   Size:   {file_size:,} bytes ({file_size/1024:.1f} KB)")

        # Navigate to remote directory
        remote_dir = os.path.dirname(remote_path)
        remote_filename = os.path.basename(remote_path)

        # Change to remote directory
        try:
            ftp.cwd('/')
            ftp.cwd(remote_dir)
            print(f"   📁 Changed to: /{remote_dir}")
        except Exception as e:
            print(f"   ⚠️  Warning: Could not change to {remote_dir}: {e}")
            print(f"   Attempting to upload anyway...")

        # Upload file in binary mode
        with open(local_path, 'rb') as f:
            ftp.storbinary(f'STOR {remote_filename}', f)

        print(f"   ✅ Uploaded successfully!")
        return True

    except Exception as e:
        print(f"   ❌ Upload failed: {e}")
        return False

def main():
    print("=" * 60)
    print("🚀 ËTRID Website Fixes - FTP Upload")
    print("=" * 60)

    try:
        # Connect to FTP
        print(f"\n🔌 Connecting to {FTP_HOST}...")
        ftp = ftplib.FTP(FTP_HOST, timeout=30)

        # Login
        print(f"🔐 Logging in as {FTP_USER}...")
        ftp.login(FTP_USER, FTP_PASS)

        # Set binary mode
        ftp.set_pasv(True)

        print(f"✅ Connected successfully!")
        print(f"📂 Current directory: {ftp.pwd()}")

        # Upload each file
        success_count = 0
        failed_count = 0

        for upload in UPLOADS:
            if upload_file(ftp, upload['local'], upload['remote'], upload['description']):
                success_count += 1
            else:
                failed_count += 1

        # Close connection
        ftp.quit()

        # Summary
        print("\n" + "=" * 60)
        print("📊 Upload Summary")
        print("=" * 60)
        print(f"✅ Successful: {success_count}/{len(UPLOADS)}")
        print(f"❌ Failed: {failed_count}/{len(UPLOADS)}")

        if success_count == len(UPLOADS):
            print("\n🎉 All fixes uploaded successfully!")
            print("\n🔍 Test URLs:")
            print("   • https://etrid.org/whitepaper/viewer-standalone.html")
            print("   • https://etrid.org/telemetry/")
            print("   • https://etrid.org/explorer/")
        else:
            print("\n⚠️  Some uploads failed. Check errors above.")
            sys.exit(1)

    except ftplib.error_perm as e:
        print(f"\n❌ FTP Permission Error: {e}")
        print("\n💡 Possible issues:")
        print("   • Incorrect username or password")
        print("   • Account locked or suspended")
        print("   • FTP access disabled in Hostinger")
        sys.exit(1)

    except Exception as e:
        print(f"\n❌ Connection failed: {e}")
        print("\n💡 Try:")
        print("   • Check FTP credentials")
        print("   • Verify FTP is enabled in Hostinger")
        print("   • Use Hostinger File Manager as alternative")
        sys.exit(1)

if __name__ == "__main__":
    main()
