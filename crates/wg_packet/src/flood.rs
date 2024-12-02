use crate::Packet;
use std::fmt::Display;
use wg_network::{NodeId, SourceRoutingHeader};

#[derive(Debug, Clone)]
pub enum NodeType {
    Client,
    Drone,
    Server,
}

#[derive(Debug, Clone)]
pub struct FloodRequest {
    pub flood_id: u64,
    pub initiator_id: NodeId,
    pub path_trace: Vec<(NodeId, NodeType)>,
}

impl FloodRequest {
    /// Creates a new flood request with the given initiator id.
    pub fn new(flood_id: u64, initiator_id: NodeId) -> Self {
        Self {
            flood_id,
            initiator_id,
            path_trace: Vec::new(),
        }
    }
    /// Initializes the flood request with the given initiator id and type.
    pub fn initialize(flood_id: u64, initiator_id: NodeId, initiator_type: NodeType) -> Self {
        Self {
            flood_id,
            initiator_id,
            path_trace: vec![(initiator_id, initiator_type)],
        }
    }
    /// Increments the path trace by one node.
    pub fn increment(&mut self, node_id: NodeId, node_type: NodeType) {
        self.path_trace.push((node_id, node_type));
    }
    pub fn get_incremented(&self, node_id: NodeId, node_type: NodeType) -> FloodRequest {
        let mut clone = self.clone();
        clone.increment(node_id, node_type);
        clone
    }
    /// Generates a response packet to the flood request.
    pub fn generate_response(&self, session_id: u64) -> Packet {
        let source_routing = SourceRoutingHeader::initialize(
            self.path_trace
                .iter()
                .map(|(id, _)| id.clone())
                .rev()
                .collect(),
        );
        Packet::new_flood_response(
            source_routing,
            session_id,
            FloodResponse {
                flood_id: self.flood_id,
                path_trace: self.path_trace.clone(),
            },
        )
    }
}

impl Display for FloodRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct FloodResponse {
    pub flood_id: u64,
    pub path_trace: Vec<(NodeId, NodeType)>,
}

impl Display for FloodResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
