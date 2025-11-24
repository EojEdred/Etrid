#!/usr/bin/env python3
import re

pbxproj_path = 'EtridWallet.xcodeproj/project.pbxproj'

# Read the project file
with open(pbxproj_path, 'r') as f:
    content = f.read()

# Find the UUIDs for the Consensus Day files from PBXFileReference
consensus_file_refs = {}
patterns = [
    (r'(E7AB2647628349968281C055) /\* Views/ConsensusDay/ConsensusDayView\.swift \*/', 'ConsensusDayView.swift'),
    (r'(0066E69074244BAC91D38D76) /\* Views/ConsensusDay/ProposalDetailView\.swift \*/', 'ProposalDetailView.swift'),
    (r'(DA823641BAFF46C2B1C575E3) /\* Views/ConsensusDay/DirectorCandidateDetailView\.swift \*/', 'DirectorCandidateDetailView.swift'),
    (r'(F63CA122C7404D4FBE0D294D) /\* Views/ConsensusDay/VotingPowerDetailView\.swift \*/', 'VotingPowerDetailView.swift'),
    (r'(FB9AB9FEFD67416E872A36A2) /\* Views/ConsensusDay/RewardsHistoryView\.swift \*/', 'RewardsHistoryView.swift'),
]

for pattern, filename in patterns:
    match = re.search(pattern, content)
    if match:
        consensus_file_refs[filename] = match.group(1)
        print(f"Found {filename}: {match.group(1)}")

# Find the EtridWallet group and add the file references to its children
etrid_wallet_group_pattern = r'(1FDF3BE722C146A2BE8A8730 /\* EtridWallet \*/ = \{[\s\S]*?children = \([\s\S]*?)(WalletConnect\.swift.*?\n)'

match = re.search(etrid_wallet_group_pattern, content)
if match:
    print("\nFound EtridWallet group")

    # Add the file references after WalletConnect.swift
    insertion_point = match.end(2)
    new_entries = ""
    for filename, uuid in consensus_file_refs.items():
        new_entries += f"\t\t\t\t{uuid} /* Views/ConsensusDay/{filename} */,\n"

    content = content[:insertion_point] + new_entries + content[insertion_point:]
    print(f"Added {len(consensus_file_refs)} file references to EtridWallet group")
else:
    print("ERROR: Could not find EtridWallet group!")
    exit(1)

# Write back
with open(pbxproj_path, 'w') as f:
    f.write(content)

print("\n✅ Successfully added Consensus Day files to EtridWallet group!")
print("\nFiles added:")
for filename in consensus_file_refs.keys():
    print(f"  - Views/ConsensusDay/{filename}")
