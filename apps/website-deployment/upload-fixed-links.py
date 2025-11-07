#!/usr/bin/env python3
"""
Upload all fixed files to FTP server
"""

import ftplib
import os

FTP_HOST = "ftp.etrid.org"
FTP_USER = "u724092535"
FTP_PASS = "Fullashit13!"

FILES_TO_UPLOAD = [
    # Validator pages
    ("website/validators/index.html", "/domains/etrid.org/public_html/validators/index.html"),
    ("website/validators/participate.html", "/domains/etrid.org/public_html/validators/participate.html"),

    # Whitepaper files
    ("website/whitepaper/ivory-paper.md", "/domains/etrid.org/public_html/whitepaper/ivory-paper.md"),
    ("website/whitepaper/viewer-standalone.html", "/domains/etrid.org/public_html/whitepaper/viewer-standalone.html"),
    ("website/whitepaper/viewer-embedded.html", "/domains/etrid.org/public_html/whitepaper/viewer-embedded.html"),
]

def upload_file(ftp, local_path, remote_path):
    """Upload a single file"""
    try:
        with open(local_path, 'rb') as f:
            file_size = os.path.getsize(local_path)
            print(f"📤 Uploading {local_path}")
            print(f"   → {remote_path}")
            print(f"   Size: {file_size:,} bytes")

            ftp.storbinary(f'STOR {remote_path}', f)

            # Verify
            uploaded_size = ftp.size(remote_path)
            if uploaded_size == file_size:
                print(f"   ✅ Success! Verified {uploaded_size:,} bytes\n")
                return True
            else:
                print(f"   ⚠️  Size mismatch: {uploaded_size} != {file_size}\n")
                return False
    except Exception as e:
        print(f"   ❌ Failed: {e}\n")
        return False

def main():
    print("=" * 70)
    print("🚀 UPLOADING FIXED LINK FILES")
    print("=" * 70)
    print()

    script_dir = os.path.dirname(os.path.abspath(__file__))
    os.chdir(script_dir)

    try:
        ftp = ftplib.FTP(FTP_HOST)
        ftp.login(FTP_USER, FTP_PASS)
        print(f"✅ Connected to {FTP_HOST}\n")

        successful = 0
        failed = 0

        for local_path, remote_path in FILES_TO_UPLOAD:
            if os.path.exists(local_path):
                if upload_file(ftp, local_path, remote_path):
                    successful += 1
                else:
                    failed += 1
            else:
                print(f"❌ File not found: {local_path}\n")
                failed += 1

        print("=" * 70)
        print(f"📊 Upload Summary:")
        print(f"   ✅ Successful: {successful}")
        print(f"   ❌ Failed: {failed}")
        print("=" * 70)

        ftp.quit()

    except Exception as e:
        print(f"❌ Error: {e}")

if __name__ == "__main__":
    main()
