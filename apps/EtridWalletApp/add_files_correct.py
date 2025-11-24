#!/usr/bin/env python3
import uuid
import re

def generate_uuid():
    return ''.join(str(uuid.uuid4()).replace('-', '').upper()[:24])

pbxproj_path = 'EtridWallet.xcodeproj/project.pbxproj'

# Files to add with correct paths
files = {
    'ConsensusDayView.swift': 'Views/ConsensusDay/ConsensusDayView.swift',
    'ProposalDetailView.swift': 'Views/ConsensusDay/ProposalDetailView.swift',
    'DirectorCandidateDetailView.swift': 'Views/ConsensusDay/DirectorCandidateDetailView.swift',
    'VotingPowerDetailView.swift': 'Views/ConsensusDay/VotingPowerDetailView.swift',
    'RewardsHistoryView.swift': 'Views/ConsensusDay/RewardsHistoryView.swift'
}

# Read the project file
with open(pbxproj_path, 'r') as f:
    content = f.read()

# Generate UUIDs
file_refs = {}
build_files = {}
for file in files.keys():
    file_refs[file] = generate_uuid()
    build_files[file] = generate_uuid()

# Add PBXFileReference entries
new_file_refs = ""
for file, path in files.items():
    new_file_refs += f"\t\t{file_refs[file]} /* {path} */ = {{isa = PBXFileReference; lastKnownFileType = sourcecode.swift; path = \"{path}\"; sourceTree = \"<group>\"; }};\n"

insert_pos = content.rfind('/* End PBXFileReference section */')
content = content[:insert_pos] + new_file_refs + content[insert_pos:]

# Add PBXBuildFile entries
new_build_files = ""
for file, path in files.items():
    new_build_files += f"\t\t{build_files[file]} /* {path} in Sources */ = {{isa = PBXBuildFile; fileRef = {file_refs[file]} /* {path} */; }};\n"

insert_pos = content.rfind('/* End PBXBuildFile section */')
content = content[:insert_pos] + new_build_files + content[insert_pos:]

# Add to PBXSourcesBuildPhase
sources_match = re.search(r'([A-F0-9]{24}) /\* Sources \*/ = \{\s+isa = PBXSourcesBuildPhase;.*?files = \((.*?)\);', content, re.DOTALL)
if sources_match:
    files_section = sources_match.group(2)
    # Find the end of the files list
    insert_pos = content.find(files_section) + len(files_section)

    for file, path in files.items():
        new_entry = f"\n\t\t\t\t{build_files[file]} /* {path} in Sources */,"
        content = content[:insert_pos] + new_entry + content[insert_pos:]
        insert_pos += len(new_entry)

# Write back
with open(pbxproj_path, 'w') as f:
    f.write(content)

print("✅ Successfully added Consensus Day files to Xcode project!")
for file, path in files.items():
    print(f"  - {path}")
