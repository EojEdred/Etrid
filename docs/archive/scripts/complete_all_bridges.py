#!/usr/bin/env python3
"""
Complete bridge integration for ALL 12 PBCs
Adds Config, parameter_types!, construct_runtime entry, and Cargo deps
"""

import re
from pathlib import Path

# Complete bridge configuration for all 12 PBCs
ALL_BRIDGE_CONFIGS = {
    'btc': {
        'pallet_crate': 'pallet_bitcoin_bridge',
        'pallet_name': 'BitcoinBridge',
        'cargo_package': 'pallet-bitcoin-bridge',
        'cargo_path': 'bitcoin-bridge',
        'min_confirmations': 6,
        'min_deposit': 10_000,  # 0.0001 BTC
        'max_deposit': 100_000_000,  # 1 BTC
        'param_prefix': 'Btc',
        'min_comment': '0.0001 BTC',
        'max_comment': '1 BTC'
    },
    'eth': {
        'pallet_crate': 'pallet_ethereum_bridge',
        'pallet_name': 'EthereumBridge',
        'cargo_package': 'eth-bridge',
        'cargo_path': 'ethereum-bridge',
        'min_confirmations': 12,
        'min_deposit': 10_000_000_000_000_000,  # 0.01 ETH
        'max_deposit': 1_000_000_000_000_000_000_000,  # 1000 ETH
        'param_prefix': 'Eth',
        'min_comment': '0.01 ETH',
        'max_comment': '1000 ETH'
    },
    'doge': {
        'pallet_crate': 'pallet_doge_bridge',
        'pallet_name': 'DogeBridge',
        'cargo_package': 'pallet-doge-bridge',
        'cargo_path': 'doge-bridge',
        'min_confirmations': 20,
        'min_deposit': 1_000_000,  # 1 DOGE
        'max_deposit': 1_000_000_000_000,  # 1M DOGE
        'param_prefix': 'Doge',
        'min_comment': '1 DOGE',
        'max_comment': '1M DOGE'
    },
    'xlm': {
        'pallet_crate': 'pallet_stellar_bridge',
        'pallet_name': 'StellarBridge',
        'cargo_package': 'stellar-bridge',
        'cargo_path': 'stellar-bridge',
        'min_confirmations': 1,
        'min_deposit': 1_000_000,  # 1 XLM
        'max_deposit': 100_000_000_000,  # 100k XLM
        'param_prefix': 'Xlm',
        'min_comment': '1 XLM',
        'max_comment': '100k XLM'
    },
    'xrp': {
        'pallet_crate': 'pallet_xrp_bridge',
        'pallet_name': 'XrpBridge',
        'cargo_package': 'xrp-bridge',
        'cargo_path': 'xrp-bridge',
        'min_confirmations': 1,
        'min_deposit': 1_000_000,  # 1 XRP
        'max_deposit': 100_000_000_000,  # 100k XRP
        'param_prefix': 'Xrp',
        'min_comment': '1 XRP',
        'max_comment': '100k XRP'
    },
    'bnb': {
        'pallet_crate': 'pallet_bnb_bridge',
        'pallet_name': 'BnbBridge',
        'cargo_package': 'bnb-bridge',
        'cargo_path': 'bnb-bridge',
        'min_confirmations': 15,
        'min_deposit': 10_000_000_000_000_000,  # 0.01 BNB
        'max_deposit': 100_000_000_000_000_000_000,  # 100 BNB
        'param_prefix': 'Bnb',
        'min_comment': '0.01 BNB',
        'max_comment': '100 BNB'
    },
    'trx': {
        'pallet_crate': 'pallet_trx_bridge',
        'pallet_name': 'TronBridge',
        'cargo_package': 'trx-bridge',
        'cargo_path': 'tron-bridge',
        'min_confirmations': 19,
        'min_deposit': 1_000_000,  # 1 TRX
        'max_deposit': 100_000_000_000,  # 100k TRX
        'param_prefix': 'Trx',
        'min_comment': '1 TRX',
        'max_comment': '100k TRX'
    },
    'ada': {
        'pallet_crate': 'pallet_cardano_bridge',
        'pallet_name': 'CardanoBridge',
        'cargo_package': 'pallet-cardano-bridge',
        'cargo_path': 'cardano-bridge',
        'min_confirmations': 15,
        'min_deposit': 1_000_000,  # 1 ADA
        'max_deposit': 100_000_000_000,  # 100k ADA
        'param_prefix': 'Ada',
        'min_comment': '1 ADA',
        'max_comment': '100k ADA'
    },
    'link': {
        'pallet_crate': 'pallet_chainlink_bridge',
        'pallet_name': 'ChainlinkBridge',
        'cargo_package': 'chainlink-bridge',
        'cargo_path': 'chainlink-bridge',
        'min_confirmations': 12,
        'min_deposit': 10_000_000_000_000_000,  # 0.01 LINK
        'max_deposit': 10_000_000_000_000_000_000_000,  # 10k LINK
        'param_prefix': 'Link',
        'min_comment': '0.01 LINK',
        'max_comment': '10k LINK'
    },
    'matic': {
        'pallet_crate': 'pallet_polygon_bridge',
        'pallet_name': 'PolygonBridge',
        'cargo_package': 'polygon-bridge',
        'cargo_path': 'polygon-bridge',
        'min_confirmations': 128,
        'min_deposit': 10_000_000_000_000_000,  # 0.01 MATIC
        'max_deposit': 100_000_000_000_000_000_000_000,  # 100k MATIC
        'param_prefix': 'Matic',
        'min_comment': '0.01 MATIC',
        'max_comment': '100k MATIC'
    },
    'sc-usdt': {
        'pallet_crate': 'pallet_stablecoin_usdt_bridge',
        'pallet_name': 'StablecoinUsdtBridge',
        'cargo_package': 'stablecoin-usdt-bridge',
        'cargo_path': 'stablecoin-usdt-bridge',
        'min_confirmations': 1,
        'min_deposit': 1_000_000,  # 1 USDT
        'max_deposit': 1_000_000_000_000,  # 1M USDT
        'param_prefix': 'ScUsdt',
        'min_comment': '1 USDT',
        'max_comment': '1M USDT'
    },
    'sol': {
        'pallet_crate': 'pallet_sol_bridge',
        'pallet_name': 'SolanaBridge',
        'cargo_package': 'sol-bridge',
        'cargo_path': 'solana-bridge',
        'min_confirmations': 32,
        'min_deposit': 10_000_000_000_000_000,  # 0.01 SOL
        'max_deposit': 100_000_000_000_000_000_000,  # 100 SOL
        'param_prefix': 'Sol',
        'min_comment': '0.01 SOL',
        'max_comment': '100 SOL'
    },
}

