use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{app::AppState, error::ApiError};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/inventory/receive", post(receive_inventory))
        .route("/inventory/balances", get(list_balances))
}

#[derive(Debug, Deserialize)]
pub struct ReceiveRequest {
    pub product_id: Uuid,
    pub qty: i64,
}

#[derive(Debug, Serialize)]
pub struct InventoryEventResponse {
    pub id: Uuid,
    pub product_id: Uuid,
    pub qty_delta: i64,
    pub event_type: String,
}

#[derive(Debug, Serialize)]
pub struct InventoryBalanceResponse {
    pub product_id: Uuid,
    pub qty_on_hand: i64,
}

pub async fn receive_inventory(
    State(state): State<AppState>,
    Json(req): Json<ReceiveRequest>,
) -> Result<Json<InventoryEventResponse>, ApiError> {
    if req.qty <= 0 {
        return Err(ApiError::BadRequest("qty must be > 0".into()));
    }

    // Friendly error instead of a FK failure
    let exists = sqlx::query_scalar!(
        r#"select exists(select 1 from products where id = $1) as "exists!""#,
        req.product_id
    )
    .fetch_one(&state.db)
    .await?;

    if !exists {
        return Err(ApiError::BadRequest("product_id not found".into()));
    }

    let id = Uuid::new_v4();
    let row = state.inventory.receive(id, req.product_id, req.qty).await?;

    Ok(Json(InventoryEventResponse {
        id: row.id,
        product_id: row.product_id,
        qty_delta: row.qty_delta,
        event_type: row.event_type,
    }))
}

pub async fn list_balances(
    State(state): State<AppState>,
) -> Result<Json<Vec<InventoryBalanceResponse>>, ApiError> {
    let rows = state.inventory.balances().await?;

    Ok(Json(
        rows.into_iter()
            .map(|r| InventoryBalanceResponse {
                product_id: r.product_id,
                qty_on_hand: r.qty_on_hand,
            })
            .collect(),
    ))
}
