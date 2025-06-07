import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { IntelliSensei } from "../intelli-sensei";
import { LRUCache } from "lru-cache";

// Mock LRU Cache
const mockLRUCache = {
  get: vi.fn(),
  set: vi.fn(),
  has: vi.fn(),
  delete: vi.fn(),
  clear: vi.fn(),
};

vi.mock("lru-cache", () => ({
  LRUCache: vi.fn(() => mockLRUCache),
}));

// Mock crypto.randomUUID if not available
if (!global.crypto) {
  global.crypto = {} as any;
}
if (!global.crypto.randomUUID) {
  global.crypto.randomUUID = vi.fn(() => `test-uuid-${Date.now()}-${Math.random()}`);
}

// Mock Monaco
const mockMonaco = {
  languages: {
    register: vi.fn(),
    registerCompletionItemProvider: vi.fn(),
    registerInlayHintsProvider: vi.fn(),
    registerCodeActionProvider: vi.fn(),
    setMonarchTokensProvider: vi.fn(),
    setMonarchTokenizer: vi.fn(),
    CompletionItemKind: {
      Property: 9,
      Function: 2,
      Variable: 5,
      Keyword: 13,
      Snippet: 14,
    },
  },
  editor: {
    defineTheme: vi.fn(),
  },
  Position: vi.fn((line, column) => ({ lineNumber: line, column })),
  Range: vi.fn((startLine, startColumn, endLine, endColumn) => ({
    startLineNumber: startLine,
    startColumn,
    endLineNumber: endLine,
    endColumn,
  })),
};

// Mock Worker
class MockWorker {
  onmessage: ((event: MessageEvent) => void) | null = null;
  private listeners: Map<string, Set<EventListener>> = new Map();
  postMessage: any;

  constructor(scriptURL: string | URL, options?: WorkerOptions) {
    this.postMessage = vi.fn((message: any) => {
    // Simulate immediate response
    setTimeout(() => {
      const response = {
        data: {
          id: message.id || "123e4567-e89b-12d3-a456-426614174000",
          result: {
            suggestions: [
              {
                type: "optimization",
                message: "Consider using list comprehension",
                impact: "medium",
                line: 2,
              },
            ],
            antiPatterns: [
              {
                type: "performance",
                message: "Avoid using eval()",
                severity: "high",
                line: 3,
              },
            ],
          },
        },
      };
      
      // Create a MessageEvent-like object
      const messageEvent = { 
        type: 'message',
        data: response.data,
        target: this,
        currentTarget: this,
        eventPhase: 2,
        bubbles: false,
        cancelable: false,
        defaultPrevented: false,
        composed: false,
        isTrusted: true,
        timeStamp: Date.now(),
        srcElement: this,
        returnValue: true,
        cancelBubble: false,
        path: [],
        composedPath: () => [],
        stopPropagation: () => {},
        stopImmediatePropagation: () => {},
        preventDefault: () => {},
        initEvent: () => {},
        NONE: 0,
        CAPTURING_PHASE: 1,
        AT_TARGET: 2,
        BUBBLING_PHASE: 3
      } as MessageEvent;
      
      // Call both onmessage and event listeners
      if (this.onmessage) {
        this.onmessage(messageEvent);
      }
      
      const messageListeners = this.listeners.get("message");
      if (messageListeners) {
        messageListeners.forEach(listener => {
          if (typeof listener === 'function') {
            listener(messageEvent);
          } else if (listener && typeof listener.handleEvent === 'function') {
            listener.handleEvent(messageEvent);
          }
        });
      }
    }, 0);
    });
  }

  addEventListener(type: string, listener: EventListener): void {
    if (!this.listeners.has(type)) {
      this.listeners.set(type, new Set());
    }
    this.listeners.get(type)!.add(listener);
    
    if (type === "message") {
      this.onmessage = listener as (event: MessageEvent) => void;
    }
  }

  removeEventListener(type: string, listener: EventListener): void {
    if (this.listeners.has(type)) {
      this.listeners.get(type)!.delete(listener);
    }
    if (type === "message") {
      this.onmessage = null;
    }
  }

  terminate(): void {}
}

global.Worker = vi.fn().mockImplementation((url, options) => new MockWorker(url, options)) as any;

// Mock crypto for UUID generation
Object.defineProperty(global, "crypto", {
  value: {
    randomUUID: () => "123e4567-e89b-12d3-a456-426614174000",
  },
});

