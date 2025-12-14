use common::central::information::ServerInformation;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use sqlx::SqlitePool;

pub async fn get_servers_list(
    State(pool): State<SqlitePool>,
) -> impl IntoResponse {
    match sqlx::query_as::<_, ServerInformation>(
        r#"SELECT id, hostname, ip_address, os_type, tags, auth_profile_id, port, bastion_server_id, wol_mac_address FROM servers"#,
    )
    .fetch_all(&pool)
    .await
    {
        Ok(rows) => {
            let result: Vec<ServerInformation> = rows
                .into_iter()
                .map(|row| ServerInformation {
                    id: row.id,
                    hostname: row.hostname,
                    ip_address: row.ip_address,
                    os_type: row.os_type,
                    tags: row.tags,
                    auth_profile_id: row.auth_profile_id,
                    port: row.port,
                    bastion_server_id: row.bastion_server_id,
                    wol_mac_address: row.wol_mac_address,
                })
                .collect();
            (StatusCode::OK, Json(result)).into_response()
        },
        Err(e) => {
            tracing::error!("Failed to fetch servers list: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        },
    }
}