use common::central::information::ServerInformation;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use sqlx::SqlitePool;

pub async fn get_server_info(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>
) -> impl IntoResponse {
    match sqlx::query_as::<_, ServerInformation>(
        r#"SELECT id, hostname, ip_address, os_type, tags, auth_profile_id, port, bastion_server_id, wol_mac_address FROM servers WHERE id = ?"#,
    )
        .bind(id)
        .fetch_one(&pool)
        .await
    {
        Ok(row) => {
            let result = ServerInformation {
                id: row.id,
                hostname: row.hostname,
                ip_address: row.ip_address,
                os_type: row.os_type,
                tags: row.tags,
                auth_profile_id: row.auth_profile_id,
                port: row.port,
                bastion_server_id: row.bastion_server_id,
                wol_mac_address: row.wol_mac_address,
            };
            (StatusCode::OK, Json(result)).into_response()
        },
        Err(e) => {
            tracing::error!("Failed to fetch server's information: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        },
    }
}