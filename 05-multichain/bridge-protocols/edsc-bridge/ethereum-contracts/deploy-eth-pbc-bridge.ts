/**
 * Deployment script for ETH PBC Bridge Adapter
 *
 * Deploys the ETHPBCBridgeAdapter contract to ETH PBC chain
 * Integrates with EDSC TokenMessenger for cross-chain transfers
 */

import { ethers } from 'hardhat'
import { ETHPBCBridgeAdapter } from '../typechain-types'

async function main() {
  console.log('Deploying ETH PBC Bridge Adapter...')

  // Get deployer
  const [deployer] = await ethers.getSigners()
  console.log('Deploying with account:', deployer.address)
  console.log('Account balance:', (await deployer.getBalance()).toString())

  // Contract addresses (update these after deployment)
  const EDSC_TOKEN_ADDRESS = process.env.EDSC_TOKEN_ADDRESS || '0x0000000000000000000000000000000000000000'
  const ETR_TOKEN_ADDRESS = process.env.ETR_TOKEN_ADDRESS || '0x0000000000000000000000000000000000000000'
  const TOKEN_MESSENGER_ADDRESS = process.env.TOKEN_MESSENGER_ADDRESS || '0x0000000000000000000000000000000000000000'
  const MASTERCHEF_ADDRESS = process.env.MASTERCHEF_ADDRESS || '0x0000000000000000000000000000000000000000'

  // Validate addresses
  if (
    EDSC_TOKEN_ADDRESS === '0x0000000000000000000000000000000000000000' ||
    ETR_TOKEN_ADDRESS === '0x0000000000000000000000000000000000000000' ||
    TOKEN_MESSENGER_ADDRESS === '0x0000000000000000000000000000000000000000' ||
    MASTERCHEF_ADDRESS === '0x0000000000000000000000000000000000000000'
  ) {
    console.error('ERROR: Please set all contract addresses in .env file')
    console.error('Required: EDSC_TOKEN_ADDRESS, ETR_TOKEN_ADDRESS, TOKEN_MESSENGER_ADDRESS, MASTERCHEF_ADDRESS')
    process.exit(1)
  }

  console.log('Configuration:')
  console.log('  EDSC Token:', EDSC_TOKEN_ADDRESS)
  console.log('  ETR Token:', ETR_TOKEN_ADDRESS)
  console.log('  TokenMessenger:', TOKEN_MESSENGER_ADDRESS)
  console.log('  MasterChef:', MASTERCHEF_ADDRESS)

  // Deploy contract
  const ETHPBCBridgeAdapterFactory = await ethers.getContractFactory('ETHPBCBridgeAdapter')
  const bridgeAdapter = (await ETHPBCBridgeAdapterFactory.deploy(
    EDSC_TOKEN_ADDRESS,
    ETR_TOKEN_ADDRESS,
    TOKEN_MESSENGER_ADDRESS,
    MASTERCHEF_ADDRESS
  )) as ETHPBCBridgeAdapter

  await bridgeAdapter.deployed()

  console.log('ETHPBCBridgeAdapter deployed to:', bridgeAdapter.address)

  // Verify deployment
  console.log('\nVerifying deployment...')
  const edscToken = await bridgeAdapter.edscToken()
  const etrToken = await bridgeAdapter.etrToken()
  const tokenMessenger = await bridgeAdapter.tokenMessenger()
  const masterChef = await bridgeAdapter.masterChef()

  console.log('Verification:')
  console.log('  EDSC Token:', edscToken)
  console.log('  ETR Token:', etrToken)
  console.log('  TokenMessenger:', tokenMessenger)
  console.log('  MasterChef:', masterChef)

  // Save deployment info
  const deploymentInfo = {
    network: (await ethers.provider.getNetwork()).name,
    chainId: (await ethers.provider.getNetwork()).chainId,
    bridgeAdapter: bridgeAdapter.address,
    edscToken: edscToken,
    etrToken: etrToken,
    tokenMessenger: tokenMessenger,
    masterChef: masterChef,
    deployer: deployer.address,
    timestamp: new Date().toISOString(),
    blockNumber: await ethers.provider.getBlockNumber(),
  }

  console.log('\nDeployment Info:')
  console.log(JSON.stringify(deploymentInfo, null, 2))

  // Write to file
  const fs = require('fs')
  const path = require('path')
  const deploymentPath = path.join(__dirname, '../deployments', `eth-pbc-bridge-${deploymentInfo.chainId}.json`)

  // Create deployments directory if it doesn't exist
  const deploymentsDir = path.join(__dirname, '../deployments')
  if (!fs.existsSync(deploymentsDir)) {
    fs.mkdirSync(deploymentsDir, { recursive: true })
  }

  fs.writeFileSync(deploymentPath, JSON.stringify(deploymentInfo, null, 2))
  console.log(`\nDeployment info saved to: ${deploymentPath}`)

  console.log('\nNext Steps:')
  console.log('1. Update web app with bridge adapter address:', bridgeAdapter.address)
  console.log('2. Configure TokenMessenger to accept bridge adapter as authorized caller')
  console.log('3. Fund bridge adapter with gas tokens if needed')
  console.log('4. Test bridging flow on testnet before production deployment')
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error)
    process.exit(1)
  })
