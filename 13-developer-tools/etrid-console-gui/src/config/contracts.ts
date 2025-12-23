/**
 * Smart Contract Addresses for ËTRID EVM
 *
 * TODO: Update these addresses after contracts are deployed to the target EVM chain
 */

export const CONTRACTS = {
  MASTERCHEF: '0x0000000000000000000000000000000000000000', // TODO: Update after MasterChef deployment
  // Add other contract addresses here as needed
} as const

export type ContractName = keyof typeof CONTRACTS
