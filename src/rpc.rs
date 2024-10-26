
pub struct Request {
  /// The operation and its arguments
  pub payload: Vec<u8>,
  /// The client id
  pub client_id: usize,
  /// The request number for the client
  pub request_number: usize,
}

pub struct Prepare {
  pub view_number: usize,
  pub payload: Vec<u8>,
  pub op_number: usize,
  pub commit_number: usize,
}

pub struct PrepareOk {
  pub view_number: usize,
  pub op_number: usize,
  pub node_id: usize,
}

pub struct Reply {
  pub view_number: usize,
  pub request_number: usize,
  pub payload: Vec<u8>,
}

pub struct Commit {
  pub view_number: usize,
  pub commit_number: usize,
}

pub struct StartViewChange {
  pub new_view_number: usize,
  pub node_id: usize,
}

pub struct DoViewChange {
  pub new_view_number: usize,
  pub log: Vec<Request>,
  pub last_normal_view_number: usize,
  pub op_number: usize,
  pub commit_number: usize,
  pub node_id: usize,
}

pub struct StartView {
  pub new_view_number: usize,
  pub log: Vec<Request>,
  pub op_number: usize,
  pub commit_number: usize,
}

pub struct Recovery {
  pub node_id: usize,
  pub nonce: usize,
}

pub struct RecoveryResponse {
  pub view_number: usize,
  pub nonce: usize,
  pub log: Vec<Request>,
  pub op_number: usize,
  pub commit_number: usize,
  pub view_primary_node_id: usize,
}
