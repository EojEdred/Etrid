# ËTRID Network Endpoints

This document is the **single source of truth** for all ËTRID network endpoints, APIs, and services.

**Domain**: `etrid.org`
**Contact**: `gizzi_io@proton.me`

---

## Quick Reference

| Service | Endpoint |
|---------|----------|
| Main Website | `https://etrid.org` |
| Documentation | `https://docs.etrid.org` |
| Web Wallet | `https://wallet.etrid.org` |
| Block Explorer | `https://explorer.etrid.org` |
| Governance | `https://governance.etrid.org` |
| Main RPC | `https://rpc.etrid.org/primearc` |
| Main WebSocket | `wss://ws.etrid.org/primearc` |
| REST API | `https://api.etrid.org/v1` |

---

## RPC Endpoints

All RPC endpoints use path-based routing under a single domain.

### HTTP RPC

| Chain | Endpoint | Chain ID |
|-------|----------|----------|
| **Primearc Core** | `https://rpc.etrid.org/primearc` | 1337 |
| ETH PBC | `https://rpc.etrid.org/eth-pbc` | 1338 |
| BTC PBC | `https://rpc.etrid.org/btc-pbc` | 1339 |
| SOL PBC | `https://rpc.etrid.org/sol-pbc` | 1340 |
| ADA PBC | `https://rpc.etrid.org/ada-pbc` | 1341 |
| DOGE PBC | `https://rpc.etrid.org/doge-pbc` | 1342 |
| MATIC PBC | `https://rpc.etrid.org/matic-pbc` | 1343 |
| BNB PBC | `https://rpc.etrid.org/bnb-pbc` | 1344 |
| LINK PBC | `https://rpc.etrid.org/link-pbc` | 1345 |
| XRP PBC | `https://rpc.etrid.org/xrp-pbc` | 1346 |
| TRX PBC | `https://rpc.etrid.org/trx-pbc` | 1347 |
| XLM PBC | `https://rpc.etrid.org/xlm-pbc` | 1348 |
| USDT PBC | `https://rpc.etrid.org/usdt-pbc` | 1349 |
| EDSC PBC | `https://rpc.etrid.org/edsc-pbc` | 1350 |

### WebSocket RPC

| Chain | Endpoint |
|-------|----------|
| **Primearc Core** | `wss://ws.etrid.org/primearc` |
| ETH PBC | `wss://ws.etrid.org/eth-pbc` |
| BTC PBC | `wss://ws.etrid.org/btc-pbc` |
| SOL PBC | `wss://ws.etrid.org/sol-pbc` |
| ADA PBC | `wss://ws.etrid.org/ada-pbc` |
| DOGE PBC | `wss://ws.etrid.org/doge-pbc` |
| MATIC PBC | `wss://ws.etrid.org/matic-pbc` |
| BNB PBC | `wss://ws.etrid.org/bnb-pbc` |
| LINK PBC | `wss://ws.etrid.org/link-pbc` |
| XRP PBC | `wss://ws.etrid.org/xrp-pbc` |
| TRX PBC | `wss://ws.etrid.org/trx-pbc` |
| XLM PBC | `wss://ws.etrid.org/xlm-pbc` |
| USDT PBC | `wss://ws.etrid.org/usdt-pbc` |
| EDSC PBC | `wss://ws.etrid.org/edsc-pbc` |

---

## REST API

Base URL: `https://api.etrid.org`

### API Versions

| Version | Base Path | Status |
|---------|-----------|--------|
| v1 | `/v1` | Current |

### Endpoints

#### Wallet API
```
GET  /v1/wallet/balance/:address
GET  /v1/wallet/transactions/:address
POST /v1/wallet/send
GET  /v1/wallet/nonce/:address
```

#### Bridge API
```
GET  /v1/bridge/status
GET  /v1/bridge/attestation/:hash
POST /v1/bridge/relay
GET  /v1/bridge/pending
```

