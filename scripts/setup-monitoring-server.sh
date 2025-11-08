#!/bin/bash
#
# Setup Centralized Monitoring for All 21 Validators
# Run this on your NEW monitoring server VM
#

set -e

echo "🔧 Installing Prometheus, Grafana, and Node Exporter..."

# Update system
sudo apt update
sudo apt install -y wget curl

# Install Prometheus
PROMETHEUS_VERSION="2.47.0"
cd /tmp
wget https://github.com/prometheus/prometheus/releases/download/v${PROMETHEUS_VERSION}/prometheus-${PROMETHEUS_VERSION}.linux-amd64.tar.gz
tar xvfz prometheus-${PROMETHEUS_VERSION}.linux-amd64.tar.gz
sudo mv prometheus-${PROMETHEUS_VERSION}.linux-amd64 /opt/prometheus
sudo useradd --no-create-home --shell /bin/false prometheus || true
sudo mkdir -p /var/lib/prometheus
sudo chown prometheus:prometheus /var/lib/prometheus

# Create Prometheus systemd service
sudo tee /etc/systemd/system/prometheus.service > /dev/null <<'EOF'
[Unit]
Description=Prometheus
Wants=network-online.target
After=network-online.target

[Service]
User=prometheus
Group=prometheus
Type=simple
ExecStart=/opt/prometheus/prometheus \
  --config.file /etc/prometheus/prometheus.yml \
  --storage.tsdb.path /var/lib/prometheus/ \
  --web.console.templates=/opt/prometheus/consoles \
  --web.console.libraries=/opt/prometheus/console_libraries

[Install]
WantedBy=multi-user.target
EOF

# Install Grafana
sudo apt-get install -y software-properties-common
sudo add-apt-repository -y "deb https://packages.grafana.com/oss/deb stable main"
wget -q -O - https://packages.grafana.com/gpg.key | sudo apt-key add -
sudo apt-get update
sudo apt-get install -y grafana

# Install Node Exporter (for monitoring server itself)
NODE_EXPORTER_VERSION="1.6.1"
cd /tmp
wget https://github.com/prometheus/node_exporter/releases/download/v${NODE_EXPORTER_VERSION}/node_exporter-${NODE_EXPORTER_VERSION}.linux-amd64.tar.gz
tar xvfz node_exporter-${NODE_EXPORTER_VERSION}.linux-amd64.tar.gz
sudo mv node_exporter-${NODE_EXPORTER_VERSION}.linux-amd64/node_exporter /usr/local/bin/

# Create Node Exporter systemd service
sudo tee /etc/systemd/system/node_exporter.service > /dev/null <<'EOF'
[Unit]
Description=Node Exporter
After=network.target

[Service]
User=prometheus
Group=prometheus
Type=simple
ExecStart=/usr/local/bin/node_exporter

[Install]
WantedBy=multi-user.target
EOF

echo "✅ All packages installed"
echo ""
echo "📝 Next steps:"
echo "1. Create Prometheus config with all 21 validator IPs"
echo "2. Run: sudo systemctl start prometheus grafana-server node_exporter"
echo "3. Run: sudo systemctl enable prometheus grafana-server node_exporter"
echo ""
echo "Access:"
echo "  Prometheus: http://$(hostname -I | awk '{print $1}'):9090"
echo "  Grafana:    http://$(hostname -I | awk '{print $1}'):3000 (admin/admin)"
