// Advisor Worker for Intelli-Sensei
// Handles code analysis and suggestion generation

import { AnnotationSuggestion, AntiPattern } from "@/types";

interface FunctionContext {
  name: string;
  signature: string;
  body: string;
  complexity: number;
  lastModified: number;
}

interface AnalysisResult {
  suggestions: AnnotationSuggestion[];
  antiPatterns: AntiPattern[];
}

interface WorkerMessage {
  type: string;
  id: string;
  context?: FunctionContext;
}

// Anti-pattern detection rules
const ANTI_PATTERNS = [
  {
    pattern: /eval\s*\(/,
    name: "eval_usage",
    description: "Using eval() is dangerous and prevents optimization",
    severity: "error" as const,
    suggestion:
      "Consider using ast.literal_eval() for safe evaluation or refactor to avoid dynamic code execution",
  },
  {
    pattern: /exec\s*\(/,
    name: "exec_usage",
    description: "Using exec() prevents static analysis and optimization",
    severity: "warning" as const,
    suggestion: "Refactor to use explicit function calls or data structures",
  },
  {
    pattern: /\*args.*\*\*kwargs/,
    name: "excessive_variadic",
    description: "Excessive use of *args/**kwargs makes type inference difficult",
    severity: "warning" as const,
    suggestion: "Consider using explicit parameters or TypedDict for better type safety",
  },
  {
    pattern: /range\(len\(/,
    name: "range_len_antipattern",
    description: "Using range(len()) is inefficient",
    severity: "info" as const,
    suggestion: "Use enumerate() instead: for i, item in enumerate(items)",
  },
  {
    pattern: /\.append\(.*\).*for.*in/,
    name: "list_comprehension_opportunity",
    description: "Loop with append can be replaced with list comprehension",
    severity: "info" as const,
    suggestion: "Consider using list comprehension for better performance",
  },
  {
    pattern: /global\s+\w+/,
    name: "global_usage",
    description: "Global variables make code harder to reason about and optimize",
    severity: "warning" as const,
    suggestion: "Consider passing values as parameters or using a class",
  },
  {
    pattern: /import\s+\*|from\s+\w+\s+import\s+\*/,
    name: "wildcard_import",
    description: "Wildcard imports make dependencies unclear",
    severity: "warning" as const,
    suggestion: "Import specific names or use qualified imports",
  },
];

// Optimization opportunity detection
const OPTIMIZATION_PATTERNS = [
  {
    pattern: /for\s+\w+\s+in\s+range\(\d+\):/,
    type: "loop_optimization",
    description: "Simple numeric loop can be optimized",
    impact: "medium" as const,
    suggestion: "Consider vectorization or compiled alternatives",
  },
  {
    pattern: /if\s+\w+\s+in\s+\[.*\]:/,
    type: "membership_optimization",
    description: "List membership check can be optimized",
    impact: "low" as const,
    suggestion: "Use a set for faster membership testing: if item in {item1, item2, item3}",
  },
  {
    pattern: /def\s+\w+\([^)]*\)\s*:\s*return\s+[^()]+\([^)]*\)/,
    type: "function_wrapper",
    description: "Simple function wrapper detected",
    impact: "low" as const,
    suggestion: "Consider inlining or using functools.partial",
  },
  {
    pattern: /str\(\d+\)|str\(\w+\)/,
    type: "string_conversion",
    description: "String conversion can be optimized",
    impact: "low" as const,
    suggestion: 'Use f-strings for better performance: f"{value}"',
  },
];

// Annotation suggestions based on code patterns
const ANNOTATION_SUGGESTIONS = [
  {
    pattern: /for\s+\w+\s+in\s+range\(\d+\):/,
    annotation_type: "optimize_energy",
    description: "Numeric loops benefit from energy optimization",
    example: "# @depyler: optimize_energy=true",
    impact: "high" as const,
  },
  {
    pattern: /def\s+\w+\([^)]*str[^)]*\)/,
    annotation_type: "string_strategy",
    description: "Functions with string parameters can use zero-copy optimization",
    example: "# @depyler: string_strategy=zero_copy",
    impact: "medium" as const,
  },
  {
    pattern: /def\s+\w+\([^)]*list\[[^]]+\][^)]*\)/,
    annotation_type: "ownership_model",
    description: "List parameters can use borrowed ownership for better performance",
    example: "# @depyler: ownership_model=borrowed",
    impact: "medium" as const,
  },
  {
    pattern: /try:|except:|raise/,
    annotation_type: "safety_level",
    description: "Error handling code benefits from strict safety checks",
    example: "# @depyler: safety_level=strict",
    impact: "high" as const,
  },
];

