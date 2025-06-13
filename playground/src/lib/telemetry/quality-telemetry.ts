/// <reference path="../../vite-env.d.ts" />
/// <reference path="../../global.d.ts" />
import { PlaygroundMetrics, PmatScore, QualityEvent } from "@/types";

interface CodeMetrics {
  sizeBytes: number;
  numFunctions: number;
  numLoops: number;
  numConditionals: number;
  maxNesting: number;
  usesAsyncAwait: boolean;
  usesComplexTypes: boolean;
  hasAnnotations: boolean;
}

interface TelemetryPayload {
  sessionId: string;
  timestamp: number;
  metrics: PlaygroundMetrics;
  pmatScore: PmatScore;
  codeMetrics: CodeMetrics;
  environment: EnvironmentInfo;
  qualityEvents: QualityEvent[];
}

interface EnvironmentInfo {
  browser: string;
  viewport: { width: number; height: number };
  connection: string;
  deviceMemory?: number;
  platform: string;
  language: string;
  timezone: string;
}

export class QualityTelemetry {
  private sessionId: string;
  private buffer: TelemetryPayload[] = [];
  private flushTimer?: number;
  private isEnabled: boolean = true;
  private endpoint: string;

  constructor(endpoint: string = "/api/telemetry") {
    this.sessionId = this.generateSessionId();
    this.endpoint = endpoint;
    
    // Disable telemetry in development to avoid 404 errors
    if (import.meta.env.DEV) {
      this.isEnabled = false;
      return;
    }

    // Batch telemetry for efficiency
    this.scheduleFlush();

    // Flush on page unload
    this.setupUnloadHandlers();

    // Check for Do Not Track
    this.checkPrivacySettings();
  }

  private generateSessionId(): string {
    // Use crypto.randomUUID if available (modern browsers)
    if (typeof crypto !== 'undefined' && crypto.randomUUID) {
      return crypto.randomUUID();
    }
    
    // Fallback for older browsers
    const timestamp = Date.now().toString(36);
    const randomString = Math.random().toString(36).substring(2, 15);
    return `${timestamp}-${randomString}`;
  }

  private checkPrivacySettings() {
    // Respect Do Not Track
    if (
      navigator.doNotTrack === "1" ||
      (window as any).doNotTrack === "1" ||
      (navigator as any).msDoNotTrack === "1"
    ) {
      this.isEnabled = false;
      console.log("Telemetry disabled due to Do Not Track preference");
    }

    // Check for local storage opt-out
    try {
      if (localStorage.getItem("depyler-telemetry-optout") === "true") {
        this.isEnabled = false;
        console.log("Telemetry disabled due to user preference");
      }
    } catch (_e) {
      // localStorage might not be available
    }
  }

  private setupUnloadHandlers() {
    // Flush on page visibility change
    document.addEventListener("visibilitychange", () => {
      if (document.visibilityState === "hidden") {
        this.flush();
      }
    });

    // Flush on page unload
    globalThis.addEventListener("beforeunload", () => {
      this.flush();
    });

    // Flush on page freeze (mobile Safari)
    globalThis.addEventListener("pagehide", () => {
      this.flush();
    });
  }

  recordQualityEvent(
    event: QualityEvent,
    codeContext: string,
    metrics?: PlaygroundMetrics,
    pmatScore?: PmatScore,
  ) {
    if (!this.isEnabled) return;

    const codeMetrics = this.analyzeCode(codeContext);
    const environment = this.captureEnvironment();

    const payload: TelemetryPayload = {
      sessionId: this.sessionId,
      timestamp: Date.now(),
      metrics: metrics || this.createEmptyMetrics(),
      pmatScore: pmatScore || this.createEmptyPmatScore(),
      codeMetrics,
      environment,
      qualityEvents: [event],
    };

    this.buffer.push(payload);

    // Immediate send for critical events
    if (event.severity === "Critical") {
      this.flush();
    }
  }

  recordUserInteraction(
    action: string,
    _context: Record<string, any> = {},
    codeContext?: string,
  ) {
    if (!this.isEnabled) return;

    const event: QualityEvent = {
      timestamp: Date.now(),
      event_type: "PerformanceImprovement", // Generic type for user interactions
      severity: "Info",
      message: `User action: ${action}`,
      metrics_snapshot: undefined,
    };

    this.recordQualityEvent(
      event,
      codeContext || "",
      undefined,
      undefined,
    );
  }

