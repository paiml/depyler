import React from "react";
import ReactDOM from "react-dom/client";
import { App } from "./components/App";
import { qualityMonitor } from "./lib/quality-monitor";
import { telemetry } from "./lib/telemetry/quality-telemetry";
import "./index.css";

// Performance monitoring
qualityMonitor.recordWasmLoadStart();

// Initialize telemetry
telemetry.recordUserInteraction("page_load", {
  url: globalThis.location.href,
  referrer: document.referrer,
  timestamp: Date.now(),
});

// Create root and render app
const root = ReactDOM.createRoot(
  document.getElementById("root") as HTMLElement,
);

root.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);

// Record when WASM finishes loading
qualityMonitor.recordWasmLoadEnd();
