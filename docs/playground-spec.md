# Depyler Interactive Environment: Technical Specification v3

> ⚠️ **EXPERIMENTAL FEATURE - UNSTABLE**
> 
> This Interactive Playground is currently under active development and is marked as:
> - **🧪 EXPERIMENTAL**: Features and APIs may change without notice
> - **⚡ UNSTABLE**: May contain bugs, performance issues, or incomplete features  
> - **🚧 NOT FOR PRODUCTION**: Use only for testing and evaluation purposes
>
> To use the playground, you must acknowledge this status by setting:
> `DEPYLER_EXPERIMENTAL=true depyler playground`

## Executive Summary

A production-grade WebAssembly playground implementing Toyota Production System
principles for continuous quality improvement. This specification establishes a
zero-configuration, deterministic analysis environment that serves as both the
primary onboarding vector and a kaizen-driven development tool for
Python-to-Rust transpilation.

## Architecture: 現地現物 (Genchi Genbutsu) - Go and See

### Workspace Structure with Quality Gates

```toml
# Cargo.toml (root) - Andon system for quality
[workspace]
members = [
    "crates/depyler",
    "crates/depyler-core",
    "crates/depyler-wasm",
    "playground/depyler-playground",
]

[workspace.metadata.quality]
max_tdg_score = 2.0
min_coverage = 85.0
max_cyclomatic_complexity = 15
required_documentation_coverage = 100.0

# playground/depyler-playground/Cargo.toml
[package]
name = "depyler-playground"
version = "0.1.0"
edition = "2021"

[dependencies]
depyler-core = { path = "../../crates/depyler-core", features = ["wasm", "deterministic"] }
wasm-bindgen = "0.2.92"
serde-wasm-bindgen = "0.6"
instant = { version = "0.1", features = ["wasm-bindgen"] }

[profile.wasm-production]
inherits = "release"
opt-level = "z"          # Size optimization
lto = "fat"              # Cross-crate inlining
codegen-units = 1        # Single compilation unit
strip = true             # Remove debug symbols
panic = "abort"          # No unwinding
incremental = false      # Deterministic builds

[build-dependencies]
wasm-opt = "0.116"       # Binaryen optimizer
```

### PMAT-Driven Quality Architecture with Configuration

```rust
// playground/depyler-playground/src/quality/mod.rs
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Configuration-driven quality metrics following PMAT principles
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PmatConfiguration {
    /// Target metrics derived from empirical analysis
    pub targets: QualityTargets,
    /// Non-linear scoring functions
    pub scoring: ScoringFunctions,
    /// Continuous improvement thresholds
    pub kaizen_thresholds: KaizenThresholds,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QualityTargets {
    /// Time to First Meaningful Paint target (P50)
    pub ttfmp_p50_ms: f64,
    /// Time to Interactive target (P90)
    pub tti_p90_ms: f64,
    /// WASM size budget (gzipped)
    pub wasm_size_budget_kb: f64,
    /// Transpilation latency targets by complexity
    pub transpile_targets: TranspilationTargets,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TranspilationTargets {
    /// Simple functions (<10 lines)
    pub simple_p95_ms: f64,
    /// Medium complexity (10-50 lines)
    pub medium_p95_ms: f64,
    /// Complex functions (>50 lines)
    pub complex_p95_ms: f64,
}

impl PmatConfiguration {
    /// Load from embedded configuration with validation
    pub fn load() -> Result<Self, ConfigError> {
        const CONFIG_TOML: &str = include_str!("../config/pmat.toml");
        let config: Self = toml::from_str(CONFIG_TOML)?;
        config.validate()?;
        Ok(config)
    }
    
    /// Validate configuration against invariants
    fn validate(&self) -> Result<(), ConfigError> {
        if self.targets.ttfmp_p50_ms <= 0.0 {
            return Err(ConfigError::InvalidTarget("ttfmp_p50_ms must be positive"));
        }
        // Additional validation rules...
        Ok(())
    }
}

/// Actual runtime metrics collected
#[derive(Debug, Clone)]
pub struct PlaygroundMetrics {
    pub page_load: PageLoadMetrics,
    pub transpilation: TranspilationMetrics,
    pub execution: ExecutionMetrics,
    pub quality_events: Vec<QualityEvent>,
}

impl PlaygroundMetrics {
    /// Calculate PMAT score using configuration
    pub fn calculate_pmat(&self, config: &PmatConfiguration) -> PmatScore {
        let productivity = self.calculate_productivity(&config.targets, &config.scoring);
        let maintainability = self.calculate_maintainability();
        let accessibility = self.calculate_accessibility(&config.targets);
        let testability = self.calculate_testability();
        
        PmatScore {
            productivity,
            maintainability,
            accessibility,
            testability,
            tdg: (productivity + maintainability + accessibility + testability) / 4.0,
            timestamp: std::time::SystemTime::now(),
        }
    }
    
    fn calculate_productivity(&self, targets: &QualityTargets, scoring: &ScoringFunctions) -> f64 {
        // Non-linear scoring based on empirical distribution
        let load_score = scoring.exponential_decay(
            self.page_load.tti_ms,
            targets.tti_p90_ms,
            0.5, // decay rate
        );
        
        let transpile_score = match self.transpilation.complexity_bucket {
            ComplexityBucket::Simple => scoring.sigmoid(
                self.transpilation.latency_p95_ms,
                targets.transpile_targets.simple_p95_ms,
            ),
            ComplexityBucket::Medium => scoring.sigmoid(
                self.transpilation.latency_p95_ms,
                targets.transpile_targets.medium_p95_ms,
            ),
            ComplexityBucket::Complex => scoring.sigmoid(
                self.transpilation.latency_p95_ms,
                targets.transpile_targets.complex_p95_ms,
            ),
        };
        
        // Weighted combination based on user impact
        0.3 * load_score + 0.7 * transpile_score
    }
}

/// Non-linear scoring functions based on empirical data
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScoringFunctions {
    sigmoid_steepness: f64,
    exponential_decay_rate: f64,
}

impl ScoringFunctions {
    /// Sigmoid function for smooth transitions
    pub fn sigmoid(&self, actual: f64, target: f64) -> f64 {
        let x = (target - actual) / target;
        1.0 / (1.0 + (-self.sigmoid_steepness * x).exp())
    }
    
    /// Exponential decay for time-based metrics
    pub fn exponential_decay(&self, actual: f64, target: f64, rate: f64) -> f64 {
        if actual <= target {
            1.0
        } else {
            (-rate * (actual - target) / target).exp()
        }
    }
}
```

