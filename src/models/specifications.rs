use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct DeviceInfo {
    pub hostname: String,
    pub os: String,
    pub kernel_version: String,
}
#[derive(Serialize)]
pub struct CpuSpec {
    pub name: String,
    pub base_freq_ghz: f32,
    pub boost_freq_ghz: f32,
    pub cores: u32,
    pub threads: u32,
}

#[derive(Serialize)]
pub struct RamSpec {
    pub capacity_gb: f64,
    #[serde(rename = "type")]
    pub ram_type: String,
    pub speed_mhz: u32,
}

#[derive(Serialize)]
pub struct StorageSpec {
    pub model: String,
    pub capacity_gb: u64,
}

#[derive(Serialize)]
pub struct GpuSpec {
    pub name: String,
    pub vram_gb: f64,
}

#[derive(Serialize)]
pub struct FullSpec {
    pub device: DeviceInfo,
    pub cpu: CpuSpec,
    pub ram: RamSpec,
    pub storage: Vec<StorageSpec>,
    pub gpu: GpuSpec,
}
