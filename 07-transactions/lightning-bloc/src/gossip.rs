//! Lightning-Bloc Gossip Protocol
//!
//! Peer-to-peer gossip protocol for network state propagation:
//! - Channel announcements
//! - Channel updates
//! - Node announcements
//! - Network state synchronization

#[cfg(not(feature = "std"))]
use alloc::{
    collections::{BTreeMap as HashMap, BTreeSet as HashSet},
    string::{String, ToString},
    vec,
    vec::Vec,
    format,
};

#[cfg(not(feature = "std"))]
use core::{
    default::Default,
    result::Result::{self, Ok, Err},
    option::Option::{self, Some, None},
    fmt,
};

#[cfg(feature = "std")]
use std::{
    collections::{HashMap, HashSet},
    default::Default,
    vec::Vec,
    string::String,
    result::Result::{self, Ok, Err},
    option::Option::{self, Some, None},
    fmt,
};

use crate::routing::{NetworkGraph, ChannelEdge, NodeId, ChannelId, RoutingError};

/// Gossip message types
#[derive(Debug, Clone, PartialEq)]
pub enum GossipMessage {
    NodeAnnouncement(NodeAnnouncement),
    ChannelAnnouncement(ChannelAnnouncement),
    ChannelUpdate(ChannelUpdate),
    SyncRequest(SyncRequest),
    SyncResponse(SyncResponse),
}

impl GossipMessage {
    /// Get the message type identifier
    pub fn message_type(&self) -> &str {
        match self {
            GossipMessage::NodeAnnouncement(_) => "node_announcement",
            GossipMessage::ChannelAnnouncement(_) => "channel_announcement",
            GossipMessage::ChannelUpdate(_) => "channel_update",
            GossipMessage::SyncRequest(_) => "sync_request",
            GossipMessage::SyncResponse(_) => "sync_response",
        }
    }

    /// Get the timestamp of the message
    pub fn timestamp(&self) -> u64 {
        match self {
            GossipMessage::NodeAnnouncement(msg) => msg.timestamp,
            GossipMessage::ChannelAnnouncement(msg) => msg.timestamp,
            GossipMessage::ChannelUpdate(msg) => msg.timestamp,
            GossipMessage::SyncRequest(msg) => msg.timestamp,
            GossipMessage::SyncResponse(msg) => msg.timestamp,
        }
    }
}

/// Node announcement message
#[derive(Debug, Clone, PartialEq)]
pub struct NodeAnnouncement {
    pub node_id: NodeId,
    pub alias: String,
    pub addresses: Vec<String>,
    pub features: NodeFeatures,
    pub timestamp: u64,
    pub signature: Vec<u8>,
}

impl NodeAnnouncement {
    pub fn new(
        node_id: NodeId,
        alias: String,
        addresses: Vec<String>,
        features: NodeFeatures,
        timestamp: u64,
        signature: Vec<u8>,
    ) -> Self {
        Self {
            node_id,
            alias,
            addresses,
            features,
            timestamp,
            signature,
        }
    }

    /// Verify the node announcement signature
    pub fn verify_signature(&self) -> bool {
        // In production, this would verify the cryptographic signature
        // For now, we just check that signature is not empty
        !self.signature.is_empty()
    }
}

/// Node feature flags
#[derive(Debug, Clone, PartialEq, Default)]
pub struct NodeFeatures {
    pub supports_multi_hop: bool,
    pub supports_watchtower: bool,
    pub supports_multi_party: bool,
    pub supports_post_quantum: bool,
}

/// Channel announcement message
#[derive(Debug, Clone, PartialEq)]
pub struct ChannelAnnouncement {
    pub channel_id: ChannelId,
    pub node_a: NodeId,
    pub node_b: NodeId,
    pub capacity: u128,
    pub timestamp: u64,
    pub signature_a: Vec<u8>,
    pub signature_b: Vec<u8>,
}