describe("IntelliSensei", () => {
  let intelliSensei: IntelliSensei;
  let mockEditor: any;

  beforeEach(() => {
    vi.clearAllMocks();

    mockEditor = {
      onDidChangeModelContent: vi.fn(),
      getValue: vi.fn(() => "def example():\n    pass"),
      getPosition: vi.fn(() => ({ lineNumber: 1, column: 1 })),
      getModel: vi.fn(() => ({
        getLineContent: vi.fn(() => "def example():"),
      })),
    };

    intelliSensei = new IntelliSensei(mockMonaco as any);
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("initializes with correct configuration", () => {
    expect(LRUCache).toHaveBeenCalledWith({
      max: 50,
      ttl: 1000 * 60 * 5,
      updateAgeOnGet: true,
    });
  });

  it("registers Depyler language with Monaco", async () => {
    await intelliSensei.initialize(mockEditor);

    // Language registration is skipped (done in CodeEditor)
    expect(mockMonaco.editor.defineTheme).toHaveBeenCalledWith(
      "depyler-dark",
      expect.any(Object)
    );
  });

  it("sets up completion provider", async () => {
    await intelliSensei.initialize(mockEditor);

    expect(mockMonaco.languages.registerCompletionItemProvider).toHaveBeenCalledWith(
      "python-depyler",
      expect.objectContaining({
        triggerCharacters: ["@", ":"],
        provideCompletionItems: expect.any(Function),
      }),
    );
  });

  it("sets up inlay hints provider", async () => {
    await intelliSensei.initialize(mockEditor);

    expect(mockMonaco.languages.registerInlayHintsProvider).toHaveBeenCalledWith(
      "python-depyler",
      expect.objectContaining({
        provideInlayHints: expect.any(Function),
      }),
    );
  });

  it("sets up change listener with debouncing", async () => {
    await intelliSensei.initialize(mockEditor);

    expect(mockEditor.onDidChangeModelContent).toHaveBeenCalled();
  });

  it("extracts function context correctly", () => {
    const code = `
def calculate_sum(numbers: list) -> int:
    total = 0
    for num in numbers:
        total += num
    return total
    `;

    const context = (intelliSensei as any).extractFunctionContext(code, {
      lineNumber: 2,
      column: 5,
    });

    expect(context).toEqual({
      name: "calculate_sum",
      signature: "def calculate_sum(numbers: list)",
      body: expect.stringContaining("total = 0"),
      complexity: expect.any(Number),
      lastModified: expect.any(Number),
    });
  });

  it("computes deterministic cache keys", () => {
    const context = {
      name: "test_function",
      signature: "def test_function()",
      body: "pass",
      complexity: 1,
      lastModified: Date.now(),
    };

    const key1 = (intelliSensei as any).computeCacheKey(context);
    const key2 = (intelliSensei as any).computeCacheKey(context);

    expect(key1).toBe(key2);
    expect(key1).toMatch(/^test_function:def test_function\(\):\d+$/);
  });

  it("validates cache correctly", () => {
    const currentContext = {
      name: "test",
      signature: "test() -> None",
      body: "pass",
      complexity: 1,
      lastModified: Date.now(),
    };

    const cachedAnalysis = {
      context: currentContext,
      suggestions: [],
      antiPatterns: [],
      validUntil: Date.now() + 60000,
    };

    const isValid = (intelliSensei as any).isCacheValid(cachedAnalysis, currentContext);
    expect(isValid).toBe(true);

    // Test with expired cache
    cachedAnalysis.validUntil = Date.now() - 1000;
    const isExpired = (intelliSensei as any).isCacheValid(cachedAnalysis, currentContext);
    expect(isExpired).toBe(false);
  });

  it("performs analysis and caches results", async () => {
    const context = {
      name: "test_function",
      signature: "test_function() -> None",
      body: 'eval("dangerous")',
      complexity: 2,
      lastModified: Date.now(),
    };

    mockLRUCache.get.mockReturnValue(null); // Cache miss

    const result = await (intelliSensei as any).performAnalysis(context);

    expect(result).toEqual({
      suggestions: expect.arrayContaining([
        expect.objectContaining({
          type: "optimization",
          message: expect.any(String),
        }),
      ]),
      antiPatterns: expect.arrayContaining([
        expect.objectContaining({
          type: "performance",
          severity: "high",
        }),
      ]),
    });
  });

  it("prevents duplicate analysis requests", async () => {
    const context = {
      name: "test",
      signature: "test() -> None",
      body: "pass",
      complexity: 1,
      lastModified: Date.now(),
    };

    mockLRUCache.get.mockReturnValue(null);

    // Start two analyses simultaneously
    const promise1 = (intelliSensei as any).analyzeContext("def test():\n    pass", {
      lineNumber: 1,
      column: 1,
    });
    const promise2 = (intelliSensei as any).analyzeContext("def test():\n    pass", {
      lineNumber: 1,
      column: 1,
    });

    const [result1, result2] = await Promise.all([promise1, promise2]);

    expect(result1).toEqual(result2);
  });

  it("generates hash codes consistently", () => {
    const hash1 = (intelliSensei as any).hashCode("test string");
    const hash2 = (intelliSensei as any).hashCode("test string");
    const hash3 = (intelliSensei as any).hashCode("different string");

    expect(hash1).toBe(hash2);
    expect(hash1).not.toBe(hash3);
    expect(typeof hash1).toBe("number");
    expect(hash1).toBeGreaterThanOrEqual(0);
  });
});

describe("IntelliSensei Performance", () => {
  let intelliSensei: IntelliSensei;

  beforeEach(() => {
    intelliSensei = new IntelliSensei(mockMonaco as any);
  });

  it("caches analysis results to avoid recomputation", async () => {
    const cachedResult = {
      context: {
        name: "cached_function",
        signature: "cached_function() -> None",
        body: "pass",
        complexity: 1,
        lastModified: Date.now(),
      },
      suggestions: [{ type: "cached", message: "From cache" }],
      antiPatterns: [],
      validUntil: Date.now() + 60000,
    };

    mockLRUCache.get.mockReturnValue(cachedResult);

    const result = await (intelliSensei as any).analyzeContext(
      "def cached_function():\n    pass",
      { lineNumber: 1, column: 1 },
    );

    expect(result.suggestions[0].message).toBe("From cache");
    expect(mockLRUCache.set).not.toHaveBeenCalled();
  });

  it("debounces rapid analysis requests", async () => {
    // Clear mocks before this specific test
    vi.clearAllMocks();
    
    // Create a spy to track analysis calls
    const analyzeContextSpy = vi.fn();
    
    const mockEditor = {
      onDidChangeModelContent: vi.fn(),
      getValue: vi.fn(() => "def test(): pass"),
      getPosition: vi.fn(() => ({ lineNumber: 1, column: 1 })),
      getModel: vi.fn(() => ({
        getLineContent: vi.fn(() => "def test(): pass"),
        getWordAtPosition: vi.fn(() => ({ word: "test", startColumn: 5, endColumn: 9 })),
        getWordUntilPosition: vi.fn(() => ({ word: "test", startColumn: 5, endColumn: 9 })),
      })),
    };

    const newIntelliSensei = new IntelliSensei(mockMonaco as any);
    
    // Replace analyzeContext with our spy
    (newIntelliSensei as any).analyzeContext = analyzeContextSpy;
    
    await newIntelliSensei.initialize(mockEditor);

    const changeHandler = mockEditor.onDidChangeModelContent.mock.calls[0][0];

    // Trigger multiple rapid changes
    changeHandler({ changes: [{ text: "a" }] });
    changeHandler({ changes: [{ text: "b" }] });
    changeHandler({ changes: [{ text: "c" }] });

    // Initially should not be called
    expect(analyzeContextSpy).not.toHaveBeenCalled();

    // Wait for debounce
    await new Promise((resolve) => setTimeout(resolve, 350));

    // Should only trigger analysis once due to debouncing
    expect(analyzeContextSpy).toHaveBeenCalledTimes(1);
  });

  it("handles large function bodies efficiently", () => {
    const largeFunctionBody = "def large_function():\n" + "    pass\n".repeat(1000);

    const startTime = performance.now();
    const context = (intelliSensei as any).extractFunctionContext(
      largeFunctionBody,
      { lineNumber: 500, column: 5 },
    );
    const endTime = performance.now();

    expect(endTime - startTime).toBeLessThan(10); // Should parse within 10ms
    expect(context).toBeDefined();
  });
});
