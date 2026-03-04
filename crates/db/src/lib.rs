use sqlx::{Pool, Postgres};

pub type Db = Pool<Postgres>;

pub mod models;
pub mod products_repo;
pub mod inventory_repo;

pub async fn connect(database_url: &str) -> Result<Db, sqlx::Error> {
    Pool::<Postgres>::connect(database_url).await
}
