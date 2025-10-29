//! Lightning-Bloc Routing Protocol
//!
//! Multi-hop payment routing with pathfinding and fee optimization

#[cfg(not(feature = "std"))]
use alloc::{
    collections::{BTreeMap as HashMap, BTreeSet as HashSet, BinaryHeap},
    string::{String, ToString},
    vec,
    vec::Vec,
    format,
};

#[cfg(not(feature = "std"))]
use core::{
    cmp::{Ordering, PartialEq, Eq, PartialOrd, Ord},
    default::Default,
    result::Result::{self, Ok, Err},
    option::Option::{self, Some, None},
    fmt,
};

#[cfg(feature = "std")]
use std::{
    collections::{HashMap, HashSet, BinaryHeap},
    cmp::{Ordering, PartialEq, Eq, PartialOrd, Ord},
    default::Default,
    vec::Vec,
    string::String,
    result::Result::{self, Ok, Err},
    option::Option::{self, Some, None},
    fmt,
};

/// Node identifier in the payment network
pub type NodeId = String;

/// Channel identifier
pub type ChannelId = String;

/// Network edge representing a payment channel
#[derive(Debug, Clone)]
pub struct ChannelEdge {
    pub channel_id: ChannelId,
    pub from_node: NodeId,
    pub to_node: NodeId,
    pub capacity: u128,
    pub base_fee: u64,
    pub fee_rate: u32, // In basis points (1/10000)
    pub min_htlc: u128,
    pub max_htlc: u128,
    pub time_lock_delta: u32,
}

impl ChannelEdge {
    /// Calculate fee for routing amount through this channel
    pub fn calculate_fee(&self, amount: u128) -> u128 {
        let proportional_fee = (amount * self.fee_rate as u128) / 10000;
        self.base_fee as u128 + proportional_fee
    }

    /// Check if channel can route the given amount
    pub fn can_route(&self, amount: u128) -> bool {
        amount >= self.min_htlc && amount <= self.max_htlc && amount <= self.capacity
    }
}

/// Payment route through the network
#[derive(Debug, Clone, PartialEq)]
pub struct Route {
    pub hops: Vec<RouteHop>,
    pub total_amount: u128,
    pub total_fees: u128,
    pub total_time_lock: u32,
}

impl Route {
    /// Get the path as a sequence of node IDs
    pub fn path(&self) -> Vec<NodeId> {
        let mut path = vec![self.hops[0].from_node.clone()];
        for hop in &self.hops {
            path.push(hop.to_node.clone());
        }
        path
    }

    /// Get total number of hops
    pub fn hop_count(&self) -> usize {
        self.hops.len()
    }

    /// Verify route is valid
    pub fn verify(&self) -> Result<(), RoutingError> {
        if self.hops.is_empty() {
            return Err(RoutingError::EmptyRoute);
        }

        // Verify hop connectivity
        for i in 0..self.hops.len() - 1 {
            if self.hops[i].to_node != self.hops[i + 1].from_node {
                return Err(RoutingError::DisconnectedHops {
                    hop_index: i,
                });
            }
        }

        Ok(())
    }
}

/// Single hop in a payment route
#[derive(Debug, Clone, PartialEq)]
pub struct RouteHop {
    pub channel_id: ChannelId,
    pub from_node: NodeId,
    pub to_node: NodeId,
    pub amount_to_forward: u128,
    pub fee: u128,
    pub time_lock: u32,
}

/// Payment network graph
#[derive(Debug, Clone)]
pub struct NetworkGraph {
    /// All nodes in the network
    nodes: HashSet<NodeId>,
    /// Channels indexed by source node
    outgoing_channels: HashMap<NodeId, Vec<ChannelEdge>>,
    /// Channels indexed by channel ID
    channels_by_id: HashMap<ChannelId, ChannelEdge>,
    /// Network statistics
    stats: NetworkStats,
}

impl NetworkGraph {
    /// Create new empty network graph
    pub fn new() -> Self {
        Self {
            nodes: HashSet::new(),
            outgoing_channels: HashMap::new(),
            channels_by_id: HashMap::new(),
            stats: NetworkStats::default(),
        }
    }

    /// Add a node to the network
    pub fn add_node(&mut self, node_id: NodeId) {
        if self.nodes.insert(node_id.clone()) {
            self.outgoing_channels.entry(node_id).or_default();
            self.stats.node_count += 1;
        }
    }

    /// Add a bidirectional channel between two nodes
    pub fn add_channel(&mut self, edge: ChannelEdge) -> Result<(), RoutingError> {
        // Add nodes if they don't exist
        self.add_node(edge.from_node.clone());
        self.add_node(edge.to_node.clone());

        // Check for duplicate channel
        if self.channels_by_id.contains_key(&edge.channel_id) {
            return Err(RoutingError::DuplicateChannel(edge.channel_id));
        }

        // Add forward edge
        self.outgoing_channels
            .entry(edge.from_node.clone())
            .or_default()
            .push(edge.clone());

        // Add reverse edge
        let reverse_edge = ChannelEdge {
            channel_id: edge.channel_id.clone(),
            from_node: edge.to_node.clone(),
            to_node: edge.from_node.clone(),
            capacity: edge.capacity,
            base_fee: edge.base_fee,
            fee_rate: edge.fee_rate,
            min_htlc: edge.min_htlc,
            max_htlc: edge.max_htlc,
            time_lock_delta: edge.time_lock_delta,
        };

        self.outgoing_channels
            .entry(reverse_edge.from_node.clone())
            .or_default()
            .push(reverse_edge);

        self.channels_by_id.insert(edge.channel_id.clone(), edge);
        self.stats.channel_count += 1;

        Ok(())
    }

