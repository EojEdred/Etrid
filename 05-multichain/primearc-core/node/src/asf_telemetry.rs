//! # ASF Consensus Telemetry Integration
//!
//! This module provides telemetry emission for ASF consensus events:
//! - Vote reception and submission
//! - Certificate generation
//! - Finality level changes
//! - Slashing events
//! - Byzantine detection
//! - Committee rotation
//! - Block production metrics

use sc_telemetry::{telemetry, TelemetryHandle, CONSENSUS_INFO, CONSENSUS_DEBUG, CONSENSUS_WARN};
use asf_algorithm::{
    FinalityLevel, ValidatorId, Hash, BlockNumber,
    Vote, Certificate,
};
use codec::Encode;

// ═══════════════════════════════════════════════════════════════════════════════
// TELEMETRY EVENTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Emit telemetry for vote reception
pub fn telemetry_vote_received(
    telemetry: &Option<TelemetryHandle>,
    vote: &Vote,
) {
    telemetry!(
        telemetry;
        CONSENSUS_DEBUG;
        "asf.vote_received";
        "block_hash" => ?vote.block_hash,
        "block_number" => vote.block_number,
        "phase" => format!("{:?}", vote.phase),
        "validator" => hex::encode(&vote.validator.encode()[..8]),
        "stake_weight" => vote.stake_weight,
        "epoch" => vote.epoch,
    );
}

/// Emit telemetry for vote submission
pub fn telemetry_vote_submitted(
    telemetry: &Option<TelemetryHandle>,
    vote: &Vote,
) {
    telemetry!(
        telemetry;
        CONSENSUS_INFO;
        "asf.vote_submitted";
        "block_hash" => ?vote.block_hash,
        "block_number" => vote.block_number,
        "phase" => format!("{:?}", vote.phase),
        "validator" => hex::encode(&vote.validator.encode()[..8]),
        "stake_weight" => vote.stake_weight,
        "epoch" => vote.epoch,
    );
}

/// Emit telemetry for certificate generation
pub fn telemetry_certificate_generated(
    telemetry: &Option<TelemetryHandle>,
    certificate: &Certificate,
) {
    telemetry!(
        telemetry;
        CONSENSUS_INFO;
        "asf.certificate_generated";
        "block_hash" => ?certificate.block_hash,
        "block_number" => certificate.block_number,
        "phase" => format!("{:?}", certificate.phase),
        "validator_count" => certificate.vote_aggregate.validator_count,
        "total_stake" => certificate.vote_aggregate.total_stake,
        "issuer" => hex::encode(&certificate.validator.encode()[..8]),
        "epoch" => certificate.epoch,
    );
}

/// Emit telemetry for certificate broadcast
pub fn telemetry_certificate_broadcast(
    telemetry: &Option<TelemetryHandle>,
    certificate: &Certificate,
) {
    telemetry!(
        telemetry;
        CONSENSUS_DEBUG;
        "asf.certificate_broadcast";
        "block_hash" => ?certificate.block_hash,
        "block_number" => certificate.block_number,
        "phase" => format!("{:?}", certificate.phase),
        "validator_count" => certificate.vote_aggregate.validator_count,
    );
}

/// Emit telemetry for finality level change
pub fn telemetry_finality_level_changed(
    telemetry: &Option<TelemetryHandle>,
    block_hash: &Hash,
    block_number: BlockNumber,
    old_level: FinalityLevel,
    new_level: FinalityLevel,
    certificate_count: u32,
) {
    let level_name = match new_level {
        FinalityLevel::None => "None",
        FinalityLevel::Weak => "Weak",
        FinalityLevel::Moderate => "Moderate",
        FinalityLevel::Strong => "Strong",
        FinalityLevel::Irreversible => "Irreversible",
    };

    telemetry!(
        telemetry;
        CONSENSUS_INFO;
        "asf.finality_level_changed";
        "block_hash" => ?block_hash,
        "block_number" => block_number,
        "old_level" => format!("{:?}", old_level),
        "new_level" => level_name,
        "certificate_count" => certificate_count,
    );
}

