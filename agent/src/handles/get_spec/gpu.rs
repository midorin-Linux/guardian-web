use common::GpuSpec;

use anyhow::Result;
use wgpu::Backends;
use wgpu::Instance;

pub async fn get_gpu_info(gpu: &Instance) -> Result<GpuSpec> {
    let gpu = GpuSpec {
        name: gpu
            .enumerate_adapters(Backends::all())
            .first()
            .unwrap()
            .get_info()
            .name
            .to_string(),
        vram_gb: format!(
            "{:.1}",
            gpu.enumerate_adapters(Backends::all())
                .iter()
                .next()
                .unwrap()
                .get_info()
                .device
                / 1024
        )
        .parse()
        .unwrap_or(0.0),
    };
    Ok(gpu)
}
