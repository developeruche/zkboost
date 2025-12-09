use crate::{
    app_state::AppState,
    handlers::{
        execute::execute_program, info::get_server_info, prove::prove_program, verify::verify_proof,
    },
};
use axum::{
    Router,
    http::Method,
    routing::{get, post},
};
use poost_core::config::PoostConfig;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

pub async fn run_server(config: &PoostConfig, app_state: AppState) -> anyhow::Result<()> {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let router = Router::new()
        .route("/", get(|| async { "Poost Server" }))
        .route("/info", get(get_server_info))
        .route("/execute", post(execute_program))
        .route("/prove", post(prove_program))
        .route("/verify", post(verify_proof))
        .with_state(app_state)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        // 400MB limit to account for the proof size
        // and the possibly large input size
        .layer(axum::extract::DefaultBodyLimit::max(400 * 1024 * 1024));

    let addr: SocketAddr = config.server_url.parse()?;
    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C handler");
    println!("graceful shutdown");
}
