#!/usr/bin/env python3
"""
Automated Xcode iOS App Project Creator
Creates a minimal but functional Xcode project for the Etrid Wallet Swift app
"""

import os
import uuid
import shutil
import subprocess
from pathlib import Path

# Project configuration
PROJECT_NAME = "EtridWallet"
BUNDLE_ID = "com.etrid.wallet"
DEPLOYMENT_TARGET = "17.0"
SWIFT_VERSION = "5.0"

# Source files to include
SOURCE_FILES = [
    "WalletApp.swift",
    "ContentView.swift",
    "WalletManager.swift",
    "KeychainManager.swift",
    "QRScannerView.swift"
]

def generate_uuid():
    """Generate a unique 24-character hex ID for Xcode objects"""
    return uuid.uuid4().hex[:24].upper()

def create_project_structure(base_path):
    """Create the Xcode project directory structure"""
    project_path = base_path / f"{PROJECT_NAME}.xcodeproj"
    app_path = base_path / PROJECT_NAME

    # Create directories
    project_path.mkdir(parents=True, exist_ok=True)
    app_path.mkdir(parents=True, exist_ok=True)
    (app_path / "Assets.xcassets").mkdir(parents=True, exist_ok=True)
    (app_path / "Assets.xcassets" / "AppIcon.appiconset").mkdir(parents=True, exist_ok=True)
    (app_path / "Assets.xcassets" / "AccentColor.colorset").mkdir(parents=True, exist_ok=True)

    return project_path, app_path

def create_info_plist(app_path):
    """Create Info.plist with required permissions"""
    info_plist_content = """<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>$(DEVELOPMENT_LANGUAGE)</string>
    <key>CFBundleExecutable</key>
    <string>$(EXECUTABLE_NAME)</string>
    <key>CFBundleIdentifier</key>
    <string>$(PRODUCT_BUNDLE_IDENTIFIER)</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>$(PRODUCT_NAME)</string>
    <key>CFBundlePackageType</key>
    <string>$(PRODUCT_BUNDLE_PACKAGE_TYPE)</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>LSRequiresIPhoneOS</key>
    <true/>
    <key>UIApplicationSceneManifest</key>
    <dict>
        <key>UIApplicationSupportsMultipleScenes</key>
        <false/>
    </dict>
    <key>UIApplicationSupportsIndirectInputEvents</key>
    <true/>
    <key>UILaunchScreen</key>
    <dict/>
    <key>UIRequiredDeviceCapabilities</key>
    <array>
        <string>armv7</string>
    </array>
    <key>UISupportedInterfaceOrientations</key>
    <array>
        <string>UIInterfaceOrientationPortrait</string>
        <string>UIInterfaceOrientationLandscapeLeft</string>
        <string>UIInterfaceOrientationLandscapeRight</string>
    </array>
    <key>UISupportedInterfaceOrientations~ipad</key>
    <array>
        <string>UIInterfaceOrientationPortrait</string>
        <string>UIInterfaceOrientationPortraitUpsideDown</string>
        <string>UIInterfaceOrientationLandscapeLeft</string>
        <string>UIInterfaceOrientationLandscapeRight</string>
    </array>
    <key>NSCameraUsageDescription</key>
    <string>Camera access is required to scan QR codes</string>
    <key>NSFaceIDUsageDescription</key>
    <string>Face ID is used to secure your wallet</string>
</dict>
</plist>
"""
    with open(app_path / "Info.plist", 'w') as f:
        f.write(info_plist_content)

def create_assets(app_path):
    """Create minimal asset catalog files"""
    # AppIcon Contents.json
    appicon_content = {
        "images": [
            {
                "idiom": "universal",
                "platform": "ios",
                "size": "1024x1024"
            }
        ],
        "info": {
            "author": "xcode",
            "version": 1
        }
    }

    import json
    with open(app_path / "Assets.xcassets" / "AppIcon.appiconset" / "Contents.json", 'w') as f:
        json.dump(appicon_content, f, indent=2)

    # AccentColor Contents.json
    accent_content = {
        "colors": [
            {
                "idiom": "universal"
            }
        ],
        "info": {
            "author": "xcode",
            "version": 1
        }
    }

    with open(app_path / "Assets.xcassets" / "AccentColor.colorset" / "Contents.json", 'w') as f:
        json.dump(accent_content, f, indent=2)

    # Main Assets Contents.json
    assets_content = {
        "info": {
            "author": "xcode",
            "version": 1
        }
    }

    with open(app_path / "Assets.xcassets" / "Contents.json", 'w') as f:
        json.dump(assets_content, f, indent=2)