## Frontend Architecture: 自働化 (Jidoka) - Build Quality In

### Enhanced Intelli-Sensei with Intelligent Caching

```typescript
// playground/src/editor/intelli-sensei.ts
import type { monaco } from "@monaco-editor/react";
import { LRUCache } from "lru-cache";
import { debounce } from "../utils/debounce";

interface FunctionContext {
  name: string;
  signature: string;
  body: string;
  complexity: number;
  lastModified: number;
}

interface AnalysisCache {
  context: FunctionContext;
  suggestions: AnnotationSuggestion[];
  antiPatterns: AntiPattern[];
  validUntil: number;
}

export class IntelliSensei {
  private advisorWorker: Worker;
  private analysisCache: LRUCache<string, AnalysisCache>;
  private pendingAnalysis: Map<string, Promise<AnalysisResult>>;

  constructor(private monaco: typeof monaco) {
    this.advisorWorker = new Worker(
      new URL("./workers/advisor.worker.ts", import.meta.url),
      { type: "module", name: "intelli-sensei-advisor" },
    );

    // LRU cache with 50 function capacity
    this.analysisCache = new LRUCache<string, AnalysisCache>({
      max: 50,
      ttl: 1000 * 60 * 5, // 5 minute TTL
      updateAgeOnGet: true,
    });

    this.pendingAnalysis = new Map();
  }

  async initialize(editor: monaco.editor.IStandaloneCodeEditor) {
    // Register Depyler-enhanced Python language
    this.registerDepylerLanguage();

    // Debounced analysis to prevent worker spam
    const debouncedAnalysis = debounce(
      (value: string, position: monaco.Position) =>
        this.analyzeContext(value, position),
      300,
    );

    // Real-time pattern detection with caching
    editor.onDidChangeModelContent(async (e) => {
      const position = editor.getPosition();
      if (!position) return;

      await debouncedAnalysis(editor.getValue(), position);
    });

    // Annotation provider with cache-aware completions
    this.monaco.languages.registerCompletionItemProvider("python-depyler", {
      triggerCharacters: ["@", ":"],
      provideCompletionItems: async (model, position) => {
        return this.getAnnotationCompletions(model, position);
      },
    });

    // Inline hints for optimization opportunities
    this.monaco.languages.registerInlayHintsProvider("python-depyler", {
      provideInlayHints: async (model, range, token) => {
        return this.getOptimizationHints(model, range);
      },
    });
  }

  private async analyzeContext(
    code: string,
    position: monaco.Position,
  ): Promise<AnalysisResult> {
    const functionContext = this.extractFunctionContext(code, position);
    if (!functionContext) return { suggestions: [], antiPatterns: [] };

    const cacheKey = this.computeCacheKey(functionContext);

    // Check cache validity
    const cached = this.analysisCache.get(cacheKey);
    if (cached && this.isCacheValid(cached, functionContext)) {
      return {
        suggestions: cached.suggestions,
        antiPatterns: cached.antiPatterns,
      };
    }

    // Prevent duplicate analysis requests
    const pending = this.pendingAnalysis.get(cacheKey);
    if (pending) return pending;

    // Perform analysis
    const analysisPromise = this.performAnalysis(functionContext);
    this.pendingAnalysis.set(cacheKey, analysisPromise);

    try {
      const result = await analysisPromise;

      // Update cache
      this.analysisCache.set(cacheKey, {
        context: functionContext,
        suggestions: result.suggestions,
        antiPatterns: result.antiPatterns,
        validUntil: Date.now() + 60000, // 1 minute validity
      });

      return result;
    } finally {
      this.pendingAnalysis.delete(cacheKey);
    }
  }

  private computeCacheKey(context: FunctionContext): string {
    // Deterministic key based on function signature and body hash
    const bodyHash = this.hashCode(context.body);
    return `${context.name}:${context.signature}:${bodyHash}`;
  }

  private isCacheValid(
    cached: AnalysisCache,
    current: FunctionContext,
  ): boolean {
    // Structural comparison to detect meaningful changes
    return cached.context.signature === current.signature &&
      cached.context.body === current.body &&
      Date.now() < cached.validUntil;
  }

  private async performAnalysis(
    context: FunctionContext,
  ): Promise<AnalysisResult> {
    return new Promise((resolve, reject) => {
      const messageId = crypto.randomUUID();

      const handler = (e: MessageEvent) => {
        if (e.data.id === messageId) {
          this.advisorWorker.removeEventListener("message", handler);
          if (e.data.error) {
            reject(new Error(e.data.error));
          } else {
            resolve(e.data.result);
          }
        }
      };

      this.advisorWorker.addEventListener("message", handler);
      this.advisorWorker.postMessage({
        type: "ANALYZE_FUNCTION",
        id: messageId,
        context,
      });
    });
  }

  private hashCode(str: string): number {
    let hash = 0;
    for (let i = 0; i < str.length; i++) {
      const char = str.charCodeAt(i);
      hash = ((hash << 5) - hash) + char;
      hash = hash & hash; // Convert to 32-bit integer
    }
    return Math.abs(hash);
  }
}
```

### Production-Grade Execution Sandbox

