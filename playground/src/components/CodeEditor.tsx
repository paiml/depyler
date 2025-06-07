import React, { useEffect, useRef } from "react";
import Editor from "@monaco-editor/react";
import type * as monaco from "monaco-editor";

interface CodeEditorProps {
  language: string;
  value: string;
  onChange: (value: string) => void;
  height?: string;
  readOnly?: boolean;
  loading?: boolean;
}

export function CodeEditor({
  language,
  value,
  onChange,
  height = "300px",
  readOnly = false,
  loading = false,
}: CodeEditorProps) {
  const editorRef = useRef<any>(null);

  const handleEditorDidMount = (editor: any, monaco: any) => {
    editorRef.current = editor;

    // Configure completion provider for Python
    if (language === "python") {
      configureDepylerCompletion(monaco);
    }

    // Set up editor options
    editor.updateOptions({
      fontSize: 14,
      lineHeight: 20,
      fontFamily: "JetBrains Mono, Fira Code, Monaco, Menlo, monospace",
      minimap: { enabled: false },
      scrollBeyondLastLine: false,
      automaticLayout: true,
      tabSize: 4,
      insertSpaces: true,
      wordWrap: "on",
      readOnly,
    });
  };

  const handleEditorChange = (value: string | undefined) => {
    if (value !== undefined && !readOnly) {
      onChange(value);
    }
  };

  return (
    <div className="relative">
      {loading && (
        <div className="absolute inset-0 bg-white bg-opacity-75 flex items-center justify-center z-10">
          <div className="flex items-center space-x-2">
            <div className="animate-spin rounded-full h-5 w-5 border-b-2 border-blue-600"></div>
            <span className="text-sm text-gray-600">Transpiling...</span>
          </div>
        </div>
      )}

      <Editor
        height={height}
        language={language}
        value={value}
        onChange={handleEditorChange}
        onMount={handleEditorDidMount}
        theme="vs-dark"
        options={{
          readOnly,
          selectOnLineNumbers: true,
          roundedSelection: false,
          cursorStyle: "line",
          automaticLayout: true,
        }}
      />
    </div>
  );
}

function configureDepylerCompletion(monaco: any) {
  // Register completion provider for Python to support @depyler annotations
  const completionProviderDisposable = monaco.languages.registerCompletionItemProvider("python", {
    triggerCharacters: ["@", ":"],
    provideCompletionItems: (model: any, position: any) => {
      const line = model.getLineContent(position.lineNumber);
      const wordInfo = model.getWordUntilPosition(position);

      if (line.includes("@depyler:") || line.includes("@depyler")) {
        return {
          suggestions: [
            {
              label: "optimize_energy",
              kind: monaco.languages.CompletionItemKind.Property,
              insertText: "optimize_energy=true",
              documentation: "Enable energy-efficient optimizations",
            },
            {
              label: "string_strategy",
              kind: monaco.languages.CompletionItemKind.Property,
              insertText: "string_strategy=zero_copy",
              documentation: "String handling strategy: zero_copy, owned, or cow",
            },
            {
              label: "ownership_model",
              kind: monaco.languages.CompletionItemKind.Property,
              insertText: "ownership_model=borrowed",
              documentation: "Memory ownership model: borrowed, owned, or shared",
            },
            {
              label: "safety_level",
              kind: monaco.languages.CompletionItemKind.Property,
              insertText: "safety_level=strict",
              documentation: "Safety guarantees: strict, moderate, or unsafe",
            },
          ],
        };
      }

      return { suggestions: [] };
    },
  });

  // Register hover provider for annotations
  monaco.languages.registerHoverProvider("python-depyler", {
    provideHover: (model: any, position: any) => {
      const word = model.getWordAtPosition(position);
      if (!word) return;

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
          range: new monaco.Range(
            position.lineNumber,
            word.startColumn,
            position.lineNumber,
            word.endColumn,
          ),
          contents: [
            { value: `**@depyler: ${word.word}**` },
            { value: documentation },
          ],
        };
      }
    },
  });
}
