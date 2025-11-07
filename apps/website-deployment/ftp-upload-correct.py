#!/usr/bin/env python3
"""
Upload telemetry files to correct Hostinger location
"""
import ftplib
import os
from datetime import datetime

# FTP credentials
FTP_HOST = "etrid.org"
FTP_USER = "u724092535"
FTP_PASS = "Fullashit13!"

# Local files
LOCAL_DIR = "/Users/macbook/Desktop/etrid/etrid-hostinger-deployment/apps/telemetry"
FILES = ["index.html", "app-telemetry-feed.js"]

# Correct web root path
REMOTE_PATH = "domains/etrid.org/public_html/telemetry"

def upload_files():
    print(f"🔌 Connecting to FTP server: {FTP_HOST}")

    try:
        ftp = ftplib.FTP(FTP_HOST)
        ftp.login(FTP_USER, FTP_PASS)

        print(f"✅ Logged in as {FTP_USER}")

        # Navigate to correct directory
        print(f"\n📂 Navigating to {REMOTE_PATH}...")
        ftp.cwd(REMOTE_PATH)
        print(f"   Now in: {ftp.pwd()}")

        # Backup existing files
        print("\n💾 Backing up existing files...")
        backup_suffix = datetime.now().strftime("%Y%m%d-%H%M%S")

        try:
            # Backup index.html
            ftp.rename('index.html', f'index.html.backup-{backup_suffix}')
            print(f"   ✅ Backed up index.html")
        except Exception as e:
            print(f"   ℹ️  No index.html to backup: {e}")

        try:
            # Backup app.js
            ftp.rename('app.js', f'app.js.backup-{backup_suffix}')
            print(f"   ✅ Backed up app.js")
        except Exception as e:
            print(f"   ℹ️  No app.js to backup: {e}")

        # Upload new files
        print("\n📤 Uploading new files...")
        for filename in FILES:
            local_path = os.path.join(LOCAL_DIR, filename)

            # Determine remote filename
            if filename == "app-telemetry-feed.js":
                remote_filename = "app.js"  # Keep the existing name
            else:
                remote_filename = filename

            print(f"   Uploading {filename} as {remote_filename}...")

            try:
                with open(local_path, 'rb') as f:
                    ftp.storbinary(f'STOR {remote_filename}', f)
                print(f"   ✅ Uploaded {remote_filename}")
            except Exception as e:
                print(f"   ❌ Failed to upload {remote_filename}: {e}")

        # Verify uploads
        print("\n✅ Verifying current files...")
        ftp.retrlines('LIST')

        ftp.quit()
        print("\n🎉 Upload complete!")
        print("🌐 Access at: https://etrid.org/telemetry/")
        print("\n⚠️  Important: The website will try to connect to ws://98.71.91.84/feed")
        print("   Make sure the telemetry server is running!")

    except Exception as e:
        print(f"❌ Error: {e}")
        import traceback
        traceback.print_exc()

if __name__ == '__main__':
    upload_files()
