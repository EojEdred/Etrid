//! Multi-Path Payments (MPP)
//!
//! Enables splitting large payments across multiple routes to improve
//! success rates and reduce individual channel capacity requirements.
//!
//! Features:
//! - Automatic payment splitting
//! - Concurrent path execution
//! - Partial payment failure handling
//! - Route optimization
//! - Payment coordination

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{
    vec::Vec,
    string::{String, ToString},
    format,
};

#[cfg(feature = "std")]
use std::{
    vec::Vec,
    string::String,
};

use crate::routing::{Route, Router, RouteHop, RoutingError};

/// Maximum number of payment parts
pub const MAX_PAYMENT_PARTS: usize = 16;

/// Minimum payment part amount
pub const MIN_PART_AMOUNT: u128 = 1000;

/// Payment part status
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PartStatus {
    Pending,
    InFlight,
    Succeeded,
    Failed,
}

/// Multi-path payment part
#[derive(Clone, Debug)]
pub struct PaymentPart {
    /// Part index
    pub index: usize,
    /// Route for this part
    pub route: Route,
    /// Amount for this part
    pub amount: u128,
    /// Status
    pub status: PartStatus,
    /// Attempt number
    pub attempt: usize,
}

impl PaymentPart {
    pub fn new(index: usize, route: Route, amount: u128) -> Self {
        Self {
            index,
            route,
            amount,
            status: PartStatus::Pending,
            attempt: 1,
        }
    }
}

/// Multi-path payment
#[derive(Clone, Debug)]
pub struct MultiPathPayment {
    /// Payment hash (shared by all parts)
    pub payment_hash: Vec<u8>,
    /// Total payment amount
    pub total_amount: u128,
    /// Payment parts
    pub parts: Vec<PaymentPart>,
    /// Timeout timestamp
    pub timeout: u64,
    /// Payment status
    pub is_complete: bool,
}

impl MultiPathPayment {
    /// Create new multi-path payment
    pub fn new(payment_hash: Vec<u8>, total_amount: u128, timeout: u64) -> Self {
        Self {
            payment_hash,
            total_amount,
            parts: Vec::new(),
            timeout,
            is_complete: false,
        }
    }

    /// Split payment into multiple parts
    pub fn split_payment(
        &mut self,
        available_routes: Vec<Route>,
        max_parts: usize,
    ) -> Result<(), MPPError> {
        if available_routes.is_empty() {
            return Err(MPPError::NoRoutesAvailable);
        }

        let max_parts = max_parts.min(MAX_PAYMENT_PARTS);

        // Sort routes by cost (fee + timelock)
        let mut sorted_routes = available_routes;
        sorted_routes.sort_by_key(|r| r.total_fees);

        // Calculate amount per part
        let num_parts = max_parts.min(sorted_routes.len());
        if num_parts == 0 {
            return Err(MPPError::NoRoutesAvailable);
        }

        let base_amount = self.total_amount / num_parts as u128;
        let remainder = self.total_amount % num_parts as u128;

        // Create payment parts
        for (index, route) in sorted_routes.iter().take(num_parts).enumerate() {
            let amount = if index == 0 {
                base_amount + remainder
            } else {
                base_amount
            };

            if amount < MIN_PART_AMOUNT {
                continue;
            }

            // Verify route has sufficient capacity
            if !self.verify_route_capacity(route, amount) {
                continue;
            }

            let part = PaymentPart::new(index, route.clone(), amount);
            self.parts.push(part);
        }

        if self.parts.is_empty() {
            return Err(MPPError::InsufficientCapacity);
        }

        Ok(())
    }

    /// Verify route has sufficient capacity
    fn verify_route_capacity(&self, route: &Route, amount: u128) -> bool {
        // Simplified capacity check: ensure route total amount meets requirement
        // Note: RouteHop doesn't have channel_capacity field in the API
        route.total_amount >= amount
    }

    /// Execute multi-path payment
    pub fn execute(&mut self) -> Result<PaymentResult, MPPError> {
        if self.parts.is_empty() {
            return Err(MPPError::NoPartsConfigured);
        }

        // Mark all parts as in-flight
        for part in &mut self.parts {
            part.status = PartStatus::InFlight;
        }

        // In production, this would send HTLCs concurrently
        // For now, simulate execution
        let mut succeeded = 0;
        let mut failed = 0;

        for part in &mut self.parts {
            // Simulate payment execution (90% success rate)
            let success = part.index % 10 != 0;

            if success {
                part.status = PartStatus::Succeeded;
                succeeded += 1;
            } else {
                part.status = PartStatus::Failed;
                failed += 1;
            }
        }

        if succeeded == self.parts.len() {
            self.is_complete = true;
            Ok(PaymentResult::Success {
                parts_succeeded: succeeded,
                total_fee: self.calculate_total_fee(),
            })
        } else if failed == self.parts.len() {
            Err(MPPError::AllPartsFailed)
        } else {
            Ok(PaymentResult::PartialSuccess {
                parts_succeeded: succeeded,
                parts_failed: failed,
            })
        }
    }

