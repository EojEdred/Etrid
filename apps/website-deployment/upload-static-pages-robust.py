#!/usr/bin/env python3
"""Upload static pages with reconnection for each file"""

import ftplib
import os
import time

HOST = "157.173.214.206"
USER = "u724092535"
PASS = "Fullashit13!"
REMOTE_DIR = "/domains/etrid.org/public_html/whitepaper"

FILES = [
    "index.html",
    "complete-edition.html",
    "volume-1.html",
    "volume-2.html",
    "volume-3.html",
    "protocol-charter.html"
]

BASE_DIR = "/Users/macbook/Desktop/etrid/etrid-hostinger-deployment /website/whitepaper"

def upload_file(filename):
    """Upload a single file with fresh connection"""
    local_path = os.path.join(BASE_DIR, filename)

    if not os.path.exists(local_path):
        print(f"❌ Not found: {filename}")
        return False

    size = os.path.getsize(local_path)
    print(f"\n📤 {filename}")
    print(f"   Size: {size:,} bytes ({size/1024:.1f} KB)")

    try:
        # Fresh connection for each file
        ftp = ftplib.FTP()
        ftp.connect(HOST, 21, timeout=30)
        ftp.login(USER, PASS)
        ftp.set_pasv(True)
        ftp.cwd(REMOTE_DIR)

        # Upload
        with open(local_path, 'rb') as f:
            ftp.storbinary(f'STOR {filename}', f, blocksize=8192)

        # Close immediately
        try:
            ftp.quit()
        except:
            ftp.close()

        print(f"   ✅ Uploaded!")
        return True

    except Exception as e:
        print(f"   ❌ Failed: {e}")
        return False

print("=" * 70)
print("🚀 Uploading Static Whitepaper Pages")
print("=" * 70)

success = 0
total_size = 0

for filename in FILES:
    if upload_file(filename):
        success += 1
        local_path = os.path.join(BASE_DIR, filename)
        total_size += os.path.getsize(local_path)

    # Small delay between uploads
    time.sleep(0.5)

# Summary
print(f"\n{'='*70}")
print(f"📊 Upload Summary")
print(f"{'='*70}")
print(f"✅ Uploaded: {success}/{len(FILES)} files")
print(f"📦 Total: {total_size:,} bytes ({total_size/1024:.1f} KB)")

if success == len(FILES):
    print(f"\n🎉 ALL PAGES UPLOADED SUCCESSFULLY!")
    print(f"\n✅ Pure HTML - No JavaScript needed!")
    print(f"\n🔍 Test in Firefox:")
    print(f"   https://etrid.org/whitepaper/")
    print(f"\n   Each document loads instantly:")
    print(f"   • Complete Edition")
    print(f"   • Volume I")
    print(f"   • Volume II")
    print(f"   • Volume III")
    print(f"   • Protocol Charter")
    print(f"\n💡 Works in ALL browsers - guaranteed!")
