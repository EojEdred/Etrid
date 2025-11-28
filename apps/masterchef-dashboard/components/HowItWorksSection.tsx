'use client';

import { useState } from 'react';

export function HowItWorksSection() {
  const [activeTab, setActiveTab] = useState<'overview' | 'steps' | 'faq'>('overview');

  return (
    <div className="bg-white rounded-lg shadow-sm overflow-hidden">
      {/* Header */}
      <div className="bg-gradient-to-r from-blue-600 to-indigo-600 px-6 py-4">
        <h2 className="text-xl font-bold text-white">📚 How LP Staking Works</h2>
        <p className="text-blue-100 text-sm mt-1">
          Earn ÉTR rewards by providing liquidity and staking your LP tokens
        </p>
      </div>

      {/* Tab Navigation */}
      <div className="border-b border-gray-200">
        <div className="flex space-x-1 px-6">
          <button
            onClick={() => setActiveTab('overview')}
            className={`px-4 py-3 text-sm font-medium border-b-2 transition-colors ${
              activeTab === 'overview'
                ? 'border-blue-600 text-blue-600'
                : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
            }`}
          >
            Overview
          </button>
          <button
            onClick={() => setActiveTab('steps')}
            className={`px-4 py-3 text-sm font-medium border-b-2 transition-colors ${
              activeTab === 'steps'
                ? 'border-blue-600 text-blue-600'
                : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
            }`}
          >
            Step-by-Step Guide
          </button>
          <button
            onClick={() => setActiveTab('faq')}
            className={`px-4 py-3 text-sm font-medium border-b-2 transition-colors ${
              activeTab === 'faq'
                ? 'border-blue-600 text-blue-600'
                : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
            }`}
          >
            FAQ
          </button>
        </div>
      </div>

      {/* Content */}
      <div className="p-6">
        {activeTab === 'overview' && (
          <div className="space-y-6">
            <div>
              <h3 className="text-lg font-semibold text-gray-900 mb-3">
                What is LP Staking?
              </h3>
              <p className="text-gray-700 leading-relaxed">
                LP (Liquidity Provider) staking allows you to earn ÉTR rewards by providing
                liquidity to trading pairs on PancakeSwap and staking your LP tokens in our
                MasterChef contract. You earn rewards continuously as blocks are produced on
                the BSC network.
              </p>
            </div>

            <div className="bg-blue-50 border border-blue-200 rounded-lg p-4">
              <h4 className="font-semibold text-blue-900 mb-2 flex items-center">
                <span className="text-2xl mr-2">💡</span>
                Key Benefits
              </h4>
              <ul className="space-y-2 text-blue-800 text-sm">
                <li className="flex items-start">
                  <span className="mr-2">•</span>
                  <span><strong>Passive Income:</strong> Earn ÉTR rewards automatically every block (~3 seconds)</span>
                </li>
                <li className="flex items-start">
                  <span className="mr-2">•</span>
                  <span><strong>High APR:</strong> Up to 85%+ annual percentage returns on select pools</span>
                </li>
                <li className="flex items-start">
                  <span className="mr-2">•</span>
                  <span><strong>Trading Fees:</strong> Also earn 0.25% of all trades in your liquidity pool</span>
                </li>
                <li className="flex items-start">
                  <span className="mr-2">•</span>
                  <span><strong>Flexible:</strong> Withdraw your stake anytime without lock-up periods</span>
                </li>
              </ul>
            </div>

            <div className="bg-yellow-50 border border-yellow-200 rounded-lg p-4">
              <h4 className="font-semibold text-yellow-900 mb-2 flex items-center">
                <span className="text-2xl mr-2">⚠️</span>
                Important to Know
              </h4>
              <ul className="space-y-2 text-yellow-800 text-sm">
                <li className="flex items-start">
                  <span className="mr-2">•</span>
                  <span><strong>Impermanent Loss:</strong> LP value can decrease if token prices diverge significantly</span>
                </li>
                <li className="flex items-start">
                  <span className="mr-2">•</span>
                  <span><strong>Smart Contract Risk:</strong> Always audit contracts and use trusted platforms</span>
                </li>
                <li className="flex items-start">
                  <span className="mr-2">•</span>
                  <span><strong>Gas Fees:</strong> Each transaction (approve, stake, claim, unstake) requires BNB for gas</span>
                </li>
              </ul>
            </div>

            <div>
              <h3 className="text-lg font-semibold text-gray-900 mb-3">
                Simple Visual Flow
              </h3>
              <div className="bg-gray-50 rounded-lg p-6">
                <div className="flex flex-col md:flex-row items-center justify-between text-center space-y-4 md:space-y-0">
                  <div className="flex-1">
                    <div className="bg-white rounded-full w-16 h-16 flex items-center justify-center mx-auto mb-2 shadow-md text-2xl">
                      💰
                    </div>
                    <p className="font-medium text-gray-900">Add Liquidity</p>
                    <p className="text-xs text-gray-500 mt-1">On PancakeSwap</p>
                  </div>
                  <div className="text-gray-400 text-2xl hidden md:block">→</div>
                  <div className="flex-1">
                    <div className="bg-white rounded-full w-16 h-16 flex items-center justify-center mx-auto mb-2 shadow-md text-2xl">
                      🎟️
                    </div>
                    <p className="font-medium text-gray-900">Receive LP Tokens</p>
                    <p className="text-xs text-gray-500 mt-1">Proof of liquidity</p>
                  </div>
                  <div className="text-gray-400 text-2xl hidden md:block">→</div>
                  <div className="flex-1">
                    <div className="bg-white rounded-full w-16 h-16 flex items-center justify-center mx-auto mb-2 shadow-md text-2xl">
                      🔒
                    </div>
                    <p className="font-medium text-gray-900">Stake LP Tokens</p>
                    <p className="text-xs text-gray-500 mt-1">In MasterChef</p>
                  </div>
                  <div className="text-gray-400 text-2xl hidden md:block">→</div>
                  <div className="flex-1">
                    <div className="bg-white rounded-full w-16 h-16 flex items-center justify-center mx-auto mb-2 shadow-md text-2xl">
                      🎁
                    </div>
                    <p className="font-medium text-gray-900">Earn ÉTR</p>
                    <p className="text-xs text-gray-500 mt-1">Every block</p>
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}

        {activeTab === 'steps' && (
          <div className="space-y-6">
            <p className="text-gray-600 mb-6">
              Follow these steps to start earning ÉTR rewards through LP staking:
            </p>

            {/* Step 1 */}
            <div className="border-l-4 border-blue-500 pl-4">
              <div className="flex items-start">
                <span className="bg-blue-500 text-white font-bold rounded-full w-8 h-8 flex items-center justify-center mr-3 flex-shrink-0">
                  1
                </span>
                <div className="flex-1">
                  <h3 className="font-semibold text-gray-900 mb-2">Connect Your Wallet</h3>
                  <p className="text-gray-700 text-sm mb-3">
                    Click the "Connect Wallet" button on this page. You can use MetaMask, Trust Wallet,
                    or any WalletConnect-compatible wallet. Make sure you're connected to BSC (BNB Chain).
                  </p>
                  <div className="bg-gray-50 rounded p-3 text-xs text-gray-600">
                    <strong>Network Settings:</strong> BSC Mainnet (Chain ID: 56)
                  </div>
                </div>
              </div>
            </div>

            {/* Step 2 */}
            <div className="border-l-4 border-blue-500 pl-4">
              <div className="flex items-start">
                <span className="bg-blue-500 text-white font-bold rounded-full w-8 h-8 flex items-center justify-center mr-3 flex-shrink-0">
                  2
                </span>
                <div className="flex-1">
                  <h3 className="font-semibold text-gray-900 mb-2">Add Liquidity on PancakeSwap</h3>
                  <p className="text-gray-700 text-sm mb-3">
                    Go to <a href="https://pancakeswap.finance/add" target="_blank" rel="noopener noreferrer"
                    className="text-blue-600 hover:underline">PancakeSwap Liquidity</a> and select your desired
                    pair (e.g., ÉTR-BNB). You'll need equal value of both tokens. Add liquidity and receive LP tokens.
                  </p>
                  <div className="bg-gray-50 rounded p-3 text-xs text-gray-600 space-y-1">
                    <div><strong>Example:</strong> To add $1000 liquidity to ÉTR-BNB:</div>
                    <div>• You need $500 worth of ÉTR + $500 worth of BNB</div>
                    <div>• PancakeSwap gives you ÉTR-BNB LP tokens representing your share</div>
                  </div>
                </div>
              </div>
            </div>

            {/* Step 3 */}
            <div className="border-l-4 border-blue-500 pl-4">
              <div className="flex items-start">
                <span className="bg-blue-500 text-white font-bold rounded-full w-8 h-8 flex items-center justify-center mr-3 flex-shrink-0">
                  3
                </span>
                <div className="flex-1">
                  <h3 className="font-semibold text-gray-900 mb-2">Choose a Pool</h3>
                  <p className="text-gray-700 text-sm mb-3">
                    Return to this dashboard and browse the available LP pools. Compare APR% (annual return),
                    TVL (total staked), and reward allocations to find the best pool for your strategy.
                  </p>
                  <div className="bg-gray-50 rounded p-3 text-xs text-gray-600">
                    <strong>Pro Tip:</strong> Higher APR pools may have higher risk. Diversify across multiple pools.
                  </div>
                </div>
              </div>
            </div>

            {/* Step 4 */}
            <div className="border-l-4 border-blue-500 pl-4">
              <div className="flex items-start">
                <span className="bg-blue-500 text-white font-bold rounded-full w-8 h-8 flex items-center justify-center mr-3 flex-shrink-0">
                  4
                </span>
                <div className="flex-1">
                  <h3 className="font-semibold text-gray-900 mb-2">Approve LP Token Spending</h3>
                  <p className="text-gray-700 text-sm mb-3">
                    Click "Approve" on your chosen pool. This grants the MasterChef contract permission to
                    interact with your LP tokens. This is a one-time transaction per LP token type.
                  </p>
                  <div className="bg-gray-50 rounded p-3 text-xs text-gray-600">
                    <strong>Gas Cost:</strong> ~$0.50 - $2.00 in BNB depending on network congestion
                  </div>
                </div>
              </div>
            </div>

            {/* Step 5 */}
            <div className="border-l-4 border-blue-500 pl-4">
              <div className="flex items-start">
                <span className="bg-blue-500 text-white font-bold rounded-full w-8 h-8 flex items-center justify-center mr-3 flex-shrink-0">
                  5
                </span>
                <div className="flex-1">
                  <h3 className="font-semibold text-gray-900 mb-2">Stake Your LP Tokens</h3>
                  <p className="text-gray-700 text-sm mb-3">
                    After approval, enter the amount of LP tokens you want to stake and click "Stake".
                    Confirm the transaction in your wallet. Once confirmed, you immediately start earning ÉTR rewards!
                  </p>
                  <div className="bg-gray-50 rounded p-3 text-xs text-gray-600">
                    <strong>Rewards Start:</strong> Instantly upon transaction confirmation (~3 seconds)
                  </div>
                </div>
              </div>
            </div>

            {/* Step 6 */}
            <div className="border-l-4 border-green-500 pl-4">
              <div className="flex items-start">
                <span className="bg-green-500 text-white font-bold rounded-full w-8 h-8 flex items-center justify-center mr-3 flex-shrink-0">
                  6
                </span>
                <div className="flex-1">
                  <h3 className="font-semibold text-gray-900 mb-2">Claim Rewards & Manage Stake</h3>
                  <p className="text-gray-700 text-sm mb-3">
                    Watch your rewards accumulate in real-time! You can claim your ÉTR rewards anytime
                    (click "Harvest"), add more LP tokens, or withdraw your stake whenever you want.
                  </p>
                  <div className="bg-green-50 rounded p-3 text-xs text-gray-700 space-y-1">
                    <div><strong>Options:</strong></div>
                    <div>• <strong>Harvest:</strong> Claim accumulated ÉTR rewards (stake remains)</div>
                    <div>• <strong>Compound:</strong> Convert rewards to LP and re-stake (coming soon)</div>
                    <div>• <strong>Withdraw:</strong> Unstake LP tokens (can re-stake anytime)</div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}

        {activeTab === 'faq' && (
          <div className="space-y-5">
            <div className="border-b border-gray-200 pb-4">
              <h3 className="font-semibold text-gray-900 mb-2">
                Q: What's the difference between liquidity mining and staking?
              </h3>
              <p className="text-gray-700 text-sm">
                <strong>Liquidity Mining (this):</strong> You provide liquidity on PancakeSwap, receive LP tokens,
                and stake them here to earn ÉTR rewards.<br/>
                <strong>Regular Staking:</strong> You lock up ÉTR tokens directly to support the network.
              </p>
            </div>

            <div className="border-b border-gray-200 pb-4">
              <h3 className="font-semibold text-gray-900 mb-2">
                Q: What is impermanent loss and how do I avoid it?
              </h3>
              <p className="text-gray-700 text-sm">
                Impermanent loss occurs when the price ratio of your two LP tokens changes significantly.
                The loss is "impermanent" because it only becomes permanent when you withdraw. <strong>To minimize:</strong>
                <br/>• Choose stable pairs (ÉTR-BUSD instead of ÉTR-BNB)
                <br/>• Hold long-term through price fluctuations
                <br/>• Ensure rewards outweigh potential IL (our high APRs help!)
              </p>
            </div>

            <div className="border-b border-gray-200 pb-4">
              <h3 className="font-semibold text-gray-900 mb-2">
                Q: Can I lose my staked LP tokens?
              </h3>
              <p className="text-gray-700 text-sm">
                Your LP tokens are <strong>never locked</strong> and can be withdrawn anytime. However, the
                <strong> value</strong> of your LP tokens can fluctuate due to:
                <br/>• Price changes of the underlying tokens (impermanent loss)
                <br/>• Smart contract bugs (use audited contracts only)
                <br/>• Rug pulls (only use reputable platforms like PancakeSwap)
              </p>
            </div>

            <div className="border-b border-gray-200 pb-4">
              <h3 className="font-semibold text-gray-900 mb-2">
                Q: How often should I claim rewards?
              </h3>
              <p className="text-gray-700 text-sm">
                It depends on gas costs vs. reward size. <strong>Strategy:</strong>
                <br/>• <strong>Large stakes ($10k+):</strong> Claim daily to reinvest
                <br/>• <strong>Medium stakes ($1k-$10k):</strong> Claim weekly
                <br/>• <strong>Small stakes (&lt;$1k):</strong> Claim monthly or when rewards exceed $50
                <br/>Always ensure gas fees ($1-3) don't eat too much of your rewards!
              </p>
            </div>

            <div className="border-b border-gray-200 pb-4">
              <h3 className="font-semibold text-gray-900 mb-2">
                Q: What happens to my rewards if I don't claim them?
              </h3>
              <p className="text-gray-700 text-sm">
                Your rewards accumulate forever and are <strong>never lost</strong>. They sit in the contract
                waiting for you to claim. However, <strong>unclaimed rewards don't compound</strong> - only
                staked LP tokens earn new rewards.
              </p>
            </div>

            <div className="border-b border-gray-200 pb-4">
              <h3 className="font-semibold text-gray-900 mb-2">
                Q: Why do I need BNB in my wallet?
              </h3>
              <p className="text-gray-700 text-sm">
                BNB is the native token of BSC and required to pay <strong>gas fees</strong> for all transactions:
                <br/>• Approving LP tokens: ~$1-2
                <br/>• Staking LP tokens: ~$1-2
                <br/>• Claiming rewards: ~$1-2
                <br/>• Withdrawing stake: ~$1-2
                <br/><strong>Keep at least $10 worth of BNB</strong> in your wallet for smooth operations.
              </p>
            </div>

            <div>
              <h3 className="font-semibold text-gray-900 mb-2">
                Q: Are the smart contracts audited?
              </h3>
              <p className="text-gray-700 text-sm">
                Our MasterChef contract is based on <strong>SushiSwap's audited MasterChef V2</strong>,
                battle-tested with billions in TVL. We recommend always:
                <br/>• Verifying contract addresses (shown on each pool card)
                <br/>• Starting with small amounts to test
                <br/>• Reading our security documentation
                <br/>• Never sharing your private keys or seed phrase
              </p>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
