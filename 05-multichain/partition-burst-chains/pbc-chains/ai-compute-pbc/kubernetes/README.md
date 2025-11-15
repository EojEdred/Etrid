# Etrid AI Compute - Kubernetes Hybrid Cloud Plugin

Automatic GPU scaling plugin for Etrid AI-Compute-PBC. Monitors job queue depth and auto-provisions cloud GPUs when local capacity is saturated.

## Features

- **Auto-scaling**: HorizontalPodAutoscaler based on queue depth, GPU utilization, and job wait time
- **Hybrid Cloud Bursting**: Automatically provisions AWS/GCP/Azure spot instances during peak demand
- **Cost Optimized**: Aggressive scale-down when queue empty
- **Multi-cloud**: Supports AWS, GCP, Azure GPU instances
- **TEE Support**: Intel SGX and AMD SEV for confidential compute
- **Compliance Ready**: Auto-enables HIPAA/GDPR/SOC2 templates

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│  AI-Compute-PBC Blockchain                                  │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │ Job Queue   │  │ GPU Registry│  │ Marketplace │         │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘         │
└─────────┼──────────────────┼─────────────────┼──────────────┘
          │                  │                 │
          ▼                  ▼                 ▼
┌─────────────────────────────────────────────────────────────┐
│  Metrics Exporter (scrapes chain state every 10s)          │
│  Exports: queue_depth, avg_wait_time, gpu_utilization      │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  Prometheus (stores time-series metrics)                    │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  HorizontalPodAutoscaler (scales based on metrics)          │
│  Rules:                                                      │
│  - queue_depth > 5 jobs/GPU → scale up 50%                  │
│  - avg_wait_time > 30s → scale up                           │
│  - queue_depth < 1 → scale down 10% (after 5min)            │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  GPU Provider Pods (workers that execute AI jobs)           │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │
│  │ GPU Pod 1│  │ GPU Pod 2│  │   ...    │  │ GPU Pod N│   │
│  │ (local)  │  │ (local)  │  │          │  │ (cloud)  │   │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## Prerequisites

1. **Kubernetes cluster** with GPU nodes
   - NVIDIA GPU Operator installed (for NVIDIA GPUs)
   - AMD GPU Device Plugin installed (for AMD GPUs)
   - Prometheus Operator installed
   - Metrics Server installed

2. **Etrid AI-Compute-PBC** running and accessible
   - RPC endpoint: `https://ai-compute-pbc.etrid.network:9944`
   - WebSocket endpoint: `wss://ai-compute-pbc.etrid.network:9944`

3. **GPU Provider account** on Etrid blockchain
   - Create account: `subkey generate --scheme sr25519`
   - Fund account with ËDSC for gas fees
   - Stake 100 ËDSC to register as GPU provider

4. **Cloud provider credentials** (optional, for hybrid bursting)
   - AWS access keys
   - GCP service account JSON
   - Azure service principal credentials

## Quick Start

### 1. Install Prerequisites

```bash
# Install NVIDIA GPU Operator (for NVIDIA GPUs)
kubectl create ns gpu-operator
helm repo add nvidia https://helm.ngc.nvidia.com/nvidia
helm install gpu-operator nvidia/gpu-operator -n gpu-operator

# Install Prometheus Operator
kubectl create ns monitoring
helm repo add prometheus-community https://prometheus-community.github.io/helm-charts
helm install kube-prometheus prometheus-community/kube-prometheus-stack -n monitoring

# Install Metrics Server
kubectl apply -f https://github.com/kubernetes-sigs/metrics-server/releases/latest/download/components.yaml
```

### 2. Configure Etrid Connection

Edit `etrid-config.yml` and update:

```yaml
data:
  rpc_url: "https://ai-compute-pbc.etrid.network:9944"  # Your AI-Compute-PBC RPC
  websocket_url: "wss://ai-compute-pbc.etrid.network:9944"
```

Edit the `etrid-provider-credentials` secret:

```yaml
stringData:
  seed_phrase: "your twelve word seed phrase here"
  account_id: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"  # Your SS58 address
```

### 3. Deploy to Kubernetes

```bash
# Create namespace and configuration
kubectl apply -f etrid-config.yml

# Deploy metrics exporter
kubectl apply -f etrid-ai-scaler.yml

# Deploy GPU provider workers
kubectl apply -f gpu-provider-deployment.yml

# Verify deployment
kubectl get pods -n etrid-ai-compute
kubectl get hpa -n etrid-ai-compute
```

### 4. Monitor Scaling

```bash
# Watch HPA status
kubectl get hpa etrid-ai-gpu-scaler -n etrid-ai-compute --watch

# View metrics
kubectl top pods -n etrid-ai-compute

# Check logs
kubectl logs -n etrid-ai-compute -l component=gpu-provider -f
```

## Configuration

### Scaling Thresholds

Edit `etrid-ai-scaler.yml` to adjust scaling behavior:

```yaml
spec:
  minReplicas: 2      # Minimum GPU pods
  maxReplicas: 100    # Maximum GPU pods
  metrics:
    - type: External
      external:
        metric:
          name: etrid_job_queue_depth
        target:
          averageValue: "5"  # Scale up if >5 jobs per GPU
```

### GPU Pricing

Edit `gpu-provider-deployment.yml` to set your GPU pricing:

```yaml
env:
  - name: PRICE_PER_SECOND
    value: "100000000000000"  # 0.0001 ËDSC/sec = $0.36/hour
```

