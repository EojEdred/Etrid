// ═══════════════════════════════════════════════════════════════════════════
// ECLIPSE ATTACK DETECTION - Detect Network Isolation Attacks
// ═══════════════════════════════════════════════════════════════════════════
//
// Detects when a validator is isolated from the honest network (eclipse attack).
// An attacker controls all peers connected to the victim, feeding false data.
//
// Detection Strategy:
// - Track unique IP sources for signatures
// - Require minimum 5 unique sources for certificates
// - Warn if signatures only from 1-2 IPs
// - Error if certificate only from single source
//
// Security Properties:
// - Detects when validator isolated from honest network
// - Prevents accepting certificates from single source
// - Provides early warning of potential eclipse attack
//
// ═══════════════════════════════════════════════════════════════════════════

use parking_lot::RwLock;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::CheckpointSignature;

/// Minimum unique IP sources required for certificate acceptance
const MIN_UNIQUE_SOURCES: usize = 5;

/// Warning threshold for low diversity
const WARNING_THRESHOLD: usize = 2;

/// Eclipse attack detector
pub struct EclipseDetector {
    /// Track unique sources of signatures: validator_id -> IP addresses
    signature_sources: Arc<RwLock<HashMap<u32, HashSet<String>>>>,

    /// Track certificate sources: block_number -> IP addresses
    certificate_sources: Arc<RwLock<HashMap<u32, HashSet<String>>>>,

    /// Track all signatures received per block: block_number -> block_hash -> IPs
    block_signature_sources: Arc<RwLock<HashMap<u32, HashMap<[u8; 32], HashSet<String>>>>>,

    /// Minimum unique sources required
    min_unique_sources: usize,

    /// Warning threshold
    warning_threshold: usize,

    /// Eclipse attack warnings (block_number, reason)
    warnings: Arc<RwLock<Vec<(u32, String)>>>,
}

