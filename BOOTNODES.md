# Ëtrid FlareChain Mainnet Bootnodes

**Network:** FlareChain Mainnet
**Chain ID:** flarechain_mainnet
**P2P Port:** 30333

---

## Official Bootnodes

⚠️ **TO BE UPDATED**: Peer IDs will be generated when validators start for the first time.

Connect to these bootnodes to join the Ëtrid FlareChain mainnet:

### Validator 1: Gizzi (AI Overseer)
```
/ip4/64.181.215.19/tcp/30333/p2p/<PEER_ID_TBD>
```

### Validator 2: EojEdred (Founder)
```
/ip4/<IP_TBD>/tcp/30333/p2p/<PEER_ID_TBD>
```

### Validator 3: Backup Bootnode
```
/ip4/<IP_TBD>/tcp/30333/p2p/<PEER_ID_TBD>
```

---

## How to Get Peer IDs

Once validators are running, extract their peer IDs:

### Method 1: From Node Logs
```bash
# Check validator logs for Local node identity
sudo journalctl -u flarechain-validator | grep "Local node identity"
# Output: Local node identity is: 12D3KooW...
```

### Method 2: Using RPC
```bash
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method":"system_localPeerId"}' \
  http://localhost:9944
```

### Method 3: From Network Secret Key
If you have the network secret key file:
```bash
./flarechain-node key inspect-node-key \
  --file /var/lib/etrid/chains/flarechain_mainnet/network/secret_ed25519
```

---

## Updating Bootnode List

After getting peer IDs:

1. Update this BOOTNODES.md file with actual peer IDs
2. Update chain spec before distributing to network
3. Publish updated bootnodes on website
4. Add to GitHub repo and documentation

---

## Using Bootnodes

### Starting a Node with Bootnodes

```bash
./flarechain-node \
  --chain flarechain-mainnet-raw.json \
  --base-path /var/lib/etrid \
  --name "My Node" \
  --bootnodes /ip4/64.181.215.19/tcp/30333/p2p/PEER_ID_HERE
```

### Multiple Bootnodes (Recommended)

```bash
./flarechain-node \
  --chain flarechain-mainnet-raw.json \
  --base-path /var/lib/etrid \
  --name "My Node" \
  --bootnodes /ip4/64.181.215.19/tcp/30333/p2p/PEER_ID_1 \
  --bootnodes /ip4/IP_2/tcp/30333/p2p/PEER_ID_2 \
  --bootnodes /ip4/IP_3/tcp/30333/p2p/PEER_ID_3
```

---

## Adding Bootnodes to Chain Spec

Bootnodes can be embedded in the chain spec for automatic discovery:

```json
{
  "bootNodes": [
    "/ip4/64.181.215.19/tcp/30333/p2p/PEER_ID_1",
    "/ip4/IP_2/tcp/30333/p2p/PEER_ID_2",
    "/ip4/IP_3/tcp/30333/p2p/PEER_ID_3"
  ]
}
```

---

## Telemetry

Mainnet validators report to:
- **Polkadot Telemetry:** wss://telemetry.polkadot.io/submit/

View network stats at: https://telemetry.polkadot.io/

Search for "Ëtrid FlareChain Mainnet" or individual validator names.

---

## Public RPC Endpoints

**⚠️ TO BE ANNOUNCED**

Public RPC endpoints will be available after mainnet launch:

- WSS: `wss://rpc.etrid.io`
- HTTPS: `https://rpc.etrid.io`

**Note:** For production applications, run your own node for maximum reliability and decentralization.

---

## Network Information

- **Total Validators:** 21
- **Consensus:** AURA (block production) + GRANDPA (finality)
- **Block Time:** 6 seconds
- **Finality:** 2/3+ validators (15 of 21)
- **Token:** ETR (18 decimals)
- **Chain Type:** Live (Mainnet)

---

## Support & Resources

- **Website:** https://etrid.io
- **Documentation:** https://docs.etrid.io
- **GitHub:** https://github.com/EojEdred/Etrid
- **Whitepaper:** https://etrid.io/whitepaper

---

**Status:** 🔧 Bootnodes pending - Will be updated once validators are deployed

**Last Updated:** 2025-10-31
