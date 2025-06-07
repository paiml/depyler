import React, { useEffect } from "react";
import { TabbedEditor } from "./TabbedEditor";
import { ExecutionButton } from "./ExecutionButton";
import { InsightDashboard } from "./InsightDashboard";
import { SettingsDropdown } from "./SettingsDropdown";
import { ExampleSelector } from "./ExampleSelector";
import { usePlaygroundStore } from "@/store";
import { preloadWasm } from "@/lib/wasm-manager";

export function App() {
  const { errors = [] } = usePlaygroundStore();

  useEffect(() => {
    // Preload WASM module for better performance
    preloadWasm().catch(console.warn);
  }, []);

  return (
    <div className="min-h-screen bg-gray-50" data-testid="playground-container">
      <header className="bg-white shadow-sm border-b">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-3">
          <div className="flex items-center justify-between">
            <div className="flex items-center space-x-6">
              <div>
                <h1 className="text-xl font-bold text-gray-900">
                  Depyler Playground
                </h1>
                <p className="text-xs text-gray-600">
                  Python to Rust transpiler
                </p>
              </div>
              <div className="flex items-center space-x-2">
                <ExampleSelector />
                <SettingsDropdown />
              </div>
            </div>
            <ExecutionButton />
          </div>
        </div>
      </header>

      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6" role="main" aria-label="Depyler Playground">
        <div className="mb-6">
          <TabbedEditor />
        </div>

        <InsightDashboard />
      </main>

      <footer className="bg-white border-t mt-12">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
          <div className="flex items-center justify-between text-sm text-gray-600">
            <div>
              <p>
                Powered by <strong>Depyler</strong> - Zero-configuration Python to Rust transpiler
              </p>
            </div>
            <div className="flex space-x-4">
              <a
                href="https://github.com/paiml/depyler"
                target="_blank"
                rel="noopener noreferrer"
                className="hover:text-gray-900"
              >
                GitHub
              </a>
              <a
                href="/docs"
                className="hover:text-gray-900"
              >
                Documentation
              </a>
            </div>
          </div>
        </div>
      </footer>
    </div>
  );
}
