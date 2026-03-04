use serde::{Deserialize, Serialize};

use crate::ids::ProductId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: ProductId,
    pub sku: String,
    pub name: String,
    pub is_active: bool,
}
