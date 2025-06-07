import { describe, expect, it, vi, beforeEach } from "vitest";
import { render } from "@testing-library/react";
import { CodeEditor } from "../CodeEditor";

// Mock Monaco editor
const mockMonaco = {
  languages: {
    getLanguages: vi.fn(() => []),
    register: vi.fn(),
    setLanguageConfiguration: vi.fn(),
    setMonarchTokensProvider: vi.fn(),
    registerCompletionItemProvider: vi.fn(),
    registerHoverProvider: vi.fn(),
    CompletionItemKind: {
      Property: 1,
    },
  },
  editor: {
    defineTheme: vi.fn(),
  },
  Range: vi.fn((startLine, startCol, endLine, endCol) => ({
    startLineNumber: startLine,
    startColumn: startCol,
    endLineNumber: endLine,
    endColumn: endCol,
  })),
};

const mockEditor = {
  updateOptions: vi.fn(),
};

vi.mock("@monaco-editor/react", () => ({
  default: ({ onMount }: any) => {
    // Simulate Monaco editor mount
    setTimeout(() => {
      onMount?.(mockEditor, mockMonaco);
    }, 0);
    return null;
  },
}));

describe("CodeEditor Monaco Configuration", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("configures Python-Depyler language when language is python", async () => {
    render(
      <CodeEditor
        language="python"
        value="# test code"
        onChange={() => {}}
      />
    );

    // Wait for Monaco to mount
    await vi.waitFor(() => {
      expect(mockMonaco.languages.register).toHaveBeenCalledWith({
        id: "python-depyler",
      });
    });
  });

  it("sets up language configuration for Python-Depyler", async () => {
    render(
      <CodeEditor
        language="python"
        value="# test code"
        onChange={() => {}}
      />
    );

    await vi.waitFor(() => {
      expect(mockMonaco.languages.setLanguageConfiguration).toHaveBeenCalledWith(
        "python-depyler",
        expect.objectContaining({
          comments: {
            lineComment: "#",
            blockComment: ['"""', '"""'],
          },
          brackets: [
            ["{", "}"],
            ["[", "]"],
            ["(", ")"],
          ],
          autoClosingPairs: expect.any(Array),
        })
      );
    });
  });

  it("configures Monarch tokenizer with Depyler annotations", async () => {
    render(
      <CodeEditor
        language="python"
        value="# test code"
        onChange={() => {}}
      />
    );

    await vi.waitFor(() => {
      expect(mockMonaco.languages.setMonarchTokensProvider).toHaveBeenCalledWith(
        "python-depyler",
        expect.objectContaining({
          tokenizer: expect.objectContaining({
            root: expect.arrayContaining([
              [/@depyler:[\w\s=,]+/, 'keyword.depyler'],
              [/@depyler(?!:)/, 'keyword.incomplete'],
            ]),
          }),
        })
      );
    });
  });

  it("defines custom theme for Python-Depyler", async () => {
    render(
      <CodeEditor
        language="python"
        value="# test code"
        onChange={() => {}}
      />
    );

    await vi.waitFor(() => {
      expect(mockMonaco.editor.defineTheme).toHaveBeenCalledWith(
        "python-depyler-theme",
        expect.objectContaining({
          base: "vs-dark",
          inherit: true,
          rules: expect.arrayContaining([
            { token: "keyword.depyler", foreground: "#4CAF50", fontStyle: "bold" },
            { token: "keyword.incomplete", foreground: "#FFC107", fontStyle: "bold" },
          ]),
        })
      );
    });
  });

  it("registers completion provider for annotations", async () => {
    render(
      <CodeEditor
        language="python"
        value="# test code"
        onChange={() => {}}
      />
    );

    await vi.waitFor(() => {
      expect(mockMonaco.languages.registerCompletionItemProvider).toHaveBeenCalledWith(
        "python-depyler",
        expect.objectContaining({
          triggerCharacters: ["@", ":"],
          provideCompletionItems: expect.any(Function),
        })
      );
    });
  });

  it("provides completions for @depyler annotations", async () => {
    render(
      <CodeEditor
        language="python"
        value="# test code"
        onChange={() => {}}
      />
    );

    await vi.waitFor(() => {
      expect(mockMonaco.languages.registerCompletionItemProvider).toHaveBeenCalled();
    });

    // Get the completion provider
    const providerCall = mockMonaco.languages.registerCompletionItemProvider.mock.calls[0];
    const provider = providerCall[1];

    // Mock model and position
    const mockModel = {
      getLineContent: vi.fn(() => "@depyler:"),
      getWordUntilPosition: vi.fn(() => ({ word: "depyler" })),
    };
    const mockPosition = { lineNumber: 1, column: 10 };

    // Call the provider
    const result = provider.provideCompletionItems(mockModel, mockPosition);

    expect(result.suggestions).toEqual(
      expect.arrayContaining([
        expect.objectContaining({
          label: "optimize_energy",
          insertText: "optimize_energy=true",
          documentation: "Enable energy-efficient optimizations",
        }),
        expect.objectContaining({
          label: "string_strategy",
          insertText: "string_strategy=zero_copy",
        }),
      ])
    );
  });

  it("registers hover provider for annotations", async () => {
    render(
      <CodeEditor
        language="python"
        value="# test code"
        onChange={() => {}}
      />
    );

    await vi.waitFor(() => {
      expect(mockMonaco.languages.registerHoverProvider).toHaveBeenCalledWith(
        "python-depyler",
        expect.objectContaining({
          provideHover: expect.any(Function),
        })
      );
    });
  });

  it("provides hover information for annotation keywords", async () => {
    render(
      <CodeEditor
        language="python"
        value="# test code"
        onChange={() => {}}
      />
    );

    await vi.waitFor(() => {
      expect(mockMonaco.languages.registerHoverProvider).toHaveBeenCalled();
    });

    // Get the hover provider
    const providerCall = mockMonaco.languages.registerHoverProvider.mock.calls[0];
    const provider = providerCall[1];

    // Mock model and position
    const mockModel = {
      getWordAtPosition: vi.fn(() => ({
        word: "optimize_energy",
        startColumn: 10,
        endColumn: 25,
      })),
    };
    const mockPosition = { lineNumber: 1, column: 15 };

    // Call the provider
    const result = provider.provideHover(mockModel, mockPosition);

    expect(result).toEqual(
      expect.objectContaining({
        contents: [
          { value: "**@depyler: optimize_energy**" },
          { value: "Enables energy-efficient code generation with CPU and memory optimizations" },
        ],
      })
    );
  });

  it("sets editor options correctly", async () => {
    render(
      <CodeEditor
        language="python"
        value="# test code"
        onChange={() => {}}
        readOnly={true}
      />
    );

    await vi.waitFor(() => {
      expect(mockEditor.updateOptions).toHaveBeenCalledWith(
        expect.objectContaining({
          fontSize: 14,
          lineHeight: 20,
          fontFamily: "JetBrains Mono, Fira Code, Monaco, Menlo, monospace",
          minimap: { enabled: false },
          scrollBeyondLastLine: false,
          automaticLayout: true,
          tabSize: 4,
          insertSpaces: true,
          wordWrap: "on",
          readOnly: true,
        })
      );
    });
  });

  it("does not configure Python-Depyler for non-Python languages", async () => {
    render(
      <CodeEditor
        language="rust"
        value="fn main() {}"
        onChange={() => {}}
      />
    );

    await vi.waitFor(() => {
      expect(mockEditor.updateOptions).toHaveBeenCalled();
    });

    expect(mockMonaco.languages.register).not.toHaveBeenCalledWith({
      id: "python-depyler",
    });
  });
});