#!/usr/bin/env python3
"""
Upload telemetry files to Hostinger via FTP
"""
import ftplib
import os

# FTP credentials
FTP_HOST = "etrid.org"
FTP_USER = "u724092535"
FTP_PASS = "Fullashit13!"

# Local files
LOCAL_DIR = "/Users/macbook/Desktop/etrid/etrid-hostinger-deployment/apps/telemetry"
FILES = ["index.html", "app-telemetry-feed.js"]

def upload_files():
    print(f"🔌 Connecting to FTP server: {FTP_HOST}")

    try:
        ftp = ftplib.FTP(FTP_HOST)
        ftp.login(FTP_USER, FTP_PASS)

        print(f"✅ Logged in as {FTP_USER}")
        print(f"📂 Current directory: {ftp.pwd()}")

        # List current directory to see structure
        print("\n📋 Root directory contents:")
        ftp.retrlines('LIST')

        # Try to create telemetry directory
        print("\n📁 Creating telemetry directory...")
        try:
            ftp.mkd('telemetry')
            print("   Created telemetry directory")
        except ftplib.error_perm as e:
            if "exists" in str(e).lower():
                print("   Directory already exists")
            else:
                print(f"   Cannot create directory: {e}")

        # Change to telemetry directory
        print("\n📂 Changing to telemetry directory...")
        try:
            ftp.cwd('telemetry')
            print(f"   Now in: {ftp.pwd()}")
        except Exception as e:
            print(f"   ❌ Cannot change to telemetry: {e}")
            return

        # Upload files
        for filename in FILES:
            local_path = os.path.join(LOCAL_DIR, filename)
            print(f"\n📤 Uploading {filename}...")

            try:
                with open(local_path, 'rb') as f:
                    ftp.storbinary(f'STOR {filename}', f)
                print(f"   ✅ Uploaded {filename}")
            except Exception as e:
                print(f"   ❌ Failed to upload {filename}: {e}")

        # Verify uploads
        print("\n✅ Verifying uploads...")
        ftp.retrlines('LIST')

        ftp.quit()
        print("\n🎉 Upload complete!")
        print("🌐 Access at: http://etrid.org/telemetry/")

    except Exception as e:
        print(f"❌ Error: {e}")
        import traceback
        traceback.print_exc()

if __name__ == '__main__':
    upload_files()