```typescript
// playground/src/workers/execution-sandbox.worker.ts
import { PyodideInterface } from "pyodide";
import { CompilationResult, RustcWasm } from "@depyler/rustc-wasm";

interface ExecutionRequest {
  id: string;
  type: "EXECUTE_PYTHON" | "EXECUTE_RUST";
  code: string;
}

interface ExecutionResult {
  id: string;
  success: boolean;
  stdout: string;
  stderr: string;
  executionTimeMs: number;
  memoryUsageMb: number;
  energyEstimate: EnergyEstimate;
  diagnostics?: CompilerDiagnostic[];
}

interface CompilerDiagnostic {
  level: "error" | "warning" | "info";
  message: string;
  location?: {
    line: number;
    column: number;
    file: string;
  };
  suggestion?: string;
}

class ExecutionSandbox {
  private pyodide: PyodideInterface | null = null;
  private rustc: RustcWasm | null = null;
  private outputBuffers: Map<string, string[]> = new Map();

  constructor() {
    // Security: Freeze network APIs immediately
    this.freezeNetworkAPIs();

    // Set up message handling
    self.addEventListener("message", this.handleMessage.bind(this));
  }

  private freezeNetworkAPIs() {
    // Prevent any network access from within the sandbox
    const networkAPIs = ["fetch", "XMLHttpRequest", "WebSocket", "EventSource"];
    networkAPIs.forEach((api) => {
      if (api in self) {
        Object.defineProperty(self, api, {
          value: undefined,
          writable: false,
          configurable: false,
        });
      }
    });
  }

  async executeRust(code: string): Promise<ExecutionResult> {
    const startTime = performance.now();
    const memBefore = this.measureMemory();

    try {
      // Compile with full diagnostics
      const compilationResult = await this.rustc!.compile({
        source: code,
        target: "wasm32-unknown-unknown",
        emitDiagnostics: true,
        optimizationLevel: 2,
      });

      if (!compilationResult.success) {
        // Parse structured compiler output
        const diagnostics = this.parseRustcDiagnostics(
          compilationResult.stderr,
        );

        return {
          id: "",
          success: false,
          stdout: compilationResult.stdout,
          stderr: compilationResult.stderr,
          executionTimeMs: performance.now() - startTime,
          memoryUsageMb: 0,
          energyEstimate: this.estimateEnergy("rust", 0),
          diagnostics,
        };
      }

      // Execute compiled WASM
      const wasmModule = await WebAssembly.compile(compilationResult.wasm);
      const memory = new WebAssembly.Memory({ initial: 256, maximum: 4096 });

      const instance = await WebAssembly.instantiate(wasmModule, {
        wasi_snapshot_preview1: this.createWasiImports(memory),
        env: { memory },
      });

      // Execute with timeout protection
      const executionResult = await this.executeWithTimeout(
        () => (instance.exports.main as Function)(),
        5000, // 5 second timeout
      );

      const executionTime = performance.now() - startTime;
      const memoryUsed = this.measureMemory() - memBefore;

      return {
        id: "",
        success: true,
        stdout: this.outputBuffers.get("rust")?.join("\n") || "",
        stderr: "",
        executionTimeMs: executionTime,
        memoryUsageMb: memoryUsed,
        energyEstimate: this.estimateEnergy("rust", executionTime, memoryUsed),
      };
    } catch (error) {
      return {
        id: "",
        success: false,
        stdout: this.outputBuffers.get("rust")?.join("\n") || "",
        stderr: String(error),
        executionTimeMs: performance.now() - startTime,
        memoryUsageMb: 0,
        energyEstimate: this.estimateEnergy("rust", 0),
      };
    }
  }

  private parseRustcDiagnostics(stderr: string): CompilerDiagnostic[] {
    const diagnostics: CompilerDiagnostic[] = [];
    const lines = stderr.split("\n");

    // Rust compiler output format:
    // error[E0308]: mismatched types
    //  --> src/main.rs:10:5
    //   |
    // 10 |     x + "string"
    //   |     ^^^^^^^^^^^^ expected `i32`, found `&str`

    const diagnosticRegex = /(error|warning)\[([A-Z0-9]+)\]: (.+)/;
    const locationRegex = /\s*--> ([^:]+):(\d+):(\d+)/;

    for (let i = 0; i < lines.length; i++) {
      const match = lines[i].match(diagnosticRegex);
      if (match) {
        const [, level, code, message] = match;
        const diagnostic: CompilerDiagnostic = {
          level: level as "error" | "warning",
          message: `${message} [${code}]`,
        };

        // Look for location on next line
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

  private estimateEnergy(
    language: "python" | "rust",
    executionMs: number,
    memoryMb: number = 0,
  ): EnergyEstimate {
    // Enhanced energy model based on Pereira et al. (2017) with memory component
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
      co2Grams: totalJoules * 0.475, // US grid average
      breakdown: {
        cpu: cpuEnergy,
        memory: memEnergy,
      },
      confidence: this.calculateConfidence(executionMs, memoryMb),
      equivalentTo: this.getDetailedEnergyEquivalent(totalJoules),
    };
  }

  private calculateConfidence(executionMs: number, memoryMb: number): number {
    // Higher confidence for longer-running, memory-intensive operations
    const timeWeight = Math.min(executionMs / 100, 1.0) * 0.7;
    const memWeight = Math.min(memoryMb / 10, 1.0) * 0.3;
    return timeWeight + memWeight;
  }
}
```

### Enhanced Visualization with Efficient D3 Patterns