impl ChannelAnnouncement {
    pub fn new(
        channel_id: ChannelId,
        node_a: NodeId,
        node_b: NodeId,
        capacity: u128,
        timestamp: u64,
        signature_a: Vec<u8>,
        signature_b: Vec<u8>,
    ) -> Self {
        Self {
            channel_id,
            node_a,
            node_b,
            capacity,
            timestamp,
            signature_a,
            signature_b,
        }
    }

    /// Verify both signatures on the channel announcement
    pub fn verify_signatures(&self) -> bool {
        // In production, this would verify both cryptographic signatures
        !self.signature_a.is_empty() && !self.signature_b.is_empty()
    }
}

/// Channel update message
#[derive(Debug, Clone, PartialEq)]
pub struct ChannelUpdate {
    pub channel_id: ChannelId,
    pub direction: ChannelDirection,
    pub base_fee: u64,
    pub fee_rate: u32,
    pub min_htlc: u128,
    pub max_htlc: u128,
    pub time_lock_delta: u32,
    pub is_disabled: bool,
    pub timestamp: u64,
    pub signature: Vec<u8>,
}

impl ChannelUpdate {
    pub fn new(
        channel_id: ChannelId,
        direction: ChannelDirection,
        base_fee: u64,
        fee_rate: u32,
        min_htlc: u128,
        max_htlc: u128,
        time_lock_delta: u32,
        is_disabled: bool,
        timestamp: u64,
        signature: Vec<u8>,
    ) -> Self {
        Self {
            channel_id,
            direction,
            base_fee,
            fee_rate,
            min_htlc,
            max_htlc,
            time_lock_delta,
            is_disabled,
            timestamp,
            signature,
        }
    }

    /// Verify the channel update signature
    pub fn verify_signature(&self) -> bool {
        !self.signature.is_empty()
    }
}

/// Direction of a channel (which node is sending)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChannelDirection {
    NodeAToB,
    NodeBToA,
}

impl fmt::Display for ChannelDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChannelDirection::NodeAToB => write!(f, "A→B"),
            ChannelDirection::NodeBToA => write!(f, "B→A"),
        }
    }
}

/// Synchronization request message
#[derive(Debug, Clone, PartialEq)]
pub struct SyncRequest {
    pub requesting_node: NodeId,
    pub last_sync_timestamp: u64,
    pub timestamp: u64,
}

impl SyncRequest {
    pub fn new(requesting_node: NodeId, last_sync_timestamp: u64, timestamp: u64) -> Self {
        Self {
            requesting_node,
            last_sync_timestamp,
            timestamp,
        }
    }
}

/// Synchronization response message
#[derive(Debug, Clone, PartialEq)]
pub struct SyncResponse {
    pub responding_node: NodeId,
    pub node_announcements: Vec<NodeAnnouncement>,
    pub channel_announcements: Vec<ChannelAnnouncement>,
    pub channel_updates: Vec<ChannelUpdate>,
    pub timestamp: u64,
}

impl SyncResponse {
    pub fn new(
        responding_node: NodeId,
        node_announcements: Vec<NodeAnnouncement>,
        channel_announcements: Vec<ChannelAnnouncement>,
        channel_updates: Vec<ChannelUpdate>,
        timestamp: u64,
    ) -> Self {
        Self {
            responding_node,
            node_announcements,
            channel_announcements,
            channel_updates,
            timestamp,
        }
    }

    /// Get total number of messages in the sync response
    pub fn message_count(&self) -> usize {
        self.node_announcements.len()
            + self.channel_announcements.len()
            + self.channel_updates.len()
    }
}

/// Gossip protocol manager
pub struct GossipManager {
    /// Our node ID
    our_node_id: NodeId,
    /// Network graph that we're maintaining
    graph: NetworkGraph,
    /// Stored node announcements
    node_announcements: HashMap<NodeId, NodeAnnouncement>,
    /// Stored channel announcements
    channel_announcements: HashMap<ChannelId, ChannelAnnouncement>,
    /// Channel updates indexed by channel ID and direction
    channel_updates: HashMap<ChannelId, HashMap<ChannelDirection, ChannelUpdate>>,
    /// Peers we're connected to
    peers: HashSet<NodeId>,
    /// Message propagation statistics
    stats: GossipStatistics,
}

