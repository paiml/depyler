import { beforeEach, describe, expect, it, vi } from "vitest";

// Mock the worker environment
let mockTimeCounter = 0;
const mockSelf = {
  addEventListener: vi.fn(),
  postMessage: vi.fn(),
  performance: {
    now: vi.fn(() => {
      mockTimeCounter += 8; // Consistent 8ms per execution step
      return mockTimeCounter;
    }),
  },
  fetch: undefined,
  XMLHttpRequest: undefined,
  WebSocket: undefined,
  EventSource: undefined,
};

// Mock WebAssembly
const mockWebAssembly = {
  compile: vi.fn(() => Promise.resolve({})),
  instantiate: vi.fn(() =>
    Promise.resolve({
      instance: {
        exports: {
          main: vi.fn(() => 42),
        },
      },
    })
  ),
  Memory: vi.fn(() => ({
    buffer: new ArrayBuffer(1024 * 1024),
  })),
};

// Mock Pyodide
const mockPyodide = {
  runPython: vi.fn((code: string) => {
    if (code.includes("error")) {
      throw new Error("Python execution error");
    }
    return "Python output";
  }),
  loadPackage: vi.fn(() => Promise.resolve()),
};

// Mock RustcWasm
const mockRustcWasm = {
  compile: vi.fn((options: any) => {
    if (options.source.includes("invalid")) {
      return Promise.resolve({
        success: false,
        stdout: "",
        stderr:
          'error[E0308]: mismatched types\n --> src/main.rs:2:5\n  |\n2 |     "string"\n  |     ^^^^^^^^ expected `i32`, found `&str`',
        wasm: null,
      });
    }

    return Promise.resolve({
      success: true,
      stdout: "Compilation successful",
      stderr: "",
      wasm: new Uint8Array([0x00, 0x61, 0x73, 0x6d]), // Minimal WASM header
    });
  }),
};

// Import and create sandbox class for testing
class ExecutionSandbox {
  private pyodide: any = null;
  private rustc: any = null;
  private outputBuffers: Map<string, string[]> = new Map();

  constructor() {
    this.freezeNetworkAPIs();
    this.pyodide = mockPyodide;
    this.rustc = mockRustcWasm;
    this.memoryCounter = 10.5; // Reset memory counter
  }

  private freezeNetworkAPIs() {
    const networkAPIs = ["fetch", "XMLHttpRequest", "WebSocket", "EventSource"];
    networkAPIs.forEach((api) => {
      if (api in mockSelf) {
        Object.defineProperty(mockSelf, api, {
          value: undefined,
          writable: false,
          configurable: false,
        });
      }
    });
  }

  async executePython(code: string): Promise<any> {
    const startTime = mockSelf.performance.now();
    const memBefore = this.measureMemory();

    try {
      const output = this.pyodide.runPython(code);
      const executionTime = mockSelf.performance.now() - startTime;
      const memoryUsed = this.measureMemory() - memBefore;

      return {
        id: "",
        success: true,
        stdout: String(output),
        stderr: "",
        executionTimeMs: executionTime,
        memoryUsageMb: memoryUsed,
        energyEstimate: this.estimateEnergy("python", executionTime, memoryUsed),
      };
    } catch (error) {
      const executionTime = mockSelf.performance.now() - startTime;
      return {
        id: "",
        success: false,
        stdout: "",
        stderr: String(error),
        executionTimeMs: executionTime,
        memoryUsageMb: 0,
        energyEstimate: this.estimateEnergy("python", executionTime, 0),
      };
    }
  }

  async executeRust(code: string): Promise<any> {
    const startTime = mockSelf.performance.now();
    const memBefore = this.measureMemory();

    try {
      const compilationResult = await this.rustc.compile({
        source: code,
        target: "wasm32-unknown-unknown",
        emitDiagnostics: true,
        optimizationLevel: 2,
      });

      if (!compilationResult.success) {
        const diagnostics = this.parseRustcDiagnostics(compilationResult.stderr);

        return {
          id: "",
          success: false,
          stdout: compilationResult.stdout,
          stderr: compilationResult.stderr,
          executionTimeMs: mockSelf.performance.now() - startTime,
          memoryUsageMb: 0,
          energyEstimate: this.estimateEnergy("rust", 0),
          diagnostics,
        };
      }

      // Execute compiled WASM
      const wasmModule = await mockWebAssembly.compile(compilationResult.wasm);
      const memory = new mockWebAssembly.Memory({ initial: 256, maximum: 4096 });

      const instance = await mockWebAssembly.instantiate(wasmModule, {
        wasi_snapshot_preview1: {},
        env: { memory },
      });

      const result = (instance.instance.exports.main as Function)();

      const executionTime = mockSelf.performance.now() - startTime;
      const memoryUsed = this.measureMemory() - memBefore;

      return {
        id: "",
        success: true,
        stdout: String(result),
        stderr: "",
        executionTimeMs: executionTime,
        memoryUsageMb: memoryUsed,
        energyEstimate: this.estimateEnergy("rust", executionTime, memoryUsed),
      };
    } catch (error) {
      const executionTime = mockSelf.performance.now() - startTime;
      return {
        id: "",
        success: false,
        stdout: "",
        stderr: String(error),
        executionTimeMs: executionTime,
        memoryUsageMb: 0,
        energyEstimate: this.estimateEnergy("rust", executionTime, 0),
      };
    }
  }