```typescript
// playground/src/components/visualizations/EnergyGauge.tsx
import { useEffect, useMemo, useRef } from "react";
import * as d3 from "d3";

interface EnergyGaugeProps {
  savings: number;
  breakdown: EnergyBreakdown;
  confidence: number;
}

export function EnergyGauge(
  { savings, breakdown, confidence }: EnergyGaugeProps,
) {
  const svgRef = useRef<SVGSVGElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);

  // Memoize scales to prevent recreation
  const scales = useMemo(() => ({
    savings: d3.scaleLinear().domain([0, 100]).range([
      -Math.PI / 2,
      Math.PI / 2,
    ]),
    color: d3.scaleSequential()
      .domain([0, 100])
      .interpolator(d3.interpolateRdYlGn),
  }), []);

  useEffect(() => {
    if (!svgRef.current || !containerRef.current) return;

    const svg = d3.select(svgRef.current);
    const { width, height } = containerRef.current.getBoundingClientRect();
    const radius = Math.min(width, height) / 2 - 20;

    // Use D3's idiomatic enter/update/exit pattern
    const g = svg.selectAll<SVGGElement, number>("g.gauge-container")
      .data([savings], (d) => d)
      .join(
        (enter) =>
          enter.append("g")
            .attr("class", "gauge-container")
            .call((g) => g.append("path").attr("class", "gauge-background"))
            .call((g) => g.append("path").attr("class", "gauge-value"))
            .call((g) => g.append("text").attr("class", "gauge-text"))
            .call((g) => g.append("text").attr("class", "gauge-label")),
        (update) => update,
        (exit) => exit.remove(),
      )
      .attr("transform", `translate(${width / 2}, ${height - 20})`);

    // Background arc
    const backgroundArc = d3.arc<any>()
      .innerRadius(radius * 0.7)
      .outerRadius(radius)
      .startAngle(-Math.PI / 2)
      .endAngle(Math.PI / 2);

    g.select(".gauge-background")
      .attr("d", backgroundArc)
      .style("fill", "#e0e0e0");

    // Value arc with smooth transition
    const valueArc = d3.arc<any>()
      .innerRadius(radius * 0.7)
      .outerRadius(radius)
      .startAngle(-Math.PI / 2);

    g.select(".gauge-value")
      .datum({ endAngle: scales.savings(savings) })
      .style("fill", scales.color(savings))
      .transition()
      .duration(750)
      .ease(d3.easeCubicInOut)
      .attrTween("d", function (d) {
        const interpolate = d3.interpolate(
          this._current || { endAngle: -Math.PI / 2 },
          d,
        );
        this._current = interpolate(1);
        return (t) => valueArc(interpolate(t));
      });

    // Animated text
    g.select(".gauge-text")
      .attr("text-anchor", "middle")
      .attr("dy", "-0.5em")
      .style("font-size", "2.5em")
      .style("font-weight", "bold")
      .style("fill", scales.color(savings))
      .transition()
      .duration(750)
      .tween("text", function () {
        const interpolate = d3.interpolate(
          this._current || 0,
          savings,
        );
        this._current = savings;
        return function (t) {
          this.textContent = `${Math.round(interpolate(t))}%`;
        };
      });

    // Confidence indicator
    g.select(".gauge-label")
      .attr("text-anchor", "middle")
      .attr("dy", "1em")
      .style("font-size", "0.9em")
      .style("fill", "#666")
      .text(`Confidence: ${Math.round(confidence * 100)}%`);
  }, [savings, breakdown, confidence, scales]);

  return (
    <div ref={containerRef} className="energy-gauge-container">
      <svg ref={svgRef} width="100%" height="200" />
      <EnergyBreakdownDetails breakdown={breakdown} />
    </div>
  );
}
```

### User Feedback for Long Operations

```typescript
// playground/src/components/ExecutionButton.tsx
import { useCallback, useState } from "react";
import { usePlaygroundStore } from "../store";

interface LoadingState {
  type: "idle" | "downloading" | "compiling" | "executing";
  progress?: number;
  message?: string;
}

export function ExecutionButton() {
  const [loadingState, setLoadingState] = useState<LoadingState>({
    type: "idle",
  });
  const { executeCode, isToolchainCached } = usePlaygroundStore();

  const handleExecute = useCallback(async () => {
    try {
      if (!isToolchainCached) {
        setLoadingState({
          type: "downloading",
          progress: 0,
          message:
            "Downloading Rust toolchain (21 MB)... This only happens once.",
        });

        // Download with progress tracking
        await downloadToolchain({
          onProgress: (progress) => {
            setLoadingState((prev) => ({
              ...prev,
              progress: Math.round(progress * 100),
            }));
          },
        });
      }

      setLoadingState({
        type: "compiling",
        message: "Compiling Rust code...",
      });

      await new Promise((resolve) => setTimeout(resolve, 100)); // UI update

      setLoadingState({
        type: "executing",
        message: "Executing and measuring performance...",
      });

      await executeCode();
    } finally {
      setLoadingState({ type: "idle" });
    }
  }, [executeCode, isToolchainCached]);

  const renderButtonContent = () => {
    switch (loadingState.type) {
      case "downloading":
        return (
          <div className="flex items-center space-x-2">
            <CircularProgress value={loadingState.progress} size={20} />
            <span className="text-sm">{loadingState.message}</span>
          </div>
        );
      case "compiling":
      case "executing":
        return (
          <div className="flex items-center space-x-2">
            <Spinner size={20} />
            <span>{loadingState.message}</span>
          </div>
        );
      default:
        return (
          <div className="flex items-center space-x-2">
            <PlayIcon size={20} />
            <span>Run Comparison</span>
          </div>
        );
    }
  };

  return (
    <button
      onClick={handleExecute}
      disabled={loadingState.type !== "idle"}
      className="execution-button"
      aria-busy={loadingState.type !== "idle"}
    >
      {renderButtonContent()}
    </button>
  );
}
```

### Enhanced Quality Telemetry

