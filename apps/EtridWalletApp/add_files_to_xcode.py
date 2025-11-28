#!/usr/bin/env python3
"""
Script to add all new Swift files to the Xcode project
"""

import os
import subprocess
import sys

def find_new_swift_files(base_dir):
    """Find all Swift files that need to be added to Xcode"""
    new_files = []

    # Models
    models_dir = os.path.join(base_dir, "EtridWallet/Models")
    model_files = [
        "NFT.swift", "Trading.swift", "Swap.swift", "Lending.swift",
        "Savings.swift", "FiatRamp.swift", "Contact.swift", "BillSplit.swift",
        "Social.swift", "DApp.swift", "WalletConnect.swift", "DAO.swift",
        "MultiSig.swift", "Notification.swift", "DeepLink.swift", "GPU.swift",
        "Hyperledger.swift", "ETHPBC.swift"
    ]
    for f in model_files:
        path = os.path.join(models_dir, f)
        if os.path.exists(path):
            new_files.append(path)

    # Services
    services_dir = os.path.join(base_dir, "EtridWallet/Services")
    service_files = [
        "HapticService.swift", "NotificationService.swift", "DeepLinkService.swift",
        "BiometricService.swift", "AppClipService.swift", "DAppBrowserService.swift",
        "WalletConnectService.swift", "DAOService.swift", "MultiSigService.swift",
        "GPUMarketplaceService.swift", "HyperledgerBridgeService.swift", "ETHPBCService.swift"
    ]
    for f in service_files:
        path = os.path.join(services_dir, f)
        if os.path.exists(path):
            new_files.append(path)

    # Views
    views_dir = os.path.join(base_dir, "EtridWallet/Views")

    # MainTabView
    main_tab = os.path.join(views_dir, "MainTabView.swift")
    if os.path.exists(main_tab):
        new_files.append(main_tab)

    # Find all Swift files in Views subdirectories
    for root, dirs, files in os.walk(views_dir):
        for file in files:
            if file.endswith(".swift") and file != "MainTabView.swift":
                full_path = os.path.join(root, file)
                # Skip already included files
                if "Components" in full_path or "Home" in full_path or "Onboarding" in full_path:
                    continue
                if "Send" in full_path or "Receive" in full_path or "Activity" in full_path:
                    continue
                new_files.append(full_path)

    return new_files

def main():
    base_dir = os.path.dirname(os.path.abspath(__file__))
    project_file = os.path.join(base_dir, "EtridWallet.xcodeproj/project.pbxproj")

    if not os.path.exists(project_file):
        print(f"❌ Error: Cannot find Xcode project at {project_file}")
        sys.exit(1)

    # Find all new files
    new_files = find_new_swift_files(base_dir)

    print(f"📦 Found {len(new_files)} files to add to Xcode project:\n")
    for f in sorted(new_files):
        print(f"  ✓ {os.path.relpath(f, base_dir)}")

    print("\n" + "="*80)
    print("⚠️  MANUAL XCODE SETUP REQUIRED")
    print("="*80)
    print("\nTo add these files to your Xcode project:")
    print("\n1. Open EtridWallet.xcodeproj in Xcode")
    print("2. Right-click on 'EtridWallet' folder in the left panel")
    print("3. Select 'Add Files to \"EtridWallet\"...'")
    print("4. Navigate to and select these folders:")
    print("   - EtridWallet/Models (select all new model files)")
    print("   - EtridWallet/Services (select all new service files)")
    print("   - EtridWallet/Views (select all new view subdirectories)")
    print("5. Make sure 'Copy items if needed' is UNCHECKED")
    print("6. Make sure 'Create groups' is selected")
    print("7. Make sure 'EtridWallet' target is checked")
    print("8. Click 'Add'")
    print("\nAlternatively, run this command to open Xcode:")
    print(f"  open {os.path.join(base_dir, 'EtridWallet.xcodeproj')}")
    print("\n" + "="*80)

    # Try using pbxproj library if available
    try:
        from pbxproj import XcodeProject

        print("\n🔧 Attempting automatic addition using pbxproj library...")

        project = XcodeProject.load(project_file)

        for file_path in new_files:
            rel_path = os.path.relpath(file_path, base_dir)
            try:
                project.add_file(rel_path, parent=project.get_or_create_group('EtridWallet'))
                print(f"  ✓ Added {rel_path}")
            except Exception as e:
                print(f"  ✗ Failed to add {rel_path}: {e}")

        project.save()
        print("\n✅ Successfully added files to Xcode project!")
        print("🔨 Now run: xcodebuild -scheme EtridWallet build")

    except ImportError:
        print("\n💡 TIP: Install pbxproj for automatic file addition:")
        print("  pip3 install pbxproj")
        print("\nFor now, please add files manually using the steps above.")

if __name__ == "__main__":
    main()
