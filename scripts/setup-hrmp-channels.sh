#!/bin/bash
# Setup HRMP Channels between FlareChain and ETH-PBC
# Run this script after both parachains are registered

set -e

# Configuration
FLARECHAIN_PARA_ID=2000
ETH_PBC_PARA_ID=2001
MAX_CAPACITY=1000
MAX_MESSAGE_SIZE=10240

echo "🔗 Setting up HRMP channels between FlareChain and ETH-PBC"
echo "FlareChain Para ID: $FLARECHAIN_PARA_ID"
echo "ETH-PBC Para ID: $ETH_PBC_PARA_ID"
echo ""

# Step 1: ETH-PBC -> FlareChain
echo "Step 1: Opening channel ETH-PBC -> FlareChain..."
echo "From ETH-PBC, requesting to open channel to FlareChain..."
polkadot-js-api tx.hrmp.hrmpInitOpenChannel \
    --recipient $FLARECHAIN_PARA_ID \
    --proposedMaxCapacity $MAX_CAPACITY \
    --proposedMaxMessageSize $MAX_MESSAGE_SIZE \
    --seed //EthPbc

echo "Waiting for FlareChain to accept..."
sleep 12  # Wait for block

echo "From FlareChain, accepting channel from ETH-PBC..."
polkadot-js-api tx.hrmp.hrmpAcceptOpenChannel \
    --sender $ETH_PBC_PARA_ID \
    --seed //FlareChain

echo "✅ Channel ETH-PBC -> FlareChain opened"
echo ""

# Step 2: FlareChain -> ETH-PBC
echo "Step 2: Opening channel FlareChain -> ETH-PBC..."
echo "From FlareChain, requesting to open channel to ETH-PBC..."
polkadot-js-api tx.hrmp.hrmpInitOpenChannel \
    --recipient $ETH_PBC_PARA_ID \
    --proposedMaxCapacity $MAX_CAPACITY \
    --proposedMaxMessageSize $MAX_MESSAGE_SIZE \
    --seed //FlareChain

echo "Waiting for ETH-PBC to accept..."
sleep 12  # Wait for block

echo "From ETH-PBC, accepting channel from FlareChain..."
polkadot-js-api tx.hrmp.hrmpAcceptOpenChannel \
    --sender $FLARECHAIN_PARA_ID \
    --seed //EthPbc

echo "✅ Channel FlareChain -> ETH-PBC opened"
echo ""

# Step 3: Verify channels
echo "Step 3: Verifying channels..."
echo "Checking ETH-PBC -> FlareChain..."
polkadot-js-api query.hrmp.hrmpChannels $ETH_PBC_PARA_ID $FLARECHAIN_PARA_ID

echo ""
echo "Checking FlareChain -> ETH-PBC..."
polkadot-js-api query.hrmp.hrmpChannels $FLARECHAIN_PARA_ID $ETH_PBC_PARA_ID

echo ""
echo "🎉 HRMP channels successfully set up!"
echo ""
echo "Next steps:"
echo "1. Deploy contracts that use custom precompiles"
echo "2. Test oracle queries via XCM"
echo "3. Monitor XCM message delivery"