```typescript
// playground/src/lib/telemetry/quality-telemetry.ts
interface CodeMetrics {
  sizeBytes: number;
  numFunctions: number;
  numLoops: number;
  numConditionals: number;
  maxNesting: number;
  usesAsyncAwait: boolean;
  usesComplexTypes: boolean;
  hasAnnotations: boolean;
}

interface TelemetryPayload {
  sessionId: string;
  timestamp: number;
  metrics: PlaygroundMetrics;
  pmatScore: PmatScore;
  codeMetrics: CodeMetrics;
  environment: {
    browser: string;
    viewport: { width: number; height: number };
    connection: string;
    deviceMemory?: number;
  };
  qualityEvents: QualityEvent[];
}

export class QualityTelemetry {
  private sessionId: string;
  private buffer: TelemetryPayload[] = [];
  private flushTimer?: number;

  constructor() {
    this.sessionId = this.generateSessionId();

    // Batch telemetry for efficiency
    this.scheduleFlush();

    // Flush on page unload
    window.addEventListener("visibilitychange", () => {
      if (document.visibilityState === "hidden") {
        this.flush();
      }
    });
  }

  recordQualityEvent(event: QualityEvent, codeContext: string) {
    const codeMetrics = this.analyzeCode(codeContext);
    const environment = this.captureEnvironment();

    const payload: TelemetryPayload = {
      sessionId: this.sessionId,
      timestamp: Date.now(),
      metrics: event.metrics,
      pmatScore: event.pmatScore,
      codeMetrics,
      environment,
      qualityEvents: [event],
    };

    this.buffer.push(payload);

    // Immediate send for critical events
    if (event.severity === "critical") {
      this.flush();
    }
  }

  private analyzeCode(code: string): CodeMetrics {
    // Quick static analysis for telemetry
    const lines = code.split("\n");
    const functionPattern = /^def\s+\w+\s*\(/gm;
    const loopPattern = /^\s*(for|while)\s+/gm;
    const conditionalPattern = /^\s*if\s+/gm;

    let maxNesting = 0;
    let currentNesting = 0;

    lines.forEach((line) => {
      const indent = line.search(/\S/);
      if (indent !== -1) {
        currentNesting = Math.floor(indent / 4);
        maxNesting = Math.max(maxNesting, currentNesting);
      }
    });

    return {
      sizeBytes: new TextEncoder().encode(code).length,
      numFunctions: (code.match(functionPattern) || []).length,
      numLoops: (code.match(loopPattern) || []).length,
      numConditionals: (code.match(conditionalPattern) || []).length,
      maxNesting,
      usesAsyncAwait: /async\s+def/.test(code),
      usesComplexTypes: /Dict\[|List\[|Tuple\[/.test(code),
      hasAnnotations: /#\s*@depyler:/.test(code),
    };
  }

  private captureEnvironment() {
    const connection = (navigator as any).connection;

    return {
      browser: navigator.userAgent,
      viewport: {
        width: window.innerWidth,
        height: window.innerHeight,
      },
      connection: connection?.effectiveType || "unknown",
      deviceMemory: (navigator as any).deviceMemory,
    };
  }

  private flush() {
    if (this.buffer.length === 0) return;

    const payload = JSON.stringify(this.buffer);
    this.buffer = [];

    // Use sendBeacon for reliability
    if (navigator.sendBeacon) {
      navigator.sendBeacon("/api/telemetry", payload);
    } else {
      // Fallback for older browsers
      fetch("/api/telemetry", {
        method: "POST",
        body: payload,
        keepalive: true,
      }).catch(() => {
        // Telemetry is best-effort
      });
    }
  }
}
```

## Continuous Improvement Pipeline

### Kaizen-Driven Development Workflow

```yaml
# .github/workflows/playground-kaizen.yml
name: Playground Continuous Improvement

on:
  push:
    paths:
      - "playground/**"
  schedule:
    - cron: "0 2 * * *" # Daily quality check

jobs:
  quality-analysis:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Run PMAT Analysis
        run: |
          cargo run -p depyler-quality -- \
            analyze playground/ \
            --format sarif \
            --output playground-quality.sarif

      - name: Check Quality Gates
        run: |
          cargo run -p depyler-quality -- \
            enforce playground/ \
            --max-tdg 2.0 \
            --min-coverage 85.0 \
            --max-complexity 15

      - name: Measure WASM Size
        run: |
          cd playground/depyler-playground
          cargo build --profile wasm-production --target wasm32-unknown-unknown
          wasm-opt -Oz --enable-bulk-memory -o optimized.wasm target/wasm32-unknown-unknown/wasm-production/depyler_playground.wasm
          gzip -9 < optimized.wasm > bundle.wasm.gz

          SIZE_KB=$(du -k bundle.wasm.gz | cut -f1)
          echo "WASM_SIZE_KB=$SIZE_KB" >> $GITHUB_ENV

          if [ $SIZE_KB -gt 1500 ]; then
            echo "::error::WASM size ${SIZE_KB}KB exceeds 1500KB budget"
            exit 1
          fi

      - name: Performance Benchmarks
        run: |
          cd playground
          deno task bench --json > benchmarks.json

          # Check P95 latencies
          P95_SIMPLE=$(jq '.transpilation.simple.p95' benchmarks.json)
          if (( $(echo "$P95_SIMPLE > 50" | bc -l) )); then
            echo "::warning::Simple transpilation P95 ${P95_SIMPLE}ms exceeds 50ms target"
          fi
```

## Deployment Architecture

