#!/usr/bin/env python3
import ftplib
import os

FTP_HOST = "ftp.etrid.org"
FTP_USER = "u724092535"
FTP_PASS = "Fullashit13!"
REMOTE_BASE = "/domains/etrid.org/public_html"

# Redirect HTML content
REDIRECT_HTML = """<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="robots" content="noindex, nofollow">
    <meta http-equiv="refresh" content="0;url=https://etrid.org/">
    <link rel="canonical" href="https://etrid.org/">
    <title>Redirecting to ËTRID</title>
    <script>
        window.location.href = 'https://etrid.org/';
    </script>
</head>
<body>
    <p>Redirecting to <a href="https://etrid.org/">etrid.org</a>...</p>
</body>
</html>
"""

# Directories to create with index.html
REDIRECT_DIRS = [
    "wallet",
    "explorer", 
    "bridge",
    "masterchef",
    "faucet",
    "governance",
    "forum",
    "blog",
    "validator-dashboard"
]

def create_redirect_directories():
    print("🚀 Creating redirect directories on server...\n")
    
    try:
        ftp = ftplib.FTP(FTP_HOST, timeout=30)
        ftp.login(FTP_USER, FTP_PASS)
        print("✅ Connected to server\n")
        
        created = 0
        failed = 0
        
        for dirname in REDIRECT_DIRS:
            try:
                dir_path = f"{REMOTE_BASE}/{dirname}"
                
                # Try to create directory
                try:
                    ftp.mkd(dir_path)
                    print(f"📁 Created directory: {dirname}/")
                except:
                    print(f"📁 Directory exists: {dirname}/")
                
                # Upload index.html to the directory
                index_path = f"{dir_path}/index.html"
                
                # Create temporary file
                temp_file = f"/tmp/redirect_index_{dirname}.html"
                with open(temp_file, 'w') as f:
                    f.write(REDIRECT_HTML)
                
                # Upload it
                with open(temp_file, 'rb') as f:
                    ftp.storbinary(f'STOR {index_path}', f)
                
                print(f"✅ Uploaded: {dirname}/index.html\n")
                created += 1
                
                # Clean up temp file
                os.remove(temp_file)
                
            except Exception as e:
                print(f"❌ Failed to create {dirname}: {str(e)}\n")
                failed += 1
        
        ftp.quit()
        
        print("="*60)
        print(f"📊 Summary:")
        print(f"   ✅ Created: {created}/{len(REDIRECT_DIRS)}")
        print(f"   ❌ Failed: {failed}/{len(REDIRECT_DIRS)}")
        print("="*60)
        
        if failed == 0:
            print("\n🎉 All redirect directories created!")
            print("\n📋 Test these URLs (should redirect to homepage):")
            for dirname in REDIRECT_DIRS:
                print(f"   https://etrid.org/{dirname}")
        
        return 0
        
    except Exception as e:
        print(f"\n❌ Error: {str(e)}")
        return 1

if __name__ == "__main__":
    import sys
    sys.exit(create_redirect_directories())