#### Lightning-Bloc API
```
GET  /v1/lightning/channels
GET  /v1/lightning/channel/:id
POST /v1/lightning/open
POST /v1/lightning/close
POST /v1/lightning/pay
```

#### Staking API
```
GET  /v1/staking/validators
GET  /v1/staking/validator/:address
GET  /v1/staking/nominations/:address
POST /v1/staking/stake
POST /v1/staking/unstake
```

#### Governance API
```
GET  /v1/governance/proposals
GET  /v1/governance/proposal/:id
POST /v1/governance/vote
GET  /v1/governance/directors
```

---

## Web Applications

| Application | URL | Description |
|-------------|-----|-------------|
| Main Portal | `https://app.etrid.org` | Unified dashboard |
| Web Wallet | `https://wallet.etrid.org` | Browser wallet |
| Block Explorer | `https://explorer.etrid.org` | Transaction explorer |
| Governance UI | `https://governance.etrid.org` | Voting interface |
| Faucet | `https://faucet.etrid.org` | Testnet tokens |
| Documentation | `https://docs.etrid.org` | Developer docs |
| Blog | `https://blog.etrid.org` | Updates & news |

---

## Infrastructure (Internal)

| Service | URL | Purpose |
|---------|-----|---------|
| Grafana | `https://grafana.etrid.org` | Monitoring dashboards |
| Telemetry | `https://telemetry.etrid.org` | Network statistics |
| Prometheus | Internal only | Metrics collection |

---

## SDK Connection Examples

### JavaScript/TypeScript

```typescript
import { ApiPromise, WsProvider } from '@polkadot/api';

// Connect to Primearc Core Chain
const provider = new WsProvider('wss://ws.etrid.org/primearc');
const api = await ApiPromise.create({ provider });

// Connect to ETH PBC
const ethProvider = new WsProvider('wss://ws.etrid.org/eth-pbc');
const ethApi = await ApiPromise.create({ provider: ethProvider });
```

### Python

```python
from etrid_sdk import EtridClient

# Connect to Primearc Core Chain
client = EtridClient("wss://ws.etrid.org/primearc")

# Get balance
balance = await client.get_balance("5GrwvaEF...")
```

### Rust

```rust
use subxt::{OnlineClient, PolkadotConfig};

// Connect to Primearc Core Chain
let api = OnlineClient::<PolkadotConfig>::from_url(
    "wss://ws.etrid.org/primearc"
).await?;
```

### cURL (HTTP RPC)

```bash
# Get chain info
curl -X POST https://rpc.etrid.org/primearc \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"chain_getBlockHash","params":[]}'

# Get balance via state query
curl -X POST https://rpc.etrid.org/primearc \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"state_call","params":["AccountNonceApi_account_nonce","0x..."]}'
```

---

## Network Configuration

### Adding to Wallet (MetaMask-compatible for ETH PBC)

```json
{
  "chainId": "0x53A",
  "chainName": "ËTRID ETH PBC",
  "nativeCurrency": {
    "name": "Ëtrid",
    "symbol": "ETR",
    "decimals": 18
  },
  "rpcUrls": ["https://rpc.etrid.org/eth-pbc"],
  "blockExplorerUrls": ["https://explorer.etrid.org"]
}
```

### Polkadot.js Apps

To connect via Polkadot.js Apps:
1. Go to https://polkadot.js.org/apps
2. Click the network selector (top left)
3. Select "Development" → "Custom"
4. Enter: `wss://ws.etrid.org/primearc`

---

## Rate Limits

| Endpoint Type | Limit |
|---------------|-------|
| HTTP RPC | 100 requests/minute |
| WebSocket | 50 subscriptions/connection |
| REST API | 60 requests/minute |

For higher limits, contact: `gizzi_io@proton.me`

---

## Status & Health

- **Status Page**: `https://status.etrid.org`
- **Health Check**: `GET https://api.etrid.org/health`

---

## Contact

For technical support or API access:
- **Email**: `gizzi_io@proton.me`
- **Discord**: `discord.gg/etrid`
- **GitHub**: `github.com/EojEdred/Etrid`
