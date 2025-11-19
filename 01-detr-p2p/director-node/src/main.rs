// director-node/src/main.rs
// DIRECTOR NODE BINARY
// Executable for running a director node

use director_node::{DirectorNode, DirectorConfig};
use detrp2p::PeerAddr;
use std::env;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .init();

    log::info!("🎯 Starting ËTRID Director Node");

    // Load config from file or environment
    let config = load_config()?;

    log::info!("📋 Director Configuration:");
    log::info!("   Tailscale IP: {}", config.tailscale_ip);
    log::info!("   Peer Directors: {:?}", config.peer_directors);
    log::info!("   Max Validators: {}", config.max_validators_per_director);
    log::info!("   Authorization Required: {}", config.require_authorization);
    log::info!("   Validator Port: {}", config.validator_port);
    log::info!("   Director Port: {}", config.director_port);

    // Create director node
    let mut director = DirectorNode::new(config.clone());

    // Load bootstrap peers (other directors)
    let bootstrap_peers = load_bootstrap_peers(&config)?;

    log::info!("🔌 Bootstrap peers: {} directors", bootstrap_peers.len());

    // Start P2P network
    director.start(bootstrap_peers).await?;

    // Connect to peer directors
    director.connect_to_directors().await?;

    log::info!("✅ Director node running on {}", config.tailscale_ip);
    log::info!("📊 Accepting connections from validators on port {}", config.validator_port);

    // Main event loop
    loop {
        // Process incoming messages
        if let Some(p2p) = &director.p2p {
            if let Some((from, msg)) = p2p.receive_message().await {
                log::debug!("📥 Received message from {:?}", from);

                // Relay message to network
                if let Err(e) = director.relay_checkpoint_message(from, msg).await {
                    log::warn!("Failed to relay message: {}", e);
                }
            }
        }

        // Periodic metrics reporting
        let metrics = director.get_metrics().await;
        let validator_count = director.get_validator_count().await;

        if metrics.total_relayed % 100 == 0 && metrics.total_relayed > 0 {
            log::info!(
                "📈 Metrics: {} validators, {} relayed, {} loops prevented",
                validator_count,
                metrics.total_relayed,
                metrics.loops_prevented
            );
        }

        // Small delay to prevent busy loop
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }
}

/// Load director configuration from file or environment
fn load_config() -> Result<DirectorConfig, Box<dyn std::error::Error>> {
    // Try to load from config file
    if let Ok(config_path) = env::var("DIRECTOR_CONFIG") {
        let config_data = fs::read_to_string(&config_path)?;
        let config: DirectorConfig = serde_json::from_str(&config_data)?;
        return Ok(config);
    }

    // Fallback to environment variables
    let mut config = DirectorConfig::default();

    if let Ok(tailscale_ip) = env::var("TAILSCALE_IP") {
        config.tailscale_ip = tailscale_ip;
    }

    if let Ok(peer_directors) = env::var("PEER_DIRECTORS") {
        config.peer_directors = peer_directors
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
    }

    if let Ok(max_validators) = env::var("MAX_VALIDATORS") {
        config.max_validators_per_director = max_validators.parse()?;
    }

    if let Ok(require_auth) = env::var("REQUIRE_AUTHORIZATION") {
        config.require_authorization = require_auth.parse()?;
    }

    if let Ok(validator_port) = env::var("VALIDATOR_PORT") {
        config.validator_port = validator_port.parse()?;
    }

    if let Ok(director_port) = env::var("DIRECTOR_PORT") {
        config.director_port = director_port.parse()?;
    }

    // Generate peer ID from Tailscale IP
    use blake2::{Blake2b512, Digest};
    let hash = Blake2b512::digest(config.tailscale_ip.as_bytes());
    let mut peer_id_bytes = [0u8; 32];
    peer_id_bytes.copy_from_slice(&hash[..32]);
    config.peer_id = detrp2p::PeerId::new(peer_id_bytes);

    Ok(config)
}

/// Load bootstrap peers from configuration
fn load_bootstrap_peers(config: &DirectorConfig) -> Result<Vec<PeerAddr>, Box<dyn std::error::Error>> {
    let mut peers = Vec::new();

    for director_ip in &config.peer_directors {
        let addr = format!("{}:{}", director_ip, config.director_port).parse()?;

        // Generate peer ID from IP
        use blake2::{Blake2b512, Digest};
        let hash = Blake2b512::digest(director_ip.as_bytes());
        let mut peer_id_bytes = [0u8; 32];
        peer_id_bytes.copy_from_slice(&hash[..32]);

        peers.push(PeerAddr {
            id: detrp2p::PeerId::new(peer_id_bytes),
            address: addr,
        });
    }

    Ok(peers)
}
