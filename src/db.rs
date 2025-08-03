use sqlx::postgres::PgPoolOptions;
pub async fn init_db() -> sqlx::PgPool {
    let db_url = std::env::var("DB_URL").expect("database url not found");
    PgPoolOptions::new()
        .max_connections(16)
        .connect(&db_url)
        .await
        .expect("can't connect to the db")
}
