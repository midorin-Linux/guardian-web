use crate::models::specifications::StorageSpec;

use anyhow::Result;
use sysinfo::Disks;

pub async fn get_storage_info(disks: &Disks) -> Result<Vec<StorageSpec>> {
    let storage = disks
        .iter()
        .map(|disk| StorageSpec {
            model: disk.mount_point().to_string_lossy().to_string(),
            capacity_gb: disk.total_space() / 1024 / 1024 / 1024,
        })
        .collect::<Vec<StorageSpec>>();
    Ok(storage)
}
