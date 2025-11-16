use crate::models::specifications::{CpuSpec, DeviceInfo, FullSpec, GpuSpec, RamSpec, StorageSpec};

use anyhow::Result;
use axum::{extract::Path, http::StatusCode, response::{IntoResponse, Json}};
use gfxinfo::active_gpu;
use serde::{Deserialize, Serialize};
use sysinfo::{Disks, System};

// pub async fn get_spec() -> impl IntoResponse {
//     let mut sys = System::new_all();
//     sys.refresh_all();
//
//     let (hostname, os, kernel, processor_brand, _processor_len, ram) = {
//         let hostname = System::host_name().unwrap_or_else(|| "Unknown".to_string());
//         let os = System::long_os_version().unwrap_or_else(|| "Unknown".to_string());
//         let kernel = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
//         let processor_brand = sys.cpus().first().unwrap().brand();
//         let processor_len = sys.cpus().len();
//         let ram = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
//
//         (hostname, os, kernel, processor_brand, processor_len, ram)
//     };
//
//     let device_spec = SpecResponse {
//         hostname: hostname,
//         os: os,
//         kernel: kernel,
//         processor: format!("{}", processor_brand),
//         ram: format!("{:.1} GB", ram),
//     };
//
//     Json(device_spec)
// }

/// ハードコードされた完全なスペック情報をJSONで返すハンドラ
pub async fn get_full_spec() -> impl IntoResponse {
    let disks = Disks::new_with_refreshed_list();
    let mut sys = System::new_all();
    sys.refresh_all();

    let device = DeviceInfo {
        hostname: System::host_name().unwrap_or_else(|| "Unknown".to_string()),
        os: System::long_os_version().unwrap_or_else(|| "Unknown".to_string()),
        kernel_version: System::kernel_version().unwrap_or_else(|| "Unknown".to_string())
    };
    let cpu = CpuSpec {
        name: sys.cpus().first().unwrap().brand().to_string(),
        base_freq_ghz: sys.cpus().first().unwrap().frequency() as f32 / 1000.0,
        boost_freq_ghz: sys.cpus().first().unwrap().frequency() as f32 / 1000.0 + 0.5, // 仮の値
        cores: sys.cpus().len() as u32 / 2, // 仮の値
        threads: sys.cpus().len() as u32,
    };
    let ram = RamSpec {
        capacity_gb: format!("{:.1}", sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0).parse().unwrap_or(0.0),
        ram_type: "DDR4".to_string(), // 仮の値
        speed_mhz: 3200, // 仮の値
    };
    let storage = disks.iter().map(|disk| {
        StorageSpec {
            model: disk.name().to_string_lossy().to_string(),
            capacity_gb: disk.total_space() / 1024 / 1024 / 1024,
        }
    }).collect::<Vec<StorageSpec>>();
    let gpu = GpuSpec {
        name: active_gpu().unwrap().model().to_string(),
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
