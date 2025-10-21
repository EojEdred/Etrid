#!/usr/bin/env python3
"""
Fix all 12 PBC runtimes to have their CORRECT bridge pallets
Currently eth/xlm/xrp/sc-usdt have Bitcoin bridge instead of their own
"""

import re
from pathlib import Path

# Bridge configuration for each PBC
BRIDGE_CONFIGS = {
    'eth': {
        'pallet_crate': 'pallet_ethereum_bridge',
        'pallet_name': 'EthereumBridge',
        'cargo_package': 'eth-bridge',
        'cargo_path': 'ethereum-bridge',
        'min_confirmations': 12,
        'min_deposit': 10_000_000_000_000_000,  # 0.01 ETH
        'max_deposit': 1_000_000_000_000_000_000_000,  # 1000 ETH
        'param_prefix': 'Eth'
    },
    'xlm': {
        'pallet_crate': 'pallet_stellar_bridge',
        'pallet_name': 'StellarBridge',
        'cargo_package': 'stellar-bridge',
        'cargo_path': 'stellar-bridge',
        'min_confirmations': 1,
        'min_deposit': 1_000_000,  # 1 XLM
        'max_deposit': 100_000_000_000,  # 100k XLM
        'param_prefix': 'Xlm'
    },
    'xrp': {
        'pallet_crate': 'pallet_xrp_bridge',
        'pallet_name': 'XrpBridge',
        'cargo_package': 'xrp-bridge',
        'cargo_path': 'xrp-bridge',
        'min_confirmations': 1,
        'min_deposit': 1_000_000,  # 1 XRP
        'max_deposit': 100_000_000_000,  # 100k XRP
        'param_prefix': 'Xrp'
    },
    'sc-usdt': {
        'pallet_crate': 'pallet_stablecoin_usdt_bridge',
        'pallet_name': 'StablecoinUsdtBridge',
        'cargo_package': 'stablecoin-usdt-bridge',
        'cargo_path': 'stablecoin-usdt-bridge',
        'min_confirmations': 1,
        'min_deposit': 1_000_000,  # 1 USDT
        'max_deposit': 1_000_000_000_000,  # 1M USDT
        'param_prefix': 'ScUsdt'
    },
}

def fix_runtime_lib(pbc: str, config: dict):
    """Fix runtime lib.rs to use correct bridge"""

    runtime_path = Path(f'05-multichain/partition-burst-chains/pbc-chains/{pbc}-pbc/runtime/src/lib.rs')

    if not runtime_path.exists():
        print(f"  ❌ Runtime file not found: {runtime_path}")
        return False

    with open(runtime_path, 'r') as f:
        content = f.read()

    # 1. Replace parameter_types! block for bridge
    old_params = r'''// Bitcoin Bridge Configuration
parameter_types! {
    pub const MinBtcConfirmations: u32 = 6;
    pub const MinBtcDepositAmount: u64 = 10_000; // 0\.0001 BTC
    pub const MaxBtcDepositAmount: u64 = 100_000_000; // 1 BTC
    pub const BridgeAuthorityAccount: AccountId = AccountId::new\(\[0u8; 32\]\);
}'''

    new_params = f'''// {config['pallet_name']} Configuration
parameter_types! {{
    pub const Min{config['param_prefix']}Confirmations: u32 = {config['min_confirmations']};
    pub const Min{config['param_prefix']}DepositAmount: u64 = {config['min_deposit']};
    pub const Max{config['param_prefix']}DepositAmount: u64 = {config['max_deposit']};
    pub const BridgeAuthorityAccount: AccountId = AccountId::new([0u8; 32]);
}}'''

    content = re.sub(old_params, new_params, content)

    # 2. Replace Config implementation
    old_config = r'''impl pallet_bitcoin_bridge::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinConfirmations = MinBtcConfirmations;
    type MinDepositAmount = MinBtcDepositAmount;
    type MaxDepositAmount = MaxBtcDepositAmount;
    type BridgeAuthority = BridgeAuthorityAccount;
}'''

    new_config = f'''impl {config['pallet_crate']}::Config for Runtime {{
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinConfirmations = Min{config['param_prefix']}Confirmations;
    type MinDepositAmount = Min{config['param_prefix']}DepositAmount;
    type MaxDepositAmount = Max{config['param_prefix']}DepositAmount;
    type BridgeAuthority = BridgeAuthorityAccount;
}}'''

    content = re.sub(old_config, new_config, content)

    # 3. Replace construct_runtime! entry
    old_runtime = r'BitcoinBridge: pallet_bitcoin_bridge,'
    new_runtime = f'{config["pallet_name"]}: {config["pallet_crate"]},'

    content = re.sub(old_runtime, new_runtime, content)

    # Write back
    with open(runtime_path, 'w') as f:
        f.write(content)

    print(f"  ✅ Updated {runtime_path}")
    return True

def fix_cargo_toml(pbc: str, config: dict):
    """Fix Cargo.toml to depend on correct bridge"""

    cargo_path = Path(f'05-multichain/partition-burst-chains/pbc-chains/{pbc}-pbc/runtime/Cargo.toml')

    if not cargo_path.exists():
        print(f"  ❌ Cargo.toml not found: {cargo_path}")
        return False

    with open(cargo_path, 'r') as f:
        content = f.read()

    # Replace bridge dependency
    old_dep = r'pallet_bitcoin_bridge = { package = "pallet-bitcoin-bridge", path = "../../../../../05-multichain/bridge-protocols/bitcoin-bridge", default-features = false }'
    new_dep = f'{config["pallet_crate"]} = {{ package = "{config["cargo_package"]}", path = "../../../../../05-multichain/bridge-protocols/{config["cargo_path"]}", default-features = false }}'

    content = re.sub(old_dep, new_dep, content)

    # Replace in std features
    old_feature = r'"pallet_bitcoin_bridge/std",'
    new_feature = f'"{config["pallet_crate"]}/std",'

    content = re.sub(old_feature, new_feature, content)

    # Write back
    with open(cargo_path, 'w') as f:
        f.write(content)

    print(f"  ✅ Updated {cargo_path}")
    return True

def main():
    print("=" * 70)
    print("🔧 Fixing Bridge Pallets for All PBCs")
    print("=" * 70)
    print()

    success_count = 0
    fail_count = 0

    for pbc, config in BRIDGE_CONFIGS.items():
        print(f"Fixing {pbc}-pbc (should have {config['pallet_name']})...")

        lib_ok = fix_runtime_lib(pbc, config)
        cargo_ok = fix_cargo_toml(pbc, config)

        if lib_ok and cargo_ok:
            print(f"  ✅ {pbc}-pbc fixed successfully")
            success_count += 1
        else:
            print(f"  ❌ {pbc}-pbc failed to fix")
            fail_count += 1

        print()

    print("=" * 70)
    print(f"Results: {success_count}/{len(BRIDGE_CONFIGS)} PBCs fixed")
    print(f"✅ Success: {success_count}")
    print(f"❌ Failed: {fail_count}")
    print("=" * 70)

    return 0 if fail_count == 0 else 1

if __name__ == '__main__':
    exit(main())
