#!/usr/bin/env python3
"""
Complete bridge integration for all 12 PBC runtimes.
This script properly configures each bridge pallet with correct Config implementations.
"""

import re
from pathlib import Path

# Bridge configuration mapping
BRIDGES = {
    "btc": {
        "pallet": "pallet_bitcoin_bridge",
        "name": "BitcoinBridge",
        "min_confirmations": 6,
        "min_deposit": "100_000",  # 0.001 BTC in satoshis
        "max_deposit": "100_000_000_000",  # 1000 BTC
    },
    "eth": {
        "pallet": "pallet_ethereum_bridge",
        "name": "EthereumBridge",
        "min_confirmations": 12,
        "min_deposit": "10_000_000_000_000_000",  # 0.01 ETH in wei
        "max_deposit": "1_000_000_000_000_000_000_000",  # 1000 ETH
    },
    "doge": {
        "pallet": "pallet_doge_bridge",
        "name": "DogeBridge",
        "min_confirmations": 20,
        "min_deposit": "100_000_000",  # 1 DOGE in koinus
        "max_deposit": "100_000_000_000_000",  # 1M DOGE
    },
    "xlm": {
        "pallet": "pallet_stellar_bridge",
        "name": "StellarBridge",
        "min_confirmations": 1,
        "min_deposit": "10_000_000",  # 1 XLM in stroops
        "max_deposit": "1_000_000_000_000",  # 100k XLM
    },
    "xrp": {
        "pallet": "pallet_xrp_bridge",
        "name": "XrpBridge",
        "min_confirmations": 1,
        "min_deposit": "1_000_000",  # 1 XRP in drops
        "max_deposit": "100_000_000_000",  # 100k XRP
    },
    "bnb": {
        "pallet": "pallet_bnb_bridge",
        "name": "BnbBridge",
        "min_confirmations": 15,
        "min_deposit": "10_000_000_000_000_000",  # 0.01 BNB in jager
        "max_deposit": "100_000_000_000_000_000_000",  # 100 BNB
    },
    "trx": {
        "pallet": "pallet_tron_bridge",
        "name": "TronBridge",
        "min_confirmations": 19,
        "min_deposit": "1_000_000",  # 1 TRX in sun
        "max_deposit": "100_000_000_000",  # 100k TRX
    },
    "ada": {
        "pallet": "pallet_cardano_bridge",
        "name": "CardanoBridge",
        "min_confirmations": 15,
        "min_deposit": "1_000_000",  # 1 ADA in lovelace
        "max_deposit": "100_000_000_000",  # 100k ADA
    },
    "link": {
        "pallet": "pallet_chainlink_bridge",
        "name": "ChainlinkBridge",
        "min_confirmations": 12,
        "min_deposit": "10_000_000_000_000_000",  # 0.01 LINK in juel
        "max_deposit": "10_000_000_000_000_000_000_000",  # 10k LINK
    },
    "matic": {
        "pallet": "pallet_polygon_bridge",
        "name": "PolygonBridge",
        "min_confirmations": 128,
        "min_deposit": "10_000_000_000_000_000",  # 0.01 MATIC in wei
        "max_deposit": "100_000_000_000_000_000_000_000",  # 100k MATIC
    },
    "sc-usdt": {
        "pallet": "pallet_stablecoin_usdt_bridge",
        "name": "StablecoinUsdtBridge",
        "min_confirmations": 1,
        "min_deposit": "1_000_000",  # 1 USDT in stroops
        "max_deposit": "1_000_000_000_000",  # 1M USDT
    },
    "sol": {
        "pallet": "pallet_solana_bridge",
        "name": "SolanaBridge",
        "min_confirmations": 32,
        "min_deposit": "10_000_000",  # 0.01 SOL in lamports
        "max_deposit": "100_000_000_000",  # 100 SOL
    },
}

BASE_DIR = Path("05-multichain/partition-burst-chains/pbc-chains")

def create_bridge_config(pbc_name, config):
    """Generate bridge Config implementation"""

    return f"""
impl {config['pallet']}::Config for Runtime {{
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinConfirmations = ConstU32<{config['min_confirmations']}>;
    type MinDepositAmount = ConstU64<{config['min_deposit']}>;
    type MaxDepositAmount = ConstU64<{config['max_deposit']}>;
    type BridgeAuthority = BridgeAuthorityAccount;
}}
"""

