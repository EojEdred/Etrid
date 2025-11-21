// ═══════════════════════════════════════════════════════════════════════════
// RATE LIMITING - DoS Protection for Checkpoint Signature Collection
// ═══════════════════════════════════════════════════════════════════════════
//
// Prevents network flooding attacks by rate limiting signature submissions:
// - Per-validator rate limits (max 5 signatures per 10 seconds)
// - Per-IP rate limits for invalid signatures (max 10 per minute)
// - Auto-ban IPs exceeding limits
// - Fast validation pipeline (fail fast on cheap checks)
//
// Security Properties:
// - Prevents DoS via signature flooding
// - Protects against invalid signature spam
// - Resource-efficient validation order
//
// ═══════════════════════════════════════════════════════════════════════════

use parking_lot::RwLock;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::{CheckpointCertificate, CheckpointCollector, CheckpointSignature};

/// Rate-limited checkpoint collector
pub struct RateLimitedCollector {
    /// Underlying collector
    collector: Arc<CheckpointCollector>,

    /// Per-validator rate limits
    validator_rate_limiter: Arc<RwLock<HashMap<u32, RateLimit>>>,

    /// Per-IP rate limits (for invalid signatures)
    ip_rate_limiter: Arc<RwLock<HashMap<String, RateLimit>>>,

    /// Banned IPs (exceeded limits)
    banned_ips: Arc<RwLock<HashSet<String>>>,

    /// Failed validation attempts per IP
    ip_failures: Arc<RwLock<HashMap<String, u32>>>,

    /// Configuration
    config: RateLimitConfig,
}

/// Rate limit tracking
#[derive(Debug, Clone)]
struct RateLimit {
    count: u32,
    window_start: u64,
    window_duration_ms: u64,
    max_per_window: u32,
}

impl RateLimit {
    fn new(window_duration_ms: u64, max_per_window: u32) -> Self {
        Self {
            count: 0,
            window_start: Self::current_time_ms(),
            window_duration_ms,
            max_per_window,
        }
    }

    fn current_time_ms() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }

    fn check_and_increment(&mut self) -> Result<(), String> {
        let now = Self::current_time_ms();

        // Reset window if expired
        if now - self.window_start >= self.window_duration_ms {
            self.count = 0;
            self.window_start = now;
        }

        // Check limit
        if self.count >= self.max_per_window {
            let reset_in = self.window_start + self.window_duration_ms - now;
            return Err(format!(
                "Rate limit exceeded. Window resets in {}ms",
                reset_in
            ));
        }

        self.count += 1;
        Ok(())
    }

    fn remaining(&self) -> u32 {
        self.max_per_window.saturating_sub(self.count)
    }
}

/// Rate limit configuration
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Validator rate limit: max signatures per window
    pub validator_max_per_window: u32,

    /// Validator rate limit: window duration in milliseconds
    pub validator_window_ms: u64,

    /// IP rate limit: max invalid signatures per window
    pub ip_max_failures_per_window: u32,

    /// IP rate limit: window duration in milliseconds
    pub ip_window_ms: u64,

    /// Auto-ban threshold: ban IP after this many failures
    pub ip_auto_ban_threshold: u32,

    /// Enable IP-based rate limiting
    pub enable_ip_rate_limiting: bool,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            validator_max_per_window: 5,   // Max 5 signatures
            validator_window_ms: 10_000,   // Per 10 seconds
            ip_max_failures_per_window: 10, // Max 10 invalid signatures
            ip_window_ms: 60_000,          // Per minute
            ip_auto_ban_threshold: 20,     // Ban after 20 failures
            enable_ip_rate_limiting: true,
        }
    }
}

