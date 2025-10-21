#!/usr/bin/env python3
"""
Fix all remaining bridges by using BTC template and customizing for each bridge
"""

import re
from pathlib import Path
from typing import Dict

# Bridge configurations with their specific Config implementations
BRIDGE_CONFIGS = {
    'eth': {
        'title': 'ETH-PBC RUNTIME - Ethereum Partition Burst Chain',
        'description': 'Integrates: Ethereum Bridge + Lightning Channels + ASF Consensus',
        'bridge_pallet': 'pallet_ethereum_bridge',
        'bridge_name': 'EthereumBridge',
        'params': '''// EthereumBridge Configuration
parameter_types! {
    pub const MinEthConfirmations: u32 = 12;
    pub const EthBridgeFeeRate: u32 = 10; // 0.1%
    pub const MaxEthGasLimit: u64 = 21_000_000;
    pub const MaxEthDepositsPerAccount: u32 = 100;
    pub const MaxEthWithdrawalsPerAccount: u32 = 50;
}''',
        'config': '''impl pallet_ethereum_bridge::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinConfirmations = MinEthConfirmations;
    type BridgeFeeRate = EthBridgeFeeRate;
    type MaxGasLimit = MaxEthGasLimit;
    type MaxDepositsPerAccount = MaxEthDepositsPerAccount;
    type MaxWithdrawalsPerAccount = MaxEthWithdrawalsPerAccount;
}''',
        're_export': 'pub use pallet_ethereum_bridge;'
    },
    'xlm': {
        'title': 'XLM-PBC RUNTIME - Stellar Partition Burst Chain',
        'description': 'Integrates: Stellar Bridge + Lightning Channels + ASF Consensus',
        'bridge_pallet': 'pallet_stellar_bridge',
        'bridge_name': 'StellarBridge',
        'params': '''// StellarBridge Configuration
parameter_types! {
    pub const MinXlmConfirmations: u32 = 1;
    pub const XlmBridgeFeeRate: u32 = 10; // 0.1%
    pub const MaxXlmDepositsPerAccount: u32 = 100;
    pub const MaxXlmWithdrawalsPerAccount: u32 = 50;
}''',
        'config': '''impl pallet_stellar_bridge::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinConfirmations = MinXlmConfirmations;
    type BridgeFeeRate = XlmBridgeFeeRate;
    type MaxDepositsPerAccount = MaxXlmDepositsPerAccount;
    type MaxWithdrawalsPerAccount = MaxXlmWithdrawalsPerAccount;
}''',
        're_export': 'pub use pallet_stellar_bridge;'
    },
    'xrp': {
        'title': 'XRP-PBC RUNTIME - XRP Ledger Partition Burst Chain',
        'description': 'Integrates: XRP Bridge + Lightning Channels + ASF Consensus',
        'bridge_pallet': 'pallet_xrp_bridge',
        'bridge_name': 'XrpBridge',
        'params': '''// XrpBridge Configuration
parameter_types! {
    pub const MinXrpConfirmations: u32 = 1;
    pub const XrpBridgeFeeRate: u32 = 10; // 0.1%
    pub const MaxXrpFeeDrops: u64 = 1_000_000; // 1 XRP
    pub const MaxXrpDepositsPerAccount: u32 = 100;
    pub const MaxXrpWithdrawalsPerAccount: u32 = 50;
}''',
        'config': '''impl pallet_xrp_bridge::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinConfirmations = MinXrpConfirmations;
    type BridgeFeeRate = XrpBridgeFeeRate;
    type MaxFeeDrops = MaxXrpFeeDrops;
    type MaxDepositsPerAccount = MaxXrpDepositsPerAccount;
    type MaxWithdrawalsPerAccount = MaxXrpWithdrawalsPerAccount;
}''',
        're_export': 'pub use pallet_xrp_bridge;'
    },
    'bnb': {
        'title': 'BNB-PBC RUNTIME - BNB Chain Partition Burst Chain',
        'description': 'Integrates: BNB Bridge + Lightning Channels + ASF Consensus',
        'bridge_pallet': 'pallet_bnb_bridge',
        'bridge_name': 'BnbBridge',
        'params': '''// BnbBridge Configuration
parameter_types! {
    pub const MinBnbConfirmations: u32 = 15;
    pub const BnbBridgeFeeRate: u32 = 10; // 0.1%
    pub const MaxBnbGasLimit: u64 = 21_000_000;
    pub const MaxBnbGasPrice: u128 = 100_000_000_000; // 100 Gwei
    pub const MaxBnbDepositsPerAccount: u32 = 100;
    pub const MaxBnbWithdrawalsPerAccount: u32 = 50;
}''',
        'config': '''impl pallet_bnb_bridge::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinConfirmations = MinBnbConfirmations;
    type BridgeFeeRate = BnbBridgeFeeRate;
    type MaxGasLimit = MaxBnbGasLimit;
    type MaxGasPrice = MaxBnbGasPrice;
    type MaxDepositsPerAccount = MaxBnbDepositsPerAccount;
    type MaxWithdrawalsPerAccount = MaxBnbWithdrawalsPerAccount;
}''',
        're_export': 'pub use pallet_bnb_bridge;'
    },
    'trx': {
        'title': 'TRX-PBC RUNTIME - Tron Partition Burst Chain',
        'description': 'Integrates: Tron Bridge + Lightning Channels + ASF Consensus',
        'bridge_pallet': 'pallet_trx_bridge',
        'bridge_name': 'TronBridge',
        'params': '''// TronBridge Configuration
parameter_types! {
    pub const MinTrxConfirmations: u32 = 19;
    pub const TrxBridgeFeeRate: u32 = 10; // 0.1%
    pub const MaxTrxEnergyLimit: u64 = 100_000_000;
    pub const MaxTrxBandwidth: u64 = 100_000;
    pub const MaxTrxDepositsPerAccount: u32 = 100;
    pub const MaxTrxWithdrawalsPerAccount: u32 = 50;
}''',
        'config': '''impl pallet_trx_bridge::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinConfirmations = MinTrxConfirmations;
    type BridgeFeeRate = TrxBridgeFeeRate;
    type MaxEnergyLimit = MaxTrxEnergyLimit;
    type MaxBandwidth = MaxTrxBandwidth;
    type MaxDepositsPerAccount = MaxTrxDepositsPerAccount;
    type MaxWithdrawalsPerAccount = MaxTrxWithdrawalsPerAccount;
}''',
        're_export': 'pub use pallet_trx_bridge;'
    },
    'link': {
        'title': 'LINK-PBC RUNTIME - Chainlink Partition Burst Chain',
        'description': 'Integrates: Chainlink Bridge + Lightning Channels + ASF Consensus',
        'bridge_pallet': 'pallet_chainlink_bridge',
        'bridge_name': 'ChainlinkBridge',
        'params': '''// ChainlinkBridge Configuration
parameter_types! {
    pub const MinLinkConfirmations: u32 = 12;
    pub const LinkBridgeFeeRate: u32 = 10; // 0.1%
    pub const MaxLinkOracleNodes: u32 = 100;
    pub const MaxLinkDataFeeds: u32 = 1000;
    pub const MaxLinkVRFRequests: u32 = 10000;
    pub const LinkPriceStalenessThreshold: u32 = 100; // blocks
}''',
        'config': '''impl pallet_chainlink_bridge::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinConfirmations = MinLinkConfirmations;
    type BridgeFeeRate = LinkBridgeFeeRate;
    type MaxOracleNodes = MaxLinkOracleNodes;
    type MaxDataFeeds = MaxLinkDataFeeds;
    type MaxVRFRequests = MaxLinkVRFRequests;
    type PriceStalenessThreshold = LinkPriceStalenessThreshold;
}''',
        're_export': 'pub use pallet_chainlink_bridge;'
    },
    'sol': {
        'title': 'SOL-PBC RUNTIME - Solana Partition Burst Chain',
        'description': 'Integrates: Solana Bridge + Lightning Channels + ASF Consensus',
        'bridge_pallet': 'pallet_sol_bridge',
        'bridge_name': 'SolanaBridge',
        'params': '''// SolanaBridge Configuration
parameter_types! {
    pub const MinSolConfirmations: u32 = 32;
    pub const SolBridgeFeeRate: u32 = 10; // 0.1%
    pub const MaxSolPriorityFee: u64 = 1_000_000; // lamports
    pub const MaxSolComputeUnits: u32 = 1_400_000;
    pub const MaxSolDepositsPerAccount: u32 = 100;
    pub const MaxSolWithdrawalsPerAccount: u32 = 50;
}''',
        'config': '''impl pallet_sol_bridge::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinConfirmations = MinSolConfirmations;
    type BridgeFeeRate = SolBridgeFeeRate;
    type MaxPriorityFee = MaxSolPriorityFee;
    type MaxComputeUnits = MaxSolComputeUnits;
    type MaxDepositsPerAccount = MaxSolDepositsPerAccount;
    type MaxWithdrawalsPerAccount = MaxSolWithdrawalsPerAccount;
}''',
        're_export': 'pub use pallet_sol_bridge;'
    },
    'sc-usdt': {
        'title': 'SC-USDT-PBC RUNTIME - Stablecoin USDT Partition Burst Chain',
        'description': 'Integrates: Stablecoin USDT Bridge + Lightning Channels + ASF Consensus',
        'bridge_pallet': 'pallet_stablecoin_usdt_bridge',
        'bridge_name': 'StablecoinUsdtBridge',
        'params': '''// StablecoinUsdtBridge Configuration
parameter_types! {
    pub const UsdtBridgeFeeRate: u32 = 5; // 0.05% for stablecoins
    pub const MaxUsdtDepositsPerAccount: u32 = 100;
    pub const MaxUsdtWithdrawalsPerAccount: u32 = 50;
}''',
        'config': '''impl pallet_stablecoin_usdt_bridge::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type BridgeFeeRate = UsdtBridgeFeeRate;
    type MaxDepositsPerAccount = MaxUsdtDepositsPerAccount;
    type MaxWithdrawalsPerAccount = MaxUsdtWithdrawalsPerAccount;
}''',
        're_export': 'pub use pallet_stablecoin_usdt_bridge;'
    },
    'doge': {
        'title': 'DOGE-PBC RUNTIME - Dogecoin Partition Burst Chain',
        'description': 'Integrates: Dogecoin Bridge + Lightning Channels + ASF Consensus',
        'bridge_pallet': 'pallet_doge_bridge',
        'bridge_name': 'DogeBridge',
        'params': '''// DogeBridge Configuration
use frame_support::PalletId;
use sp_runtime::Perbill;

parameter_types! {
    pub const DogeBridgeFee: Perbill = Perbill::from_percent(1);
    pub const MinDogeBridgeAmount: Balance = 1_000_000; // 0.001 ETR
    pub const MaxDogeBridgeAmount: Balance = 1_000_000_000_000; // 1M ETR
    pub const DogeBridgePalletId: PalletId = PalletId(*b"doge/brd");
    pub const DogeConfirmations: u32 = 20;
    pub const DogeConversionRate: u64 = 1_000_000; // 1 DOGE = 0.001 ETR
}''',
        'config': '''impl pallet_doge_bridge::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type BridgeFee = DogeBridgeFee;
    type MinBridgeAmount = MinDogeBridgeAmount;
    type MaxBridgeAmount = MaxDogeBridgeAmount;
    type PalletId = DogeBridgePalletId;
    type DogeConfirmations = DogeConfirmations;
    type DogeConversionRate = DogeConversionRate;
}''',
        're_export': 'pub use pallet_doge_bridge;'
    },
    'matic': {
        'title': 'MATIC-PBC RUNTIME - Polygon Partition Burst Chain',
        'description': 'Integrates: Polygon Bridge + Lightning Channels + ASF Consensus',
        'bridge_pallet': 'pallet_polygon_bridge',
        'bridge_name': 'PolygonBridge',
        'params': '''// PolygonBridge Configuration
use frame_support::PalletId;

parameter_types! {
    pub const MinMaticConfirmations: u32 = 128;
    pub const MaticBridgeFeeRate: u32 = 10; // 0.1%
    pub const MaxMaticGasLimit: u64 = 21_000_000;
    pub const MinMaticBridgeAmount: Balance = 1_000_000; // 0.001 ETR
    pub const MaxMaticDepositsPerAccount: u32 = 100;
    pub const MaxMaticWithdrawalsPerAccount: u32 = 50;
    pub const MaticBridgePalletId: PalletId = PalletId(*b"matic/br");
}''',
        'config': '''impl pallet_polygon_bridge::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinConfirmations = MinMaticConfirmations;
    type BridgeFeeRate = MaticBridgeFeeRate;
    type MaxGasLimit = MaxMaticGasLimit;
    type MinBridgeAmount = MinMaticBridgeAmount;
    type MaxDepositsPerAccount = MaxMaticDepositsPerAccount;
    type MaxWithdrawalsPerAccount = MaxMaticWithdrawalsPerAccount;
    type PalletId = MaticBridgePalletId;
}''',
        're_export': 'pub use pallet_polygon_bridge;'
    },
}

