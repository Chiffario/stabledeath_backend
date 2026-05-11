use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};

use crate::{
    server::ServerState,
    types::{PointLineResponse, RatioRegressionResponse},
};

pub async fn user_count_graph(State(state): State<ServerState>) -> Json<PointLineResponse> {
    let response: Json<PointLineResponse> = state.lock().await.cache().daily_user_graph().into();
    tracing::info!(
        points = response.0.timestamp.len(),
        "Served daily graph data"
    );

    response
}

pub async fn history_user_graph(State(state): State<ServerState>) -> Json<PointLineResponse> {
    let response: Json<PointLineResponse> =
        state.lock().await.cache().historical_user_graph().into();
    tracing::info!(
        points = response.0.timestamp.len(),
        "Served history graph data"
    );

    response
}

pub async fn ratio_estimate(
    State(state): State<ServerState>,
    Path(percentage): Path<f64>,
) -> Result<Json<RatioRegressionResponse>, StatusCode> {
    if !percentage.is_finite() || !(0.0..=100.0).contains(&percentage) {
        tracing::warn!(percentage, "Rejected invalid ratio estimate target");
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut state = state.lock().await;
    let estimate = state
        .database()
        .estimate_ratio_percentage(percentage)
        .await
        .inspect_err(|error| tracing::warn!(%error, percentage, "Failed to estimate ratio target"))
        .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(Json(estimate.into()))
}

pub fn router() -> Router<ServerState> {
    tracing::debug!("Building graphs router");
    Router::new()
        .route("/day", get(user_count_graph))
        .route("/history", get(history_user_graph))
        .route("/ratio_estimate/{percentage}", get(ratio_estimate))
}
