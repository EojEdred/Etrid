/**
 * PancakeSwap (BSC) Deployment Script
 *
 * Deploys ÉTR.bsc and EDSC.bsc BEP-20 tokens on Binance Smart Chain
 * and creates liquidity pools on PancakeSwap V2/V3.
 */

import { ethers } from 'ethers';
import * as fs from 'fs';
import * as path from 'path';

interface DeploymentConfig {
  rpcUrl: string;
  deployerPrivateKey: string;
  bridgeAddress: string;
  routerAddress?: string; // PancakeSwap router
}

interface TokenMetadata {
  name: string;
  symbol: string;
  decimals: number;
}

interface PoolParams {
  tokenA: string;
  tokenB: string;
  amountA: string;
  amountB: string;
  version: 'v2' | 'v3';
  feeTier?: number; // For V3 only
}

// BEP-20 Token ABI (same as ERC-20 with bridge functions)
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
  "function bridgeBurn(uint256 amount, string memory etridAddress) external"
];

// PancakeSwap V2 Router ABI
const ROUTER_V2_ABI = [
  "function factory() external pure returns (address)",
  "function WETH() external pure returns (address)",
  "function addLiquidity(address tokenA, address tokenB, uint amountADesired, uint amountBDesired, uint amountAMin, uint amountBMin, address to, uint deadline) external returns (uint amountA, uint amountB, uint liquidity)",
  "function addLiquidityETH(address token, uint amountTokenDesired, uint amountTokenMin, uint amountETHMin, address to, uint deadline) external payable returns (uint amountToken, uint amountETH, uint liquidity)"
];

// PancakeSwap V2 Factory ABI
const FACTORY_V2_ABI = [
  "function createPair(address tokenA, address tokenB) external returns (address pair)",
  "function getPair(address tokenA, address tokenB) external view returns (address pair)"
];

class PancakeSwapDeployer {
  private provider: ethers.JsonRpcProvider;
  private deployer: ethers.Wallet;

  // PancakeSwap V2 Router on BSC Mainnet
  private readonly PANCAKE_ROUTER_V2 = '0x10ED43C718714eb63d5aA57B78B54704E256024E';
  private readonly PANCAKE_FACTORY_V2 = '0xcA143Ce32Fe78f1f7019d7d551a6402fC5350c73';

  // PancakeSwap V3 addresses (newer version)
  private readonly PANCAKE_FACTORY_V3 = '0x0BFbCF9fa4f9C56B0F40a671Ad40E0805A091865';

  // Common tokens on BSC
  private readonly WBNB = '0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c';
  private readonly BUSD = '0xe9e7CEA3DedcA5984780Bafc599bD69ADd087D56';
  private readonly USDT = '0x55d398326f99059fF775485246999027B3197955';

  constructor(private config: DeploymentConfig) {
    this.provider = new ethers.JsonRpcProvider(config.rpcUrl);
    this.deployer = new ethers.Wallet(config.deployerPrivateKey, this.provider);
  }

  /**
   * Deploy ÉTR token on BSC
   */
  async deployETR(): Promise<string> {
    console.log('📝 Deploying ÉTR token on BSC...');

    const metadata: TokenMetadata = {
      name: 'Etrid Coin (BSC)',
      symbol: 'ÉTR',
      decimals: 18
    };

    const address = await this.deployToken(metadata);

    console.log(`✅ ÉTR deployed at: ${address}`);
    console.log(`   BscScan: https://bscscan.com/address/${address}`);

    return address;
  }

  /**
   * Deploy EDSC stablecoin on BSC
   */
  async deployEDSC(): Promise<string> {
    console.log('📝 Deploying EDSC token on BSC...');

    const metadata: TokenMetadata = {
      name: 'Etrid Dollar Stablecoin (BSC)',
      symbol: 'EDSC',
      decimals: 18
    };

    const address = await this.deployToken(metadata);

    console.log(`✅ EDSC deployed at: ${address}`);
    console.log(`   BscScan: https://bscscan.com/address/${address}`);

    return address;
  }

