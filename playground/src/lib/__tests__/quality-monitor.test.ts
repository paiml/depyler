import { describe, expect, it, vi, beforeEach, afterEach } from "vitest";
import { QualityMonitor } from "../quality-monitor";
import { QualityTelemetry } from "../telemetry/quality-telemetry";

// Mock QualityTelemetry
vi.mock("../telemetry/quality-telemetry", () => ({
  QualityTelemetry: vi.fn().mockImplementation(() => ({
    track: vi.fn(),
    trackPageLoad: vi.fn(),
    trackTranspilation: vi.fn(),
    trackExecution: vi.fn(),
    trackQualityEvent: vi.fn(),
    flush: vi.fn(() => Promise.resolve()),
  })),
}));

describe("QualityMonitor", () => {
  let monitor: QualityMonitor;
  let mockTelemetry: any;

  beforeEach(() => {
    vi.clearAllMocks();
    monitor = new QualityMonitor();
    mockTelemetry = (monitor as any).telemetry;
  });

  describe("Page Load Monitoring", () => {
    it("tracks page load metrics", () => {
      const metrics = {
        ttfmp_ms: 1200,
        tti_ms: 2500,
        wasm_load_ms: 800,
        wasm_size_kb: 450,
      };

      monitor.trackPageLoad(metrics);

      expect(mockTelemetry.trackPageLoad).toHaveBeenCalledWith(metrics);
    });

    it("validates page load thresholds", () => {
      const metrics = {
        ttfmp_ms: 3000, // Above threshold
        tti_ms: 2500,
        wasm_load_ms: 800,
        wasm_size_kb: 450,
      };

      monitor.trackPageLoad(metrics);

      // Should track quality event for slow page load
      expect(mockTelemetry.trackQualityEvent).toHaveBeenCalledWith(
        expect.objectContaining({
          event_type: "PerformanceRegression",
          severity: "Warning",
        })
      );
    });
  });

  describe("Transpilation Monitoring", () => {
    it("tracks successful transpilation", () => {
      const result = {
        success: true,
        rust_code: "fn main() {}",
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

      monitor.trackTranspilation("def test(): pass", result);

      expect(mockTelemetry.trackTranspilation).toHaveBeenCalledWith({
        latency_p95_ms: result.transpile_time_ms,
        complexity_bucket: "Simple",
        cache_hit_rate: 0,
        error_rate: 0,
      });
    });

    it("categorizes complexity correctly", () => {
      const simpleResult = {
        success: true,
        rust_code: "fn main() {}",
        errors: [],
        warnings: [],
        transpile_time_ms: 50,
        memory_usage_mb: 1.2,
        energy_estimate: null,
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

      monitor.trackTranspilation("def simple(): pass", simpleResult);

      expect(mockTelemetry.trackTranspilation).toHaveBeenCalledWith(
        expect.objectContaining({
          complexity_bucket: "Simple",
        })
      );

      const complexResult = {
        ...simpleResult,
        quality_metrics: {
          ...simpleResult.quality_metrics,
          cyclomatic_complexity: 15,
        },
      };

      monitor.trackTranspilation("def complex(): pass", complexResult);

      expect(mockTelemetry.trackTranspilation).toHaveBeenLastCalledWith(
        expect.objectContaining({
          complexity_bucket: "Complex",
        })
      );
    });

    it("tracks transpilation errors", () => {
      const errorResult = {
        success: false,
        rust_code: "",
        errors: ["SyntaxError: invalid syntax"],
        warnings: [],
        transpile_time_ms: 10,
        memory_usage_mb: 0.5,
        energy_estimate: null,
        quality_metrics: null,
      };

      monitor.trackTranspilation("def invalid syntax", errorResult);

      expect(mockTelemetry.trackTranspilation).toHaveBeenCalledWith(
        expect.objectContaining({
          error_rate: 1,
        })
      );
    });
  });

  describe("Execution Monitoring", () => {
    it("tracks code execution metrics", () => {
      const executionResult = {
        python: {
          output: "Hello",
          error: null,
          execution_time_ms: 100,
        },
        rust: {
          output: "Hello",
          error: null,
          execution_time_ms: 10,
          compilation_time_ms: 50,
        },
        energy_savings_percent: 90,
      };

      monitor.trackExecution("def test(): pass", "fn main() {}", executionResult);

      expect(mockTelemetry.trackExecution).toHaveBeenCalledWith({
        rust_execution_ms: 10,
        python_execution_ms: 100,
        energy_savings_percent: 90,
        memory_usage_mb: expect.any(Number),
      });
    });

    it("tracks performance improvements", () => {
      const executionResult = {
        python: {
          output: "Result",
          error: null,
          execution_time_ms: 1000,
        },
        rust: {
          output: "Result",
          error: null,
          execution_time_ms: 10,
          compilation_time_ms: 50,
        },
        energy_savings_percent: 99,
      };

      monitor.trackExecution("def slow(): pass", "fn fast() {}", executionResult);

      expect(mockTelemetry.trackQualityEvent).toHaveBeenCalledWith(
        expect.objectContaining({
          event_type: "PerformanceImprovement",
          message: expect.stringContaining("100.0x speedup"),
        })
      );
    });

    it("tracks energy efficiency improvements", () => {
      const executionResult = {
        python: {
          output: "Result",
          error: null,
          execution_time_ms: 100,
        },
        rust: {
          output: "Result",
          error: null,
          execution_time_ms: 10,
          compilation_time_ms: 5,
        },
        energy_savings_percent: 95,
      };

      monitor.trackExecution("def test(): pass", "fn main() {}", executionResult);

      expect(mockTelemetry.trackQualityEvent).toHaveBeenCalledWith(
        expect.objectContaining({
          event_type: "EnergyEfficiencyImprovement",
          message: expect.stringContaining("95% energy savings"),
        })
      );
    });
  });

  describe("PMAT Score Calculation", () => {
    it("calculates PMAT score from transpilation result", () => {
      const result = {
        success: true,
        rust_code: "fn main() {}",
        errors: [],
        warnings: [],
        transpile_time_ms: 50,
        memory_usage_mb: 1.2,
        energy_estimate: null,
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

      const score = monitor.calculatePmatScore(result);

      expect(score.productivity).toBe(0.8);
      expect(score.maintainability).toBe(0.9);
      expect(score.accessibility).toBe(1.0);
      expect(score.testability).toBe(0.8);
      expect(score.tdg).toBeCloseTo(0.125, 2); // (1 - 0.85) * 0.85 ≈ 0.1275
    });

    it("handles missing quality metrics", () => {
      const result = {
        success: true,
        rust_code: "fn main() {}",
        errors: [],
        warnings: [],
        transpile_time_ms: 50,
        memory_usage_mb: 1.2,
        energy_estimate: null,
        quality_metrics: null,
      };

      const score = monitor.calculatePmatScore(result);

      expect(score.productivity).toBe(0.5);
      expect(score.maintainability).toBe(0.5);
      expect(score.accessibility).toBe(0.5);
      expect(score.testability).toBe(0.5);
      expect(score.tdg).toBe(0.5);
    });
  });

  describe("Threshold Monitoring", () => {
    it("detects error threshold exceeded", () => {
      // Simulate multiple errors
      for (let i = 0; i < 10; i++) {
        const errorResult = {
          success: false,
          rust_code: "",
          errors: [`Error ${i}`],
          warnings: [],
          transpile_time_ms: 10,
          memory_usage_mb: 0.5,
          energy_estimate: null,
          quality_metrics: null,
        };
        
        monitor.trackTranspilation(`def error${i}(): pass`, errorResult);
      }

      // Should eventually trigger error threshold event
      expect(mockTelemetry.trackQualityEvent).toHaveBeenCalledWith(
        expect.objectContaining({
          event_type: "ErrorThresholdExceeded",
          severity: "Critical",
        })
      );
    });

    it("monitors cache efficiency", () => {
      // Track many transpilations to build cache stats
      const result = {
        success: true,
        rust_code: "fn main() {}",
        errors: [],
        warnings: [],
        transpile_time_ms: 50,
        memory_usage_mb: 1.2,
        energy_estimate: null,
        quality_metrics: null,
      };

      for (let i = 0; i < 20; i++) {
        monitor.trackTranspilation(`def test${i}(): pass`, result);
      }

      // Cache hit rate should be tracked
      const lastCall = mockTelemetry.trackTranspilation.mock.calls[
        mockTelemetry.trackTranspilation.mock.calls.length - 1
      ][0];
      
      expect(lastCall).toHaveProperty("cache_hit_rate");
    });
  });

  describe("Export and Analysis", () => {
    it("exports metrics for analysis", () => {
      const metrics = monitor.exportMetrics();

      expect(metrics).toHaveProperty("page_load");
      expect(metrics).toHaveProperty("transpilation");
      expect(metrics).toHaveProperty("execution");
      expect(metrics).toHaveProperty("quality_events");
    });

    it("flushes telemetry data", async () => {
      await monitor.flush();

      expect(mockTelemetry.flush).toHaveBeenCalled();
    });
  });

  describe("Performance Tracking", () => {
    it("tracks P95 latency correctly", () => {
      // Track multiple transpilations with varying times
      const times = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
      
      times.forEach((time, i) => {
        const result = {
          success: true,
          rust_code: "fn main() {}",
          errors: [],
          warnings: [],
          transpile_time_ms: time,
          memory_usage_mb: 1.2,
          energy_estimate: null,
          quality_metrics: null,
        };
        
        monitor.trackTranspilation(`def test${i}(): pass`, result);
      });

      // P95 should be tracked (95th percentile of times)
      const lastCall = mockTelemetry.trackTranspilation.mock.calls[
        mockTelemetry.trackTranspilation.mock.calls.length - 1
      ][0];
      
      expect(lastCall.latency_p95_ms).toBeGreaterThanOrEqual(90);
      expect(lastCall.latency_p95_ms).toBeLessThanOrEqual(100);
    });
  });
});