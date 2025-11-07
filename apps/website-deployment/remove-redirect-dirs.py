#!/usr/bin/env python3
import ftplib

FTP_HOST = "ftp.etrid.org"
FTP_USER = "u724092535"
FTP_PASS = "Fullashit13!"
REMOTE_BASE = "/domains/etrid.org/public_html"

DIRS_TO_REMOVE = [
    "wallet",
    "bridge",
    "masterchef",
    "faucet",
    "validator-dashboard"
]

def remove_directory_recursive(ftp, path):
    """Recursively remove a directory and all its contents"""
    try:
        # Try to list files in directory
        files = []
        ftp.retrlines(f'LIST {path}', files.append)
        
        for file_info in files:
            parts = file_info.split()
            if len(parts) < 9:
                continue
            
            filename = ' '.join(parts[8:])
            if filename in ['.', '..']:
                continue
            
            file_path = f"{path}/{filename}"
            
            # Check if it's a directory
            if file_info.startswith('d'):
                remove_directory_recursive(ftp, file_path)
            else:
                ftp.delete(file_path)
        
        # Remove the directory itself
        ftp.rmd(path)
        return True
    except Exception as e:
        print(f"   Error: {str(e)}")
        return False

def cleanup_directories():
    print("🧹 Removing conflicting redirect directories...\n")
    
    try:
        ftp = ftplib.FTP(FTP_HOST, timeout=30)
        ftp.login(FTP_USER, FTP_PASS)
        print("✅ Connected to server\n")
        
        removed = 0
        failed = 0
        
        for dirname in DIRS_TO_REMOVE:
            dir_path = f"{REMOTE_BASE}/{dirname}"
            print(f"🗑️  Removing: {dirname}/")
            
            if remove_directory_recursive(ftp, dir_path):
                print(f"✅ Removed successfully\n")
                removed += 1
            else:
                print(f"❌ Failed to remove\n")
                failed += 1
        
        ftp.quit()
        
        print("="*60)
        print(f"📊 Summary:")
        print(f"   ✅ Removed: {removed}/{len(DIRS_TO_REMOVE)}")
        print(f"   ❌ Failed: {failed}/{len(DIRS_TO_REMOVE)}")
        print("="*60)
        
        if failed == 0:
            print("\n🎉 Cleanup complete!")
            print("✅ .htaccess 301 redirects will now work correctly")
        
        return 0
        
    except Exception as e:
        print(f"\n❌ Error: {str(e)}")
        return 1

if __name__ == "__main__":
    import sys
    sys.exit(cleanup_directories())