/// Emit telemetry for finality reached
pub fn telemetry_finality_reached(
    telemetry: &Option<TelemetryHandle>,
    block_hash: &Hash,
    block_number: BlockNumber,
    finality_level: FinalityLevel,
    time_to_finalization_ms: Option<u64>,
) {
    telemetry!(
        telemetry;
        CONSENSUS_INFO;
        "asf.finality_reached";
        "block_hash" => ?block_hash,
        "block_number" => block_number,
        "finality_level" => format!("{:?}", finality_level),
        "time_ms" => time_to_finalization_ms,
    );
}

/// Emit telemetry for slashing event
pub fn telemetry_slashing_event(
    telemetry: &Option<TelemetryHandle>,
    validator: &ValidatorId,
    severity: &str,
    reason: &str,
    amount_slashed: u128,
    epoch: u32,
    excluded: bool,
) {
    telemetry!(
        telemetry;
        CONSENSUS_WARN;
        "asf.slashing_event";
        "validator" => hex::encode(&validator.encode()[..8]),
        "severity" => severity,
        "reason" => reason,
        "amount_slashed" => amount_slashed,
        "epoch" => epoch,
        "excluded" => excluded,
    );
}

/// Emit telemetry for Byzantine detection
pub fn telemetry_byzantine_detected(
    telemetry: &Option<TelemetryHandle>,
    validator: &ValidatorId,
    reason: &str,
    incident_count: u32,
    epoch: u32,
) {
    telemetry!(
        telemetry;
        CONSENSUS_WARN;
        "asf.byzantine_detected";
        "validator" => hex::encode(&validator.encode()[..8]),
        "reason" => reason,
        "incident_count" => incident_count,
        "epoch" => epoch,
    );
}

/// Emit telemetry for committee rotation
pub fn telemetry_committee_rotated(
    telemetry: &Option<TelemetryHandle>,
    new_epoch: u32,
    committee_size: u32,
    total_stake: u128,
) {
    telemetry!(
        telemetry;
        CONSENSUS_INFO;
        "asf.committee_rotated";
        "epoch" => new_epoch,
        "committee_size" => committee_size,
        "total_stake" => total_stake,
    );
}

/// Emit telemetry for PPFA proposer selection
pub fn telemetry_ppfa_proposer_selected(
    telemetry: &Option<TelemetryHandle>,
    slot_number: u64,
    proposer: &ValidatorId,
    block_number: BlockNumber,
) {
    telemetry!(
        telemetry;
        CONSENSUS_DEBUG;
        "asf.ppfa_proposer_selected";
        "slot" => slot_number,
        "proposer" => hex::encode(&proposer.encode()[..8]),
        "block_number" => block_number,
    );
}

/// Emit telemetry for block production
pub fn telemetry_block_produced(
    telemetry: &Option<TelemetryHandle>,
    block_hash: &Hash,
    block_number: BlockNumber,
    extrinsic_count: usize,
    proposer: &ValidatorId,
) {
    telemetry!(
        telemetry;
        CONSENSUS_INFO;
        "asf.block_produced";
        "block_hash" => ?block_hash,
        "block_number" => block_number,
        "extrinsics" => extrinsic_count,
        "proposer" => hex::encode(&proposer.encode()[..8]),
    );
}

/// Emit telemetry for block import
pub fn telemetry_block_imported(
    telemetry: &Option<TelemetryHandle>,
    block_hash: &Hash,
    block_number: BlockNumber,
    is_new_best: bool,
) {
    telemetry!(
        telemetry;
        CONSENSUS_DEBUG;
        "asf.block_imported";
        "block_hash" => ?block_hash,
        "block_number" => block_number,
        "is_new_best" => is_new_best,
    );
}

