use common::DeviceInfo;

use anyhow::Result;
use sysinfo::System;

pub async fn get_device_info() -> Result<DeviceInfo> {
    let device = DeviceInfo {
        hostname: System::host_name().unwrap_or_else(|| "Unknown".to_string()),
        os: System::long_os_version().unwrap_or_else(|| "Unknown".to_string()),
        kernel_version: System::kernel_version().unwrap_or_else(|| "Unknown".to_string()),
    };
    Ok(device)
}
