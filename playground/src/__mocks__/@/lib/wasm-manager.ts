import { vi } from 'vitest';

export const transpileCode = vi.fn(() => Promise.resolve({
  success: true,
  rust_code: 'fn test() {}',
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
    equivalentTo: 'powering an LED for 1 second'
  },
  quality_metrics: {
    pmat_score: 0.85,
    productivity: 0.8,
    maintainability: 0.9,
    accessibility: 1.0,
    testability: 0.8,
    code_complexity: 2,
    cyclomatic_complexity: 3
  }
}));

export const analyzeCode = vi.fn(() => Promise.resolve({}));

export const benchmarkCode = vi.fn(() => Promise.resolve({}));

export const preloadWasm = vi.fn(() => Promise.resolve());