```nginx
# nginx.conf - Production deployment with intelligent caching
server {
    listen 443 ssl http2;
    server_name playground.depyler.io;
    
    # Security headers
    add_header X-Content-Type-Options nosniff;
    add_header X-Frame-Options DENY;
    add_header X-XSS-Protection "1; mode=block";
    add_header Content-Security-Policy "default-src 'self'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self' 'unsafe-inline'";
    
    # WASM optimization
    location ~ \.wasm$ {
        add_header Cache-Control "public, max-age=31536000, immutable";
        add_header Content-Type "application/wasm";
        gzip_static on;
        brotli_static on;
    }
    
    # Service worker for offline support
    location /sw.js {
        add_header Cache-Control "no-cache";
        add_header Service-Worker-Allowed "/";
    }
    
    # API endpoints with rate limiting
    location /api/ {
        limit_req zone=api burst=20 nodelay;
        proxy_pass http://localhost:8080;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

This specification embodies the Toyota Way principles throughout, with
continuous quality measurement, intelligent caching for performance,
comprehensive error handling, and a relentless focus on user experience. The
architecture ensures deterministic builds, measurable quality improvements, and
a foundation for kaizen-driven development.

# Depyler Interactive Environment: Implementation Checklist

## Phase 0: Foundation & Infrastructure

### Workspace Configuration

- [ ] Create `playground/` directory structure in repository root
- [ ] Add `playground/depyler-playground/` to workspace members in root
      `Cargo.toml`
- [ ] Create `playground/depyler-playground/Cargo.toml` with:
  - [ ] WASM-specific profile `[profile.wasm-production]`
  - [ ] Verify `opt-level = "z"` and `lto = "fat"`
  - [ ] Add `wasm-bindgen = "0.2.92"` dependency
  - [ ] Add `instant = { version = "0.1", features = ["wasm-bindgen"] }`
- [ ] Create `playground/depyler-playground/config/pmat.toml`:
  ```toml
  [targets]
  ttfmp_p50_ms = 1000.0
  tti_p90_ms = 3000.0
  wasm_size_budget_kb = 1500.0

  [targets.transpile_targets]
  simple_p95_ms = 50.0
  medium_p95_ms = 100.0
  complex_p95_ms = 250.0

  [scoring]
  sigmoid_steepness = 5.0
  exponential_decay_rate = 0.5
  ```
- [ ] Verify WASM adapter crate exists at `crates/depyler-wasm/`
- [ ] Add WASM feature flag to `crates/depyler-core/Cargo.toml`:
  ```toml
  [features]
  wasm = ["wasm-bindgen", "console_error_panic_hook"]
  deterministic = ["rustc-hash"]
  ```

### Development Environment

- [ ] Create `playground/deno.json`:
  ```json
  {
    "tasks": {
      "dev": "vite",
      "build": "vite build",
      "preview": "vite preview",
      "lint": "deno lint",
      "test": "vitest",
      "bench": "vitest bench",
      "wasm:build": "wasm-pack build ../depyler-playground --target web --out-dir ../../dist/wasm",
      "wasm:optimize": "wasm-opt -Oz -o dist/optimized.wasm dist/wasm/depyler_playground_bg.wasm"
    },
    "imports": {
      "@/": "./src/",
      "react": "https://esm.sh/react@18.2.0",
      "react-dom": "https://esm.sh/react-dom@18.2.0",
      "@monaco-editor/react": "https://esm.sh/@monaco-editor/react@4.6.0",
      "d3": "https://esm.sh/d3@7.8.5",
      "zustand": "https://esm.sh/zustand@4.5.0",
      "lru-cache": "https://esm.sh/lru-cache@10.2.0"
    }
  }
  ```
- [ ] Create `playground/vite.config.ts` with WASM support:
  ```typescript
  import { defineConfig } from "vite";
  import react from "@vitejs/plugin-react";
  import wasm from "vite-plugin-wasm";

  export default defineConfig({
    plugins: [react(), wasm()],
    worker: {
      format: "es",
      plugins: [wasm()],
    },
    build: {
      target: "es2022",
      minify: "terser",
      terserOptions: {
        compress: {
          drop_console: true,
          passes: 3,
        },
      },
    },
  });
  ```

## Phase 1: WASM Module Implementation

### Core WASM Bindings

- [ ] Create `crates/depyler-wasm/src/lib.rs`:
  - [ ] Implement `WasmTranspiler` struct with `#[wasm_bindgen]`
  - [ ] Add `transpile()` method returning structured metrics
  - [ ] Implement `TranspileMetrics` with serde serialization
  - [ ] Add `estimate_energy_reduction()` function
  - [ ] Verify panic hook installation: `console_error_panic_hook::set_once()`
- [ ] Create `crates/depyler-wasm/src/error.rs`:
  - [ ] Define WASM-compatible error types
  - [ ] Implement `From<TranspileError> for JsValue`
  - [ ] Add structured error serialization
- [ ] Build and verify WASM module:
  ```bash
  cd crates/depyler-wasm
  wasm-pack build --target web --out-dir ../../playground/public/wasm
  ls -lh ../../playground/public/wasm/*.wasm  # Verify < 5MB uncompressed
  ```

### Quality Module

- [ ] Create `playground/depyler-playground/src/quality/mod.rs`:
  - [ ] Implement `PmatConfiguration` with TOML deserialization
  - [ ] Add validation in `PmatConfiguration::load()`
  - [ ] Implement `PlaygroundMetrics` struct
  - [ ] Add non-linear scoring functions:
    - [ ] `sigmoid()` with configurable steepness
    - [ ] `exponential_decay()` for time metrics
  - [ ] Write unit tests for score calculations:
    ```rust
    #[test]
    fn test_productivity_score_bounds() {
        let metrics = PlaygroundMetrics { /* ... */ };
        let score = metrics.calculate_productivity(&config);
        assert!(score >= 0.0 && score <= 30.0);
    }
    ```

## Phase 2: Frontend Infrastructure

### TypeScript Setup

- [ ] Create `playground/src/types/index.ts`:
  ```typescript
  export interface TranspileMetrics {
    rust_code: string;
    parse_time_ms: number;
    transpile_time_ms: number;
    ast_nodes: number;
    complexity_score: number;
    energy_reduction: EnergyEstimate;
  }

  export interface EnergyEstimate {
    joules: number;
    wattsAverage: number;
    co2Grams: number;
    breakdown: { cpu: number; memory: number };
    confidence: number;
    equivalentTo: string;
  }
  ```
- [ ] Create `playground/src/store/index.ts` with Zustand:
  - [ ] Define `PlaygroundState` interface
  - [ ] Implement actions: `setPythonCode`, `transpileCode`, `executeCode`
  - [ ] Add debounced transpilation (300ms)
  - [ ] Implement optimistic UI updates

### WASM Module Manager

- [ ] Create `playground/src/lib/wasm-manager.ts`:
  - [ ] Implement `WasmModuleManager` class
  - [ ] Add streaming compilation: `WebAssembly.compileStreaming()`
  - [ ] Implement shared memory allocation
  - [ ] Add module caching in Map
  - [ ] Create worker pre-warming logic
