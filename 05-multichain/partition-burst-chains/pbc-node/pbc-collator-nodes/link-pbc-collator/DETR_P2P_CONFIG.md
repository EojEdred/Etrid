# DETR P2P Configuration Guide for LINK-PBC Collator

This document describes how to configure the DETR P2P networking layer for the LINK (Chainlink) PBC Collator.

## Features

The LINK-PBC collator now includes full DETR P2P integration with:

1. **PeerId Identity Remapping** - Automatic remapping from socket-derived temporary IDs to real cryptographic identities when receiving Announce messages
2. **Public IP Auto-Detection** - Automatically detects your public IP using STUN protocol for proper NAT traversal
3. **Automatic Reconnection** - Maintains connections with known peers, automatically reconnecting after transient network failures
4. **Background Maintenance** - All maintenance tasks (DHT, discovery, reconnection) started with a single `start_all_maintenance()` call
5. **Encryption** - Full encryption via aecomms using X25519 + ChaCha20-Poly1305

## Environment Variables

### DETR_P2P_NODE_IDENTITY

**Type:** String (64-character hex)
**Required:** No
**Default:** Auto-generated on first run

Your node's cryptographic identity (32 bytes as hex string). This identity is used for:
- Kademlia DHT routing
- Peer-to-peer authentication
- Block announcement signatures

```bash
export DETR_P2P_NODE_IDENTITY="a1b2c3d4e5f6789012345678901234567890123456789012345678901234567890"
```

**How to generate a persistent identity:**

When you first run the collator without this variable set, it will generate a new identity and print it to the logs:

```
🔑 Generated new node identity: a1b2c3...
   Set DETR_P2P_NODE_IDENTITY=a1b2c3... to persist this identity
```

Copy that hex string and set it as an environment variable to maintain the same identity across restarts.

### DETR_P2P_LISTEN_ADDR

**Type:** Socket Address (IP:PORT)
**Required:** No
**Default:** `0.0.0.0:30333`

The local address to listen on for incoming P2P connections.

```bash
export DETR_P2P_LISTEN_ADDR="0.0.0.0:30333"
```

**Common configurations:**

- Development (local only): `127.0.0.1:30333`
- Production (all interfaces): `0.0.0.0:30333`
- Custom port: `0.0.0.0:9999`

### DETR_P2P_ANNOUNCE_IP

**Type:** IP Address (IPv4 or IPv6)
**Required:** No (auto-detected if not set)
**Default:** Auto-detected via STUN

The public IP address to announce to other peers. This is crucial for NAT traversal.

```bash
export DETR_P2P_ANNOUNCE_IP="203.0.113.45"
```

**When to set this manually:**

- When auto-detection fails (firewall blocks STUN)
- When running behind NAT without STUN access
- When you have multiple network interfaces and want to specify which one

**When to leave it unset:**

- When running on a cloud VM with a public IP (auto-detection works)
- When running in a properly configured network with STUN access

