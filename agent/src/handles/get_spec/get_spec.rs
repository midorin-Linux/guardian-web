use crate::handles::get_spec::{
    cpu::get_cpu_info, device::get_device_info, gpu::get_gpu_info, memory::get_memory_info,
    storage::get_storage_info,
};
use common::{CpuSpec, DeviceInfo, GpuSpec, RamSpec, StorageSpec};

use anyhow::Result;
use sysinfo::{Disks, System};
use wgpu::Instance;

pub struct GetSpec {
    disks: Disks,
    gpu: Instance,
    system: System,
}

impl GetSpec {
    pub fn new() -> Result<Self> {
        let disks = Disks::new_with_refreshed_list();
        let instance = Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let mut sys = System::new_all();
        sys.refresh_all();

        Ok(Self {
            disks,
            gpu: instance,
            system: sys,
        })
    }

    pub async fn cpu(&mut self) -> Result<CpuSpec> {
        match get_cpu_info(&self.system).await {
            Ok(cpu) => Ok(cpu),
            Err(e) => Err(e),
        }
    }

    pub async fn device(&mut self) -> Result<DeviceInfo> {
        match get_device_info().await {
            Ok(device) => Ok(device),
            Err(e) => Err(e),
        }
    }
    //
    pub async fn ram(&mut self) -> Result<RamSpec> {
        match get_memory_info(&self.system).await {
            Ok(ram) => Ok(ram),
            Err(e) => Err(e),
        }
    }

    pub async fn storage(&mut self) -> Result<Vec<StorageSpec>> {
        match get_storage_info(&self.disks).await {
            Ok(storages) => Ok(storages),
            Err(e) => Err(e),
        }
    }

    pub async fn gpu(&mut self) -> Result<GpuSpec> {
        match get_gpu_info(&self.gpu).await {
            Ok(gpu) => Ok(gpu),
            Err(e) => Err(e),
        }
    }
}
