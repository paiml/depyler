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

export interface PlaygroundState {
  pythonCode: string;
  rustCode: string;
  isTranspiling: boolean;
  isExecuting: boolean;
  transpileResult: TranspileResult | null;
  executionResult: ExecutionResult | null;
  errors: string[];
  warnings: string[];
  metrics: PlaygroundMetrics | null;
  pmatScore: PmatScore | null;
}

export interface TranspileResult {
  success: boolean;
  rust_code: string;
  errors: string[];
  warnings: string[];
  transpile_time_ms: number;
  memory_usage_mb: number;
  energy_estimate: EnergyEstimate;
  quality_metrics: QualityMetrics;
}

export interface ExecutionResult {
  python: {
    output: string;
    error: string | null;
    execution_time_ms: number;
  };
  rust: {
    output: string;
    error: string | null;
    execution_time_ms: number;
    compilation_time_ms: number;
  };
  energy_savings_percent: number;
}

export interface QualityMetrics {
  pmat_score: number;
  productivity: number;
  maintainability: number;
  accessibility: number;
  testability: number;
  code_complexity: number;
  cyclomatic_complexity: number;
}

export interface PlaygroundMetrics {
  page_load: PageLoadMetrics;
  transpilation: TranspilationMetrics;
  execution: ExecutionMetrics;
  quality_events: QualityEvent[];
}

export interface PageLoadMetrics {
  ttfmp_ms: number;
  tti_ms: number;
  wasm_load_ms: number;
  wasm_size_kb: number;
}

export interface TranspilationMetrics {
  latency_p95_ms: number;
  complexity_bucket: "Simple" | "Medium" | "Complex";
  cache_hit_rate: number;
  error_rate: number;
}

export interface ExecutionMetrics {
  rust_execution_ms: number;
  python_execution_ms: number;
  energy_savings_percent: number;
  memory_usage_mb: number;
}

export interface QualityEvent {
  timestamp: number;
  event_type: QualityEventType;
  severity: "Info" | "Warning" | "Critical";
  message: string;
  metrics_snapshot?: PmatScore;
}

export type QualityEventType =
  | "PerformanceRegression"
  | "PerformanceImprovement"
  | "ErrorThresholdExceeded"
  | "CacheEfficiencyDrop"
  | "EnergyEfficiencyImprovement";

export interface PmatScore {
  productivity: number;
  maintainability: number;
  accessibility: number;
  testability: number;
  tdg: number;
  timestamp: number;
}

export interface LoadingState {
  type: "idle" | "downloading" | "compiling" | "executing";
  progress?: number;
  message?: string;
}

export interface AnnotationSuggestion {
  line: number;
  column: number;
  annotation_type: string;
  description: string;
  example: string;
  impact: "low" | "medium" | "high";
}

export interface AntiPattern {
  line: number;
  column: number;
  pattern: string;
  description: string;
  severity: "info" | "warning" | "error";
  suggestion?: string;
}

export interface StaticAnalysis {
  complexity: number;
  cyclomatic_complexity: number;
  functions: FunctionInfo[];
  imports: string[];
  suggestions: OptimizationSuggestion[];
  anti_patterns: AntiPattern[];
}

export interface FunctionInfo {
  name: string;
  line_start: number;
  line_end: number;
  complexity: number;
  parameters: string[];
  return_type?: string;
}

export interface OptimizationSuggestion {
  line: number;
  column: number;
  message: string;
  suggestion_type: string;
  confidence: number;
}

export interface BenchmarkResult {
  iterations: number;
  times_ms: number[];
  min_ms: number;
  max_ms: number;
  mean_ms: number;
  median_ms: number;
  std_dev_ms: number;
}
