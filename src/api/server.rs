use std::net::SocketAddr;

use axum::{extract::State, routing::get, Json, Router};
use tokio::net::TcpListener;

use crate::diagnose;
use crate::models::AppUsageData;
use crate::state::SharedState;

pub fn router(shared_state: SharedState) -> Router {
    Router::new()
        .route("/usage", get(get_usage))
        .with_state(shared_state)
}

pub async fn serve(shared_state: SharedState, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = match TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(error) => {
            diagnose::log_error(
                &format!("unable to bind HTTP usage API to 127.0.0.1:{port}"),
                error,
            );
            return;
        }
    };

    if let Err(error) = axum::serve(listener, router(shared_state)).await {
        diagnose::log_error("usage HTTP API server stopped unexpectedly", error);
    }
}

async fn get_usage(State(shared_state): State<SharedState>) -> Json<AppUsageData> {
    let data = shared_state
        .read()
        .ok()
        .and_then(|state| state.clone())
        .unwrap_or_default();

    Json(data)
}