  recordPerformanceMetrics(metrics: {
    transpileTime: number;
    executionTime: number;
    memoryUsage: number;
    codeSize: number;
  }) {
    if (!this.isEnabled) return;

    // Record performance data for analysis
    try {
      performance.mark("telemetry-record-start");

      const event: QualityEvent = {
        timestamp: Date.now(),
        event_type: "PerformanceImprovement",
        severity: "Info",
        message:
          `Performance: transpile=${metrics.transpileTime}ms, exec=${metrics.executionTime}ms`,
        metrics_snapshot: undefined,
      };

      this.recordQualityEvent(event, "", undefined, undefined);

      performance.mark("telemetry-record-end");
      performance.measure("telemetry-record", "telemetry-record-start", "telemetry-record-end");
    } catch (error) {
      console.warn("Failed to record performance metrics:", error);
    }
  }

  private analyzeCode(code: string): CodeMetrics {
    const lines = code.split("\n");
    const functionPattern = /^def\s+\w+\s*\(/gm;
    const loopPattern = /^\s*(for|while)\s+/gm;
    const conditionalPattern = /^\s*if\s+/gm;

    let maxNesting = 0;
    let currentNesting = 0;

    lines.forEach((line) => {
      const indent = line.search(/\S/);
      if (indent !== -1) {
        currentNesting = Math.floor(indent / 4);
        maxNesting = Math.max(maxNesting, currentNesting);
      }
    });

    return {
      sizeBytes: new TextEncoder().encode(code).length,
      numFunctions: (code.match(functionPattern) || []).length,
      numLoops: (code.match(loopPattern) || []).length,
      numConditionals: (code.match(conditionalPattern) || []).length,
      maxNesting,
      usesAsyncAwait: /async\s+def/.test(code),
      usesComplexTypes: /Dict\[|List\[|Tuple\[/.test(code),
      hasAnnotations: /#\s*@depyler:/.test(code),
    };
  }

  private captureEnvironment(): EnvironmentInfo {
    const connection = (navigator as any).connection || (navigator as any).mozConnection ||
      (navigator as any).webkitConnection;

    return {
      browser: this.getBrowserInfo(),
      viewport: {
        width: globalThis.innerWidth,
        height: globalThis.innerHeight,
      },
      connection: connection?.effectiveType || "unknown",
      deviceMemory: (navigator as any).deviceMemory,
      platform: navigator.platform,
      language: navigator.language,
      timezone: Intl.DateTimeFormat().resolvedOptions().timeZone,
    };
  }

  private getBrowserInfo(): string {
    const userAgent = navigator.userAgent;

    // Simplified browser detection
    if (userAgent.includes("Firefox/")) {
      const match = userAgent.match(/Firefox\/(\d+)/);
      return `Firefox ${match ? match[1] : "unknown"}`;
    } else if (userAgent.includes("Chrome/")) {
      const match = userAgent.match(/Chrome\/(\d+)/);
      return `Chrome ${match ? match[1] : "unknown"}`;
    } else if (userAgent.includes("Safari/") && !userAgent.includes("Chrome")) {
      const match = userAgent.match(/Version\/(\d+)/);
      return `Safari ${match ? match[1] : "unknown"}`;
    } else if (userAgent.includes("Edge/")) {
      const match = userAgent.match(/Edge\/(\d+)/);
      return `Edge ${match ? match[1] : "unknown"}`;
    }

    return "Unknown";
  }

  private scheduleFlush() {
    this.flushTimer = globalThis.setInterval(() => {
      if (this.buffer.length > 0) {
        this.flush();
      }
    }, 5000); // Flush every 5 seconds
  }

  private flush() {
    if (!this.isEnabled || this.buffer.length === 0) return;

    const payload = JSON.stringify({
      version: "1.0",
      source: "depyler-playground",
      data: this.buffer,
    });

    this.buffer = [];

    // Use sendBeacon for reliability
    if (navigator.sendBeacon && this.endpoint.startsWith("/")) {
      const success = navigator.sendBeacon(this.endpoint, payload);
      if (!success) {
        console.warn("Failed to send telemetry via sendBeacon");
        this.fallbackSend(payload);
      }
    } else {
      this.fallbackSend(payload);
    }
  }

  private fallbackSend(payload: string) {
    // Fallback to fetch with keepalive
    fetch(this.endpoint, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: payload,
      keepalive: true,
    }).catch((error) => {
      // Telemetry is best-effort, don't throw
      console.warn("Telemetry send failed:", error);
    });
  }

  private createEmptyMetrics(): PlaygroundMetrics {
    return {
      page_load: {
        ttfmp_ms: 0,
        tti_ms: 0,
        wasm_load_ms: 0,
        wasm_size_kb: 0,
      },
      transpilation: {
        latency_p95_ms: 0,
        complexity_bucket: "Simple",
        cache_hit_rate: 0,
        error_rate: 0,
      },
      execution: {
        rust_execution_ms: 0,
        python_execution_ms: 0,
        energy_savings_percent: 0,
        memory_usage_mb: 0,
      },
      quality_events: [],
    };
  }

  private createEmptyPmatScore(): PmatScore {
    return {
      productivity: 0,
      maintainability: 0,
      accessibility: 0,
      testability: 0,
      tdg: 0,
      timestamp: Date.now(),
    };
  }

  // Public API for opting out
  public optOut() {
    this.isEnabled = false;
    try {
      localStorage.setItem("depyler-telemetry-optout", "true");
    } catch (_e) {
      // localStorage might not be available
    }

    // Clear any buffered data
    this.buffer = [];

    console.log("Telemetry disabled");
  }

  // Public API for opting back in
  public optIn() {
    this.isEnabled = true;
    try {
      localStorage.removeItem("depyler-telemetry-optout");
    } catch (_e) {
      // localStorage might not be available
    }

    console.log("Telemetry enabled");
  }

  // Check if telemetry is enabled
  public isOptedIn(): boolean {
    return this.isEnabled;
  }

  // Get aggregated stats (for debugging)
  public getStats() {
    return {
      sessionId: this.sessionId,
      isEnabled: this.isEnabled,
      bufferSize: this.buffer.length,
      endpoint: this.endpoint,
    };
  }

  // Cleanup method
  public dispose() {
    if (this.flushTimer) {
      clearInterval(this.flushTimer);
    }

    // Final flush
    this.flush();
  }
}

// Singleton instance for the playground
export const telemetry = new QualityTelemetry("/api/telemetry");

// Helper functions for common telemetry tasks
export function recordCodeTranspilation(
  pythonCode: string,
  _rustCode: string,
  transpileTime: number,
  success: boolean,
) {
  telemetry.recordPerformanceMetrics({
    transpileTime,
    executionTime: 0,
    memoryUsage: 0,
    codeSize: pythonCode.length,
  });

  if (!success) {
    const event: QualityEvent = {
      timestamp: Date.now(),
      event_type: "ErrorThresholdExceeded",
      severity: "Warning",
      message: "Transpilation failed",
    };

    telemetry.recordQualityEvent(event, pythonCode);
  }
}

export function recordCodeExecution(
  _pythonTime: number,
  rustTime: number,
  energySavings: number,
  codeContext: string,
) {
  telemetry.recordPerformanceMetrics({
    transpileTime: 0,
    executionTime: rustTime,
    memoryUsage: 0,
    codeSize: codeContext.length,
  });

  if (energySavings > 50) {
    const event: QualityEvent = {
      timestamp: Date.now(),
      event_type: "EnergyEfficiencyImprovement",
      severity: "Info",
      message: `Significant energy savings: ${energySavings.toFixed(1)}%`,
    };

    telemetry.recordQualityEvent(event, codeContext);
  }
}

export function recordUserFeedback(rating: number, feedback: string, context: string) {
  const event: QualityEvent = {
    timestamp: Date.now(),
    event_type: "PerformanceImprovement",
    severity: "Info",
    message: `User feedback: rating=${rating}, feedback="${feedback}"`,
  };

  telemetry.recordQualityEvent(event, context);
}
