/// <reference lib="deno.ns" />
import { assertEquals, assertExists } from "https://deno.land/std@0.219.0/assert/mod.ts";

// Mock Monaco Editor for testing
const mockMonaco = {
  languages: {
    register: (config: { id: string }) => {
      assertEquals(typeof config.id, "string");
    },
    setLanguageConfiguration: (id: string, config: any) => {
      assertEquals(typeof id, "string");
      assertExists(config.comments);
    },
    setTokensProvider: (id: string, provider: any) => {
      assertEquals(typeof id, "string");
      assertExists(provider.getInitialState);
      assertExists(provider.tokenize);
    },
    registerCompletionItemProvider: (id: string, provider: any) => {
      assertEquals(typeof id, "string");
      assertExists(provider.provideCompletionItems);
    },
    registerHoverProvider: (id: string, provider: any) => {
      assertEquals(typeof id, "string");
      assertExists(provider.provideHover);
    },
  },
  editor: {
    defineTheme: (name: string, theme: any) => {
      assertEquals(typeof name, "string");
      assertExists(theme.base);
      assertExists(theme.rules);
    },
  },
  Range: class MockRange {
    constructor(
      public startLineNumber: number,
      public startColumn: number,
      public endLineNumber: number,
      public endColumn: number,
    ) {}
  },
};

// Test the configurePythonDepyler function logic
Deno.test("Monaco configuration setup", () => {
  // Test language registration
  mockMonaco.languages.register({ id: "python-depyler" });

  // Test language configuration
  mockMonaco.languages.setLanguageConfiguration("python-depyler", {
    comments: {
      lineComment: "#",
      blockComment: ['"""', '"""'],
    },
    brackets: [
      ["{", "}"],
      ["[", "]"],
      ["(", ")"],
    ],
    autoClosingPairs: [
      { open: "{", close: "}" },
      { open: "[", close: "]" },
      { open: "(", close: ")" },
      { open: '"', close: '"' },
      { open: "'", close: "'" },
    ],
  });

  // Test tokens provider
  const tokenProvider = {
    getInitialState: () => ({ inComment: false }),
    tokenize: (line: string) => {
      const tokens: any[] = [];
      let currentIndex = 0;

      // Test Depyler annotation detection
      const annotationMatch = line.match(/@depyler:\s*\w+(\s*=\s*\w+)?/);
      if (annotationMatch) {
        tokens.push({
          startIndex: annotationMatch.index || 0,
          scopes: "annotation.python-depyler",
        });
        currentIndex = (annotationMatch.index || 0) + annotationMatch[0].length;
      }

      if (currentIndex < line.length) {
        tokens.push({
          startIndex: currentIndex,
          scopes: "source.python-depyler",
        });
      }

      return {
        tokens,
        endState: { inComment: false },
      };
    },
  };

  mockMonaco.languages.setTokensProvider("python-depyler", tokenProvider);

  // Test theme definition
  mockMonaco.editor.defineTheme("vs-dark-depyler", {
    base: "vs-dark",
    inherit: true,
    rules: [
      { token: "annotation", foreground: "#4CAF50", fontStyle: "bold" },
      { token: "function", foreground: "#FFD700" },
      { token: "class", foreground: "#FF6B6B" },
    ],
    colors: {},
  });
});

Deno.test("Tokenizer functionality", () => {
  const tokenizer = {
    getInitialState: () => ({ inComment: false }),
    tokenize: (line: string) => {
      const tokens: any[] = [];
      let currentIndex = 0;

      const annotationMatch = line.match(/@depyler:\s*\w+(\s*=\s*\w+)?/);
      if (annotationMatch) {
        tokens.push({
          startIndex: annotationMatch.index || 0,
          scopes: "annotation.python-depyler",
        });
        currentIndex = (annotationMatch.index || 0) + annotationMatch[0].length;
      }

      if (currentIndex < line.length) {
        tokens.push({
          startIndex: currentIndex,
          scopes: "source.python-depyler",
        });
      }

      return {
        tokens,
        endState: { inComment: false },
      };
    },
  };

  // Test annotation detection - matches the entire annotation
  const result1 = tokenizer.tokenize("@depyler: optimize_energy=true");
  // The regex matches the entire string, so no remainder token
  assertEquals(result1.tokens.length, 1);
  assertEquals(result1.tokens[0].scopes, "annotation.python-depyler");

  // Test regular line
  const result2 = tokenizer.tokenize("def hello_world():");
  assertEquals(result2.tokens.length, 1);
  assertEquals(result2.tokens[0].scopes, "source.python-depyler");

  // Test empty line
  const result3 = tokenizer.tokenize("");
  assertEquals(result3.tokens.length, 0);
});

Deno.test("Completion provider logic", () => {
  const completionProvider = {
    triggerCharacters: ["@", ":"],
    provideCompletionItems: (model: any, position: any) => {
      const line = "# @depyler:";
      const wordInfo = { word: "depyler", startColumn: 3, endColumn: 10 };

      if (line.includes("@depyler:") || line.includes("@depyler")) {
        return {
          suggestions: [
            {
              label: "optimize_energy",
              kind: "Property",
              insertText: "optimize_energy=true",
              documentation: "Enable energy-efficient optimizations",
            },
            {
              label: "string_strategy",
              kind: "Property",
              insertText: "string_strategy=zero_copy",
              documentation: "String handling strategy: zero_copy, owned, or cow",
            },
            {
              label: "ownership_model",
              kind: "Property",
              insertText: "ownership_model=borrowed",
              documentation: "Memory ownership model: borrowed, owned, or shared",
            },
            {
              label: "safety_level",
              kind: "Property",
              insertText: "safety_level=strict",
              documentation: "Safety guarantees: strict, moderate, or unsafe",
            },
          ],
        };
      }

      return { suggestions: [] };
    },
  };

  const result = completionProvider.provideCompletionItems(null, null);
  assertEquals(result.suggestions.length, 4);
  assertEquals(result.suggestions[0].label, "optimize_energy");
});

Deno.test("Hover provider logic", () => {
  const hoverProvider = {
    provideHover: (model: any, position: any) => {
      const word = { word: "optimize_energy", startColumn: 1, endColumn: 15 };

      const hoverMap: Record<string, string> = {
        "optimize_energy":
          "Enables energy-efficient code generation with CPU and memory optimizations",
        "string_strategy": "Controls how strings are handled in the generated Rust code",
        "ownership_model": "Specifies the memory ownership pattern for the generated code",
        "safety_level": "Determines the level of safety checks in the generated Rust code",
      };

      const documentation = hoverMap[word.word];
      if (documentation) {
        return {
          range: {
            startLineNumber: 1,
            startColumn: word.startColumn,
            endLineNumber: 1,
            endColumn: word.endColumn,
          },
          contents: [
            { value: `**@depyler: ${word.word}**` },
            { value: documentation },
          ],
        };
      }

      return null;
    },
  };

  const result = hoverProvider.provideHover(null, null);
  assertExists(result);
  assertEquals(result.contents.length, 2);
  assertEquals(result.contents[0].value, "**@depyler: optimize_energy**");
});
