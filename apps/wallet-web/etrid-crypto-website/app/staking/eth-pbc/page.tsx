import { MasterChefStaking } from '@/components/eth-pbc/MasterChefStaking'
import { Metadata } from 'next'

export const metadata: Metadata = {
  title: 'ETH PBC Staking | Ëtrid',
  description: 'Stake LP tokens on ETH Partition Burst Chain and earn ETR rewards',
}

export default function EthPbcStakingPage() {
  return (
    <div className="container mx-auto px-4 py-12">
      <div className="mb-12 text-center">
        <h1 className="text-4xl font-bold mb-4">ETH PBC MasterChef Staking</h1>
        <p className="text-xl text-muted-foreground max-w-2xl mx-auto">
          Stake your LP tokens on the Ethereum Partition Burst Chain and earn ETR rewards.
          Connect your wallet to get started.
        </p>
      </div>

      <MasterChefStaking
        poolId={0}
        // lpTokenAddress="0x..." // TODO: Add LP token address after deployment
        lpTokenSymbol="ETH-ETR LP"
      />

      <div className="mt-12 max-w-2xl mx-auto space-y-6">
        <div className="p-6 bg-muted rounded-lg">
          <h2 className="text-2xl font-bold mb-4">How it works</h2>
          <ol className="list-decimal list-inside space-y-2 text-muted-foreground">
            <li>Connect your wallet to the ETH PBC network</li>
            <li>Approve the MasterChef contract to spend your LP tokens</li>
            <li>Stake your LP tokens to start earning ETR rewards</li>
            <li>Harvest your rewards at any time</li>
            <li>Unstake your LP tokens whenever you want</li>
          </ol>
        </div>

        <div className="p-6 bg-muted rounded-lg">
          <h2 className="text-2xl font-bold mb-4">Important Information</h2>
          <ul className="list-disc list-inside space-y-2 text-muted-foreground">
            <li>
              <strong>Network:</strong> Make sure you're connected to the ETH PBC network
              (Chain ID: 8888)
            </li>
            <li>
              <strong>RPC Endpoint:</strong> ws://127.0.0.1:9944 (local development)
            </li>
            <li>
              <strong>Rewards:</strong> ETR tokens are distributed per block based on pool
              allocation
            </li>
            <li>
              <strong>Emergency Withdraw:</strong> Use only in emergencies - you will forfeit all
              pending rewards
            </li>
          </ul>
        </div>

        <div className="p-6 bg-yellow-500/10 border border-yellow-500/20 rounded-lg">
          <h3 className="text-lg font-bold mb-2 text-yellow-500">⚠️ Developer Note</h3>
          <p className="text-sm text-muted-foreground">
            The ETH PBC node must be running and the MasterChef contract must be deployed before
            you can use this interface. See the ETH_PBC_WEB_INTEGRATION_HANDOFF.md document for
            setup instructions.
          </p>
        </div>
      </div>
    </div>
  )
}
