import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { App } from '../../components/App';
import { mockWasmModule } from '@test/setup';

// Mock the entire WASM module for integration testing
const mockWasmManager = {
  loadModule: vi.fn(),
  isLoaded: vi.fn(() => true),
  getModule: vi.fn(() => mockWasmModule)
};

vi.mock('@/lib/wasm-manager', () => ({
  WasmModuleManager: vi.fn(() => mockWasmManager)
}));

// Mock worker for execution sandbox
class MockExecutionWorker {
  onmessage: ((event: MessageEvent) => void) | null = null;
  
  postMessage(message: any): void {
    setTimeout(() => {
      if (this.onmessage) {
        this.onmessage({
          data: {
            id: message.id,
            type: 'EXECUTION_RESULT',
            result: {
              success: true,
              stdout: 'Execution successful',
              stderr: '',
              executionTimeMs: 15,
              memoryUsageMb: 2.5,
              energyEstimate: {
                joules: 0.0005,
                wattsAverage: 0.5,
                co2Grams: 0.0002,
                breakdown: { cpu: 0.0004, memory: 0.0001 },
                confidence: 0.9
              }
            }
          }
        } as MessageEvent);
      }
    }, 50); // Simulate realistic execution time
  }
  
  addEventListener(type: string, listener: EventListener): void {
    if (type === 'message') {
      this.onmessage = listener as (event: MessageEvent) => void;
    }
  }
  
  removeEventListener(): void {}
  terminate(): void {}
}

global.Worker = MockExecutionWorker as any;

