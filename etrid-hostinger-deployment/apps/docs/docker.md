# Docker Setup

Run ËTRID nodes using Docker for simplified deployment.

## Quick Start

```bash
# Pull latest image
docker pull etrid/etrid:latest

# Run local development node
docker run -p 9944:9944 -p 9933:9933 etrid/etrid:latest \
  --dev --ws-external --rpc-external
```

Access your node:
- WebSocket: `ws://localhost:9944`
- HTTP RPC: `http://localhost:9933`

---

## Docker Images

| Image | Purpose | Size |
|-------|---------|------|
| `etrid/etrid:latest` | Mainnet node | ~500MB |
| `etrid/etrid:testnet` | Testnet node | ~500MB |
| `etrid/etrid:dev` | Development | ~600MB |

---

## Running a Validator

### Using Docker Compose

```yaml
# docker-compose.yml
version: '3.8'

services:
  validator:
    image: etrid/etrid:latest
    container_name: etrid-validator
    restart: unless-stopped
    ports:
      - "30333:30333"  # P2P Substrate
      - "30334:30334"  # P2P DETR
      - "9944:9944"    # WebSocket RPC
      - "9933:9933"    # HTTP RPC
    volumes:
      - etrid-data:/data
      - ./chain-spec.json:/chain-spec.json:ro
    command: >
      --validator
      --name "My Validator"
      --chain /chain-spec.json
      --base-path /data
      --ws-external
      --rpc-external
      --rpc-cors all
      --prometheus-external
      --telemetry-url "wss://telemetry.polkadot.io/submit 0"
    environment:
      - RUST_LOG=info

volumes:
  etrid-data:
```

**Start validator:**
```bash
docker-compose up -d
```

**View logs:**
```bash
docker-compose logs -f validator
```

---

## Configuration

### Environment Variables

```bash
# Set via docker run
docker run -e RUST_LOG=debug etrid/etrid:latest

# Or in docker-compose.yml
environment:
  - RUST_LOG=debug
  - ETRID_TELEMETRY=1
```

### Volume Mounts

```bash
# Chain data persistence
-v etrid-data:/data

# Custom chain spec
-v ./chain-spec.json:/chain-spec.json:ro

# Session keys
-v ./keys:/keys:ro
```

---

## Building Custom Images

### Dockerfile

```dockerfile
# Dockerfile
FROM rust:1.70 as builder

WORKDIR /etrid
COPY . .

RUN cargo build --release

FROM ubuntu:22.04

RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /etrid/target/release/etrid /usr/local/bin/

EXPOSE 9944 9933 30333 30334

ENTRYPOINT ["/usr/local/bin/etrid"]
```

**Build:**
```bash
docker build -t my-etrid-node .
```

---

## Docker Networking

### Bridge Network (Default)

```bash
docker network create etrid-net

docker run --network etrid-net etrid/etrid:latest
```

### Host Network (Better Performance)

```bash
docker run --network host etrid/etrid:latest \
  --validator \
  --name "My Validator"
```

---

## Monitoring with Docker

### Docker Stats

```bash
# Real-time stats
docker stats etrid-validator

# Formatted output
docker stats --format "table {{.Container}}\t{{.CPUPerc}}\t{{.MemUsage}}"
```

### Prometheus + Grafana Stack

```yaml
# monitoring-stack.yml
version: '3.8'

services:
  prometheus:
    image: prom/prometheus:latest
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml
      - prometheus-data:/prometheus
    ports:
      - "9090:9090"

  grafana:
    image: grafana/grafana:latest
    ports:
      - "3000:3000"
    volumes:
      - grafana-data:/var/lib/grafana
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin

  node-exporter:
    image: prom/node-exporter:latest
    ports:
      - "9100:9100"

volumes:
  prometheus-data:
  grafana-data:
```

---

## Best Practices

### ✅ DO

- Use official images from Docker Hub
- Pin specific versions in production
- Persist data with volumes
- Set resource limits
- Enable automatic restarts
- Monitor container health

### ❌ DON'T

- Run as root (use `--user` flag)
- Use `:latest` tag in production
- Expose all ports publicly
- Ignore container logs
- Forget to back up volumes

---

## Resource Limits

```yaml
services:
  validator:
    image: etrid/etrid:latest
    deploy:
      resources:
        limits:
          cpus: '4'
          memory: 16G
        reservations:
          cpus: '2'
          memory: 8G
```

---

## Troubleshooting

### Container Won't Start

```bash
# Check logs
docker logs etrid-validator

# Inspect container
docker inspect etrid-validator

# Check if port in use
netstat -tulpn | grep 9944
```

### Performance Issues

```bash
# Check resource usage
docker stats

# Increase limits
docker update --cpus=4 --memory=16g etrid-validator
```

---

## Security

### Non-Root User

```dockerfile
RUN useradd -m -u 1000 etrid
USER etrid
```

### Read-Only Root Filesystem

```yaml
services:
  validator:
    read_only: true
    tmpfs:
      - /tmp
      - /data
```

---

## Resources

**Official Images:**
- [Docker Hub: etrid/etrid](https://hub.docker.com/r/etrid/etrid)

**Documentation:**
- [Docker Docs](https://docs.docker.com)
- [Docker Compose Reference](https://docs.docker.com/compose/compose-file/)

**Support:**
- 💬 [Discord #docker-support](https://discord.gg/etrid)
- 📧 Email: docker@etrid.org
