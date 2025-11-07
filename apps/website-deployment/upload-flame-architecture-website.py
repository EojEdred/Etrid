#!/usr/bin/env python3
"""
Upload Flame Architecture Enhanced Website to Hostinger
"""

import ftplib
import os
import sys

# FTP Configuration
FTP_HOST = "ftp.etrid.org"
FTP_USER = "u724092535"
FTP_PASS = "Fullashit13!"

# File to upload
FILE_TO_UPLOAD = ("website/index.html", "domains/etrid.org/public_html/index.html")

def upload_file(ftp, local_path, remote_path):
    """Upload a single file via FTP"""
    try:
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
    print("🔥 ËTRID Flame Architecture Website Upload")
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

        # Upload file
        local_path, remote_path = FILE_TO_UPLOAD
        success = upload_file(ftp, local_path, remote_path)

        # Summary
        print("=" * 70)
        if success:
            print("🎉 Website updated successfully!")
            print()
            print("✨ What's New:")
            print()
            print("  🔥 FLAME ARCHITECTURE VISUALIZATION")
            print("     • Interactive 3D-style animated flame")
            print("     • 🔵 Blue Core (FlareChain) - 2,600°F hottest")
            print("     • 🔶 Orange Ring (PBCs) - 1,800-2,100°F")
            print("     • ⚡ Yellow Outer (Lightning-Bloc) - 1,200-1,800°F")
            print()
            print("  📊 UPDATED PERFORMANCE STATS")
            print("     • All TPS mentions updated: 142k → 171,000+")
            print("     • Layer 1: 171,000+ TPS")
            print("     • Layer 2: 1M+ TPS")
            print("     • Finality: <500ms")
            print()
            print("  🎨 VISUAL ENHANCEMENTS")
            print("     • Animated concentric circles")
            print("     • Pulsing blue core")
            print("     • Rotating orange PBC ring (12s)")
            print("     • Rotating yellow Lightning ring (20s)")
            print("     • Interactive hover effects")
            print("     • TPS stats overlays")
            print()
            print("  📱 RESPONSIVE DESIGN")
            print("     • Desktop: 400x400px flame visualization")
            print("     • Mobile: 300x300px optimized")
            print("     • Smooth animations on all devices")
            print()
            print("🌐 Live Website:")
            print("  🏠 Main: https://etrid.org")
            print("  🔥 Flame Section: https://etrid.org/#flame-architecture")
            print()
            print("📏 File Stats:")
            print("  • Total lines: 1,070 (added ~266 lines)")
            print("  • New CSS: 165 lines of flame styling")
            print("  • New HTML: ~100 lines flame architecture section")
            print()
            print("💡 Tip: View the flame visualization on desktop for best")
            print("   experience. The animation is smooth and visually stunning!")
        else:
            print("⚠️  Upload failed. Check errors above.")
            sys.exit(1)

        print("=" * 70)
        ftp.quit()

    except Exception as e:
        print(f"❌ FTP Error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
