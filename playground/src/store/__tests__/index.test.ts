import { describe, it, expect, vi, beforeEach } from 'vitest';
import { renderHook, act } from '@testing-library/react';
import { usePlaygroundStore } from '../index';
import { mockWasmModule } from '@test/setup';

// Mock WASM manager
const mockWasmManager = {
  loadModule: vi.fn(() => Promise.resolve(mockWasmModule)),
  isLoaded: vi.fn(() => true),
  getModule: vi.fn(() => mockWasmModule)
};

vi.mock('@/lib/wasm-manager', () => ({
  WasmModuleManager: vi.fn(() => mockWasmManager)
}));

// Mock debounce utility
vi.mock('@/utils/debounce', () => ({
  debounce: vi.fn((fn: Function) => fn)
}));

describe('usePlaygroundStore', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    // Reset store state
    const { result } = renderHook(() => usePlaygroundStore());
    act(() => {
      result.current.setPythonCode('');
    });
  });

  describe('Initial State', () => {
    it('has correct initial state', () => {
      const { result } = renderHook(() => usePlaygroundStore());
      
      expect(result.current.pythonCode).toBe('');
      expect(result.current.rustCode).toBe('');
      expect(result.current.isTranspiling).toBe(false);
      expect(result.current.isExecuting).toBe(false);
      expect(result.current.metrics).toBeNull();
      expect(result.current.error).toBeNull();
      expect(result.current.isToolchainCached).toBe(false);
    });
  });

  describe('setPythonCode', () => {
    it('updates Python code', () => {
      const { result } = renderHook(() => usePlaygroundStore());
      
      act(() => {
        result.current.setPythonCode('def hello(): pass');
      });
      
      expect(result.current.pythonCode).toBe('def hello(): pass');
    });
    
    it('clears error when setting new code', () => {
      const { result } = renderHook(() => usePlaygroundStore());
      
      // Set an error first
      act(() => {
        (result.current as any).setError(new Error('Test error'));
      });
      
      expect(result.current.error).toBeTruthy();
      
      act(() => {
        result.current.setPythonCode('def new_code(): pass');
      });
      
      expect(result.current.error).toBeNull();
    });
  });

  describe('transpileCode', () => {
    it('transpiles code successfully', async () => {
      const { result } = renderHook(() => usePlaygroundStore());
      
      act(() => {
        result.current.setPythonCode('def add(a: int, b: int) -> int: return a + b');
      });
      
      await act(async () => {
        await result.current.transpileCode();
      });
      
      expect(result.current.isTranspiling).toBe(false);
      expect(result.current.rustCode).toBe('fn add(a: i32, b: i32) -> i32 { a + b }');
      expect(result.current.metrics).toBeDefined();
      expect(result.current.error).toBeNull();
    });
    
    it('sets transpiling state during operation', async () => {
      const { result } = renderHook(() => usePlaygroundStore());
      
      // Mock async WASM call
      mockWasmModule.transpile.mockImplementation(() => 
        new Promise(resolve => 
          setTimeout(() => resolve({
            success: true,
            rust_code: 'fn test() {}',
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
          }), 100)
        )
      );
      
      act(() => {
        result.current.setPythonCode('def test(): pass');
      });
      
      const transpilePromise = act(async () => {
        return result.current.transpileCode();
      });
      
      // Should be transpiling
      expect(result.current.isTranspiling).toBe(true);
      
      await transpilePromise;
      
      // Should be done transpiling
      expect(result.current.isTranspiling).toBe(false);
    });
    
    it('handles transpilation errors', async () => {
      const { result } = renderHook(() => usePlaygroundStore());
      
      mockWasmModule.transpile.mockImplementation(() => {
        throw new Error('Transpilation failed');
      });
      
      act(() => {
        result.current.setPythonCode('invalid syntax');
      });
      
      await act(async () => {
        await result.current.transpileCode();
      });
      
      expect(result.current.isTranspiling).toBe(false);
      expect(result.current.error).toBeTruthy();
      expect(result.current.rustCode).toBe('');
    });
    
    it('does not transpile empty code', async () => {
      const { result } = renderHook(() => usePlaygroundStore());
      
      await act(async () => {
        await result.current.transpileCode();
      });
      
      expect(mockWasmModule.transpile).not.toHaveBeenCalled();
      expect(result.current.rustCode).toBe('');
    });
  });

  describe('executeCode', () => {
    it('executes code successfully', async () => {
      const { result } = renderHook(() => usePlaygroundStore());
      
      // Set up successful transpilation first
      act(() => {
        result.current.setPythonCode('def add(a: int, b: int) -> int: return a + b');
      });
      
      await act(async () => {
        await result.current.transpileCode();
      });
      
      await act(async () => {
        await result.current.executeCode();
      });
      
      expect(result.current.isExecuting).toBe(false);
      expect(result.current.error).toBeNull();
    });
    
    it('sets executing state during operation', async () => {
      const { result } = renderHook(() => usePlaygroundStore());
      
      act(() => {
        result.current.setPythonCode('def test(): pass');
      });
      
      await act(async () => {
        await result.current.transpileCode();
      });
      
      const executePromise = act(async () => {
        return result.current.executeCode();
      });
      
      // Should be executing
      expect(result.current.isExecuting).toBe(true);
      
      await executePromise;
      
      // Should be done executing
      expect(result.current.isExecuting).toBe(false);
    });
    
    it('requires transpilation before execution', async () => {
      const { result } = renderHook(() => usePlaygroundStore());
      
      act(() => {
        result.current.setPythonCode('def test(): pass');
      });
      
      await act(async () => {
        await result.current.executeCode();
      });
      
      // Should have transpiled first
      expect(mockWasmModule.transpile).toHaveBeenCalled();
    });
  });

  describe('Optimistic Updates', () => {
    it('shows intermediate results during transpilation', async () => {
      const { result } = renderHook(() => usePlaygroundStore());
      
      act(() => {
        result.current.setPythonCode('def add(a: int, b: int) -> int: return a + b');
      });
      
      const transpilePromise = act(async () => {
        return result.current.transpileCode();
      });
      
      // Should show loading state
      expect(result.current.isTranspiling).toBe(true);
      expect(result.current.error).toBeNull();
      
      await transpilePromise;
      
      expect(result.current.isTranspiling).toBe(false);
      expect(result.current.rustCode).toBeDefined();
    });
  });

  describe('Error Handling', () => {
    it('clears previous errors on new operations', async () => {
      const { result } = renderHook(() => usePlaygroundStore());
      
      // Cause an error
      mockWasmModule.transpile.mockImplementationOnce(() => {
        throw new Error('First error');
      });
      
      act(() => {
        result.current.setPythonCode('bad code');
      });
      
      await act(async () => {
        await result.current.transpileCode();
      });
      
      expect(result.current.error).toBeTruthy();
      
      // Fix the mock and try again
      mockWasmModule.transpile.mockImplementationOnce(() => ({
        success: true,
        rust_code: 'fn good() {}',
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
      }));
      
      act(() => {
        result.current.setPythonCode('def good(): pass');
      });
      
      await act(async () => {
        await result.current.transpileCode();
      });
      
      expect(result.current.error).toBeNull();
    });
  });

  describe('State Persistence', () => {
    it('maintains state across multiple operations', async () => {
      const { result } = renderHook(() => usePlaygroundStore());
      
      // First operation
      act(() => {
        result.current.setPythonCode('def first(): pass');
      });
      
      await act(async () => {
        await result.current.transpileCode();
      });
      
      const firstRustCode = result.current.rustCode;
      const firstMetrics = result.current.metrics;
      
      // Second operation
      act(() => {
        result.current.setPythonCode('def second(): pass');
      });
      
      await act(async () => {
        await result.current.transpileCode();
      });
      
      // Should have new results
      expect(result.current.rustCode).not.toBe(firstRustCode);
      expect(result.current.metrics).not.toBe(firstMetrics);
    });
  });

  describe('Performance', () => {
    it('handles rapid code changes efficiently', async () => {
      const { result } = renderHook(() => usePlaygroundStore());
      
      const startTime = performance.now();
      
      // Simulate rapid typing
      for (let i = 0; i < 10; i++) {
        act(() => {
          result.current.setPythonCode(`def func_${i}(): pass`);
        });
      }
      
      const updateTime = performance.now() - startTime;
      
      expect(updateTime).toBeLessThan(50); // Should handle updates within 50ms
    });
    
    it('debounces transpilation calls', async () => {
      const { result } = renderHook(() => usePlaygroundStore());
      
      // Multiple rapid code changes
      act(() => {
        result.current.setPythonCode('def a(): pass');
      });
      
      act(() => {
        result.current.setPythonCode('def ab(): pass');
      });
      
      act(() => {
        result.current.setPythonCode('def abc(): pass');
      });
      
      await act(async () => {
        await result.current.transpileCode();
      });
      
      // Should only transpile the final version
      expect(mockWasmModule.transpile).toHaveBeenCalledTimes(1);
    });
  });
});