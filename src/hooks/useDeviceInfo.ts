import { useEffect, useState } from "react";
import { getDeviceInfo } from "../services/deviceService";
import type { DeviceInfo } from "../types/device";

interface UseDeviceInfoResult {
  device: DeviceInfo | null;
  error: string | null;
  isLoading: boolean;
}

export function useDeviceInfo(): UseDeviceInfoResult {
  const [device, setDevice] = useState<DeviceInfo | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    let isMounted = true;

    getDeviceInfo()
      .then((deviceInfo) => {
        if (isMounted) setDevice(deviceInfo);
      })
      .catch((reason: unknown) => {
        if (isMounted) {
          const message = reason instanceof Error ? reason.message : String(reason);
          setError(message || "Unable to read device information.");
        }
      })
      .finally(() => {
        if (isMounted) setIsLoading(false);
      });

    return () => {
      isMounted = false;
    };
  }, []);

  return { device, error, isLoading };
}
