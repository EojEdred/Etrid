#!/bin/bash
# Ëtrid Protocol - Automated Monitoring Stack Setup
# Installs and configures Prometheus + Grafana for performance monitoring

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m'

ETRID_ROOT="${ETRID_ROOT:-$(pwd)}"

echo -e "${PURPLE}"
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║                                                              ║"
echo "║     ËTRID MONITORING STACK SETUP                            ║"
echo "║     Prometheus + Grafana Automated Installation             ║"
echo "║                                                              ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo -e "${NC}"
echo ""

log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
log_warning() { echo -e "${YELLOW}[WARNING]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# ============================================================================
# Detect Operating System
# ============================================================================

detect_os() {
    log_info "Detecting operating system..."

    if [[ "$OSTYPE" == "darwin"* ]]; then
        OS="macos"
        log_success "Detected: macOS"
    elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
        OS="linux"
        log_success "Detected: Linux"
    else
        log_error "Unsupported operating system: $OSTYPE"
        exit 1
    fi
}

# ============================================================================
# Install Prometheus
# ============================================================================

install_prometheus() {
    log_info "Installing Prometheus..."

    if command -v prometheus &> /dev/null; then
        PROM_VERSION=$(prometheus --version 2>&1 | head -1)
        log_success "Prometheus already installed: $PROM_VERSION"
        return 0
    fi

    if [[ "$OS" == "macos" ]]; then
        if ! command -v brew &> /dev/null; then
            log_error "Homebrew not found. Install from: https://brew.sh"
            exit 1
        fi

        log_info "Installing via Homebrew..."
        brew install prometheus
        log_success "Prometheus installed"

    elif [[ "$OS" == "linux" ]]; then
        log_info "Installing via package manager..."

        # Try different package managers
        if command -v apt-get &> /dev/null; then
            sudo apt-get update
            sudo apt-get install -y prometheus
        elif command -v yum &> /dev/null; then
            sudo yum install -y prometheus
        elif command -v dnf &> /dev/null; then
            sudo dnf install -y prometheus
        else
            # Download binary
            log_info "No package manager found, downloading binary..."
            PROM_VERSION="2.45.0"
            wget "https://github.com/prometheus/prometheus/releases/download/v${PROM_VERSION}/prometheus-${PROM_VERSION}.linux-amd64.tar.gz"
            tar xvfz "prometheus-${PROM_VERSION}.linux-amd64.tar.gz"
            sudo mv "prometheus-${PROM_VERSION}.linux-amd64/prometheus" /usr/local/bin/
            sudo mv "prometheus-${PROM_VERSION}.linux-amd64/promtool" /usr/local/bin/
            rm -rf "prometheus-${PROM_VERSION}.linux-amd64"*
        fi

        log_success "Prometheus installed"
    fi
}

# ============================================================================
# Configure Prometheus
# ============================================================================

configure_prometheus() {
    log_info "Configuring Prometheus..."

    # Create Prometheus directories
    mkdir -p "$ETRID_ROOT/monitoring/prometheus/data"

    # Copy configuration
    if [[ -f "$ETRID_ROOT/scripts/testnet/prometheus.yml" ]]; then
        cp "$ETRID_ROOT/scripts/testnet/prometheus.yml" "$ETRID_ROOT/monitoring/prometheus/prometheus.yml"
        log_success "Prometheus configuration copied"
    else
        log_warning "prometheus.yml not found, creating default config..."

        cat > "$ETRID_ROOT/monitoring/prometheus/prometheus.yml" <<EOF
global:
  scrape_interval: 15s
  evaluation_interval: 15s
  external_labels:
    monitor: 'etrid-production'

scrape_configs:
  - job_name: 'flarechain-validator'
    static_configs:
      - targets: ['localhost:9615']
        labels:
          instance: 'validator-01'
          node_type: 'validator'

  - job_name: 'prometheus'
    static_configs:
      - targets: ['localhost:9090']
EOF
        log_success "Default Prometheus config created"
    fi

    # Create systemd service (Linux) or launch agent (macOS)
    if [[ "$OS" == "linux" ]]; then
        log_info "Creating systemd service..."

        sudo tee /etc/systemd/system/prometheus-etrid.service > /dev/null <<EOF
[Unit]
Description=Prometheus for Ëtrid Protocol
After=network.target

[Service]
Type=simple
User=$USER
ExecStart=/usr/local/bin/prometheus \\
  --config.file=$ETRID_ROOT/monitoring/prometheus/prometheus.yml \\
  --storage.tsdb.path=$ETRID_ROOT/monitoring/prometheus/data \\
  --web.console.templates=/etc/prometheus/consoles \\
  --web.console.libraries=/etc/prometheus/console_libraries
Restart=always

[Install]
WantedBy=multi-user.target
EOF

        sudo systemctl daemon-reload
        log_success "Systemd service created"
    fi
}

# ============================================================================
# Install Grafana
# ============================================================================

install_grafana() {
    log_info "Installing Grafana..."

    if command -v grafana-server &> /dev/null; then
        GRAFANA_VERSION=$(grafana-server -v 2>&1 | head -1)
        log_success "Grafana already installed: $GRAFANA_VERSION"
        return 0
    fi

    if [[ "$OS" == "macos" ]]; then
        log_info "Installing via Homebrew..."
        brew install grafana
        log_success "Grafana installed"

    elif [[ "$OS" == "linux" ]]; then
        log_info "Installing via package manager..."

        if command -v apt-get &> /dev/null; then
            # Debian/Ubuntu
            sudo apt-get install -y software-properties-common
            sudo add-apt-repository "deb https://packages.grafana.com/oss/deb stable main"
            wget -q -O - https://packages.grafana.com/gpg.key | sudo apt-key add -
            sudo apt-get update
            sudo apt-get install -y grafana
        elif command -v yum &> /dev/null || command -v dnf &> /dev/null; then
            # RHEL/CentOS
            cat > /tmp/grafana.repo <<EOF
[grafana]
name=grafana
baseurl=https://packages.grafana.com/oss/rpm
repo_gpgcheck=1
enabled=1
gpgcheck=1
gpgkey=https://packages.grafana.com/gpg.key
sslverify=1
sslcacert=/etc/pki/tls/certs/ca-bundle.crt
EOF
            sudo mv /tmp/grafana.repo /etc/yum.repos.d/grafana.repo

            if command -v dnf &> /dev/null; then
                sudo dnf install -y grafana
            else
                sudo yum install -y grafana
            fi
        fi

        log_success "Grafana installed"
    fi
}

# ============================================================================
# Configure Grafana
# ============================================================================

configure_grafana() {
    log_info "Configuring Grafana..."

    # Create Grafana directories
    mkdir -p "$ETRID_ROOT/monitoring/grafana/dashboards"
    mkdir -p "$ETRID_ROOT/monitoring/grafana/provisioning/datasources"
    mkdir -p "$ETRID_ROOT/monitoring/grafana/provisioning/dashboards"

    # Configure Prometheus datasource
    cat > "$ETRID_ROOT/monitoring/grafana/provisioning/datasources/prometheus.yml" <<EOF
apiVersion: 1

datasources:
  - name: Prometheus
    type: prometheus
    access: proxy
    url: http://localhost:9090
    isDefault: true
    editable: true
EOF

    log_success "Grafana datasource configured"

    # Configure dashboard provisioning
    cat > "$ETRID_ROOT/monitoring/grafana/provisioning/dashboards/etrid.yml" <<EOF
apiVersion: 1

providers:
  - name: 'Ëtrid Dashboards'
    orgId: 1
    folder: 'Ëtrid Protocol'
    type: file
    disableDeletion: false
    updateIntervalSeconds: 10
    allowUiUpdates: true
    options:
      path: $ETRID_ROOT/monitoring/grafana/dashboards
EOF

    log_success "Grafana dashboard provisioning configured"

    # Copy dashboard if exists
    if [[ -f "$ETRID_ROOT/scripts/testnet/grafana-dashboard.json" ]]; then
        cp "$ETRID_ROOT/scripts/testnet/grafana-dashboard.json" "$ETRID_ROOT/monitoring/grafana/dashboards/"
        log_success "Ëtrid dashboard copied"
    fi
}

# ============================================================================
# Start Services
# ============================================================================

start_services() {
    log_info "Starting monitoring services..."

    if [[ "$OS" == "macos" ]]; then
        # macOS with Homebrew services
        log_info "Starting Prometheus..."
        brew services start prometheus

        log_info "Starting Grafana..."
        brew services start grafana

    elif [[ "$OS" == "linux" ]]; then
        # Linux with systemd
        log_info "Starting Prometheus..."
        sudo systemctl enable prometheus-etrid
        sudo systemctl start prometheus-etrid

        log_info "Starting Grafana..."
        sudo systemctl enable grafana-server
        sudo systemctl start grafana-server
    fi

    log_success "Services started"
}

# ============================================================================
# Verify Installation
# ============================================================================

verify_installation() {
    log_info "Verifying installation..."

    sleep 5  # Wait for services to start

    # Check Prometheus
    if curl -s http://localhost:9090/-/healthy &>/dev/null; then
        log_success "Prometheus is running: http://localhost:9090"
    else
        log_warning "Prometheus may not be running yet (wait 10s and check)"
    fi

    # Check Grafana
    if curl -s http://localhost:3000/api/health &>/dev/null; then
        log_success "Grafana is running: http://localhost:3000"
        log_info "  Default credentials: admin / admin"
    else
        log_warning "Grafana may not be running yet (wait 10s and check)"
    fi

    # Check if node is exposing metrics
    if curl -s http://localhost:9615/metrics &>/dev/null; then
        METRIC_COUNT=$(curl -s http://localhost:9615/metrics | wc -l)
        log_success "FlareChain node metrics available ($METRIC_COUNT metrics)"
    else
        log_warning "FlareChain node not running or metrics not exposed on :9615"
        log_info "  Start node with: ./scripts/start-validator-optimized.sh"
    fi
}

# ============================================================================
# Create Helper Scripts
# ============================================================================

create_helper_scripts() {
    log_info "Creating helper scripts..."

    # Monitoring status script
    cat > "$ETRID_ROOT/scripts/monitoring-status.sh" <<'EOF'
#!/bin/bash
# Check monitoring stack status

echo "=== Monitoring Stack Status ==="
echo ""

# Prometheus
if curl -s http://localhost:9090/-/healthy &>/dev/null; then
    echo "✓ Prometheus: Running (http://localhost:9090)"
else
    echo "✗ Prometheus: Not running"
fi

# Grafana
if curl -s http://localhost:3000/api/health &>/dev/null; then
    echo "✓ Grafana: Running (http://localhost:3000)"
else
    echo "✗ Grafana: Not running"
fi

# FlareChain metrics
if curl -s http://localhost:9615/metrics &>/dev/null; then
    METRIC_COUNT=$(curl -s http://localhost:9615/metrics | wc -l)
    echo "✓ FlareChain: Exposing $METRIC_COUNT metrics"
else
    echo "✗ FlareChain: No metrics available"
fi

echo ""
echo "=== Quick Commands ==="
echo "  View Prometheus:  open http://localhost:9090"
echo "  View Grafana:     open http://localhost:3000"
echo "  View Metrics:     curl http://localhost:9615/metrics"
EOF

    chmod +x "$ETRID_ROOT/scripts/monitoring-status.sh"
    log_success "Helper scripts created"
}

# ============================================================================
# Main Execution
# ============================================================================

main() {
    detect_os
    echo ""

    install_prometheus
    configure_prometheus
    echo ""

    install_grafana
    configure_grafana
    echo ""

    start_services
    echo ""

    create_helper_scripts
    echo ""

    verify_installation

    echo ""
    echo -e "${GREEN}"
    echo "═══════════════════════════════════════════════════════════════"
    echo "           MONITORING STACK SETUP COMPLETE                     "
    echo "═══════════════════════════════════════════════════════════════"
    echo -e "${NC}"
    echo ""
    echo "Next Steps:"
    echo "  1. Access Grafana: http://localhost:3000 (admin/admin)"
    echo "  2. Access Prometheus: http://localhost:9090"
    echo "  3. Start FlareChain node with metrics enabled"
    echo "  4. Check status: ./scripts/monitoring-status.sh"
    echo ""
    echo "Configuration:"
    echo "  Prometheus config: $ETRID_ROOT/monitoring/prometheus/prometheus.yml"
    echo "  Grafana dashboards: $ETRID_ROOT/monitoring/grafana/dashboards/"
    echo ""
}

main "$@"
