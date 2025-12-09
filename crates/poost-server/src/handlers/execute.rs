use axum::{Json, extract::State, http::StatusCode};
use ere_zkvm_interface::Input;
use poost_core::primitives::execute::{ExecuteRequest, ExecuteResponse};
use tracing::instrument;
use crate::app_state::AppState;


#[instrument(skip_all)]
pub async fn execute_program(
    State(state): State<AppState>,
    Json(req): Json<ExecuteRequest>,
) -> Result<Json<ExecuteResponse>, (StatusCode, String)> {
    let program_id = req.program_id.clone();
    let programs = state.programs.read().await;

    let program = programs
        .get(&program_id)
        .ok_or((StatusCode::NOT_FOUND, "Program not found".to_string()))?;

    let input = Input::new().with_prefixed_stdin(req.input.input);

    let (public_input, report) = program.zkvm_instance.vm.execute(&input).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to execute program: {}", e),
        )
    })?;
    

    Ok(Json(ExecuteResponse {
        program_id,
        total_num_cycles: report.total_num_cycles,
        region_cycles: report.region_cycles,
        execution_time_duration: report.execution_duration,
        public_input,
    }))
}
