use crate::{node::NodeIdentifier, rpc::{Reply, Request, RPC}};

pub struct Client {
    pub id: usize,
    pub next_request_number: usize,
    pub nodes: Vec<NodeIdentifier>,
    pub leader_index: usize,
}

impl Client {
    pub fn new(id: usize, leader_index: usize) -> Self {
        Self {
            id,
            next_request_number: 0,
            nodes: Vec::new(),
            leader_index,
        }
    }

    pub fn set_nodes(&mut self, nodes: Vec<NodeIdentifier>) {
        self.nodes = nodes;
    }

    pub fn set_leader_index(&mut self, leader_index: usize) {
        self.leader_index = leader_index;
    }

    pub fn send_request(&mut self, payload: Vec<u8>) -> Reply {
        let rpc = RPC::Request(Request {
            payload,
            client_id: self.id,
            request_number: self.next_request_number,
        });
        // TODO: send rpc using the transport protocol
        // when we get a reply, we need to update the next request number
        self.next_request_number += 1;
        panic!("Not implemented");
    }
}