def create_project_pbxproj(project_path, app_path):
    """Create the project.pbxproj file with all necessary configurations"""

    # Generate UUIDs for all objects
    main_group_id = generate_uuid()
    app_group_id = generate_uuid()
    products_group_id = generate_uuid()
    app_product_id = generate_uuid()
    project_id = generate_uuid()
    target_id = generate_uuid()
    native_target_id = generate_uuid()
    sources_build_phase_id = generate_uuid()
    resources_build_phase_id = generate_uuid()
    frameworks_build_phase_id = generate_uuid()
    build_config_list_project_id = generate_uuid()
    build_config_list_target_id = generate_uuid()
    debug_config_project_id = generate_uuid()
    release_config_project_id = generate_uuid()
    debug_config_target_id = generate_uuid()
    release_config_target_id = generate_uuid()
    info_plist_ref_id = generate_uuid()
    assets_ref_id = generate_uuid()

    # Generate file references and build file IDs for source files
    source_file_refs = {}
    source_build_files = {}
    for source_file in SOURCE_FILES:
        source_file_refs[source_file] = generate_uuid()
        source_build_files[source_file] = generate_uuid()

    # Start building the project.pbxproj content
    pbxproj = f"""// !$*UTF8*$!
{{
	archiveVersion = 1;
	classes = {{
	}};
	objectVersion = 56;
	objects = {{

/* Begin PBXBuildFile section */
"""

    # Add build files for sources
    for source_file, build_file_id in source_build_files.items():
        file_ref_id = source_file_refs[source_file]
        pbxproj += f"\t\t{build_file_id} /* {source_file} in Sources */ = {{isa = PBXBuildFile; fileRef = {file_ref_id} /* {source_file} */; }};\n"

    # Add assets build file
    assets_build_file_id = generate_uuid()
    pbxproj += f"\t\t{assets_build_file_id} /* Assets.xcassets in Resources */ = {{isa = PBXBuildFile; fileRef = {assets_ref_id} /* Assets.xcassets */; }};\n"

    pbxproj += """/* End PBXBuildFile section */

/* Begin PBXFileReference section */
"""

    # Add file references
    pbxproj += f"\t\t{app_product_id} /* {PROJECT_NAME}.app */ = {{isa = PBXFileReference; explicitFileType = wrapper.application; includeInIndex = 0; path = {PROJECT_NAME}.app; sourceTree = BUILT_PRODUCTS_DIR; }};\n"

    for source_file, file_ref_id in source_file_refs.items():
        pbxproj += f"\t\t{file_ref_id} /* {source_file} */ = {{isa = PBXFileReference; lastKnownFileType = sourcecode.swift; path = {source_file}; sourceTree = \"<group>\"; }};\n"

    pbxproj += f"\t\t{info_plist_ref_id} /* Info.plist */ = {{isa = PBXFileReference; lastKnownFileType = text.plist.xml; path = Info.plist; sourceTree = \"<group>\"; }};\n"
    pbxproj += f"\t\t{assets_ref_id} /* Assets.xcassets */ = {{isa = PBXFileReference; lastKnownFileType = folder.assetcatalog; path = Assets.xcassets; sourceTree = \"<group>\"; }};\n"

    pbxproj += """/* End PBXFileReference section */

/* Begin PBXFrameworksBuildPhase section */
"""

    pbxproj += f"""\t\t{frameworks_build_phase_id} /* Frameworks */ = {{
			isa = PBXFrameworksBuildPhase;
			buildActionMask = 2147483647;
			files = (
			);
			runOnlyForDeploymentPostprocessing = 0;
		}};
/* End PBXFrameworksBuildPhase section */

/* Begin PBXGroup section */
\t\t{main_group_id} = {{
			isa = PBXGroup;
			children = (
				{app_group_id} /* {PROJECT_NAME} */,
				{products_group_id} /* Products */,
			);
			sourceTree = \"<group>\";
		}};
\t\t{app_group_id} /* {PROJECT_NAME} */ = {{
			isa = PBXGroup;
			children = (
"""

    # Add source files to group
    for source_file, file_ref_id in source_file_refs.items():
        pbxproj += f"\t\t\t\t{file_ref_id} /* {source_file} */,\n"

    pbxproj += f"""\t\t\t\t{assets_ref_id} /* Assets.xcassets */,
\t\t\t\t{info_plist_ref_id} /* Info.plist */,
			);
			path = {PROJECT_NAME};
			sourceTree = \"<group>\";
		}};
\t\t{products_group_id} /* Products */ = {{
			isa = PBXGroup;
			children = (
				{app_product_id} /* {PROJECT_NAME}.app */,
			);
			name = Products;
			sourceTree = \"<group>\";
		}};
/* End PBXGroup section */

/* Begin PBXNativeTarget section */
\t\t{native_target_id} /* {PROJECT_NAME} */ = {{
			isa = PBXNativeTarget;
			buildConfigurationList = {build_config_list_target_id} /* Build configuration list for PBXNativeTarget \"{PROJECT_NAME}\" */;
			buildPhases = (
				{sources_build_phase_id} /* Sources */,
				{frameworks_build_phase_id} /* Frameworks */,
				{resources_build_phase_id} /* Resources */,
			);
			buildRules = (
			);
			dependencies = (
			);
			name = {PROJECT_NAME};
			productName = {PROJECT_NAME};
			productReference = {app_product_id} /* {PROJECT_NAME}.app */;
			productType = \"com.apple.product-type.application\";
		}};
/* End PBXNativeTarget section */

/* Begin PBXProject section */
\t\t{project_id} /* Project object */ = {{
			isa = PBXProject;
			attributes = {{
				BuildIndependentTargetsInParallel = 1;
				LastSwiftUpdateCheck = 1600;
				LastUpgradeCheck = 1600;
				TargetAttributes = {{
					{native_target_id} = {{
						CreatedOnToolsVersion = 16.0;
					}};
				}};
			}};
			buildConfigurationList = {build_config_list_project_id} /* Build configuration list for PBXProject \"{PROJECT_NAME}\" */;
			compatibilityVersion = \"Xcode 14.0\";
			developmentRegion = en;
			hasScannedForEncodings = 0;
			knownRegions = (
				en,
				Base,
			);
			mainGroup = {main_group_id};
			productRefGroup = {products_group_id} /* Products */;
			projectDirPath = \"\";
			projectRoot = \"\";
			targets = (
				{native_target_id} /* {PROJECT_NAME} */,
			);
		}};
/* End PBXProject section */

/* Begin PBXResourcesBuildPhase section */
\t\t{resources_build_phase_id} /* Resources */ = {{
			isa = PBXResourcesBuildPhase;
			buildActionMask = 2147483647;
			files = (
				{assets_build_file_id} /* Assets.xcassets in Resources */,
			);
			runOnlyForDeploymentPostprocessing = 0;
		}};
/* End PBXResourcesBuildPhase section */

/* Begin PBXSourcesBuildPhase section */
\t\t{sources_build_phase_id} /* Sources */ = {{
			isa = PBXSourcesBuildPhase;
			buildActionMask = 2147483647;
			files = (
"""

    # Add source files to build phase
    for source_file, build_file_id in source_build_files.items():
        pbxproj += f"\t\t\t\t{build_file_id} /* {source_file} in Sources */,\n"

    pbxproj += f"""\t\t\t);
			runOnlyForDeploymentPostprocessing = 0;
		}};
/* End PBXSourcesBuildPhase section */

/* Begin XCBuildConfiguration section */
\t\t{debug_config_project_id} /* Debug */ = {{
			isa = XCBuildConfiguration;
			buildSettings = {{
				ALWAYS_SEARCH_USER_PATHS = NO;
				ASSETCATALOG_COMPILER_GENERATE_SWIFT_ASSET_SYMBOL_EXTENSIONS = YES;
				CLANG_ANALYZER_NONNULL = YES;
				CLANG_ANALYZER_NUMBER_OBJECT_CONVERSION = YES_AGGRESSIVE;
				CLANG_CXX_LANGUAGE_STANDARD = \"gnu++20\";
				CLANG_ENABLE_MODULES = YES;
				CLANG_ENABLE_OBJC_ARC = YES;
				CLANG_ENABLE_OBJC_WEAK = YES;
				CLANG_WARN_BLOCK_CAPTURE_AUTORELEASING = YES;
				CLANG_WARN_BOOL_CONVERSION = YES;
				CLANG_WARN_COMMA = YES;
				CLANG_WARN_CONSTANT_CONVERSION = YES;
				CLANG_WARN_DEPRECATED_OBJC_IMPLEMENTATIONS = YES;
				CLANG_WARN_DIRECT_OBJC_ISA_USAGE = YES_ERROR;
				CLANG_WARN_DOCUMENTATION_COMMENTS = YES;
				CLANG_WARN_EMPTY_BODY = YES;
				CLANG_WARN_ENUM_CONVERSION = YES;
				CLANG_WARN_INFINITE_RECURSION = YES;
				CLANG_WARN_INT_CONVERSION = YES;
				CLANG_WARN_NON_LITERAL_NULL_CONVERSION = YES;
				CLANG_WARN_OBJC_IMPLICIT_RETAIN_SELF = YES;
				CLANG_WARN_OBJC_LITERAL_CONVERSION = YES;
				CLANG_WARN_OBJC_ROOT_CLASS = YES_ERROR;
				CLANG_WARN_QUOTED_INCLUDE_IN_FRAMEWORK_HEADER = YES;
				CLANG_WARN_RANGE_LOOP_ANALYSIS = YES;
				CLANG_WARN_STRICT_PROTOTYPES = YES;
				CLANG_WARN_SUSPICIOUS_MOVE = YES;
				CLANG_WARN_UNGUARDED_AVAILABILITY = YES_AGGRESSIVE;
				CLANG_WARN_UNREACHABLE_CODE = YES;
				CLANG_WARN__DUPLICATE_METHOD_MATCH = YES;
				COPY_PHASE_STRIP = NO;
				DEBUG_INFORMATION_FORMAT = dwarf;
				ENABLE_STRICT_OBJC_MSGSEND = YES;
				ENABLE_TESTABILITY = YES;
				ENABLE_USER_SCRIPT_SANDBOXING = YES;
				GCC_C_LANGUAGE_STANDARD = gnu17;
				GCC_DYNAMIC_NO_PIC = NO;
				GCC_NO_COMMON_BLOCKS = YES;
				GCC_OPTIMIZATION_LEVEL = 0;
				GCC_PREPROCESSOR_DEFINITIONS = (
					\"DEBUG=1\",
				);
				GCC_WARN_64_TO_32_BIT_CONVERSION = YES;
				GCC_WARN_ABOUT_RETURN_TYPE = YES_ERROR;
				GCC_WARN_UNDECLARED_SELECTOR = YES;
				GCC_WARN_UNINITIALIZED_AUTOS = YES_AGGRESSIVE;
				GCC_WARN_UNUSED_FUNCTION = YES;
				GCC_WARN_UNUSED_VARIABLE = YES;
				IPHONEOS_DEPLOYMENT_TARGET = {DEPLOYMENT_TARGET};
				LOCALIZATION_PREFERS_STRING_CATALOGS = YES;
				MTL_ENABLE_DEBUG_INFO = INCLUDE_SOURCE;
				MTL_FAST_MATH = YES;
				ONLY_ACTIVE_ARCH = YES;
				SDKROOT = iphoneos;
				SWIFT_ACTIVE_COMPILATION_CONDITIONS = DEBUG;
				SWIFT_OPTIMIZATION_LEVEL = \"-Onone\";
			}};
			name = Debug;
		}};
\t\t{release_config_project_id} /* Release */ = {{
			isa = XCBuildConfiguration;
			buildSettings = {{
				ALWAYS_SEARCH_USER_PATHS = NO;
				ASSETCATALOG_COMPILER_GENERATE_SWIFT_ASSET_SYMBOL_EXTENSIONS = YES;
				CLANG_ANALYZER_NONNULL = YES;
				CLANG_ANALYZER_NUMBER_OBJECT_CONVERSION = YES_AGGRESSIVE;
				CLANG_CXX_LANGUAGE_STANDARD = \"gnu++20\";
				CLANG_ENABLE_MODULES = YES;
				CLANG_ENABLE_OBJC_ARC = YES;
				CLANG_ENABLE_OBJC_WEAK = YES;
				CLANG_WARN_BLOCK_CAPTURE_AUTORELEASING = YES;
				CLANG_WARN_BOOL_CONVERSION = YES;
				CLANG_WARN_COMMA = YES;
				CLANG_WARN_CONSTANT_CONVERSION = YES;
				CLANG_WARN_DEPRECATED_OBJC_IMPLEMENTATIONS = YES;
				CLANG_WARN_DIRECT_OBJC_ISA_USAGE = YES_ERROR;
				CLANG_WARN_DOCUMENTATION_COMMENTS = YES;
				CLANG_WARN_EMPTY_BODY = YES;
				CLANG_WARN_ENUM_CONVERSION = YES;
				CLANG_WARN_INFINITE_RECURSION = YES;
				CLANG_WARN_INT_CONVERSION = YES;
				CLANG_WARN_NON_LITERAL_NULL_CONVERSION = YES;
				CLANG_WARN_OBJC_IMPLICIT_RETAIN_SELF = YES;
				CLANG_WARN_OBJC_LITERAL_CONVERSION = YES;
				CLANG_WARN_OBJC_ROOT_CLASS = YES_ERROR;
				CLANG_WARN_QUOTED_INCLUDE_IN_FRAMEWORK_HEADER = YES;
				CLANG_WARN_RANGE_LOOP_ANALYSIS = YES;
				CLANG_WARN_STRICT_PROTOTYPES = YES;
				CLANG_WARN_SUSPICIOUS_MOVE = YES;
				CLANG_WARN_UNGUARDED_AVAILABILITY = YES_AGGRESSIVE;
				CLANG_WARN_UNREACHABLE_CODE = YES;
				CLANG_WARN__DUPLICATE_METHOD_MATCH = YES;
				COPY_PHASE_STRIP = NO;
				DEBUG_INFORMATION_FORMAT = \"dwarf-with-dsym\";
				ENABLE_NS_ASSERTIONS = NO;
				ENABLE_STRICT_OBJC_MSGSEND = YES;
				ENABLE_USER_SCRIPT_SANDBOXING = YES;
				GCC_C_LANGUAGE_STANDARD = gnu17;
				GCC_NO_COMMON_BLOCKS = YES;
				GCC_WARN_64_TO_32_BIT_CONVERSION = YES;
				GCC_WARN_ABOUT_RETURN_TYPE = YES_ERROR;
				GCC_WARN_UNDECLARED_SELECTOR = YES;
				GCC_WARN_UNINITIALIZED_AUTOS = YES_AGGRESSIVE;
				GCC_WARN_UNUSED_FUNCTION = YES;
				GCC_WARN_UNUSED_VARIABLE = YES;
				IPHONEOS_DEPLOYMENT_TARGET = {DEPLOYMENT_TARGET};
				LOCALIZATION_PREFERS_STRING_CATALOGS = YES;
				MTL_ENABLE_DEBUG_INFO = NO;
				MTL_FAST_MATH = YES;
				SDKROOT = iphoneos;
				SWIFT_COMPILATION_MODE = wholemodule;
				VALIDATE_PRODUCT = YES;
			}};
			name = Release;
		}};
\t\t{debug_config_target_id} /* Debug */ = {{
			isa = XCBuildConfiguration;
			buildSettings = {{
				ASSETCATALOG_COMPILER_APPICON_NAME = AppIcon;
				ASSETCATALOG_COMPILER_GLOBAL_ACCENT_COLOR_NAME = AccentColor;
				CODE_SIGN_STYLE = Automatic;
				CURRENT_PROJECT_VERSION = 1;
				DEVELOPMENT_TEAM = \"\";
				GENERATE_INFOPLIST_FILE = NO;
				INFOPLIST_FILE = {PROJECT_NAME}/Info.plist;
				INFOPLIST_KEY_UIApplicationSupportsIndirectInputEvents = YES;
				INFOPLIST_KEY_UILaunchScreen_Generation = YES;
				INFOPLIST_KEY_UISupportedInterfaceOrientations_iPad = \"UIInterfaceOrientationPortrait UIInterfaceOrientationPortraitUpsideDown UIInterfaceOrientationLandscapeLeft UIInterfaceOrientationLandscapeRight\";
				INFOPLIST_KEY_UISupportedInterfaceOrientations_iPhone = \"UIInterfaceOrientationPortrait UIInterfaceOrientationLandscapeLeft UIInterfaceOrientationLandscapeRight\";
				LD_RUNPATH_SEARCH_PATHS = (
					\"$$(inherited)\",
					\"@executable_path/Frameworks\",
				);
				MARKETING_VERSION = 1.0;
				PRODUCT_BUNDLE_IDENTIFIER = {BUNDLE_ID};
				PRODUCT_NAME = \"{PROJECT_NAME}\";
				SWIFT_EMIT_LOC_STRINGS = YES;
				SWIFT_VERSION = {SWIFT_VERSION};
				TARGETED_DEVICE_FAMILY = \"1,2\";
			}};
			name = Debug;
		}};
\t\t{release_config_target_id} /* Release */ = {{
			isa = XCBuildConfiguration;
			buildSettings = {{
				ASSETCATALOG_COMPILER_APPICON_NAME = AppIcon;
				ASSETCATALOG_COMPILER_GLOBAL_ACCENT_COLOR_NAME = AccentColor;
				CODE_SIGN_STYLE = Automatic;
				CURRENT_PROJECT_VERSION = 1;
				DEVELOPMENT_TEAM = \"\";
				GENERATE_INFOPLIST_FILE = NO;
				INFOPLIST_FILE = {PROJECT_NAME}/Info.plist;
				INFOPLIST_KEY_UIApplicationSupportsIndirectInputEvents = YES;
				INFOPLIST_KEY_UILaunchScreen_Generation = YES;
				INFOPLIST_KEY_UISupportedInterfaceOrientations_iPad = \"UIInterfaceOrientationPortrait UIInterfaceOrientationPortraitUpsideDown UIInterfaceOrientationLandscapeLeft UIInterfaceOrientationLandscapeRight\";
				INFOPLIST_KEY_UISupportedInterfaceOrientations_iPhone = \"UIInterfaceOrientationPortrait UIInterfaceOrientationLandscapeLeft UIInterfaceOrientationLandscapeRight\";
				LD_RUNPATH_SEARCH_PATHS = (
					\"$$(inherited)\",
					\"@executable_path/Frameworks\",
				);
				MARKETING_VERSION = 1.0;
				PRODUCT_BUNDLE_IDENTIFIER = {BUNDLE_ID};
				PRODUCT_NAME = \"{PROJECT_NAME}\";
				SWIFT_EMIT_LOC_STRINGS = YES;
				SWIFT_VERSION = {SWIFT_VERSION};
				TARGETED_DEVICE_FAMILY = \"1,2\";
			}};
			name = Release;
		}};
/* End XCBuildConfiguration section */

/* Begin XCConfigurationList section */
\t\t{build_config_list_project_id} /* Build configuration list for PBXProject \"{PROJECT_NAME}\" */ = {{
			isa = XCConfigurationList;
			buildConfigurations = (
				{debug_config_project_id} /* Debug */,
				{release_config_project_id} /* Release */,
			);
			defaultConfigurationIsVisible = 0;
			defaultConfigurationName = Release;
		}};
\t\t{build_config_list_target_id} /* Build configuration list for PBXNativeTarget \"{PROJECT_NAME}\" */ = {{
			isa = XCConfigurationList;
			buildConfigurations = (
				{debug_config_target_id} /* Debug */,
				{release_config_target_id} /* Release */,
			);
			defaultConfigurationIsVisible = 0;
			defaultConfigurationName = Release;
		}};
/* End XCConfigurationList section */
	}};
	rootObject = {project_id} /* Project object */;
}}
"""

    # Write the project.pbxproj file
    with open(project_path / "project.pbxproj", 'w') as f:
        f.write(pbxproj)

