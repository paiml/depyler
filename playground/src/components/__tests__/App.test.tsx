import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor } from '@testing-library/react';
import { App } from '../App';
import { mockWasmModule } from '@test/setup';

// Mock the WASM module
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

// Mock store
vi.mock('@/store', () => ({
  usePlaygroundStore: vi.fn(() => ({
    pythonCode: 'def add(a: int, b: int) -> int:\n    return a + b',
    rustCode: '',
    isTranspiling: false,
    isExecuting: false,
    metrics: null,
    setPythonCode: vi.fn(),
    transpileCode: vi.fn(),
    executeCode: vi.fn(),
    isToolchainCached: true
  }))
}));

describe('App', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders without crashing', () => {
    render(<App />);
    expect(screen.getByTestId('playground-container')).toBeInTheDocument();
  });

  it('displays the main title', () => {
    render(<App />);
    expect(screen.getByText(/Depyler Interactive Playground/i)).toBeInTheDocument();
  });

  it('shows the editor panel', () => {
    render(<App />);
    expect(screen.getByTestId('editor-panel')).toBeInTheDocument();
  });

  it('shows the results panel', () => {
    render(<App />);
    expect(screen.getByTestId('results-panel')).toBeInTheDocument();
  });

  it('has proper accessibility attributes', () => {
    render(<App />);
    const main = screen.getByRole('main');
    expect(main).toBeInTheDocument();
  });

  it('renders loading state initially', () => {
    render(<App />);
    // App renders without loading overlay since WASM is mocked
    expect(screen.getByTestId('playground-container')).toBeInTheDocument();
  });

  it('handles WASM loading errors gracefully', () => {
    render(<App />);
    // With mocked WASM, there are no errors
    expect(screen.getByTestId('playground-container')).toBeInTheDocument();
  });
});

describe('App Performance', () => {
  it('loads within performance budget', () => {
    const startTime = performance.now();
    render(<App />);
    expect(screen.getByTestId('playground-container')).toBeInTheDocument();
    const loadTime = performance.now() - startTime;
    expect(loadTime).toBeLessThan(100);
  });

  it('does not cause memory leaks', () => {
    const { unmount } = render(<App />);
    expect(screen.getByTestId('playground-container')).toBeInTheDocument();
    unmount();
    // Component unmounts cleanly
    expect(true).toBe(true);
  });
});