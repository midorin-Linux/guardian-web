use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct ServerInformation {
    pub id: String,
    pub hostname: String,
    pub ip_address: String,
    pub os_type: String,
    pub tags: Option<String>,
    pub auth_profile_id: String,
    pub port: u16,
    pub bastion_server_id: Option<String>,
    pub wol_mac_address: Option<String>
}