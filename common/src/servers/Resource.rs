use chrono::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum Status {
    Online,
    Caution,
    Offline
}

#[derive(Deserialize, Serialize)]
pub struct Data {
    pub cpu: f32,
    pub memory_used_mib: u64,
    pub memory_total_mib: u64,
    pub disk_usage_percent: f32,
    pub status: Status
}

#[derive(Deserialize, Serialize)]
pub struct ResourceUpdate {
    pub server_id: String,
    pub timestamp: DateTime<chrono::Utc>,
    pub data: Data
}