import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { InsightDashboard } from "../InsightDashboard";
import { createMockPlaygroundStore } from "@test/setup";

// Mock the store
const mockStore = createMockPlaygroundStore();
vi.mock("@/store", () => ({
  usePlaygroundStore: vi.fn(() => mockStore),
}));

// Mock the visualization components
vi.mock("../visualizations/EnergyGauge", () => ({
  EnergyGauge: ({ savings }: { savings: number }) => (
    <div data-testid="energy-gauge">Energy Savings: {savings}%</div>
  ),
}));

vi.mock("../visualizations/PerformanceChart", () => ({
  PerformanceChart: ({ _data }: any) => (
    <div data-testid="performance-chart">Performance Chart</div>
  ),
}));

describe("InsightDashboard", () => {
  it("shows placeholder when no transpilation result", () => {
    mockStore.transpileResult = null;
    render(<InsightDashboard />);
    
    expect(screen.getByText(/Run transpilation to see insights/)).toBeInTheDocument();
  });

  it("displays energy savings", () => {
    mockStore.transpileResult = {
      success: true,
      rust_code: "fn test() {}",
      errors: [],
      warnings: [],
      transpile_time_ms: 50,
      memory_usage_mb: 1.2,
      energy_estimate: {
        joules: 0.5,
        wattsAverage: 2.1,
        co2Grams: 0.1,
        breakdown: { cpu: 0.35, memory: 0.15 },
        confidence: 0.9,
        equivalentTo: "powering an LED for 1 second",
      },
      quality_metrics: {
        pmat_score: 0.85,
        productivity: 0.8,
        maintainability: 0.9,
        accessibility: 1.0,
        testability: 0.8,
        code_complexity: 2,
        cyclomatic_complexity: 3,
      },
    };
    
    render(<InsightDashboard />);
    
    expect(screen.getByTestId("energy-gauge")).toBeInTheDocument();
    expect(screen.getByText("Energy Efficiency")).toBeInTheDocument();
  });

  it("displays quality metrics", () => {
    mockStore.transpileResult = {
      success: true,
      rust_code: "fn test() {}",
      errors: [],
      warnings: [],
      transpile_time_ms: 50,
      memory_usage_mb: 1.2,
      energy_estimate: {
        joules: 0.5,
        wattsAverage: 2.1,
        co2Grams: 0.1,
        breakdown: { cpu: 0.35, memory: 0.15 },
        confidence: 0.9,
        equivalentTo: "powering an LED for 1 second",
      },
      quality_metrics: {
        pmat_score: 0.85,
        productivity: 0.8,
        maintainability: 0.9,
        accessibility: 1.0,
        testability: 0.8,
        code_complexity: 2,
        cyclomatic_complexity: 3,
      },
    };
    
    render(<InsightDashboard />);
    
    expect(screen.getByText("Quality Metrics")).toBeInTheDocument();
    expect(screen.getByText(/PMAT Score/)).toBeInTheDocument();
    expect(screen.getByText(/85/)).toBeInTheDocument(); // PMAT score * 100
  });

  it("shows performance metrics when execution result exists", () => {
    mockStore.executionResult = {
      python: {
        output: "Hello",
        error: null,
        execution_time_ms: 100,
      },
      rust: {
        output: "Hello",
        error: null,
        execution_time_ms: 10,
        compilation_time_ms: 5,
      },
      energy_savings_percent: 90,
    };
    
    render(<InsightDashboard />);
    
    expect(screen.getByTestId("performance-chart")).toBeInTheDocument();
  });

  it("shows energy equivalent description", () => {
    mockStore.transpileResult = {
      success: true,
      rust_code: "fn test() {}",
      errors: [],
      warnings: [],
      transpile_time_ms: 50,
      memory_usage_mb: 1.2,
      energy_estimate: {
        joules: 0.5,
        wattsAverage: 2.1,
        co2Grams: 0.1,
        breakdown: { cpu: 0.35, memory: 0.15 },
        confidence: 0.9,
        equivalentTo: "powering an LED for 1 second",
      },
      quality_metrics: {
        pmat_score: 0.85,
        productivity: 0.8,
        maintainability: 0.9,
        accessibility: 1.0,
        testability: 0.8,
        code_complexity: 2,
        cyclomatic_complexity: 3,
      },
    };
    
    render(<InsightDashboard />);
    
    expect(screen.getByText(/powering an LED for 1 second/)).toBeInTheDocument();
  });

  it("displays confidence indicator", () => {
    mockStore.transpileResult = {
      success: true,
      rust_code: "fn test() {}",
      errors: [],
      warnings: [],
      transpile_time_ms: 50,
      memory_usage_mb: 1.2,
      energy_estimate: {
        joules: 0.5,
        wattsAverage: 2.1,
        co2Grams: 0.1,
        breakdown: { cpu: 0.35, memory: 0.15 },
        confidence: 0.9,
        equivalentTo: "powering an LED for 1 second",
      },
      quality_metrics: {
        pmat_score: 0.85,
        productivity: 0.8,
        maintainability: 0.9,
        accessibility: 1.0,
        testability: 0.8,
        code_complexity: 2,
        cyclomatic_complexity: 3,
      },
    };
    
    render(<InsightDashboard />);
    
    expect(screen.getByText(/Confidence.*90%/)).toBeInTheDocument();
  });

  it("handles missing quality metrics gracefully", () => {
    mockStore.transpileResult = {
      success: true,
      rust_code: "fn test() {}",
      errors: [],
      warnings: [],
      transpile_time_ms: 50,
      memory_usage_mb: 1.2,
      energy_estimate: {
        joules: 0.5,
        wattsAverage: 2.1,
        co2Grams: 0.1,
        breakdown: { cpu: 0.35, memory: 0.15 },
        confidence: 0.9,
        equivalentTo: "powering an LED for 1 second",
      },
      quality_metrics: undefined as any,
    };
    
    render(<InsightDashboard />);
    
    expect(screen.queryByText("Quality Metrics")).not.toBeInTheDocument();
  });

  it("formats CO2 emissions correctly", () => {
    mockStore.transpileResult = {
      success: true,
      rust_code: "fn test() {}",
      errors: [],
      warnings: [],
      transpile_time_ms: 50,
      memory_usage_mb: 1.2,
      energy_estimate: {
        joules: 0.5,
        wattsAverage: 2.1,
        co2Grams: 0.123456,
        breakdown: { cpu: 0.35, memory: 0.15 },
        confidence: 0.9,
        equivalentTo: "powering an LED for 1 second",
      },
      quality_metrics: {
        pmat_score: 0.85,
        productivity: 0.8,
        maintainability: 0.9,
        accessibility: 1.0,
        testability: 0.8,
        code_complexity: 2,
        cyclomatic_complexity: 3,
      },
    };
    
    render(<InsightDashboard />);
    
    expect(screen.getByText(/CO2.*0\.12.*g/)).toBeInTheDocument();
  });
});