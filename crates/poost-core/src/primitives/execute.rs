use crate::{common::ProgramID, program::ProgramInput};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecuteRequest {
    pub program_id: ProgramID,
    pub input: ProgramInput,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecuteResponse {
    pub program_id: ProgramID,
    pub total_num_cycles: u64,
    pub region_cycles: IndexMap<String, u64>,
    pub execution_time_duration: Duration,
    pub public_input: Vec<u8>,
}
