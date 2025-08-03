mod db;
mod models;
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{delete, get, patch},
};
use db::init_db;
use models::{CreateTaskReq, CreateTaskRow, TaskRow};
use serde_json::json;
use sqlx::PgPool;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    //.env file initiliazing
    dotenvy::dotenv().expect("unable to access .env file");

    //exposing .env file vars
    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned());

    //create db pool
    let db_pool = init_db().await;

    //tcp_listner
    let listner = TcpListener::bind(server_address)
        .await
        .expect("can't connect to the server");
    println!("connecting to {}", listner.local_addr().unwrap());

    //compose the routes
    let app = Router::new()
        .route("/", get(|| async { "hello world" }))
        .route("/tasks", get(get_tasks).post(create_task))
        .route("/tasks/:task_id", patch(update_task).delete(delete_task))
        .with_state(db_pool);

    axum::serve(listner, app)
        .await
        .expect("error serving application");
}

async fn get_tasks(
    State(db_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let rows = sqlx::query_as::<_, TaskRow>("SELECT task_id, title FROM tasks ORDER BY task_id")
        .fetch_all(&db_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "success": false, "message": e.to_string() }).to_string(),
            )
        })?;

    Ok((
        StatusCode::OK,
        json!({ "success": true, "data": rows }).to_string(),
    ))
}

async fn create_task(
    State(db_pool): State<PgPool>,
    Json(task): Json<CreateTaskReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let row = sqlx::query_as::<_, CreateTaskRow>(
        "INSERT INTO tasks (title) VALUES ($1) RETURNING task_id",
    )
    .bind(task.title)
    .fetch_one(&db_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "success": false, "message": e.to_string() }).to_string(),
        )
    })?;

    Ok((
        StatusCode::CREATED,
        json!({ "success": true, "data": row }).to_string(),
    ))
}
async fn update_task(
    State(db_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    todo!();
}

async fn delete_task(
    State(db_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    todo!();
}
