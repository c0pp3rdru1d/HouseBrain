use axum::{routing::get, Router};
use crate::app::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/inventory", get(inventory_stub))
}

async fn inventory_stub() -> &'static str {
    "inventory endpoint stub"
}