impl GossipManager {
    /// Create a new gossip manager
    pub fn new(our_node_id: NodeId) -> Self {
        Self {
            our_node_id,
            graph: NetworkGraph::new(),
            node_announcements: HashMap::new(),
            channel_announcements: HashMap::new(),
            channel_updates: HashMap::new(),
            peers: HashSet::new(),
            stats: GossipStatistics::default(),
        }
    }

    /// Add a peer to our peer list
    pub fn add_peer(&mut self, peer_id: NodeId) {
        if self.peers.insert(peer_id) {
            self.stats.peer_count += 1;
        }
    }

    /// Remove a peer from our peer list
    pub fn remove_peer(&mut self, peer_id: &NodeId) {
        if self.peers.remove(peer_id) {
            self.stats.peer_count = self.stats.peer_count.saturating_sub(1);
        }
    }

    /// Process an incoming gossip message
    pub fn process_message(&mut self, message: GossipMessage) -> Result<(), GossipError> {
        match message {
            GossipMessage::NodeAnnouncement(msg) => self.process_node_announcement(msg),
            GossipMessage::ChannelAnnouncement(msg) => self.process_channel_announcement(msg),
            GossipMessage::ChannelUpdate(msg) => self.process_channel_update(msg),
            GossipMessage::SyncRequest(msg) => self.process_sync_request(msg),
            GossipMessage::SyncResponse(msg) => self.process_sync_response(msg),
        }
    }

    /// Process a node announcement
    fn process_node_announcement(&mut self, announcement: NodeAnnouncement) -> Result<(), GossipError> {
        // Verify signature
        if !announcement.verify_signature() {
            return Err(GossipError::InvalidSignature);
        }

        // Check if we already have a newer announcement
        if let Some(existing) = self.node_announcements.get(&announcement.node_id) {
            if existing.timestamp >= announcement.timestamp {
                return Err(GossipError::StaleMessage);
            }
        }

        // Add node to graph
        self.graph.add_node(announcement.node_id.clone());

        // Store announcement
        self.node_announcements.insert(announcement.node_id.clone(), announcement);
        self.stats.nodes_announced += 1;

        Ok(())
    }

    /// Process a channel announcement
    fn process_channel_announcement(&mut self, announcement: ChannelAnnouncement) -> Result<(), GossipError> {
        // Verify signatures
        if !announcement.verify_signatures() {
            return Err(GossipError::InvalidSignature);
        }

        // Check if we already have this channel
        if self.channel_announcements.contains_key(&announcement.channel_id) {
            return Err(GossipError::DuplicateChannel);
        }

        // Create channel edge with default parameters
        let edge = ChannelEdge {
            channel_id: announcement.channel_id.clone(),
            from_node: announcement.node_a.clone(),
            to_node: announcement.node_b.clone(),
            capacity: announcement.capacity,
            base_fee: 0, // Will be updated by ChannelUpdate messages
            fee_rate: 0,
            min_htlc: 1,
            max_htlc: announcement.capacity,
            time_lock_delta: 40,
        };

        // Add to graph
        self.graph.add_channel(edge)
            .map_err(|e| GossipError::RoutingError(e))?;

        // Store announcement
        self.channel_announcements.insert(announcement.channel_id.clone(), announcement);
        self.stats.channels_announced += 1;

        Ok(())
    }

