/// Chain configuration for Ëtrid multichain ecosystem
///
/// Defines endpoints and parameters for FlareChain and all 13 PBCs

class ChainConfig {
  final String id;
  final String name;
  final String rpcEndpoint;
  final String wsEndpoint;
  final int ss58Prefix;
  final String symbol;
  final int decimals;
  final ChainType type;

  const ChainConfig({
    required this.id,
    required this.name,
    required this.rpcEndpoint,
    required this.wsEndpoint,
    required this.ss58Prefix,
    required this.symbol,
    required this.decimals,
    required this.type,
  });
}

enum ChainType {
  relay,  // FlareChain
  pbc,    // Partition Burst Chain
}

class EtridChainRegistry {
  // FlareChain (Main settlement layer)
  static const flareChain = ChainConfig(
    id: 'flarechain',
    name: 'FlareChain',
    rpcEndpoint: 'http://127.0.0.1:9944',
    wsEndpoint: 'ws://127.0.0.1:9944',
    ss58Prefix: 42,
    symbol: 'ÉTR',
    decimals: 18,
    type: ChainType.relay,
  );

  // Bitcoin PBC
  static const btcPbc = ChainConfig(
    id: 'btc-pbc',
    name: 'Bitcoin PBC',
    rpcEndpoint: 'http://127.0.0.1:8000',
    wsEndpoint: 'ws://127.0.0.1:8000',
    ss58Prefix: 42,
    symbol: 'BTC',
    decimals: 8,
    type: ChainType.pbc,
  );

  // Ethereum PBC
  static const ethPbc = ChainConfig(
    id: 'eth-pbc',
    name: 'Ethereum PBC',
    rpcEndpoint: 'http://127.0.0.1:8001',
    wsEndpoint: 'ws://127.0.0.1:8001',
    ss58Prefix: 42,
    symbol: 'ETH',
    decimals: 18,
    type: ChainType.pbc,
  );

  // Dogecoin PBC
  static const dogePbc = ChainConfig(
    id: 'doge-pbc',
    name: 'Dogecoin PBC',
    rpcEndpoint: 'http://127.0.0.1:8002',
    wsEndpoint: 'ws://127.0.0.1:8002',
    ss58Prefix: 42,
    symbol: 'DOGE',
    decimals: 8,
    type: ChainType.pbc,
  );

  // Solana PBC
  static const solPbc = ChainConfig(
    id: 'sol-pbc',
    name: 'Solana PBC',
    rpcEndpoint: 'http://127.0.0.1:8003',
    wsEndpoint: 'ws://127.0.0.1:8003',
    ss58Prefix: 42,
    symbol: 'SOL',
    decimals: 9,
    type: ChainType.pbc,
  );

  // Stellar PBC
  static const xlmPbc = ChainConfig(
    id: 'xlm-pbc',
    name: 'Stellar PBC',
    rpcEndpoint: 'http://127.0.0.1:8004',
    wsEndpoint: 'ws://127.0.0.1:8004',
    ss58Prefix: 42,
    symbol: 'XLM',
    decimals: 7,
    type: ChainType.pbc,
  );

  // XRP PBC
  static const xrpPbc = ChainConfig(
    id: 'xrp-pbc',
    name: 'XRP PBC',
    rpcEndpoint: 'http://127.0.0.1:8005',
    wsEndpoint: 'ws://127.0.0.1:8005',
    ss58Prefix: 42,
    symbol: 'XRP',
    decimals: 6,
    type: ChainType.pbc,
  );

  // BNB PBC
  static const bnbPbc = ChainConfig(
    id: 'bnb-pbc',
    name: 'BNB PBC',
    rpcEndpoint: 'http://127.0.0.1:8006',
    wsEndpoint: 'ws://127.0.0.1:8006',
    ss58Prefix: 42,
    symbol: 'BNB',
    decimals: 18,
    type: ChainType.pbc,
  );

  // Tron PBC
  static const trxPbc = ChainConfig(
    id: 'trx-pbc',
    name: 'Tron PBC',
    rpcEndpoint: 'http://127.0.0.1:8007',
    wsEndpoint: 'ws://127.0.0.1:8007',
    ss58Prefix: 42,
    symbol: 'TRX',
    decimals: 6,
    type: ChainType.pbc,
  );

  // Cardano PBC
  static const adaPbc = ChainConfig(
    id: 'ada-pbc',
    name: 'Cardano PBC',
    rpcEndpoint: 'http://127.0.0.1:8008',
    wsEndpoint: 'ws://127.0.0.1:8008',
    ss58Prefix: 42,
    symbol: 'ADA',
    decimals: 6,
    type: ChainType.pbc,
  );

  // Chainlink PBC
  static const linkPbc = ChainConfig(
    id: 'link-pbc',
    name: 'Chainlink PBC',
    rpcEndpoint: 'http://127.0.0.1:8009',
    wsEndpoint: 'ws://127.0.0.1:8009',
    ss58Prefix: 42,
    symbol: 'LINK',
    decimals: 18,
    type: ChainType.pbc,
  );

  // Polygon PBC
  static const maticPbc = ChainConfig(
    id: 'matic-pbc',
    name: 'Polygon PBC',
    rpcEndpoint: 'http://127.0.0.1:8010',
    wsEndpoint: 'ws://127.0.0.1:8010',
    ss58Prefix: 42,
    symbol: 'MATIC',
    decimals: 18,
    type: ChainType.pbc,
  );

  // Tether (Smart Contract) PBC
  static const scUsdtPbc = ChainConfig(
    id: 'sc-usdt-pbc',
    name: 'Tether PBC',
    rpcEndpoint: 'http://127.0.0.1:8011',
    wsEndpoint: 'ws://127.0.0.1:8011',
    ss58Prefix: 42,
    symbol: 'USDT',
    decimals: 6,
    type: ChainType.pbc,
  );

  // EDSC (Ëtrid Dollar Stablecoin) PBC
  static const edscPbc = ChainConfig(
    id: 'edsc-pbc',
    name: 'EDSC PBC',
    rpcEndpoint: 'http://127.0.0.1:8012',
    wsEndpoint: 'ws://127.0.0.1:8012',
    ss58Prefix: 42,
    symbol: 'EDSC',
    decimals: 18,
    type: ChainType.pbc,
  );

  /// All chains in the Ëtrid ecosystem
  static const allChains = [
    flareChain,
    btcPbc,
    ethPbc,
    dogePbc,
    solPbc,
    xlmPbc,
    xrpPbc,
    bnbPbc,
    trxPbc,
    adaPbc,
    linkPbc,
    maticPbc,
    scUsdtPbc,
    edscPbc,
  ];

  /// Get chain configuration by ID
  static ChainConfig? getChainById(String id) {
    try {
      return allChains.firstWhere((chain) => chain.id == id);
    } catch (e) {
      return null;
    }
  }

  /// Get all PBC chains
  static List<ChainConfig> get allPbcs =>
      allChains.where((chain) => chain.type == ChainType.pbc).toList();

  /// Get relay chain (FlareChain)
  static ChainConfig get relayChain => flareChain;
}
