use axum::{Json, extract::State, http::StatusCode};
use poost_core::primitives::verify::{VerifyRequest, VerifyResponse};
use tracing::instrument;
use crate::app_state::AppState;




#[instrument(skip_all)]
pub async fn verify_proof(
    State(state): State<AppState>,
    Json(req): Json<VerifyRequest>,
) -> Result<Json<VerifyResponse>, (StatusCode, String)> {
    // Check if the program_id is correct
    let programs = state.programs.read().await;

    let program = programs
        .get(&req.program_id)
        .ok_or((StatusCode::NOT_FOUND, "Program not found".to_string()))?;

    // Verify the proof   
    let (verified, failure_reason, public_values) = match program.zkvm_instance.vm.verify(&req.proof) {
        Ok(public_values) => (true, String::default(), public_values),
        Err(err) => (false, format!("{}", err), vec![]),
    };

    Ok(Json(VerifyResponse {
        program_id: req.program_id,
        verified,
        public_values,
        failure_reason,
    }))
}