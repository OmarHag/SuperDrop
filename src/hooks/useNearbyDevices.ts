import { useEffect, useState } from "react";
import { getNearbyDevices } from "../services/deviceService";
import type { NearbyDevice } from "../types/device";

const DISCOVERY_REFRESH_INTERVAL_MS = 2_000;

interface UseNearbyDevicesResult {
  devices: NearbyDevice[];
  error: string | null;
  isSearching: boolean;
  isAdvertising: boolean;
}

export function useNearbyDevices(): UseNearbyDevicesResult {
  const [devices, setDevices] = useState<NearbyDevice[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [isAdvertising, setIsAdvertising] = useState(false);

  useEffect(() => {
    let isMounted = true;

    const refreshDevices = async () => {
      try {
        const snapshot = await getNearbyDevices();
        if (isMounted) {
          setDevices(snapshot.devices);
          setIsAdvertising(snapshot.diagnostics.advertised);
          setError(snapshot.diagnostics.lastError);
        }
      } catch (reason: unknown) {
        if (isMounted) {
          setError(reason instanceof Error ? reason.message : String(reason));
        }
      }
    };

    void refreshDevices();
    const interval = window.setInterval(refreshDevices, DISCOVERY_REFRESH_INTERVAL_MS);

    return () => {
      isMounted = false;
      window.clearInterval(interval);
    };
  }, []);

  return {
    devices,
    error,
    isSearching: devices.length === 0 && error === null,
    isAdvertising,
  };
}
