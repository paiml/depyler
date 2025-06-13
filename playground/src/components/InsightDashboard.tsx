import { useState } from "react";
import { usePlaygroundStore } from "@/store";
import { EnergyGauge } from "./visualizations/EnergyGauge";
import { PerformanceChart } from "./visualizations/PerformanceChart";
import { EnergyEstimate } from "@/types";

type TabType = "output" | "performance" | "energy" | "deep-dive";

export function InsightDashboard() {
  const [activeTab, setActiveTab] = useState<TabType>("output");
  const { executionResult, transpileResult } = usePlaygroundStore();

  const tabs = [
    { id: "output" as TabType, label: "Output", icon: "📄" },
    { id: "performance" as TabType, label: "Performance", icon: "⚡" },
    { id: "energy" as TabType, label: "Energy", icon: "🔋" },
    { id: "deep-dive" as TabType, label: "Deep Dive", icon: "🔍" },
  ];

  // Calculate energy data for visualization
  const energyData = React.useMemo(() => {
    if (!executionResult || !transpileResult) {
      return {
        python: createEmptyEnergyEstimate(),
        rust: createEmptyEnergyEstimate(),
      };
    }

    // Create energy estimates based on execution times
    const pythonEnergy: EnergyEstimate = {
      joules: executionResult.python.execution_time_ms * 0.07588,
      wattsAverage: 75.88,
      co2Grams: executionResult.python.execution_time_ms * 0.07588 * 0.475,
      breakdown: {
        cpu: executionResult.python.execution_time_ms * 0.07588 * 0.8,
        memory: executionResult.python.execution_time_ms * 0.07588 * 0.2,
      },
      confidence: 0.8,
      equivalentTo: "running a laptop for a few seconds",
    };

    const rustEnergy: EnergyEstimate = {
      joules: executionResult.rust.execution_time_ms * 0.001,
      wattsAverage: 1.0,
      co2Grams: executionResult.rust.execution_time_ms * 0.001 * 0.475,
      breakdown: {
        cpu: executionResult.rust.execution_time_ms * 0.001 * 0.7,
        memory: executionResult.rust.execution_time_ms * 0.001 * 0.3,
      },
      confidence: 0.9,
      equivalentTo: "powering an LED for a few seconds",
    };

    return { python: pythonEnergy, rust: rustEnergy };
  }, [executionResult, transpileResult]);

  const energySavings = React.useMemo(() => {
    if (energyData.python.joules === 0) return 0;
    return Math.max(
      0,
      Math.min(100, (1 - energyData.rust.joules / energyData.python.joules) * 100),
    );
  }, [energyData]);

  return (
    <div className="bg-white rounded-lg shadow-sm border">
      <div className="border-b border-gray-200">
        <nav className="flex space-x-8 px-6" aria-label="Tabs">
          {tabs.map((tab) => (
            <button
              type="button"
              key={tab.id}
              onClick={() => setActiveTab(tab.id)}
              className={`
                py-4 px-1 border-b-2 font-medium text-sm transition-colors duration-200
                ${
                activeTab === tab.id
                  ? "border-blue-500 text-blue-600"
                  : "border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300"
              }
              `}
            >
              <span className="mr-2">{tab.icon}</span>
              {tab.label}
            </button>
          ))}
        </nav>
      </div>

      <div className="p-6">
        {activeTab === "output" && <OutputTab />}
        {activeTab === "performance" && <PerformanceTab />}
        {activeTab === "energy" && <EnergyTab energyData={energyData} savings={energySavings} />}
        {activeTab === "deep-dive" && <DeepDiveTab />}
      </div>
    </div>
  );
}