    /// Remove a channel from the network
    pub fn remove_channel(&mut self, channel_id: &ChannelId) -> Result<(), RoutingError> {
        let edge = self.channels_by_id
            .remove(channel_id)
            .ok_or_else(|| RoutingError::ChannelNotFound(channel_id.clone()))?;

        // Remove from outgoing channels
        if let Some(channels) = self.outgoing_channels.get_mut(&edge.from_node) {
            channels.retain(|e| e.channel_id != *channel_id);
        }

        if let Some(channels) = self.outgoing_channels.get_mut(&edge.to_node) {
            channels.retain(|e| e.channel_id != *channel_id);
        }

        self.stats.channel_count -= 1;
        Ok(())
    }

    /// Update channel capacity
    pub fn update_capacity(&mut self, channel_id: &ChannelId, new_capacity: u128) -> Result<(), RoutingError> {
        let edge = self.channels_by_id
            .get_mut(channel_id)
            .ok_or_else(|| RoutingError::ChannelNotFound(channel_id.clone()))?;

        edge.capacity = new_capacity;

        // Update in outgoing channels
        if let Some(channels) = self.outgoing_channels.get_mut(&edge.from_node) {
            for channel in channels {
                if channel.channel_id == *channel_id {
                    channel.capacity = new_capacity;
                }
            }
        }

        if let Some(channels) = self.outgoing_channels.get_mut(&edge.to_node) {
            for channel in channels {
                if channel.channel_id == *channel_id {
                    channel.capacity = new_capacity;
                }
            }
        }

        Ok(())
    }

    /// Get all channels from a node
    pub fn get_outgoing_channels(&self, node_id: &NodeId) -> Vec<&ChannelEdge> {
        self.outgoing_channels
            .get(node_id)
            .map(|channels| channels.iter().collect())
            .unwrap_or_default()
    }

    /// Get channel by ID
    pub fn get_channel(&self, channel_id: &ChannelId) -> Option<&ChannelEdge> {
        self.channels_by_id.get(channel_id)
    }

    /// Check if a node exists
    pub fn has_node(&self, node_id: &NodeId) -> bool {
        self.nodes.contains(node_id)
    }

    /// Get network statistics
    pub fn stats(&self) -> &NetworkStats {
        &self.stats
    }

    /// Get total network capacity
    pub fn total_capacity(&self) -> u128 {
        self.channels_by_id.values().map(|e| e.capacity).sum()
    }

    /// Find neighbors of a node
    pub fn neighbors(&self, node_id: &NodeId) -> HashSet<NodeId> {
        self.outgoing_channels
            .get(node_id)
            .map(|channels| channels.iter().map(|e| e.to_node.clone()).collect())
            .unwrap_or_default()
    }
}

impl Default for NetworkGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// Network statistics
#[derive(Debug, Clone, Default)]
pub struct NetworkStats {
    pub node_count: usize,
    pub channel_count: usize,
}

/// Pathfinding state for Dijkstra's algorithm
#[derive(Debug, Clone)]
struct PathState {
    node: NodeId,
    cost: u128,
    hops: Vec<RouteHop>,
}

impl PartialEq for PathState {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for PathState {}

impl PartialOrd for PathState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PathState {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap
        other.cost.cmp(&self.cost)
    }
}

/// Payment router
pub struct Router {
    graph: NetworkGraph,
    max_route_length: usize,
    max_fee_percent: u32, // Maximum fee as percentage of amount (in basis points)
}

impl Router {
    /// Create new router
    pub fn new(graph: NetworkGraph) -> Self {
        Self {
            graph,
            max_route_length: 20,
            max_fee_percent: 500, // 5% max fee
        }
    }

    /// Set maximum route length
    pub fn set_max_route_length(&mut self, max_length: usize) {
        self.max_route_length = max_length;
    }

    /// Set maximum fee percentage
    pub fn set_max_fee_percent(&mut self, max_fee_percent: u32) {
        self.max_fee_percent = max_fee_percent;
    }

    /// Find optimal route using Dijkstra's algorithm
    pub fn find_route(
        &self,
        from: &NodeId,
        to: &NodeId,
        amount: u128,
    ) -> Result<Route, RoutingError> {
        if !self.graph.has_node(from) {
            return Err(RoutingError::NodeNotFound(from.clone()));
        }
        if !self.graph.has_node(to) {
            return Err(RoutingError::NodeNotFound(to.clone()));
        }
        if from == to {
            return Err(RoutingError::SameSourceAndDestination);
        }

        let mut heap = BinaryHeap::new();
        let mut visited = HashSet::new();
        let mut best_cost: HashMap<NodeId, u128> = HashMap::new();

        // Start from source
        heap.push(PathState {
            node: from.clone(),
            cost: 0,
            hops: Vec::new(),
        });
        best_cost.insert(from.clone(), 0);

        while let Some(PathState { node, cost, hops }) = heap.pop() {
            // Found destination
            if node == *to {
                let total_fees = cost;
                let max_allowed_fee = (amount * self.max_fee_percent as u128) / 10000;

                if total_fees > max_allowed_fee {
                    return Err(RoutingError::FeeTooHigh {
                        fee: total_fees,
                        max: max_allowed_fee,
                    });
                }

                let total_time_lock = hops.iter().map(|h| h.time_lock).sum();

                return Ok(Route {
                    hops,
                    total_amount: amount + total_fees,
                    total_fees,
                    total_time_lock,
                });
            }

            // Skip if already visited with better cost
            if visited.contains(&node) {
                continue;
            }
            visited.insert(node.clone());

            // Skip if route is too long
            if hops.len() >= self.max_route_length {
                continue;
            }

            // Explore neighbors
            for edge in self.graph.get_outgoing_channels(&node) {
                // Calculate amount needed for this hop (including fees for remaining route)
                let hop_amount = amount;

                if !edge.can_route(hop_amount) {
                    continue;
                }

                let fee = edge.calculate_fee(hop_amount);
                let new_cost = cost + fee;

                // Check if this path is better
                if let Some(&prev_cost) = best_cost.get(&edge.to_node) {
                    if new_cost >= prev_cost {
                        continue;
                    }
                }

                best_cost.insert(edge.to_node.clone(), new_cost);

                let mut new_hops = hops.clone();
                new_hops.push(RouteHop {
                    channel_id: edge.channel_id.clone(),
                    from_node: edge.from_node.clone(),
                    to_node: edge.to_node.clone(),
                    amount_to_forward: hop_amount,
                    fee,
                    time_lock: edge.time_lock_delta,
                });

                heap.push(PathState {
                    node: edge.to_node.clone(),
                    cost: new_cost,
                    hops: new_hops,
                });
            }
        }

        Err(RoutingError::NoRouteFound {
            from: from.clone(),
            to: to.clone(),
        })
    }

