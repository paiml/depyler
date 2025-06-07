# Depyler Playground UX Specification

## 1. Overview

The Depyler Playground provides a zero-friction web interface for Python-to-Rust transpilation, modeled after TypeScript Playground's proven UX patterns. The interface prioritizes real-time feedback, shareability, and educational value while maintaining sub-100ms responsiveness.

### 1.1 Core Principles

- **Instant Gratification**: Transpilation begins within 50ms of typing pause
- **Progressive Disclosure**: Advanced features hidden until needed
- **URL-First Design**: Every state change reflected in URL
- **Mobile-First Responsive**: Touch-optimized with desktop enhancements
- **Accessibility-First**: WCAG 2.1 AA compliance minimum

### 1.2 Target Metrics

```
Initial Load: < 2s (P90)
Time to Interactive: < 3s (P90)
Transpilation Latency: < 100ms (P50), < 300ms (P90)
Memory Usage: < 50MB baseline, < 200MB peak
Frame Rate: 60fps during all animations
```

## 2. Layout Architecture

### 2.1 Primary Layout Grid

```
┌─────────────────────────────────────────────────────────────┐
│ Header Toolbar (48px)                                       │
├─────────────────────────┬───────────────────────────────────┤
│                         │                                   │
│   Python Editor         │   Rust Output                     │
│   (50% - 4px)          │   (50% - 4px)                    │
│                         │                                   │
├─────────────────────────┴───────────────────────────────────┤
│ Status Bar (32px)                                           │
└─────────────────────────────────────────────────────────────┘
```

### 2.2 Responsive Breakpoints

```typescript
const breakpoints = {
  mobile: 640,    // Stack panels vertically
  tablet: 1024,   // Show side-by-side, hide settings
  desktop: 1280,  // Full feature set
  wide: 1920      // Multi-panel layouts
};
```

### 2.3 Panel Management

- **Draggable Splitter**: 8px hit target, 1px visual
- **Panel Constraints**: Min 200px, max 80% viewport
- **Double-Click**: Reset to 50/50 split
- **Keyboard**: Alt+[ and Alt+] to resize by 10%

## 3. Core Components

### 3.1 Header Toolbar

```typescript
interface ToolbarConfig {
  left: [
    { type: 'logo', width: 120 },
    { type: 'dropdown', id: 'examples', width: 200 },
    { type: 'button', id: 'run', variant: 'primary' }
  ],
  center: [
    { type: 'tabs', id: 'mode', options: ['Standard', 'Lambda', 'Optimize'] }
  ],
  right: [
    { type: 'button', id: 'share', icon: 'link' },
    { type: 'button', id: 'export', icon: 'download' },
    { type: 'toggle', id: 'settings', icon: 'cog' }
  ]
}
```

### 3.2 Editor Configuration

```typescript
const editorOptions: monaco.editor.IStandaloneEditorConstructionOptions = {
  theme: 'depyler-dark',
  fontSize: 14,
  fontFamily: 'JetBrains Mono, Consolas, monospace',
  minimap: { enabled: false },
  scrollBeyondLastLine: false,
  lineNumbers: 'on',
  glyphMargin: true,  // For breakpoints/annotations
  folding: true,
  formatOnPaste: true,
  formatOnType: true,
  autoIndent: 'full',
  tabSize: 4,
  insertSpaces: true,
  wordWrap: 'on',
  wrappingStrategy: 'advanced',
  quickSuggestions: {
    other: true,
    comments: false,
    strings: false
  }
};
```

### 3.3 Real-time Transpilation

```typescript
class TranspilationManager {
  private worker: Worker;
  private debounceTimer: number;
  private readonly DEBOUNCE_MS = 300;
  
  async transpile(source: string, options: TranspileOptions): Promise<Result> {
    // Cancel pending transpilation
    this.worker.postMessage({ type: 'cancel' });
    
    // Debounce
    clearTimeout(this.debounceTimer);
    this.debounceTimer = setTimeout(() => {
      this.performTranspilation(source, options);
    }, this.DEBOUNCE_MS);
  }
  
  private async performTranspilation(source: string, options: TranspileOptions) {
    const start = performance.now();
    
    this.worker.postMessage({
      type: 'transpile',
      payload: { source, options }
    });
    
    // Update UI immediately with loading state
    this.emit('transpiling', { estimatedTime: this.estimateTime(source) });
  }
}
```