/// Emit telemetry for epoch transition
pub fn telemetry_epoch_transition(
    telemetry: &Option<TelemetryHandle>,
    old_epoch: u32,
    new_epoch: u32,
    block_number: BlockNumber,
) {
    telemetry!(
        telemetry;
        CONSENSUS_INFO;
        "asf.epoch_transition";
        "old_epoch" => old_epoch,
        "new_epoch" => new_epoch,
        "block_number" => block_number,
    );
}

/// Emit telemetry for validator health check
pub fn telemetry_validator_health(
    telemetry: &Option<TelemetryHandle>,
    validator: &ValidatorId,
    is_online: bool,
    uptime_percentage: f64,
) {
    telemetry!(
        telemetry;
        CONSENSUS_DEBUG;
        "asf.validator_health";
        "validator" => hex::encode(&validator.encode()[..8]),
        "online" => is_online,
        "uptime_pct" => uptime_percentage,
    );
}

/// Emit telemetry for consensus metrics snapshot
pub fn telemetry_consensus_metrics(
    telemetry: &Option<TelemetryHandle>,
    epoch: u32,
    active_validators: u32,
    total_stake: u128,
    blocks_finalized: u64,
    certificates_issued: u64,
    avg_finality_time_ms: u64,
) {
    telemetry!(
        telemetry;
        CONSENSUS_INFO;
        "asf.consensus_metrics";
        "epoch" => epoch,
        "active_validators" => active_validators,
        "total_stake" => total_stake,
        "blocks_finalized" => blocks_finalized,
        "certificates_issued" => certificates_issued,
        "avg_finality_time_ms" => avg_finality_time_ms,
    );
}

