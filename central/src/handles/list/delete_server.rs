use axum::{
    extract::{State, Path},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;
use sqlx::SqlitePool;

pub async fn delete_server(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let result = sqlx::query(
        r#"DELETE FROM servers WHERE id=?"#,
    )
        .bind(id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => {
            StatusCode::OK.into_response()
        },
        Err(e) => {
            tracing::error!("Failed to delete server from list: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response()
        },
    }
}