The auto-detection will try:
1. STUN protocol (Google's public STUN servers)
2. Fallback to listen address if detection fails

### DETR_P2P_BOOTSTRAP_PEERS

**Type:** Comma-separated list of `<peer_id_hex>@<address>` entries
**Required:** No (but recommended for production)
**Default:** Empty (standalone mode)

List of bootstrap peers to connect to on startup.

```bash
export DETR_P2P_BOOTSTRAP_PEERS="a1b2c3d4e5f6789012345678901234567890123456789012345678901234567890@192.168.1.10:30333,f1e2d3c4b5a6798012345678901234567890123456789012345678901234567890@192.168.1.11:30333"
```

**Format:**
```
<peer_id_1>@<ip_1>:<port_1>,<peer_id_2>@<ip_2>:<port_2>,...
```

Where:
- `<peer_id_N>` is the 64-character hex node identity
- `<ip_N>` is the IP address (IPv4 or IPv6)
- `<port_N>` is the port number

**Example with 3 validators:**

```bash
export DETR_P2P_BOOTSTRAP_PEERS="\
abcd1234567890abcd1234567890abcd1234567890abcd1234567890abcd1234@192.168.1.10:30333,\
ef123456789012ef123456789012ef123456789012ef123456789012ef123456@192.168.1.11:30333,\
56789abcdef01256789abcdef01256789abcdef01256789abcdef012567890ab@192.168.1.12:30333"
```

## Complete Configuration Examples

### Example 1: Development (Single Node)

```bash
# Generate a new identity (will be printed on first run)
./link-pbc-collator

# On subsequent runs, use the generated identity
export DETR_P2P_NODE_IDENTITY="a1b2c3d4e5f6..."
export DETR_P2P_LISTEN_ADDR="127.0.0.1:30333"
./link-pbc-collator
```

### Example 2: Production (3-Node Validator Network)

**Validator 1 (192.168.1.10):**
```bash
export DETR_P2P_NODE_IDENTITY="abcd1234567890abcd1234567890abcd1234567890abcd1234567890abcd1234"
export DETR_P2P_LISTEN_ADDR="0.0.0.0:30333"
export DETR_P2P_ANNOUNCE_IP="192.168.1.10"
export DETR_P2P_BOOTSTRAP_PEERS="ef123456789012ef123456789012ef123456789012ef123456789012ef123456@192.168.1.11:30333,56789abcdef01256789abcdef01256789abcdef01256789abcdef012567890ab@192.168.1.12:30333"
./link-pbc-collator
```

**Validator 2 (192.168.1.11):**
```bash
export DETR_P2P_NODE_IDENTITY="ef123456789012ef123456789012ef123456789012ef123456789012ef123456"
export DETR_P2P_LISTEN_ADDR="0.0.0.0:30333"
export DETR_P2P_ANNOUNCE_IP="192.168.1.11"
export DETR_P2P_BOOTSTRAP_PEERS="abcd1234567890abcd1234567890abcd1234567890abcd1234567890abcd1234@192.168.1.10:30333,56789abcdef01256789abcdef01256789abcdef01256789abcdef012567890ab@192.168.1.12:30333"
./link-pbc-collator
```

**Validator 3 (192.168.1.12):**
```bash
export DETR_P2P_NODE_IDENTITY="56789abcdef01256789abcdef01256789abcdef01256789abcdef012567890ab"
export DETR_P2P_LISTEN_ADDR="0.0.0.0:30333"
export DETR_P2P_ANNOUNCE_IP="192.168.1.12"
export DETR_P2P_BOOTSTRAP_PEERS="abcd1234567890abcd1234567890abcd1234567890abcd1234567890abcd1234@192.168.1.10:30333,ef123456789012ef123456789012ef123456789012ef123456789012ef123456@192.168.1.11:30333"
./link-pbc-collator
```

### Example 3: Cloud Deployment (AWS/GCP/Azure)

```bash
# Let auto-detection find the public IP
export DETR_P2P_NODE_IDENTITY="your_persistent_identity_hex"
export DETR_P2P_LISTEN_ADDR="0.0.0.0:30333"
# DETR_P2P_ANNOUNCE_IP will be auto-detected
export DETR_P2P_BOOTSTRAP_PEERS="bootstrap_peer_1@ip1:port1,bootstrap_peer_2@ip2:port2"
./link-pbc-collator
```

### Example 4: Docker Container

```bash
docker run \
  -e DETR_P2P_NODE_IDENTITY="your_persistent_identity_hex" \
  -e DETR_P2P_LISTEN_ADDR="0.0.0.0:30333" \
  -e DETR_P2P_ANNOUNCE_IP="$(curl -s https://api.ipify.org)" \
  -e DETR_P2P_BOOTSTRAP_PEERS="peer1@ip1:port1,peer2@ip2:port2" \
  -p 30333:30333 \
  link-pbc-collator:latest
```

## Monitoring and Diagnostics

### Log Messages

The collator logs will show P2P network activity:

```
🚀 Initializing DETR P2P Network for LINK-PBC Collator
   Listen address: 0.0.0.0:30333
   Node ID: abcd1234567890...
📢 Auto-detecting public IP address...
📢 Detected public IP via STUN: 203.0.113.45
📢 Announce address: 203.0.113.45:30333
✅ DETR P2P initialized successfully
🚀 Starting DETR P2P Network...
✅ P2P server started on 0.0.0.0:30333
🌐 Bootstrapping from 2 peers...
  ✅ Connected to 192.168.1.11:30333
  ✅ Connected to 192.168.1.12:30333
✅ Bootstrap complete
✅ All maintenance tasks started
📨 Message processor started
✅ DETR P2P Network fully operational
```

### P2P Network Stats

Every 30 seconds, the collator logs network statistics:

```
📊 P2P Network Stats: 20 connected peers | Local ID: abcd1234567890...
```

### Block Broadcasting

When blocks are produced and broadcast:

```
🔗 LINK-PBC: Block #42 produced with state root: 0x1234...
📦 Encoded block #42 (1024 bytes)
📢 Block #42 broadcast to 20 peers
```

## Troubleshooting

### Issue: "Could not detect public IP"

**Symptoms:**
```
⚠️ Could not detect public IP - announce address may be incorrect
⚠️ Set DETR_P2P_ANNOUNCE_IP environment variable to your public IP
```

**Solution:**
Manually set your public IP:
```bash
export DETR_P2P_ANNOUNCE_IP="203.0.113.45"
```

### Issue: "Failed to connect to bootstrap peer"

**Symptoms:**
```
⚠️ Failed to connect to bootstrap peer 192.168.1.11:30333: Connection refused
```

**Solutions:**
1. Check that the bootstrap peer is running
2. Verify firewall rules allow connections on port 30333
3. Confirm the IP address and port are correct
4. Check network connectivity: `ping 192.168.1.11`

### Issue: "No bootstrap peers configured"

**Symptoms:**
```
⚠️ No bootstrap peers configured - running in standalone mode
   Set DETR_P2P_BOOTSTRAP_PEERS to connect to other nodes
```

**Solution:**
This is informational - the node will run but won't connect to others. To join a network, configure bootstrap peers as shown above.

### Issue: Low peer count

**Symptoms:**
```
📊 P2P Network Stats: 2 connected peers | Local ID: abcd...
```

**Solutions:**
1. Check that DETR_P2P_ANNOUNCE_IP is set correctly (or auto-detected)
2. Verify that other nodes can reach your announce address
3. Check firewall rules allow incoming connections
4. Ensure NAT traversal is working (STUN detection succeeded)

## Security Considerations

1. **Node Identity:** Keep your `DETR_P2P_NODE_IDENTITY` secret and backed up. If lost, you'll need to reconfigure all peers.

2. **Encryption:** All P2P communication is encrypted using X25519 + ChaCha20-Poly1305. No additional TLS configuration needed.

3. **Firewall Rules:** Ensure port 30333 (or your custom port) is open for both incoming and outgoing TCP connections.

4. **Public IP Exposure:** Your announce IP will be shared with all peers. This is necessary for P2P connectivity.

## Advanced Configuration

### Custom Port Range

If running multiple collators on the same machine:

```bash
# Collator 1
export DETR_P2P_LISTEN_ADDR="0.0.0.0:30333"

# Collator 2
export DETR_P2P_LISTEN_ADDR="0.0.0.0:30334"

# Collator 3
export DETR_P2P_LISTEN_ADDR="0.0.0.0:30335"
```

### IPv6 Support

DETR P2P supports IPv6:

```bash
export DETR_P2P_LISTEN_ADDR="[::]:30333"
export DETR_P2P_ANNOUNCE_IP="2001:db8::1"
```

## Support

For issues or questions about DETR P2P configuration:
- Check the logs for detailed error messages
- Review this configuration guide
- Contact the Ëtrid development team
