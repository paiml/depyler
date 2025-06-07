import { create } from "zustand";
import { ExecutionResult, PlaygroundState, TranspileResult } from "@/types";

interface PlaygroundActions {
  setPythonCode: (code: string) => void;
  setRustCode: (code: string) => void;
  transpileCode: () => Promise<void>;
  executeCode: () => Promise<void>;
  clearErrors: () => void;
  reset: () => void;
}

type PlaygroundStore = PlaygroundState & PlaygroundActions;

const initialState: PlaygroundState = {
  pythonCode: `# @depyler: optimize_energy=true
def calculate_fibonacci(n: int) -> int:
    """Calculate the nth Fibonacci number efficiently."""
    if n <= 1:
        return n
    
    a, b = 0, 1
    for i in range(2, n + 1):
        a, b = b, a + b
    
    return b

# Example usage
result = calculate_fibonacci(20)
print(f"The 20th Fibonacci number is: {result}")`,
  rustCode: "",
  isTranspiling: false,
  isExecuting: false,
  transpileResult: null,
  executionResult: null,
  errors: [],
  warnings: [],
  metrics: null,
  pmatScore: null,
};

// Debounced transpilation function
let transpileTimeout: NodeJS.Timeout | null = null;

export const usePlaygroundStore = create<PlaygroundStore>((set, get) => ({
  ...initialState,

  setPythonCode: (code: string) => {
    set({ pythonCode: code });

    // Debounced auto-transpilation
    if (transpileTimeout) {
      clearTimeout(transpileTimeout);
    }

    transpileTimeout = setTimeout(() => {
      get().transpileCode();
    }, 300);
  },

  setRustCode: (code: string) => {
    set({ rustCode: code });
  },

  transpileCode: async () => {
    const { pythonCode } = get();
    if (!pythonCode.trim()) {
      set({ rustCode: "", transpileResult: null, errors: [], warnings: [] });
      return;
    }

    set({ isTranspiling: true, errors: [], warnings: [] });

    try {
      // Lazy load WASM module
      const wasmModule = await import("@/lib/wasm-manager");
      const result = await wasmModule.transpileCode(pythonCode, {
        verify: true,
        optimize: true,
        emit_docs: false,
        target_version: "1.83",
      });

      if (result.success) {
        set({
          rustCode: result.rust_code,
          transpileResult: result,
          errors: result.errors,
          warnings: result.warnings,
          isTranspiling: false,
        });
      } else {
        set({
          rustCode: "",
          transpileResult: result,
          errors: result.errors,
          warnings: result.warnings,
          isTranspiling: false,
        });
      }
    } catch (error) {
      set({
        errors: [error instanceof Error ? error.message : "Unknown transpilation error"],
        isTranspiling: false,
      });
    }
  },

  executeCode: async () => {
    const { pythonCode, rustCode } = get();
    if (!pythonCode.trim() || !rustCode.trim()) {
      return;
    }

    set({ isExecuting: true });

    try {
      // Execute both Python and Rust code
      const executionModule = await import("@/lib/execution-manager");
      const result = await executionModule.executeComparison(pythonCode, rustCode);

      set({
        executionResult: result,
        isExecuting: false,
      });
    } catch (error) {
      set({
        errors: [...get().errors, error instanceof Error ? error.message : "Execution failed"],
        isExecuting: false,
      });
    }
  },

  clearErrors: () => {
    set({ errors: [], warnings: [] });
  },

  reset: () => {
    set(initialState);
  },
}));
