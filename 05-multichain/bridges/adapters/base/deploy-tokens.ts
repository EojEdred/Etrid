/**
 * Base L2 Token Deployment Script
 *
 * Deploys ÉTR.e and EDSC.e ERC-20 tokens on Base L2
 * with bridge-compatible minting/burning functionality.
 */

import { ethers } from 'ethers';
import * as fs from 'fs';
import * as path from 'path';

interface DeploymentConfig {
  rpcUrl: string;
  deployerPrivateKey: string;
  bridgeAddress: string;
  multisigAddress: string;
}

interface TokenMetadata {
  name: string;
  symbol: string;
  decimals: number;
  initialSupply: string;
}

// ERC-20 Bridgeable Token ABI (simplified, would use compiled contract)
const TOKEN_ABI = [
  "constructor(string name, string symbol, uint8 decimals, address bridge)",
  "function name() view returns (string)",
  "function symbol() view returns (string)",
  "function decimals() view returns (uint8)",
  "function totalSupply() view returns (uint256)",
  "function balanceOf(address) view returns (uint256)",
  "function transfer(address to, uint256 amount) returns (bool)",
  "function approve(address spender, uint256 amount) returns (bool)",
  "function transferFrom(address from, address to, uint256 amount) returns (bool)",
  "function bridgeMint(address to, uint256 amount, bytes32 txHash) external",
  "function bridgeBurn(uint256 amount, string memory etridAddress) external",
  "event Transfer(address indexed from, address indexed to, uint256 value)",
  "event Approval(address indexed owner, address indexed spender, uint256 value)",
  "event BridgeMint(address indexed to, uint256 amount, bytes32 txHash)",
  "event BridgeBurn(address indexed from, uint256 amount, string etridAddress)"
];

// Simplified bytecode (in production, compile from Solidity source)
const TOKEN_BYTECODE = `
pragma solidity ^0.8.20;

contract BridgeableERC20 {
    string public name;
    string public symbol;
    uint8 public decimals;
    uint256 public totalSupply;
    address public bridge;

    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    mapping(bytes32 => bool) public processedTransactions;

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
    event BridgeMint(address indexed to, uint256 amount, bytes32 txHash);
    event BridgeBurn(address indexed from, uint256 amount, string etridAddress);

    constructor(
        string memory _name,
        string memory _symbol,
        uint8 _decimals,
        address _bridge
    ) {
        name = _name;
        symbol = _symbol;
        decimals = _decimals;
        bridge = _bridge;
    }

    modifier onlyBridge() {
        require(msg.sender == bridge, "Only bridge can call");
        _;
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        require(balanceOf[msg.sender] >= amount, "Insufficient balance");
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        emit Transfer(msg.sender, to, amount);
        return true;
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        require(balanceOf[from] >= amount, "Insufficient balance");
        require(allowance[from][msg.sender] >= amount, "Allowance exceeded");

        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        allowance[from][msg.sender] -= amount;

        emit Transfer(from, to, amount);
        return true;
    }

    function bridgeMint(
        address to,
        uint256 amount,
        bytes32 txHash
    ) external onlyBridge {
        require(!processedTransactions[txHash], "Transaction already processed");

        processedTransactions[txHash] = true;
        totalSupply += amount;
        balanceOf[to] += amount;

        emit BridgeMint(to, amount, txHash);
        emit Transfer(address(0), to, amount);
    }

    function bridgeBurn(uint256 amount, string memory etridAddress) external {
        require(balanceOf[msg.sender] >= amount, "Insufficient balance");
        require(bytes(etridAddress).length > 0, "Invalid Etrid address");

        balanceOf[msg.sender] -= amount;
        totalSupply -= amount;

        emit BridgeBurn(msg.sender, amount, etridAddress);
        emit Transfer(msg.sender, address(0), amount);
    }

    function setBridge(address newBridge) external onlyBridge {
        bridge = newBridge;
    }
}
`;

class BaseTokenDeployer {
  private provider: ethers.JsonRpcProvider;
  private deployer: ethers.Wallet;

  constructor(private config: DeploymentConfig) {
    this.provider = new ethers.JsonRpcProvider(config.rpcUrl);
    this.deployer = new ethers.Wallet(config.deployerPrivateKey, this.provider);
  }

