use axum::{routing::get, Router};
use crate::app::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/products", get(list_products_stub))
}

async fn list_products_stub() -> &'static str {
    "products endpoint stub"
}