## 4. Interactive Features

### 4.1 Smart Selection Synchronization

When user selects code in either panel:

```typescript
interface SelectionSync {
  python: { start: Position, end: Position };
  rust: { start: Position, end: Position };
  mapping: SourceMap;
}

function syncSelection(selection: monaco.Selection, source: 'python' | 'rust') {
  const mapped = sourceMap.map(selection, source);
  const targetEditor = source === 'python' ? rustEditor : pythonEditor;
  
  targetEditor.setSelection(mapped);
  targetEditor.revealLineInCenter(mapped.startLineNumber);
  
  // Highlight with subtle animation
  highlightRange(targetEditor, mapped, {
    className: 'synchronized-selection',
    duration: 2000
  });
}
```

### 4.2 Inline Annotations

```typescript
interface AnnotationWidget {
  line: number;
  type: 'hint' | 'warning' | 'optimization';
  message: string;
  suggestion?: {
    text: string;
    apply: () => void;
  };
}

class AnnotationProvider {
  provide(model: monaco.editor.ITextModel): AnnotationWidget[] {
    const annotations: AnnotationWidget[] = [];
    
    // Energy optimization hints
    if (hasNestedLoops(model)) {
      annotations.push({
        line: loopStart.line,
        type: 'optimization',
        message: 'Nested loops detected. Consider @depyler:vectorize',
        suggestion: {
          text: 'Add vectorization hint',
          apply: () => insertAnnotation(model, loopStart.line, '@depyler:vectorize')
        }
      });
    }
    
    return annotations;
  }
}
```

### 4.3 Live Metrics Display

```html
<div class="metrics-bar">
  <div class="metric" data-tooltip="Estimated based on CPU cycles">
    <Icon name="battery" />
    <span class="value">-74%</span>
    <span class="label">Energy</span>
  </div>
  
  <div class="metric" data-tooltip="WebAssembly execution vs Python baseline">
    <Icon name="rocket" />
    <span class="value">12.3x</span>
    <span class="label">Faster</span>
  </div>
  
  <div class="metric" data-tooltip="Rust binary size after optimization">
    <Icon name="package" />
    <span class="value">18KB</span>
    <span class="label">Binary</span>
  </div>
  
  <div class="metric animated" data-tooltip="Real-time transpilation duration">
    <Icon name="clock" />
    <span class="value">87ms</span>
    <span class="label">Time</span>
  </div>
</div>
```

## 5. URL State Management

### 5.1 State Encoding

```typescript
interface PlaygroundState {
  code: string;
  options: TranspileOptions;
  ui: UIState;
}

class URLStateManager {
  private readonly VERSION = 1;
  
  encode(state: PlaygroundState): string {
    const compressed = lz.compressToEncodedURIComponent(
      JSON.stringify({
        v: this.VERSION,
        c: state.code,
        o: this.encodeOptions(state.options),
        u: this.encodeUI(state.ui)
      })
    );
    
    return `#code/${compressed}`;
  }
  
  decode(hash: string): PlaygroundState | null {
    try {
      const compressed = hash.replace('#code/', '');
      const json = lz.decompressFromEncodedURIComponent(compressed);
      const data = JSON.parse(json);
      
      if (data.v !== this.VERSION) {
        return this.migrate(data);
      }
      
      return {
        code: data.c,
        options: this.decodeOptions(data.o),
        ui: this.decodeUI(data.u)
      };
    } catch {
      return null;
    }
  }
}
```

### 5.2 Share Functionality

```typescript
async function sharePlayground(): Promise<string> {
  const state = captureCurrentState();
  const encoded = urlManager.encode(state);
  
  // For long URLs, use shortener service
  if (encoded.length > 2000) {
    const response = await fetch('/api/shorten', {
      method: 'POST',
      body: JSON.stringify({ state }),
      headers: { 'Content-Type': 'application/json' }
    });
    
    const { shortId } = await response.json();
    return `${window.location.origin}/p/${shortId}`;
  }
  
  return `${window.location.origin}/${encoded}`;
}
```

## 6. Examples System

### 6.1 Example Structure

```typescript
interface Example {
  id: string;
  category: 'basics' | 'lambda' | 'optimization' | 'advanced';
  title: string;
  description: string;
  code: string;
  options?: Partial<TranspileOptions>;
  annotations?: string[];
  expectedMetrics?: {
    energySavings: number;
    speedup: number;
    binarySize: number;
  };
}

