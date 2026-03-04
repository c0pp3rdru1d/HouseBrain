use sqlx::{query, query_as, PgPool};
use uuid::Uuid;

use crate::models::ProductRow;

#[derive(Clone)]
pub struct ProductsRepo {
    pool: PgPool,
}

impl ProductsRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_product(
        &self,
        id: Uuid,
        sku: &str,
        name: &str,
    ) -> Result<ProductRow, sqlx::Error> {
        query_as!(
            ProductRow,
            r#"
            insert into products (id, sku, name, is_active)
            values ($1, $2, $3, true)
            returning id, sku, name, is_active
            "#,
            id,
            sku,
            name
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn list_products(&self) -> Result<Vec<ProductRow>, sqlx::Error> {
        query_as!(
            ProductRow,
            r#"
            select id, sku, name, is_active
            from products
            order by sku asc
            "#
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn exists_sku(&self, sku: &str) -> Result<bool, sqlx::Error> {
        let rec = query!(
            r#"
            select exists(select 1 from products where sku = $1) as "exists!"
            "#,
            sku
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(rec.exists)
    }
}
