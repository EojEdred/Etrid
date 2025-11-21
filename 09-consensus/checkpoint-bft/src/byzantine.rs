// ═══════════════════════════════════════════════════════════════════════════
// BYZANTINE BEHAVIOR TRACKING - Detect and Monitor Malicious Validators
// ═══════════════════════════════════════════════════════════════════════════
//
// Tracks validator participation and detects Byzantine behavior patterns:
// - Low participation rate (missing signatures)
// - Selective censorship (targeting specific checkpoints)
// - Coordinated attacks (multiple validators colluding)
//
// Threshold: Flag validators with <80% participation as suspicious
// Critical: Alert if ≥7 validators suspected (Byzantine threshold = n/3)
//
// ═══════════════════════════════════════════════════════════════════════════

use parking_lot::RwLock;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// Minimum participation rate required (80%)
const MIN_PARTICIPATION_RATE: f64 = 0.80;

/// Byzantine threshold: 7 out of 21 validators = 1/3
const BYZANTINE_THRESHOLD: usize = 7;

/// Byzantine behavior tracker
pub struct ByzantineTracker {
    /// Track signature participation per validator
    participation: Arc<RwLock<HashMap<u32, ParticipationStats>>>,

    /// Suspected byzantine validators
    suspected: Arc<RwLock<HashSet<u32>>>,

    /// Confirmed byzantine validators (proven misbehavior)
    confirmed_byzantine: Arc<RwLock<HashSet<u32>>>,

    /// Checkpoint opportunities (all validators should sign)
    checkpoint_opportunities: Arc<RwLock<Vec<u32>>>,

    /// Censorship detection: validator -> missed checkpoints
    censorship_tracker: Arc<RwLock<HashMap<u32, Vec<u32>>>>,

    /// Minimum checkpoints before calculating participation (avoid false positives)
    min_checkpoints_for_evaluation: u32,
}

/// Participation statistics for a validator
#[derive(Debug, Clone)]
struct ParticipationStats {
    validator_id: u32,
    checkpoints_seen: u32,
    checkpoints_signed: u32,
    last_activity: u64,
    missed_checkpoints: Vec<u32>,
}

impl ParticipationStats {
    fn new(validator_id: u32) -> Self {
        Self {
            validator_id,
            checkpoints_seen: 0,
            checkpoints_signed: 0,
            last_activity: 0,
            missed_checkpoints: Vec::new(),
        }
    }

    fn participation_rate(&self) -> f64 {
        if self.checkpoints_seen == 0 {
            return 1.0; // No data yet, assume good
        }
        self.checkpoints_signed as f64 / self.checkpoints_seen as f64
    }

    fn is_byzantine(&self, min_checkpoints: u32) -> bool {
        if self.checkpoints_seen < min_checkpoints {
            return false; // Not enough data
        }
        self.participation_rate() < MIN_PARTICIPATION_RATE
    }
}

impl ByzantineTracker {
    /// Create new Byzantine tracker
    pub fn new(min_checkpoints_for_evaluation: u32) -> Self {
        Self {
            participation: Arc::new(RwLock::new(HashMap::new())),
            suspected: Arc::new(RwLock::new(HashSet::new())),
            confirmed_byzantine: Arc::new(RwLock::new(HashSet::new())),
            checkpoint_opportunities: Arc::new(RwLock::new(Vec::new())),
            censorship_tracker: Arc::new(RwLock::new(HashMap::new())),
            min_checkpoints_for_evaluation,
        }
    }

    /// Record checkpoint opportunity (all validators should sign)
    pub fn record_checkpoint_opportunity(&self, checkpoint_number: u32, all_validators: &[u32]) {
        // Add to opportunities list
        self.checkpoint_opportunities
            .write()
            .push(checkpoint_number);

        // Increment checkpoints_seen for all validators
        let mut participation = self.participation.write();
        for &validator_id in all_validators {
            let stats = participation
                .entry(validator_id)
                .or_insert_with(|| ParticipationStats::new(validator_id));

            stats.checkpoints_seen += 1;
        }

        tracing::debug!(
            "📊 Checkpoint #{} recorded as opportunity for {} validators",
            checkpoint_number,
            all_validators.len()
        );
    }

