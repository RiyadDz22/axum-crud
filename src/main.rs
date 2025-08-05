mod db;
mod handlers;
mod models;
use axum::{
    Router,
    routing::{get, patch},
};
use db::init_db;
use handlers::{create_task, delete_task, get_tasks, update_task};
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
    println!("connected to {}", listner.local_addr().unwrap());

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
