use crate::models::{CreateTaskReq, CreateTaskRow, TaskRow};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde_json::json;
use sqlx::PgPool;

pub async fn get_tasks(
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

pub async fn create_task(
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

pub async fn update_task(
    State(db_pool): State<PgPool>,
    Path(task_id): Path<i32>,
    Json(task): Json<CreateTaskReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    sqlx::query("UPDATE tasks SET title = $1 WHERE task_id = $2")
        .bind(&task.title)
        .bind(task_id)
        .execute(&db_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "success": false, "message": e.to_string() }).to_string(),
            )
        })?;

    Ok((
        StatusCode::OK,
        json!({ "success": true, "message": "Task updated successfully" }).to_string(),
    ))
}

pub async fn delete_task(
    State(db_pool): State<PgPool>,
    Path(task_id): Path<i32>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    sqlx::query("DELETE FROM tasks WHERE task_id = $1")
        .bind(task_id)
        .execute(&db_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "success": false, "message": e.to_string() }).to_string(),
            )
        })?;

    Ok((
        StatusCode::CREATED,
        json!({ "success": true, "message": "Task deleted successfully" }).to_string(),
    ))
}
