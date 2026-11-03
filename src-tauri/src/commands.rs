use serde::Serialize;
use tauri::State;

use crate::network::discovery::{DiscoveryService, DiscoverySnapshot};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo {
    name: String,
    ip_address: String,
    operating_system: String,
}

#[tauri::command]
pub fn get_device_info() -> Result<DeviceInfo, String> {
    current_device_info()
}

#[tauri::command]
pub fn get_nearby_devices(
    discovery: State<'_, DiscoveryService>,
) -> Result<DiscoverySnapshot, String> {
    discovery.snapshot()
}

pub fn current_device_info() -> Result<DeviceInfo, String> {
    let name = hostname::get()
        .map_err(|error| format!("failed to read hostname: {error}"))?
        .to_string_lossy()
        .into_owned();

    let ip_address = local_ip_address::local_ip()
        .map_err(|error| format!("failed to determine local IP address: {error}"))?
        .to_string();

    Ok(DeviceInfo {
        name,
        ip_address,
        operating_system: operating_system_name().to_owned(),
    })
}

impl DeviceInfo {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ip_address(&self) -> &str {
        &self.ip_address
    }

    pub fn operating_system(&self) -> &str {
        &self.operating_system
    }
}

fn operating_system_name() -> &'static str {
    match std::env::consts::OS {
        "macos" => "macOS",
        "windows" => "Windows",
        "linux" => "Linux",
        other => other,
    }
}
