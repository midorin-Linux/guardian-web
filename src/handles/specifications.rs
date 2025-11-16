use crate::models::specifications::{CpuSpec, DeviceInfo, FullSpec, GpuSpec, RamSpec, StorageSpec};

use axum::{response::{IntoResponse, Json}};
use gfxinfo::active_gpu;
use hardware_query::HardwareInfo;
use sysinfo::{Disks, System};

pub async fn get_full_spec() -> impl IntoResponse {
    let disks = Disks::new_with_refreshed_list();
    let mut sys = System::new_all();
    sys.refresh_all();

    let hw_info = HardwareInfo::query().unwrap();

    let device = DeviceInfo {
        hostname: System::host_name().unwrap_or_else(|| "Unknown".to_string()),
        os: System::long_os_version().unwrap_or_else(|| "Unknown".to_string()),
        kernel_version: System::kernel_version().unwrap_or_else(|| "Unknown".to_string())
    };
    let cpu = CpuSpec {
        name: hw_info.cpu.model_name,
        base_freq_ghz: format!("{:.1}", hw_info.cpu.base_frequency as f64 / 1000.0).parse().unwrap_or(0.0),
        cores: hw_info.cpu.physical_cores,
        threads: hw_info.cpu.logical_cores,
    };
    let ram = RamSpec {
        capacity_gb: format!("{:.1}", hw_info.memory.total_mb() as f64 / 1024.0).parse().unwrap_or(0.0),
        speed_mhz: hw_info.memory.speed_mhz,
    };
    let storage = disks.iter().map(|disk| {
        StorageSpec {
            model: disk.mount_point().to_string_lossy().to_string(),
            capacity_gb: disk.total_space() / 1024 / 1024 / 1024,
        }
    }).collect::<Vec<StorageSpec>>();
    let gpu = GpuSpec {
        name: hw_info.gpus.first().unwrap().model_name.clone(),
        vram_gb: format!("{:.1}", active_gpu().unwrap().info().total_vram() as f64 / 1024.0 / 1024.0 / 1024.0).parse().unwrap_or(0.0),
    };

    let spec = FullSpec {
        device,
        cpu,
        ram,
        storage,
        gpu,
    };
    Json(spec)
}