const examples: Example[] = [
  {
    id: 'fibonacci',
    category: 'basics',
    title: 'Fibonacci Sequence',
    description: 'Classic recursive algorithm with memoization',
    code: `def fibonacci(n: int) -> int:
    """Calculate nth Fibonacci number."""
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)`,
    expectedMetrics: {
      energySavings: 82,
      speedup: 15.3,
      binarySize: 12288
    }
  },
  // ... more examples
];
```

### 6.2 Example Loading Animation

```css
.example-transition {
  animation: fadeInCode 0.3s ease-out;
}

@keyframes fadeInCode {
  0% {
    opacity: 0;
    transform: translateY(-10px);
  }
  100% {
    opacity: 1;
    transform: translateY(0);
  }
}

/* Syntax highlight animation */
.token {
  animation: tokenFade 0.1s ease-out;
  animation-fill-mode: both;
}

.token:nth-child(n) {
  animation-delay: calc(n * 0.01s);
}
```

## 7. Export Functionality

### 7.1 Export Options

```typescript
interface ExportOptions {
  format: 'file' | 'project' | 'gist' | 'codesandbox';
  includeTests: boolean;
  includeCargoToml: boolean;
  optimizationLevel: 'debug' | 'release';
}

class ExportManager {
  async exportProject(options: ExportOptions): Promise<Blob | string> {
    const files = {
      'src/main.rs': this.wrapInMain(this.rustCode),
      'Cargo.toml': this.generateCargoToml(options),
      'README.md': this.generateReadme(),
      'original.py': this.pythonCode
    };
    
    if (options.includeTests) {
      files['src/tests.rs'] = this.generateTests();
    }
    
    switch (options.format) {
      case 'project':
        return this.createZip(files);
      case 'gist':
        return this.createGist(files);
      case 'codesandbox':
        return this.createCodeSandbox(files);
      default:
        return new Blob([files['src/main.rs']], { type: 'text/plain' });
    }
  }
}
```

## 8. Settings Panel

### 8.1 Settings Structure

```typescript
interface Settings {
  transpilation: {
    mode: 'standard' | 'lambda' | 'optimize';
    target: 'stable' | 'nightly';
    edition: '2018' | '2021' | '2024';
    optimizationLevel: 0 | 1 | 2 | 3;
    features: Set<'async' | 'serde' | 'rayon' | 'tokio'>;
  };
  editor: {
    theme: 'light' | 'dark' | 'high-contrast';
    fontSize: number;
    wordWrap: boolean;
    showAnnotations: boolean;
    autoFormat: boolean;
  };
  ui: {
    layout: 'horizontal' | 'vertical' | 'tabs';
    showMetrics: boolean;
    autoRun: boolean;
    preserveLog: boolean;
  };
}
```

### 8.2 Settings UI

```html
<aside class="settings-panel" data-state="collapsed">
  <div class="settings-section">
    <h3>Transpilation</h3>
    <div class="setting-group">
      <label for="optimization">Optimization Level</label>
      <input type="range" id="optimization" min="0" max="3" />
      <span class="value">2</span>
    </div>
    
    <div class="setting-group">
      <label for="target">Rust Target</label>
      <select id="target">
        <option value="stable">Stable</option>
        <option value="nightly">Nightly</option>
      </select>
    </div>
    
    <div class="setting-group">
      <label>Features</label>
      <div class="checkbox-group">
        <label><input type="checkbox" value="async" /> Async/Await</label>
        <label><input type="checkbox" value="serde" /> Serialization</label>
        <label><input type="checkbox" value="rayon" /> Parallelization</label>
      </div>
    </div>
  </div>
