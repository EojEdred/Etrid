#!/usr/bin/env python3
"""
Validate bridge configuration in all 12 PBC runtimes
Checks:
- Bridge Config trait implementation exists
- Bridge is in construct_runtime! macro
- Cargo.toml has bridge dependency
- Security parameters are reasonable
"""

import os
import re
from pathlib import Path

# Expected bridges for each PBC
BRIDGE_MAPPING = {
    'btc': ('pallet_bitcoin_bridge', 'BitcoinBridge', 'pallet-bitcoin-bridge'),
    'eth': ('pallet_ethereum_bridge', 'EthereumBridge', 'eth-bridge'),
    'doge': ('pallet_doge_bridge', 'DogeBridge', 'pallet-doge-bridge'),
    'xlm': ('pallet_stellar_bridge', 'StellarBridge', 'stellar-bridge'),
    'xrp': ('pallet_xrp_bridge', 'XrpBridge', 'xrp-bridge'),
    'bnb': ('pallet_bnb_bridge', 'BnbBridge', 'bnb-bridge'),
    'trx': ('pallet_trx_bridge', 'TronBridge', 'trx-bridge'),
    'ada': ('pallet_cardano_bridge', 'CardanoBridge', 'pallet-cardano-bridge'),
    'link': ('pallet_chainlink_bridge', 'ChainlinkBridge', 'chainlink-bridge'),
    'matic': ('pallet_polygon_bridge', 'PolygonBridge', 'polygon-bridge'),
    'sc-usdt': ('pallet_stablecoin_usdt_bridge', 'StablecoinUsdtBridge', 'stablecoin-usdt-bridge'),
    'sol': ('pallet_sol_bridge', 'SolanaBridge', 'sol-bridge'),
}

def check_runtime(pbc: str) -> dict:
    """Check a single PBC runtime for bridge integration"""

    config_trait, runtime_name, cargo_name = BRIDGE_MAPPING[pbc]

    runtime_path = Path(f'05-multichain/partition-burst-chains/pbc-chains/{pbc}-pbc/runtime/src/lib.rs')
    cargo_path = Path(f'05-multichain/partition-burst-chains/pbc-chains/{pbc}-pbc/runtime/Cargo.toml')

    result = {
        'pbc': pbc,
        'has_config': False,
        'in_construct_runtime': False,
        'in_cargo': False,
        'has_parameters': False,
        'issues': []
    }

    if not runtime_path.exists():
        result['issues'].append('Runtime file missing')
        return result

    if not cargo_path.exists():
        result['issues'].append('Cargo.toml missing')
        return result

    # Read runtime file
    with open(runtime_path, 'r') as f:
        runtime_content = f.read()

    # Check Config implementation
    config_pattern = rf'impl {config_trait}::Config for Runtime'
    if re.search(config_pattern, runtime_content):
        result['has_config'] = True
    else:
        result['issues'].append(f'Missing Config implementation for {config_trait}')

    # Check construct_runtime! macro
    runtime_pattern = rf'{runtime_name}:\s*{config_trait}'
    if re.search(runtime_pattern, runtime_content):
        result['in_construct_runtime'] = True
    else:
        result['issues'].append(f'{runtime_name} not in construct_runtime!')

    # Check parameter_types! (security parameters)
    param_patterns = [
        r'MinConfirmations',
        r'MinDepositAmount',
        r'MaxDepositAmount',
        r'BridgeAuthority'
    ]

    found_params = sum(1 for p in param_patterns if re.search(p, runtime_content))
    if found_params >= 3:  # At least 3 of 4 parameters
        result['has_parameters'] = True
    else:
        result['issues'].append('Missing security parameters')

    # Read Cargo.toml
    with open(cargo_path, 'r') as f:
        cargo_content = f.read()

    # Check Cargo dependency (with package renaming support)
    cargo_patterns = [
        rf'{config_trait}\s*=.*{cargo_name}',  # With package renaming
        rf'{cargo_name}\s*=.*path'  # Direct dependency
    ]

    if any(re.search(p, cargo_content) for p in cargo_patterns):
        result['in_cargo'] = True
    else:
        result['issues'].append(f'Missing Cargo dependency for {cargo_name}')

    return result

def main():
    print("=" * 60)
    print("🔍 Validating Bridge Configuration (12/12 PBCs)")
    print("=" * 60)
    print()

    all_results = []
    pass_count = 0
    fail_count = 0

    for pbc in BRIDGE_MAPPING.keys():
        result = check_runtime(pbc)
        all_results.append(result)

        # Check if all validations passed
        if (result['has_config'] and
            result['in_construct_runtime'] and
            result['in_cargo'] and
            result['has_parameters'] and
            not result['issues']):
            status = "✅ PASS"
            pass_count += 1
        else:
            status = "❌ FAIL"
            fail_count += 1

        print(f"{pbc:12s} {status:12s}", end="")

        # Show validation details
        checks = []
        if result['has_config']:
            checks.append("Config✓")
        if result['in_construct_runtime']:
            checks.append("Runtime✓")
        if result['in_cargo']:
            checks.append("Cargo✓")
        if result['has_parameters']:
            checks.append("Params✓")

        print(f" [{', '.join(checks)}]")

        if result['issues']:
            for issue in result['issues']:
                print(f"             ⚠️  {issue}")

    print()
    print("=" * 60)
    print(f"Results: {pass_count}/12 PBCs fully configured")
    print(f"✅ Pass: {pass_count}")
    print(f"❌ Fail: {fail_count}")
    print("=" * 60)

    return 0 if fail_count == 0 else 1

if __name__ == '__main__':
    exit(main())
