use common::agent::information::*;

use anyhow::Result;
use axum::{
    response::{IntoResponse, Json},
};
use sysinfo::{Disks, System};
use wgpu::{Backends, Instance};

pub async fn get_server_information() -> impl IntoResponse {
    let mut sys = System::new_all();

    let device_info = get_device_information().await.unwrap();
    let cpu_info = get_cpu_information(&mut sys).await.unwrap();
    let memory_info = get_memory_information(&mut sys).await.unwrap();
    let disk_info = get_disk_information().await.unwrap();
    let gpu_info = get_gpu_information().await.unwrap();

    Json(ServerInformation {
        device: device_info,
        cpu: cpu_info,
        memory: memory_info,
        disk: disk_info,
        gpu: gpu_info
    })
}

async fn get_device_information() -> Result<Device> {
    Ok(Device {
        hostname: System::host_name().unwrap_or("Unknown hostname".to_string()).to_string(),
        os: System::long_os_version().unwrap_or("Unknown OS".to_string()).to_string(),
        kernel: System::kernel_version().unwrap_or("Unknown kernel version".to_string()).to_string(),
    })
}

async fn get_cpu_information(sys: &mut System) -> Result<Cpu> {
    sys.refresh_cpu_all();
    Ok(Cpu {
        name: sys.cpus().first().unwrap().brand().to_string(),
        base_freq_mhz: sys.cpus().first().unwrap().frequency(),
        cores: System::physical_core_count().unwrap() as u32,
        threads: sys.cpus().len() as u32,
    })
}

async fn get_memory_information(sys: &mut System) -> Result<Memory> {
    sys.refresh_memory();
    Ok(Memory {
        total_bytes: sys.total_memory()
    })
}

async fn get_disk_information() -> Result<Vec<Disk>> {
    let mut disks = Disks::new();
    disks.refresh(true);

    let storage = disks
        .iter()
        .map(|disk| Disk {
            mount: disk.mount_point().to_string_lossy().to_string(),
            total_bytes: disk.total_space(),
            device: disk.name().to_string_lossy().to_string(),
        })
        .collect::<Vec<Disk>>();
    Ok(storage)
}

// ToDo: 正確な値を返せるようにする
async fn get_gpu_information() -> Result<Vec<Gpu>> {
    let instance = Instance::new(&wgpu::InstanceDescriptor {
        backends: Backends::PRIMARY,
        ..Default::default()
    });
    let gpu = instance.enumerate_adapters(Backends::PRIMARY)
        .await.iter()
        .map(|gpu| Gpu {
            name: gpu.get_info().name,
            video_ram_mb: gpu.get_info().device,
            driver_version: gpu.get_info().driver,
        })
        .collect::<Vec<Gpu>>();

    Ok(gpu)
}