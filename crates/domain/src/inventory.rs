use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::ids::{InventoryEventId, ProductId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InventoryEventType {
    Receive,
    Adjust,
    Allocate,
    Pick,
    Return,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryEvent {
    pub id: InventoryEventId,
    pub product_id: ProductId,
    pub qty_delta: i64,
    pub event_type: InventoryEventType,
    pub created_at: OffsetDateTime,
}
