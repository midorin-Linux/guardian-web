use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Cpu {
    pub usage_percent: f32,
    pub cores: u64,
    pub threads: u64,
}

#[derive(Deserialize, Serialize)]
pub struct Memory {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub free_bytes: u64
}

#[derive(Deserialize, Serialize)]
pub struct Disk {
    pub mount: String,
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub free_bytes: u64,
    pub device: String
}

#[derive(Deserialize, Serialize)]
pub struct ServerMetrics {
    pub cpu: Cpu,
    pub memory: Memory,
    pub disk: Vec<Disk>,
    pub uptime_seconds: u64
}