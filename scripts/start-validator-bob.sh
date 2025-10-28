#!/bin/bash
set -e

BASEDIR="/tmp/testchain-bob"

echo "Generating network key..."
./flarechain-node key generate-node-key --base-path="$BASEDIR"

echo "Inserting ASF validator key..."
./flarechain-node key insert \
  --base-path="$BASEDIR" \
  --chain=local \
  --key-type=asfk \
  --scheme=sr25519 \
  --suri="//Bob"

echo "Starting validator Bob..."
exec ./flarechain-node \
  --chain=local \
  --bob \
  --base-path="$BASEDIR"
