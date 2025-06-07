import "@testing-library/jest-dom";
import { afterEach, beforeAll, vi } from "vitest";
import { cleanup } from "@testing-library/react";
import React from "react";

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
    RuntimeError: Error,
  } as any;

  // Mock Performance API
  global.performance = {
    ...global.performance,
    mark: vi.fn(),
    measure: vi.fn(),
    getEntriesByName: vi.fn(() => [{ duration: 100 }]),
    now: vi.fn(() => Date.now()),
  };

  // Mock ResizeObserver
  global.ResizeObserver = vi.fn(() => ({
    observe: vi.fn(),
    unobserve: vi.fn(),
    disconnect: vi.fn(),
  }));

  // Mock Monaco Editor
  vi.mock("@monaco-editor/react", () => ({
    default: vi.fn(({ value, onChange }) => {
      return React.createElement("textarea", {
        value,
        onChange: (e: any) => onChange?.(e.target.value),
        "data-testid": "monaco-editor",
      });
    }),
    Editor: vi.fn(({ value, onChange }) => {
      return React.createElement("textarea", {
        value,
        onChange: (e: any) => onChange?.(e.target.value),
        "data-testid": "monaco-editor",
      });
    }),
    DiffEditor: vi.fn(() => React.createElement("div", { "data-testid": "monaco-diff-editor" })),
  }));

  // Mock Monaco API
  global.monaco = {
    languages: {
      setMonarchTokenizer: vi.fn(),
      registerCompletionItemProvider: vi.fn(),
      registerInlayHintsProvider: vi.fn(),
      setLanguageConfiguration: vi.fn(),
      register: vi.fn(),
      registerCodeActionProvider: vi.fn(),
      registerCodeLensProvider: vi.fn(),
      registerHoverProvider: vi.fn(),
      registerSignatureHelpProvider: vi.fn(),
      registerDefinitionProvider: vi.fn(),
      registerReferenceProvider: vi.fn(),
      registerDocumentHighlightProvider: vi.fn(),
      registerDocumentSymbolProvider: vi.fn(),
      registerOnTypeFormattingEditProvider: vi.fn(),
      registerDocumentFormattingEditProvider: vi.fn(),
      registerDocumentRangeFormattingEditProvider: vi.fn(),
      registerRenameProvider: vi.fn(),
      registerFoldingRangeProvider: vi.fn(),
      registerTypeDefinitionProvider: vi.fn(),
      registerImplementationProvider: vi.fn(),
      registerDeclarationProvider: vi.fn(),
      registerSelectionRangeProvider: vi.fn(),
      registerCallHierarchyProvider: vi.fn(),
      registerLinkedEditingRangeProvider: vi.fn(),
      registerDocumentSemanticTokensProvider: vi.fn(),
      registerDocumentRangeSemanticTokensProvider: vi.fn(),
      registerColorProvider: vi.fn(),
      registerInlineCompletionsProvider: vi.fn(),
      registerEvaluatableExpressionProvider: vi.fn(),
      CompletionItemKind: {
        Text: 0,
        Method: 1,
        Function: 2,
        Constructor: 3,
        Field: 4,
        Variable: 5,
        Class: 6,
        Interface: 7,
        Module: 8,
        Property: 9,
        Unit: 10,
        Value: 11,
        Enum: 12,
        Keyword: 13,
        Snippet: 14,
        Color: 15,
        File: 16,
        Reference: 17,
        Folder: 18,
        EnumMember: 19,
        Constant: 20,
        Struct: 21,
        Event: 22,
        Operator: 23,
        TypeParameter: 24,
      },
      getLanguages: vi.fn(() => []),
      setTokensProvider: vi.fn(),
    },
    editor: {
      create: vi.fn(),
      defineTheme: vi.fn(),
      setTheme: vi.fn(),
      setModelLanguage: vi.fn(),
      createModel: vi.fn(() => ({
        getValue: vi.fn(() => ""),
        setValue: vi.fn(),
        getLineContent: vi.fn(() => ""),
        getLineCount: vi.fn(() => 1),
        getWordAtPosition: vi.fn(() => null),
        getWordUntilPosition: vi.fn(() => ({ word: "", startColumn: 1, endColumn: 1 })),
      })),
      getModel: vi.fn(() => null),
      getModels: vi.fn(() => []),
    },
    KeyCode: {
      Tab: 9,
      Enter: 13,
    },
    KeyMod: {
      CtrlCmd: 1,
    },
    Range: vi.fn((startLine, startColumn, endLine, endColumn) => ({
      startLineNumber: startLine,
      startColumn,
      endLineNumber: endLine,
      endColumn,
    })),
    Position: vi.fn((line, column) => ({ lineNumber: line, column })),
  };

  // Mock D3
  vi.mock("d3", () => ({
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
                  attrTween: vi.fn(),
                })),
              })),
            })),
          })),
        })),
        remove: vi.fn(),
      })),
    })),
    scaleLinear: vi.fn(() => ({
      domain: vi.fn(() => ({ range: vi.fn(() => ({})) })),
    })),
    scaleSequential: vi.fn(() => ({
      domain: vi.fn(() => ({ interpolator: vi.fn(() => ({})) })),
    })),
    interpolateRdYlGn: vi.fn(),
    arc: vi.fn(() => vi.fn()),
    easeCubicInOut: vi.fn(),
    interpolate: vi.fn(() => vi.fn()),
  }));

  // Mock Worker
  global.Worker = vi.fn(() => ({
    postMessage: vi.fn(),
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    terminate: vi.fn(),
  }));

  // Mock WASM Manager
  vi.mock("@/lib/wasm-manager", () => ({
    transpileCode: vi.fn(() =>
      Promise.resolve({
        success: true,
        rust_code: "fn test() {}",
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
          equivalentTo: "powering an LED for 1 second",
        },
        quality_metrics: {
          pmat_score: 0.85,
          productivity: 0.8,
          maintainability: 0.9,
          accessibility: 1.0,
          testability: 0.8,
          code_complexity: 2,
          cyclomatic_complexity: 3,
        },
      })
    ),
    analyzeCode: vi.fn(() => Promise.resolve({})),
    benchmarkCode: vi.fn(() => Promise.resolve({})),
    preloadWasm: vi.fn(() => Promise.resolve()),
    isWasmLoaded: vi.fn(() => true),
    getWasmInstance: vi.fn(() => ({
      transpile: vi.fn(() => "fn test() {}"),
      analyze: vi.fn(() => ({})),
    })),
  }));
});

