#!/usr/bin/env python3
"""
Convert hex-encoded AccountIds in chainspec JSON to SS58 format.
This allows raw chainspec generation to work properly.
"""
import json
import sys
from substrateinterface import Keypair

def hex_to_ss58(hex_address, ss58_format=42):
    """Convert hex address to SS58 format."""
    # Remove 0x prefix if present
    if hex_address.startswith('0x'):
        hex_address = hex_address[2:]

    # Convert hex to SS58
    try:
        # Create a keypair from the public key (hex address is the public key)
        public_key_bytes = bytes.fromhex(hex_address)
        address = Keypair(public_key=public_key_bytes, ss58_format=ss58_format).ss58_address
        return address
    except Exception as e:
        print(f"Error converting {hex_address}: {e}", file=sys.stderr)
        return hex_address

def convert_value(value, ss58_format=42):
    """Recursively convert hex addresses to SS58 in JSON structure."""
    if isinstance(value, str) and value.startswith('0x') and len(value) == 66:
        # This looks like a hex-encoded AccountId (32 bytes = 64 hex chars + 0x)
        return hex_to_ss58(value, ss58_format)
    elif isinstance(value, dict):
        return {k: convert_value(v, ss58_format) for k, v in value.items()}
    elif isinstance(value, list):
        return [convert_value(item, ss58_format) for item in value]
    else:
        return value

def main():
    if len(sys.argv) < 2:
        print("Usage: convert_hex_to_ss58.py <input_chainspec.json> [output_chainspec.json]")
        sys.exit(1)

    input_file = sys.argv[1]
    output_file = sys.argv[2] if len(sys.argv) > 2 else input_file.replace('.json', '_ss58.json')

    # Read input chainspec
    with open(input_file, 'r') as f:
        chainspec = json.load(f)

    # Get SS58 format from chainspec properties if available
    ss58_format = chainspec.get('properties', {}).get('ss58Format', 42)

    # Convert genesis config
    if 'genesis' in chainspec and 'runtimeGenesis' in chainspec['genesis']:
        genesis_config = chainspec['genesis']['runtimeGenesis'].get('config', {})
        if genesis_config:
            chainspec['genesis']['runtimeGenesis']['config'] = convert_value(genesis_config, ss58_format)

    # Write output chainspec
    with open(output_file, 'w') as f:
        json.dump(chainspec, f, indent=2)

    print(f"Converted chainspec written to: {output_file}")

if __name__ == '__main__':
    main()