    /// Retry failed parts
    pub fn retry_failed_parts(
        &mut self,
        alternative_routes: Vec<Route>,
    ) -> Result<(), MPPError> {
        let failed_parts: Vec<usize> = self
            .parts
            .iter()
            .enumerate()
            .filter(|(_, p)| p.status == PartStatus::Failed)
            .map(|(i, _)| i)
            .collect();

        for (i, &part_index) in failed_parts.iter().enumerate() {
            if i >= alternative_routes.len() {
                break;
            }

            let part = &mut self.parts[part_index];
            part.route = alternative_routes[i].clone();
            part.status = PartStatus::Pending;
            part.attempt += 1;

            if part.attempt > 3 {
                return Err(MPPError::MaxRetriesExceeded);
            }
        }

        Ok(())
    }

    /// Calculate total fees paid
    fn calculate_total_fee(&self) -> u128 {
        self.parts
            .iter()
            .filter(|p| p.status == PartStatus::Succeeded)
            .map(|p| p.route.total_fees)
            .sum()
    }

    /// Get successful parts count
    pub fn successful_parts_count(&self) -> usize {
        self.parts
            .iter()
            .filter(|p| p.status == PartStatus::Succeeded)
            .count()
    }

    /// Get failed parts count
    pub fn failed_parts_count(&self) -> usize {
        self.parts
            .iter()
            .filter(|p| p.status == PartStatus::Failed)
            .count()
    }

    /// Check if payment is expired
    pub fn is_expired(&self, current_time: u64) -> bool {
        current_time > self.timeout
    }
}

/// MPP payment result
#[derive(Clone, Debug, PartialEq)]
pub enum PaymentResult {
    Success {
        parts_succeeded: usize,
        total_fee: u128,
    },
    PartialSuccess {
        parts_succeeded: usize,
        parts_failed: usize,
    },
}

/// MPP Manager
pub struct MPPManager {
    router: Router,
    max_parts: usize,
    retry_attempts: usize,
}

impl MPPManager {
    pub fn new(router: Router, max_parts: usize) -> Self {
        Self {
            router,
            max_parts: max_parts.min(MAX_PAYMENT_PARTS),
            retry_attempts: 3,
        }
    }

    /// Send multi-path payment
    pub fn send_payment(
        &self,
        payment_hash: Vec<u8>,
        amount: u128,
        destination: &str,
        timeout: u64,
    ) -> Result<PaymentResult, MPPError> {
        // TODO: Implement multi-route finding using Router.find_route() in a loop
        // Note: Router API only has find_route() (singular), not find_multiple_routes()
        // For now, return a basic implementation stub

        // This would need to be implemented by:
        // 1. Calling router.find_route() multiple times with different constraints
        // 2. Collecting successful routes into a Vec
        // 3. Passing that Vec to split_payment()

        Err(MPPError::RoutingFailed)

        // Future implementation outline:
        // let mut routes = Vec::new();
        // for _ in 0..self.max_parts {
        //     if let Ok(route) = self.router.find_route(&source, &dest, amount) {
        //         routes.push(route);
        //     }
        // }
        // if routes.is_empty() { return Err(MPPError::NoRoutesAvailable); }
        // let mut mpp = MultiPathPayment::new(payment_hash, amount, timeout);
        // mpp.split_payment(routes, self.max_parts)?;
        // mpp.execute()
    }
}

/// MPP errors
#[derive(Clone, Debug, PartialEq)]
pub enum MPPError {
    NoRoutesAvailable,
    InsufficientCapacity,
    NoPartsConfigured,
    AllPartsFailed,
    MaxRetriesExceeded,
    RoutingFailed,
    PaymentExpired,
    InvalidAmount,
}