def fix_bridge(pbc: str, config: Dict) -> bool:
    """Fix a single bridge by customizing from BTC template"""

    # Read BTC template
    btc_path = Path('05-multichain/partition-burst-chains/pbc-chains/btc-pbc/runtime/src/lib.rs')
    if not btc_path.exists():
        print(f"  ❌ BTC template not found")
        return False

    with open(btc_path, 'r') as f:
        content = f.read()

    # 1. Replace title and description
    content = re.sub(
        r'//! BTC Partition Burst Chain Runtime.*?//! Imports:.*?\n',
        f'''//! {config['title']}
//! {config['description']}
''',
        content,
        flags=re.DOTALL
    )

    # 2. Replace re-export
    content = re.sub(
        r'pub use pallet_bitcoin_bridge;',
        config['re_export'],
        content
    )

    # 3. Replace Bitcoin Bridge Configuration with new bridge config
    content = re.sub(
        r'// Bitcoin Bridge Configuration\s+parameter_types!.*?\}.*?impl pallet_bitcoin_bridge::Config for Runtime \{.*?\}',
        config['params'] + '\n\n' + config['config'],
        content,
        flags=re.DOTALL
    )

    # 4. Replace construct_runtime! bridge entry
    content = re.sub(
        r'BitcoinBridge: pallet_bitcoin_bridge,',
        f'{config["bridge_name"]}: {config["bridge_pallet"]},',
        content
    )

    # Write to target PBC
    target_path = Path(f'05-multichain/partition-burst-chains/pbc-chains/{pbc}-pbc/runtime/src/lib.rs')
    with open(target_path, 'w') as f:
        f.write(content)

    return True

def main():
    print("=" * 70)
    print("🔧 Fixing All 10 Remaining Bridges from BTC Template")
    print("=" * 70)
    print()

    success = 0
    fail = 0

    for pbc, config in BRIDGE_CONFIGS.items():
        print(f"Fixing {pbc}-pbc...")

        if fix_bridge(pbc, config):
            print(f"  ✅ Fixed")
            success += 1
        else:
            print(f"  ❌ Failed")
            fail += 1

    print()
    print("=" * 70)
    print(f"Results: {success}/10 bridges fixed from template")
    print(f"✅ Success: {success}")
    print(f"❌ Failed: {fail}")
    print("=" * 70)

    return 0 if fail == 0 else 1

if __name__ == '__main__':
    exit(main())
