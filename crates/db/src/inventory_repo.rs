use sqlx::{query_as, PgPool};
use uuid::Uuid;

use crate::models::{InventoryBalanceRow, InventoryEventRow};

#[derive(Clone)]
pub struct InventoryRepo {
    pool: PgPool,
}

impl InventoryRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn receive(
        &self,
        id: Uuid,
        product_id: Uuid,
        qty: i64,
    ) -> Result<InventoryEventRow, sqlx::Error> {
        query_as!(
            InventoryEventRow,
            r#"
            insert into inventory_events (id, product_id, qty_delta, event_type)
            values ($1, $2, $3, 'receive'::inventory_event_type)
            returning id, product_id, qty_delta, event_type::text as "event_type!"
            "#,
            id,
            product_id,
            qty
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn balances(&self) -> Result<Vec<InventoryBalanceRow>, sqlx::Error> {
        query_as!(
            InventoryBalanceRow,
            r#"
            select product_id as "product_id!", qty_on_hand as "qty_on_hand!"
            from inventory_balances
            order by product_id
            "#
        )
        .fetch_all(&self.pool)
        .await
    }
}
