use axum::{
    extract::{State, Path},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::SqlitePool;

#[derive(Deserialize, Serialize)]
pub struct RegisterRequest {
    hostname: String,
    ip_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<String>>,
    port: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    bastion_server_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wol_mac_address: Option<String>
}

pub async fn edit_server_info(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
    Json(json): Json<RegisterRequest>
) -> impl IntoResponse {
    let result = sqlx::query(
        r#"UPDATE servers SET hostname=?, ip_address=?, tags=?, port=?, bastion_server_id=?, wol_mac_address=? WHERE id=?"#,
    )
        .bind(json.hostname)
        .bind(json.ip_address)
        .bind(json.tags.map(|t| serde_json::to_string(&t).unwrap_or_else(|_| "[]".to_string())))
        .bind(json.port)
        .bind(json.bastion_server_id)
        .bind(json.wol_mac_address)
        .bind(id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => {
            StatusCode::OK.into_response()
        },
        Err(e) => {
            tracing::error!("Failed to edit server's information: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response()
        },
    }
}