class CodeAnalyzer {
  analyzeFunction(context: FunctionContext): AnalysisResult {
    const suggestions: AnnotationSuggestion[] = [];
    const antiPatterns: AntiPattern[] = [];

    const lines = context.body.split("\n");

    // Detect anti-patterns
    for (let lineIndex = 0; lineIndex < lines.length; lineIndex++) {
      const line = lines[lineIndex];
      const lineNumber = lineIndex + 1;

      for (const antiPattern of ANTI_PATTERNS) {
        const match = line.match(antiPattern.pattern);
        if (match) {
          const column = line.indexOf(match[0]);
          antiPatterns.push({
            line: lineNumber,
            column,
            pattern: antiPattern.name,
            description: antiPattern.description,
            severity: antiPattern.severity,
            suggestion: antiPattern.suggestion,
          });
        }
      }
    }

    // Generate annotation suggestions
    for (const suggestionRule of ANNOTATION_SUGGESTIONS) {
      const match = context.body.match(suggestionRule.pattern);
      if (match) {
        // Find the function definition line
        const functionDefMatch = context.body.match(/def\s+\w+/);
        if (functionDefMatch) {
          const functionLine =
            context.body.substring(0, context.body.indexOf(functionDefMatch[0])).split("\n").length;

          suggestions.push({
            line: functionLine - 1, // Suggest annotation before function
            column: 0,
            annotation_type: suggestionRule.annotation_type,
            description: suggestionRule.description,
            example: suggestionRule.example,
            impact: suggestionRule.impact,
          });
        }
      }
    }

    // Complexity-based suggestions
    if (context.complexity > 10) {
      suggestions.push({
        line: 1,
        column: 0,
        annotation_type: "optimize_energy",
        description: "High complexity function benefits from energy optimization",
        example: "# @depyler: optimize_energy=true",
        impact: "high",
      });
    }

    if (context.complexity > 15) {
      suggestions.push({
        line: 1,
        column: 0,
        annotation_type: "safety_level",
        description: "Complex function should use strict safety checking",
        example: "# @depyler: safety_level=strict",
        impact: "high",
      });
    }

    return {
      suggestions: this.deduplicateSuggestions(suggestions),
      antiPatterns,
    };
  }

  private deduplicateSuggestions(suggestions: AnnotationSuggestion[]): AnnotationSuggestion[] {
    const seen = new Set<string>();
    return suggestions.filter((suggestion) => {
      const key = `${suggestion.line}:${suggestion.annotation_type}`;
      if (seen.has(key)) {
        return false;
      }
      seen.add(key);
      return true;
    });
  }
}

// Worker message handling
const analyzer = new CodeAnalyzer();

self.addEventListener("message", (event: MessageEvent<WorkerMessage>) => {
  const { type, id, context } = event.data;

  try {
    switch (type) {
      case "ANALYZE_FUNCTION": {
        if (!context) {
          throw new Error("Missing function context");
        }

        const result = analyzer.analyzeFunction(context);

        self.postMessage({
          id,
          result,
        });
        break;
      }

      default:
        throw new Error(`Unknown message type: ${type}`);
    }
  } catch (error) {
    self.postMessage({
      id,
      error: error instanceof Error ? error.message : "Unknown error",
    });
  }
});

// Handle worker errors
self.addEventListener("error", (event) => {
  console.error("Advisor worker error:", event);
});

self.addEventListener("unhandledrejection", (event) => {
  console.error("Advisor worker unhandled rejection:", event);
});
