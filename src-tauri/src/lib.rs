mod commands;
mod network;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            use tauri::Manager;

            let device = commands::current_device_info()?;
            let local_ip = device
                .ip_address()
                .parse()
                .map_err(|error| format!("invalid local IP address: {error}"))?;
            let discovery = network::discovery::DiscoveryService::start(
                device.name(),
                local_ip,
                device.operating_system(),
            )?;
            app.manage(discovery);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_device_info,
            commands::get_nearby_devices
        ])
        .run(tauri::generate_context!())
        .expect("error while running SuperDrop");
}