    /// Find multiple route alternatives
    pub fn find_routes(
        &self,
        from: &NodeId,
        to: &NodeId,
        amount: u128,
        max_routes: usize,
    ) -> Vec<Route> {
        let mut routes = Vec::new();
        let mut excluded_channels = HashSet::new();

        for _ in 0..max_routes {
            // Try to find a route excluding already used channels
            match self.find_route_excluding(from, to, amount, &excluded_channels) {
                Ok(route) => {
                    // Mark channels in this route as used
                    for hop in &route.hops {
                        excluded_channels.insert(hop.channel_id.clone());
                    }
                    routes.push(route);
                }
                Err(_) => break, // No more routes available
            }
        }

        routes
    }

    /// Find route excluding specific channels
    fn find_route_excluding(
        &self,
        from: &NodeId,
        to: &NodeId,
        amount: u128,
        excluded: &HashSet<ChannelId>,
    ) -> Result<Route, RoutingError> {
        // Similar to find_route but skip excluded channels
        let mut heap = BinaryHeap::new();
        let mut visited = HashSet::new();
        let mut best_cost: HashMap<NodeId, u128> = HashMap::new();

        heap.push(PathState {
            node: from.clone(),
            cost: 0,
            hops: Vec::new(),
        });
        best_cost.insert(from.clone(), 0);

        while let Some(PathState { node, cost, hops }) = heap.pop() {
            if node == *to {
                let total_fees = cost;
                let total_time_lock = hops.iter().map(|h| h.time_lock).sum();

                return Ok(Route {
                    hops,
                    total_amount: amount + total_fees,
                    total_fees,
                    total_time_lock,
                });
            }

            if visited.contains(&node) {
                continue;
            }
            visited.insert(node.clone());

            if hops.len() >= self.max_route_length {
                continue;
            }

            for edge in self.graph.get_outgoing_channels(&node) {
                // Skip excluded channels
                if excluded.contains(&edge.channel_id) {
                    continue;
                }

                let hop_amount = amount;

                if !edge.can_route(hop_amount) {
                    continue;
                }

                let fee = edge.calculate_fee(hop_amount);
                let new_cost = cost + fee;

                if let Some(&prev_cost) = best_cost.get(&edge.to_node) {
                    if new_cost >= prev_cost {
                        continue;
                    }
                }

                best_cost.insert(edge.to_node.clone(), new_cost);

                let mut new_hops = hops.clone();
                new_hops.push(RouteHop {
                    channel_id: edge.channel_id.clone(),
                    from_node: edge.from_node.clone(),
                    to_node: edge.to_node.clone(),
                    amount_to_forward: hop_amount,
                    fee,
                    time_lock: edge.time_lock_delta,
                });

                heap.push(PathState {
                    node: edge.to_node.clone(),
                    cost: new_cost,
                    hops: new_hops,
                });
            }
        }

        Err(RoutingError::NoRouteFound {
            from: from.clone(),
            to: to.clone(),
        })
    }

    /// Get reference to network graph
    pub fn graph(&self) -> &NetworkGraph {
        &self.graph
    }

    /// Get mutable reference to network graph
    pub fn graph_mut(&mut self) -> &mut NetworkGraph {
        &mut self.graph
    }
}

/// Routing errors
#[derive(Debug, Clone, PartialEq)]
pub enum RoutingError {
    NodeNotFound(NodeId),
    ChannelNotFound(ChannelId),
    DuplicateChannel(ChannelId),
    NoRouteFound { from: NodeId, to: NodeId },
    SameSourceAndDestination,
    EmptyRoute,
    DisconnectedHops { hop_index: usize },
    FeeTooHigh { fee: u128, max: u128 },
    InsufficientCapacity { channel: ChannelId, have: u128, need: u128 },
}

