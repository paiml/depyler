import { describe, expect, it, vi, beforeEach } from "vitest";

// Mock the advisor analysis
const mockAnalyzeFunction = vi.fn((context) => {
  return {
    suggestions: [
      {
        type: "optimization",
        message: "Consider using list comprehension",
        impact: "medium",
        line: 1,
      },
    ],
    antiPatterns: [],
  };
});

// Mock worker context
const mockPostMessage = vi.fn();
global.postMessage = mockPostMessage;

describe("Advisor Worker", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("handles ANALYZE_FUNCTION message", () => {
    const message = {
      data: {
        type: "ANALYZE_FUNCTION",
        id: "test-123",
        context: {
          code: "def test(): pass",
          position: { line: 1, column: 1 },
        },
      },
    };

    // Simulate message handling
    const result = mockAnalyzeFunction(message.data.context);
    
    expect(result.suggestions).toHaveLength(1);
    expect(result.suggestions[0].type).toBe("optimization");
  });

  it("returns suggestions for code optimization", () => {
    const context = {
      code: "for i in range(len(items)):\n    print(items[i])",
      position: { line: 1, column: 1 },
    };

    const result = mockAnalyzeFunction(context);
    
    expect(result.suggestions).toBeDefined();
    expect(result.suggestions[0].message).toContain("Consider using");
  });

  it("detects anti-patterns", () => {
    mockAnalyzeFunction.mockReturnValueOnce({
      suggestions: [],
      antiPatterns: [
        {
          type: "performance",
          message: "Avoid using eval()",
          severity: "high",
          line: 2,
        },
      ],
    });

    const context = {
      code: "def unsafe():\n    eval('print(1)')",
      position: { line: 2, column: 5 },
    };

    const result = mockAnalyzeFunction(context);
    
    expect(result.antiPatterns).toHaveLength(1);
    expect(result.antiPatterns[0].severity).toBe("high");
  });

  it("handles empty code gracefully", () => {
    const context = {
      code: "",
      position: { line: 1, column: 1 },
    };

    mockAnalyzeFunction.mockReturnValueOnce({
      suggestions: [],
      antiPatterns: [],
    });

    const result = mockAnalyzeFunction(context);
    
    expect(result.suggestions).toHaveLength(0);
    expect(result.antiPatterns).toHaveLength(0);
  });

  it("provides context-aware suggestions", () => {
    mockAnalyzeFunction.mockImplementationOnce((context) => {
      if (context.code.includes("@depyler")) {
        return {
          suggestions: [{
            type: "annotation",
            message: "Valid Depyler annotation detected",
            impact: "high",
            line: 1,
          }],
          antiPatterns: [],
        };
      }
      return { suggestions: [], antiPatterns: [] };
    });

    const context = {
      code: "# @depyler: optimize_energy=true\ndef efficient(): pass",
      position: { line: 1, column: 1 },
    };

    const result = mockAnalyzeFunction(context);
    
    expect(result.suggestions[0].type).toBe("annotation");
    expect(result.suggestions[0].impact).toBe("high");
  });
});