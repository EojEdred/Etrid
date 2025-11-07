#!/usr/bin/env python3
"""
Upload Complete Telemetry System to Hostinger
Deploys enhanced telemetry page with interactive flame architecture
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
    # Enhanced telemetry page
    ("apps/telemetry/index.html", "domains/etrid.org/public_html/telemetry/index.html"),
    ("apps/telemetry/app-telemetry-feed.js", "domains/etrid.org/public_html/telemetry/app-telemetry-feed.js"),

    # Also upload to telemetry subdomain
    ("apps/telemetry/index.html", "domains/telemetry.etrid.org/public_html/index.html"),
    ("apps/telemetry/app-telemetry-feed.js", "domains/telemetry.etrid.org/public_html/app-telemetry-feed.js"),

    # Main website (verify flame architecture is there)
    ("website/index.html", "domains/etrid.org/public_html/index.html"),
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
    print("🔥 ËTRID Complete Telemetry System Upload")
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
            print("🎉 Complete telemetry system deployed successfully!")
            print()
            print("✨ What's New:")
            print()
            print("  🔥 INTERACTIVE FLAME ARCHITECTURE")
            print("     • Click blue core → FlareChain telemetry (21 validators)")
            print("     • Click orange ring → PBC telemetry (all 13 chains)")
            print("     • Click yellow outer → Lightning-Bloc telemetry")
            print()
            print("  📊 REAL-TIME DATA DISPLAYS")
            print("     • Live TPS charts with 60-second history")
            print("     • Node counts and uptime percentages")
            print("     • Block heights updating every 5 seconds")
            print("     • Health indicators (color-coded)")
            print()
            print("  💎 13 PBC CHAINS ACCESSIBLE")
            print("     • Bitcoin, Ethereum, BSC, Polygon, Avalanche")
            print("     • Solana, Cardano, Polkadot, Cosmos")
            print("     • Arbitrum, Optimism, Base, zkSync")
            print()
            print("  ⚡ ASF CONSENSUS MONITORING")
            print("     • PPFA committee validators")
            print("     • Consensus health status")
            print("     • Finality metrics")
            print()
            print("🌐 Live Telemetry URLs:")
            print("  📡 Main: https://etrid.org/telemetry")
            print("  📡 Subdomain: https://telemetry.etrid.org")
            print("  🏠 Main Site: https://etrid.org")
            print()
            print("🎨 Features:")
            print("  • Animated rotating flame layers")
            print("  • Slide-in modals with glassmorphism")
            print("  • Canvas-based TPS charts")
            print("  • Auto-refresh every 5 seconds")
            print("  • Responsive design (mobile-ready)")
            print("  • ESC key to close modals")
            print()
            print("📏 File Stats:")
            print("  • Telemetry page: 54 KB (complete system)")
            print("  • Main website: 95 KB (with flame architecture)")
            print("  • Total deployment: ~150 KB")
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
