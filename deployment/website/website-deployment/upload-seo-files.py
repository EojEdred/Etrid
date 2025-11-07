#!/usr/bin/env python3
import ftplib
import os
import sys

# FTP Configuration
FTP_HOST = "ftp.etrid.org"
FTP_USER = "u724092535"
FTP_PASS = "Fullashit13!"
REMOTE_BASE = "/domains/etrid.org/public_html"

# Files to upload: (local_path, remote_path)
FILES_TO_UPLOAD = [
    # SEO Configuration Files
    ("website/robots.txt", "/domains/etrid.org/public_html/robots.txt"),
    ("website/sitemap.xml", "/domains/etrid.org/public_html/sitemap.xml"),
    ("website/.htaccess", "/domains/etrid.org/public_html/.htaccess"),
    
    # Redirect HTML Pages
    ("website/redirects/wallet.html", "/domains/etrid.org/public_html/wallet.html"),
    ("website/redirects/explorer.html", "/domains/etrid.org/public_html/explorer.html"),
    ("website/redirects/bridge.html", "/domains/etrid.org/public_html/bridge.html"),
    ("website/redirects/masterchef.html", "/domains/etrid.org/public_html/masterchef.html"),
    ("website/redirects/faucet.html", "/domains/etrid.org/public_html/faucet.html"),
    ("website/redirects/governance-portal.html", "/domains/etrid.org/public_html/governance.html"),
    ("website/redirects/forum.html", "/domains/etrid.org/public_html/forum.html"),
    ("website/redirects/blog.html", "/domains/etrid.org/public_html/blog.html"),
    ("website/redirects/validator-dashboard.html", "/domains/etrid.org/public_html/validator-dashboard.html"),
]

def upload_files():
    print("\n🚀 Starting SEO files upload to etrid.org...\n")
    
    try:
        # Connect to FTP
        print(f"📡 Connecting to {FTP_HOST}...")
        ftp = ftplib.FTP(FTP_HOST, timeout=30)
        ftp.login(FTP_USER, FTP_PASS)
        print("✅ Connected successfully\n")
        
        uploaded = 0
        failed = 0
        
        for local_path, remote_path in FILES_TO_UPLOAD:
            try:
                if not os.path.exists(local_path):
                    print(f"❌ File not found: {local_path}")
                    failed += 1
                    continue
                
                file_size = os.path.getsize(local_path)
                print(f"📤 Uploading: {local_path} ({file_size:,} bytes)")
                print(f"   → {remote_path}")
                
                with open(local_path, 'rb') as f:
                    ftp.storbinary(f'STOR {remote_path}', f)
                
                print(f"✅ Uploaded successfully\n")
                uploaded += 1
                
            except Exception as e:
                print(f"❌ Failed to upload {local_path}: {str(e)}\n")
                failed += 1
        
        ftp.quit()
        
        print("\n" + "="*60)
        print(f"📊 Upload Summary:")
        print(f"   ✅ Successful: {uploaded}/{len(FILES_TO_UPLOAD)}")
        print(f"   ❌ Failed: {failed}/{len(FILES_TO_UPLOAD)}")
        print("="*60)
        
        if failed == 0:
            print("\n🎉 All SEO files uploaded successfully!")
            print("\n📋 Next Steps:")
            print("   1. Wait 5-10 minutes for files to propagate")
            print("   2. Test redirects: https://etrid.org/wallet (should redirect to /)")
            print("   3. Verify robots.txt: https://etrid.org/robots.txt")
            print("   4. Verify sitemap: https://etrid.org/sitemap.xml")
            print("   5. Submit sitemap to Google Search Console")
            print("   6. Request removal of old URLs from Google index")
        else:
            print(f"\n⚠️ {failed} file(s) failed to upload")
            return 1
        
        return 0
        
    except Exception as e:
        print(f"\n❌ FTP Error: {str(e)}")
        return 1

if __name__ == "__main__":
    sys.exit(upload_files())
