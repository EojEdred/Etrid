/**
 * Reserve Vault Lending Example
 *
 * Demonstrates how to:
 * 1. Deposit multi-asset collateral
 * 2. Borrow against collateral
 * 3. Monitor vault health and collateral ratio
 * 4. Repay loans
 * 5. Withdraw collateral
 *
 * Ëtrid's Reserve Vault enables over-collateralized DeFi lending.
 */

import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';
import {
  ReserveVaultWrapper,
  VaultStatus,
} from '../src/wrappers/ReserveVaultWrapper';

async function main() {
  // 1. Connect to Ëtrid node
  console.log('Connecting to Ëtrid Primearc Core Chain...');
  const provider = new WsProvider('ws://localhost:9944');
  const api = await ApiPromise.create({ provider });
  console.log('✓ Connected to chain:', (await api.rpc.system.chain()).toString());

  // 2. Initialize account
  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri('//Alice');

  console.log(`\nAccount: ${alice.address}`);

  // 3. Create Reserve Vault wrapper
  const vault = new ReserveVaultWrapper(api);

  console.log('\n═══════════════════════════════════════════');
  console.log('Supported Collateral Assets');
  console.log('═══════════════════════════════════════════\n');

  try {
    const assets = await vault.getSupportedAssets();

    console.log('Asset      LTV Ratio  Liq. Threshold  Interest  Price');
    console.log('─'.repeat(65));

    assets.forEach((asset) => {
      const ltv = asset.ltvRatio / 100;
      const liqThreshold = asset.liquidationThreshold / 100;
      const interest = asset.interestRate / 100;
      const price = Number(asset.priceUSD) / 1e18;

      console.log(
        `${asset.symbol.padEnd(10)} ${ltv.toString().padStart(3)}%       ${liqThreshold.toString().padStart(3)}%           ${interest.toFixed(2)}%     $${price.toFixed(2)}`
      );
    });

    console.log('\nKey Terms:');
    console.log('  • LTV: Loan-to-Value - Max % you can borrow');
    console.log('  • Liq. Threshold: Collateral ratio before liquidation');
    console.log('  • Interest: Annual percentage rate on borrowed amounts');

  } catch (error) {
    console.log('⚠ Mock asset list (for demonstration)');
    console.log('\nExample assets: ETR, BTC, ETH, USDT, USDC');
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 1: Deposit Collateral');
  console.log('═══════════════════════════════════════════\n');

  try {
    console.log('Depositing 1000 ÉTR as collateral...');

    const depositResult = await vault.depositCollateral(
      alice,
      'ETR',
      1000n * 10n**18n
    );

    console.log(`\n✓ Collateral deposited!`);
    console.log(`  Asset: ${depositResult.assetId}`);
    console.log(`  Amount: ${depositResult.amount / 10n**18n} ÉTR`);
    console.log(`  Transaction: ${depositResult.txHash}`);

    console.log(`\n  New Vault State:`);
    console.log(`    Total Collateral: $${depositResult.newBalance.totalCollateralUSD / 10n**18n}`);
    console.log(`    Available to Borrow: $${depositResult.newBalance.availableToBorrow / 10n**18n}`);
    console.log(`    Status: ${depositResult.newBalance.status}`);

  } catch (error) {
    console.log('⚠ Mock collateral deposit (for demonstration)');
    console.log('  Deposited: 1000 ÉTR ($50,000)');
    console.log('  Available to borrow: ~$37,500 (75% LTV)');
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 2: Check Vault Health');
  console.log('═══════════════════════════════════════════\n');

  try {
    const balance = await vault.getVaultBalance(alice.address);

    console.log('Vault Overview:');
    console.log(`  Owner: ${balance.owner}`);
    console.log(`  Status: ${balance.status}`);
    console.log(`\n  Financials:`);
    console.log(`    Total Collateral: $${balance.totalCollateralUSD / 10n**18n}`);
    console.log(`    Total Debt: $${balance.totalDebtUSD / 10n**18n}`);
    console.log(`    Collateral Ratio: ${balance.collateralRatio / 100}%`);
    console.log(`\n  Thresholds:`);
    console.log(`    Minimum Ratio: ${balance.minCollateralRatio / 100}%`);
    console.log(`    Liquidation: ${balance.liquidationThreshold / 100}%`);
    console.log(`\n  Available:`);
    console.log(`    To Borrow: $${balance.availableToBorrow / 10n**18n}`);
    console.log(`    To Withdraw: $${balance.availableToWithdraw / 10n**18n}`);

    // Health indicator
    if (balance.collateralRatio > 0) {
      const healthFactor = balance.collateralRatio / balance.minCollateralRatio;
      console.log(`\n  Health Factor: ${healthFactor.toFixed(2)}`);

      if (healthFactor > 1.5) {
        console.log('  Status: 🟢 Very Safe');
      } else if (healthFactor > 1.2) {
        console.log('  Status: 🟡 Safe');
      } else if (healthFactor > 1.0) {
        console.log('  Status: 🟠 At Risk');
      } else {
        console.log('  Status: 🔴 Liquidatable!');
      }
    }

    console.log(`\n  Collateral Positions:`);
    balance.positions.forEach((pos, i) => {
      console.log(`    ${i + 1}. ${pos.assetId}: ${pos.amount} ($${pos.valueUSD / 10n**18n})`);
    });

  } catch (error) {
    console.log('⚠ Mock vault balance (for demonstration)');
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 3: Borrow Against Collateral');
  console.log('═══════════════════════════════════════════\n');

  try {
    console.log('Borrowing 500 USDT...');

    const borrowResult = await vault.borrow(
      alice,
      'USDT',
      500n * 10n**6n // 500 USDT (6 decimals)
    );

    console.log(`\n✓ Loan approved!`);
    console.log(`  Asset: ${borrowResult.assetId}`);
    console.log(`  Borrowed: ${borrowResult.amount / 10n**6n} USDT`);
    console.log(`  Transaction: ${borrowResult.txHash}`);

    console.log(`\n  Loan Details:`);
    console.log(`    Total Debt: $${borrowResult.newDebt / 10n**18n}`);
    console.log(`    New Collateral Ratio: ${borrowResult.newCollateralRatio / 100}%`);

    // Calculate interest
    const annualRate = 5.0; // 5% APR example
    const dailyInterest = borrowResult.newDebt * BigInt(Math.floor(annualRate * 100)) / 36500n;
    console.log(`    Daily Interest: ~$${dailyInterest / 10n**18n} (${annualRate}% APR)`);

  } catch (error) {
    console.log('⚠ Mock borrow operation (for demonstration)');
    console.log('  Borrowed: 500 USDT');
    console.log('  New ratio: 200% (safe)');
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 4: Monitor Collateral Ratio');
  console.log('═══════════════════════════════════════════\n');

  try {
    const { collateralRatio, status } = await vault.getCollateralRatio(alice.address);

    console.log(`Current Collateral Ratio: ${collateralRatio / 100}%`);
    console.log(`Vault Status: ${status}`);

    // Scenarios
    console.log('\nScenarios:');

    const scenarios = [
      { ratio: 250, status: 'Very Safe', action: 'Can borrow more' },
      { ratio: 180, status: 'Safe', action: 'Comfortable position' },
      { ratio: 140, status: 'At Risk', action: '⚠️  Add collateral or repay debt' },
      { ratio: 115, status: 'Liquidatable', action: '🚨 Immediate action required!' },
    ];

    scenarios.forEach((scenario) => {
      const current = scenario.ratio === Math.floor(collateralRatio / 100);
      const marker = current ? '→' : ' ';
      console.log(`  ${marker} ${scenario.ratio}%: ${scenario.status.padEnd(15)} ${scenario.action}`);
    });

  } catch (error) {
    console.log('⚠ Mock ratio check (for demonstration)');
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 5: Repay Loan');
  console.log('═══════════════════════════════════════════\n');

  try {
    console.log('Repaying 200 USDT...');

    const repayResult = await vault.repay(
      alice,
      'USDT',
      200n * 10n**6n
    );

    console.log(`\n✓ Payment successful!`);
    console.log(`  Asset: ${repayResult.assetId}`);
    console.log(`  Repaid: ${repayResult.amount / 10n**6n} USDT`);
    console.log(`  Transaction: ${repayResult.txHash}`);

    console.log(`\n  Updated Loan:`);
    console.log(`    Remaining Debt: $${repayResult.remainingDebt / 10n**18n}`);
    console.log(`    New Collateral Ratio: ${repayResult.newCollateralRatio / 100}%`);
    console.log(`    Improvement: Safer position!`);

  } catch (error) {
    console.log('⚠ Mock repay operation (for demonstration)');
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 6: Withdraw Collateral');
  console.log('═══════════════════════════════════════════\n');

  try {
    console.log('Withdrawing 100 ÉTR...');

    const withdrawResult = await vault.withdrawCollateral(
      alice,
      'ETR',
      100n * 10n**18n
    );

    console.log(`\n✓ Withdrawal successful!`);
    console.log(`  Asset: ${withdrawResult.assetId}`);
    console.log(`  Withdrawn: ${withdrawResult.amount / 10n**18n} ÉTR`);
    console.log(`  Transaction: ${withdrawResult.txHash}`);

    console.log(`\n  Remaining Vault:`);
    console.log(`    Collateral: $${withdrawResult.newBalance.totalCollateralUSD / 10n**18n}`);
    console.log(`    Ratio: ${withdrawResult.newBalance.collateralRatio / 100}%`);
    console.log(`    Status: ${withdrawResult.newBalance.status}`);

  } catch (error) {
    console.log('⚠ Mock withdraw operation (for demonstration)');
    console.log('  Note: Withdrawal maintains minimum collateral ratio');
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Liquidation Protection');
  console.log('═══════════════════════════════════════════\n');

  console.log('How Liquidation Works:');
  console.log('  1. Collateral ratio falls below threshold (e.g., 120%)');
  console.log('  2. Liquidators repay debt in exchange for collateral');
  console.log('  3. Liquidation penalty applied (e.g., 5-10%)');
  console.log('  4. Remaining collateral returned to owner');

  console.log('\nPreventing Liquidation:');
  console.log('  ✓ Maintain high collateral ratio (200%+ recommended)');
  console.log('  ✓ Monitor price volatility');
  console.log('  ✓ Set up price alerts');
  console.log('  ✓ Keep emergency funds ready');
  console.log('  ✓ Diversify collateral assets');

  console.log('\n═══════════════════════════════════════════');
  console.log('DeFi Strategies');
  console.log('═══════════════════════════════════════════\n');

  console.log('1. Leverage Trading:');
  console.log('   • Deposit ETR collateral');
  console.log('   • Borrow stablecoins');
  console.log('   • Buy more ETR');
  console.log('   • Repeat (carefully!)');

  console.log('\n2. Yield Farming:');
  console.log('   • Deposit blue-chip assets');
  console.log('   • Borrow at low interest');
  console.log('   • Farm higher yields');
  console.log('   • Profit from spread');

  console.log('\n3. Hedging:');
  console.log('   • Lock in crypto collateral');
  console.log('   • Borrow stablecoins for expenses');
  console.log('   • Avoid selling during dips');
  console.log('   • Tax-efficient liquidity');

  console.log('\n4. Arbitrage:');
  console.log('   • Borrow on Ëtrid (low fees)');
  console.log('   • Lend on other platforms');
  console.log('   • Profit from rate differences');
  console.log('   • Manage cross-platform risk');

  console.log('\n═══════════════════════════════════════════');
  console.log('Risk Management');
  console.log('═══════════════════════════════════════════\n');

  console.log('Collateral Risk:');
  console.log('  • Price volatility can reduce collateral value');
  console.log('  • Use stable assets for lower risk');
  console.log('  • Diversify across multiple assets');
  console.log('  • Monitor correlations');

  console.log('\nInterest Rate Risk:');
  console.log('  • Variable rates can increase costs');
  console.log('  • Plan for rate increases');
  console.log('  • Compare with other platforms');
  console.log('  • Consider fixed-rate alternatives');

  console.log('\nLiquidation Risk:');
  console.log('  • Most critical risk in volatile markets');
  console.log('  • Maintain buffer above minimum ratio');
  console.log('  • Set up automated alerts');
  console.log('  • Have repayment plan ready');

  console.log('\n═══════════════════════════════════════════');
  console.log('Best Practices');
  console.log('═══════════════════════════════════════════\n');

  console.log('1. Conservative Borrowing:');
  console.log('   • Never max out LTV ratio');
  console.log('   • Keep ratio above 200%');
  console.log('   • Leave room for volatility');
  console.log('   • Borrow less than available');

  console.log('\n2. Active Monitoring:');
  console.log('   • Check vault health daily');
  console.log('   • Watch collateral prices');
  console.log('   • Set up notifications');
  console.log('   • Review positions weekly');

  console.log('\n3. Emergency Planning:');
  console.log('   • Keep extra collateral ready');
  console.log('   • Have repayment funds accessible');
  console.log('   • Know your liquidation price');
  console.log('   • Test emergency procedures');

  console.log('\n4. Cost Optimization:');
  console.log('   • Repay high-interest loans first');
  console.log('   • Compound earnings back into vault');
  console.log('   • Minimize unnecessary transactions');
  console.log('   • Time operations for low gas fees');

  console.log('\n═══════════════════════════════════════════');
  console.log('Example Vault Lifecycle');
  console.log('═══════════════════════════════════════════\n');

  console.log('Day 1: Setup');
  console.log('  → Deposit 1000 ÉTR ($50,000)');
  console.log('  → Collateral ratio: ∞% (no debt)');

  console.log('\nDay 2: Borrow');
  console.log('  → Borrow $20,000 USDT');
  console.log('  → Collateral ratio: 250%');
  console.log('  → Status: Safe');

  console.log('\nDay 30: Market Dip');
  console.log('  → ÉTR price drops 20%');
  console.log('  → Collateral value: $40,000');
  console.log('  → Collateral ratio: 200%');
  console.log('  → Action: Add $5,000 more collateral');

  console.log('\nDay 60: Recovery');
  console.log('  → ÉTR price recovers');
  console.log('  → Repay $10,000');
  console.log('  → Ratio improves to 450%');
  console.log('  → Consider taking profits');

  console.log('\nDay 90: Close Position');
  console.log('  → Repay remaining debt');
  console.log('  → Withdraw all collateral');
  console.log('  → Net profit: Yield - Interest');

  // Cleanup
  await api.disconnect();
  console.log('\n✓ Disconnected from chain');
}

// Run example
main()
  .then(() => {
    console.log('\n✅ Example completed successfully!');
    process.exit(0);
  })
  .catch((error) => {
    console.error('\n❌ Example failed:', error);
    process.exit(1);
  });
