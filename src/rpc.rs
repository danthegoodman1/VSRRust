
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