function OutputTab() {
  const { executionResult } = usePlaygroundStore();

  if (!executionResult) {
    return (
      <div className="text-center py-12">
        <div className="w-16 h-16 mx-auto mb-4 bg-gray-100 rounded-full flex items-center justify-center">
          <svg
            className="w-8 h-8 text-gray-400"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
            />
          </svg>
        </div>
        <p className="text-gray-500">Run your code to see the output comparison</p>
      </div>
    );
  }

  return (
    <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <div className="bg-yellow-50 rounded-lg border border-yellow-200">
        <div className="px-4 py-3 border-b border-yellow-200 bg-yellow-100 rounded-t-lg">
          <h3 className="text-lg font-semibold text-yellow-800">Python Output</h3>
          <p className="text-sm text-yellow-600">
            Executed in {executionResult.python.execution_time_ms.toFixed(2)}ms
          </p>
        </div>
        <div className="p-4">
          {executionResult.python.error
            ? (
              <pre className="text-red-600 text-sm font-mono whitespace-pre-wrap">
              {executionResult.python.error}
              </pre>
            )
            : (
              <pre className="text-yellow-900 text-sm font-mono whitespace-pre-wrap">
              {executionResult.python.output || 'No output'}
              </pre>
            )}
        </div>
      </div>

      <div className="bg-green-50 rounded-lg border border-green-200">
        <div className="px-4 py-3 border-b border-green-200 bg-green-100 rounded-t-lg">
          <h3 className="text-lg font-semibold text-green-800">Rust Output</h3>
          <p className="text-sm text-green-600">
            Executed in {executionResult.rust.execution_time_ms.toFixed(2)}ms
            {executionResult.rust.compilation_time_ms && (
              <span>(compile: {executionResult.rust.compilation_time_ms.toFixed(2)}ms)</span>
            )}
          </p>
        </div>
        <div className="p-4">
          {executionResult.rust.error
            ? (
              <pre className="text-red-600 text-sm font-mono whitespace-pre-wrap">
              {executionResult.rust.error}
              </pre>
            )
            : (
              <pre className="text-green-900 text-sm font-mono whitespace-pre-wrap">
              {executionResult.rust.output || 'No output'}
              </pre>
            )}
        </div>
      </div>
    </div>
  );
}

function PerformanceTab() {
  const { executionResult } = usePlaygroundStore();

  return (
    <div className="space-y-6">
      <PerformanceChart executionResult={executionResult} width={600} height={400} />

      {executionResult && (
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div className="bg-blue-50 rounded-lg p-4 border border-blue-200">
            <h4 className="text-sm font-semibold text-blue-800 mb-2">Speed Improvement</h4>
            <div className="text-2xl font-bold text-blue-900">
              {(executionResult.python.execution_time_ms / executionResult.rust.execution_time_ms)
                .toFixed(1)}×
            </div>
            <p className="text-sm text-blue-700">times faster</p>
          </div>

          <div className="bg-purple-50 rounded-lg p-4 border border-purple-200">
            <h4 className="text-sm font-semibold text-purple-800 mb-2">Time Saved</h4>
            <div className="text-2xl font-bold text-purple-900">
              {(executionResult.python.execution_time_ms - executionResult.rust.execution_time_ms)
                .toFixed(1)}
            </div>
            <p className="text-sm text-purple-700">milliseconds per run</p>
          </div>

          <div className="bg-indigo-50 rounded-lg p-4 border border-indigo-200">
            <h4 className="text-sm font-semibold text-indigo-800 mb-2">Efficiency</h4>
            <div className="text-2xl font-bold text-indigo-900">
              {Math.round(
                (1 -
                  executionResult.rust.execution_time_ms /
                    executionResult.python.execution_time_ms) * 100,
              )}%
            </div>
            <p className="text-sm text-indigo-700">performance gain</p>
          </div>
        </div>
      )}
    </div>
  );
}

function EnergyTab(
  { energyData, savings }: {
    energyData: { python: EnergyEstimate; rust: EnergyEstimate };
    savings: number;
  },
) {
  const confidence = (energyData.python.confidence + energyData.rust.confidence) / 2;

  return (
    <div className="space-y-6">
      <EnergyGauge
        savings={savings}
        energyData={energyData}
        confidence={confidence}
      />

      <div className="bg-gray-50 rounded-lg p-6">
        <h4 className="text-lg font-semibold text-gray-900 mb-4">Environmental Impact</h4>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div>
            <h5 className="text-sm font-semibold text-gray-700 mb-2">Per Execution</h5>
            <ul className="space-y-1 text-sm text-gray-600">
              <li>
                CO₂ reduced:{" "}
                {((energyData.python.co2Grams - energyData.rust.co2Grams) * 1000).toFixed(2)}mg
              </li>
              <li>
                Energy saved:{" "}
                {((energyData.python.joules - energyData.rust.joules) * 1000).toFixed(2)}mJ
              </li>
              <li>Equivalent to: {energyData.rust.equivalentTo}</li>
            </ul>
          </div>
          <div>
            <h5 className="text-sm font-semibold text-gray-700 mb-2">
              Scaled Impact (1M executions)
            </h5>
            <ul className="space-y-1 text-sm text-gray-600">
              <li>
                CO₂ reduced:{" "}
                {((energyData.python.co2Grams - energyData.rust.co2Grams) * 1000000).toFixed(2)}g
              </li>
              <li>
                Energy saved:{" "}
                {((energyData.python.joules - energyData.rust.joules) * 1000000 / 3600).toFixed(
                  2,
                )}Wh
              </li>
              <li>
                Tree equivalent:{" "}
                {(((energyData.python.co2Grams - energyData.rust.co2Grams) * 1000000) / 21000)
                  .toFixed(3)} trees/year
              </li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  );
}

