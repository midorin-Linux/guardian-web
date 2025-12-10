use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, Row};

#[derive(Serialize)]
pub struct ServerInfo {
    pub server_name: String,
    pub server_address: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub async fn find_server_list(
    State(pool): State<SqlitePool>
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

            let result_row = sqlx::query(
                r#"SELECT name, address FROM servers"#,
            )
                .fetch_all(&mut *conn)
                .await;

            match result_row {
                Ok(query_result) => {
                    let result = query_result
                        .iter()
                        .map(|info| ServerInfo {
                            server_name: info.get("name"),
                            server_address: info.get("address")
                        })
                        .collect::<Vec<ServerInfo>>();
                    (
                        StatusCode::OK,
                        Json(result),
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