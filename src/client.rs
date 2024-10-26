use crate::node::NodeIdentifier;

pub struct Client {
    pub id: usize,
    pub last_request_number: usize,
    pub nodes: Vec<NodeIdentifier>,
    pub leader_index: usize,
}

impl Client {
    pub fn new(id: usize, leader_index: usize) -> Self {
        Self {
            id,
            last_request_number: 0,
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
}