impl RateLimitedCollector {
    /// Create new rate-limited collector
    pub fn new(collector: CheckpointCollector, config: RateLimitConfig) -> Self {
        Self {
            collector: Arc::new(collector),
            validator_rate_limiter: Arc::new(RwLock::new(HashMap::new())),
            ip_rate_limiter: Arc::new(RwLock::new(HashMap::new())),
            banned_ips: Arc::new(RwLock::new(HashSet::new())),
            ip_failures: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Add signature with rate limiting and fast validation pipeline
    pub fn add_signature(
        &self,
        signature: CheckpointSignature,
        source_ip: Option<String>,
    ) -> Result<Option<CheckpointCertificate>, String> {
        // STEP 1: Check IP ban (cheapest check)
        if let Some(ref ip) = source_ip {
            self.check_ip_ban(ip)?;
        }

        // STEP 2: Check validator rate limit (very cheap)
        self.check_validator_rate_limit(signature.validator_id)?;

        // STEP 3: Validate signature (expensive - do last)
        // Delegate to underlying collector which does crypto verification
        let result = self.collector.add_signature(signature.clone());

        // STEP 4: Track failures for IP rate limiting
        if let Err(ref e) = result {
            if let Some(ip) = source_ip {
                self.track_ip_failure(ip, e);
            }
        }

        result
    }

    /// Check validator rate limit (max 5 signatures per 10 seconds)
    fn check_validator_rate_limit(&self, validator_id: u32) -> Result<(), String> {
        let mut rate_limiter = self.validator_rate_limiter.write();

        let limit = rate_limiter
            .entry(validator_id)
            .or_insert_with(|| {
                RateLimit::new(
                    self.config.validator_window_ms,
                    self.config.validator_max_per_window,
                )
            });

        match limit.check_and_increment() {
            Ok(()) => {
                tracing::debug!(
                    "Validator {} rate limit: {}/{} remaining",
                    validator_id,
                    limit.remaining(),
                    self.config.validator_max_per_window
                );
                Ok(())
            }
            Err(e) => {
                tracing::warn!(
                    "⚠️ RATE LIMIT: Validator {} exceeded limit ({})",
                    validator_id,
                    e
                );
                Err(format!("Validator {} rate limit: {}", validator_id, e))
            }
        }
    }

    /// Track failed validation attempts per IP
    fn track_ip_failure(&self, ip: String, error: &str) {
        if !self.config.enable_ip_rate_limiting {
            return;
        }

        let mut failures = self.ip_failures.write();
        let count = failures.entry(ip.clone()).or_insert(0);
        *count += 1;

        tracing::debug!(
            "IP {} failed validation: {} (total failures: {})",
            ip,
            error,
            count
        );

        // Check auto-ban threshold
        if *count >= self.config.ip_auto_ban_threshold {
            self.ban_ip(ip.clone());
        } else {
            // Check IP rate limit
            let mut ip_limiter = self.ip_rate_limiter.write();
            let limit = ip_limiter.entry(ip.clone()).or_insert_with(|| {
                RateLimit::new(
                    self.config.ip_window_ms,
                    self.config.ip_max_failures_per_window,
                )
            });

            if limit.check_and_increment().is_err() {
                tracing::warn!("⚠️ IP {} exceeded failure rate limit, banning", ip);
                drop(ip_limiter);
                self.ban_ip(ip);
            }
        }
    }

    /// Ban IP if too many invalid signatures
    fn ban_ip(&self, ip: String) {
        let mut banned = self.banned_ips.write();
        if banned.insert(ip.clone()) {
            tracing::error!(
                "🚨 BANNED IP: {} exceeded validation failure threshold",
                ip
            );
        }
    }

    /// Check if IP is banned
    fn check_ip_ban(&self, ip: &str) -> Result<(), String> {
        if !self.config.enable_ip_rate_limiting {
            return Ok(());
        }

        if self.banned_ips.read().contains(ip) {
            Err(format!("IP {} is banned", ip))
        } else {
            Ok(())
        }
    }

    /// Manually unban IP (for admin operations)
    pub fn unban_ip(&self, ip: &str) -> bool {
        let removed = self.banned_ips.write().remove(ip);
        if removed {
            // Reset failure count
            self.ip_failures.write().remove(ip);
            tracing::info!("🔓 Unbanned IP: {}", ip);
        }
        removed
    }

    /// Get banned IPs
    pub fn get_banned_ips(&self) -> Vec<String> {
        self.banned_ips.read().iter().cloned().collect()
    }

    /// Get IP failure count
    pub fn get_ip_failure_count(&self, ip: &str) -> u32 {
        self.ip_failures.read().get(ip).cloned().unwrap_or(0)
    }

    /// Get validator rate limit status
    pub fn get_validator_rate_status(&self, validator_id: u32) -> Option<(u32, u32)> {
        self.validator_rate_limiter
            .read()
            .get(&validator_id)
            .map(|limit| (limit.count, limit.max_per_window))
    }

    /// Reset validator rate limit (for testing/admin)
    pub fn reset_validator_rate_limit(&self, validator_id: u32) {
        self.validator_rate_limiter.write().remove(&validator_id);
        tracing::info!("🔄 Reset rate limit for validator {}", validator_id);
    }

    /// Get underlying collector (for read operations)
    pub fn collector(&self) -> &CheckpointCollector {
        &self.collector
    }

    /// Cleanup old rate limit data
    pub fn cleanup_rate_limiters(&self) {
        let now = RateLimit::current_time_ms();

        // Clean expired validator rate limits
        let mut validator_limiter = self.validator_rate_limiter.write();
        validator_limiter.retain(|_, limit| {
            now - limit.window_start < limit.window_duration_ms * 2
        });

        // Clean expired IP rate limits
        let mut ip_limiter = self.ip_rate_limiter.write();
        ip_limiter.retain(|_, limit| {
            now - limit.window_start < limit.window_duration_ms * 2
        });

        tracing::debug!("🧹 Cleaned expired rate limiters");
    }

    /// Generate rate limiting report
    pub fn generate_report(&self) -> RateLimitReport {
        let validator_status: Vec<_> = self
            .validator_rate_limiter
            .read()
            .iter()
            .map(|(id, limit)| ValidatorRateStatus {
                validator_id: *id,
                current_count: limit.count,
                max_count: limit.max_per_window,
                window_start: limit.window_start,
                window_duration_ms: limit.window_duration_ms,
            })
            .collect();

        let banned_ips = self.get_banned_ips();
        let ip_failures: HashMap<String, u32> = self.ip_failures.read().clone();

        RateLimitReport {
            validator_status,
            banned_ips,
            ip_failures,
            config: self.config.clone(),
        }
    }
}

/// Rate limit report
#[derive(Debug, Clone)]
pub struct RateLimitReport {
    pub validator_status: Vec<ValidatorRateStatus>,
    pub banned_ips: Vec<String>,
    pub ip_failures: HashMap<String, u32>,
    pub config: RateLimitConfig,
}

#[derive(Debug, Clone)]
pub struct ValidatorRateStatus {
    pub validator_id: u32,
    pub current_count: u32,
    pub max_count: u32,
    pub window_start: u64,
    pub window_duration_ms: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AuthoritySet;
    use sp_core::ed25519::Pair;
    use sp_core::Pair as PairT;

    fn create_test_signature(
        validator_id: u32,
        block_number: u32,
        block_hash: [u8; 32],
        _pair: &Pair,
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
    fn test_validator_rate_limit() {
        let pairs: Vec<_> = (0..5)
            .map(|i| Pair::from_string(&format!("//Validator{}", i), None).unwrap())
            .collect();

        let authorities: Vec<[u8; 32]> = pairs.iter().map(|p| p.public().0).collect();
        let authority_set = AuthoritySet::new(1, authorities);
        let collector = CheckpointCollector::new(authority_set);

        let mut config = RateLimitConfig::default();
        config.validator_max_per_window = 3; // Max 3 signatures

        let rate_limited = RateLimitedCollector::new(collector, config);

        let block_hash = [1u8; 32];
        let block_number = 16;

        // Add 3 signatures - should succeed
        for _ in 0..3 {
            let sig = create_test_signature(0, block_number, block_hash, &pairs[0]);
            assert!(rate_limited.add_signature(sig, None).is_ok());
        }

        // 4th signature - should fail rate limit
        let sig = create_test_signature(0, block_number, block_hash, &pairs[0]);
        let result = rate_limited.add_signature(sig, None);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("rate limit"));
    }

    #[test]
    fn test_ip_ban() {
        let pairs: Vec<_> = (0..5)
            .map(|i| Pair::from_string(&format!("//Validator{}", i), None).unwrap())
            .collect();

        let authorities: Vec<[u8; 32]> = pairs.iter().map(|p| p.public().0).collect();
        let authority_set = AuthoritySet::new(1, authorities);
        let collector = CheckpointCollector::new(authority_set);

        let mut config = RateLimitConfig::default();
        config.ip_auto_ban_threshold = 5; // Ban after 5 failures

        let rate_limited = RateLimitedCollector::new(collector, config);

        let block_hash = [1u8; 32];
        let block_number = 16;
        let bad_ip = "192.168.1.100".to_string();

        // Submit 5 invalid signatures (invalid validator_id)
        for _ in 0..5 {
            let mut sig = create_test_signature(0, block_number, block_hash, &pairs[0]);
            sig.validator_id = 999; // Invalid
            rate_limited.add_signature(sig, Some(bad_ip.clone())).ok();
        }

        // IP should be banned
        assert!(rate_limited.get_banned_ips().contains(&bad_ip));

        // Further signatures from this IP should be rejected immediately
        let sig = create_test_signature(0, block_number, block_hash, &pairs[0]);
        let result = rate_limited.add_signature(sig, Some(bad_ip.clone()));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("banned"));
    }

    #[test]
    fn test_unban_ip() {
        let pairs: Vec<_> = (0..5)
            .map(|i| Pair::from_string(&format!("//Validator{}", i), None).unwrap())
            .collect();

        let authorities: Vec<[u8; 32]> = pairs.iter().map(|p| p.public().0).collect();
        let authority_set = AuthoritySet::new(1, authorities);
        let collector = CheckpointCollector::new(authority_set);

        let config = RateLimitConfig::default();
        let rate_limited = RateLimitedCollector::new(collector, config);

        let ip = "192.168.1.100".to_string();

        // Manually ban IP
        rate_limited.ban_ip(ip.clone());
        assert!(rate_limited.get_banned_ips().contains(&ip));

        // Unban
        assert!(rate_limited.unban_ip(&ip));
        assert!(!rate_limited.get_banned_ips().contains(&ip));
    }

    #[test]
    fn test_rate_limit_report() {
        let pairs: Vec<_> = (0..5)
            .map(|i| Pair::from_string(&format!("//Validator{}", i), None).unwrap())
            .collect();

        let authorities: Vec<[u8; 32]> = pairs.iter().map(|p| p.public().0).collect();
        let authority_set = AuthoritySet::new(1, authorities);
        let collector = CheckpointCollector::new(authority_set);

        let config = RateLimitConfig::default();
        let rate_limited = RateLimitedCollector::new(collector, config);

        // Add some signatures
        let block_hash = [1u8; 32];
        let block_number = 16;
        for i in 0..3 {
            let sig = create_test_signature(i, block_number, block_hash, &pairs[i as usize]);
            rate_limited.add_signature(sig, None).ok();
        }

        let report = rate_limited.generate_report();
        assert_eq!(report.validator_status.len(), 3);
    }
}
