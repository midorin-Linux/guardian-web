use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub server_name: String,
    pub server_address: String,
    pub port: u16,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<i64>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub async fn server_register(
    State(pool): State<SqlitePool>,
    Json(json): Json<RegisterRequest>
) -> impl IntoResponse {
    let mut conn = match pool.acquire().await {
        Ok(conn) => conn,
        Err(e) => {
            tracing::error!("Failed to acquire database connection: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Database connection error".to_string(),
                }),
            )
                .into_response();
        }
    };

    // テーブルが存在するか確認するクエリ（例：servers テーブル）
    let table_check = sqlx::query(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='servers'"
    )
        .fetch_optional(&mut *conn)
        .await;

    match table_check {
        Ok(Some(_)) => {
            tracing::info!("Server registration endpoint accessed");

            let result = sqlx::query(
                r#"INSERT INTO servers (name, address, port) VALUES (?, ?, ?)"#,
            )
                .bind(json.server_name)
                .bind(json.server_address)
                .bind(json.port)
                .execute(&mut *conn)
                .await;

            match result {
                Ok(query_result) => {
                    (
                        StatusCode::OK,
                        Json(RegisterResponse {
                            success: true,
                            message: "Server registered successfully".to_string(),
                            server_id: Some(query_result.last_insert_rowid()),
                        }),
                    )
                        .into_response()
                }
                Err(e) => {
                    tracing::error!("Failed to register server: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            error: "Failed to register server".to_string(),
                        }),
                    )
                        .into_response()
                }
            }
        }
        Ok(None) => {
            tracing::warn!("Database schema not initialized");
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(ErrorResponse {
                    error: "Database not initialized".to_string(),
                }),
            )
                .into_response()
        }
        Err(e) => {
            tracing::error!("Database query error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Database query failed".to_string(),
                }),
            )
                .into_response()
        }
    }
}