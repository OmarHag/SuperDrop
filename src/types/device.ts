export interface DeviceInfo {
  name: string;
  ipAddress: string;
  operatingSystem: string;
}

export type DeviceStatus = "online" | "offline";

export interface NearbyDevice {
  id: string;
  name: string;
  ipAddress: string;
  operatingSystem: string;
}

export interface DiscoverySnapshot {
  devices: NearbyDevice[];
  diagnostics: {
    advertised: boolean;
    lastError: string | null;
  };
}
