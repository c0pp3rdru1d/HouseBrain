use axum::Router;
use crate::app::AppState;

pub mod products;
pub mod inventory;

pub fn router() -> Router<AppState> {
    Router::new()
        // Placeholder: we’ll add real routes next
        .merge(products::router())
        .merge(inventory::router())
}