    /// Process a channel update
    fn process_channel_update(&mut self, update: ChannelUpdate) -> Result<(), GossipError> {
        // Verify signature
        if !update.verify_signature() {
            return Err(GossipError::InvalidSignature);
        }

        // Check if channel exists
        if !self.channel_announcements.contains_key(&update.channel_id) {
            return Err(GossipError::UnknownChannel);
        }

        // Check if we already have a newer update for this direction
        if let Some(updates) = self.channel_updates.get(&update.channel_id) {
            if let Some(existing) = updates.get(&update.direction) {
                if existing.timestamp >= update.timestamp {
                    return Err(GossipError::StaleMessage);
                }
            }
        }

        // Get channel announcement to know the node ordering
        let announcement = self.channel_announcements.get(&update.channel_id)
            .ok_or(GossipError::UnknownChannel)?;

        // Determine which direction to update
        let (from_node, to_node) = match update.direction {
            ChannelDirection::NodeAToB => (announcement.node_a.clone(), announcement.node_b.clone()),
            ChannelDirection::NodeBToA => (announcement.node_b.clone(), announcement.node_a.clone()),
        };

        // Update the channel in the graph
        // Note: NetworkGraph stores bidirectional channels, so we need to update the specific direction
        if let Some(channel) = self.graph.get_channel(&update.channel_id) {
            let mut updated_edge = channel.clone();

            // Only update if this matches the direction
            if updated_edge.from_node == from_node && updated_edge.to_node == to_node {
                updated_edge.base_fee = update.base_fee;
                updated_edge.fee_rate = update.fee_rate;
                updated_edge.min_htlc = update.min_htlc;
                updated_edge.max_htlc = update.max_htlc;
                updated_edge.time_lock_delta = update.time_lock_delta;

                // Handle disabled channels by setting capacity to 0
                if update.is_disabled {
                    self.graph.update_capacity(&update.channel_id, 0)
                        .map_err(|e| GossipError::RoutingError(e))?;
                }
            }
        }

        // Store the update
        self.channel_updates
            .entry(update.channel_id.clone())
            .or_default()
            .insert(update.direction, update);

        self.stats.channels_updated += 1;

        Ok(())
    }

    /// Process a sync request
    fn process_sync_request(&mut self, request: SyncRequest) -> Result<(), GossipError> {
        // Filter messages newer than the requested timestamp
        let node_announcements: Vec<NodeAnnouncement> = self.node_announcements
            .values()
            .filter(|a| a.timestamp > request.last_sync_timestamp)
            .cloned()
            .collect();

        let channel_announcements: Vec<ChannelAnnouncement> = self.channel_announcements
            .values()
            .filter(|a| a.timestamp > request.last_sync_timestamp)
            .cloned()
            .collect();

        let channel_updates: Vec<ChannelUpdate> = self.channel_updates
            .values()
            .flat_map(|updates| updates.values())
            .filter(|u| u.timestamp > request.last_sync_timestamp)
            .cloned()
            .collect();

        // In a real implementation, we would send the sync response to the requesting peer
        // For now, we just track the statistics
        self.stats.sync_requests_received += 1;

        Ok(())
    }

    /// Process a sync response
    fn process_sync_response(&mut self, response: SyncResponse) -> Result<(), GossipError> {
        // Process all node announcements
        for announcement in response.node_announcements {
            let _ = self.process_node_announcement(announcement);
        }

        // Process all channel announcements
        for announcement in response.channel_announcements {
            let _ = self.process_channel_announcement(announcement);
        }

        // Process all channel updates
        for update in response.channel_updates {
            let _ = self.process_channel_update(update);
        }

        self.stats.sync_responses_received += 1;

        Ok(())
    }

    /// Broadcast a node announcement to all peers
    pub fn broadcast_node_announcement(&mut self, announcement: NodeAnnouncement) -> Result<(), GossipError> {
        // Process the announcement ourselves first
        self.process_node_announcement(announcement.clone())?;

        // In a real implementation, we would send this to all peers
        self.stats.messages_broadcasted += 1;

        Ok(())
    }

    /// Broadcast a channel announcement to all peers
    pub fn broadcast_channel_announcement(&mut self, announcement: ChannelAnnouncement) -> Result<(), GossipError> {
        // Process the announcement ourselves first
        self.process_channel_announcement(announcement.clone())?;

        // In a real implementation, we would send this to all peers
        self.stats.messages_broadcasted += 1;

        Ok(())
    }

    /// Broadcast a channel update to all peers
    pub fn broadcast_channel_update(&mut self, update: ChannelUpdate) -> Result<(), GossipError> {
        // Process the update ourselves first
        self.process_channel_update(update.clone())?;

        // In a real implementation, we would send this to all peers
        self.stats.messages_broadcasted += 1;

        Ok(())
    }

