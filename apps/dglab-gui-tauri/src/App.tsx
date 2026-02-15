import { BrowserRouter as Router, Routes, Route, Navigate } from "react-router-dom";
import { Toaster } from "sonner";
import { Dashboard } from "./pages/Dashboard";
import { DeviceScanner } from "./pages/DeviceScanner";
import { PowerControl } from "./pages/PowerControl";
import { WaveformGenerator } from "./pages/WaveformGenerator";
import { PresetManager } from "./pages/PresetManager";
import { useAppStore } from "./stores/appStore";
import { useDeviceEvents } from "./hooks/useDeviceEvents";
import { useEffect } from "react";

function App() {
  const { theme } = useAppStore();

  // Setup device event listeners
  useDeviceEvents();

  // Apply theme to document
  useEffect(() => {
    const root = window.document.documentElement;
    root.classList.remove("light", "dark");
    root.classList.add(theme);
  }, [theme]);

  return (
    <Router>
      <div className="min-h-screen bg-background text-foreground">
        <Routes>
          <Route path="/" element={<Dashboard />} />
          <Route path="/scanner" element={<DeviceScanner />} />
          <Route path="/control" element={<PowerControl />} />
          <Route path="/waveform" element={<WaveformGenerator />} />
          <Route path="/presets" element={<PresetManager />} />
          <Route path="*" element={<Navigate to="/" replace />} />
        </Routes>
        <Toaster 
          position="top-right" 
          theme={theme as "light" | "dark"}
          richColors
          closeButton
        />
      </div>
    </Router>
  );
}

export default App;