def copy_source_files(source_dir, app_path):
    """Copy Swift source files to the app directory"""
    for source_file in SOURCE_FILES:
        src = source_dir / source_file
        dst = app_path / source_file
        if src.exists():
            shutil.copy2(src, dst)
            print(f"✓ Copied {source_file}")
        else:
            print(f"✗ Warning: {source_file} not found at {src}")

def main():
    print("=" * 60)
    print("Etrid Wallet - Xcode iOS App Project Generator")
    print("=" * 60)

    # Paths
    script_dir = Path(__file__).parent
    source_dir = script_dir / "EtridWalletSwift" / "Sources" / "EtridWalletSwift"
    output_dir = script_dir / "EtridWalletApp"

    # Clean up if exists
    if output_dir.exists():
        print(f"\n🗑️  Removing existing project at {output_dir}")
        shutil.rmtree(output_dir)

    print(f"\n📁 Creating project structure...")
    project_path, app_path = create_project_structure(output_dir)

    print(f"\n📄 Creating Info.plist...")
    create_info_plist(app_path)

    print(f"\n🎨 Creating asset catalog...")
    create_assets(app_path)

    print(f"\n🔨 Generating Xcode project file...")
    create_project_pbxproj(project_path, app_path)

    print(f"\n📋 Copying Swift source files...")
    copy_source_files(source_dir, app_path)

    print("\n" + "=" * 60)
    print("✅ Xcode project created successfully!")
    print("=" * 60)
    print(f"\nProject location: {output_dir}")
    print(f"Xcode project: {project_path}")
    print(f"\nNext steps:")
    print(f"  1. Open project: open {output_dir}/{PROJECT_NAME}.xcodeproj")
    print(f"  2. Or build from CLI: cd {output_dir} && xcodebuild")
    print()

if __name__ == "__main__":
    main()