  /**
   * Deploy ÉTR token on Base L2
   */
  async deployETR(): Promise<string> {
    console.log('📝 Deploying ÉTR token on Base L2...');

    const metadata: TokenMetadata = {
      name: 'Etrid Coin (Base)',
      symbol: 'ÉTR',
      decimals: 18,
      initialSupply: '0' // Minted via bridge only
    };

    const address = await this.deployToken(metadata);

    console.log(`✅ ÉTR deployed at: ${address}`);
    console.log(`   Explorer: https://basescan.org/address/${address}`);

    return address;
  }

  /**
   * Deploy EDSC stablecoin on Base L2
   */
  async deployEDSC(): Promise<string> {
    console.log('📝 Deploying EDSC token on Base L2...');

    const metadata: TokenMetadata = {
      name: 'Etrid Dollar Stablecoin (Base)',
      symbol: 'EDSC',
      decimals: 18,
      initialSupply: '0' // Minted via bridge only
    };

    const address = await this.deployToken(metadata);

    console.log(`✅ EDSC deployed at: ${address}`);
    console.log(`   Explorer: https://basescan.org/address/${address}`);

    return address;
  }

  /**
   * Deploy a bridgeable ERC-20 token
   */
  private async deployToken(metadata: TokenMetadata): Promise<string> {
    try {
      // Get current deployer balance
      const balance = await this.provider.getBalance(this.deployer.address);
      console.log(`   Deployer: ${this.deployer.address}`);
      console.log(`   Balance: ${ethers.formatEther(balance)} ETH`);

      // Estimate gas
      const gasPrice = await this.provider.getFeeData();
      console.log(`   Gas Price: ${ethers.formatUnits(gasPrice.gasPrice || 0n, 'gwei')} gwei`);

      // In production, load compiled bytecode from artifacts
      // For now, using ethers ContractFactory pattern

      // Create contract factory (simplified - would use actual compiled bytecode)
      const factory = new ethers.ContractFactory(
        TOKEN_ABI,
        "0x..." + TOKEN_BYTECODE, // Placeholder - compile from Solidity first
        this.deployer
      );

      // Deploy contract
      console.log(`   Deploying ${metadata.name}...`);

      const contract = await factory.deploy(
        metadata.name,
        metadata.symbol,
        metadata.decimals,
        this.config.bridgeAddress,
        {
          gasLimit: 2000000 // 2M gas
        }
      );

      console.log(`   TX: ${contract.deploymentTransaction()?.hash}`);
      console.log(`   Waiting for confirmation...`);

      // Wait for deployment
      await contract.waitForDeployment();
      const address = await contract.getAddress();

      // Verify deployment
      const deployedName = await contract.name();
      const deployedSymbol = await contract.symbol();
      const deployedDecimals = await contract.decimals();

      console.log(`   ✅ Deployed successfully!`);
      console.log(`      Name: ${deployedName}`);
      console.log(`      Symbol: ${deployedSymbol}`);
      console.log(`      Decimals: ${deployedDecimals}`);
      console.log(`      Bridge: ${this.config.bridgeAddress}`);

      // Save deployment info
      await this.saveDeployment(address, metadata);

      return address;

    } catch (error: any) {
      console.error(`❌ Deployment failed:`, error.message);
      throw error;
    }
  }

  /**
   * Create Uniswap V3 pool on Base
   */
  async createUniswapPool(params: {
    tokenAddress: string;
    pairTokenAddress: string; // WETH, USDC, etc.
    feeTier: number; // 500, 3000, 10000 (0.05%, 0.3%, 1%)
    initialPrice: string;
  }): Promise<string> {
    console.log('💧 Creating Uniswap V3 pool on Base...');

    // Uniswap V3 Factory on Base
    const FACTORY_ADDRESS = '0x33128a8fC17869897dcE68Ed026d694621f6FDfD';
    const FACTORY_ABI = [
      "function createPool(address tokenA, address tokenB, uint24 fee) external returns (address pool)",
      "function getPool(address tokenA, address tokenB, uint24 fee) view returns (address pool)"
    ];

    const factory = new ethers.Contract(
      FACTORY_ADDRESS,
      FACTORY_ABI,
      this.deployer
    );

    try {
      // Check if pool already exists
      const existingPool = await factory.getPool(
        params.tokenAddress,
        params.pairTokenAddress,
        params.feeTier
      );

      if (existingPool !== ethers.ZeroAddress) {
        console.log(`   ℹ️  Pool already exists: ${existingPool}`);
        return existingPool;
      }

      // Create new pool
      console.log(`   Creating pool: ${params.tokenAddress} / ${params.pairTokenAddress}`);
      console.log(`   Fee tier: ${params.feeTier / 10000}%`);

      const tx = await factory.createPool(
        params.tokenAddress,
        params.pairTokenAddress,
        params.feeTier
      );

      console.log(`   TX: ${tx.hash}`);
      const receipt = await tx.wait();

      // Get pool address from event
      const poolAddress = await factory.getPool(
        params.tokenAddress,
        params.pairTokenAddress,
        params.feeTier
      );

      console.log(`   ✅ Pool created: ${poolAddress}`);
      console.log(`      Explorer: https://basescan.org/address/${poolAddress}`);

      // Initialize pool with price (requires position manager)
      // TODO: Call initialize() and mint initial position

      return poolAddress;

    } catch (error: any) {
      console.error(`❌ Pool creation failed:`, error.message);
      throw error;
    }
  }

