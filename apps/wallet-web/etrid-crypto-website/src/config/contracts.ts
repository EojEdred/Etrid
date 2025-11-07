/**
 * Smart Contract Addresses for ETH PBC
 *
 * TODO: Update these addresses after contracts are deployed to ETH PBC
 */

export const CONTRACTS = {
  MASTERCHEF: '0x0000000000000000000000000000000000000000', // TODO: Update after MasterChef deployment
  // Add other contract addresses here as needed
} as const

export type ContractName = keyof typeof CONTRACTS