function DeepDiveTab() {
  const { pythonCode, rustCode, transpileResult } = usePlaygroundStore();

  return (
    <div className="space-y-6">
      <div className="bg-gray-50 rounded-lg p-6">
        <h4 className="text-lg font-semibold text-gray-900 mb-4">Code Analysis</h4>

        {transpileResult && (
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <h5 className="text-sm font-semibold text-gray-700 mb-2">Quality Metrics</h5>
              <div className="space-y-2 text-sm">
                <div className="flex justify-between">
                  <span className="text-gray-600">PMAT Score:</span>
                  <span className="font-mono">
                    {transpileResult.quality_metrics.pmat_score.toFixed(2)}
                  </span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-600">Productivity:</span>
                  <span className="font-mono">
                    {transpileResult.quality_metrics.productivity.toFixed(2)}
                  </span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-600">Maintainability:</span>
                  <span className="font-mono">
                    {transpileResult.quality_metrics.maintainability.toFixed(2)}
                  </span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-600">Testability:</span>
                  <span className="font-mono">
                    {transpileResult.quality_metrics.testability.toFixed(2)}
                  </span>
                </div>
              </div>
            </div>

            <div>
              <h5 className="text-sm font-semibold text-gray-700 mb-2">Complexity Analysis</h5>
              <div className="space-y-2 text-sm">
                <div className="flex justify-between">
                  <span className="text-gray-600">Code Complexity:</span>
                  <span className="font-mono">
                    {transpileResult.quality_metrics.code_complexity}
                  </span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-600">Cyclomatic:</span>
                  <span className="font-mono">
                    {transpileResult.quality_metrics.cyclomatic_complexity}
                  </span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-600">Python Lines:</span>
                  <span className="font-mono">{pythonCode.split("\n").length}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-600">Rust Lines:</span>
                  <span className="font-mono">{rustCode.split("\n").length}</span>
                </div>
              </div>
            </div>
          </div>
        )}
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-4">
        <div className="bg-blue-50 rounded-lg p-4 border border-blue-200">
          <h5 className="text-sm font-semibold text-blue-800 mb-2">Python Analysis</h5>
          <div className="space-y-1 text-xs">
            <div>Functions: {(pythonCode.match(/def\s+\w+/g) || []).length}</div>
            <div>Loops: {(pythonCode.match(/\b(for|while)\b/g) || []).length}</div>
            <div>Conditionals: {(pythonCode.match(/\bif\b/g) || []).length}</div>
          </div>
        </div>

        <div className="bg-green-50 rounded-lg p-4 border border-green-200">
          <h5 className="text-sm font-semibold text-green-800 mb-2">Rust Analysis</h5>
          <div className="space-y-1 text-xs">
            <div>Functions: {(rustCode.match(/fn\s+\w+/g) || []).length}</div>
            <div>Loops: {(rustCode.match(/\b(for|while)\b/g) || []).length}</div>
            <div>Conditionals: {(rustCode.match(/\bif\b/g) || []).length}</div>
          </div>
        </div>

        <div className="bg-purple-50 rounded-lg p-4 border border-purple-200">
          <h5 className="text-sm font-semibold text-purple-800 mb-2">Safety Features</h5>
          <div className="space-y-1 text-xs">
            <div>Ownership: {rustCode.includes("&") ? "Borrowed" : "Owned"}</div>
            <div>Error Handling: {rustCode.includes("Result") ? "Yes" : "No"}</div>
            <div>Memory Safe: {rustCode.includes("unsafe") ? "No" : "Yes"}</div>
          </div>
        </div>
      </div>
    </div>
  );
}

function createEmptyEnergyEstimate(): EnergyEstimate {
  return {
    joules: 0,
    wattsAverage: 0,
    co2Grams: 0,
    breakdown: { cpu: 0, memory: 0 },
    confidence: 0,
    equivalentTo: "",
  };
}
