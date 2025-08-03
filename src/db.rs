use sqlx::postgres::PgPoolOptions;
use std::env;
pub async fn initDB() -> sqlx::PgPool {
    let db_url = std::env::var("DB_URL").expect("database url not found");
    lPgPoolOptions::new()
        .max_connections(16)
        .connect(&db_url)
        .await
        .expect("can't connect to the db")
}
