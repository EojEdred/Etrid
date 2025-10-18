#!/usr/bin/env python3
"""
Complete all 9 remaining runtimes by copying btc-pbc and customizing properly.
This handles bridge pallet names, block times, and all chain-specific values.
"""

import re
from pathlib import Path
import shutil

# Configuration for each PBC: (name, bridge_pallet, block_time_ms, description)
PBCS = {
    "doge": {
        "bridge_pallet": "pallet_doge_bridge",
        "bridge_name": "DogeBridge",
        "block_time": "60000",  # 1 minute
        "min_period": "30000",
        "chain_name": "Dogecoin",
        "ticker": "DOGE",
        "unit": "Koinus",
        "description": "Much crypto, such decentralized, wow!"
    },
    "xrp": {
        "bridge_pallet": "pallet_ripple_bridge",
        "bridge_name": "RippleBridge",
        "block_time": "5000",  # 5 seconds
        "min_period": "2500",
        "chain_name": "XRP Ledger",
        "ticker": "XRP",
        "unit": "Drops",
        "description": "Enterprise-grade cross-border payments"
    },
    "bnb": {
        "bridge_pallet": "pallet_binance_bridge",
        "bridge_name": "BinanceBridge",
        "block_time": "3000",  # 3 seconds
        "min_period": "1500",
        "chain_name": "Binance Smart Chain",
        "ticker": "BNB",
        "unit": "Jager",
        "description": "DeFi powerhouse, Binance style"
    },
    "trx": {
        "bridge_pallet": "pallet_tron_bridge",
        "bridge_name": "TronBridge",
        "block_time": "3000",
        "min_period": "1500",
        "chain_name": "Tron",
        "ticker": "TRX",
        "unit": "Sun",
        "description": "Decentralize the web"
    },
    "ada": {
        "bridge_pallet": "pallet_cardano_bridge",
        "bridge_name": "CardanoBridge",
        "block_time": "20000",  # 20 seconds
        "min_period": "10000",
        "chain_name": "Cardano",
        "ticker": "ADA",
        "unit": "Lovelace",
        "description": "Peer-reviewed blockchain perfection"
    },
    "link": {
        "bridge_pallet": "pallet_chainlink_bridge",
        "bridge_name": "ChainlinkBridge",
        "block_time": "12000",  # 12 seconds
        "min_period": "6000",
        "chain_name": "Chainlink",
        "ticker": "LINK",
        "unit": "Juel",
        "description": "Oracles connecting chains"
    },
    "matic": {
        "bridge_pallet": "pallet_polygon_bridge",
        "bridge_name": "PolygonBridge",
        "block_time": "2000",  # 2 seconds
        "min_period": "1000",
        "chain_name": "Polygon",
        "ticker": "MATIC",
        "unit": "Wei",
        "description": "Ethereum scaling solution"
    },
    "sc-usdt": {
        "bridge_pallet": "pallet_stellar_usdt_bridge",
        "bridge_name": "StellarUsdtBridge",
        "block_time": "5000",
        "min_period": "2500",
        "chain_name": "Stellar USDT",
        "ticker": "SC-USDT",
        "unit": "Stroops",
        "description": "Stablecoin on Stellar"
    },
    "sol": {
        "bridge_pallet": "pallet_solana_bridge",
        "bridge_name": "SolanaBridge",
        "block_time": "400",  # 400ms
        "min_period": "200",
        "chain_name": "Solana",
        "ticker": "SOL",
        "unit": "Lamport",
        "description": "High-performance blockchain"
    }
}

BASE_DIR = Path("05-multichain/partition-burst-chains/pbc-chains")
BTC_RUNTIME = BASE_DIR / "btc-pbc" / "runtime" / "src" / "lib.rs"

