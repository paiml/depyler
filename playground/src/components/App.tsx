import React, { useEffect } from 'react';
import { CodeEditor } from './CodeEditor';
import { ExecutionButton } from './ExecutionButton';
import { InsightDashboard } from './InsightDashboard';
import { usePlaygroundStore } from '@/store';
import { preloadWasm } from '@/lib/wasm-manager';

export function App() {
  const { pythonCode, rustCode, isTranspiling, isExecuting, errors = [], setPythonCode } = usePlaygroundStore();

  useEffect(() => {
    // Preload WASM module for better performance
    preloadWasm().catch(console.warn);
  }, []);

  return (
    <div className="min-h-screen bg-gray-50" data-testid="playground-container">
      <header className="bg-white shadow-sm border-b">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4">
          <div className="flex items-center justify-between">
            <div>
              <h1 className="text-2xl font-bold text-gray-900">
                Depyler Interactive Playground
              </h1>
              <p className="text-sm text-gray-600">
                Python-to-Rust transpiler with energy efficiency analysis
              </p>
            </div>
            <ExecutionButton />
          </div>
        </div>
      </header>

      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
        {errors.length > 0 && (
          <div className="mb-6 bg-red-50 border border-red-200 rounded-md p-4">
            <h3 className="text-sm font-medium text-red-800">Errors:</h3>
            <ul className="mt-2 text-sm text-red-700">
              {errors.map((error, index) => (
                <li key={index} className="mt-1">
                  {error}
                </li>
              ))}
            </ul>
          </div>
        )}

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
          <div className="bg-white rounded-lg shadow-sm border" data-testid="editor-panel">
            <div className="px-4 py-3 border-b bg-gray-50 rounded-t-lg">
              <h2 className="text-lg font-semibold text-gray-900">Python Code</h2>
              <p className="text-sm text-gray-600">
                Write Python code with @depyler annotations for optimization hints
              </p>
            </div>
            <div className="p-4">
              <CodeEditor
                language="python"
                value={pythonCode}
                onChange={setPythonCode}
                height="400px"
                loading={isTranspiling}
              />
            </div>
          </div>

          <div className="bg-white rounded-lg shadow-sm border" data-testid="results-panel">
            <div className="px-4 py-3 border-b bg-gray-50 rounded-t-lg">
              <h2 className="text-lg font-semibold text-gray-900">Generated Rust</h2>
              <p className="text-sm text-gray-600">
                Automatically transpiled Rust code with safety guarantees
              </p>
            </div>
            <div className="p-4">
              <CodeEditor
                language="rust"
                value={rustCode}
                onChange={() => {}} // Read-only
                height="400px"
                readOnly
                loading={isTranspiling}
              />
            </div>
          </div>
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