def integrate_bridge(pbc_name):
    """Integrate bridge into runtime"""

    if pbc_name not in BRIDGES:
        print(f"  ⚠️  No bridge config for {pbc_name}")
        return False

    config = BRIDGES[pbc_name]
    runtime_path = BASE_DIR / f"{pbc_name}-pbc" / "runtime" / "src" / "lib.rs"

    if not runtime_path.exists():
        print(f"  ❌ Runtime not found: {runtime_path}")
        return False

    content = runtime_path.read_text()

    # 1. Uncomment bridge import
    content = re.sub(
        rf'// pub use {config["pallet"]};.*',
        f'pub use {config["pallet"]};',
        content
    )

    # 2. Add bridge authority parameter (if not exists)
    if 'BridgeAuthorityAccount' not in content:
        # Add after other parameter_types
        param_insert = f'''
parameter_types! {{
    pub BridgeAuthorityAccount: AccountId = AccountId::from([0u8; 32]); // TODO: Set actual bridge authority
}}
'''
        # Find last parameter_types block
        match = list(re.finditer(r'parameter_types!\s*\{[^}]+\}', content))
        if match:
            last_params_end = match[-1].end()
            content = content[:last_params_end] + param_insert + content[last_params_end:]

    # 3. Replace commented bridge Config with proper implementation
    bridge_config_impl = create_bridge_config(pbc_name, config)

    # Remove old commented implementation
    content = re.sub(
        rf'// impl {config["pallet"]}::Config for Runtime \{{.*?// \}}',
        '',
        content,
        flags=re.DOTALL
    )

    # Insert new implementation after Consensus Config
    consensus_config_pattern = r'(impl pallet_consensus::Config for Runtime \{[^}]+\})'
    match = re.search(consensus_config_pattern, content, re.DOTALL)

    if match:
        insert_pos = match.end()
        content = content[:insert_pos] + bridge_config_impl + content[insert_pos:]
    else:
        print(f"  ⚠️  Could not find Consensus Config in {pbc_name}")
        return False

    # 4. Uncomment bridge in construct_runtime!
    content = re.sub(
        rf'// \s*{config["name"]}: {config["pallet"]},.*',
        f'        {config["name"]}: {config["pallet"]},',
        content
    )

    # Write back
    runtime_path.write_text(content)
    return True

def add_bridge_dependency(pbc_name):
    """Add bridge pallet to Cargo.toml if not present"""

    if pbc_name not in BRIDGES:
        return False

    config = BRIDGES[pbc_name]
    cargo_path = BASE_DIR / f"{pbc_name}-pbc" / "runtime" / "Cargo.toml"

    if not cargo_path.exists():
        return False

    content = cargo_path.read_text()

    # Check if dependency already exists
    if config['pallet'] in content:
        return True  # Already added

    # Find bridge pallets directory path
    bridge_name = config['pallet'].replace('pallet_', '').replace('_', '-')

    # Add dependency
    dep_line = f'{config["pallet"]} = {{ path = "../../../../../bridge-protocols/{bridge_name}", default-features = false }}\n'

    # Insert after other pallet dependencies
    insert_marker = '[dependencies]'
    if insert_marker in content:
        lines = content.split('\n')
        for i, line in enumerate(lines):
            if insert_marker in line:
                # Find a good insertion point (after existing pallets)
                for j in range(i+1, len(lines)):
                    if lines[j].startswith('pallet'):
                        continue
                    else:
                        lines.insert(j, dep_line.rstrip())
                        break
                break
        content = '\n'.join(lines)

    # Also add to features
    if '[features]' in content and 'std = [' in content:
        feature_line = f'    "{config["pallet"]}/std",\n'
        # Find std features array
        std_pattern = r'(std = \[)(.*?)(\])'
        match = re.search(std_pattern, content, re.DOTALL)
        if match and config['pallet'] not in match.group(2):
            replacement = match.group(1) + match.group(2).rstrip() + '\n' + feature_line + match.group(3)
            content = re.sub(std_pattern, replacement, content, flags=re.DOTALL)

    cargo_path.write_text(content)
    return True

def main():
    print("🌉 Complete Bridge Integration for All 12 PBCs")
    print("=" * 60)

    integrated = []
    failed = []

    for pbc_name in BRIDGES.keys():
        print(f"\n📦 Integrating {pbc_name}-pbc bridge...")

        # Add Cargo.toml dependency
        if add_bridge_dependency(pbc_name):
            print(f"  ✅ Cargo.toml dependency added")
        else:
            print(f"  ⚠️  Cargo.toml dependency check failed")

        # Integrate bridge into runtime
        if integrate_bridge(pbc_name):
            print(f"  ✅ Bridge integrated into runtime")
            integrated.append(pbc_name)
        else:
            print(f"  ❌ Bridge integration failed")
            failed.append(pbc_name)

    print(f"\n{'=' * 60}")
    print(f"✅ Integrated: {len(integrated)}/12")
    if integrated:
        print(f"   {', '.join(integrated)}")
    if failed:
        print(f"❌ Failed: {len(failed)}")
        print(f"   {', '.join(failed)}")
    print(f"{'=' * 60}")

if __name__ == "__main__":
    main()