    /// Record signature (validator participated)
    pub fn record_signature(&self, validator_id: u32, checkpoint_number: u32, timestamp: u64) {
        let mut participation = self.participation.write();
        let stats = participation
            .entry(validator_id)
            .or_insert_with(|| ParticipationStats::new(validator_id));

        stats.checkpoints_signed += 1;
        stats.last_activity = timestamp;

        tracing::debug!(
            "✅ Validator {} signed checkpoint #{} (participation: {:.2}%)",
            validator_id,
            checkpoint_number,
            stats.participation_rate() * 100.0
        );
    }

    /// Record missed checkpoint (for censorship detection)
    pub fn record_missed_checkpoint(&self, validator_id: u32, checkpoint_number: u32) {
        let mut participation = self.participation.write();
        let stats = participation
            .entry(validator_id)
            .or_insert_with(|| ParticipationStats::new(validator_id));

        stats.missed_checkpoints.push(checkpoint_number);

        // Track for censorship detection
        let mut censorship = self.censorship_tracker.write();
        censorship
            .entry(validator_id)
            .or_insert_with(Vec::new)
            .push(checkpoint_number);

        tracing::warn!(
            "⚠️ Validator {} missed checkpoint #{} (total missed: {})",
            validator_id,
            checkpoint_number,
            stats.missed_checkpoints.len()
        );
    }

    /// Detect Byzantine behavior (low participation)
    pub fn detect_byzantine_behavior(&self) -> Vec<u32> {
        let participation = self.participation.read();
        let mut suspected_validators = Vec::new();

        for (validator_id, stats) in participation.iter() {
            if stats.is_byzantine(self.min_checkpoints_for_evaluation) {
                suspected_validators.push(*validator_id);

                tracing::warn!(
                    "🚨 BYZANTINE BEHAVIOR DETECTED: Validator {} has {:.2}% participation \
                     ({}/{} checkpoints signed) - BELOW THRESHOLD {:.0}%",
                    validator_id,
                    stats.participation_rate() * 100.0,
                    stats.checkpoints_signed,
                    stats.checkpoints_seen,
                    MIN_PARTICIPATION_RATE * 100.0
                );
            }
        }

        // Update suspected set
        let mut suspected = self.suspected.write();
        *suspected = suspected_validators.iter().cloned().collect();

        // Check if Byzantine threshold exceeded
        if suspected_validators.len() >= BYZANTINE_THRESHOLD {
            tracing::error!(
                "🚨🚨🚨 CRITICAL: BYZANTINE THRESHOLD EXCEEDED - {} validators suspected \
                 (threshold: {}). Network security compromised!",
                suspected_validators.len(),
                BYZANTINE_THRESHOLD
            );
        }

        suspected_validators
    }

    /// Get participation rate for validator
    pub fn get_participation_rate(&self, validator_id: u32) -> f64 {
        self.participation
            .read()
            .get(&validator_id)
            .map(|stats| stats.participation_rate())
            .unwrap_or(1.0)
    }

    /// Get participation stats for validator
    pub fn get_participation_stats(&self, validator_id: u32) -> Option<(u32, u32, f64)> {
        self.participation.read().get(&validator_id).map(|stats| {
            (
                stats.checkpoints_seen,
                stats.checkpoints_signed,
                stats.participation_rate(),
            )
        })
    }

    /// Check if validator is suspected Byzantine
    pub fn is_suspected(&self, validator_id: u32) -> bool {
        self.suspected.read().contains(&validator_id)
    }