    /// Request network state synchronization from a peer
    pub fn request_sync(&mut self, peer_id: NodeId, last_sync_timestamp: u64) -> SyncRequest {
        let request = SyncRequest::new(
            self.our_node_id.clone(),
            last_sync_timestamp,
            self.current_timestamp(),
        );

        self.stats.sync_requests_sent += 1;

        request
    }

    /// Create a sync response for a peer
    pub fn create_sync_response(&self, since_timestamp: u64) -> SyncResponse {
        let node_announcements: Vec<NodeAnnouncement> = self.node_announcements
            .values()
            .filter(|a| a.timestamp > since_timestamp)
            .cloned()
            .collect();

        let channel_announcements: Vec<ChannelAnnouncement> = self.channel_announcements
            .values()
            .filter(|a| a.timestamp > since_timestamp)
            .cloned()
            .collect();

        let channel_updates: Vec<ChannelUpdate> = self.channel_updates
            .values()
            .flat_map(|updates| updates.values())
            .filter(|u| u.timestamp > since_timestamp)
            .cloned()
            .collect();

        SyncResponse::new(
            self.our_node_id.clone(),
            node_announcements,
            channel_announcements,
            channel_updates,
            self.current_timestamp(),
        )
    }

    /// Get a reference to the network graph
    pub fn graph(&self) -> &NetworkGraph {
        &self.graph
    }

    /// Get a mutable reference to the network graph
    pub fn graph_mut(&mut self) -> &mut NetworkGraph {
        &mut self.graph
    }

    /// Get gossip statistics
    pub fn stats(&self) -> &GossipStatistics {
        &self.stats
    }

    /// Get our node ID
    pub fn our_node_id(&self) -> &NodeId {
        &self.our_node_id
    }

    /// Get the list of peers
    pub fn peers(&self) -> Vec<NodeId> {
        self.peers.iter().cloned().collect()
    }

    /// Get node announcement for a specific node
    pub fn get_node_announcement(&self, node_id: &NodeId) -> Option<&NodeAnnouncement> {
        self.node_announcements.get(node_id)
    }

    /// Get channel announcement for a specific channel
    pub fn get_channel_announcement(&self, channel_id: &ChannelId) -> Option<&ChannelAnnouncement> {
        self.channel_announcements.get(channel_id)
    }

    /// Get channel updates for a specific channel
    pub fn get_channel_updates(&self, channel_id: &ChannelId) -> Option<&HashMap<ChannelDirection, ChannelUpdate>> {
        self.channel_updates.get(channel_id)
    }

    /// Get current timestamp (placeholder - in production this would use actual system time)
    fn current_timestamp(&self) -> u64 {
        // In production, this would return actual system time
        0
    }
}

/// Gossip protocol statistics
#[derive(Debug, Clone, Default)]
pub struct GossipStatistics {
    pub peer_count: usize,
    pub nodes_announced: usize,
    pub channels_announced: usize,
    pub channels_updated: usize,
    pub messages_broadcasted: usize,
    pub sync_requests_sent: usize,
    pub sync_requests_received: usize,
    pub sync_responses_received: usize,
}

/// Gossip protocol errors
#[derive(Debug, Clone, PartialEq)]
pub enum GossipError {
    InvalidSignature,
    StaleMessage,
    DuplicateChannel,
    UnknownChannel,
    RoutingError(RoutingError),
}