</aside>
```

## 9. Performance Optimizations

### 9.1 Web Worker Architecture

```typescript
// main thread
const transpiler = new TranspilerWorker();

transpiler.on('progress', (progress) => {
  updateProgressBar(progress);
});

transpiler.on('metrics', (metrics) => {
  updateMetricsDisplay(metrics);
});

// worker.ts
self.onmessage = async (e) => {
  const { type, payload } = e.data;
  
  switch (type) {
    case 'transpile':
      const wasm = await getWasmInstance();
      const result = wasm.transpile(payload.source, payload.options);
      
      self.postMessage({
        type: 'result',
        payload: {
          rust: result.code,
          metrics: result.metrics,
          sourceMap: result.sourceMap
        }
      });
      break;
      
    case 'cancel':
      wasm.cancel_current();
      break;
  }
};
```

### 9.2 Virtual Scrolling for Large Files

```typescript
class VirtualScroller {
  private readonly OVERSCAN = 5;
  private readonly LINE_HEIGHT = 22;
  
  calculateVisibleRange(scrollTop: number, clientHeight: number): Range {
    const startLine = Math.floor(scrollTop / this.LINE_HEIGHT);
    const endLine = Math.ceil((scrollTop + clientHeight) / this.LINE_HEIGHT);
    
    return {
      start: Math.max(0, startLine - this.OVERSCAN),
      end: endLine + this.OVERSCAN
    };
  }
  
  renderVisibleLines(lines: string[], range: Range): HTMLElement {
    const container = document.createElement('div');
    container.style.height = `${lines.length * this.LINE_HEIGHT}px`;
    
    const visible = document.createElement('div');
    visible.style.transform = `translateY(${range.start * this.LINE_HEIGHT}px)`;
    
    for (let i = range.start; i < range.end && i < lines.length; i++) {
      const line = this.renderLine(lines[i], i);
      visible.appendChild(line);
    }
    
    container.appendChild(visible);
    return container;
  }
}
```

## 10. Accessibility

### 10.1 Keyboard Navigation

```typescript
const keyboardShortcuts: KeyboardShortcut[] = [
  { key: 'Ctrl+Enter', action: 'transpile', description: 'Run transpilation' },
  { key: 'Ctrl+S', action: 'format', description: 'Format code' },
  { key: 'Ctrl+D', action: 'toggleDiff', description: 'Toggle diff view' },
  { key: 'Ctrl+/', action: 'toggleComment', description: 'Toggle comment' },
  { key: 'F1', action: 'commandPalette', description: 'Open command palette' },
  { key: 'Alt+Shift+F', action: 'format', description: 'Format document' },
  { key: 'Ctrl+Shift+P', action: 'commandPalette', description: 'Command palette' },
  { key: 'Escape', action: 'closePanels', description: 'Close open panels' }
];
```

### 10.2 Screen Reader Support

```html
<div role="application" aria-label="Depyler Playground">
  <div role="region" aria-label="Python Editor" aria-describedby="python-desc">
    <span id="python-desc" class="sr-only">
      Edit Python code here. Press Ctrl+Enter to transpile to Rust.
    </span>
    <div role="textbox" 
         aria-multiline="true" 
         aria-label="Python code editor"
         aria-live="polite"
         aria-atomic="true">
    </div>
  </div>
  
  <div role="region" aria-label="Rust Output" aria-live="polite">
    <div role="status" aria-label="Transpilation status">
      <span class="sr-only">Transpilation complete in 87 milliseconds</span>
    </div>
  </div>
</div>
```

## 11. Error Handling

### 11.1 Error Display

```typescript
interface TranspileError {
  type: 'syntax' | 'type' | 'unsupported' | 'internal';
  message: string;
  line?: number;
  column?: number;
  suggestion?: string;
}

