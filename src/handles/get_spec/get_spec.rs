use crate::models::specifications::{CpuSpec, DeviceInfo, FullSpec, GpuSpec, RamSpec, StorageSpec};

use anyhow::{Context, Result};
use gfxinfo::active_gpu;
use hardware_query::HardwareInfo;
use sysinfo::{Disks, System};

pub struct GetSpec{
    disks: Disks,
    hw_info: HardwareInfo,
    system: System,
}

impl GetSpec{
    pub fn new() -> Result<Self> {
        let disks = Disks::new_with_refreshed_list();
        let mut sys = System::new_all();
        sys.refresh_all();

        let hw_info = HardwareInfo::query()?;

        Ok(Self{
            disks,
            hw_info,
            system: sys,
        })
    }

    pub async fn cpu(&mut self) -> Result<CpuSpec> {
        let cpu = CpuSpec {
            name: self.hw_info.cpu.model_name,
            base_freq_ghz: format!("{:.1}", self.hw_info.cpu.base_frequency as f64 / 1000.0)
                .parse()
                .unwrap_or(0.0),
            cores: self.hw_info.cpu.physical_cores,
            threads: self.hw_info.cpu.logical_cores,
        };
        Ok(cpu)
    }
    
    pub async fn device(&mut self) -> Result<DeviceInfo> {
        let device = DeviceInfo {
            hostname: System::host_name().unwrap_or_else(|| "Unknown".to_string()),
            os: System::long_os_version().unwrap_or_else(|| "Unknown".to_string()),
            kernel_version: System::kernel_version().unwrap_or_else(|| "Unknown".to_string())
        };
        Ok(device)
    }
    
    pub async fn ram(&mut self) -> Result<RamSpec> {
        let ram = RamSpec {
            capacity_gb: format!("{:.1}", self.hw_info.memory.total_mb() as f64 / 1024.0)
                .parse()
                .unwrap_or(0.0),
            speed_mhz: self.hw_info.memory.speed_mhz,
        };
        Ok(ram)
    }
    
    pub async fn storage(&mut self) -> Result<Vec<StorageSpec>> {
        let storage = self.disks
            .iter()
            .map(|disk| StorageSpec {
                model: disk.mount_point().to_string_lossy().to_string(),
                capacity_gb: disk.total_space() / 1024 / 1024 / 1024,
            })
            .collect::<Vec<StorageSpec>>();
        Ok(storage)
    }
    
    pub async fn gpu(&mut self) -> Result<GpuSpec> {

        let gpu = GpuSpec {
            name: self.hw_info.gpus.first().unwrap().model_name.clone(),
            vram_gb: format!(
                "{:.1}",
                active_gpu().unwrap().info().total_vram() as f64 / 1024.0 / 1024.0 / 1024.0
            )
                .parse()
                .unwrap_or(0.0),
        };
        Ok(gpu)
    }
}