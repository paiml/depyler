import { TranspileResult } from '@/types';

interface WasmModule {
  DepylerWasm: new () => any;
  WasmTranspileOptions: new () => any;
  WasmTranspileResult: new () => any;
  WasmEnergyEstimate: new () => any;
  WasmQualityMetrics: new () => any;
  default: () => Promise<void>;
}

class WasmModuleManager {
  private module: WasmModule | null = null;
  private loading: Promise<WasmModule> | null = null;
  private engine: any = null;

  async loadModule(): Promise<WasmModule> {
    if (this.module) {
      return this.module;
    }

    if (this.loading) {
      return this.loading;
    }

    this.loading = this.initializeModule();
    this.module = await this.loading;
    this.loading = null;
    
    return this.module;
  }

  private async initializeModule(): Promise<WasmModule> {
    performance.mark('wasm-load-start');
    
    try {
      // Load the WASM module
      const wasmModule = await import('/wasm/depyler_wasm.js');
      await wasmModule.default();
      
      performance.mark('wasm-load-end');
      performance.measure('wasm-load', 'wasm-load-start', 'wasm-load-end');
      
      const loadTime = performance.getEntriesByName('wasm-load')[0]?.duration || 0;
      console.log(`WASM module loaded in ${loadTime.toFixed(2)}ms`);
      
      return wasmModule;
    } catch (error) {
      performance.mark('wasm-load-error');
      console.error('Failed to load WASM module:', error);
      throw new Error('Failed to initialize Depyler WASM module');
    }
  }

  async getEngine(): Promise<any> {
    if (this.engine) {
      return this.engine;
    }

    const module = await this.loadModule();
    this.engine = new module.DepylerWasm();
    return this.engine;
  }

  async createOptions(options: {
    verify: boolean;
    optimize: boolean;
    emit_docs: boolean;
    target_version: string;
  }): Promise<any> {
    const module = await this.loadModule();
    const wasmOptions = new module.WasmTranspileOptions();
    
    wasmOptions.set_verify(options.verify);
    wasmOptions.set_optimize(options.optimize);
    // Note: emit_docs and target_version would need to be added to WASM bindings
    
    return wasmOptions;
  }
}

// Singleton instance
const wasmManager = new WasmModuleManager();

export async function transpileCode(
  pythonCode: string,
  options: {
    verify: boolean;
    optimize: boolean;
    emit_docs: boolean;
    target_version: string;
  }
): Promise<TranspileResult> {
  try {
    const engine = await wasmManager.getEngine();
    const wasmOptions = await wasmManager.createOptions(options);
    
    performance.mark('transpile-start');
    const result = engine.transpile_with_metrics(pythonCode, wasmOptions);
    performance.mark('transpile-end');
    
    const transpileTime = performance.getEntriesByName('transpile-end')[0]?.startTime - 
                         performance.getEntriesByName('transpile-start')[0]?.startTime || 0;
    
    // Parse the result (it comes as a complex object with WASM bindings)
    const parsedResult = typeof result === 'string' ? JSON.parse(result) : result;
    
    return {
      success: parsedResult.transpile_result.success,
      rust_code: parsedResult.transpile_result.rust_code,
      errors: parsedResult.transpile_result.errors || [],
      warnings: parsedResult.transpile_result.warnings || [],
      transpile_time_ms: transpileTime,
      memory_usage_mb: parsedResult.transpile_result.memory_usage_mb || 0,
      energy_estimate: {
        joules: parsedResult.transpile_result.energy_estimate.joules,
        wattsAverage: parsedResult.transpile_result.energy_estimate.watts_average,
        co2Grams: parsedResult.transpile_result.energy_estimate.co2_grams,
        breakdown: {
          cpu: parsedResult.transpile_result.energy_estimate.joules * 0.7, // Estimated
          memory: parsedResult.transpile_result.energy_estimate.joules * 0.3,
        },
        confidence: parsedResult.transpile_result.energy_estimate.confidence,
        equivalentTo: getEnergyEquivalent(parsedResult.transpile_result.energy_estimate.joules),
      },
      quality_metrics: {
        pmat_score: parsedResult.pmat_score.tdg,
        productivity: parsedResult.pmat_score.productivity,
        maintainability: parsedResult.pmat_score.maintainability,
        accessibility: parsedResult.pmat_score.accessibility,
        testability: parsedResult.pmat_score.testability,
        code_complexity: 0, // Would need to be calculated
        cyclomatic_complexity: 0, // Would need to be calculated
      },
    };
  } catch (error) {
    console.error('Transpilation failed:', error);
    return {
      success: false,
      rust_code: '',
      errors: [error instanceof Error ? error.message : 'Transpilation failed'],
      warnings: [],
      transpile_time_ms: 0,
      memory_usage_mb: 0,
      energy_estimate: {
        joules: 0,
        wattsAverage: 0,
        co2Grams: 0,
        breakdown: { cpu: 0, memory: 0 },
        confidence: 0,
        equivalentTo: '',
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
}

function getEnergyEquivalent(joules: number): string {
  const equivalents = [
    { threshold: 0.001, text: "powering an LED for 1 second" },
    { threshold: 0.01, text: "sending 10 emails" },
    { threshold: 0.1, text: "charging a phone for 1 minute" },
    { threshold: 1.0, text: "keeping a light bulb on for 1 second" },
    { threshold: 10.0, text: "running a laptop for 1 second" },
    { threshold: 100.0, text: "boiling a cup of water" },
  ];
  
  for (const equiv of equivalents) {
    if (joules < equiv.threshold) {
      return equiv.text;
    }
  }
  
  return "running a small appliance for 1 minute";
}

export async function analyzeCode(pythonCode: string) {
  try {
    const engine = await wasmManager.getEngine();
    const result = engine.analyze_code(pythonCode);
    return typeof result === 'string' ? JSON.parse(result) : result;
  } catch (error) {
    console.error('Code analysis failed:', error);
    return null;
  }
}

export async function benchmarkCode(pythonCode: string, iterations: number = 10) {
  try {
    const engine = await wasmManager.getEngine();
    const result = engine.benchmark(pythonCode, iterations);
    return typeof result === 'string' ? JSON.parse(result) : result;
  } catch (error) {
    console.error('Benchmark failed:', error);
    return null;
  }
}

// Preload WASM module for better performance
export async function preloadWasm(): Promise<void> {
  try {
    await wasmManager.loadModule();
    console.log('WASM module preloaded successfully');
  } catch (error) {
    console.warn('Failed to preload WASM module:', error);
  }
}