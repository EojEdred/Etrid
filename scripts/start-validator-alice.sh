#!/bin/bash
set -e

BASEDIR="/tmp/testchain-alice"

echo "Generating network key..."
./flarechain-node key generate-node-key --base-path="$BASEDIR"

echo "Inserting ASF validator key..."
./flarechain-node key insert \
  --base-path="$BASEDIR" \
  --chain=local \
  --key-type=asfk \
  --scheme=sr25519 \
  --suri="//Alice"

echo "Starting validator Alice..."
exec ./flarechain-node \
  --chain=local \
  --alice \
  --base-path="$BASEDIR"
