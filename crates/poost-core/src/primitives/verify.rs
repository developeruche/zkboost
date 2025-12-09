use ere_zkvm_interface::Proof;
use serde::{Deserialize, Serialize};
use crate::common::ProgramID;


#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyRequest {
    pub program_id: ProgramID,
    pub proof: Proof,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyResponse {
    pub program_id: ProgramID,
    pub verified: bool,
    pub public_values: Vec<u8>,
    // Empty if verification returned true
    pub failure_reason: String,
}