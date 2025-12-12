use common::servers::Information::ServerInformation;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::SqlitePool;
use uuid::Uuid;

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

pub async fn register_server(
    State(pool): State<SqlitePool>,
    Json(json): Json<RegisterRequest>
) -> impl IntoResponse {
    let id = Uuid::new_v4();
    let hostname = json.hostname;
    let ip_address = json.ip_address;
    let os_type = "ubuntu"; //ToDo: 被管理サーバー用のアプリケーションから取得する
    let tags_json = json.tags.map(|t| serde_json::to_string(&t).unwrap_or_else(|_| "[]".to_string()));
    let auth_profile_id = Uuid::new_v4(); //ToDo: 認証情報への外部キーを取得する
    let port = json.port;
    let bastion_server_id = json.bastion_server_id;
    let wol_mac_address = json.wol_mac_address;
    
    let result = sqlx::query(
        r#"INSERT INTO servers (id, hostname, ip_address, os_type, tags, auth_profile_id, port, bastion_server_id, wol_mac_address) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
        .bind(id.to_string())
        .bind(&hostname)
        .bind(&ip_address)
        .bind(os_type)
        .bind(&tags_json)
        .bind(auth_profile_id.to_string())
        .bind(port)
        .bind(&bastion_server_id)
        .bind(&wol_mac_address)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => {
            let server_info = ServerInformation {
                id: id.to_string(),
                hostname,
                ip_address,
                os_type: os_type.to_string(),
                tags: tags_json,
                auth_profile_id: auth_profile_id.to_string(),
                port,
                bastion_server_id,
                wol_mac_address
            };
            (StatusCode::CREATED, Json(server_info)).into_response()
        },
        Err(e) => {
            tracing::error!("Failed to fetch servers list: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response()
        },
    }
}