Pricing examples (assuming 1 ËDSC = $0.10):
- `100000000000000` = 0.0001 ËDSC/sec = **$0.36/hour** (budget tier)
- `500000000000000` = 0.0005 ËDSC/sec = **$1.80/hour** (mid tier)
- `1000000000000000` = 0.001 ËDSC/sec = **$3.60/hour** (premium tier)

### Cloud Bursting

Edit `etrid-config.yml` to enable cloud providers:

```yaml
data:
  aws_enabled: "true"
  aws_region: "us-west-2"
  aws_instance_type: "p4d.24xlarge"  # 8x A100 80GB
  aws_spot_max_price: "15.00"        # Max $15/hour
```

Add cloud credentials to `cloud-provider-credentials` secret.

## Hybrid Cloud Bursting

When local GPU capacity is saturated, the system automatically provisions cloud GPUs:

1. **Trigger**: Job queue depth > 5 jobs/GPU for 60 seconds
2. **Action**: HPA scales up Deployment
3. **Cloud Provisioning**: Cluster Autoscaler provisions new GPU nodes from cloud
4. **Cost Optimization**: Uses spot/preemptible instances for 70% cost savings
5. **Auto-Termination**: Scales down after 5 minutes of idle time

Supported cloud providers:
- **AWS**: p4d.24xlarge (8x A100 80GB) - $15-20/hour spot
- **GCP**: a2-ultragpu-8g (8x A100 80GB) - $14-18/hour preemptible
- **Azure**: Standard_ND96amsr_A100_v4 (8x A100 80GB) - $16-22/hour spot

## Monitoring

### Prometheus Metrics

The metrics exporter exposes:

```
# Job queue metrics
etrid_job_queue_depth{pbc="ai-compute",status="pending"}
etrid_job_queue_depth{pbc="ai-compute",status="assigned"}
etrid_job_queue_depth{pbc="ai-compute",status="completed"}

# Performance metrics
etrid_avg_job_wait_seconds{pbc="ai-compute"}
etrid_avg_job_duration_seconds{pbc="ai-compute"}

# GPU metrics
etrid_gpu_utilization_percent{provider="<account_id>"}
etrid_gpu_jobs_completed_total{provider="<account_id>"}

# Revenue metrics
etrid_gpu_earnings_total_edsc{provider="<account_id>"}
```

### Grafana Dashboard

Import the included dashboard:

```bash
kubectl port-forward -n monitoring svc/kube-prometheus-grafana 3000:80
# Open http://localhost:3000
# Import dashboard from kubernetes/grafana-dashboard.json
```

## Security

### TEE (Trusted Execution Environment)

Enable Intel SGX or AMD SEV for confidential compute:

```yaml
env:
  - name: ENABLE_TEE
    value: "true"
```

Add TEE attestation keys to `etrid-tee-keys` secret.

### Compliance Templates

Auto-enable HIPAA/GDPR/SOC2:

```yaml
env:
  - name: ENABLE_COMPLIANCE
    value: "true"
```

This automatically:
- Encrypts all job data at rest and in transit
- Enables audit logging
- Enforces data residency rules
- Generates compliance reports

## Troubleshooting

### Pods not scaling

```bash
# Check HPA status
kubectl describe hpa etrid-ai-gpu-scaler -n etrid-ai-compute

# Check metrics server
kubectl get apiservices | grep metrics

# Check Prometheus targets
kubectl port-forward -n monitoring svc/kube-prometheus-prometheus 9090:9090
# Open http://localhost:9090/targets
```

### GPUs not detected

```bash
# Verify GPU nodes
kubectl get nodes -o json | jq '.items[].status.allocatable'

# Check GPU operator
kubectl get pods -n gpu-operator

# Check device plugin daemonset
kubectl get daemonset -n kube-system | grep nvidia
```

### Connection to blockchain failed

```bash
# Check RPC endpoint
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' https://ai-compute-pbc.etrid.network:9944

# Check pod logs
kubectl logs -n etrid-ai-compute -l component=gpu-provider
```

## Cost Optimization

### Local GPUs vs Cloud GPUs

| Provider | GPU Type | Cost | When to Use |
|----------|----------|------|-------------|
| Local | Your hardware | $0 | Baseline load |
| AWS Spot | p4d.24xlarge | $15/hr | Burst traffic |
| GCP Preemptible | a2-ultragpu-8g | $14/hr | Batch jobs |
| Azure Spot | ND96amsr_A100_v4 | $16/hr | Variable load |

### Best Practices

1. **Aggressive local scaling**: Set `minReplicas` to match your local GPU count
2. **Conservative cloud scaling**: Set `scaleDown.stabilizationWindowSeconds: 300` to avoid thrashing
3. **Use spot instances**: Save 60-70% vs on-demand
4. **Set max prices**: Cap cloud costs with `aws_spot_max_price`
5. **Monitor costs**: Track `etrid_cloud_cost_usd_total` metric

## Revenue Projections

Assuming 8x local A100 GPUs running 24/7:

```
Local capacity: 8 GPUs
Avg utilization: 70%
Price: $2/hour per GPU
Revenue/day: 8 × 24 × 0.7 × $2 = $268.80
Revenue/month: $8,064
Revenue/year: $96,768

With cloud bursting (10 extra GPU-hours/day):
Additional revenue/day: 10 × $2 = $20
Cloud cost/day: 10 × $15 = $150
Net additional: -$130/day (not profitable)

Recommendation: Only burst to cloud if you can charge premium pricing (>$15/hour)
```

## Support

- Documentation: https://docs.etrid.network/ai-compute-pbc
- Discord: https://discord.gg/etrid
- GitHub Issues: https://github.com/etrid/etrid/issues

## License

Apache-2.0