def fix_runtime(pbc_name, config):
    """Copy btc runtime and customize for specific PBC"""

    dest_runtime = BASE_DIR / f"{pbc_name}-pbc" / "runtime" / "src" / "lib.rs"

    # Backup current file
    if dest_runtime.exists():
        shutil.copy(dest_runtime, str(dest_runtime) + ".before_completion")

    # Read btc runtime as template
    content = BTC_RUNTIME.read_text()

    # Replace header comment
    content = re.sub(
        r'// BTC-PBC RUNTIME.*?\n',
        f'// {config["ticker"]}-PBC RUNTIME - {config["chain_name"]} Partition Burst Chain\n',
        content
    )

    # Replace description
    content = re.sub(
        r'// Features:.*?\n',
        f'// Features: {config["description"]}\n',
        content
    )

    # Replace all btc/bitcoin references
    replacements = [
        (r'\bbtc_pbc\b', f'{pbc_name.replace("-", "_")}_pbc'),
        (r'\bbtc-pbc\b', f'{pbc_name}-pbc'),
        (r'\bBtcPbc\b', f'{to_pascal(pbc_name)}Pbc'),
        (r'\bBTC-PBC\b', f'{config["ticker"]}-PBC'),
        (r'\bBTC\b', config["ticker"]),
        (r'\bBitcoin\b', config["chain_name"]),
        (r'\bSatoshi\b', config["unit"]),
        # Bridge pallet replacements
        (r'\bpallet_bitcoin_bridge\b', config["bridge_pallet"]),
        (r'\bBitcoinBridge\b', config["bridge_name"]),
        # Block times
        (r'\b600000\b', config["block_time"]),  # BTC's 10 min = 600000ms
        (r'\b300000\b', config["min_period"]),  # BTC's min period = 300000ms
    ]

    for pattern, replacement in replacements:
        content = re.sub(pattern, replacement, content)

    # Write the customized runtime
    dest_runtime.write_text(content)

    return dest_runtime

def to_pascal(name):
    """Convert kebab-case to PascalCase"""
    if name == "sc-usdt":
        return "ScUsdt"
    parts = name.split('-')
    return ''.join(p.capitalize() for p in parts)

def main():
    print("🚀 Completing All 9 Remaining Runtimes")
    print("=" * 60)

    if not BTC_RUNTIME.exists():
        print(f"❌ BTC runtime template not found at {BTC_RUNTIME}")
        return

    completed = []
    failed = []

    for pbc_name, config in PBCS.items():
        print(f"\n📦 Processing {pbc_name}-pbc...")
        print(f"   Bridge: {config['bridge_pallet']}")
        print(f"   Chain: {config['chain_name']} ({config['ticker']})")

        try:
            runtime_file = fix_runtime(pbc_name, config)
            print(f"   ✅ Runtime created: {runtime_file}")
            completed.append(pbc_name)
        except Exception as e:
            print(f"   ❌ Failed: {e}")
            failed.append(pbc_name)

    print(f"\n{'=' * 60}")
    print(f"✅ Completed: {len(completed)}/{len(PBCS)}")
    if completed:
        print(f"   {', '.join(completed)}")
    if failed:
        print(f"❌ Failed: {len(failed)}")
        print(f"   {', '.join(failed)}")
    print(f"{'=' * 60}")

    # Now verify they compile
    print(f"\n🧪 Verifying compilation...")
    import subprocess

    passing = []
    failing = []

    for pbc_name in completed:
        print(f"   Testing {pbc_name}-pbc-runtime...", end=" ")
        result = subprocess.run(
            ["cargo", "check", "-p", f"{pbc_name}-pbc-runtime"],
            env={"SKIP_WASM_BUILD": "1", **subprocess.os.environ},
            capture_output=True,
            text=True
        )

        if result.returncode == 0:
            print("✅")
            passing.append(pbc_name)
        else:
            print("❌")
            failing.append(pbc_name)

    print(f"\n{'=' * 60}")
    print(f"📊 Compilation Results: {len(passing)}/{len(completed)} passing")
    if passing:
        print(f"✅ Passing: {', '.join(passing)}")
    if failing:
        print(f"⚠️  Failing: {', '.join(failing)}")
    print(f"{'=' * 60}")

if __name__ == "__main__":
    main()
