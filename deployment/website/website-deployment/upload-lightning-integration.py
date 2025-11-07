#!/usr/bin/env python3
"""
Lightning Bloc Integration Deployment Script
Uploads Lightning Bloc Next.js app and updated main website to Hostinger
"""
import ftplib
import os
import sys
from pathlib import Path

# FTP Configuration
FTP_HOST = "ftp.etrid.org"
FTP_USER = "u724092535"
FTP_PASS = "Fullashit13!"
REMOTE_BASE = "/domains/etrid.org/public_html"

# Base paths
SCRIPT_DIR = Path(__file__).parent
# Go up to etrid root: deployment/website/website-deployment -> Desktop/etrid
ETRID_ROOT = SCRIPT_DIR.parent.parent.parent
LIGHTNING_OUT = ETRID_ROOT / "apps/lightning-landing/out"
WEBSITE_DIR = SCRIPT_DIR / "website"

def ensure_remote_dir(ftp, remote_path):
    """Create remote directory if it doesn't exist"""
    parts = remote_path.strip('/').split('/')
    current = ''
    for part in parts:
        current += '/' + part
        try:
            ftp.cwd(current)
        except:
            try:
                ftp.mkd(current)
                print(f"   📁 Created directory: {current}")
            except:
                pass
    ftp.cwd('/')

def upload_directory(ftp, local_dir, remote_dir):
    """Recursively upload directory contents"""
    uploaded = 0
    failed = 0

    for item in local_dir.rglob('*'):
        if item.is_file():
            # Get relative path
            rel_path = item.relative_to(local_dir)
            remote_path = f"{remote_dir}/{rel_path}".replace('\\', '/')

            # Ensure parent directory exists
            remote_parent = '/'.join(remote_path.split('/')[:-1])
            ensure_remote_dir(ftp, remote_parent)

            try:
                file_size = item.stat().st_size
                print(f"📤 Uploading: {rel_path} ({file_size:,} bytes)")
                print(f"   → {remote_path}")

                with open(item, 'rb') as f:
                    ftp.storbinary(f'STOR {remote_path}', f)

                print(f"✅ Uploaded successfully\n")
                uploaded += 1

            except Exception as e:
                print(f"❌ Failed to upload {rel_path}: {str(e)}\n")
                failed += 1

    return uploaded, failed

def upload_file(ftp, local_path, remote_path):
    """Upload a single file"""
    try:
        if not os.path.exists(local_path):
            print(f"❌ File not found: {local_path}")
            return False

        file_size = os.path.getsize(local_path)
        print(f"📤 Uploading: {local_path} ({file_size:,} bytes)")
        print(f"   → {remote_path}")

        with open(local_path, 'rb') as f:
            ftp.storbinary(f'STOR {remote_path}', f)

        print(f"✅ Uploaded successfully\n")
        return True

    except Exception as e:
        print(f"❌ Failed to upload {local_path}: {str(e)}\n")
        return False

def main():
    print("\n" + "="*70)
    print("⚡ LIGHTNING BLOC INTEGRATION DEPLOYMENT")
    print("="*70 + "\n")

    # Check if Lightning build exists
    if not LIGHTNING_OUT.exists():
        print("❌ Lightning Bloc build not found!")
        print(f"   Expected: {LIGHTNING_OUT}")
        print("\n💡 Run this first:")
        print(f"   cd {LIGHTNING_OUT.parent}")
        print("   npm install")
        print("   npm run build")
        return 1

    # Check if website files exist
    index_html = WEBSITE_DIR / "index.html"
    if not index_html.exists():
        print(f"❌ Website index.html not found: {index_html}")
        return 1

    try:
        # Connect to FTP
        print(f"📡 Connecting to {FTP_HOST}...")
        ftp = ftplib.FTP(FTP_HOST, timeout=60)
        ftp.login(FTP_USER, FTP_PASS)
        print("✅ Connected successfully\n")

        total_uploaded = 0
        total_failed = 0

        # Step 1: Upload Lightning Bloc app
        print("="*70)
        print("STEP 1: Uploading Lightning Bloc Application")
        print("="*70 + "\n")

        remote_lightning = f"{REMOTE_BASE}/lightning"
        uploaded, failed = upload_directory(ftp, LIGHTNING_OUT, remote_lightning)
        total_uploaded += uploaded
        total_failed += failed

        print(f"\n📊 Lightning Bloc: {uploaded} files uploaded, {failed} failed\n")

        # Step 2: Upload updated main website index.html
        print("="*70)
        print("STEP 2: Uploading Updated Main Website")
        print("="*70 + "\n")

        remote_index = f"{REMOTE_BASE}/index.html"
        if upload_file(ftp, str(index_html), remote_index):
            total_uploaded += 1
        else:
            total_failed += 1

        ftp.quit()

        # Summary
        print("\n" + "="*70)
        print("📊 DEPLOYMENT SUMMARY")
        print("="*70)
        print(f"   ✅ Successful uploads: {total_uploaded}")
        print(f"   ❌ Failed uploads: {total_failed}")
        print("="*70)

        if total_failed == 0:
            print("\n🎉 Lightning Bloc Integration Deployed Successfully!")
            print("\n🌐 Your Lightning Bloc page is now live at:")
            print("   https://etrid.org/lightning/")
            print("\n📋 Next Steps:")
            print("   1. Wait 2-3 minutes for CDN/cache to clear")
            print("   2. Visit https://etrid.org/ and check the navigation menu")
            print("   3. Click the '⚡ Lightning' link to test the integration")
            print("   4. Verify all features work on the Lightning page")
            print("   5. Test on mobile devices")
            print("\n✨ The main website now has:")
            print("   • Lightning link in desktop navigation (with ⚡ icon)")
            print("   • Lightning link in mobile navigation")
            print("   • All links point to /lightning/")
        else:
            print(f"\n⚠️ {total_failed} file(s) failed to upload")
            print("   Please review the errors above and try again")
            return 1

        return 0

    except Exception as e:
        print(f"\n❌ Deployment Error: {str(e)}")
        import traceback
        traceback.print_exc()
        return 1

if __name__ == "__main__":
    sys.exit(main())
