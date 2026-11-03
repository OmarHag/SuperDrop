import type { NearbyDevice } from "../types/device";

interface NearbyDevicesProps {
  devices: NearbyDevice[];
  error: string | null;
  isSearching: boolean;
  isAdvertising: boolean;
}

export function NearbyDevices({ devices, error, isSearching, isAdvertising }: NearbyDevicesProps) {
  return (
    <section className="section" aria-labelledby="nearby-heading">
      <div className="section-heading">
        <div>
          <span className="eyebrow">Local network</span>
          <h2 id="nearby-heading">Nearby Devices</h2>
        </div>
        {isSearching && <div className="searching-indicator" aria-hidden="true"><span /><span /><span /></div>}
      </div>

      {error && <div className="discovery-error" role="alert">Discovery unavailable: {error}</div>}

      {!error && (
        <p className="discovery-diagnostic">
          <span className={isAdvertising ? "diagnostic-dot active" : "diagnostic-dot"} />
          {isAdvertising ? "Advertising on the local network" : "Starting local network advertisement…"}
        </p>
      )}

      {!error && devices.length === 0 && (
        <div className="empty-state">
          <div className="radar" aria-hidden="true"><span className="radar-core" /></div>
          <h3>Searching for nearby devices...</h3>
          <p>Keep SuperDrop open on another computer connected to this network.</p>
        </div>
      )}

      {devices.length > 0 && (
        <div className="nearby-list">
          {devices.map((device) => (
            <article className="nearby-device" key={device.id}>
              <div className="nearby-device-icon" aria-hidden="true">
                <svg viewBox="0 0 24 24"><rect x="3" y="4" width="18" height="13" rx="2" /><path d="M8 21h8M12 17v4" /></svg>
              </div>
              <div className="nearby-device-details">
                <h3>{device.name}</h3>
                <p>{device.ipAddress}<span aria-hidden="true">•</span>{device.operatingSystem}</p>
              </div>
              <span className="nearby-status"><span />Available</span>
            </article>
          ))}
        </div>
      )}
    </section>
  );
}
