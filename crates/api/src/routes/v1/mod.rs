use axum::Router;
use crate::app::AppState;

pub mod products;
pub mod inventory;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(products::router())
        .merge(inventory::router())
}
