use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PoostConfig {
    pub server_url: String,
    pub program_instances: Vec<ProgramInstanceParams>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgramInstanceParams {
    pub name: String,
    pub zkvm_name: String,
    pub program_path: String,
}
