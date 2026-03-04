use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{app::AppState, error::ApiError};

pub fn router() -> Router<AppState> {
    Router::new().route("/products", get(list_products).post(create_product))
}

#[derive(Debug, Deserialize)]
pub struct CreateProductRequest {
    pub sku: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct ProductResponse {
    pub id: Uuid,
    pub sku: String,
    pub name: String,
    pub is_active: bool,
}

pub async fn create_product(
    State(state): State<AppState>,
    Json(req): Json<CreateProductRequest>,
) -> Result<Json<ProductResponse>, ApiError> {
    let sku = req.sku.trim();
    let name = req.name.trim();

    if sku.is_empty() {
        return Err(ApiError::BadRequest("sku is required".into()));
    }
    if name.is_empty() {
        return Err(ApiError::BadRequest("name is required".into()));
    }

    let sku_norm = sku.to_uppercase();

    if state.products.exists_sku(&sku_norm).await? {
        return Err(ApiError::Conflict(format!("sku already exists: {sku_norm}")));
    }

    let id = Uuid::new_v4();
    let row = state
        .products
        .create_product(id, &sku_norm, name)
        .await?;

    Ok(Json(ProductResponse {
        id: row.id,
        sku: row.sku,
        name: row.name,
        is_active: row.is_active,
    }))
}

pub async fn list_products(
    State(state): State<AppState>,
) -> Result<Json<Vec<ProductResponse>>, ApiError> {
    let rows = state.products.list_products().await?;

    Ok(Json(
        rows.into_iter()
            .map(|r| ProductResponse {
                id: r.id,
                sku: r.sku,
                name: r.name,
                is_active: r.is_active,
            })
            .collect(),
    ))
}
