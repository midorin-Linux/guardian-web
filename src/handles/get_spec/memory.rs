use crate::models::specifications::RamSpec;

use anyhow::Result;
use sysinfo::System;

pub async fn get_memory_info(sys: &System) -> Result<RamSpec> {
    let ram = RamSpec {
        capacity_gb: format!("{:.1}", sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0)
            .parse()
            .unwrap_or(0.0),
        speed_mhz: 3200,
    };
    Ok(ram)
}
