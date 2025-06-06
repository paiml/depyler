import type * as monaco from 'monaco-editor';
import { LRUCache } from 'lru-cache';
import { AnnotationSuggestion, AntiPattern, StaticAnalysis } from '@/types';

interface FunctionContext {
  name: string;
  signature: string;
  body: string;
  complexity: number;
  lastModified: number;
}

interface AnalysisCache {
  context: FunctionContext;
  suggestions: AnnotationSuggestion[];
  antiPatterns: AntiPattern[];
  validUntil: number;
}

interface AnalysisResult {
  suggestions: AnnotationSuggestion[];
  antiPatterns: AntiPattern[];
}

export class IntelliSensei {
  private advisorWorker: Worker;
  private analysisCache: LRUCache<string, AnalysisCache>;
  private pendingAnalysis: Map<string, Promise<AnalysisResult>>;
  private monaco: typeof monaco;
  
  constructor(monacoInstance: typeof monaco) {
    this.monaco = monacoInstance;
    
    this.advisorWorker = new Worker(
      new URL('../workers/advisor.worker.ts', import.meta.url),
      { type: 'module', name: 'intelli-sensei-advisor' }
    );
    
    // LRU cache with 50 function capacity
    this.analysisCache = new LRUCache<string, AnalysisCache>({
      max: 50,
      ttl: 1000 * 60 * 5, // 5 minute TTL
      updateAgeOnGet: true,
    });
    
    this.pendingAnalysis = new Map();
  }
  
  async initialize(editor: monaco.editor.IStandaloneCodeEditor) {
    // Register Depyler-enhanced Python language
    this.registerDepylerLanguage();
    
    // Debounced analysis to prevent worker spam
    const debouncedAnalysis = this.debounce(
      (value: string, position: monaco.Position) => this.analyzeContext(value, position),
      300
    );
    
    // Real-time pattern detection with caching
    editor.onDidChangeModelContent(async (e) => {
      const position = editor.getPosition();
      if (!position) return;
      
      await debouncedAnalysis(editor.getValue(), position);
    });
    
    // Annotation provider with cache-aware completions
    this.monaco.languages.registerCompletionItemProvider('python-depyler', {
      triggerCharacters: ['@', ':'],
      provideCompletionItems: async (model, position) => {
        return this.getAnnotationCompletions(model, position);
      }
    });
    
    // Inline hints for optimization opportunities
    this.monaco.languages.registerInlayHintsProvider('python-depyler', {
      provideInlayHints: async (model, range, token) => {
        return this.getOptimizationHints(model, range);
      }
    });
    
    // Code action provider for automatic fixes
    this.monaco.languages.registerCodeActionProvider('python-depyler', {
      provideCodeActions: async (model, range, context, token) => {
        return this.getCodeActions(model, range, context);
      }
    });
  }
  