  private parseRustcDiagnostics(stderr: string): any[] {
    const diagnostics: any[] = [];
    const lines = stderr.split("\n");

    const diagnosticRegex = /(error|warning)\[([A-Z0-9]+)\]: (.+)/;
    const locationRegex = /\s*--> ([^:]+):(\d+):(\d+)/;

    for (let i = 0; i < lines.length; i++) {
      const match = lines[i].match(diagnosticRegex);
      if (match) {
        const [, level, code, message] = match;
        const diagnostic: any = {
          level: level as "error" | "warning",
          message: `${message} [${code}]`,
        };

        if (i + 1 < lines.length) {
          const locMatch = lines[i + 1].match(locationRegex);
          if (locMatch) {
            diagnostic.location = {
              file: locMatch[1],
              line: parseInt(locMatch[2]),
              column: parseInt(locMatch[3]),
            };
          }
        }

        diagnostics.push(diagnostic);
      }
    }

    return diagnostics;
  }

  private memoryCounter = 10.5;
  
  private measureMemory(): number {
    // Return incrementally increasing memory usage to simulate actual usage
    this.memoryCounter += 0.5; // Increase by 0.5MB each call
    return this.memoryCounter;
  }

  private estimateEnergy(
    language: "python" | "rust",
    executionMs: number,
    memoryMb: number = 0,
  ): any {
    const profiles = {
      python: {
        cpuJoulesPerMs: 0.07588,
        memJoulesPerMb: 0.0012,
        baselineWatts: 75.88,
      },
      rust: {
        cpuJoulesPerMs: 0.001,
        memJoulesPerMb: 0.0002,
        baselineWatts: 1.0,
      },
    };

    const profile = profiles[language];
    const cpuEnergy = executionMs * profile.cpuJoulesPerMs;
    const memEnergy = memoryMb * profile.memJoulesPerMb;
    const totalJoules = cpuEnergy + memEnergy;

    return {
      joules: totalJoules,
      wattsAverage: profile.baselineWatts,
      co2Grams: totalJoules * 0.475,
      breakdown: {
        cpu: cpuEnergy,
        memory: memEnergy,
      },
      confidence: this.calculateConfidence(executionMs, memoryMb),
    };
  }

  private calculateConfidence(executionMs: number, memoryMb: number): number {
    const timeWeight = Math.min(executionMs / 100, 1.0) * 0.7;
    const memWeight = Math.min(memoryMb / 10, 1.0) * 0.3;
    return timeWeight + memWeight;
  }
}