impl fmt::Display for GossipError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GossipError::InvalidSignature => write!(f, "Invalid signature"),
            GossipError::StaleMessage => write!(f, "Stale message"),
            GossipError::DuplicateChannel => write!(f, "Duplicate channel"),
            GossipError::UnknownChannel => write!(f, "Unknown channel"),
            GossipError::RoutingError(e) => write!(f, "Routing error: {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_signature() -> Vec<u8> {
        vec![1, 2, 3, 4, 5]
    }

    #[test]
    fn test_gossip_manager_creation() {
        let manager = GossipManager::new("node1".to_string());
        assert_eq!(manager.our_node_id(), "node1");
        assert_eq!(manager.peers().len(), 0);
    }

    #[test]
    fn test_add_remove_peer() {
        let mut manager = GossipManager::new("node1".to_string());

        manager.add_peer("peer1".to_string());
        assert_eq!(manager.peers().len(), 1);
        assert_eq!(manager.stats().peer_count, 1);

        manager.remove_peer(&"peer1".to_string());
        assert_eq!(manager.peers().len(), 0);
        assert_eq!(manager.stats().peer_count, 0);
    }

    #[test]
    fn test_node_announcement() {
        let mut manager = GossipManager::new("node1".to_string());

        let announcement = NodeAnnouncement::new(
            "node2".to_string(),
            "Node2".to_string(),
            vec!["127.0.0.1:9735".to_string()],
            NodeFeatures::default(),
            1000,
            create_test_signature(),
        );

        assert!(manager.process_node_announcement(announcement).is_ok());
        assert_eq!(manager.stats().nodes_announced, 1);
        assert!(manager.graph().has_node(&"node2".to_string()));
    }

    #[test]
    fn test_channel_announcement() {
        let mut manager = GossipManager::new("node1".to_string());

        let announcement = ChannelAnnouncement::new(
            "ch1".to_string(),
            "nodeA".to_string(),
            "nodeB".to_string(),
            1000,
            1000,
            create_test_signature(),
            create_test_signature(),
        );

        assert!(manager.process_channel_announcement(announcement).is_ok());
        assert_eq!(manager.stats().channels_announced, 1);
        assert!(manager.graph().get_channel(&"ch1".to_string()).is_some());
    }

    #[test]
    fn test_channel_update() {
        let mut manager = GossipManager::new("node1".to_string());

        // First announce the channel
        let announcement = ChannelAnnouncement::new(
            "ch1".to_string(),
            "nodeA".to_string(),
            "nodeB".to_string(),
            1000,
            1000,
            create_test_signature(),
            create_test_signature(),
        );
        manager.process_channel_announcement(announcement).unwrap();

        // Then update it
        let update = ChannelUpdate::new(
            "ch1".to_string(),
            ChannelDirection::NodeAToB,
            10,
            100,
            1,
            900,
            40,
            false,
            1100,
            create_test_signature(),
        );

        assert!(manager.process_channel_update(update).is_ok());
        assert_eq!(manager.stats().channels_updated, 1);
    }

    #[test]
    fn test_channel_update_unknown_channel() {
        let mut manager = GossipManager::new("node1".to_string());

        let update = ChannelUpdate::new(
            "unknown_ch".to_string(),
            ChannelDirection::NodeAToB,
            10,
            100,
            1,
            900,
            40,
            false,
            1100,
            create_test_signature(),
        );

        assert_eq!(
            manager.process_channel_update(update),
            Err(GossipError::UnknownChannel)
        );
    }

    #[test]
    fn test_stale_node_announcement() {
        let mut manager = GossipManager::new("node1".to_string());

        let announcement1 = NodeAnnouncement::new(
            "node2".to_string(),
            "Node2".to_string(),
            vec!["127.0.0.1:9735".to_string()],
            NodeFeatures::default(),
            1000,
            create_test_signature(),
        );
        manager.process_node_announcement(announcement1).unwrap();

        // Try to process older announcement
        let announcement2 = NodeAnnouncement::new(
            "node2".to_string(),
            "Node2 Old".to_string(),
            vec!["127.0.0.1:9735".to_string()],
            NodeFeatures::default(),
            900, // Older timestamp
            create_test_signature(),
        );

        assert_eq!(
            manager.process_node_announcement(announcement2),
            Err(GossipError::StaleMessage)
        );
    }

    #[test]
    fn test_sync_request() {
        let mut manager = GossipManager::new("node1".to_string());

        let request = manager.request_sync("peer1".to_string(), 0);
        assert_eq!(request.requesting_node, "node1");
        assert_eq!(manager.stats().sync_requests_sent, 1);
    }

    #[test]
    fn test_sync_response() {
        let mut manager = GossipManager::new("node1".to_string());

        // Add some data
        let announcement = NodeAnnouncement::new(
            "node2".to_string(),
            "Node2".to_string(),
            vec![],
            NodeFeatures::default(),
            1000,
            create_test_signature(),
        );
        manager.process_node_announcement(announcement).unwrap();

        // Create sync response
        let response = manager.create_sync_response(0);
        assert_eq!(response.node_announcements.len(), 1);
        assert_eq!(response.responding_node, "node1");
    }

    #[test]
    fn test_broadcast_channel_announcement() {
        let mut manager = GossipManager::new("node1".to_string());

        let announcement = ChannelAnnouncement::new(
            "ch1".to_string(),
            "nodeA".to_string(),
            "nodeB".to_string(),
            1000,
            1000,
            create_test_signature(),
            create_test_signature(),
        );

        assert!(manager.broadcast_channel_announcement(announcement).is_ok());
        assert_eq!(manager.stats().messages_broadcasted, 1);
        assert_eq!(manager.stats().channels_announced, 1);
    }

    #[test]
    fn test_invalid_signature() {
        let mut manager = GossipManager::new("node1".to_string());

        let announcement = NodeAnnouncement::new(
            "node2".to_string(),
            "Node2".to_string(),
            vec![],
            NodeFeatures::default(),
            1000,
            vec![], // Empty signature
        );

        assert_eq!(
            manager.process_node_announcement(announcement),
            Err(GossipError::InvalidSignature)
        );
    }

    #[test]
    fn test_message_type() {
        let announcement = GossipMessage::NodeAnnouncement(NodeAnnouncement::new(
            "node1".to_string(),
            "Node1".to_string(),
            vec![],
            NodeFeatures::default(),
            1000,
            create_test_signature(),
        ));

        assert_eq!(announcement.message_type(), "node_announcement");
        assert_eq!(announcement.timestamp(), 1000);
    }

    #[test]
    fn test_channel_direction_display() {
        assert_eq!(format!("{}", ChannelDirection::NodeAToB), "A→B");
        assert_eq!(format!("{}", ChannelDirection::NodeBToA), "B→A");
    }

    #[test]
    fn test_sync_response_message_count() {
        let response = SyncResponse::new(
            "node1".to_string(),
            vec![NodeAnnouncement::new(
                "node2".to_string(),
                "Node2".to_string(),
                vec![],
                NodeFeatures::default(),
                1000,
                create_test_signature(),
            )],
            vec![],
            vec![],
            1000,
        );

        assert_eq!(response.message_count(), 1);
    }

    #[test]
    fn test_get_node_announcement() {
        let mut manager = GossipManager::new("node1".to_string());

        let announcement = NodeAnnouncement::new(
            "node2".to_string(),
            "Node2".to_string(),
            vec![],
            NodeFeatures::default(),
            1000,
            create_test_signature(),
        );
        manager.process_node_announcement(announcement).unwrap();

        let retrieved = manager.get_node_announcement(&"node2".to_string());
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().alias, "Node2");
    }

    #[test]
    fn test_get_channel_announcement() {
        let mut manager = GossipManager::new("node1".to_string());

        let announcement = ChannelAnnouncement::new(
            "ch1".to_string(),
            "nodeA".to_string(),
            "nodeB".to_string(),
            1000,
            1000,
            create_test_signature(),
            create_test_signature(),
        );
        manager.process_channel_announcement(announcement).unwrap();

        let retrieved = manager.get_channel_announcement(&"ch1".to_string());
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().capacity, 1000);
    }

    #[test]
    fn test_duplicate_channel_announcement() {
        let mut manager = GossipManager::new("node1".to_string());

        let announcement = ChannelAnnouncement::new(
            "ch1".to_string(),
            "nodeA".to_string(),
            "nodeB".to_string(),
            1000,
            1000,
            create_test_signature(),
            create_test_signature(),
        );

        manager.process_channel_announcement(announcement.clone()).unwrap();

        assert_eq!(
            manager.process_channel_announcement(announcement),
            Err(GossipError::DuplicateChannel)
        );
    }
}
