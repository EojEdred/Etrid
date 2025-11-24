#!/usr/bin/env python3
import uuid
import re

def generate_uuid():
    return ''.join(str(uuid.uuid4()).replace('-', '').upper()[:24])

def add_files_to_xcode_project():
    pbxproj_path = 'EtridWallet.xcodeproj/project.pbxproj'

    # Read the project file
    with open(pbxproj_path, 'r') as f:
        content = f.read()

    # Files to add
    files = [
        'ConsensusDayView.swift',
        'ProposalDetailView.swift',
        'DirectorCandidateDetailView.swift',
        'VotingPowerDetailView.swift',
        'RewardsHistoryView.swift'
    ]

    # Generate UUIDs for each file (2 per file: one for PBXFileReference, one for PBXBuildFile)
    file_refs = {}
    build_files = {}
    for file in files:
        file_refs[file] = generate_uuid()
        build_files[file] = generate_uuid()

    # Find the PBXFileReference section
    file_ref_section = re.search(r'(/\* Begin PBXFileReference section \*/.*?/\* End PBXFileReference section \*/)', content, re.DOTALL)

    if file_ref_section:
        file_ref_end = file_ref_section.end()

        # Create PBXFileReference entries
        new_file_refs = ""
        for file in files:
            new_file_refs += f"\t\t{file_refs[file]} /* {file} */ = {{isa = PBXFileReference; fileEncoding = 4; lastKnownFileType = sourcecode.swift; path = {file}; sourceTree = \"<group>\"; }};\n"

        # Insert before "End PBXFileReference"
        insert_pos = content.rfind('/* End PBXFileReference section */')
        content = content[:insert_pos] + new_file_refs + content[insert_pos:]

    # Find the PBXBuildFile section
    build_file_section = re.search(r'(/\* Begin PBXBuildFile section \*/.*?/\* End PBXBuildFile section \*/)', content, re.DOTALL)

    if build_file_section:
        # Create PBXBuildFile entries
        new_build_files = ""
        for file in files:
            new_build_files += f"\t\t{build_files[file]} /* {file} in Sources */ = {{isa = PBXBuildFile; fileRef = {file_refs[file]} /* {file} */; }};\n"

        # Insert before "End PBXBuildFile"
        insert_pos = content.rfind('/* End PBXBuildFile section */')
        content = content[:insert_pos] + new_build_files + content[insert_pos:]

    # Find the PBXGroup section for Views
    # We need to add a ConsensusDay group under Views
    views_group_match = re.search(r'([A-F0-9]{24}) /\* Views \*/ = \{[^}]*children = \((.*?)\);', content, re.DOTALL)

    if views_group_match:
        consensus_group_uuid = generate_uuid()

        # Add ConsensusDay group reference to Views children
        views_children = views_group_match.group(2)
        insert_pos = content.find(views_children) + len(views_children)
        content = content[:insert_pos] + f"\n\t\t\t\t{consensus_group_uuid} /* ConsensusDay */," + content[insert_pos:]

        # Create the ConsensusDay group
        consensus_group = f"""\t\t{consensus_group_uuid} /* ConsensusDay */ = {{
\t\t\tisa = PBXGroup;
\t\t\tchildren = (
"""
        for file in files:
            consensus_group += f"\t\t\t\t{file_refs[file]} /* {file} */,\n"
        consensus_group += f"""\t\t\t);
\t\t\tpath = ConsensusDay;
\t\t\tsourceTree = \"<group>\";
\t\t}};
"""

        # Insert the group definition before "End PBXGroup"
        insert_pos = content.rfind('/* End PBXGroup section */')
        content = content[:insert_pos] + consensus_group + content[insert_pos:]

    # Find PBXSourcesBuildPhase and add build file references
    sources_phase = re.search(r'([A-F0-9]{24}) /\* Sources \*/ = \{[^}]*files = \((.*?)\);', content, re.DOTALL)

    if sources_phase:
        sources_files = sources_phase.group(2)
        insert_pos = content.find(sources_files) + len(sources_files)

        for file in files:
            content = content[:insert_pos] + f"\n\t\t\t\t{build_files[file]} /* {file} in Sources */," + content[insert_pos:]
            insert_pos += len(f"\n\t\t\t\t{build_files[file]} /* {file} in Sources */,")

    # Write back
    with open(pbxproj_path, 'w') as f:
        f.write(content)

    print("✅ Successfully added Consensus Day files to Xcode project!")
    print("Files added:")
    for file in files:
        print(f"  - {file}")

if __name__ == '__main__':
    add_files_to_xcode_project()
