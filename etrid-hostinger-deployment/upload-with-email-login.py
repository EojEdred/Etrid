#!/usr/bin/env python3
"""Upload fixes using email login credentials"""

import ftplib
import os

# FTP Configuration with email
FTP_HOSTS = ["ftp.etrid.org", "etrid.org", "157.173.214.206"]
FTP_USER = "eojedredbitepubkey1@proton.me"
FTP_PASS = "Fullashit13!"

FILES = [
    {
        'local': '/Users/macbook/Desktop/etrid/etrid-hostinger-deployment /website/whitepaper/viewer-standalone.html',
        'remote': 'public_html/whitepaper/viewer-standalone.html',
        'desc': 'Whitepaper (AOS removed)'
    },
    {
        'local': '/Users/macbook/Desktop/etrid/etrid-hostinger-deployment /apps/telemetry/app.js',
        'remote': 'public_html/telemetry/app.js',
        'desc': 'Telemetry (RPC updated)'
    },
    {
        'local': '/Users/macbook/Desktop/etrid/etrid-hostinger-deployment /apps/explorer/index.html',
        'remote': 'public_html/explorer/index.html',
        'desc': 'Explorer (endpoints updated)'
    }
]

def upload_files(ftp):
    """Upload all files"""
    success = 0

    for file_info in FILES:
        local = file_info['local']
        remote = file_info['remote']
        desc = file_info['desc']

        if not os.path.exists(local):
            print(f"❌ File not found: {local}")
            continue

        size = os.path.getsize(local)
        print(f"\n📤 Uploading: {desc}")
        print(f"   Local:  {local}")
        print(f"   Remote: {remote}")
        print(f"   Size:   {size:,} bytes")

        try:
            # Navigate to directory
            remote_dir = os.path.dirname(remote)
            remote_file = os.path.basename(remote)

            ftp.cwd('/')
            ftp.cwd(remote_dir)
            print(f"   📁 Changed to: {remote_dir}")

            # Upload in binary mode
            with open(local, 'rb') as f:
                ftp.storbinary(f'STOR {remote_file}', f)

            print(f"   ✅ Uploaded successfully!")
            success += 1

        except Exception as e:
            print(f"   ❌ Upload failed: {e}")

    return success

def main():
    print("=" * 70)
    print("🚀 ËTRID Website Fixes - Upload with Email Credentials")
    print("=" * 70)

    for host in FTP_HOSTS:
        print(f"\n🔌 Trying host: {host}")
        print(f"👤 User: {FTP_USER}")

        try:
            # Connect
            ftp = ftplib.FTP(timeout=30)
            ftp.connect(host, 21)
            print(f"✅ Connected to {host}")

            # Login
            ftp.login(FTP_USER, FTP_PASS)
            print(f"✅ Logged in successfully!")

            # Set passive mode
            ftp.set_pasv(True)

            # Show current directory
            pwd = ftp.pwd()
            print(f"📂 Current directory: {pwd}")

            # Upload files
            success = upload_files(ftp)

            # Close
            ftp.quit()

            if success == len(FILES):
                print("\n" + "=" * 70)
                print("🎉 SUCCESS! All files uploaded!")
                print("=" * 70)
                print("\n✅ Fixed files deployed:")
                print("   • Whitepaper viewer (no AOS) - 401 KB")
                print("   • Telemetry app (RPC: ws://98.71.91.84:9944)")
                print("   • Explorer (endpoints: 98.71.91.84:9933/9944)")
                print("\n🔍 Test your site:")
                print("   • https://etrid.org/whitepaper/viewer-standalone.html")
                print("   • https://etrid.org/telemetry/")
                print("   • https://etrid.org/explorer/")
                print("\n💡 Hard refresh browser (Ctrl+Shift+R) to see changes!")
                return True

        except ftplib.error_perm as e:
            print(f"❌ Login failed: {e}")
            continue

        except Exception as e:
            print(f"❌ Connection failed: {e}")
            continue

    print("\n" + "=" * 70)
    print("❌ Could not connect to any FTP host")
    print("=" * 70)
    print("\n💡 Please check:")
    print("   • Is FTP enabled in Hostinger?")
    print("   • Are credentials correct?")
    print("   • Try Hostinger File Manager: hpanel.hostinger.com")

    return False

if __name__ == "__main__":
    main()
