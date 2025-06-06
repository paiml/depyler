import '@testing-library/jest-dom';
import { afterEach, beforeAll, vi } from 'vitest';
import { cleanup } from '@testing-library/react';
import React from 'react';

// Cleanup after each test case
afterEach(() => {
  cleanup();
});

// Mock WASM module for tests
beforeAll(() => {
  // Mock WebAssembly for JSDOM environment
  global.WebAssembly = {
    compile: vi.fn(() => Promise.resolve({})),
    compileStreaming: vi.fn(() => Promise.resolve({})),
    instantiate: vi.fn(() => Promise.resolve({ instance: {}, module: {} })),
    instantiateStreaming: vi.fn(() => Promise.resolve({ instance: {}, module: {} })),
    validate: vi.fn(() => true),
    Module: vi.fn(),
    Instance: vi.fn(),
    Memory: vi.fn(),
    Table: vi.fn(),
    Global: vi.fn(),
    CompileError: Error,
    LinkError: Error,
    RuntimeError: Error
  } as any;

  // Mock Performance API
  global.performance = {
    ...global.performance,
    mark: vi.fn(),
    measure: vi.fn(),
    getEntriesByName: vi.fn(() => [{ duration: 100 }]),
    now: vi.fn(() => Date.now())
  };

  // Mock ResizeObserver
  global.ResizeObserver = vi.fn(() => ({
    observe: vi.fn(),
    unobserve: vi.fn(),
    disconnect: vi.fn()
  }));

  // Mock Monaco Editor
  vi.mock('@monaco-editor/react', () => ({
    default: vi.fn(({ value, onChange }) => {
      return React.createElement('textarea', {
        value,
        onChange: (e: any) => onChange?.(e.target.value),
        'data-testid': 'monaco-editor'
      });
    }),
    Editor: vi.fn(({ value, onChange }) => {
      return React.createElement('textarea', {
        value,
        onChange: (e: any) => onChange?.(e.target.value),
        'data-testid': 'monaco-editor'
      });
    }),
    DiffEditor: vi.fn(() => React.createElement('div', { 'data-testid': 'monaco-diff-editor' }))
  }));

  // Mock Monaco API
  global.monaco = {
    languages: {
      setMonarchTokenizer: vi.fn(),
      registerCompletionItemProvider: vi.fn(),
      registerInlayHintsProvider: vi.fn(),
      setLanguageConfiguration: vi.fn(),
      register: vi.fn()
    },
    editor: {
      create: vi.fn(),
      defineTheme: vi.fn(),
      setTheme: vi.fn()
    },
    KeyCode: {
      Tab: 9,
      Enter: 13
    },
    KeyMod: {
      CtrlCmd: 1
    }
  };

  // Mock D3
  vi.mock('d3', () => ({
    select: vi.fn(() => ({
      selectAll: vi.fn(() => ({
        data: vi.fn(() => ({
          join: vi.fn(() => ({
            attr: vi.fn(() => ({ style: vi.fn() })),
            style: vi.fn(),
            text: vi.fn(),
            transition: vi.fn(() => ({
              duration: vi.fn(() => ({
                ease: vi.fn(() => ({
                  attrTween: vi.fn()
                }))
              }))
            }))
          }))
        }))
      }))
    })),
    scaleLinear: vi.fn(() => ({
      domain: vi.fn(() => ({ range: vi.fn(() => ({})) }))
    })),
    scaleSequential: vi.fn(() => ({
      domain: vi.fn(() => ({ interpolator: vi.fn(() => ({})) }))
    })),
    interpolateRdYlGn: vi.fn(),
    arc: vi.fn(() => vi.fn()),
    easeCubicInOut: vi.fn(),
    interpolate: vi.fn(() => vi.fn())
  }));

  // Mock Worker
  global.Worker = vi.fn(() => ({
    postMessage: vi.fn(),
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    terminate: vi.fn()
  }));

  // Mock WASM Manager
  vi.mock('@/lib/wasm-manager', () => ({
    transpileCode: vi.fn(() => Promise.resolve({
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
    })),
    analyzeCode: vi.fn(() => Promise.resolve({})),
    benchmarkCode: vi.fn(() => Promise.resolve({})),
    preloadWasm: vi.fn(() => Promise.resolve())
  }));
});

// Global test utilities
export const mockWasmModule = {
  transpile: vi.fn(() => ({
    success: true,
    rust_code: 'fn add(a: i32, b: i32) -> i32 { a + b }',
    parse_time_ms: 10,
    transpile_time_ms: 25,
    ast_nodes: 5,
    complexity_score: 1,
    energy_reduction: {
      joules: 0.001,
      wattsAverage: 1.0,
      co2Grams: 0.000475,
      breakdown: { cpu: 0.0008, memory: 0.0002 },
      confidence: 0.8,
      equivalentTo: 'powering an LED for 1 second'
    }
  })),
  analyze_code: vi.fn(() => ({
    suggestions: [],
    antiPatterns: []
  })),
  benchmark: vi.fn(() => ({
    iterations: 5,
    average_ms: 25,
    min_ms: 20,
    max_ms: 30
  }))
};

export const createMockPlaygroundStore = () => ({
  pythonCode: 'def add(a: int, b: int) -> int:\n    return a + b',
  rustCode: 'fn add(a: i32, b: i32) -> i32 { a + b }',
  isTranspiling: false,
  isExecuting: false,
  metrics: {
    transpile_time_ms: 25,
    energy_reduction: {
      joules: 0.001,
      wattsAverage: 1.0,
      co2Grams: 0.000475,
      breakdown: { cpu: 0.0008, memory: 0.0002 },
      confidence: 0.8,
      equivalentTo: 'powering an LED for 1 second'
    }
  },
  setPythonCode: vi.fn(),
  transpileCode: vi.fn(),
  executeCode: vi.fn(),
  isToolchainCached: true
});

// Mock fetch for API calls
global.fetch = vi.fn(() =>
  Promise.resolve({
    ok: true,
    json: () => Promise.resolve({}),
    text: () => Promise.resolve('')
  })
) as any;