  /**
   * Verify contract on BaseScan
   */
  async verifyContract(address: string, metadata: TokenMetadata): Promise<void> {
    console.log(`🔍 Verifying contract on BaseScan...`);

    // In production, use Hardhat verification plugin or API
    // Simplified for demo

    console.log(`   Visit: https://basescan.org/address/${address}#code`);
    console.log(`   Upload source code and constructor args`);
    console.log(`   Constructor args: ${metadata.name}, ${metadata.symbol}, ${metadata.decimals}, ${this.config.bridgeAddress}`);
  }

  /**
   * Save deployment information to file
   */
  private async saveDeployment(address: string, metadata: TokenMetadata): Promise<void> {
    const deployment = {
      address,
      name: metadata.name,
      symbol: metadata.symbol,
      decimals: metadata.decimals,
      bridge: this.config.bridgeAddress,
      deployer: this.deployer.address,
      network: 'base',
      timestamp: new Date().toISOString()
    };

    const outputPath = path.join(__dirname, 'deployments.json');

    let deployments: any[] = [];
    if (fs.existsSync(outputPath)) {
      deployments = JSON.parse(fs.readFileSync(outputPath, 'utf-8'));
    }

    deployments.push(deployment);
    fs.writeFileSync(outputPath, JSON.stringify(deployments, null, 2));

    console.log(`   💾 Deployment info saved to ${outputPath}`);
  }
}

// CLI entry point
if (require.main === module) {
  const config: DeploymentConfig = {
    rpcUrl: process.env.BASE_RPC_URL || 'https://mainnet.base.org',
    deployerPrivateKey: process.env.DEPLOYER_PRIVATE_KEY || '',
    bridgeAddress: process.env.BRIDGE_ADDRESS || '',
    multisigAddress: process.env.MULTISIG_ADDRESS || ''
  };

  // Validate config
  if (!config.deployerPrivateKey) {
    console.error('❌ DEPLOYER_PRIVATE_KEY environment variable required');
    process.exit(1);
  }

  if (!config.bridgeAddress) {
    console.error('❌ BRIDGE_ADDRESS environment variable required');
    process.exit(1);
  }

  const deployer = new BaseTokenDeployer(config);

  (async () => {
    try {
      console.log('🚀 Starting Base L2 token deployment...\n');

      // Deploy tokens
      const etrAddress = await deployer.deployETR();
      console.log('');

      const edscAddress = await deployer.deployEDSC();
      console.log('');

      // Verify contracts
      await deployer.verifyContract(etrAddress, {
        name: 'Etrid Coin (Base)',
        symbol: 'ÉTR',
        decimals: 18,
        initialSupply: '0'
      });

      await deployer.verifyContract(edscAddress, {
        name: 'Etrid Dollar Stablecoin (Base)',
        symbol: 'EDSC',
        decimals: 18,
        initialSupply: '0'
      });

      console.log('\n✅ Base L2 deployment complete!');
      console.log(`   ÉTR: ${etrAddress}`);
      console.log(`   EDSC: ${edscAddress}`);
      console.log('\nNext steps:');
      console.log('1. Verify contracts on BaseScan');
      console.log('2. Create Uniswap V3 pools');
      console.log('3. Seed initial liquidity');
      console.log('4. Configure bridge adapter');

    } catch (error) {
      console.error('\n❌ Deployment failed:', error);
      process.exit(1);
    }
  })();
}

export default BaseTokenDeployer;
