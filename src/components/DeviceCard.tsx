import type { DeviceInfo } from "../types/device";

interface DeviceCardProps {
  device: DeviceInfo | null;
  error: string | null;
  isLoading: boolean;
}

export function DeviceCard({ device, error, isLoading }: DeviceCardProps) {
  return (
    <section className="section" aria-labelledby="this-device-heading">
      <div className="section-heading">
        <div>
          <span className="eyebrow">Local connection</span>
          <h2 id="this-device-heading">This Device</h2>
        </div>
        <span className="status"><span className="status-dot" />Online</span>
      </div>

      <div className="device-card">
        <div className="device-icon" aria-hidden="true">
          <svg viewBox="0 0 24 24"><rect x="3" y="4" width="18" height="13" rx="2" /><path d="M8 21h8M12 17v4" /></svg>
        </div>
        <div className="device-details">
          {isLoading && <p className="device-name skeleton-text">Reading this device…</p>}
          {error && <p className="device-error">Could not load device details: {error}</p>}
          {device && (
            <>
              <p className="device-name">{device.name}</p>
              <div className="metadata">
                <span>{device.ipAddress}</span>
                <span className="separator" aria-hidden="true" />
                <span>{device.operatingSystem}</span>
              </div>
            </>
          )}
        </div>
      </div>
    </section>
  );
}
