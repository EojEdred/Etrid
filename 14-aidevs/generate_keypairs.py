#!/usr/bin/env python3
"""
Generate Ed25519 keypairs for all AI Dev DIDs
"""

import json
import base58
from cryptography.hazmat.primitives.asymmetric import ed25519
from cryptography.hazmat.primitives import serialization

# List of all AI Dev identities
IDENTITIES = [
    # 12 AI Devs
    "consensus-dev01",
    "compiler-dev01",
    "governance-dev01",
    "audit-dev01",
    "oracle-dev01",
    "runtime-dev01",
    "economics-dev01",
    "edsc-dev01",
    "security-dev01",
    "multichain-dev01",
    "ethics-dev01",
    "docs-dev01",

    # 3 Gizzi personas
    "gizzi",
    "gizzi-claude",
    "gizzi-claudecode"
]

def generate_keypair(identity_name):
    """Generate Ed25519 keypair and return public key in base58"""
    # Generate private key
    private_key = ed25519.Ed25519PrivateKey.generate()

    # Get public key
    public_key = private_key.public_key()

    # Serialize keys
    private_bytes = private_key.private_bytes(
        encoding=serialization.Encoding.Raw,
        format=serialization.PrivateFormat.Raw,
        encryption_algorithm=serialization.NoEncryption()
    )

    public_bytes = public_key.public_bytes(
        encoding=serialization.Encoding.Raw,
        format=serialization.PublicFormat.Raw
    )

    # Encode public key in base58
    public_key_base58 = base58.b58encode(public_bytes).decode('utf-8')

    return {
        "identity": identity_name,
        "private_key_hex": private_bytes.hex(),
        "public_key_hex": public_bytes.hex(),
        "public_key_base58": public_key_base58
    }

def main():
    print("Generating Ed25519 keypairs for 15 AI Dev identities...\n")

    keypairs = []

    for identity in IDENTITIES:
        keypair = generate_keypair(identity)
        keypairs.append(keypair)
        print(f"✅ Generated keypair for: {identity}")
        print(f"   Public Key (base58): {keypair['public_key_base58'][:32]}...")
        print()

    # Save to JSON file (KEEP THIS SECURE!)
    output_file = "dids/keypairs.json"
    with open(output_file, 'w') as f:
        json.dump(keypairs, f, indent=2)

    print(f"✅ All keypairs saved to: {output_file}")
    print("\n⚠️  WARNING: Keep keypairs.json SECURE! Contains private keys.")
    print("   Recommended: Encrypt this file or move to offline storage.\n")

    # Generate summary for DID documents (public keys only)
    summary_file = "dids/public_keys.json"
    public_only = [
        {
            "identity": kp["identity"],
            "public_key_base58": kp["public_key_base58"],
            "public_key_hex": kp["public_key_hex"]
        }
        for kp in keypairs
    ]

    with open(summary_file, 'w') as f:
        json.dump(public_only, f, indent=2)

    print(f"✅ Public keys summary saved to: {summary_file}")
    print("   (Safe to share - public keys only)\n")

if __name__ == "__main__":
    main()