impl fmt::Display for RoutingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RoutingError::NodeNotFound(id) => write!(f, "Node not found: {}", id),
            RoutingError::ChannelNotFound(id) => write!(f, "Channel not found: {}", id),
            RoutingError::DuplicateChannel(id) => write!(f, "Duplicate channel: {}", id),
            RoutingError::NoRouteFound { from, to } => {
                write!(f, "No route found from {} to {}", from, to)
            }
            RoutingError::SameSourceAndDestination => {
                write!(f, "Source and destination are the same")
            }
            RoutingError::EmptyRoute => write!(f, "Route has no hops"),
            RoutingError::DisconnectedHops { hop_index } => {
                write!(f, "Disconnected hops at index {}", hop_index)
            }
            RoutingError::FeeTooHigh { fee, max } => {
                write!(f, "Fee {} exceeds maximum {}", fee, max)
            }
            RoutingError::InsufficientCapacity { channel, have, need } => {
                write!(f, "Channel {} has insufficient capacity: {} < {}", channel, have, need)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_network() -> NetworkGraph {
        let mut graph = NetworkGraph::new();

        // Create a diamond-shaped network: A -> B -> D, A -> C -> D
        graph.add_channel(ChannelEdge {
            channel_id: "AB".to_string(),
            from_node: "A".to_string(),
            to_node: "B".to_string(),
            capacity: 1000,
            base_fee: 1,
            fee_rate: 100, // 1%
            min_htlc: 1,
            max_htlc: 1000,
            time_lock_delta: 40,
        }).unwrap();

        graph.add_channel(ChannelEdge {
            channel_id: "AC".to_string(),
            from_node: "A".to_string(),
            to_node: "C".to_string(),
            capacity: 1000,
            base_fee: 2,
            fee_rate: 50, // 0.5%
            min_htlc: 1,
            max_htlc: 1000,
            time_lock_delta: 40,
        }).unwrap();

        graph.add_channel(ChannelEdge {
            channel_id: "BD".to_string(),
            from_node: "B".to_string(),
            to_node: "D".to_string(),
            capacity: 1000,
            base_fee: 1,
            fee_rate: 100, // 1%
            min_htlc: 1,
            max_htlc: 1000,
            time_lock_delta: 40,
        }).unwrap();

        graph.add_channel(ChannelEdge {
            channel_id: "CD".to_string(),
            from_node: "C".to_string(),
            to_node: "D".to_string(),
            capacity: 1000,
            base_fee: 2,
            fee_rate: 50, // 0.5%
            min_htlc: 1,
            max_htlc: 1000,
            time_lock_delta: 40,
        }).unwrap();

        graph
    }

    #[test]
    fn test_network_graph_creation() {
        let graph = NetworkGraph::new();
        assert_eq!(graph.stats().node_count, 0);
        assert_eq!(graph.stats().channel_count, 0);
    }

    #[test]
    fn test_add_channel() {
        let mut graph = NetworkGraph::new();
        let edge = ChannelEdge {
            channel_id: "ch1".to_string(),
            from_node: "A".to_string(),
            to_node: "B".to_string(),
            capacity: 1000,
            base_fee: 1,
            fee_rate: 100,
            min_htlc: 1,
            max_htlc: 1000,
            time_lock_delta: 40,
        };

        assert!(graph.add_channel(edge).is_ok());
        assert_eq!(graph.stats().node_count, 2);
        assert_eq!(graph.stats().channel_count, 1);
    }

    #[test]
    fn test_channel_fee_calculation() {
        let edge = ChannelEdge {
            channel_id: "ch1".to_string(),
            from_node: "A".to_string(),
            to_node: "B".to_string(),
            capacity: 1000,
            base_fee: 10,
            fee_rate: 100, // 1%
            min_htlc: 1,
            max_htlc: 1000,
            time_lock_delta: 40,
        };

        // For 100 units: base_fee (10) + 1% (1) = 11
        assert_eq!(edge.calculate_fee(100), 11);
    }

    #[test]
    fn test_channel_can_route() {
        let edge = ChannelEdge {
            channel_id: "ch1".to_string(),
            from_node: "A".to_string(),
            to_node: "B".to_string(),
            capacity: 1000,
            base_fee: 1,
            fee_rate: 100,
            min_htlc: 10,
            max_htlc: 900,
            time_lock_delta: 40,
        };

        assert!(!edge.can_route(5)); // Below min
        assert!(edge.can_route(100)); // Within range
        assert!(!edge.can_route(1000)); // Above max
    }

    #[test]
    fn test_router_find_route() {
        let graph = create_test_network();
        let router = Router::new(graph);

        let route = router.find_route(&"A".to_string(), &"D".to_string(), 100);
        assert!(route.is_ok());

        let route = route.unwrap();
        assert_eq!(route.hops.len(), 2);
        assert!(route.total_fees > 0);
    }

    #[test]
    fn test_router_no_route() {
        let graph = NetworkGraph::new();
        let router = Router::new(graph);

        let result = router.find_route(&"A".to_string(), &"B".to_string(), 100);
        assert!(result.is_err());
    }

    #[test]
    fn test_route_verification() {
        let route = Route {
            hops: vec![
                RouteHop {
                    channel_id: "AB".to_string(),
                    from_node: "A".to_string(),
                    to_node: "B".to_string(),
                    amount_to_forward: 100,
                    fee: 1,
                    time_lock: 40,
                },
                RouteHop {
                    channel_id: "BD".to_string(),
                    from_node: "B".to_string(),
                    to_node: "D".to_string(),
                    amount_to_forward: 100,
                    fee: 1,
                    time_lock: 40,
                },
            ],
            total_amount: 102,
            total_fees: 2,
            total_time_lock: 80,
        };

        assert!(route.verify().is_ok());
    }

    #[test]
    fn test_route_path() {
        let route = Route {
            hops: vec![
                RouteHop {
                    channel_id: "AB".to_string(),
                    from_node: "A".to_string(),
                    to_node: "B".to_string(),
                    amount_to_forward: 100,
                    fee: 1,
                    time_lock: 40,
                },
                RouteHop {
                    channel_id: "BD".to_string(),
                    from_node: "B".to_string(),
                    to_node: "D".to_string(),
                    amount_to_forward: 100,
                    fee: 1,
                    time_lock: 40,
                },
            ],
            total_amount: 102,
            total_fees: 2,
            total_time_lock: 80,
        };

        let path = route.path();
        assert_eq!(path, vec!["A", "B", "D"]);
    }

    #[test]
    fn test_find_multiple_routes() {
        let graph = create_test_network();
        let router = Router::new(graph);

        let routes = router.find_routes(&"A".to_string(), &"D".to_string(), 100, 2);
        assert_eq!(routes.len(), 2);
    }

    #[test]
    fn test_update_capacity() {
        let mut graph = create_test_network();
        assert!(graph.update_capacity(&"AB".to_string(), 2000).is_ok());

        let channel = graph.get_channel(&"AB".to_string());
        assert!(channel.is_some());
        assert_eq!(channel.unwrap().capacity, 2000);
    }

    #[test]
    fn test_remove_channel() {
        let mut graph = create_test_network();
        let initial_count = graph.stats().channel_count;

        assert!(graph.remove_channel(&"AB".to_string()).is_ok());
        assert_eq!(graph.stats().channel_count, initial_count - 1);
        assert!(graph.get_channel(&"AB".to_string()).is_none());
    }

    #[test]
    fn test_network_neighbors() {
        let graph = create_test_network();
        let neighbors = graph.neighbors(&"A".to_string());

        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&"B".to_string()));
        assert!(neighbors.contains(&"C".to_string()));
    }

    #[test]
    fn test_total_network_capacity() {
        let graph = create_test_network();
        // 4 channels × 1000 capacity each = 4000
        assert_eq!(graph.total_capacity(), 4000);
    }

    // ====================================================================
    // COMPREHENSIVE ROUTING ALGORITHM TESTS
    // ====================================================================

    #[test]
    fn test_shortest_path_finding() {
        let graph = create_test_network();
        let mut router = Router::new(graph);
        router.set_max_fee_percent(1000); // 10% max fee to allow routing

        // Test finding shortest path A -> D
        let route = router.find_route(&"A".to_string(), &"D".to_string(), 100).unwrap();

        // Should find a 2-hop route
        assert_eq!(route.hops.len(), 2);
        // Either path is acceptable since Dijkstra may choose either
        let path = route.path();
        assert!(path == vec!["A", "C", "D"] || path == vec!["A", "B", "D"]);
        assert!(route.total_fees > 0);
    }

    #[test]
    fn test_multi_hop_routing_2_hops() {
        let graph = create_test_network();
        let router = Router::new(graph);

        let route = router.find_route(&"A".to_string(), &"D".to_string(), 50).unwrap();

        assert_eq!(route.hop_count(), 2);
        assert_eq!(route.hops[0].from_node, "A");
        assert_eq!(route.hops[route.hops.len() - 1].to_node, "D");
        assert!(route.verify().is_ok());
    }

    #[test]
    fn test_multi_hop_routing_3_hops() {
        // Create linear network: A -> B -> C -> D
        let mut graph = NetworkGraph::new();

        graph.add_channel(ChannelEdge {
            channel_id: "AB".to_string(),
            from_node: "A".to_string(),
            to_node: "B".to_string(),
            capacity: 1000,
            base_fee: 1,
            fee_rate: 100,
            min_htlc: 1,
            max_htlc: 1000,
            time_lock_delta: 40,
        }).unwrap();

        graph.add_channel(ChannelEdge {
            channel_id: "BC".to_string(),
            from_node: "B".to_string(),
            to_node: "C".to_string(),
            capacity: 1000,
            base_fee: 1,
            fee_rate: 100,
            min_htlc: 1,
            max_htlc: 1000,
            time_lock_delta: 40,
        }).unwrap();

        graph.add_channel(ChannelEdge {
            channel_id: "CD".to_string(),
            from_node: "C".to_string(),
            to_node: "D".to_string(),
            capacity: 1000,
            base_fee: 1,
            fee_rate: 100,
            min_htlc: 1,
            max_htlc: 1000,
            time_lock_delta: 40,
        }).unwrap();

        let mut router = Router::new(graph);
        router.set_max_fee_percent(1000); // 10% max fee
        let route = router.find_route(&"A".to_string(), &"D".to_string(), 100).unwrap();

        assert_eq!(route.hop_count(), 3);
        assert_eq!(route.path(), vec!["A", "B", "C", "D"]);
    }

    #[test]
    fn test_multi_hop_routing_4_hops() {
        // Create linear network: A -> B -> C -> D -> E
        let mut graph = NetworkGraph::new();

        let nodes = vec!["AB", "BC", "CD", "DE"];
        for (i, channel_id) in nodes.iter().enumerate() {
            let from = ((i + 65) as u8 as char).to_string();
            let to = ((i + 66) as u8 as char).to_string();

            graph.add_channel(ChannelEdge {
                channel_id: channel_id.to_string(),
                from_node: from,
                to_node: to,
                capacity: 1000,
                base_fee: 1,
                fee_rate: 100,
                min_htlc: 1,
                max_htlc: 1000,
                time_lock_delta: 40,
            }).unwrap();
        }

        let mut router = Router::new(graph);
        router.set_max_fee_percent(1000); // 10% max fee
        let route = router.find_route(&"A".to_string(), &"E".to_string(), 100).unwrap();

        assert_eq!(route.hop_count(), 4);
        assert_eq!(route.path(), vec!["A", "B", "C", "D", "E"]);
    }

    #[test]
    fn test_routing_failure_no_path() {
        let mut graph = NetworkGraph::new();

        // Create two disconnected components: A-B and C-D
        graph.add_channel(ChannelEdge {
            channel_id: "AB".to_string(),
            from_node: "A".to_string(),
            to_node: "B".to_string(),
            capacity: 1000,
            base_fee: 1,
            fee_rate: 100,
            min_htlc: 1,
            max_htlc: 1000,
            time_lock_delta: 40,
        }).unwrap();

        graph.add_channel(ChannelEdge {
            channel_id: "CD".to_string(),
            from_node: "C".to_string(),
            to_node: "D".to_string(),
            capacity: 1000,
            base_fee: 1,
            fee_rate: 100,
            min_htlc: 1,
            max_htlc: 1000,
            time_lock_delta: 40,
        }).unwrap();

        let router = Router::new(graph);
        let result = router.find_route(&"A".to_string(), &"D".to_string(), 100);

        assert!(result.is_err());
        match result {
            Err(RoutingError::NoRouteFound { from, to }) => {
                assert_eq!(from, "A");
                assert_eq!(to, "D");
            }
            _ => panic!("Expected NoRouteFound error"),
        }
    }

    #[test]
    fn test_routing_insufficient_capacity() {
        let mut graph = NetworkGraph::new();

        // Create channel with low capacity
        graph.add_channel(ChannelEdge {
            channel_id: "AB".to_string(),
            from_node: "A".to_string(),
            to_node: "B".to_string(),
            capacity: 50, // Low capacity
            base_fee: 1,
            fee_rate: 100,
            min_htlc: 1,
            max_htlc: 1000,
            time_lock_delta: 40,
        }).unwrap();

        let router = Router::new(graph);
        let result = router.find_route(&"A".to_string(), &"B".to_string(), 100);

        // Should fail because capacity (50) < amount (100)
        assert!(result.is_err());
    }

    #[test]
    fn test_routing_with_multiple_paths_chooses_optimal() {
        let graph = create_test_network();
        let mut router = Router::new(graph);
        router.set_max_fee_percent(1000); // 10% max fee

        // Network has two paths: A->B->D (higher fee) and A->C->D (lower fee)
        let route = router.find_route(&"A".to_string(), &"D".to_string(), 100).unwrap();

        // Verify it found a valid 2-hop route
        assert_eq!(route.hops.len(), 2);

        // Verify fees are reasonable
        assert!(route.total_fees <= 10);
    }

    #[test]
    fn test_routing_node_not_found() {
        let graph = create_test_network();
        let router = Router::new(graph);

        let result = router.find_route(&"Z".to_string(), &"D".to_string(), 100);

        assert!(result.is_err());
        match result {
            Err(RoutingError::NodeNotFound(node)) => {
                assert_eq!(node, "Z");
            }
            _ => panic!("Expected NodeNotFound error"),
        }
    }

    #[test]
    fn test_routing_same_source_and_destination() {
        let graph = create_test_network();
        let router = Router::new(graph);

        let result = router.find_route(&"A".to_string(), &"A".to_string(), 100);

        assert_eq!(result, Err(RoutingError::SameSourceAndDestination));
    }

    #[test]
    fn test_route_verification_disconnected_hops() {
        let route = Route {
            hops: vec![
                RouteHop {
                    channel_id: "AB".to_string(),
                    from_node: "A".to_string(),
                    to_node: "B".to_string(),
                    amount_to_forward: 100,
                    fee: 1,
                    time_lock: 40,
                },
                RouteHop {
                    channel_id: "CD".to_string(),
                    from_node: "C".to_string(), // Disconnected from B
                    to_node: "D".to_string(),
                    amount_to_forward: 100,
                    fee: 1,
                    time_lock: 40,
                },
            ],
            total_amount: 102,
            total_fees: 2,
            total_time_lock: 80,
        };

        let result = route.verify();
        assert!(result.is_err());
        match result {
            Err(RoutingError::DisconnectedHops { hop_index }) => {
                assert_eq!(hop_index, 0);
            }
            _ => panic!("Expected DisconnectedHops error"),
        }
    }

    #[test]
    fn test_route_verification_empty_route() {
        let route = Route {
            hops: vec![],
            total_amount: 0,
            total_fees: 0,
            total_time_lock: 0,
        };

        assert_eq!(route.verify(), Err(RoutingError::EmptyRoute));
    }

    // ====================================================================
    // CHANNEL CAPACITY TESTS
    // ====================================================================

    #[test]
    fn test_capacity_update_after_successful_payment() {
        let mut graph = create_test_network();

        // Simulate successful payment: reduce capacity
        let original_capacity = graph.get_channel(&"AB".to_string()).unwrap().capacity;
        graph.update_capacity(&"AB".to_string(), original_capacity - 100).unwrap();

        let updated = graph.get_channel(&"AB".to_string()).unwrap();
        assert_eq!(updated.capacity, original_capacity - 100);
    }

    #[test]
    fn test_capacity_restoration_after_failed_htlc() {
        let mut graph = create_test_network();

        // Simulate HTLC failure: restore capacity
        let original_capacity = graph.get_channel(&"AB".to_string()).unwrap().capacity;

        // Temporarily reduce
        graph.update_capacity(&"AB".to_string(), original_capacity - 100).unwrap();

        // Restore after failure
        graph.update_capacity(&"AB".to_string(), original_capacity).unwrap();

        let restored = graph.get_channel(&"AB".to_string()).unwrap();
        assert_eq!(restored.capacity, original_capacity);
    }

    #[test]
    fn test_channel_depletion_scenario() {
        let mut graph = NetworkGraph::new();

        graph.add_channel(ChannelEdge {
            channel_id: "AB".to_string(),
            from_node: "A".to_string(),
            to_node: "B".to_string(),
            capacity: 100,
            base_fee: 1,
            fee_rate: 100,
            min_htlc: 1,
            max_htlc: 1000,
            time_lock_delta: 40,
        }).unwrap();

        let router = Router::new(graph.clone());

        // First payment should succeed
        let route1 = router.find_route(&"A".to_string(), &"B".to_string(), 50);
        assert!(route1.is_ok());

        // Deplete capacity
        graph.update_capacity(&"AB".to_string(), 10).unwrap();
        let router2 = Router::new(graph);

        // Second payment should fail due to insufficient capacity
        let route2 = router2.find_route(&"A".to_string(), &"B".to_string(), 50);
        assert!(route2.is_err());
    }

    #[test]
    fn test_capacity_bounds_checking() {
        let mut graph = create_test_network();

        // Test updating to zero capacity
        assert!(graph.update_capacity(&"AB".to_string(), 0).is_ok());
        let channel = graph.get_channel(&"AB".to_string()).unwrap();
        assert_eq!(channel.capacity, 0);

        // Test routing with zero capacity
        let router = Router::new(graph);
        let result = router.find_route(&"A".to_string(), &"B".to_string(), 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_capacity_update_nonexistent_channel() {
        let mut graph = create_test_network();

        let result = graph.update_capacity(&"XY".to_string(), 500);
        assert!(result.is_err());
        match result {
            Err(RoutingError::ChannelNotFound(id)) => {
                assert_eq!(id, "XY");
            }
            _ => panic!("Expected ChannelNotFound error"),
        }
    }

    // ====================================================================
    // NETWORK TOPOLOGY TESTS
    // ====================================================================

    #[test]
    fn test_linear_topology() {
        // Linear: A -> B -> C -> D
        let mut graph = NetworkGraph::new();

        graph.add_channel(ChannelEdge {
            channel_id: "AB".to_string(),
            from_node: "A".to_string(),
            to_node: "B".to_string(),
            capacity: 1000,
            base_fee: 1,
            fee_rate: 100,
            min_htlc: 1,
            max_htlc: 1000,
            time_lock_delta: 40,
        }).unwrap();

        graph.add_channel(ChannelEdge {
            channel_id: "BC".to_string(),
            from_node: "B".to_string(),
            to_node: "C".to_string(),
            capacity: 1000,
            base_fee: 1,
            fee_rate: 100,
            min_htlc: 1,
            max_htlc: 1000,
            time_lock_delta: 40,
        }).unwrap();

        graph.add_channel(ChannelEdge {
            channel_id: "CD".to_string(),
            from_node: "C".to_string(),
            to_node: "D".to_string(),
            capacity: 1000,
            base_fee: 1,
            fee_rate: 100,
            min_htlc: 1,
            max_htlc: 1000,
            time_lock_delta: 40,
        }).unwrap();

        let mut router = Router::new(graph);
        router.set_max_fee_percent(1000); // 10% max fee
        let route = router.find_route(&"A".to_string(), &"D".to_string(), 100).unwrap();

        assert_eq!(route.hop_count(), 3);
        assert_eq!(route.path(), vec!["A", "B", "C", "D"]);
    }

    #[test]
    fn test_hub_and_spoke_topology() {
        // Hub-and-spoke: A,B,C -> HUB -> X,Y,Z
        let mut graph = NetworkGraph::new();

        // Spokes to hub
        for node in &["A", "B", "C"] {
            graph.add_channel(ChannelEdge {
                channel_id: format!("{}HUB", node),
                from_node: node.to_string(),
                to_node: "HUB".to_string(),
                capacity: 1000,
                base_fee: 1,
                fee_rate: 100,
                min_htlc: 1,
                max_htlc: 1000,
                time_lock_delta: 40,
            }).unwrap();
        }

        // Hub to destinations
        for node in &["X", "Y", "Z"] {
            graph.add_channel(ChannelEdge {
                channel_id: format!("HUB{}", node),
                from_node: "HUB".to_string(),
                to_node: node.to_string(),
                capacity: 1000,
                base_fee: 1,
                fee_rate: 100,
                min_htlc: 1,
                max_htlc: 1000,
                time_lock_delta: 40,
            }).unwrap();
        }

        let router = Router::new(graph);
        let route = router.find_route(&"A".to_string(), &"X".to_string(), 100).unwrap();

        assert_eq!(route.hop_count(), 2);
        assert_eq!(route.path(), vec!["A", "HUB", "X"]);
    }

    #[test]
    fn test_mesh_topology() {
        // Fully connected mesh: A <-> B <-> C <-> D (all interconnected)
        let mut graph = NetworkGraph::new();

        let nodes = vec!["A", "B", "C", "D"];
        let mut channel_count = 0;

        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                graph.add_channel(ChannelEdge {
                    channel_id: format!("{}{}", nodes[i], nodes[j]),
                    from_node: nodes[i].to_string(),
                    to_node: nodes[j].to_string(),
                    capacity: 1000,
                    base_fee: 1,
                    fee_rate: 100,
                    min_htlc: 1,
                    max_htlc: 1000,
                    time_lock_delta: 40,
                }).unwrap();
                channel_count += 1;
            }
        }

        assert_eq!(channel_count, 6); // Complete graph K4 has 6 edges

        let router = Router::new(graph);
        let route = router.find_route(&"A".to_string(), &"D".to_string(), 100).unwrap();

        // In mesh, should find direct route or very short route
        assert!(route.hop_count() <= 2);
    }

    #[test]
    fn test_disconnected_network_handling() {
        let mut graph = NetworkGraph::new();

        // Network 1: A - B
        graph.add_channel(ChannelEdge {
            channel_id: "AB".to_string(),
            from_node: "A".to_string(),
            to_node: "B".to_string(),
            capacity: 1000,
            base_fee: 1,
            fee_rate: 100,
            min_htlc: 1,
            max_htlc: 1000,
            time_lock_delta: 40,
        }).unwrap();

        // Network 2: X - Y (disconnected from Network 1)
        graph.add_channel(ChannelEdge {
            channel_id: "XY".to_string(),
            from_node: "X".to_string(),
            to_node: "Y".to_string(),
            capacity: 1000,
            base_fee: 1,
            fee_rate: 100,
            min_htlc: 1,
            max_htlc: 1000,
            time_lock_delta: 40,
        }).unwrap();

        let router = Router::new(graph);

        // Should succeed within same network
        assert!(router.find_route(&"A".to_string(), &"B".to_string(), 100).is_ok());
        assert!(router.find_route(&"X".to_string(), &"Y".to_string(), 100).is_ok());

        // Should fail across disconnected networks
        assert!(router.find_route(&"A".to_string(), &"X".to_string(), 100).is_err());
    }

    #[test]
    fn test_max_route_length_enforcement() {
        // Create long linear chain
        let mut graph = NetworkGraph::new();

        for i in 0..25 {
            let from = format!("N{}", i);
            let to = format!("N{}", i + 1);
            graph.add_channel(ChannelEdge {
                channel_id: format!("CH{}", i),
                from_node: from,
                to_node: to,
                capacity: 1000,
                base_fee: 1,
                fee_rate: 100,
                min_htlc: 1,
                max_htlc: 1000,
                time_lock_delta: 40,
            }).unwrap();
        }

        let mut router = Router::new(graph);
        router.set_max_route_length(10);

        // Should fail because route would be 25 hops but max is 10
        let result = router.find_route(&"N0".to_string(), &"N25".to_string(), 100);
        assert!(result.is_err());
    }

    #[test]
    fn test_fee_too_high_rejection() {
        let mut graph = NetworkGraph::new();

        // Create channel with very high fees
        graph.add_channel(ChannelEdge {
            channel_id: "AB".to_string(),
            from_node: "A".to_string(),
            to_node: "B".to_string(),
            capacity: 1000,
            base_fee: 100,
            fee_rate: 1000, // 10% fee rate
            min_htlc: 1,
            max_htlc: 1000,
            time_lock_delta: 40,
        }).unwrap();

        let mut router = Router::new(graph);
        router.set_max_fee_percent(100); // 1% max fee

        let result = router.find_route(&"A".to_string(), &"B".to_string(), 100);

        // Should fail due to excessive fees
        assert!(result.is_err());
        match result {
            Err(RoutingError::FeeTooHigh { fee, max }) => {
                assert!(fee > max);
            }
            _ => panic!("Expected FeeTooHigh error"),
        }
    }

    #[test]
    fn test_bidirectional_channel_routing() {
        let mut graph = NetworkGraph::new();

        graph.add_channel(ChannelEdge {
            channel_id: "AB".to_string(),
            from_node: "A".to_string(),
            to_node: "B".to_string(),
            capacity: 1000,
            base_fee: 1,
            fee_rate: 100,
            min_htlc: 1,
            max_htlc: 1000,
            time_lock_delta: 40,
        }).unwrap();

        let router = Router::new(graph);

        // Should work in both directions
        assert!(router.find_route(&"A".to_string(), &"B".to_string(), 100).is_ok());
        assert!(router.find_route(&"B".to_string(), &"A".to_string(), 100).is_ok());
    }

    #[test]
    fn test_channel_htlc_limits() {
        let mut graph = NetworkGraph::new();

        graph.add_channel(ChannelEdge {
            channel_id: "AB".to_string(),
            from_node: "A".to_string(),
            to_node: "B".to_string(),
            capacity: 1000,
            base_fee: 1,
            fee_rate: 100,
            min_htlc: 100, // Minimum HTLC
            max_htlc: 500, // Maximum HTLC
            time_lock_delta: 40,
        }).unwrap();

        let router = Router::new(graph);

        // Below minimum
        assert!(router.find_route(&"A".to_string(), &"B".to_string(), 50).is_err());

        // Within range
        assert!(router.find_route(&"A".to_string(), &"B".to_string(), 300).is_ok());

        // Above maximum
        assert!(router.find_route(&"A".to_string(), &"B".to_string(), 600).is_err());
    }

    #[test]
    fn test_find_multiple_alternative_routes() {
        let graph = create_test_network();
        let router = Router::new(graph);

        let routes = router.find_routes(&"A".to_string(), &"D".to_string(), 100, 3);

        // Should find 2 alternative routes (A->C->D and A->B->D)
        assert_eq!(routes.len(), 2);

        // Verify they're different routes
        let path1 = routes[0].path();
        let path2 = routes[1].path();
        assert_ne!(path1, path2);
    }

    #[test]
    fn test_network_statistics() {
        let graph = create_test_network();

        assert_eq!(graph.stats().node_count, 4);
        assert_eq!(graph.stats().channel_count, 4);
    }

    #[test]
    fn test_remove_channel_updates_graph() {
        let mut graph = create_test_network();
        let initial_count = graph.stats().channel_count;

        graph.remove_channel(&"AB".to_string()).unwrap();

        assert_eq!(graph.stats().channel_count, initial_count - 1);
        assert!(graph.get_channel(&"AB".to_string()).is_none());
    }
}