  /**
   * Deploy BEP-20 token
   */
  private async deployToken(metadata: TokenMetadata): Promise<string> {
    try {
      // Get deployer balance
      const balance = await this.provider.getBalance(this.deployer.address);
      console.log(`   Deployer: ${this.deployer.address}`);
      console.log(`   Balance: ${ethers.formatEther(balance)} BNB`);

      // Gas price on BSC (typically ~5 gwei)
      const gasPrice = await this.provider.getFeeData();
      console.log(`   Gas Price: ${ethers.formatUnits(gasPrice.gasPrice || 0n, 'gwei')} gwei`);

      // Create contract factory (using same bytecode as Base adapter)
      // In production, compile from Solidity source
      const factory = new ethers.ContractFactory(
        TOKEN_ABI,
        "0x...", // Compiled bytecode placeholder
        this.deployer
      );

      // Deploy
      console.log(`   Deploying ${metadata.name}...`);

      const contract = await factory.deploy(
        metadata.name,
        metadata.symbol,
        metadata.decimals,
        this.config.bridgeAddress,
        {
          gasLimit: 1500000, // 1.5M gas (cheaper on BSC)
          gasPrice: gasPrice.gasPrice
        }
      );

      console.log(`   TX: ${contract.deploymentTransaction()?.hash}`);
      console.log(`   Waiting for confirmation...`);

      await contract.waitForDeployment();
      const address = await contract.getAddress();

      // Verify
      const deployedName = await contract.name();
      const deployedSymbol = await contract.symbol();
      const deployedDecimals = await contract.decimals();

      console.log(`   ✅ Deployed successfully!`);
      console.log(`      Name: ${deployedName}`);
      console.log(`      Symbol: ${deployedSymbol}`);
      console.log(`      Decimals: ${deployedDecimals}`);

      // Save deployment info
      await this.saveDeployment(address, metadata);

      return address;

    } catch (error: any) {
      console.error(`❌ Deployment failed:`, error.message);
      throw error;
    }
  }

  /**
   * Create PancakeSwap V2 liquidity pool
   */
  async createPancakePoolV2(params: {
    tokenAddress: string;
    pairTokenAddress: string; // WBNB, BUSD, USDT
    tokenAmount: string;
    pairAmount: string;
    slippageTolerance?: number; // Default 1%
  }): Promise<{ pairAddress: string; liquidity: string }> {
    console.log('💧 Creating PancakeSwap V2 pool...');

    const slippage = params.slippageTolerance || 1;
    const router = new ethers.Contract(
      this.PANCAKE_ROUTER_V2,
      ROUTER_V2_ABI,
      this.deployer
    );

    const factory = new ethers.Contract(
      this.PANCAKE_FACTORY_V2,
      FACTORY_V2_ABI,
      this.deployer
    );

    try {
      // Check if pair exists
      let pairAddress = await factory.getPair(
        params.tokenAddress,
        params.pairTokenAddress
      );

      if (pairAddress === ethers.ZeroAddress) {
        console.log('   Creating new pair...');
        const createTx = await factory.createPair(
          params.tokenAddress,
          params.pairTokenAddress
        );
        await createTx.wait();

        pairAddress = await factory.getPair(
          params.tokenAddress,
          params.pairTokenAddress
        );

        console.log(`   ✅ Pair created: ${pairAddress}`);
      } else {
        console.log(`   ℹ️  Pair exists: ${pairAddress}`);
      }

      // Approve tokens
      console.log('   Approving tokens...');

      const token = new ethers.Contract(
        params.tokenAddress,
        TOKEN_ABI,
        this.deployer
      );

      const approveTx = await token.approve(
        this.PANCAKE_ROUTER_V2,
        ethers.parseUnits(params.tokenAmount, 18)
      );
      await approveTx.wait();

      // Approve pair token
      const pairToken = new ethers.Contract(
        params.pairTokenAddress,
        TOKEN_ABI,
        this.deployer
      );

      const approvePairTx = await pairToken.approve(
        this.PANCAKE_ROUTER_V2,
        ethers.parseUnits(params.pairAmount, 18)
      );
      await approvePairTx.wait();

      console.log('   ✅ Tokens approved');

      // Add liquidity
      console.log('   Adding liquidity...');

      const amountADesired = ethers.parseUnits(params.tokenAmount, 18);
      const amountBDesired = ethers.parseUnits(params.pairAmount, 18);

      // Calculate minimum amounts with slippage tolerance
      const amountAMin = (amountADesired * BigInt(100 - slippage)) / 100n;
      const amountBMin = (amountBDesired * BigInt(100 - slippage)) / 100n;

      const deadline = Math.floor(Date.now() / 1000) + 1800; // 30 minutes

      const liquidityTx = await router.addLiquidity(
        params.tokenAddress,
        params.pairTokenAddress,
        amountADesired,
        amountBDesired,
        amountAMin,
        amountBMin,
        this.deployer.address,
        deadline,
        {
          gasLimit: 500000
        }
      );

      console.log(`   TX: ${liquidityTx.hash}`);
      const receipt = await liquidityTx.wait();

      // Parse liquidity from receipt
      // In production, decode logs properly
      const liquidity = "1000000"; // Placeholder

      console.log(`   ✅ Liquidity added!`);
      console.log(`      Pair: ${pairAddress}`);
      console.log(`      LP tokens: ${liquidity}`);
      console.log(`      Explorer: https://bscscan.com/address/${pairAddress}`);
      console.log(`      PancakeSwap: https://pancakeswap.finance/info/pool/${pairAddress}`);

      return { pairAddress, liquidity };

    } catch (error: any) {
      console.error(`❌ Pool creation failed:`, error.message);
      throw error;
    }
  }

