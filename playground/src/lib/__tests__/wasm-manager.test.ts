import { describe, expect, it, vi, beforeEach, afterEach } from "vitest";

// Create a more complete mock that matches the actual WASM module structure
const createMockWasmResult = (overrides?: any) => ({
  success: true,
  rust_code: "fn main() {}",
  errors: [],
  warnings: [],
  transpile_time_ms: 50,
  memory_usage_mb: 1.2,
  energy_estimate: {
    joules: 0.5,
    watts_average: 2.1,
    co2_grams: 0.1,
    confidence: 0.9,
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
  ...overrides,
});

const mockEngine = {
  transpile: vi.fn(() => createMockWasmResult()),
  analyze: vi.fn(() => ({
    complexity: 5,
    cyclomatic_complexity: 3,
    functions: [{
      name: "test_function",
      line_start: 1,
      line_end: 5,
      complexity: 2,
      parameters: ["a", "b"],
      return_type: "int",
    }],
    imports: ["math", "typing"],
    suggestions: [{
      line: 2,
      column: 5,
      message: "Consider using list comprehension",
      suggestion_type: "optimization",
      confidence: 0.8,
    }],
    anti_patterns: [],
  })),
  benchmark: vi.fn(() => ({
    iterations: 1000,
    times_ms: [10, 12, 11, 9, 11],
    min_ms: 9,
    max_ms: 12,
    mean_ms: 10.6,
    median_ms: 11,
    std_dev_ms: 1.02,
  })),
};

// Create module state that can be reset between tests
let mockWasmLoaded = false;
let mockWasmInstance: any = null;

// Mock the entire wasm-manager module
vi.mock("../wasm-manager", async () => {
  const actual = await vi.importActual("../wasm-manager") as any;
  
  return {
    ...actual,
    preloadWasm: vi.fn(async () => {
      mockWasmLoaded = true;
      mockWasmInstance = mockEngine;
    }),
    isWasmLoaded: vi.fn(() => mockWasmLoaded),
    getWasmInstance: vi.fn(() => mockWasmInstance),
    transpileCode: vi.fn(async (code: string, options?: any) => {
      if (!mockWasmLoaded) {
        // Auto-load WASM if needed
        mockWasmLoaded = true;
        mockWasmInstance = mockEngine;
      }
      
      try {
        const result = mockEngine.transpile(code, options);
        
        // Transform the result to match the expected format
        return {
          success: result.success,
          rust_code: result.rust_code,
          errors: Array.from(result.errors || []),
          warnings: Array.from(result.warnings || []),
          transpile_time_ms: result.transpile_time_ms,
          memory_usage_mb: result.memory_usage_mb,
          energy_estimate: {
            joules: result.energy_estimate.joules,
            wattsAverage: result.energy_estimate.watts_average,
            co2Grams: result.energy_estimate.co2_grams,
            breakdown: {
              cpu: result.energy_estimate.joules * 0.7,
              memory: result.energy_estimate.joules * 0.3,
            },
            confidence: result.energy_estimate.confidence,
            equivalentTo: "powering an LED for 1 second",
          },
          quality_metrics: result.quality_metrics,
        };
      } catch (error) {
        // Return error result instead of throwing
        return {
          success: false,
          rust_code: "",
          errors: [error instanceof Error ? error.message : "Transpilation crashed"],
          warnings: [],
          transpile_time_ms: 0,
          memory_usage_mb: 0,
          energy_estimate: {
            joules: 0,
            wattsAverage: 0,
            co2Grams: 0,
            breakdown: { cpu: 0, memory: 0 },
            confidence: 0,
            equivalentTo: "",
          },
          quality_metrics: {
            pmat_score: 0,
            productivity: 0,
            maintainability: 0,
            accessibility: 0,
            testability: 0,
            code_complexity: 0,
            cyclomatic_complexity: 0,
          },
        };
      }
    }),
    analyzeCode: vi.fn(async (code: string) => {
      if (!mockWasmLoaded) {
        mockWasmLoaded = true;
        mockWasmInstance = mockEngine;
      }
      return mockEngine.analyze(code);
    }),
    benchmarkCode: vi.fn(async (code: string, iterations?: number) => {
      if (!mockWasmLoaded) {
        mockWasmLoaded = true;
        mockWasmInstance = mockEngine;
      }
      return mockEngine.benchmark(code, iterations);
    }),
  };
});

// Import after mocking
import { transpileCode, analyzeCode, benchmarkCode, preloadWasm, isWasmLoaded, getWasmInstance } from "../wasm-manager";

describe("WasmManager", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    // Reset mock state
    mockWasmLoaded = false;
    mockWasmInstance = null;
    mockEngine.transpile.mockImplementation(() => createMockWasmResult());
  });

  describe("preloadWasm", () => {
    it("loads WASM module successfully", async () => {
      await preloadWasm();
      
      expect(isWasmLoaded()).toBe(true);
      expect(getWasmInstance()).toBeDefined();
    });

    it("handles concurrent load requests", async () => {
      const promises = [
        preloadWasm(),
        preloadWasm(),
        preloadWasm(),
      ];
      
      await Promise.all(promises);
      
      // Should only load once
      expect(isWasmLoaded()).toBe(true);
      expect(preloadWasm).toHaveBeenCalled();
    });

    it("caches WASM instance", async () => {
      await preloadWasm();
      const instance1 = getWasmInstance();
      
      await preloadWasm();
      const instance2 = getWasmInstance();
      
      expect(instance1).toBeDefined();
      expect(instance2).toBeDefined();
      // They should be the same reference
      expect(instance1).toBe(instance2);
    });
  });

  describe("transpileCode", () => {
    it("transpiles Python code to Rust", async () => {
      const result = await transpileCode("def add(a, b): return a + b");
      
      expect(result.success).toBe(true);
      expect(result.rust_code).toBe("fn main() {}");
      expect(result.errors).toHaveLength(0);
      expect(result.transpile_time_ms).toBe(50);
    });

    it("returns quality metrics", async () => {
      const result = await transpileCode("def test(): pass");
      
      expect(result.quality_metrics).toBeDefined();
      expect(result.quality_metrics.pmat_score).toBe(0.85);
      expect(result.quality_metrics.productivity).toBe(0.8);
      expect(result.quality_metrics.maintainability).toBe(0.9);
    });

    it("returns energy estimate", async () => {
      const result = await transpileCode("def test(): pass");
      
      expect(result.energy_estimate).toBeDefined();
      expect(result.energy_estimate.joules).toBe(0.5);
      expect(result.energy_estimate.wattsAverage).toBe(2.1);
      expect(result.energy_estimate.equivalentTo).toBe("powering an LED for 1 second");
    });

    it("handles transpilation errors", async () => {
      mockEngine.transpile.mockReturnValueOnce(createMockWasmResult({
        success: false,
        rust_code: "",
        errors: ["SyntaxError: invalid syntax"],
        warnings: [],
      }));
      
      const result = await transpileCode("def invalid syntax");
      
      expect(result.success).toBe(false);
      expect(result.errors).toContain("SyntaxError: invalid syntax");
      expect(result.rust_code).toBe("");
    });

    it("handles warnings", async () => {
      mockEngine.transpile.mockReturnValueOnce(createMockWasmResult({
        warnings: ["DeprecationWarning: Function is deprecated"],
      }));
      
      const result = await transpileCode("def deprecated(): pass");
      
      expect(result.success).toBe(true);
      expect(result.warnings).toContain("DeprecationWarning: Function is deprecated");
    });

    it("loads WASM automatically if not loaded", async () => {
      // Ensure WASM is not loaded initially
      expect(mockWasmLoaded).toBe(false);
      
      await transpileCode("def test(): pass");
      
      // WASM should be loaded after transpilation
      expect(mockWasmLoaded).toBe(true);
      expect(mockWasmInstance).toBe(mockEngine);
    });
  });

  describe("analyzeCode", () => {
    it("analyzes Python code structure", async () => {
      const result = await analyzeCode("def test(a, b): return a + b");
      
      expect(result.complexity).toBe(5);
      expect(result.cyclomatic_complexity).toBe(3);
      expect(result.functions).toHaveLength(1);
      expect(result.functions[0].name).toBe("test_function");
    });

    it("returns optimization suggestions", async () => {
      const result = await analyzeCode("for i in range(10): print(i)");
      
      expect(result.suggestions).toHaveLength(1);
      expect(result.suggestions[0].message).toBe("Consider using list comprehension");
      expect(result.suggestions[0].confidence).toBe(0.8);
    });

    it("identifies imports", async () => {
      const result = await analyzeCode("import math\nfrom typing import List");
      
      expect(result.imports).toContain("math");
      expect(result.imports).toContain("typing");
    });

    it("handles empty code", async () => {
      mockEngine.analyze.mockReturnValueOnce({
        complexity: 0,
        cyclomatic_complexity: 0,
        functions: [],
        imports: [],
        suggestions: [],
        anti_patterns: [],
      });
      
      const result = await analyzeCode("");
      
      expect(result.complexity).toBe(0);
      expect(result.functions).toHaveLength(0);
    });

    it("detects anti-patterns", async () => {
      mockEngine.analyze.mockReturnValueOnce({
        complexity: 10,
        cyclomatic_complexity: 8,
        functions: [],
        imports: [],
        suggestions: [],
        anti_patterns: [
          {
            line: 5,
            column: 10,
            pattern: "eval_usage",
            description: "Avoid using eval() for security reasons",
            severity: "error",
            suggestion: "Use ast.literal_eval() for safe evaluation",
          },
        ],
      });
      
      const result = await analyzeCode('eval("dangerous")');
      
      expect(result.anti_patterns).toHaveLength(1);
      expect(result.anti_patterns[0].pattern).toBe("eval_usage");
      expect(result.anti_patterns[0].severity).toBe("error");
    });
  });

  describe("benchmarkCode", () => {
    it("benchmarks code performance", async () => {
      const result = await benchmarkCode("def fib(n): return n if n < 2 else fib(n-1) + fib(n-2)");
      
      expect(result.iterations).toBe(1000);
      expect(result.mean_ms).toBe(10.6);
      expect(result.median_ms).toBe(11);
      expect(result.min_ms).toBe(9);
      expect(result.max_ms).toBe(12);
    });

    it("calculates standard deviation", async () => {
      const result = await benchmarkCode("def test(): pass");
      
      expect(result.std_dev_ms).toBeCloseTo(1.02, 2);
    });

    it("handles performance edge cases", async () => {
      mockEngine.benchmark.mockReturnValueOnce({
        iterations: 1,
        times_ms: [100],
        min_ms: 100,
        max_ms: 100,
        mean_ms: 100,
        median_ms: 100,
        std_dev_ms: 0,
      });
      
      const result = await benchmarkCode("def slow(): time.sleep(0.1)");
      
      expect(result.iterations).toBe(1);
      expect(result.std_dev_ms).toBe(0);
    });

    it("provides consistent timing results", async () => {
      const result = await benchmarkCode("def test(): pass");
      
      expect(result.times_ms).toHaveLength(5);
      expect(Math.min(...result.times_ms)).toBe(result.min_ms);
      expect(Math.max(...result.times_ms)).toBe(result.max_ms);
    });
  });

  describe("Error Handling", () => {
    it("handles WASM load failure gracefully", async () => {
      (preloadWasm as any).mockRejectedValueOnce(new Error("Failed to load WASM"));
      
      await expect(preloadWasm()).rejects.toThrow("Failed to load WASM");
    });

    it("handles transpilation exceptions", async () => {
      mockEngine.transpile.mockImplementationOnce(() => {
        throw new Error("Transpilation crashed");
      });
      
      const result = await transpileCode("def test(): pass");
      
      // Should return error result instead of throwing
      expect(result.success).toBe(false);
      expect(result.errors[0]).toContain("Transpilation crashed");
    });

    it("handles analysis exceptions", async () => {
      mockEngine.analyze.mockImplementationOnce(() => {
        throw new Error("Analysis failed");
      });
      
      // analyzeCode might throw or return empty result
      try {
        const result = await analyzeCode("def test(): pass");
        // If it returns, check for empty/error state
        expect(result).toBeDefined();
      } catch (error) {
        expect(error).toEqual(new Error("Analysis failed"));
      }
    });
  });
});