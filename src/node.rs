use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
    fmt,
};

use serde::{Deserialize, Serialize};

use crate::{
    log::{LogEntry, LogEntryKey},
    rpc::{Commit, Prepare, PrepareOk, Reply, Request, RespondableRPC, RPC},
};

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
    pub last_reply: Option<Reply>,
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
    log: BTreeMap<LogEntryKey, LogEntry>, // TODO: make this a trait so it can be modular
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
            log: BTreeMap::new(),
            commit_number: 0,
            client_table: HashMap::new(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RPCError {
    #[error("dropped request")]
    DroppedRequest { reason: String },
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

    pub async fn handle_client_request(
        &mut self,
        request: Request,
    ) -> Result<Reply, Box<dyn std::error::Error>> {
        let new_client = if let Some(entry) = self.state.client_table.get_mut(&request.client_id) {
            if request.request_number <= entry.request_number {
                if request.request_number == entry.request_number {
                    // send the most recent response if we have it
                    if let Some(reply) = &entry.last_reply {
                        return Ok(reply.clone());
                    }
                }

                // drop the request
                return Err(RPCError::DroppedRequest {
                    reason: "Request number is out of order".to_string(),
                }
                .into());
            }
            false
        } else {
            // We need to add this client to the client table
            self.state.client_table.insert(
                request.client_id,
                ClientTableEntry {
                    request_number: request.request_number,
                    last_reply: None,
                },
            );
            true
        };

        // Advance the op number
        self.state.op_number += 1;

        // append to the log
        let log_entry = LogEntry {
            view_number: self.state.view_number,
            op_number: self.state.op_number,
            request: request.clone(),
        };
        self.state.log.insert(log_entry.key(), log_entry);

        // Update the client table only if it's not a new client (we already updated the request number)
        if !new_client {
            self.state
                .client_table
                .get_mut(&request.client_id)
                .unwrap()
                .request_number = request.request_number;
        }

        let replica_request = RPC::Prepare(Prepare {
            view_number: self.state.view_number,
            op_number: self.state.op_number,
            request: request.clone(),
            commit_number: self.state.commit_number,
        });

        // TODO: send RPC to other replicas

        // up call to application
        let application_response = self.upcall_application(request.payload).await?;

        // increment commit number
        self.state.commit_number += 1;

        Ok(Reply {
            view_number: self.state.view_number,
            request_number: request.request_number,
            payload: application_response,
        })
    }

    pub async fn handle_peer_rpc(&mut self, rpc: RPC) -> Result<RPC, Box<dyn std::error::Error>> {
        match rpc {
            RPC::Prepare(prepare) => {
                let response = self.prepare(prepare).await?;
                Ok(RPC::PrepareOk(response))
            }
            _ => panic!("Unsupported RPC"),
        }
    }

    async fn prepare(&mut self, rpc: Prepare) -> Result<PrepareOk, Box<dyn std::error::Error>> {
        // TODO: check that this OP number is in order, if not, state transfer

        // update the op number
        self.state.op_number += 1;

        // append to the log
        let log_entry = LogEntry {
            view_number: self.state.view_number,
            op_number: self.state.op_number,
            request: rpc.request.clone(),
        };
        self.state.log.insert(log_entry.key(), log_entry);

        Ok(PrepareOk {
            view_number: self.state.view_number,
            op_number: self.state.op_number,
            node_id: self.id,
        })
    }

    /// In this case, the commit number is the op number
    async fn commit_op(
        &mut self,
        view_number: usize,
        commit_number: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: verify that the commit number with view number is in the log, if not, state transfer
        // TODO: can do this all in the background since it's in the log, as long as it's in order, so we can batch the upcalls
        // TODO: we can also commit anything lower than this in the log if it has not been too

        // Get the committed request without holding a mutable reference
        let committed_request = self
            .state
            .log
            .get(&LogEntryKey::new(view_number, commit_number))
            .cloned()
            .ok_or_else(|| Box::<dyn std::error::Error>::from("Log entry not found"))?;

        // Call previous commit to application code from log
        let application_response = self
            .upcall_application(committed_request.request.payload.clone())
            .await?;

        // Increment commit number
        self.state.commit_number += 1;

        // Update the client table
        self.update_client_table(&committed_request, application_response);

        Ok(())
    }

    fn update_client_table(&mut self, committed_request: &LogEntry, application_response: Vec<u8>) {
        let client_id = committed_request.request.client_id;
        let request_number = committed_request.request.request_number;

        let entry = self
            .state
            .client_table
            .entry(client_id)
            .or_insert(ClientTableEntry {
                request_number,
                last_reply: None,
            });

        entry.request_number = request_number;
        entry.last_reply = Some(Reply {
            view_number: self.state.view_number,
            request_number,
            payload: application_response,
        });
    }

    async fn upcall_application(
        &mut self,
        payload: Vec<u8>,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        panic!("Not implemented");
    }

    pub async fn start(self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
