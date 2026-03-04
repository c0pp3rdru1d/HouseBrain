use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

use db::Db;

use crate::health::health_handler;
use crate::routes;

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
}

pub fn build_app(db: Db) -> Router {
    let state = AppState { db };

    Router::new()
        .route("/health", get(health_handler))
        .nest("/v1", routes::v1_router())
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}