function displayError(error: TranspileError) {
  // Inline error widget
  if (error.line) {
    monaco.editor.setModelMarkers(pythonModel, 'depyler', [{
      startLineNumber: error.line,
      startColumn: error.column || 1,
      endLineNumber: error.line,
      endColumn: error.column || 1000,
      message: error.message,
      severity: monaco.MarkerSeverity.Error
    }]);
  }
  
  // Error panel
  showErrorPanel({
    title: getErrorTitle(error.type),
    message: error.message,
    suggestion: error.suggestion,
    actions: [
      { label: 'View Documentation', href: `/docs/errors/${error.type}` },
      { label: 'Report Issue', onClick: () => reportIssue(error) }
    ]
  });
}
```

## 12. Mobile Optimization

### 12.1 Touch Interactions

```typescript
class TouchManager {
  private startX: number = 0;
  private currentPanel: 'python' | 'rust' = 'python';
  
  handleTouchStart(e: TouchEvent) {
    this.startX = e.touches[0].clientX;
  }
  
  handleTouchMove(e: TouchEvent) {
    const deltaX = e.touches[0].clientX - this.startX;
    
    // Swipe between panels
    if (Math.abs(deltaX) > 50) {
      if (deltaX > 0 && this.currentPanel === 'rust') {
        this.showPanel('python');
      } else if (deltaX < 0 && this.currentPanel === 'python') {
        this.showPanel('rust');
      }
    }
  }
  
  showPanel(panel: 'python' | 'rust') {
    const transform = panel === 'python' ? 'translateX(0)' : 'translateX(-100%)';
    document.querySelector('.editor-container').style.transform = transform;
    this.currentPanel = panel;
  }
}
```

### 12.2 Mobile Layout

```css
@media (max-width: 640px) {
  .playground-container {
    grid-template-rows: 48px 1fr 100px 32px;
    grid-template-columns: 1fr;
  }
  
  .editor-container {
    display: flex;
    width: 200%;
    transition: transform 0.3s ease-out;
  }
  
  .editor-panel {
    width: 50%;
    flex-shrink: 0;
  }
  
  .metrics-bar {
    display: flex;
    overflow-x: auto;
    -webkit-overflow-scrolling: touch;
  }
  
  .settings-panel {
    position: fixed;
    bottom: 0;
    left: 0;
    right: 0;
    transform: translateY(100%);
    transition: transform 0.3s ease-out;
  }
  
  .settings-panel[data-state="open"] {
    transform: translateY(0);
  }
}
```

## 13. Implementation Checklist

### Phase 1: Core Infrastructure (Week 1-2)
- [ ] Monaco editor integration
- [ ] WASM module loading
- [ ] Basic transpilation flow
- [ ] URL state management
- [ ] Split panel layout

### Phase 2: Interactive Features (Week 3-4)
- [ ] Real-time transpilation
- [ ] Selection synchronization
- [ ] Error highlighting
- [ ] Basic metrics display
- [ ] Example system

### Phase 3: Advanced Features (Week 5-6)
- [ ] Annotation suggestions
- [ ] Export functionality
- [ ] Settings panel
- [ ] Diff view
- [ ] Share functionality

### Phase 4: Polish & Performance (Week 7-8)
- [ ] Mobile optimization
- [ ] Accessibility audit
- [ ] Performance optimization
- [ ] Cross-browser testing
- [ ] Analytics integration

## 14. Success Metrics

```typescript
interface PlaygroundMetrics {
  performance: {
    initialLoad: Percentile<number>;      // Target: P90 < 2s
    timeToInteractive: Percentile<number>; // Target: P90 < 3s
    transpilationTime: Percentile<number>; // Target: P50 < 100ms
    memoryUsage: Percentile<number>;       // Target: P90 < 200MB
  };
  
  engagement: {
    sessionDuration: number;        // Target: > 5 minutes
    transpilationsPerSession: number; // Target: > 10
    shareRate: number;              // Target: > 5%
    exportRate: number;             // Target: > 2%
  };
  
  quality: {
    errorRate: number;              // Target: < 0.1%
    crashRate: number;              // Target: < 0.01%
    accessibilityScore: number;     // Target: 100
    lighthouseScore: number;        // Target: > 95
  };
}
```
