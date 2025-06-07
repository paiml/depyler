import { describe, expect, it, vi } from "vitest";
import { act, renderHook } from "@testing-library/react";
import { usePlaygroundStore } from "../index";

// Mock WASM manager to prevent dynamic imports
vi.mock("@/lib/wasm-manager", () => ({
  transpileCode: vi.fn(() => Promise.resolve({
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
  })),
}));

// Mock execution manager
vi.mock("@/lib/execution-manager", () => ({
  executeComparison: vi.fn(() => Promise.resolve({
    python: { stdout: "test", stderr: "", executionTimeMs: 10, memoryUsageMb: 1 },
    rust: { stdout: "test", stderr: "", executionTimeMs: 5, memoryUsageMb: 0.5 },
    performance: { speedup: 2.0, memoryReduction: 0.5 },
    energy: { joules: 0.001, wattsAverage: 1.0, co2Grams: 0.0005 }
  })),
}));

describe("usePlaygroundStore Basic Tests", () => {
  it("store hook returns valid functions", () => {
    const { result } = renderHook(() => usePlaygroundStore());
    
    expect(result.current).toBeTruthy();
    expect(typeof result.current.setPythonCode).toBe('function');
    expect(typeof result.current.setRustCode).toBe('function');
    expect(typeof result.current.transpileCode).toBe('function');
    expect(typeof result.current.executeCode).toBe('function');
    expect(typeof result.current.clearErrors).toBe('function');
    expect(typeof result.current.reset).toBe('function');
  });

  it("can update Python code", () => {
    const { result } = renderHook(() => usePlaygroundStore());
    
    act(() => {
      result.current.setPythonCode("def hello(): pass");
    });
    
    expect(result.current.pythonCode).toBe("def hello(): pass");
  });

  it("can update Rust code", () => {
    const { result } = renderHook(() => usePlaygroundStore());
    
    act(() => {
      result.current.setRustCode("fn hello() {}");
    });
    
    expect(result.current.rustCode).toBe("fn hello() {}");
  });

  it("has initial arrays for errors and warnings", () => {
    const { result } = renderHook(() => usePlaygroundStore());
    
    expect(Array.isArray(result.current.errors)).toBe(true);
    expect(Array.isArray(result.current.warnings)).toBe(true);
  });

  it("can clear errors", () => {
    const { result } = renderHook(() => usePlaygroundStore());
    
    act(() => {
      result.current.clearErrors();
    });
    
    expect(result.current.errors).toEqual([]);
    expect(result.current.warnings).toEqual([]);
  });
});