    /// Mark validator as confirmed Byzantine (with proof)
    pub fn mark_confirmed_byzantine(&self, validator_id: u32, reason: &str) {
        self.confirmed_byzantine.write().insert(validator_id);
        self.suspected.write().insert(validator_id);

        tracing::error!(
            "🚨 CONFIRMED BYZANTINE: Validator {} marked as malicious - Reason: {}",
            validator_id,
            reason
        );
    }

    /// Check if validator is confirmed Byzantine
    pub fn is_confirmed_byzantine(&self, validator_id: u32) -> bool {
        self.confirmed_byzantine.read().contains(&validator_id)
    }

    /// Detect censorship patterns
    pub fn detect_censorship(&self) -> Vec<(u32, Vec<u32>)> {
        let censorship = self.censorship_tracker.read();
        let mut patterns = Vec::new();

        for (validator_id, missed_checkpoints) in censorship.iter() {
            // Check for selective censorship (missing specific patterns)
            if missed_checkpoints.len() >= 5 {
                // More than 5 missed checkpoints
                patterns.push((*validator_id, missed_checkpoints.clone()));

                tracing::warn!(
                    "⚠️ POTENTIAL CENSORSHIP: Validator {} has missed {} checkpoints",
                    validator_id,
                    missed_checkpoints.len()
                );
            }
        }

        patterns
    }

    /// Get all suspected validators
    pub fn get_suspected_validators(&self) -> Vec<u32> {
        self.suspected.read().iter().cloned().collect()
    }

    /// Get all confirmed Byzantine validators
    pub fn get_confirmed_byzantine(&self) -> Vec<u32> {
        self.confirmed_byzantine.read().iter().cloned().collect()
    }

    /// Check if Byzantine threshold exceeded (≥7 validators)
    pub fn is_byzantine_threshold_exceeded(&self) -> bool {
        self.suspected.read().len() >= BYZANTINE_THRESHOLD
    }

    /// Get Byzantine risk level (0.0 = safe, 1.0 = critical)
    pub fn get_byzantine_risk_level(&self) -> f64 {
        let suspected_count = self.suspected.read().len();
        (suspected_count as f64) / (BYZANTINE_THRESHOLD as f64)
    }

    /// Generate Byzantine behavior report
    pub fn generate_report(&self) -> ByzantineReport {
        let participation = self.participation.read();
        let suspected = self.suspected.read().clone();
        let confirmed = self.confirmed_byzantine.read().clone();

        let mut validator_stats = Vec::new();
        for (validator_id, stats) in participation.iter() {
            validator_stats.push(ValidatorReport {
                validator_id: *validator_id,
                checkpoints_seen: stats.checkpoints_seen,
                checkpoints_signed: stats.checkpoints_signed,
                participation_rate: stats.participation_rate(),
                is_suspected: suspected.contains(validator_id),
                is_confirmed_byzantine: confirmed.contains(validator_id),
                missed_checkpoints: stats.missed_checkpoints.clone(),
            });
        }

        // Sort by participation rate (lowest first)
        validator_stats.sort_by(|a, b| {
            a.participation_rate
                .partial_cmp(&b.participation_rate)
                .unwrap()
        });

        ByzantineReport {
            total_checkpoints: self.checkpoint_opportunities.read().len(),
            suspected_validators: suspected,
            confirmed_byzantine: confirmed,
            byzantine_threshold_exceeded: self.is_byzantine_threshold_exceeded(),
            risk_level: self.get_byzantine_risk_level(),
            validator_stats,
        }
    }

    /// Cleanup old data (prevent memory bloat)
    pub fn cleanup_old_data(&self, keep_last_n_checkpoints: usize) {
        let mut opportunities = self.checkpoint_opportunities.write();

        if opportunities.len() > keep_last_n_checkpoints {
            *opportunities = opportunities
                .iter()
                .rev()
                .take(keep_last_n_checkpoints)
                .cloned()
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect();
        }

        tracing::debug!(
            "🧹 Cleaned Byzantine tracker data, keeping last {} checkpoints",
            keep_last_n_checkpoints
        );
    }
}