  private registerDepylerLanguage() {
    // Enhanced Python language with Depyler annotations
    this.monaco.languages.register({ id: 'python-depyler' });
    
    this.monaco.languages.setMonarchTokenizer('python-depyler', {
      tokenizer: {
        root: [
          // Depyler annotations - enhanced patterns
          [/@depyler:\s*[\w_]+(\s*=\s*[\w_]+)?(\s*,\s*[\w_]+(\s*=\s*[\w_]+)?)*/, 'annotation'],
          [/@depyler/, 'annotation.incomplete'],
          
          // Python keywords
          [/\b(def|class|if|else|elif|for|while|try|except|finally|with|as|import|from|return|yield|break|continue|pass|raise|assert|global|nonlocal|lambda|and|or|not|in|is|async|await)\b/, 'keyword'],
          
          // Built-in types and common types
          [/\b(int|float|str|bool|list|dict|tuple|set|None|True|False|Optional|Union|List|Dict|Tuple|Set)\b/, 'type'],
          
          // Function and class definitions
          [/(def)\s+([a-zA-Z_]\w*)\s*\(/, ['keyword', 'function.definition']],
          [/(class)\s+([a-zA-Z_]\w*)/, ['keyword', 'class.definition']],
          
          // Strings with f-string support
          [/f["']/, { token: 'string.fstring', next: '@fstring' }],
          [/["']([^"'\\]|\\.)*["']/, 'string'],
          [/"""[\s\S]*?"""/, 'string.multiline'],
          [/'''[\s\S]*?'''/, 'string.multiline'],
          
          // Numbers
          [/\b\d+(\.\d+)?([eE][+-]?\d+)?\b/, 'number'],
          
          // Comments
          [/#.*$/, 'comment'],
          
          // Operators
          [/[+\-*/%=<>!&|^~]/, 'operator'],
          
          // Delimiters
          [/[{}()\[\]]/, 'delimiter'],
        ],
        
        fstring: [
          [/[^{"'\\]+/, 'string.fstring'],
          [/\{[^}]*\}/, 'string.fstring.expression'],
          [/["']/, { token: 'string.fstring', next: '@pop' }],
        ],
      },
    });
    
    // Enhanced theme for better visualization
    this.monaco.editor.defineTheme('depyler-dark', {
      base: 'vs-dark',
      inherit: true,
      rules: [
        { token: 'annotation', foreground: '#4CAF50', fontStyle: 'bold' },
        { token: 'annotation.incomplete', foreground: '#FFC107', fontStyle: 'bold' },
        { token: 'function.definition', foreground: '#FFD700', fontStyle: 'bold' },
        { token: 'class.definition', foreground: '#FF6B6B', fontStyle: 'bold' },
        { token: 'string.fstring', foreground: '#E1F5FE' },
        { token: 'string.fstring.expression', foreground: '#81C784' },
        { token: 'type', foreground: '#64B5F6' },
      ],
      colors: {
        'editor.background': '#1e1e1e',
        'editor.lineHighlightBackground': '#2d2d30',
      },
    });
  }
  
  private async analyzeContext(code: string, position: monaco.Position): Promise<AnalysisResult> {
    const functionContext = this.extractFunctionContext(code, position);
    if (!functionContext) return { suggestions: [], antiPatterns: [] };
    
    const cacheKey = this.computeCacheKey(functionContext);
    
    // Check cache validity
    const cached = this.analysisCache.get(cacheKey);
    if (cached && this.isCacheValid(cached, functionContext)) {
      return {
        suggestions: cached.suggestions,
        antiPatterns: cached.antiPatterns,
      };
    }
    
    // Prevent duplicate analysis requests
    const pending = this.pendingAnalysis.get(cacheKey);
    if (pending) return pending;
    
    // Perform analysis
    const analysisPromise = this.performAnalysis(functionContext);
    this.pendingAnalysis.set(cacheKey, analysisPromise);
    
    try {
      const result = await analysisPromise;
      
      // Update cache
      this.analysisCache.set(cacheKey, {
        context: functionContext,
        suggestions: result.suggestions,
        antiPatterns: result.antiPatterns,
        validUntil: Date.now() + 60000, // 1 minute validity
      });
      
      return result;
    } finally {
      this.pendingAnalysis.delete(cacheKey);
    }
  }
  
  private extractFunctionContext(code: string, position: monaco.Position): FunctionContext | null {
    const lines = code.split('\n');
    let functionStart = -1;
    let functionEnd = -1;
    let functionName = '';
    let signature = '';
    let indentLevel = 0;
    
    // Find the function containing the current position
    for (let i = position.lineNumber - 1; i >= 0; i--) {
      const line = lines[i];
      const trimmed = line.trim();
      
      if (trimmed.startsWith('def ')) {
        functionStart = i;
        const match = trimmed.match(/def\s+(\w+)\s*\(([^)]*)\)/);
        if (match) {
          functionName = match[1];
          signature = `def ${match[1]}(${match[2]})`;
          indentLevel = line.length - line.trimStart().length;
        }
        break;
      }
    }
    
    if (functionStart === -1) return null;
    
    // Find the end of the function
    for (let i = functionStart + 1; i < lines.length; i++) {
      const line = lines[i];
      if (line.trim() === '') continue; // Skip empty lines
      
      const currentIndent = line.length - line.trimStart().length;
      if (currentIndent <= indentLevel) {
        functionEnd = i - 1;
        break;
      }
    }
    
    if (functionEnd === -1) functionEnd = lines.length - 1;
    
    const body = lines.slice(functionStart, functionEnd + 1).join('\n');
    const complexity = this.calculateComplexity(body);
    
    return {
      name: functionName,
      signature,
      body,
      complexity,
      lastModified: Date.now(),
    };
  }
  
  private calculateComplexity(code: string): number {
    let complexity = 1; // Base complexity
    
    const complexityPatterns = [
      /\bif\b/g,
      /\belif\b/g,
      /\bfor\b/g,
      /\bwhile\b/g,
      /\btry\b/g,
      /\bexcept\b/g,
      /\band\b/g,
      /\bor\b/g,
    ];
    
    for (const pattern of complexityPatterns) {
      const matches = code.match(pattern);
      if (matches) {
        complexity += matches.length;
      }
    }
    
    return complexity;
  }
  
  private computeCacheKey(context: FunctionContext): string {
    // Deterministic key based on function signature and body hash
    const bodyHash = this.hashCode(context.body);
    return `${context.name}:${context.signature}:${bodyHash}`;
  }
  
  private isCacheValid(cached: AnalysisCache, current: FunctionContext): boolean {
    // Structural comparison to detect meaningful changes
    return cached.context.signature === current.signature &&
           cached.context.body === current.body &&
           Date.now() < cached.validUntil;
  }
  
  private async performAnalysis(context: FunctionContext): Promise<AnalysisResult> {
    return new Promise((resolve, reject) => {
      const messageId = crypto.randomUUID();
      
      const handler = (e: MessageEvent) => {
        if (e.data.id === messageId) {
          this.advisorWorker.removeEventListener('message', handler);
          if (e.data.error) {
            reject(new Error(e.data.error));
          } else {
            resolve(e.data.result);
          }
        }
      };
      
      this.advisorWorker.addEventListener('message', handler);
      this.advisorWorker.postMessage({
        type: 'ANALYZE_FUNCTION',
        id: messageId,
        context,
      });
      
      // Timeout after 5 seconds
      setTimeout(() => {
        this.advisorWorker.removeEventListener('message', handler);
        reject(new Error('Analysis timeout'));
      }, 5000);
    });
  }
  
  private async getAnnotationCompletions(model: any, position: any) {
    const line = model.getLineContent(position.lineNumber);
    const wordInfo = model.getWordUntilPosition(position);
    
    if (line.includes('@depyler:') || line.includes('@depyler')) {
      return {
        suggestions: [
          {
            label: 'optimize_energy=true',
            kind: this.monaco.languages.CompletionItemKind.Property,
            insertText: 'optimize_energy=true',
            documentation: 'Enable energy-efficient optimizations including CPU and memory usage reduction',
            detail: 'Depyler Energy Optimization',
          },
          {
            label: 'string_strategy=zero_copy',
            kind: this.monaco.languages.CompletionItemKind.Property,
            insertText: 'string_strategy=zero_copy',
            documentation: 'Use zero-copy string handling where possible',
            detail: 'String Strategy',
          },
          {
            label: 'ownership_model=borrowed',
            kind: this.monaco.languages.CompletionItemKind.Property,
            insertText: 'ownership_model=borrowed',
            documentation: 'Use borrowed references instead of owned values',
            detail: 'Memory Ownership',
          },
          {
            label: 'safety_level=strict',
            kind: this.monaco.languages.CompletionItemKind.Property,
            insertText: 'safety_level=strict',
            documentation: 'Maximum safety checks and bounds checking',
            detail: 'Safety Level',
          },
          {
            label: 'parallel=true',
            kind: this.monaco.languages.CompletionItemKind.Property,
            insertText: 'parallel=true',
            documentation: 'Enable parallel execution where safe',
            detail: 'Parallelization',
          },
        ],
      };
    }
    
    return { suggestions: [] };
  }
  
  private async getOptimizationHints(model: any, range: any) {
    // This would analyze the code and provide inline hints
    // For now, return empty array
    return { hints: [] };
  }
  
  private async getCodeActions(model: any, range: any, context: any) {
    const actions = [];
    
    // Check for common anti-patterns and suggest fixes
    const text = model.getValueInRange(range);
    
    if (text.includes('range(len(')) {
      actions.push({
        title: 'Replace with enumerate()',
        kind: 'quickfix',
        edit: {
          edits: [{
            resource: model.uri,
            edit: {
              range: range,
              text: text.replace(/range\(len\(([^)]+)\)\)/, 'enumerate($1)'),
            },
          }],
        },
      });
    }
    
    return { actions };
  }
  
  private hashCode(str: string): number {
    let hash = 0;
    for (let i = 0; i < str.length; i++) {
      const char = str.charCodeAt(i);
      hash = ((hash << 5) - hash) + char;
      hash = hash & hash; // Convert to 32-bit integer
    }
    return Math.abs(hash);
  }
  
  private debounce<T extends (...args: any[]) => any>(
    func: T,
    wait: number
  ): (...args: Parameters<T>) => void {
    let timeout: NodeJS.Timeout;
    
    return (...args: Parameters<T>) => {
      clearTimeout(timeout);
      timeout = setTimeout(() => func(...args), wait);
    };
  }
  
  public dispose() {
    this.advisorWorker.terminate();
    this.analysisCache.clear();
    this.pendingAnalysis.clear();
  }
}