import { invoke } from "@tauri-apps/api/core";
import type { DeviceInfo, DiscoverySnapshot } from "../types/device";

export function getDeviceInfo(): Promise<DeviceInfo> {
  return invoke<DeviceInfo>("get_device_info");
}

export function getNearbyDevices(): Promise<DiscoverySnapshot> {
  return invoke<DiscoverySnapshot>("get_nearby_devices");
}
