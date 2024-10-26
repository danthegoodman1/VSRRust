pub enum RPC {
    Request(Request),
    Prepare(Prepare),
    PrepareOk(PrepareOk),
    Reply(Reply),
    Commit(Commit),
    StartViewChange(StartViewChange),
    DoViewChange(DoViewChange),
    StartView(StartView),
    Recovery(Recovery),
    RecoveryResponse(RecoveryResponse),
}

pub struct RespondableRPC {
    pub rpc: RPC,
    pub sender: tokio::sync::mpsc::Sender<RPC>,
}

#[derive(Debug)]
pub struct Request {
    /// The operation and its arguments
    pub payload: Vec<u8>,
    /// The client id
    pub client_id: usize,
    /// The request number for the client
    pub request_number: usize,
}

#[derive(Debug)]
pub struct Prepare {
    pub view_number: usize,
    pub payload: Vec<u8>,
    pub op_number: usize,
    pub commit_number: usize,
}

#[derive(Debug)]
pub struct PrepareOk {
    pub view_number: usize,
    pub op_number: usize,
    pub node_id: usize,
}

#[derive(Debug)]
pub struct Reply {
    pub view_number: usize,
    pub request_number: usize,
    pub payload: Vec<u8>,
}

#[derive(Debug)]
pub struct Commit {
    pub view_number: usize,
    pub commit_number: usize,
}

#[derive(Debug)]
pub struct StartViewChange {
    pub new_view_number: usize,
    pub node_id: usize,
}

#[derive(Debug)]
pub struct DoViewChange {
    pub new_view_number: usize,
    pub log: Vec<Request>,
    pub last_normal_view_number: usize,
    pub op_number: usize,
    pub commit_number: usize,
    pub node_id: usize,
}

#[derive(Debug)]
pub struct StartView {
    pub new_view_number: usize,
    pub log: Vec<Request>,
    pub op_number: usize,
    pub commit_number: usize,
}

#[derive(Debug)]
pub struct Recovery {
    pub node_id: usize,
    pub nonce: usize,
}

#[derive(Debug)]
pub struct RecoveryResponse {
    pub view_number: usize,
    pub nonce: usize,
    pub log: Vec<Request>,
    pub op_number: usize,
    pub commit_number: usize,
    pub view_primary_node_id: usize,
}
