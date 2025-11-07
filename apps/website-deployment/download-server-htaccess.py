#!/usr/bin/env python3
import ftplib

FTP_HOST = "ftp.etrid.org"
FTP_USER = "u724092535"
FTP_PASS = "Fullashit13!"

print("📥 Downloading current .htaccess from server...\n")

try:
    ftp = ftplib.FTP(FTP_HOST, timeout=30)
    ftp.login(FTP_USER, FTP_PASS)
    
    # Download .htaccess
    local_file = "/tmp/live-htaccess.txt"
    try:
        with open(local_file, 'wb') as f:
            ftp.retrbinary('RETR /domains/etrid.org/public_html/.htaccess', f.write)
        print("✅ Downloaded .htaccess\n")
        
        # Show content
        with open(local_file, 'r') as f:
            content = f.read()
            print("="*60)
            print("CURRENT .htaccess CONTENT:")
            print("="*60)
            print(content)
            print("="*60)
            
    except Exception as e:
        print(f"Could not download .htaccess: {str(e)}")
    
    # List files in root
    print("\n📁 Files in public_html root:\n")
    ftp.cwd("/domains/etrid.org/public_html")
    files = []
    ftp.retrlines('LIST', files.append)
    for f in files[:30]:  # Show first 30
        print(f)
    
    ftp.quit()
    
except Exception as e:
    print(f"❌ Error: {str(e)}")
