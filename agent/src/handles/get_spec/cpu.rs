use common::CpuSpec;

use anyhow::Result;
use sysinfo::System;

pub async fn get_cpu_info(sys: &System) -> Result<CpuSpec> {
    let cpu = CpuSpec {
        name: sys.cpus().first().unwrap().brand().to_string(),
        base_freq_ghz: format!(
            "{:.1}",
            sys.cpus().first().unwrap().frequency() as f64 / 1000.0
        )
        .parse()
        .unwrap_or(0.0),
        cores: System::physical_core_count().unwrap() as u32,
        threads: sys.cpus().len() as u32,
    };
    Ok(cpu)
}