// Global test utilities
export const mockWasmModule = {
  transpile: vi.fn(() => ({
    success: true,
    rust_code: "fn add(a: i32, b: i32) -> i32 { a + b }",
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
      equivalentTo: "powering an LED for 1 second",
    },
  })),
  analyze_code: vi.fn(() => ({
    suggestions: [],
    antiPatterns: [],
  })),
  benchmark: vi.fn(() => ({
    iterations: 5,
    average_ms: 25,
    min_ms: 20,
    max_ms: 30,
  })),
};

export const createMockPlaygroundStore = () => ({
  pythonCode: "def add(a: int, b: int) -> int:\n    return a + b",
  rustCode: "fn add(a: i32, b: i32) -> i32 { a + b }",
  isTranspiling: false,
  isExecuting: false,
  transpileResult: null,
  executionResult: null,
  errors: [],
  warnings: [],
  metrics: {
    transpile_time_ms: 25,
    energy_reduction: {
      joules: 0.001,
      wattsAverage: 1.0,
      co2Grams: 0.000475,
      breakdown: { cpu: 0.0008, memory: 0.0002 },
      confidence: 0.8,
      equivalentTo: "powering an LED for 1 second",
    },
  },
  pmatScore: null,
  setPythonCode: vi.fn(),
  setRustCode: vi.fn(),
  transpileCode: vi.fn().mockResolvedValue(undefined),
  executeCode: vi.fn().mockResolvedValue(undefined),
  clearErrors: vi.fn(),
  reset: vi.fn(),
  isToolchainCached: true,
  error: null,
});

// Mock fetch for API calls
global.fetch = vi.fn(() =>
  Promise.resolve({
    ok: true,
    json: () => Promise.resolve({}),
    text: () => Promise.resolve(""),
  })
) as any;