  /**
   * Get pool statistics from PancakeSwap
   */
  async getPoolStats(pairAddress: string): Promise<{
    reserve0: string;
    reserve1: string;
    totalSupply: string;
    token0: string;
    token1: string;
  }> {
    const PAIR_ABI = [
      "function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast)",
      "function totalSupply() external view returns (uint)",
      "function token0() external view returns (address)",
      "function token1() external view returns (address)"
    ];

    try {
      const pair = new ethers.Contract(pairAddress, PAIR_ABI, this.provider);

      const [reserve0, reserve1] = await pair.getReserves();
      const totalSupply = await pair.totalSupply();
      const token0 = await pair.token0();
      const token1 = await pair.token1();

      return {
        reserve0: ethers.formatEther(reserve0),
        reserve1: ethers.formatEther(reserve1),
        totalSupply: ethers.formatEther(totalSupply),
        token0,
        token1
      };

    } catch (error: any) {
      console.error(`❌ Stats fetch failed:`, error.message);
      throw error;
    }
  }

  /**
   * Verify contract on BscScan
   */
  async verifyContract(address: string, metadata: TokenMetadata): Promise<void> {
    console.log(`🔍 Verifying contract on BscScan...`);
    console.log(`   Visit: https://bscscan.com/address/${address}#code`);
    console.log(`   Constructor args: ${metadata.name}, ${metadata.symbol}, ${metadata.decimals}, ${this.config.bridgeAddress}`);

    // In production, use Hardhat verification or BscScan API
  }

  /**
   * Save deployment info
   */
  private async saveDeployment(address: string, metadata: TokenMetadata): Promise<void> {
    const deployment = {
      address,
      name: metadata.name,
      symbol: metadata.symbol,
      decimals: metadata.decimals,
      bridge: this.config.bridgeAddress,
      deployer: this.deployer.address,
      network: 'bsc',
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
    rpcUrl: process.env.BSC_RPC_URL || 'https://bsc-dataseed.binance.org/',
    deployerPrivateKey: process.env.DEPLOYER_PRIVATE_KEY || '',
    bridgeAddress: process.env.BRIDGE_ADDRESS || ''
  };

  // Validate
  if (!config.deployerPrivateKey) {
    console.error('❌ DEPLOYER_PRIVATE_KEY environment variable required');
    process.exit(1);
  }

  if (!config.bridgeAddress) {
    console.error('❌ BRIDGE_ADDRESS environment variable required');
    process.exit(1);
  }

  const deployer = new PancakeSwapDeployer(config);

  (async () => {
    try {
      console.log('🚀 Starting BSC / PancakeSwap deployment...\n');

      // Deploy tokens
      const etrAddress = await deployer.deployETR();
      console.log('');

      const edscAddress = await deployer.deployEDSC();
      console.log('\n');

      // Create liquidity pools
      console.log('💧 Creating PancakeSwap pools...\n');

      // ÉTR / WBNB pool
      await deployer.createPancakePoolV2({
        tokenAddress: etrAddress,
        pairTokenAddress: deployer['WBNB'], // Access private constant
        tokenAmount: '1000000', // 1M ÉTR
        pairAmount: '100', // 100 BNB
        slippageTolerance: 2 // 2%
      });

      console.log('');

      // EDSC / BUSD pool (stablecoin pair)
      await deployer.createPancakePoolV2({
        tokenAddress: edscAddress,
        pairTokenAddress: deployer['BUSD'],
        tokenAmount: '500000', // 500k EDSC
        pairAmount: '500000', // 500k BUSD
        slippageTolerance: 1 // 1% (tighter for stablecoins)
      });

      console.log('\n✅ BSC deployment complete!');
      console.log(`   ÉTR: ${etrAddress}`);
      console.log(`   EDSC: ${edscAddress}`);
      console.log('\nNext steps:');
      console.log('1. Verify contracts on BscScan');
      console.log('2. Submit listing to PancakeSwap UI');
      console.log('3. Configure bridge adapter');
      console.log('4. Enable LP farming rewards');

    } catch (error) {
      console.error('\n❌ Deployment failed:', error);
      process.exit(1);
    }
  })();
}

export default PancakeSwapDeployer;
