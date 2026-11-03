//! Local peer discovery over mDNS.
//!
//! Discovery records are untrusted presence announcements, not proof of identity
//! and never authorization to receive a file.

use mdns_sd::{DaemonEvent, IfKind, ServiceDaemon, ServiceEvent, ServiceInfo};
use serde::Serialize;
use std::{
    collections::HashMap,
    net::IpAddr,
    sync::{Arc, Mutex},
    thread,
};

const SERVICE_TYPE: &str = "_superdrop._tcp.local.";
const PROTOCOL_VERSION: &str = "1";
const RESERVED_TRANSFER_PORT: u16 = 53_317;
const MAX_DEVICE_NAME_LENGTH: usize = 128;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NearbyDevice {
    id: String,
    name: String,
    ip_address: String,
    operating_system: String,
}

#[derive(Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscoveryDiagnostics {
    advertised: bool,
    last_error: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscoverySnapshot {
    devices: Vec<NearbyDevice>,
    diagnostics: DiscoveryDiagnostics,
}

pub struct DiscoveryService {
    daemon: ServiceDaemon,
    devices: Arc<Mutex<HashMap<String, NearbyDevice>>>,
    diagnostics: Arc<Mutex<DiscoveryDiagnostics>>,
}

impl DiscoveryService {
    pub fn start(
        device_name: &str,
        local_ip: IpAddr,
        operating_system: &str,
    ) -> Result<Self, String> {
        let daemon =
            ServiceDaemon::new().map_err(|error| format!("failed to start mDNS: {error}"))?;
        let monitor = daemon
            .monitor()
            .map_err(|error| format!("failed to monitor mDNS: {error}"))?;
        let diagnostics = Arc::new(Mutex::new(DiscoveryDiagnostics::default()));
        let monitored_diagnostics = Arc::clone(&diagnostics);
        thread::Builder::new()
            .name("superdrop-mdns-monitor".to_owned())
            .spawn(move || {
                while let Ok(event) = monitor.recv() {
                    if let Ok(mut diagnostics) = monitored_diagnostics.lock() {
                        match event {
                            DaemonEvent::Announce(_, _) => diagnostics.advertised = true,
                            DaemonEvent::Error(error) => {
                                diagnostics.last_error = Some(error.to_string())
                            }
                            _ => {}
                        }
                    }
                }
            })
            .map_err(|error| format!("failed to start discovery monitor: {error}"))?;

        let instance_name = service_instance_name(device_name, local_ip);
        let host_name = format!(
            "superdrop-{}.local.",
            local_ip.to_string().replace(['.', ':'], "-")
        );
        let properties = [
            ("version", PROTOCOL_VERSION),
            ("name", bounded_name(device_name)),
            ("os", operating_system),
        ];
        let mut service = ServiceInfo::new(
            SERVICE_TYPE,
            &instance_name,
            &host_name,
            local_ip,
            RESERVED_TRANSFER_PORT,
            &properties[..],
        )
        .map_err(|error| format!("failed to create mDNS record: {error}"))?;
        service.set_interfaces(vec![IfKind::Addr(local_ip)]);
        let own_fullname = service.get_fullname().to_owned();

        daemon
            .register(service)
            .map_err(|error| format!("failed to advertise SuperDrop: {error}"))?;
        let receiver = daemon
            .browse(SERVICE_TYPE)
            .map_err(|error| format!("failed to browse for SuperDrop devices: {error}"))?;
        let devices = Arc::new(Mutex::new(HashMap::new()));
        let discovered_devices = Arc::clone(&devices);

        thread::Builder::new()
            .name("superdrop-mdns-browser".to_owned())
            .spawn(move || {
                while let Ok(event) = receiver.recv() {
                    match event {
                        ServiceEvent::ServiceResolved(service) => {
                            if service.get_fullname() == own_fullname {
                                continue;
                            }

                            if let Some(device) = resolved_device(&service) {
                                if let Ok(mut devices) = discovered_devices.lock() {
                                    devices.insert(device.id.clone(), device);
                                }
                            }
                        }
                        ServiceEvent::ServiceRemoved(_, fullname) => {
                            if let Ok(mut devices) = discovered_devices.lock() {
                                devices.remove(&fullname);
                            }
                        }
                        _ => {}
                    }
                }
            })
            .map_err(|error| format!("failed to start discovery listener: {error}"))?;

        Ok(Self {
            daemon,
            devices,
            diagnostics,
        })
    }

    pub fn snapshot(&self) -> Result<DiscoverySnapshot, String> {
        let devices = self
            .devices
            .lock()
            .map_err(|_| "discovery state is unavailable".to_owned())?;
        let mut snapshot: Vec<_> = devices.values().cloned().collect();
        snapshot.sort_by(|left, right| left.name.to_lowercase().cmp(&right.name.to_lowercase()));
        let diagnostics = self
            .diagnostics
            .lock()
            .map_err(|_| "discovery diagnostics are unavailable".to_owned())?
            .clone();
        Ok(DiscoverySnapshot {
            devices: snapshot,
            diagnostics,
        })
    }
}

impl Drop for DiscoveryService {
    fn drop(&mut self) {
        let _ = self.daemon.shutdown();
    }
}

fn resolved_device(service: &mdns_sd::ResolvedService) -> Option<NearbyDevice> {
    if service.get_property_val_str("version")? != PROTOCOL_VERSION {
        return None;
    }

    let name = service.get_property_val_str("name")?.trim();
    let operating_system = service.get_property_val_str("os")?.trim();
    if name.is_empty()
        || name.chars().count() > MAX_DEVICE_NAME_LENGTH
        || operating_system.is_empty()
        || operating_system.chars().count() > 32
    {
        return None;
    }

    let ip_address = service
        .get_addresses_v4()
        .into_iter()
        .next()
        .map(|address| address.to_string())
        .or_else(|| {
            service
                .get_addresses()
                .iter()
                .next()
                .map(ToString::to_string)
        })?;

    Some(NearbyDevice {
        id: service.get_fullname().to_owned(),
        name: name.to_owned(),
        ip_address,
        operating_system: operating_system.to_owned(),
    })
}

fn service_instance_name(device_name: &str, local_ip: IpAddr) -> String {
    format!(
        "{}-{}",
        bounded_name(device_name),
        local_ip.to_string().replace(['.', ':'], "-")
    )
}

fn bounded_name(name: &str) -> &str {
    name.char_indices()
        .nth(MAX_DEVICE_NAME_LENGTH)
        .map_or(name, |(index, _)| &name[..index])
}

#[cfg(test)]
mod tests {
    use super::{bounded_name, service_instance_name};
    use std::net::{IpAddr, Ipv4Addr};

    #[test]
    fn instance_name_contains_device_and_address() {
        let name = service_instance_name("Omar's Mac", IpAddr::V4(Ipv4Addr::new(192, 168, 1, 4)));
        assert_eq!(name, "Omar's Mac-192-168-1-4");
    }

    #[test]
    fn advertised_names_are_bounded_on_utf8_boundaries() {
        let long_name = "é".repeat(140);
        assert_eq!(bounded_name(&long_name).chars().count(), 128);
    }
}
