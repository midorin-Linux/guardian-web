use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Device {
    pub hostname: String,
    pub os: String,
    pub kernel: String
}

#[derive(Deserialize, Serialize)]
pub struct Cpu {
    pub name: String,
    pub base_freq_mhz: u64,
    pub cores: u32,
    pub threads: u32
}

#[derive(Deserialize, Serialize)]
pub struct Memory {
    pub total_bytes: u64,
    // pub freq_hz: u32,
}

#[derive(Deserialize, Serialize)]
pub struct Disk {
    pub mount: String,
    pub total_bytes: u64,
    pub device: String
}

#[derive(Deserialize, Serialize)]
pub struct Gpu {
    pub name: String,
    pub video_ram_mb: u32,
    pub driver_version: String
}

#[derive(Deserialize, Serialize)]
pub struct ServerInformation {
    pub device: Device,
    pub cpu: Cpu,
    pub memory: Memory,
    pub disk: Vec<Disk>,
    pub gpu: Vec<Gpu>,
}