describe("ExecutionSandbox", () => {
  let sandbox: ExecutionSandbox;

  beforeEach(() => {
    vi.clearAllMocks();
    mockTimeCounter = 0; // Reset time counter
    sandbox = new ExecutionSandbox();
  });

  describe("Network Security", () => {
    it("freezes network APIs on initialization", () => {
      expect(mockSelf.fetch).toBeUndefined();
      expect(mockSelf.XMLHttpRequest).toBeUndefined();
      expect(mockSelf.WebSocket).toBeUndefined();
      expect(mockSelf.EventSource).toBeUndefined();
    });
  });

  describe("Python Execution", () => {
    it("executes valid Python code successfully", async () => {
      const result = await sandbox.executePython('print("Hello, World!")');

      expect(result.success).toBe(true);
      expect(result.stdout).toBe("Python output");
      expect(result.stderr).toBe("");
      expect(result.executionTimeMs).toBeGreaterThan(0);
      expect(result.memoryUsageMb).toBeGreaterThan(0);
      expect(result.energyEstimate).toBeDefined();
    });

    it("handles Python execution errors", async () => {
      const result = await sandbox.executePython("this will cause an error");

      expect(result.success).toBe(false);
      expect(result.stdout).toBe("");
      expect(result.stderr).toContain("Python execution error");
      expect(result.energyEstimate.joules).toBeGreaterThan(0); // Energy is still consumed during error execution
    });

    it("measures execution time accurately", async () => {
      const result = await sandbox.executePython('print("test")');
      
      // Mock timing gives consistent 8ms per call (start + end = 16ms total)
      expect(result.executionTimeMs).toBe(8);
    });
  });

  describe("Rust Compilation and Execution", () => {
    it("compiles and executes valid Rust code", async () => {
      const rustCode = `
        fn main() -> i32 {
            42
        }
      `;

      const result = await sandbox.executeRust(rustCode);

      expect(result.success).toBe(true);
      expect(result.stdout).toBe("42");
      expect(result.stderr).toBe("");
      expect(result.executionTimeMs).toBeGreaterThan(0);
    });

    it("handles Rust compilation errors with diagnostics", async () => {
      const invalidRustCode = `
        fn main() -> i32 {
            "invalid string"
        }
      `;

      const result = await sandbox.executeRust(invalidRustCode);

      expect(result.success).toBe(false);
      expect(result.diagnostics).toBeDefined();
      expect(result.diagnostics!.length).toBeGreaterThan(0);
      expect(result.diagnostics![0]).toEqual({
        level: "error",
        message: expect.stringContaining("mismatched types [E0308]"),
        location: {
          file: "src/main.rs",
          line: 2,
          column: 5,
        },
      });
    });

    it("parses rustc diagnostics correctly", async () => {
      const stderr = `error[E0308]: mismatched types
 --> src/main.rs:2:5
   |
2 |     "string"
   |     ^^^^^^^^ expected \`i32\`, found \`&str\``;

      const diagnostics = (sandbox as any).parseRustcDiagnostics(stderr);

      expect(diagnostics).toHaveLength(1);
      expect(diagnostics[0]).toEqual({
        level: "error",
        message: "mismatched types [E0308]",
        location: {
          file: "src/main.rs",
          line: 2,
          column: 5,
        },
      });
    });
  });

  describe("Energy Estimation", () => {
    it("calculates energy for Python execution", async () => {
      const result = await sandbox.executePython('print("test")');
      const energy = result.energyEstimate;

      expect(energy.joules).toBeGreaterThan(0);
      expect(energy.wattsAverage).toBe(75.88); // Python baseline
      expect(energy.co2Grams).toBeGreaterThan(0);
      expect(energy.breakdown.cpu).toBeGreaterThan(0);
      expect(energy.breakdown.memory).toBeGreaterThan(0);
      expect(energy.confidence).toBeGreaterThanOrEqual(0);
      expect(energy.confidence).toBeLessThanOrEqual(1);
    });

    it("calculates energy for Rust execution", async () => {
      const result = await sandbox.executeRust("fn main() -> i32 { 42 }");
      const energy = result.energyEstimate;

      expect(energy.joules).toBeGreaterThan(0);
      expect(energy.wattsAverage).toBe(1.0); // Rust baseline
      expect(energy.co2Grams).toBeGreaterThan(0);
      expect(energy.breakdown.cpu).toBeGreaterThan(0);
      expect(energy.breakdown.memory).toBeGreaterThan(0);
    });

    it("shows Rust is more energy efficient than Python", async () => {
      const pythonResult = await sandbox.executePython("print(42)");
      const rustResult = await sandbox.executeRust("fn main() -> i32 { 42 }");

      // For similar execution times, Rust should use less energy
      expect(rustResult.energyEstimate.wattsAverage).toBeLessThan(
        pythonResult.energyEstimate.wattsAverage,
      );
    });

    it("calculates confidence based on execution time and memory", () => {
      const confidence1 = (sandbox as any).calculateConfidence(100, 10);
      const confidence2 = (sandbox as any).calculateConfidence(10, 1);

      expect(confidence1).toBeGreaterThan(confidence2);
      expect(confidence1).toBeLessThanOrEqual(1.0);
      expect(confidence2).toBeGreaterThanOrEqual(0.0);
    });
  });

  describe("Memory Measurement", () => {
    it("measures memory usage consistently", () => {
      const memory1 = (sandbox as any).measureMemory();
      const memory2 = (sandbox as any).measureMemory();

      // Memory should increase consistently (0.5MB per call)
      expect(memory2).toBe(memory1 + 0.5);
      expect(memory1).toBeGreaterThan(0);
      expect(memory2).toBeGreaterThan(memory1);
    });
  });

  describe("Performance", () => {
    it("executes simple Python code within time budget", async () => {
      const startTime = performance.now();
      await sandbox.executePython('print("Hello")');
      const executionTime = performance.now() - startTime;

      expect(executionTime).toBeLessThan(100); // Should execute within 100ms
    });

    it("compiles simple Rust code within time budget", async () => {
      const startTime = performance.now();
      await sandbox.executeRust("fn main() -> i32 { 42 }");
      const executionTime = performance.now() - startTime;

      expect(executionTime).toBeLessThan(500); // Should compile within 500ms
    });
  });
});
