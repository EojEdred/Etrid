#!/usr/bin/env python3
"""
Fix ALL bridge Config implementations to match their actual trait requirements
Groups:
- Group A (BTC-style): BTC ✅, ADA ✅
- Group B (Fee-based): ETH, XLM, XRP, BNB, TRX, LINK, SOL, SC-USDT
- Group C (Pallet ID-based): DOGE, MATIC
"""

import re
from pathlib import Path

# Bridge configurations by group
BRIDGES_CONFIG = {
    # Group B: Fee-based bridges (ETH-style)
    'eth': {
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
        'runtime_entry': 'EthereumBridge: pallet_ethereum_bridge,'
    },
    'xlm': {
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
        'runtime_entry': 'StellarBridge: pallet_stellar_bridge,'
    },
    'xrp': {
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
        'runtime_entry': 'XrpBridge: pallet_xrp_bridge,'
    },
    'bnb': {
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
        'runtime_entry': 'BnbBridge: pallet_bnb_bridge,'
    },
    'trx': {
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
        'runtime_entry': 'TronBridge: pallet_trx_bridge,'
    },
    'link': {
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
        'runtime_entry': 'ChainlinkBridge: pallet_chainlink_bridge,'
    },
    'sol': {
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
        'runtime_entry': 'SolanaBridge: pallet_sol_bridge,'
    },
    'sc-usdt': {
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
        'runtime_entry': 'StablecoinUsdtBridge: pallet_stablecoin_usdt_bridge,'
    },
    # Group C: PalletId-based bridges
    'doge': {
        'params': '''// DogeBridge Configuration
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
        'runtime_entry': 'DogeBridge: pallet_doge_bridge,'
    },
    'matic': {
        'params': '''// PolygonBridge Configuration
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
        'runtime_entry': 'PolygonBridge: pallet_polygon_bridge,'
    },
}

def fix_bridge(pbc: str, config: dict) -> bool:
    """Fix a single bridge implementation"""

    runtime_path = Path(f'05-multichain/partition-burst-chains/pbc-chains/{pbc}-pbc/runtime/src/lib.rs')

    if not runtime_path.exists():
        return False

    with open(runtime_path, 'r') as f:
        content = f.read()

    # Remove old bridge Config implementation
    content = re.sub(
        r'\/\/.*Bridge Configuration.*?parameter_types!.*?\{[^}]*\}.*?impl pallet_\w+_bridge::Config for Runtime \{[^}]*\}',
        '',
        content,
        flags=re.DOTALL
    )

    # Remove old bridge from construct_runtime!
    content = re.sub(
        r'\s*\/\/.*\n\s*\w+Bridge: pallet_\w+_bridge,',
        '',
        content
    )

    # Add new Config before construct_runtime!
    construct_match = re.search(r'(\/\/ Create the runtime.*?)(construct_runtime!\()', content, re.DOTALL)
    if not construct_match:
        return False

    insert_pos = construct_match.start(2)
    new_config = config['params'] + '\n\n' + config['config'] + '\n\n'
    content = content[:insert_pos] + new_config + content[insert_pos:]

    # Add to construct_runtime! after Consensus
    content = re.sub(
        r'(\/\/ Ëtrid Core\s+Consensus: pallet_consensus,)',
        f'\\1\n\n        // Cross-chain Bridge\n        {config["runtime_entry"]}',
        content
    )

    with open(runtime_path, 'w') as f:
        f.write(content)

    return True

def main():
    print("=" * 70)
    print("🔧 Fixing ALL Bridge Implementations (10/12 remaining)")
    print("=" * 70)
    print()

    success = 0
    fail = 0

    for pbc, config in BRIDGES_CONFIG.items():
        print(f"Fixing {pbc}-pbc...")

        if fix_bridge(pbc, config):
            print(f"  ✅ Fixed")
            success += 1
        else:
            print(f"  ❌ Failed")
            fail += 1

    print()
    print("=" * 70)
    print(f"Results: {success}/{len(BRIDGES_CONFIG)} bridges fixed")
    print(f"✅ Success: {success}")
    print(f"❌ Failed: {fail}")
    print("=" * 70)

    return 0 if fail == 0 else 1

if __name__ == '__main__':
    exit(main())