- [ ] Create `playground/src/workers/wasm-loader.worker.ts`:
  - [ ] Handle `LOAD_MODULE` messages
  - [ ] Implement module instantiation
  - [ ] Add structured error reporting
- [ ] Verify WASM loading performance:
  ```typescript
  performance.mark("wasm-load-start");
  await wasmManager.loadModule("depyler", "/wasm/depyler_bg.wasm");
  performance.mark("wasm-load-end");
  performance.measure("wasm-load", "wasm-load-start", "wasm-load-end");
  // Assert: < 500ms on fast connection
  ```

## Phase 3: Intelli-Sensei Editor

### Monaco Editor Integration

- [ ] Create `playground/src/editor/monaco-config.ts`:
  - [ ] Register `python-depyler` language
  - [ ] Define Monarch tokenizer with `@depyler:` annotation support
  - [ ] Add syntax highlighting for annotations
- [ ] Create `playground/src/editor/intelli-sensei.ts`:
  - [ ] Implement `IntelliSensei` class
  - [ ] Add LRU cache (50 functions, 5min TTL):
    ```typescript
    this.analysisCache = new LRUCache<string, AnalysisCache>({
      max: 50,
      ttl: 1000 * 60 * 5,
      updateAgeOnGet: true,
    });
    ```
  - [ ] Implement `computeCacheKey()` with deterministic hashing
  - [ ] Add debounced analysis (300ms)
  - [ ] Implement completion provider for annotations
  - [ ] Add inlay hints provider

### Advisor Worker

- [ ] Create `playground/src/workers/advisor.worker.ts`:
  - [ ] Handle `ANALYZE_FUNCTION` messages
  - [ ] Implement anti-pattern detection:
    - [ ] Dynamic class generation
    - [ ] Excessive `*args`/`**kwargs`
    - [ ] `eval()` usage
    - [ ] C extension imports
  - [ ] Add optimization opportunity detection
  - [ ] Return structured suggestions with impact levels
- [ ] Create `playground/src/editor/providers/transpilation-advisor.ts`:
  - [ ] Implement `TranspilationAdvisor` class
  - [ ] Add pattern matching for common issues
  - [ ] Generate contextual fix suggestions

## Phase 4: Execution Sandbox

### Sandbox Worker Implementation

- [ ] Create `playground/src/workers/execution-sandbox.worker.ts`:
  - [ ] Implement network API freezing:
    ```typescript
    ["fetch", "XMLHttpRequest", "WebSocket", "EventSource"].forEach((api) => {
      Object.defineProperty(self, api, {
        value: undefined,
        writable: false,
        configurable: false,
      });
    });
    ```
  - [ ] Add Pyodide initialization with minimal packages
  - [ ] Implement Rust compilation with diagnostics
  - [ ] Add structured error parsing for rustc output
  - [ ] Implement execution timeout (5s)
  - [ ] Add memory measurement hooks

### Rust Toolchain Integration

- [ ] Create mock `@depyler/rustc-wasm` package:
  - [ ] Define `CompilationResult` interface
  - [ ] Add diagnostic parsing
  - [ ] Implement WASI imports
- [ ] Add cargo template generation:
  ```rust
  const CARGO_TEMPLATE = `
  [package]
  name = "playground"
  version = "0.1.0"
  edition = "2021"

  [dependencies]

  [[bin]]
  name = "playground"
  path = "main.rs"
  `;
  ```
- [ ] Verify compilation error propagation:
  ```typescript
  // Test: Compile invalid Rust
  const result = await sandbox.executeRust(
    'fn main() { let x: i32 = "string"; }',
  );
  assert(result.diagnostics[0].message.includes("mismatched types"));
  ```

### Energy Estimation

- [ ] Implement enhanced energy model:
  - [ ] CPU energy: `executionMs * profile.cpuJoulesPerMs`
  - [ ] Memory energy: `memoryMb * profile.memJoulesPerMb`
  - [ ] Confidence calculation based on execution duration
  - [ ] Energy equivalent mappings:
    ```typescript
    const equivalents = [
      { threshold: 0.001, text: "powering an LED for 1 second" },
      { threshold: 0.01, text: "sending 10 emails" },
      { threshold: 0.1, text: "charging a phone for 1 minute" },
      // ...
    ];
    ```

## Phase 5: Visualization Components

### Energy Gauge

- [ ] Create `playground/src/components/visualizations/EnergyGauge.tsx`:
  - [ ] Implement D3 enter/update/exit pattern
  - [ ] Add smooth transitions (750ms, cubic-in-out)
  - [ ] Implement color scale (red-yellow-green)
  - [ ] Add confidence indicator
  - [ ] Prevent DOM pollution with proper selections
- [ ] Create
      `playground/src/components/visualizations/EnergyBreakdownDetails.tsx`:
  - [ ] Show CPU vs memory energy breakdown
  - [ ] Add tooltip explanations
  - [ ] Include confidence visualization

### Execution Button

- [ ] Create `playground/src/components/ExecutionButton.tsx`:
  - [ ] Implement loading states: idle, downloading, compiling, executing
  - [ ] Add progress tracking for downloads
  - [ ] Show descriptive messages:
    - First run: "Downloading Rust toolchain (21 MB)... This only happens once."
    - Subsequent: "Compiling Rust code..."
  - [ ] Add circular progress indicator
  - [ ] Implement proper ARIA attributes

### Performance Dashboard

- [ ] Create `playground/src/components/InsightDashboard.tsx`:
  - [ ] Add tabs: Output, Performance, Energy, Deep Dive
  - [ ] Implement side-by-side output comparison
  - [ ] Add semantic equivalence checking
  - [ ] Create performance bar charts
  - [ ] Add "X times faster" calculations
- [ ] Create `playground/src/components/DeepDiveView.tsx`:
  - [ ] Three-column synchronized scroll
  - [ ] Python → HIR → Rust mapping
  - [ ] Hover highlighting across columns

## Phase 6: Quality Monitoring

