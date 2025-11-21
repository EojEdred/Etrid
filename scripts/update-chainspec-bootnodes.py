#!/usr/bin/env python3
import json
import sys

# Read bootnode list
with open('/tmp/bootnode-list.txt', 'r') as f:
    bootnodes = [line.strip() for line in f if line.strip()]

print(f"Loaded {len(bootnodes)} bootnodes")

# Read existing chain spec from build VM's fixed version
input_file = sys.argv[1] if len(sys.argv) > 1 else '/tmp/chainspec-fixed.json'
output_file = sys.argv[2] if len(sys.argv) > 2 else '/tmp/chainspec-with-bootnodes.json'

with open(input_file, 'r') as f:
    chainspec = json.load(f)

# Update bootNodes
chainspec['bootNodes'] = bootnodes

print(f"Updated bootNodes to {len(bootnodes)} nodes")
print("First 3 bootnodes:")
for bn in bootnodes[:3]:
    print(f"  {bn}")

# Write updated chain spec
with open(output_file, 'w') as f:
    json.dump(chainspec, f, indent=2)

print(f"\nUpdated chain spec saved to: {output_file}")
