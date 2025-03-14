use anyhow::Result;
use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};

pub async fn init_db(database_url: &str) -> Result<Pool<Sqlite>> {
    // Create connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    // Run migrations (requires "sqlx-cli" dev-dependency or you can embed migrations)
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}