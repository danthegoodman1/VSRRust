use std::{collections::HashMap, fmt};

use crate::rpc::{Request, RespondableRPC, RPC};

#[derive(Debug)]
pub struct Node {
    // Must be unique to the node
    pub id: usize,
    state: NodeState,
    receiver: tokio::sync::mpsc::UnboundedReceiver<RespondableRPC>,
    sender: tokio::sync::mpsc::UnboundedSender<RespondableRPC>,
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

impl NodeState {
    pub fn new(nodes: Vec<NodeIdentifier>) -> Self {
        Self {
            nodes,
            replica_number: 0,
            view_number: 0,
            status: NodeStatus::Normal,
            op_number: 0,
            log: vec![],
            client_table: HashMap::new(),
        }
    }
}

impl Node {
    pub fn new(id: usize, nodes: Vec<NodeIdentifier>) -> Self {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        Self {
            id,
            state: NodeState::new(nodes),
            receiver,
            sender,
        }
    }

    pub async fn send_update(&self, update: RPC) -> Result<tokio::sync::mpsc::Receiver<RPC>, Box<dyn std::error::Error>> {
        // generate a channel for the receiver to send responses back to
        let (sender, receiver) = tokio::sync::mpsc::channel(1);

        let send = self.sender.send(RespondableRPC { rpc: update, sender });
        if let Err(e) = send {
            return Err(e.into());
        }

        Ok(receiver)
    }

    pub async fn start(self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
