#!/usr/bin/env python3
"""
Ëtrid FlareChain - Chainspec Hex→SS58 Converter

This script converts hex addresses to SS58 format in a plain chainspec,
enabling successful raw chainspec generation.

Usage:
    python3 convert-chainspec-to-raw.py <input-plain-chainspec> <output-plain-chainspec>

Author: Claude AI + Eoj
Date: November 2, 2025
"""

import json
import sys
import hashlib
import base58


def ss58_encode(public_key_hex, ss58_format=42):
    """
    Encode a public key to SS58 format.
    
    Args:
        public_key_hex: Hex string with or without '0x' prefix
        ss58_format: Network prefix (42 for generic Substrate)
    
    Returns:
        SS58-encoded address string
    """
    # Remove '0x' prefix if present
    if public_key_hex.startswith('0x'):
        public_key_hex = public_key_hex[2:]
    
    # Convert hex to bytes
    public_key_bytes = bytes.fromhex(public_key_hex)
    
    # SS58 encoding: prefix + public_key + checksum
    if ss58_format < 64:
        prefix = bytes([ss58_format])
    else:
        prefix = bytes([
            ((ss58_format & 0xfc) >> 2) | 0x40,
            (ss58_format >> 8) | ((ss58_format & 0x03) << 6)
        ])
    
    data = prefix + public_key_bytes
    checksum_input = b'SS58PRE' + data
    checksum = hashlib.blake2b(checksum_input, digest_size=64).digest()
    
    # Use first 2 bytes of checksum
    ss58_addr = base58.b58encode(data + checksum[:2]).decode('ascii')
    
    return ss58_addr


def convert_chainspec(input_file, output_file):
    """
    Convert hex addresses to SS58 in a plain chainspec.
    
    Args:
        input_file: Path to input plain chainspec JSON
        output_file: Path to output modified plain chainspec JSON
    """
    print("=" * 70)
    print("Ëtrid FlareChain - Chainspec Hex→SS58 Converter")
    print("=" * 70)
    
    # Load the plain chainspec
    print(f"\n📖 Loading: {input_file}")
    with open(input_file, 'r') as f:
        chainspec = json.load(f)
    
    # Get genesis config
    genesis = chainspec['genesis']['runtimeGenesis']['patch']
    
    # Convert GRANDPA authorities
    if 'grandpa' in genesis and 'authorities' in genesis['grandpa']:
        grandpa_authorities = genesis['grandpa']['authorities']
        print(f"\n🔄 Converting {len(grandpa_authorities)} GRANDPA authorities...")
        
        converted = 0
        for authority in grandpa_authorities:
            if authority[0].startswith('0x'):
                hex_key = authority[0]
                ss58_key = ss58_encode(hex_key, 42)
                authority[0] = ss58_key
                converted += 1
        
        print(f"   ✓ Converted {converted} GRANDPA keys to SS58")
    
    # Convert validatorCommittee validators
    if 'validatorCommittee' in genesis and 'validators' in genesis['validatorCommittee']:
        vc_validators = genesis['validatorCommittee']['validators']
        print(f"\n🔄 Converting {len(vc_validators)} ValidatorCommittee validators...")
        
        converted = 0
        for validator in vc_validators:
            if validator[0].startswith('0x'):
                hex_key = validator[0]
                ss58_key = ss58_encode(hex_key, 42)
                validator[0] = ss58_key
                converted += 1
        
        print(f"   ✓ Converted {converted} validator keys to SS58")
    
    # Save modified chainspec
    print(f"\n💾 Saving: {output_file}")
    with open(output_file, 'w') as f:
        json.dump(chainspec, f, indent=2)
    
    print("\n" + "=" * 70)
    print("✅ Conversion complete!")
    print("=" * 70)
    print("\nNext step: Generate raw chainspec with:")
    print(f"  flarechain-node build-spec --chain {output_file} --raw > chainspec-raw.json")
    print()


if __name__ == '__main__':
    if len(sys.argv) != 3:
        print("Usage: python3 convert-chainspec-to-raw.py <input-plain> <output-plain>")
        print("\nExample:")
        print("  python3 convert-chainspec-to-raw.py \\")
        print("    chainspec-mainnet-plain.json \\")
        print("    chainspec-mainnet-plain-fixed.json")
        sys.exit(1)
    
    input_chainspec = sys.argv[1]
    output_chainspec = sys.argv[2]
    
    try:
        convert_chainspec(input_chainspec, output_chainspec)
    except Exception as e:
        print(f"\n❌ Error: {e}")
        sys.exit(1)
