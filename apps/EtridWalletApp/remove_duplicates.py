#!/usr/bin/env python3
import re

pbxproj_path = 'EtridWallet.xcodeproj/project.pbxproj'

# Read the project file
with open(pbxproj_path, 'r') as f:
    content = f.read()

# UUIDs to remove (old entries without full path)
uuids_to_remove = {
    # ConsensusDayView.swift (old entry)
    '890A8DC01BC54EC29E34E251',  # PBXBuildFile
    '6F845E6B68D749FC8DA0A8E8',  # PBXFileReference

    # ProposalDetailView.swift (old entry)
    '75ED7C52159C49ABB4C19CAF',  # PBXBuildFile
    'C65CF837E9D7452287B12997',  # PBXFileReference

    # DirectorCandidateDetailView.swift (old entry)
    '30AB34C2D5E84D2286F71D27',  # PBXBuildFile
    '6D95A39A1DB34FDF9E7E4444',  # PBXFileReference

    # VotingPowerDetailView.swift (old entry)
    'AB6E0083B3B34661B3FC23E3',  # PBXBuildFile
    '788B6417694C4AD9AEBF7F0B',  # PBXFileReference

    # RewardsHistoryView.swift (old entry)
    'C840C4BAC2F549859CFCF4FA',  # PBXBuildFile
    '8F91BCF3F6464C2AABEB8481',  # PBXFileReference
}

# Remove lines containing these UUIDs
lines = content.split('\n')
filtered_lines = []
removed_count = 0

for line in lines:
    should_remove = False
    for uuid in uuids_to_remove:
        if uuid in line:
            should_remove = True
            removed_count += 1
            print(f"Removing line: {line.strip()}")
            break

    if not should_remove:
        filtered_lines.append(line)

# Join lines back together
content = '\n'.join(filtered_lines)

# Write back
with open(pbxproj_path, 'w') as f:
    f.write(content)

print(f"\n✅ Successfully removed {removed_count} duplicate entries!")
print("\nRemaining Consensus Day files (correct entries):")
print("  - Views/ConsensusDay/ConsensusDayView.swift")
print("  - Views/ConsensusDay/ProposalDetailView.swift")
print("  - Views/ConsensusDay/DirectorCandidateDetailView.swift")
print("  - Views/ConsensusDay/VotingPowerDetailView.swift")
print("  - Views/ConsensusDay/RewardsHistoryView.swift")