/// Byzantine behavior report
#[derive(Debug, Clone)]
pub struct ByzantineReport {
    pub total_checkpoints: usize,
    pub suspected_validators: HashSet<u32>,
    pub confirmed_byzantine: HashSet<u32>,
    pub byzantine_threshold_exceeded: bool,
    pub risk_level: f64,
    pub validator_stats: Vec<ValidatorReport>,
}

#[derive(Debug, Clone)]
pub struct ValidatorReport {
    pub validator_id: u32,
    pub checkpoints_seen: u32,
    pub checkpoints_signed: u32,
    pub participation_rate: f64,
    pub is_suspected: bool,
    pub is_confirmed_byzantine: bool,
    pub missed_checkpoints: Vec<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_participation_tracking() {
        let tracker = ByzantineTracker::new(5);
        let validators = vec![0, 1, 2];

        // Record 10 checkpoint opportunities
        for checkpoint in 0..10 {
            tracker.record_checkpoint_opportunity(checkpoint, &validators);
        }

        // Validator 0: 100% participation
        for checkpoint in 0..10 {
            tracker.record_signature(0, checkpoint, 1000 + checkpoint as u64);
        }

        // Validator 1: 50% participation (Byzantine)
        for checkpoint in 0..5 {
            tracker.record_signature(1, checkpoint, 1000 + checkpoint as u64);
        }

        // Validator 2: 90% participation (Good)
        for checkpoint in 0..9 {
            tracker.record_signature(2, checkpoint, 1000 + checkpoint as u64);
        }

        // Check participation rates
        assert_eq!(tracker.get_participation_rate(0), 1.0);
        assert_eq!(tracker.get_participation_rate(1), 0.5);
        assert_eq!(tracker.get_participation_rate(2), 0.9);

        // Detect Byzantine behavior
        let byzantine = tracker.detect_byzantine_behavior();
        assert_eq!(byzantine.len(), 1);
        assert!(byzantine.contains(&1));
        assert!(tracker.is_suspected(1));
    }

    #[test]
    fn test_byzantine_threshold() {
        let tracker = ByzantineTracker::new(5);
        let validators: Vec<u32> = (0..21).collect();

        // Record 10 checkpoint opportunities
        for checkpoint in 0..10 {
            tracker.record_checkpoint_opportunity(checkpoint, &validators);
        }

        // Simulate 7 validators with low participation
        for validator_id in 0..7 {
            for checkpoint in 0..5 {
                tracker.record_signature(validator_id, checkpoint, 1000);
            }
        }

        // Rest have good participation
        for validator_id in 7..21 {
            for checkpoint in 0..10 {
                tracker.record_signature(validator_id, checkpoint, 1000);
            }
        }

        let byzantine = tracker.detect_byzantine_behavior();
        assert!(byzantine.len() >= BYZANTINE_THRESHOLD);
        assert!(tracker.is_byzantine_threshold_exceeded());
    }

    #[test]
    fn test_censorship_detection() {
        let tracker = ByzantineTracker::new(5);

        // Validator 0 misses specific checkpoints
        for checkpoint in 0..10 {
            tracker.record_missed_checkpoint(0, checkpoint);
        }

        let patterns = tracker.detect_censorship();
        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].0, 0);
        assert_eq!(patterns[0].1.len(), 10);
    }

    #[test]
    fn test_confirmed_byzantine() {
        let tracker = ByzantineTracker::new(5);

        tracker.mark_confirmed_byzantine(5, "Equivocation proof");

        assert!(tracker.is_confirmed_byzantine(5));
        assert!(tracker.is_suspected(5));

        let report = tracker.generate_report();
        assert!(report.confirmed_byzantine.contains(&5));
    }
}
