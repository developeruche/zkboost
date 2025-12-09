//! Module holds Axum app state and related functionalities.
use poost_core::{common::ProgramID, config::PoostConfig, program::ProgramInstance};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct AppState {
    pub programs: Arc<RwLock<HashMap<ProgramID, ProgramInstance>>>,
}

impl AppState {
    pub async fn init(config: &PoostConfig) -> Self {
        let state = AppState {
            programs: Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
        };

        for program_params in config.program_instances.iter() {
            let program_instance = ProgramInstance::new(
                program_params.name.clone(),
                program_params.program_path.clone(),
                program_params.zkvm_name.clone(),
            )
            .unwrap();
            let mut program_instances = state.programs.write().await;
            program_instances.insert(program_instance.program_id.clone(), program_instance);
        }

        state
    }
}
