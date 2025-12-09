use axum::{Json, extract::State};
use poost_core::{info::{get_cpu_info, get_gpu_info, get_memory_info, get_os_info}, primitives::info::ServerInfoResponse};
use tracing::instrument;
use crate::app_state::AppState;

#[instrument]
pub async fn get_server_info(State(state): State<AppState>) -> Json<ServerInfoResponse> {
    let program_instances = state.programs.read().await;
    let program_instances = program_instances.iter().map(|program| program.1.into()).collect();
    Json(ServerInfoResponse {
        cpu: get_cpu_info(),
        memory: get_memory_info(),
        os: get_os_info(),
        architecture: std::env::consts::ARCH.into(),
        gpu: get_gpu_info(),
        program_instances,
    })
}