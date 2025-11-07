#!/usr/bin/env python3
"""
Upload Flame Architecture Documentation Updates to Hostinger
"""

import ftplib
import os
import sys

# FTP Configuration
FTP_HOST = "ftp.etrid.org"
FTP_USER = "u724092535"
FTP_PASS = "Fullashit13!"

# Files to upload (local_path, remote_path)
FILES_TO_UPLOAD = [
    ("apps/docs/README.md", "domains/docs.etrid.org/public_html/README.md"),
    ("apps/docs/FAQ.md", "domains/docs.etrid.org/public_html/FAQ.md"),
    ("apps/docs/OPERATOR_GUIDE.md", "domains/docs.etrid.org/public_html/OPERATOR_GUIDE.md"),
    ("apps/docs/architecture.md", "domains/docs.etrid.org/public_html/architecture.md"),
    ("apps/docs/_sidebar.md", "domains/docs.etrid.org/public_html/_sidebar.md"),
    ("apps/docs/specifications/ivory-paper.md", "domains/docs.etrid.org/public_html/specifications/ivory-paper.md"),
]

def create_remote_dir(ftp, path):
    """Create remote directory if it doesn't exist"""
    dirs = path.split('/')
    current = ""
    for d in dirs[:-1]:  # Exclude filename
        if d:
            current += "/" + d
            try:
                ftp.mkd(current)
                print(f"   📁 Created directory: {current}")
            except:
                pass  # Directory already exists

def upload_file(ftp, local_path, remote_path):
    """Upload a single file via FTP"""
    try:
        # Create remote directory if needed
        create_remote_dir(ftp, remote_path)

        with open(local_path, 'rb') as f:
            file_size = os.path.getsize(local_path)
            print(f"📤 Uploading {local_path} ({file_size:,} bytes)")
            print(f"   → {remote_path}")
            ftp.storbinary(f'STOR {remote_path}', f)
            print(f"   ✅ Success!\n")
            return True
    except Exception as e:
        print(f"   ❌ Failed: {e}\n")
        return False

def main():
    print("=" * 70)
    print("🔥 ËTRID Flame Architecture Documentation Upload")
    print("=" * 70)
    print()

    # Change to deployment directory
    script_dir = os.path.dirname(os.path.abspath(__file__))
    os.chdir(script_dir)
    print(f"📂 Working directory: {script_dir}\n")

    try:
        # Connect to FTP
        print(f"🔌 Connecting to {FTP_HOST}...")
        ftp = ftplib.FTP(FTP_HOST)
        ftp.login(FTP_USER, FTP_PASS)
        print(f"✅ Connected as {FTP_USER}\n")

        # Upload files
        successful_uploads = 0
        failed_uploads = 0

        for local_path, remote_path in FILES_TO_UPLOAD:
            if os.path.exists(local_path):
                if upload_file(ftp, local_path, remote_path):
                    successful_uploads += 1
                else:
                    failed_uploads += 1
            else:
                print(f"⚠️  File not found: {local_path}\n")
                failed_uploads += 1

        # Summary
        print("=" * 70)
        print(f"📊 Upload Summary:")
        print(f"   ✅ Successful: {successful_uploads}")
        print(f"   ❌ Failed: {failed_uploads}")
        print()

        if failed_uploads == 0:
            print("🎉 All documentation updated successfully!")
            print()
            print("✨ What's New:")
            print("  🔥 Flame Architecture visualization (ASCII art)")
            print("  🔵 Blue Core (FlareChain) - 2,600°F hottest")
            print("  🔶 Orange Ring (PBCs) - 1,800-2,100°F")
            print("  ⚡ Yellow Outer (Lightning) - 1,200-1,800°F")
            print()
            print("  📚 New Files:")
            print("    ✅ FAQ.md - Complete FAQ with flame metaphor")
            print("    ✅ README.md - Updated with flame visualization")
            print("    ✅ OPERATOR_GUIDE.md - +672 lines (PBC collator guide)")
            print("    ✅ ivory-paper.md - +560 lines (CCTP Section 8.9)")
            print("    ✅ architecture.md - +336 lines (state sync)")
            print()
            print("  🎨 Design Theme:")
            print("    • Flame gradient colors throughout")
            print("    • 171,000+ TPS prominent messaging")
            print("    • Temperature-based layer metaphor")
            print()
            print("🌐 Live Documentation:")
            print("  📖 Main: https://docs.etrid.org")
            print("  📄 FAQ: https://docs.etrid.org/#/FAQ")
            print("  👨‍💻 Operator Guide: https://docs.etrid.org/#/OPERATOR_GUIDE")
            print("  📜 Ivory Paper: https://docs.etrid.org/#/specifications/ivory-paper")
            print()
            print("🔥 Total: 1,612 lines added across 5 documentation files")
        else:
            print("⚠️  Some uploads failed. Check errors above.")
            sys.exit(1)

        print("=" * 70)
        ftp.quit()

    except Exception as e:
        print(f"❌ FTP Error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