/// Emit telemetry for network sync status
pub fn telemetry_sync_status(
    telemetry: &Option<TelemetryHandle>,
    is_syncing: bool,
    best_block: BlockNumber,
    target_block: BlockNumber,
    peers_count: usize,
) {
    telemetry!(
        telemetry;
        CONSENSUS_DEBUG;
        "asf.sync_status";
        "syncing" => is_syncing,
        "best_block" => best_block,
        "target_block" => target_block,
        "peers" => peers_count,
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// TELEMETRY AGGREGATOR
// ═══════════════════════════════════════════════════════════════════════════════

/// Aggregates telemetry metrics for periodic reporting
pub struct TelemetryAggregator {
    // Vote metrics
    votes_received: u64,
    votes_submitted: u64,

    // Certificate metrics
    certificates_generated: u64,
    certificates_broadcast: u64,

    // Finality metrics
    blocks_finalized: u64,
    avg_finality_time_ms: u64,
    finality_time_samples: Vec<u64>,

    // Slashing metrics
    slashing_events: u64,
    byzantine_detections: u64,

    // Block production metrics
    blocks_produced: u64,
    blocks_imported: u64,

    // Epoch
    current_epoch: u32,
}

impl TelemetryAggregator {
    /// Create a new telemetry aggregator
    pub fn new() -> Self {
        Self {
            votes_received: 0,
            votes_submitted: 0,
            certificates_generated: 0,
            certificates_broadcast: 0,
            blocks_finalized: 0,
            avg_finality_time_ms: 0,
            finality_time_samples: Vec::new(),
            slashing_events: 0,
            byzantine_detections: 0,
            blocks_produced: 0,
            blocks_imported: 0,
            current_epoch: 0,
        }
    }

    /// Record vote received
    pub fn record_vote_received(&mut self) {
        self.votes_received += 1;
    }

    /// Record vote submitted
    pub fn record_vote_submitted(&mut self) {
        self.votes_submitted += 1;
    }

    /// Record certificate generated
    pub fn record_certificate_generated(&mut self) {
        self.certificates_generated += 1;
    }

    /// Record certificate broadcast
    pub fn record_certificate_broadcast(&mut self) {
        self.certificates_broadcast += 1;
    }

    /// Record block finalized
    pub fn record_block_finalized(&mut self, time_to_finalization_ms: u64) {
        self.blocks_finalized += 1;
        self.finality_time_samples.push(time_to_finalization_ms);

        // Keep last 100 samples
        if self.finality_time_samples.len() > 100 {
            self.finality_time_samples.remove(0);
        }

        // Update average
        self.avg_finality_time_ms =
            self.finality_time_samples.iter().sum::<u64>() / self.finality_time_samples.len() as u64;
    }

    /// Record slashing event
    pub fn record_slashing_event(&mut self) {
        self.slashing_events += 1;
    }

    /// Record Byzantine detection
    pub fn record_byzantine_detection(&mut self) {
        self.byzantine_detections += 1;
    }

    /// Record block produced
    pub fn record_block_produced(&mut self) {
        self.blocks_produced += 1;
    }

    /// Record block imported
    pub fn record_block_imported(&mut self) {
        self.blocks_imported += 1;
    }

    /// Update epoch
    pub fn update_epoch(&mut self, epoch: u32) {
        self.current_epoch = epoch;
    }

    /// Get snapshot of current metrics
    pub fn snapshot(&self) -> TelemetrySnapshot {
        TelemetrySnapshot {
            votes_received: self.votes_received,
            votes_submitted: self.votes_submitted,
            certificates_generated: self.certificates_generated,
            certificates_broadcast: self.certificates_broadcast,
            blocks_finalized: self.blocks_finalized,
            avg_finality_time_ms: self.avg_finality_time_ms,
            slashing_events: self.slashing_events,
            byzantine_detections: self.byzantine_detections,
            blocks_produced: self.blocks_produced,
            blocks_imported: self.blocks_imported,
            current_epoch: self.current_epoch,
        }
    }

    /// Reset counters (for new epoch)
    pub fn reset(&mut self) {
        self.votes_received = 0;
        self.votes_submitted = 0;
        self.certificates_generated = 0;
        self.certificates_broadcast = 0;
        self.blocks_produced = 0;
        self.blocks_imported = 0;
        // Keep finality metrics and epoch
    }
}

impl Default for TelemetryAggregator {
    fn default() -> Self {
        Self::new()
    }
}

/// Snapshot of telemetry metrics
#[derive(Debug, Clone)]
pub struct TelemetrySnapshot {
    pub votes_received: u64,
    pub votes_submitted: u64,
    pub certificates_generated: u64,
    pub certificates_broadcast: u64,
    pub blocks_finalized: u64,
    pub avg_finality_time_ms: u64,
    pub slashing_events: u64,
    pub byzantine_detections: u64,
    pub blocks_produced: u64,
    pub blocks_imported: u64,
    pub current_epoch: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telemetry_aggregator() {
        let mut agg = TelemetryAggregator::new();

        agg.record_vote_received();
        agg.record_vote_received();
        agg.record_vote_submitted();

        assert_eq!(agg.votes_received, 2);
        assert_eq!(agg.votes_submitted, 1);

        let snapshot = agg.snapshot();
        assert_eq!(snapshot.votes_received, 2);
        assert_eq!(snapshot.votes_submitted, 1);
    }

    #[test]
    fn test_finality_time_tracking() {
        let mut agg = TelemetryAggregator::new();

        agg.record_block_finalized(1000);
        agg.record_block_finalized(2000);
        agg.record_block_finalized(3000);

        assert_eq!(agg.avg_finality_time_ms, 2000); // (1000+2000+3000)/3
        assert_eq!(agg.blocks_finalized, 3);
    }

    #[test]
    fn test_epoch_tracking() {
        let mut agg = TelemetryAggregator::new();
        assert_eq!(agg.current_epoch, 0);

        agg.update_epoch(5);
        assert_eq!(agg.current_epoch, 5);
    }
}
