#!/usr/bin/env python3
"""
Generate a 21-validator chain spec preset for local testing.
This creates a JSON preset that can be used with build-spec.
"""

import json

# Standard Substrate test accounts (well-known mnemonics)
VALIDATORS = [
    "Alice", "Bob", "Charlie", "Dave", "Eve", "Ferdie",
    "Alice//stash", "Bob//stash", "Charlie//stash", "Dave//stash", "Eve//stash", "Ferdie//stash",
    "Validator1", "Validator2", "Validator3", "Validator4", "Validator5",
    "Validator6", "Validator7", "Validator8", "Validator9"
]

# SS58 addresses for well-known test accounts (Sr25519)
VALIDATOR_ADDRESSES = {
    "Alice": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    "Bob": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
    "Charlie": "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y",
    "Dave": "5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy",
    "Eve": "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw",
    "Ferdie": "5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL",
    "Alice//stash": "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY",
    "Bob//stash": "5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc",
    "Charlie//stash": "5Ck5SLSHYac6WFt5UZRSsdJjwmpSZq85fd5TRNAdZQVzEAPT",
    "Dave//stash": "5HKPmK9GYtE1PSLsS1qiYU9xQ9Si1NcEhdeCq9sw5bqu4ns8",
    "Eve//stash": "5EhrCtDaQRYjVbLi7BafbGpFqcMhjZJdu8eW8gy6VRXh6HDp",
    "Ferdie//stash": "5CfwdnXWpLEHPBECErPQfQDUd7QNkXsXPqEp6W9m4VB8gSGB",
}

# GRANDPA keys (Ed25519) - these are different from Sr25519
GRANDPA_KEYS = {
    "Alice": "5FA9nQDVg267DEd8m1ZypXLBnvN7SFxYwV7ndqSYGiN9TTpu",
    "Bob": "5GoNkf6WdbxCFnPdAnYYQyCjAKPJgLNxXwPjwTh6DGg6gN3E",
    "Charlie": "5Fe3jZRbKes6aeuQ6HkcTvQeNhkkRPTXBwmNkuAPoimGEv45",
    "Dave": "5CtAmfGmqGTFNL1TgdASiUMwE2i1xrLojLSHZEZvfwCE7aPH",
    "Eve": "5CAZm6k4ZF39HS5Hhr5TpTEaUvgVUJ9BGEkKx2zwWxkPr1Yu",
    "Ferdie": "5FAFWdTk7NbTMr6Jh4cJ2dJLDFZpZZN8kGcCB5b8BdW3J8Ed",
    "Alice//stash": "5EHQngGEzB5tLR75FjdVS2YUMvvdqJYBW5TKaFpV8zwaTqRY",
    "Bob//stash": "5HbNF1sSZmMk88hqbNKmL2X5KpN3MBPXbYsFVjV1bHcxfqcb",
    "Charlie//stash": "5DXyDpCeGKsJRgL4rLHsGcG7cMcdHxWF9jmJmWJKxR7WSKR9",
    "Dave//stash": "5FPy7NvHGPzcyaZqKBHmQPjvMu2qh2CzZZWXzPKV7H6kkxqK",
    "Eve//stash": "5HE4gB2nUjwbVFh3DyYb1pXMYEjVNEdPkzGwY7kFgNQjkTN8",
    "Ferdie//stash": "5EBg2L2KA8Py6YcgHtTHTqQv9pNjVcJF3u4oGb8YP3zcfZjN",
}

# Generate addresses for additional validators (use derivation paths)
for i in range(1, 10):
    key = f"Validator{i}"
    # These are placeholder addresses - in real setup, you'd generate these
    VALIDATOR_ADDRESSES[key] = f"5{'F' * 47}{i:02d}"
    GRANDPA_KEYS[key] = f"5{'E' * 47}{i:02d}"

def generate_preset():
    """Generate the 21-validator preset JSON"""

    preset = {
        "balances": {
            "balances": []
        },
        "sudo": {
            "key": VALIDATOR_ADDRESSES["Alice"]
        },
        "grandpa": {
            "authorities": []
        },
        "consensus": {
            "validators": [],
            "slotDuration": 6000
        }
    }

    # Add balances for all validators
    for validator in VALIDATORS:
        address = VALIDATOR_ADDRESSES.get(validator)
        if address:
            preset["balances"]["balances"].append([
                address,
                100000000000000000000000  # 100,000 tokens
            ])

    # Add GRANDPA authorities
    for validator in VALIDATORS:
        grandpa_key = GRANDPA_KEYS.get(validator)
        if grandpa_key:
            preset["grandpa"]["authorities"].append([grandpa_key, 1])

    # Add consensus validators
    for validator in VALIDATORS:
        address = VALIDATOR_ADDRESSES.get(validator)
        if address:
            preset["consensus"]["validators"].append([
                address,
                64000000000000000000000,  # Stake amount
                "FlareNode"
            ])

    return preset

if __name__ == "__main__":
    preset = generate_preset()
    output_file = "05-multichain/flare-chain/runtime/presets/test_21validator.json"

    with open(output_file, 'w') as f:
        json.dump(preset, f, indent=2)

    print(f"✅ Generated 21-validator preset: {output_file}")
    print(f"   - {len(preset['balances']['balances'])} validators with balances")
    print(f"   - {len(preset['grandpa']['authorities'])} GRANDPA authorities")
    print(f"   - {len(preset['consensus']['validators'])} consensus validators")