### Telemetry System

- [ ] Create `playground/src/lib/telemetry/quality-telemetry.ts`:
  - [ ] Implement `QualityTelemetry` class
  - [ ] Add code analysis for metrics:
    - [ ] Function count via regex
    - [ ] Loop/conditional detection
    - [ ] Nesting depth calculation
    - [ ] Type complexity detection
  - [ ] Implement batched sending (5s intervals)
  - [ ] Add `sendBeacon` for reliability
  - [ ] Include environment capture:
    ```typescript
    {
      browser: navigator.userAgent,
      viewport: { width, height },
      connection: navigator.connection?.effectiveType,
      deviceMemory: navigator.deviceMemory
    }
    ```

### Performance Monitoring

- [ ] Create `playground/src/lib/quality-monitor.ts`:
  - [ ] Use PerformanceObserver API
  - [ ] Track key metrics:
    - [ ] Time to First Meaningful Paint
    - [ ] Time to Interactive
    - [ ] Transpilation P95 latency
    - [ ] Memory high water mark
  - [ ] Implement quality gate evaluation
  - [ ] Add console warnings for violations

## Phase 7: Build & Optimization

### WASM Build Pipeline

- [ ] Create `playground/scripts/build-wasm.sh`:
  ```bash
  #!/bin/bash
  set -euo pipefail

  cd ../crates/depyler-wasm
  wasm-pack build --target web --out-dir ../../playground/public/wasm

  # Optimize with wasm-opt
  wasm-opt -Oz \
    -o ../../playground/public/wasm/optimized.wasm \
    ../../playground/public/wasm/depyler_bg.wasm

  # Compress
  gzip -9 < ../../playground/public/wasm/optimized.wasm > ../../playground/public/wasm/depyler.wasm.gz

  # Verify size
  SIZE_KB=$(du -k ../../playground/public/wasm/depyler.wasm.gz | cut -f1)
  if [ $SIZE_KB -gt 1500 ]; then
    echo "Error: WASM size ${SIZE_KB}KB exceeds 1500KB budget"
    exit 1
  fi
  ```
- [ ] Add pre-commit hook for WASM size check
- [ ] Implement deterministic builds verification

### Frontend Build

- [ ] Configure Terser for aggressive minification
- [ ] Set up code splitting for lazy loading:
  - [ ] Critical: Monaco, core UI
  - [ ] High: Pyodide, D3
  - [ ] Medium: Examples, sharing
- [ ] Implement service worker for offline support
- [ ] Add Brotli compression for static assets

## Phase 8: Testing & Validation

### Unit Tests

- [ ] WASM module tests:
  - [ ] Transpilation accuracy
  - [ ] Energy estimation precision
  - [ ] Error handling
- [ ] Frontend component tests:
  - [ ] Editor behavior
  - [ ] Worker communication
  - [ ] State management
- [ ] Quality scoring tests:
  - [ ] PMAT calculation accuracy
  - [ ] Threshold validation

### Integration Tests

- [ ] End-to-end transpilation flow
- [ ] Python execution verification
- [ ] Rust compilation and execution
- [ ] Performance benchmarks:
  ```typescript
  it("should transpile simple function in <50ms", async () => {
    const start = performance.now();
    await transpiler.transpile("def add(a: int, b: int) -> int: return a + b");
    expect(performance.now() - start).toBeLessThan(50);
  });
  ```

### Performance Validation

- [ ] Measure and verify:
  - [ ] TTFMP < 1000ms (fast connection)
  - [ ] TTI < 3000ms
  - [ ] WASM size < 1.5MB gzipped
  - [ ] Transpilation P95 < targets
- [ ] Run Lighthouse audits:
  - [ ] Performance score > 90
  - [ ] Accessibility score = 100
  - [ ] Best practices score = 100

## Phase 9: CI/CD Integration

### GitHub Actions

- [ ] Create `.github/workflows/playground-ci.yml`:
  - [ ] WASM build and size check
  - [ ] Frontend build and tests
  - [ ] PMAT quality analysis
  - [ ] Performance benchmarks
  - [ ] Lighthouse CI integration
- [ ] Add deployment workflow:
  - [ ] Build optimized assets
  - [ ] Deploy to CDN
  - [ ] Invalidate caches
  - [ ] Run smoke tests

### Quality Gates

- [ ] Enforce in CI:
  - [ ] PMAT TDG score < 2.0
  - [ ] Test coverage > 85%
  - [ ] No cyclomatic complexity > 15
  - [ ] WASM size budget adherence
  - [ ] Performance budget compliance

## Phase 10: Production Deployment

### Infrastructure

- [ ] Configure nginx with:
  - [ ] HTTP/2 and Brotli
  - [ ] Security headers
  - [ ] WASM-specific caching
  - [ ] Rate limiting for API
- [ ] Set up CDN distribution:
  - [ ] WASM files with immutable caching
  - [ ] Static assets with fingerprinting
  - [ ] Geographic distribution
- [ ] Implement monitoring:
  - [ ] Real User Monitoring (RUM)
  - [ ] Error tracking
  - [ ] Performance dashboards

### Documentation

- [ ] Write user guide
- [ ] Document architecture
- [ ] Create troubleshooting guide
- [ ] Add performance tuning guide

## Verification Criteria

Each phase must meet these criteria before proceeding:

1. **Code Quality**: PMAT TDG score < 2.0
2. **Performance**: Meets specified latency targets
3. **Size**: WASM bundle < 1.5MB gzipped
4. **Testing**: >85% code coverage
5. **Accessibility**: WCAG 2.1 AA compliance
6. **Security**: No critical vulnerabilities

## Success Metrics

The implementation is complete when:

- [ ] Zero-configuration playground loads in <3s
- [ ] Transpilation latency <100ms for typical code
- [ ] Energy savings visualization is accurate ±10%
- [ ] User can understand value proposition in <15s
- [ ] All quality gates pass in CI
- [ ] Production deployment handles 1000 concurrent users