def add_bridge_to_runtime(pbc: str, config: dict) -> bool:
    """Add complete bridge integration to a runtime"""

    runtime_path = Path(f'05-multichain/partition-burst-chains/pbc-chains/{pbc}-pbc/runtime/src/lib.rs')

    if not runtime_path.exists():
        print(f"  ❌ Runtime not found: {runtime_path}")
        return False

    with open(runtime_path, 'r') as f:
        content = f.read()

    # Check if bridge already integrated
    if f"impl {config['pallet_crate']}::Config for Runtime" in content:
        if f"{config['pallet_name']}: {config['pallet_crate']}," in content:
            print(f"  ✅ Already integrated (active)")
            return True
        else:
            print(f"  ⚠️  Config exists but not in construct_runtime!")

    # Create bridge configuration
    bridge_config = f'''
// {config['pallet_name']} Configuration
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

    # Find location to insert (before construct_runtime!)
    construct_runtime_match = re.search(r'construct_runtime!\(', content)
    if not construct_runtime_match:
        print(f"  ❌ construct_runtime! not found")
        return False

    insert_pos = construct_runtime_match.start()

    # Insert bridge configuration
    content = content[:insert_pos] + bridge_config + "\n" + content[insert_pos:]

    # Add to construct_runtime! - find the closing brace of the runtime struct
    # Look for pattern like:  Consensus: pallet_consensus,\n    }
    runtime_block_match = re.search(
        r'(construct_runtime!\(\s*pub struct Runtime\s*\{[^}]*?)(\s*\}\s*\);)',
        content,
        re.DOTALL
    )

    if not runtime_block_match:
        print(f"  ❌ construct_runtime! struct not found")
        return False

    # Insert bridge before closing brace
    before_close = runtime_block_match.group(1)
    after_close = runtime_block_match.group(2)

    # Add bridge entry
    new_runtime = (
        before_close +
        f'\n\n        // Cross-chain Bridge\n        {config["pallet_name"]}: {config["pallet_crate"]},' +
        after_close
    )

    content = re.sub(
        r'construct_runtime!\(\s*pub struct Runtime\s*\{.*?\}\s*\);',
        new_runtime,
        content,
        flags=re.DOTALL
    )

    # Write back
    with open(runtime_path, 'w') as f:
        f.write(content)

    print(f"  ✅ Added bridge integration to runtime")
    return True

def add_cargo_dependency(pbc: str, config: dict) -> bool:
    """Add bridge dependency to Cargo.toml"""

    cargo_path = Path(f'05-multichain/partition-burst-chains/pbc-chains/{pbc}-pbc/runtime/Cargo.toml')

    if not cargo_path.exists():
        print(f"  ❌ Cargo.toml not found")
        return False

    with open(cargo_path, 'r') as f:
        content = f.read()

    # Check if already exists
    if config['pallet_crate'] in content or config['cargo_package'] in content:
        print(f"  ✅ Cargo dependency already exists")
        return True

    # Find [dependencies] section
    deps_match = re.search(r'\[dependencies\]', content)
    if not deps_match:
        print(f"  ❌ [dependencies] section not found")
        return False

    # Find end of dependencies (next section starting with [)
    next_section = re.search(r'\n\[', content[deps_match.end():])
    if next_section:
        insert_pos = deps_match.end() + next_section.start()
    else:
        insert_pos = len(content)

    # Add dependency
    new_dep = f'\n{config["pallet_crate"]} = {{ package = "{config["cargo_package"]}", path = "../../../../../05-multichain/bridge-protocols/{config["cargo_path"]}", default-features = false }}\n'

    content = content[:insert_pos] + new_dep + content[insert_pos:]

    # Add to std features
    std_features_match = re.search(r'(std = \[[^\]]*)', content, re.DOTALL)
    if std_features_match:
        # Add before closing bracket
        std_section = std_features_match.group(1)
        new_std = std_section + f'\n    "{config["pallet_crate"]}/std",'

        content = content.replace(std_section, new_std)

    # Write back
    with open(cargo_path, 'w') as f:
        f.write(content)

    print(f"  ✅ Added Cargo dependency")
    return True

def main():
    print("=" * 70)
    print("🚀 Complete Bridge Integration for All 12 PBCs")
    print("=" * 70)
    print()

    success_count = 0
    fail_count = 0

    for pbc, config in ALL_BRIDGE_CONFIGS.items():
        print(f"Processing {pbc}-pbc ({config['pallet_name']})...")

        runtime_ok = add_bridge_to_runtime(pbc, config)
        cargo_ok = add_cargo_dependency(pbc, config)

        if runtime_ok and cargo_ok:
            success_count += 1
        else:
            fail_count += 1

        print()

    print("=" * 70)
    print(f"Results: {success_count}/12 PBCs processed")
    print(f"✅ Success: {success_count}")
    print(f"❌ Failed: {fail_count}")
    print("=" * 70)

    return 0 if fail_count == 0 else 1

if __name__ == '__main__':
    exit(main())
