#!/usr/bin/env python3
"""Try all possible FTP connection methods"""

import ftplib
import os

# Try multiple hosts
HOSTS = [
    "ftp.etrid.org",
    "etrid.org",
    "157.173.214.206",
]

USER = "u724092535"
PASS = "Fullashit13!"

FILES = [
    {
        'local': '/Users/macbook/Desktop/etrid/etrid-hostinger-deployment /website/whitepaper/viewer-standalone.html',
        'remote': 'public_html/whitepaper/viewer-standalone.html'
    },
    {
        'local': '/Users/macbook/Desktop/etrid/etrid-hostinger-deployment /apps/telemetry/app.js',
        'remote': 'public_html/telemetry/app.js'
    },
    {
        'local': '/Users/macbook/Desktop/etrid/etrid-hostinger-deployment /apps/explorer/index.html',
        'remote': 'public_html/explorer/index.html'
    }
]

def try_upload(host):
    """Try to upload using this host"""
    print(f"\n{'='*60}")
    print(f"🔌 Trying host: {host}")
    print(f"{'='*60}")

    try:
        # Connect
        print(f"Connecting...")
        ftp = ftplib.FTP(timeout=15)
        ftp.connect(host, 21)
        print(f"✅ Connected to {host}")

        # Login
        print(f"Logging in as {USER}...")
        ftp.login(USER, PASS)
        print(f"✅ Logged in successfully!")

        # Set passive mode
        ftp.set_pasv(True)

        # Get current directory
        pwd = ftp.pwd()
        print(f"📂 Current directory: {pwd}")

        # List files
        print(f"📂 Directory listing:")
        try:
            files = ftp.nlst()
            for f in files[:10]:
                print(f"   - {f}")
        except:
            pass

        # Try to upload files
        success_count = 0
        for file_info in FILES:
            local_path = file_info['local']
            remote_path = file_info['remote']

            if not os.path.exists(local_path):
                print(f"❌ Local file not found: {local_path}")
                continue

            print(f"\n📤 Uploading: {os.path.basename(local_path)}")
            print(f"   To: {remote_path}")

            try:
                # Navigate to directory
                remote_dir = os.path.dirname(remote_path)
                remote_file = os.path.basename(remote_path)

                try:
                    ftp.cwd('/')
                    ftp.cwd(remote_dir)
                except Exception as e:
                    print(f"   ⚠️  Could not change to {remote_dir}: {e}")
                    continue

                # Upload
                with open(local_path, 'rb') as f:
                    ftp.storbinary(f'STOR {remote_file}', f)

                print(f"   ✅ Uploaded!")
                success_count += 1

            except Exception as e:
                print(f"   ❌ Upload failed: {e}")

        ftp.quit()

        if success_count == len(FILES):
            print(f"\n🎉 SUCCESS! All {success_count} files uploaded to {host}!")
            return True
        else:
            print(f"\n⚠️  Partial success: {success_count}/{len(FILES)} files uploaded")
            return False

    except ftplib.error_perm as e:
        print(f"❌ Permission error: {e}")
        return False
    except Exception as e:
        print(f"❌ Connection failed: {e}")
        return False

def main():
    print("🚀 ËTRID Website Fixes - Multi-Host FTP Upload")
    print("=" * 60)

    for host in HOSTS:
        if try_upload(host):
            print("\n" + "=" * 60)
            print("✅ DEPLOYMENT SUCCESSFUL!")
            print("=" * 60)
            print("\n🔍 Test your fixes:")
            print("   • https://etrid.org/whitepaper/viewer-standalone.html")
            print("   • https://etrid.org/telemetry/")
            print("   • https://etrid.org/explorer/")
            return

    print("\n" + "=" * 60)
    print("❌ ALL CONNECTION ATTEMPTS FAILED")
    print("=" * 60)
    print("\n💡 Manual upload required:")
    print("   1. Login to: hpanel.hostinger.com")
    print("   2. Go to: Files → File Manager")
    print("   3. Upload the 3 fixed files")
    print("\nSee FIXES_PUSHED_TO_GITHUB.md for detailed instructions.")

if __name__ == "__main__":
    main()
