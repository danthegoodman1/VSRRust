use std::{collections::HashMap, fmt};

use crate::rpc::Request;

#[derive(Debug)]
pub struct Node {
    // Must be unique to the node
    pub id: usize,
    state: NodeState,
}

#[derive(Debug)]
pub struct NodeIdentifier {
    pub id: usize,
    pub address: String,
}

#[derive(Debug)]
pub enum NodeStatus {
    Normal,
    ViewChange,
    Recovering,
}

impl fmt::Display for NodeStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeStatus::Normal => write!(f, "Normal"),
            NodeStatus::ViewChange => write!(f, "ViewChange"),
            NodeStatus::Recovering => write!(f, "Recovering"),
        }
    }
}

#[derive(Debug)]
struct ClientTableEntry {
    /// The request number of the last request sent to the client
    pub request_number: usize,
    /// The last request sent to the client, if it was executed
    pub last_request: Option<Request>,
}

#[derive(Debug)]
pub struct NodeState {
    /// Sorted array of nodes
    pub nodes: Vec<NodeIdentifier>,
    /// The replica number of this node (index in the nodes array)
    pub replica_number: usize,
    pub view_number: usize,
    pub status: NodeStatus,
    pub op_number: usize,

    log: Vec<Request>,
    client_table: HashMap<usize, ClientTableEntry>,
}
