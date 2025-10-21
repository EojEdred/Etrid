//! Lightning-Bloc Routing Protocol
//!
//! Multi-hop payment routing with pathfinding and fee optimization

use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Ordering;

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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
            self.outgoing_channels.entry(node_id).or_insert_with(Vec::new);
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
            .or_insert_with(Vec::new)
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
            .or_insert_with(Vec::new)
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

impl std::fmt::Display for RoutingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
}
