use common::central::information::ServerInformation;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use reqwest::{Client as HttpClient};
use serde_json::{Value, json};
use sqlx::SqlitePool;

pub async fn get_server_health(
    State(pool): State<SqlitePool>,
    Path(server_uuid): Path<String>,
) -> impl IntoResponse {
    match sqlx::query_as::<_, ServerInformation>(
        r#"SELECT id, hostname, ip_address, os_type, tags, auth_profile_id, port, bastion_server_id, wol_mac_address FROM servers WHERE id = ?"#,
    )
        .bind(server_uuid)
        .fetch_one(&pool)
        .await
    {
        Ok(row) => {
            let http_client = HttpClient::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .map_err(|e| {
                    tracing::error!("Failed to build HTTP client: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                })
                .unwrap();

            let req_address = format!("http://{}/api/agent/v1/health", row.ip_address);

            match http_client.get(req_address).send().await {
                Ok(_) => StatusCode::OK.into_response(),
                Err(_) => StatusCode::GATEWAY_TIMEOUT.into_response()
            }
        },
        Err(e) => {
            tracing::error!("Failed to fetch server's information: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        },
    }
}