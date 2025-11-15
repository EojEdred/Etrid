use chrono::{DateTime, Utc};
use std::collections::VecDeque;
use std::time::Duration;
use tokio::time::interval;

use crate::rpc::RpcClient;

// ═══════════════════════════════════════════════════════════════════════════════
// APPLICATION STATE
// ═══════════════════════════════════════════════════════════════════════════════

pub struct App {
    pub state: MonitorState,
    pub rpc_client: RpcClient,
    pub validator_address: Option<String>,
    pub scroll_offset: usize,
    update_interval: u64,
    tick_count: u64,
}

impl App {
    pub fn new(rpc_url: String, validator: Option<String>, update_interval: u64) -> Self {
        Self {
            state: MonitorState::default(),
            rpc_client: RpcClient::new(rpc_url),
            validator_address: validator,
            scroll_offset: 0,
            update_interval,
            tick_count: 0,
        }
    }

    pub async fn on_tick(&mut self) -> anyhow::Result<()> {
        self.tick_count += 1;

        // Update every N ticks (based on interval)
        let ticks_per_update = (self.update_interval * 4) as u64; // 4 ticks per second
        if self.tick_count % ticks_per_update == 0 {
            self.refresh().await?;
        }

        Ok(())
    }

    pub async fn refresh(&mut self) -> anyhow::Result<()> {
        // Fetch latest data from RPC
        match self.rpc_client.fetch_network_status().await {
            Ok(network) => {
                self.state.chain_name = network.chain;
                self.state.current_block = network.block_number;
                self.state.peer_count = network.peer_count;
                self.state.is_syncing = network.is_syncing;

                self.state.add_log("INFO", "Network status updated");
            }
            Err(e) => {
                self.state.add_log("ERROR", &format!("Failed to fetch network status: {}", e));
            }
        }

        // Fetch validator-specific data if address is provided
        if let Some(ref validator_addr) = self.validator_address {
            match self.rpc_client.fetch_validator_status(validator_addr).await {
                Ok(validator) => {
                    self.state.is_active = validator.is_active;
                    self.state.in_committee = validator.in_committee;
                    self.state.is_slashed = validator.is_slashed;
                    self.state.votes_cast = validator.votes_cast;
                    self.state.certificates_issued = validator.certificates;
                    self.state.blocks_signed = validator.blocks_signed;
                    self.state.missed_blocks = validator.missed_blocks;
                    self.state.health_score = validator.health_score;
                    self.state.reputation = validator.reputation;
                    self.state.finality_level = validator.finality_level;

                    // Calculate uptime
                    let total = validator.blocks_signed + validator.missed_blocks;
                    if total > 0 {
                        self.state.uptime_percentage = ((validator.blocks_signed * 100) / total) as u8;
                    }

                    // Update block history
                    self.state.block_history.push_back(if validator.blocks_signed > 0 { 1 } else { 0 });
                    if self.state.block_history.len() > 60 {
                        self.state.block_history.pop_front();
                    }

                    self.state.add_log("INFO", "Validator status updated");
                }
                Err(e) => {
                    self.state.add_log("WARN", &format!("Failed to fetch validator status: {}", e));
                }
            }
        }

        Ok(())
    }

    pub fn clear_history(&mut self) {
        self.state.activity_log.clear();
        self.state.add_log("INFO", "Activity log cleared");
    }

    pub fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

    pub fn scroll_down(&mut self) {
        if self.scroll_offset < self.state.activity_log.len().saturating_sub(1) {
            self.scroll_offset += 1;
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// MONITOR STATE
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct MonitorState {
    // Network
    pub chain_name: String,
    pub current_block: u64,
    pub finality_level: u8,
    pub peer_count: u32,
    pub is_syncing: bool,

    // Validator
    pub is_active: bool,
    pub in_committee: bool,
    pub is_slashed: bool,
    pub votes_cast: u64,
    pub certificates_issued: u64,

    // Performance
    pub uptime_percentage: u8,
    pub blocks_signed: u64,
    pub missed_blocks: u64,
    pub health_score: u8,
    pub reputation: u8,

    // History
    pub block_history: VecDeque<u64>,
    pub activity_log: Vec<LogEntry>,
}

impl Default for MonitorState {
    fn default() -> Self {
        Self {
            chain_name: "Unknown".to_string(),
            current_block: 0,
            finality_level: 0,
            peer_count: 0,
            is_syncing: true,
            is_active: false,
            in_committee: false,
            is_slashed: false,
            votes_cast: 0,
            certificates_issued: 0,
            uptime_percentage: 0,
            blocks_signed: 0,
            missed_blocks: 0,
            health_score: 0,
            reputation: 0,
            block_history: VecDeque::with_capacity(60),
            activity_log: Vec::new(),
        }
    }
}

impl MonitorState {
    pub fn add_log(&mut self, level: &str, message: &str) {
        let entry = LogEntry {
            timestamp: Utc::now().format("%H:%M:%S").to_string(),
            level: level.to_string(),
            message: message.to_string(),
        };

        self.activity_log.push(entry);

        // Keep only last 1000 entries
        if self.activity_log.len() > 1000 {
            self.activity_log.remove(0);
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// LOG ENTRY
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub message: String,
}
