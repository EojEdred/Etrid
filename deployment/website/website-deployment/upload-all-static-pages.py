#!/usr/bin/env python3
"""Upload all static whitepaper pages"""

import ftplib
import os

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

print("=" * 70)
print("🚀 Uploading Static Whitepaper Pages")
print("=" * 70)

# Connect
print(f"\n🔌 Connecting to {HOST}...")
ftp = ftplib.FTP()
ftp.connect(HOST, 21, timeout=30)
ftp.login(USER, PASS)
ftp.set_pasv(True)
print(f"✅ Connected!")

# Navigate to whitepaper directory
ftp.cwd(REMOTE_DIR)
print(f"📁 Directory: {REMOTE_DIR}")

# Upload each file
success = 0
total_size = 0

for filename in FILES:
    local_path = os.path.join(BASE_DIR, filename)

    if not os.path.exists(local_path):
        print(f"\n❌ Not found: {filename}")
        continue

    size = os.path.getsize(local_path)
    total_size += size

    print(f"\n📤 Uploading: {filename}")
    print(f"   Size: {size:,} bytes ({size/1024:.1f} KB)")

    try:
        with open(local_path, 'rb') as f:
            ftp.storbinary(f'STOR {filename}', f, blocksize=8192)
        print(f"   ✅ Uploaded!")
        success += 1
    except Exception as e:
        print(f"   ❌ Failed: {e}")

ftp.quit()

# Summary
print(f"\n{'='*70}")
print(f"📊 Upload Summary")
print(f"{'='*70}")
print(f"✅ Uploaded: {success}/{len(FILES)} files")
print(f"📦 Total: {total_size:,} bytes ({total_size/1024:.1f} KB)")

if success == len(FILES):
    print(f"\n🎉 ALL PAGES UPLOADED SUCCESSFULLY!")
    print(f"\n✅ Each document is now a separate static HTML file:")
    print(f"   • https://etrid.org/whitepaper/index.html (main page)")
    print(f"   • https://etrid.org/whitepaper/complete-edition.html")
    print(f"   • https://etrid.org/whitepaper/volume-1.html")
    print(f"   • https://etrid.org/whitepaper/volume-2.html")
    print(f"   • https://etrid.org/whitepaper/volume-3.html")
    print(f"   • https://etrid.org/whitepaper/protocol-charter.html")
    print(f"\n💡 NO JavaScript - just pure HTML!")
    print(f"💡 Works in ALL browsers - Firefox, Chrome, Safari, etc.")
    print(f"💡 Loads INSTANTLY - no waiting!")
else:
    print(f"\n⚠️  Some files failed to upload")
