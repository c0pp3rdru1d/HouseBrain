use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct ProductRow {
    pub id: Uuid,
    pub sku: String,
    pub name: String,
    pub is_active: bool,
}

#[derive(Debug, Clone, FromRow)]
pub struct InventoryEventRow {
    pub id: Uuid,
    pub product_id: Uuid,
    pub qty_delta: i64,
    pub event_type: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct InventoryBalanceRow {
    pub product_id: Uuid,
    pub qty_on_hand: i64,
}
