#!/usr/bin/env python3
"""
Add bridge integration to remaining 7 PBCs
"""

import re
from pathlib import Path

# Remaining PBCs that need bridges
REMAINING_BRIDGES = {
    'doge': {
        'pallet_crate': 'pallet_doge_bridge',
        'pallet_name': 'DogeBridge',
        'param_prefix': 'Doge',
        'min_confirmations': 20,
        'min_deposit': 1_000_000,
        'max_deposit': 1_000_000_000_000,
        'min_comment': '1 DOGE',
        'max_comment': '1M DOGE'
    },
    'bnb': {
        'pallet_crate': 'pallet_bnb_bridge',
        'pallet_name': 'BnbBridge',
        'param_prefix': 'Bnb',
        'min_confirmations': 15,
        'min_deposit': 10_000_000_000_000_000,
        'max_deposit': 100_000_000_000_000_000_000,
        'min_comment': '0.01 BNB',
        'max_comment': '100 BNB'
    },
    'trx': {
        'pallet_crate': 'pallet_trx_bridge',
        'pallet_name': 'TronBridge',
        'param_prefix': 'Trx',
        'min_confirmations': 19,
        'min_deposit': 1_000_000,
        'max_deposit': 100_000_000_000,
        'min_comment': '1 TRX',
        'max_comment': '100k TRX'
    },
    'ada': {
        'pallet_crate': 'pallet_cardano_bridge',
        'pallet_name': 'CardanoBridge',
        'param_prefix': 'Ada',
        'min_confirmations': 15,
        'min_deposit': 1_000_000,
        'max_deposit': 100_000_000_000,
        'min_comment': '1 ADA',
        'max_comment': '100k ADA'
    },
    'link': {
        'pallet_crate': 'pallet_chainlink_bridge',
        'pallet_name': 'ChainlinkBridge',
        'param_prefix': 'Link',
        'min_confirmations': 12,
        'min_deposit': 10_000_000_000_000_000,
        'max_deposit': 10_000_000_000_000_000_000_000,
        'min_comment': '0.01 LINK',
        'max_comment': '10k LINK'
    },
    'matic': {
        'pallet_crate': 'pallet_polygon_bridge',
        'pallet_name': 'PolygonBridge',
        'param_prefix': 'Matic',
        'min_confirmations': 128,
        'min_deposit': 10_000_000_000_000_000,
        'max_deposit': 100_000_000_000_000_000_000_000,
        'min_comment': '0.01 MATIC',
        'max_comment': '100k MATIC'
    },
    'sol': {
        'pallet_crate': 'pallet_sol_bridge',
        'pallet_name': 'SolanaBridge',
        'param_prefix': 'Sol',
        'min_confirmations': 32,
        'min_deposit': 10_000_000_000_000_000,
        'max_deposit': 100_000_000_000_000_000_000,
        'min_comment': '0.01 SOL',
        'max_comment': '100 SOL'
    },
}

def add_bridge(pbc: str, config: dict) -> bool:
    """Add bridge to a runtime"""

    runtime_path = Path(f'05-multichain/partition-burst-chains/pbc-chains/{pbc}-pbc/runtime/src/lib.rs')

    if not runtime_path.exists():
        print(f"  ❌ Runtime not found")
        return False

    with open(runtime_path, 'r') as f:
        content = f.read()

    # Check if already exists
    if f"impl {config['pallet_crate']}::Config for Runtime" in content:
        print(f"  ✅ Bridge already exists")
        return True

    # Create bridge configuration
    bridge_config = f'''// {config['pallet_name']} Configuration
parameter_types! {{
    pub const Min{config['param_prefix']}Confirmations: u32 = {config['min_confirmations']};
    pub const Min{config['param_prefix']}DepositAmount: u64 = {config['min_deposit']}; // {config['min_comment']}
    pub const Max{config['param_prefix']}DepositAmount: u64 = {config['max_deposit']}; // {config['max_comment']}
    pub const BridgeAuthorityAccount: AccountId = AccountId::new([0u8; 32]);
}}

impl {config['pallet_crate']}::Config for Runtime {{
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinConfirmations = Min{config['param_prefix']}Confirmations;
    type MinDepositAmount = Min{config['param_prefix']}DepositAmount;
    type MaxDepositAmount = Max{config['param_prefix']}DepositAmount;
    type BridgeAuthority = BridgeAuthorityAccount;
}}

'''

    # Find construct_runtime! and insert before it
    construct_match = re.search(r'(// Create the runtime.*?)(construct_runtime!\()', content, re.DOTALL)

    if not construct_match:
        print(f"  ❌ Could not find construct_runtime!")
        return False

    # Insert bridge config
    insert_pos = construct_match.start(2)
    content = content[:insert_pos] + bridge_config + content[insert_pos:]

    # Add to construct_runtime! - find Consensus line and add bridge after it
    # Look for: "Consensus: pallet_consensus,"
    consensus_pattern = r'(        // Ëtrid Core\s+Consensus: pallet_consensus,)'

    bridge_entry = f'''\\1

        // Cross-chain Bridge
        {config['pallet_name']}: {config['pallet_crate']},'''

    content = re.sub(consensus_pattern, bridge_entry, content)

    # Write back
    with open(runtime_path, 'w') as f:
        f.write(content)

    print(f"  ✅ Added bridge to runtime")
    return True

def main():
    print("=" * 70)
    print("🔧 Adding Bridges to Remaining 7 PBCs")
    print("=" * 70)
    print()

    success = 0
    fail = 0

    for pbc, config in REMAINING_BRIDGES.items():
        print(f"Processing {pbc}-pbc ({config['pallet_name']})...")

        if add_bridge(pbc, config):
            success += 1
        else:
            fail += 1

        print()

    print("=" * 70)
    print(f"Results: {success}/7 PBCs processed")
    print(f"✅ Success: {success}")
    print(f"❌ Failed: {fail}")
    print("=" * 70)

    return 0 if fail == 0 else 1

if __name__ == '__main__':
    exit(main())
