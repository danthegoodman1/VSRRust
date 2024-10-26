use std::{collections::HashMap, fmt};

use crate::rpc::{Prepare, Reply, Request, RespondableRPC, RPC};

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

#[derive(Debug, Clone)]
struct ClientTableEntry {
    /// The request number of the last request sent to the client
    pub request_number: usize,
    /// The last request sent to the client, if it was executed
    pub last_request: Option<Reply>,
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
    log: Vec<Request>, // TODO: make this a trait so it can be modular
    pub commit_number: usize,
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
            commit_number: 0,
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

    pub async fn handle_client_request(&mut self, request: Request) -> Result<Reply, Box<dyn std::error::Error>> {
        let new_client = if let Some(entry) = self.state.client_table.get_mut(&request.client_id) {
            if request.request_number <= entry.request_number {
                if request.request_number == entry.request_number {
                    // send the most recent response if we have it
                    if let Some(reply) = &entry.last_request {
                        return Ok(reply.clone());
                    }
                }

                // TODO: drop the request
                return Err("Request number is out of order, dropping".into());
            }
            false
        } else {
            // We need to add this client to the client table
            self.state.client_table.insert(request.client_id, ClientTableEntry {
                request_number: request.request_number,
                last_request: None,
            });
            true
        };

        // Advance the op number
        self.state.op_number += 1;

        // append to the log
        self.state.log.push(request.clone());

        // Update the client table only if it's not a new client (we already updated the request number)
        if !new_client {
            self.state.client_table.get_mut(&request.client_id).unwrap().request_number = request.request_number;
        }

        // TODO: Send Prepare RPC to other replicas
        let rpc = RPC::Prepare(Prepare {
            view_number: self.state.view_number,
            op_number: self.state.op_number,
            payload: request.payload,
            commit_number: self.state.commit_number,
        });

        // TODO: up call to application
        let response_payload = vec![];

        Ok(Reply {
            view_number: self.state.view_number,
            request_number: request.request_number,
            payload: response_payload,
        })
    }

    pub fn handle_peer_rpc(&mut self, rpc: RPC) -> Result<RPC, Box<dyn std::error::Error>> {
        panic!("Not implemented");
    }

    pub async fn start(self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
