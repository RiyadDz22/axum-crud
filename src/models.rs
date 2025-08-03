use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, FromRow)]
pub struct TaskRow {
    pub task_id: i32,
    pub title: String,
}

#[derive(Deserialize)]
pub struct CreateTaskReq {
    pub title: String,
}

#[derive(Serialize, FromRow)]
pub struct CreateTaskRow {
    pub task_id: i32,
}
