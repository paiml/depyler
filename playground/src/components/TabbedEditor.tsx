import { useState } from "react";
import { CodeEditor } from "./CodeEditor";
import { usePlaygroundStore } from "@/store";

type TabType = "python" | "rust" | "output" | "errors";

interface Tab {
  id: TabType;
  label: string;
  icon?: string;
}

const tabs: Tab[] = [
  { id: "python", label: "Python", icon: "🐍" },
  { id: "rust", label: "Rust", icon: "🦀" },
  { id: "output", label: "Output", icon: "▶" },
  { id: "errors", label: "Errors", icon: "⚠️" },
];

export function TabbedEditor() {
  const [activeTab, setActiveTab] = useState<TabType>("python");
  const {
    pythonCode,
    rustCode,
    setPythonCode,
    isTranspiling,
    executionResult,
    errors,
    warnings,
  } = usePlaygroundStore();

  const hasErrors = errors.length > 0;
  const hasWarnings = warnings.length > 0;

  const renderTabContent = () => {
    switch (activeTab) {
      case "python":
        return (
          <CodeEditor
            language="python"
            value={pythonCode}
            onChange={setPythonCode}
            height="500px"
            loading={isTranspiling}
          />
        );

      case "rust":
        return (
          <CodeEditor
            language="rust"
            value={rustCode}
            onChange={() => {}}
            height="500px"
            readOnly
            loading={isTranspiling}
          />
        );

      case "output":
        return (
          <div className="h-[500px] overflow-auto bg-gray-900 text-gray-100 p-4 font-mono text-sm">
            {executionResult ? (
              <div>
                <div className="mb-4">
                  <h3 className="text-green-400 font-semibold mb-2">Python Output:</h3>
                  <pre className="whitespace-pre-wrap">{executionResult.pythonOutput}</pre>
                  <div className="text-gray-500 text-xs mt-1">
                    Time: {executionResult.pythonTime.toFixed(2)}ms
                  </div>
                </div>
                
                <div className="border-t border-gray-700 pt-4">
                  <h3 className="text-blue-400 font-semibold mb-2">Rust Output:</h3>
                  <pre className="whitespace-pre-wrap">{executionResult.rustOutput}</pre>
                  <div className="text-gray-500 text-xs mt-1">
                    Time: {executionResult.rustTime.toFixed(2)}ms
                  </div>
                </div>

                <div className="border-t border-gray-700 pt-4 mt-4">
                  <h3 className="text-yellow-400 font-semibold mb-2">Performance Comparison:</h3>
                  <div className="text-sm">
                    <div>Speedup: {executionResult.speedup.toFixed(2)}x faster</div>
                    <div>Memory saved: {executionResult.memorySaved.toFixed(2)} MB</div>
                  </div>
                </div>
              </div>
            ) : (
              <div className="text-gray-500">No output yet. Click "Run" to execute the code.</div>
            )}
          </div>
        );

      case "errors":
        return (
          <div className="h-[500px] overflow-auto bg-gray-900 text-gray-100 p-4 font-mono text-sm">
            {hasErrors || hasWarnings ? (
              <div>
                {hasErrors && (
                  <div className="mb-4">
                    <h3 className="text-red-400 font-semibold mb-2">Errors:</h3>
                    {errors.map((error, index) => (
                      <div key={index} className="mb-2 text-red-300">
                        <span className="text-red-500">✗</span> {error}
                      </div>
                    ))}
                  </div>
                )}
                
                {hasWarnings && (
                  <div>
                    <h3 className="text-yellow-400 font-semibold mb-2">Warnings:</h3>
                    {warnings.map((warning, index) => (
                      <div key={index} className="mb-2 text-yellow-300">
                        <span className="text-yellow-500">⚠</span> {warning}
                      </div>
                    ))}
                  </div>
                )}
              </div>
            ) : (
              <div className="text-gray-500">No errors or warnings.</div>
            )}
          </div>
        );
    }
  };

  return (
    <div className="bg-white rounded-lg shadow-sm border">
      {/* Tab Navigation */}
      <div className="flex border-b" role="tablist">
        {tabs.map((tab) => {
          const isActive = activeTab === tab.id;
          const hasContent = 
            (tab.id === "errors" && (hasErrors || hasWarnings)) ||
            (tab.id === "output" && executionResult) ||
            (tab.id === "rust" && rustCode);

          return (
            <button
              type="button"
              key={tab.id}
              role="tab"
              aria-selected={isActive}
              aria-controls={`tabpanel-${tab.id}`}
              onClick={() => setActiveTab(tab.id)}
              className={`
                px-4 py-3 text-sm font-medium transition-colors relative
                ${isActive 
                  ? "text-blue-600 border-b-2 border-blue-600 bg-blue-50" 
                  : "text-gray-600 hover:text-gray-900 hover:bg-gray-50"
                }
              `}
            >
              <span className="flex items-center space-x-2">
                {tab.icon && <span>{tab.icon}</span>}
                <span>{tab.label}</span>
                {hasContent && !isActive && (
                  <span className="w-2 h-2 bg-blue-400 rounded-full ml-2"></span>
                )}
                {tab.id === "errors" && hasErrors && (
                  <span className="ml-2 px-1.5 py-0.5 text-xs bg-red-100 text-red-700 rounded">
                    {errors.length}
                  </span>
                )}
              </span>
            </button>
          );
        })}
      </div>

      {/* Tab Content */}
      <div
        id={`tabpanel-${activeTab}`}
        role="tabpanel"
        aria-labelledby={`tab-${activeTab}`}
      >
        {renderTabContent()}
      </div>
      
      {/* Live regions for screen readers */}
      <div className="sr-only" role="status" aria-live="polite" aria-atomic="true">
        {isTranspiling && "Transpiling Python code to Rust..."}
        {hasErrors && `${errors.length} errors found`}
      </div>
    </div>
  );
}