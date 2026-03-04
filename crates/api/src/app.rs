use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

use db::{inventory_repo::InventoryRepo, products_repo::ProductsRepo, Db};

use crate::health::health_handler;
use crate::routes;

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
    pub products: ProductsRepo,
    pub inventory: InventoryRepo,
}

pub fn build_app(db: Db) -> Router {
    let products = ProductsRepo::new(db.clone());
    let inventory = InventoryRepo::new(db.clone());
    let state = AppState {
        db,
        products,
        inventory,
    };

    Router::new()
        .route("/health", get(health_handler))
        .nest("/v1", routes::v1_router())
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}