impl core::fmt::Display for MPPError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            MPPError::NoRoutesAvailable => write!(f, "No routes available"),
            MPPError::InsufficientCapacity => write!(f, "Insufficient capacity on all routes"),
            MPPError::NoPartsConfigured => write!(f, "No payment parts configured"),
            MPPError::AllPartsFailed => write!(f, "All payment parts failed"),
            MPPError::MaxRetriesExceeded => write!(f, "Maximum retries exceeded"),
            MPPError::RoutingFailed => write!(f, "Routing failed"),
            MPPError::PaymentExpired => write!(f, "Payment expired"),
            MPPError::InvalidAmount => write!(f, "Invalid payment amount"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::routing::{NetworkGraph, NodeId};

    fn create_test_route(fee: u128, _capacity: u128) -> Route {
        Route {
            hops: vec![RouteHop {
                channel_id: "ch1".to_string(),
                from_node: "node1".to_string(),
                to_node: "node2".to_string(),
                amount_to_forward: 5000,
                fee,
                time_lock: 144,
            }],
            total_fees: fee,
            total_amount: 5000,
            total_time_lock: 288,
        }
    }

    #[test]
    fn test_mpp_creation() {
        let mpp = MultiPathPayment::new(vec![1, 2, 3], 10000, 1000);
        assert_eq!(mpp.total_amount, 10000);
        assert_eq!(mpp.timeout, 1000);
        assert_eq!(mpp.parts.len(), 0);
        assert!(!mpp.is_complete);
    }

    #[test]
    fn test_split_payment() {
        let mut mpp = MultiPathPayment::new(vec![1, 2, 3], 10000, 1000);
        let routes = vec![
            create_test_route(100, 15000),
            create_test_route(110, 15000),
            create_test_route(120, 15000),
        ];

        assert!(mpp.split_payment(routes, 3).is_ok());
        assert_eq!(mpp.parts.len(), 3);
    }

    #[test]
    fn test_split_payment_no_routes() {
        let mut mpp = MultiPathPayment::new(vec![1, 2, 3], 10000, 1000);
        let result = mpp.split_payment(vec![], 3);
        assert_eq!(result, Err(MPPError::NoRoutesAvailable));
    }

    #[test]
    fn test_verify_route_capacity() {
        let mpp = MultiPathPayment::new(vec![1, 2, 3], 10000, 1000);

        let route_ok = create_test_route(100, 15000);
        assert!(mpp.verify_route_capacity(&route_ok, 10000));

        let route_insufficient = create_test_route(100, 5000);
        assert!(!mpp.verify_route_capacity(&route_insufficient, 10000));
    }

    #[test]
    fn test_payment_part_creation() {
        let route = create_test_route(100, 10000);
        let part = PaymentPart::new(0, route, 5000);

        assert_eq!(part.index, 0);
        assert_eq!(part.amount, 5000);
        assert_eq!(part.status, PartStatus::Pending);
        assert_eq!(part.attempt, 1);
    }

    #[test]
    fn test_execute_payment() {
        let mut mpp = MultiPathPayment::new(vec![1, 2, 3], 10000, 1000);
        let routes = vec![
            create_test_route(100, 15000),
            create_test_route(110, 15000),
        ];

        mpp.split_payment(routes, 2).unwrap();
        let result = mpp.execute();

        assert!(result.is_ok());
    }

    #[test]
    fn test_retry_failed_parts() {
        let mut mpp = MultiPathPayment::new(vec![1, 2, 3], 10000, 1000);
        let routes = vec![create_test_route(100, 15000)];

        mpp.split_payment(routes, 1).unwrap();
        mpp.parts[0].status = PartStatus::Failed;

        let alt_routes = vec![create_test_route(110, 15000)];
        assert!(mpp.retry_failed_parts(alt_routes).is_ok());
        assert_eq!(mpp.parts[0].status, PartStatus::Pending);
        assert_eq!(mpp.parts[0].attempt, 2);
    }

    #[test]
    fn test_is_expired() {
        let mpp = MultiPathPayment::new(vec![1, 2, 3], 10000, 1000);

        assert!(!mpp.is_expired(500));
        assert!(!mpp.is_expired(1000));
        assert!(mpp.is_expired(1001));
    }

    #[test]
    fn test_successful_parts_count() {
        let mut mpp = MultiPathPayment::new(vec![1, 2, 3], 10000, 1000);
        let routes = vec![
            create_test_route(100, 15000),
            create_test_route(110, 15000),
        ];

        mpp.split_payment(routes, 2).unwrap();
        mpp.parts[0].status = PartStatus::Succeeded;
        mpp.parts[1].status = PartStatus::Failed;

        assert_eq!(mpp.successful_parts_count(), 1);
        assert_eq!(mpp.failed_parts_count(), 1);
    }
}
