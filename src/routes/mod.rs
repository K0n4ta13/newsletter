use axum::Router;
use axum::routing::{get, post};
use tower_http::trace::TraceLayer;
use super::AppState;

mod health_check;
mod subscription;

pub fn serve() -> Router<AppState> {
    health_check::init_time();

    Router::new()
        .route("/health_check", get(health_check::health_check))
        .route("/subscription", post(subscription::subscription))
        .layer(TraceLayer::new_for_http())

}