describe('Transpilation Flow Integration', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    
    // Reset WASM module mocks
    mockWasmModule.transpile.mockResolvedValue({
      success: true,
      rust_code: 'fn add(a: i32, b: i32) -> i32 { a + b }',
      parse_time_ms: 8,
      transpile_time_ms: 22,
      ast_nodes: 7,
      complexity_score: 1,
      energy_reduction: {
        joules: 0.001,
        wattsAverage: 1.0,
        co2Grams: 0.000475,
        breakdown: { cpu: 0.0008, memory: 0.0002 },
        confidence: 0.85,
        equivalentTo: 'powering an LED for 1 second'
      }
    });
    
    mockWasmModule.analyze_code.mockResolvedValue({
      suggestions: [
        {
          type: 'optimization',
          message: 'Consider using type hints for better performance',
          impact: 'medium',
          line: 1
        }
      ],
      antiPatterns: []
    });
    
    mockWasmModule.benchmark.mockResolvedValue({
      iterations: 5,
      average_ms: 22,
      min_ms: 18,
      max_ms: 28,
      standard_deviation: 3.2
    });
  });

  describe('Complete Transpilation Workflow', () => {
    it('performs end-to-end transpilation successfully', async () => {
      const user = userEvent.setup();
      render(<App />);
      
      // Wait for app to load
      await waitFor(() => {
        expect(screen.getByTestId('playground-container')).toBeInTheDocument();
      });
      
      // Find and interact with code editor
      const editor = screen.getByTestId('monaco-editor');
      expect(editor).toBeInTheDocument();
      
      // Input Python code
      const pythonCode = 'def add(a: int, b: int) -> int:\n    return a + b';
      await user.clear(editor);
      await user.type(editor, pythonCode);
      
      // Trigger transpilation
      const transpileButton = screen.getByRole('button', { name: /transpile/i });
      await user.click(transpileButton);
      
      // Wait for transpilation to complete
      await waitFor(() => {
        expect(screen.getByTestId('rust-output')).toBeInTheDocument();
      }, { timeout: 3000 });
      
      // Verify results are displayed
      expect(screen.getByText(/fn add\(a: i32, b: i32\) -> i32/)).toBeInTheDocument();
      expect(screen.getByTestId('transpilation-metrics')).toBeInTheDocument();
      expect(screen.getByTestId('energy-gauge')).toBeInTheDocument();
      
      // Verify WASM module was called correctly
      expect(mockWasmModule.transpile).toHaveBeenCalledWith(
        pythonCode,
        expect.any(Object)
      );
    });
    
    it('shows transpilation progress indicators', async () => {
      const user = userEvent.setup();
      
      // Mock slower transpilation
      mockWasmModule.transpile.mockImplementation(() => 
        new Promise(resolve => 
          setTimeout(() => resolve({
            success: true,
            rust_code: 'fn slow() {}',
            parse_time_ms: 100,
            transpile_time_ms: 200,
            ast_nodes: 5,
            complexity_score: 1,
            energy_reduction: {
              joules: 0.002,
              wattsAverage: 2.0,
              co2Grams: 0.00095,
              breakdown: { cpu: 0.0016, memory: 0.0004 },
              confidence: 0.8,
              equivalentTo: 'powering an LED for 2 seconds'
            }
          }), 500)
        )
      );
      
      render(<App />);
      
      await waitFor(() => {
        expect(screen.getByTestId('playground-container')).toBeInTheDocument();
      });
      
      const editor = screen.getByTestId('monaco-editor');
      await user.type(editor, 'def slow_function(): pass');
      
      const transpileButton = screen.getByRole('button', { name: /transpile/i });
      await user.click(transpileButton);
      
      // Should show loading state
      expect(screen.getByTestId('transpilation-loading')).toBeInTheDocument();
      expect(transpileButton).toBeDisabled();
      
      // Wait for completion
      await waitFor(() => {
        expect(screen.queryByTestId('transpilation-loading')).not.toBeInTheDocument();
        expect(transpileButton).toBeEnabled();
      }, { timeout: 1000 });
    });
    
    it('handles transpilation errors gracefully', async () => {
      const user = userEvent.setup();
      
      // Mock transpilation failure
      mockWasmModule.transpile.mockRejectedValue(new Error('Syntax error in Python code'));
      
      render(<App />);
      
      await waitFor(() => {
        expect(screen.getByTestId('playground-container')).toBeInTheDocument();
      });
      
      const editor = screen.getByTestId('monaco-editor');
      await user.type(editor, 'def broken(:\n    pass');
      
      const transpileButton = screen.getByRole('button', { name: /transpile/i });
      await user.click(transpileButton);
      
      // Should show error message
      await waitFor(() => {
        expect(screen.getByTestId('error-display')).toBeInTheDocument();
        expect(screen.getByText(/syntax error/i)).toBeInTheDocument();
      });
      
      // Should not show Rust output
      expect(screen.queryByTestId('rust-output')).not.toBeInTheDocument();
    });
  });

  describe('Code Analysis Integration', () => {
    it('provides real-time code analysis feedback', async () => {
      const user = userEvent.setup();
      render(<App />);
      
      await waitFor(() => {
        expect(screen.getByTestId('playground-container')).toBeInTheDocument();
      });
      
      const editor = screen.getByTestId('monaco-editor');
      
      // Type code that should trigger analysis
      await user.type(editor, 'def analyze_me(data):\n    eval(data)');
      
      // Wait for analysis to complete (debounced)
      await waitFor(() => {
        expect(mockWasmModule.analyze_code).toHaveBeenCalled();
      }, { timeout: 1000 });
      
      // Should show analysis results
      expect(screen.getByTestId('code-analysis')).toBeInTheDocument();
    });
    
    it('updates analysis when code changes', async () => {
      const user = userEvent.setup();
      render(<App />);
      
      await waitFor(() => {
        expect(screen.getByTestId('playground-container')).toBeInTheDocument();
      });
      
      const editor = screen.getByTestId('monaco-editor');
      
      // Initial code
      await user.type(editor, 'def first(): pass');
      
      await waitFor(() => {
        expect(mockWasmModule.analyze_code).toHaveBeenCalledTimes(1);
      });
      
      // Change code
      await user.clear(editor);
      await user.type(editor, 'def second(): return 42');
      
      await waitFor(() => {
        expect(mockWasmModule.analyze_code).toHaveBeenCalledTimes(2);
      });
    });
  });

  describe('Performance Benchmarking Integration', () => {
    it('runs performance benchmarks after transpilation', async () => {
      const user = userEvent.setup();
      render(<App />);
      
      await waitFor(() => {
        expect(screen.getByTestId('playground-container')).toBeInTheDocument();
      });
      
      const editor = screen.getByTestId('monaco-editor');
      await user.type(editor, 'def benchmark_me(): return sum(range(100))');
      
      const transpileButton = screen.getByRole('button', { name: /transpile/i });
      await user.click(transpileButton);
      
      // Wait for transpilation and benchmarking
      await waitFor(() => {
        expect(screen.getByTestId('performance-metrics')).toBeInTheDocument();
      });
      
      // Verify benchmark was called
      expect(mockWasmModule.benchmark).toHaveBeenCalled();
      
      // Should show benchmark results
      expect(screen.getByText(/average.*22.*ms/i)).toBeInTheDocument();
    });
  });

  describe('Energy Visualization Integration', () => {
    it('displays energy savings visualization', async () => {
      const user = userEvent.setup();
      render(<App />);
      
      await waitFor(() => {
        expect(screen.getByTestId('playground-container')).toBeInTheDocument();
      });
      
      const editor = screen.getByTestId('monaco-editor');
      await user.type(editor, 'def energy_efficient(): return 42');
      
      const transpileButton = screen.getByRole('button', { name: /transpile/i });
      await user.click(transpileButton);
      
      await waitFor(() => {
        expect(screen.getByTestId('energy-gauge')).toBeInTheDocument();
      });
      
      // Should show energy metrics
      expect(screen.getByTestId('energy-breakdown')).toBeInTheDocument();
      expect(screen.getByText(/powering an led/i)).toBeInTheDocument();
    });
    
    it('updates energy visualization with code changes', async () => {
      const user = userEvent.setup();
      render(<App />);
      
      await waitFor(() => {
        expect(screen.getByTestId('playground-container')).toBeInTheDocument();
      });
      
      const editor = screen.getByTestId('monaco-editor');
      
      // First code version
      await user.type(editor, 'def simple(): return 1');
      
      const transpileButton = screen.getByRole('button', { name: /transpile/i });
      await user.click(transpileButton);
      
      await waitFor(() => {
        expect(screen.getByTestId('energy-gauge')).toBeInTheDocument();
      });
      
      const firstEnergyReading = screen.getByTestId('energy-value').textContent;
      
      // Change to more complex code
      mockWasmModule.transpile.mockResolvedValueOnce({
        success: true,
        rust_code: 'fn complex() { for i in 0..1000 { println!("{}", i); } }',
        parse_time_ms: 15,
        transpile_time_ms: 45,
        ast_nodes: 20,
        complexity_score: 3,
        energy_reduction: {
          joules: 0.005,
          wattsAverage: 5.0,
          co2Grams: 0.002375,
          breakdown: { cpu: 0.004, memory: 0.001 },
          confidence: 0.9,
          equivalentTo: 'powering an LED for 5 seconds'
        }
      });
      
      await user.clear(editor);
      await user.type(editor, 'def complex(): [print(i) for i in range(1000)]');
      await user.click(transpileButton);
      
      await waitFor(() => {
        const newEnergyReading = screen.getByTestId('energy-value').textContent;
        expect(newEnergyReading).not.toBe(firstEnergyReading);
      });
    });
  });

  describe('Execution Flow Integration', () => {
    it('executes transpiled code successfully', async () => {
      const user = userEvent.setup();
      render(<App />);
      
      await waitFor(() => {
        expect(screen.getByTestId('playground-container')).toBeInTheDocument();
      });
      
      // Transpile first
      const editor = screen.getByTestId('monaco-editor');
      await user.type(editor, 'def execute_me(): return "Hello, World!"');
      
      const transpileButton = screen.getByRole('button', { name: /transpile/i });
      await user.click(transpileButton);
      
      await waitFor(() => {
        expect(screen.getByTestId('rust-output')).toBeInTheDocument();
      });
      
      // Then execute
      const executeButton = screen.getByRole('button', { name: /run comparison/i });
      await user.click(executeButton);
      
      await waitFor(() => {
        expect(screen.getByTestId('execution-results')).toBeInTheDocument();
      });
      
      // Should show execution output
      expect(screen.getByText(/execution successful/i)).toBeInTheDocument();
      expect(screen.getByTestId('execution-metrics')).toBeInTheDocument();
    });
  });

  describe('Error Recovery Integration', () => {
    it('recovers from errors and continues working', async () => {
      const user = userEvent.setup();
      render(<App />);
      
      await waitFor(() => {
        expect(screen.getByTestId('playground-container')).toBeInTheDocument();
      });
      
      const editor = screen.getByTestId('monaco-editor');
      
      // First, cause an error
      mockWasmModule.transpile.mockRejectedValueOnce(new Error('Transpilation failed'));
      
      await user.type(editor, 'def broken(: pass');
      
      const transpileButton = screen.getByRole('button', { name: /transpile/i });
      await user.click(transpileButton);
      
      await waitFor(() => {
        expect(screen.getByTestId('error-display')).toBeInTheDocument();
      });
      
      // Then fix the code and try again
      mockWasmModule.transpile.mockResolvedValueOnce({
        success: true,
        rust_code: 'fn fixed() -> i32 { 42 }',
        parse_time_ms: 5,
        transpile_time_ms: 15,
        ast_nodes: 3,
        complexity_score: 1,
        energy_reduction: {
          joules: 0.0005,
          wattsAverage: 0.5,
          co2Grams: 0.0002,
          breakdown: { cpu: 0.0004, memory: 0.0001 },
          confidence: 0.95,
          equivalentTo: 'powering an LED for 0.5 seconds'
        }
      });
      
      await user.clear(editor);
      await user.type(editor, 'def fixed(): return 42');
      await user.click(transpileButton);
      
      await waitFor(() => {
        expect(screen.queryByTestId('error-display')).not.toBeInTheDocument();
        expect(screen.getByTestId('rust-output')).toBeInTheDocument();
      });
    });
  });
});