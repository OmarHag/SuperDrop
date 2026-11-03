import { DeviceCard } from "./components/DeviceCard";
import { NearbyDevices } from "./components/NearbyDevices";
import { SettingsButton } from "./components/SettingsButton";
import { useDeviceInfo } from "./hooks/useDeviceInfo";
import { useNearbyDevices } from "./hooks/useNearbyDevices";

export default function App() {
  const deviceInfo = useDeviceInfo();
  const nearbyDevices = useNearbyDevices();

  return (
    <main className="app-shell">
      <header className="app-header">
        <div className="brand">
          <div className="brand-mark" aria-hidden="true"><span /></div>
          <div>
            <h1>SuperDrop</h1>
            <p>Fast peer-to-peer file sharing.</p>
          </div>
        </div>
        <SettingsButton />
      </header>

      <div className="content">
        <DeviceCard {...deviceInfo} />
        <NearbyDevices {...nearbyDevices} />
      </div>

      <footer><span className="footer-dot" />Ready to connect securely on your local network</footer>
    </main>
  );
}
