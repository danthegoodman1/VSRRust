use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use crate::rpc::Request;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogEntry {
    pub view_number: usize,
    pub op_number: usize,
    pub request: Request,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LogEntryKey(usize, usize);

impl LogEntry {
    pub fn key(&self) -> LogEntryKey {
        LogEntryKey(self.view_number, self.op_number)
    }
}

impl LogEntryKey {
    pub fn new(view_number: usize, op_number: usize) -> Self {
        Self(view_number, op_number)
    }
}