impl EclipseDetector {
    /// Create new eclipse detector
    pub fn new(min_unique_sources: usize, warning_threshold: usize) -> Self {
        Self {
            signature_sources: Arc::new(RwLock::new(HashMap::new())),
            certificate_sources: Arc::new(RwLock::new(HashMap::new())),
            block_signature_sources: Arc::new(RwLock::new(HashMap::new())),
            min_unique_sources,
            warning_threshold,
            warnings: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Create with default settings
    pub fn default() -> Self {
        Self::new(MIN_UNIQUE_SOURCES, WARNING_THRESHOLD)
    }

    /// Validate signature source diversity
    pub fn validate_signature_diversity(
        &self,
        signature: &CheckpointSignature,
        source_ip: String,
    ) -> Result<(), String> {
        let validator_id = signature.validator_id;
        let block_number = signature.block_number;
        let block_hash = signature.block_hash;

        // Track signature source for this validator
        let mut sig_sources = self.signature_sources.write();
        let sources = sig_sources.entry(validator_id).or_insert_with(HashSet::new);
        sources.insert(source_ip.clone());

        // Track signature source for this block
        let mut block_sources = self.block_signature_sources.write();
        let block_entry = block_sources.entry(block_number).or_insert_with(HashMap::new);
        let hash_entry = block_entry
            .entry(block_hash)
            .or_insert_with(HashSet::new);
        hash_entry.insert(source_ip.clone());

        let unique_sources = hash_entry.len();

        // Check diversity
        if unique_sources < self.warning_threshold {
            let warning = format!(
                "Low signature source diversity for block #{}: only {} unique sources",
                block_number, unique_sources
            );

            tracing::warn!(
                "⚠️ ECLIPSE RISK: {}",
                warning
            );

            self.warnings.write().push((block_number, warning.clone()));

            // Don't reject yet, but warn
            tracing::warn!(
                "Validator {} signature for block #{} from IP {} (total unique sources: {})",
                validator_id,
                block_number,
                source_ip,
                unique_sources
            );
        }

        Ok(())
    }

    /// Validate certificate source diversity (strict check)
    pub fn validate_certificate_diversity(
        &self,
        block_number: u32,
        block_hash: [u8; 32],
        source_ip: String,
    ) -> Result<(), String> {
        // Track certificate source
        let mut cert_sources = self.certificate_sources.write();
        let sources = cert_sources.entry(block_number).or_insert_with(HashSet::new);
        sources.insert(source_ip.clone());

        // Check against block signature sources
        let block_sources = self.block_signature_sources.read();
        let unique_sources = block_sources
            .get(&block_number)
            .and_then(|blocks| blocks.get(&block_hash))
            .map(|ips| ips.len())
            .unwrap_or(1);

        // Strict check for certificates
        if unique_sources < self.min_unique_sources {
            let error = format!(
                "Certificate source diversity too low for block #{}: {} sources (minimum: {})",
                block_number, unique_sources, self.min_unique_sources
            );

            tracing::error!(
                "🚨 ECLIPSE ATTACK SUSPECTED: {}",
                error
            );

            return Err(error);
        }

        tracing::debug!(
            "✅ Certificate for block #{} has sufficient source diversity: {} unique sources",
            block_number,
            unique_sources
        );

        Ok(())
    }

    /// Check if likely under eclipse attack
    pub fn check_eclipse_risk(&self) -> bool {
        // Check recent warnings
        let warnings = self.warnings.read();
        let recent_warnings = warnings.iter().rev().take(10).count();

        if recent_warnings >= 5 {
            tracing::error!(
                "🚨 HIGH ECLIPSE RISK: {} recent low-diversity warnings",
                recent_warnings
            );
            return true;
        }

        // Check if any validator has very few unique sources
        let sig_sources = self.signature_sources.read();
        for (validator_id, sources) in sig_sources.iter() {
            if sources.len() == 1 {
                tracing::warn!(
                    "⚠️ ECLIPSE RISK: Validator {} has only seen signatures from 1 IP",
                    validator_id
                );
                return true;
            }
        }

        false
    }

    /// Get unique source count for validator
    pub fn get_validator_source_count(&self, validator_id: u32) -> usize {
        self.signature_sources
            .read()
            .get(&validator_id)
            .map(|sources| sources.len())
            .unwrap_or(0)
    }

    /// Get unique source count for block
    pub fn get_block_source_count(&self, block_number: u32, block_hash: [u8; 32]) -> usize {
        self.block_signature_sources
            .read()
            .get(&block_number)
            .and_then(|blocks| blocks.get(&block_hash))
            .map(|ips| ips.len())
            .unwrap_or(0)
    }

    /// Get unique sources for validator
    pub fn get_validator_sources(&self, validator_id: u32) -> Vec<String> {
        self.signature_sources
            .read()
            .get(&validator_id)
            .map(|sources| sources.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// Get unique sources for block
    pub fn get_block_sources(&self, block_number: u32, block_hash: [u8; 32]) -> Vec<String> {
        self.block_signature_sources
            .read()
            .get(&block_number)
            .and_then(|blocks| blocks.get(&block_hash))
            .map(|ips| ips.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// Get recent warnings
    pub fn get_recent_warnings(&self, limit: usize) -> Vec<(u32, String)> {
        let warnings = self.warnings.read();
        warnings.iter().rev().take(limit).cloned().collect()
    }

    /// Clear old warnings
    pub fn clear_old_warnings(&self, keep_last_n: usize) {
        let mut warnings = self.warnings.write();
        if warnings.len() > keep_last_n {
            *warnings = warnings
                .iter()
                .rev()
                .take(keep_last_n)
                .cloned()
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect();
        }
    }

    /// Cleanup old data (prevent memory bloat)
    pub fn cleanup_old_data(&self, current_block: u32, keep_blocks: u32) {
        let cutoff = current_block.saturating_sub(keep_blocks);

        // Clean certificate sources
        self.certificate_sources
            .write()
            .retain(|&block_number, _| block_number >= cutoff);

        // Clean block signature sources
        self.block_signature_sources
            .write()
            .retain(|&block_number, _| block_number >= cutoff);

        tracing::debug!(
            "🧹 Cleaned eclipse detector data older than block #{}",
            cutoff
        );
    }

    /// Generate eclipse detection report
    pub fn generate_report(&self) -> EclipseReport {
        let sig_sources = self.signature_sources.read();
        let mut validator_diversity = Vec::new();

        for (validator_id, sources) in sig_sources.iter() {
            validator_diversity.push(ValidatorDiversity {
                validator_id: *validator_id,
                unique_sources: sources.len(),
                sources: sources.iter().cloned().collect(),
                is_at_risk: sources.len() < self.warning_threshold,
            });
        }

        // Sort by diversity (lowest first - highest risk)
        validator_diversity.sort_by_key(|v| v.unique_sources);

        let block_sources = self.block_signature_sources.read();
        let mut block_diversity = Vec::new();

        for (block_number, blocks) in block_sources.iter() {
            for (block_hash, sources) in blocks.iter() {
                block_diversity.push(BlockDiversity {
                    block_number: *block_number,
                    block_hash: *block_hash,
                    unique_sources: sources.len(),
                    sources: sources.iter().cloned().collect(),
                    is_at_risk: sources.len() < self.min_unique_sources,
                });
            }
        }

        let recent_warnings = self.get_recent_warnings(20);
        let eclipse_risk = self.check_eclipse_risk();

        EclipseReport {
            validator_diversity,
            block_diversity,
            recent_warnings,
            eclipse_risk,
            min_unique_sources: self.min_unique_sources,
            warning_threshold: self.warning_threshold,
        }
    }
}

/// Eclipse detection report
#[derive(Debug, Clone)]
pub struct EclipseReport {
    pub validator_diversity: Vec<ValidatorDiversity>,
    pub block_diversity: Vec<BlockDiversity>,
    pub recent_warnings: Vec<(u32, String)>,
    pub eclipse_risk: bool,
    pub min_unique_sources: usize,
    pub warning_threshold: usize,
}

#[derive(Debug, Clone)]
pub struct ValidatorDiversity {
    pub validator_id: u32,
    pub unique_sources: usize,
    pub sources: Vec<String>,
    pub is_at_risk: bool,
}

#[derive(Debug, Clone)]
pub struct BlockDiversity {
    pub block_number: u32,
    pub block_hash: [u8; 32],
    pub unique_sources: usize,
    pub sources: Vec<String>,
    pub is_at_risk: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use sp_core::ed25519::Pair;
    use sp_core::Pair as PairT;

    fn create_test_signature(
        validator_id: u32,
        block_number: u32,
        block_hash: [u8; 32],
    ) -> CheckpointSignature {
        CheckpointSignature {
            chain_id: [0u8; 32],
            block_number,
            block_hash,
            validator_id,
            validator_pubkey: [0u8; 32],
            authority_set_id: 1,
            authority_set_hash: [0u8; 32],
            checkpoint_type: crate::vrf::CheckpointType::Guaranteed,
            signature_nonce: 1,
            signature: vec![0u8; 64],
            timestamp_ms: 0,
        }
    }

    #[test]
    fn test_signature_diversity_tracking() {
        let detector = EclipseDetector::default();

        let sig = create_test_signature(0, 16, [1u8; 32]);

        // Add signatures from different IPs
        detector
            .validate_signature_diversity(&sig, "192.168.1.1".to_string())
            .ok();
        detector
            .validate_signature_diversity(&sig, "192.168.1.2".to_string())
            .ok();
        detector
            .validate_signature_diversity(&sig, "192.168.1.3".to_string())
            .ok();

        assert_eq!(detector.get_validator_source_count(0), 3);
        assert_eq!(detector.get_block_source_count(16, [1u8; 32]), 3);
    }

    #[test]
    fn test_low_diversity_warning() {
        let detector = EclipseDetector::default();

        let sig = create_test_signature(0, 16, [1u8; 32]);

        // All signatures from same IP - should warn
        detector
            .validate_signature_diversity(&sig, "192.168.1.1".to_string())
            .ok();

        let warnings = detector.get_recent_warnings(10);
        assert!(!warnings.is_empty());
    }

    #[test]
    fn test_certificate_diversity_check() {
        let detector = EclipseDetector::default();

        let block_number = 16;
        let block_hash = [1u8; 32];

        // Add signatures from 5 different IPs
        for i in 1..=5 {
            let sig = create_test_signature(i, block_number, block_hash);
            let ip = format!("192.168.1.{}", i);
            detector.validate_signature_diversity(&sig, ip).ok();
        }

        // Certificate should pass diversity check
        let result = detector.validate_certificate_diversity(
            block_number,
            block_hash,
            "192.168.1.10".to_string(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_certificate_diversity_failure() {
        let detector = EclipseDetector::default();

        let block_number = 16;
        let block_hash = [1u8; 32];

        // Add signatures from only 2 IPs (below minimum)
        for i in 1..=2 {
            let sig = create_test_signature(i, block_number, block_hash);
            let ip = format!("192.168.1.{}", i);
            detector.validate_signature_diversity(&sig, ip).ok();
        }

        // Certificate should fail diversity check
        let result = detector.validate_certificate_diversity(
            block_number,
            block_hash,
            "192.168.1.10".to_string(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_eclipse_risk_detection() {
        let detector = EclipseDetector::default();

        let sig = create_test_signature(0, 16, [1u8; 32]);

        // All signatures from single IP
        for _ in 0..10 {
            detector
                .validate_signature_diversity(&sig, "192.168.1.1".to_string())
                .ok();
        }

        // Should detect eclipse risk
        assert!(detector.check_eclipse_risk());
    }

    #[test]
    fn test_eclipse_report() {
        let detector = EclipseDetector::default();

        // Add diverse sources for validator 0
        for i in 1..=5 {
            let sig = create_test_signature(0, 16, [1u8; 32]);
            let ip = format!("192.168.1.{}", i);
            detector.validate_signature_diversity(&sig, ip).ok();
        }

        // Add single source for validator 1 (at risk)
        let sig = create_test_signature(1, 16, [1u8; 32]);
        detector
            .validate_signature_diversity(&sig, "192.168.1.100".to_string())
            .ok();

        let report = detector.generate_report();

        // Validator 1 should be at risk
        // SAFETY: Test data always includes validator 1
        let val1_report = report
            .validator_diversity
            .iter()
            .find(|v| v.validator_id == 1)
            .unwrap();
        assert!(val1_report.is_at_risk);
    }
}
