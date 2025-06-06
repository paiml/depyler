import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { QualityTelemetry } from '../telemetry/quality-telemetry';

// Mock navigator APIs
const mockNavigator = {
  sendBeacon: vi.fn(() => true),
  userAgent: 'Mozilla/5.0 (Test Environment)',
  connection: {
    effectiveType: '4g'
  },
  deviceMemory: 8
};

Object.defineProperty(global, 'navigator', {
  value: mockNavigator,
  writable: true
});

// Mock window APIs
const mockWindow = {
  innerWidth: 1920,
  innerHeight: 1080,
  addEventListener: vi.fn()
};

Object.defineProperty(global, 'window', {
  value: mockWindow,
  writable: true
});

// Mock document
const mockDocument = {
  visibilityState: 'visible',
  addEventListener: vi.fn()
};

Object.defineProperty(global, 'document', {
  value: mockDocument,
  writable: true
});

// Mock fetch
global.fetch = vi.fn(() =>
  Promise.resolve({
    ok: true,
    json: () => Promise.resolve({})
  })
) as any;

// Mock TextEncoder
global.TextEncoder = vi.fn(() => ({
  encode: vi.fn((text: string) => new Uint8Array(text.length))
})) as any;

describe('QualityTelemetry', () => {
  let telemetry: QualityTelemetry;
  
  beforeEach(() => {
    vi.clearAllMocks();
    vi.useFakeTimers();
    telemetry = new QualityTelemetry();
  });
  
  afterEach(() => {
    vi.useRealTimers();
  });

  describe('Initialization', () => {
    it('generates a unique session ID', () => {
      const telemetry1 = new QualityTelemetry();
      const telemetry2 = new QualityTelemetry();
      
      expect((telemetry1 as any).sessionId).toBeDefined();
      expect((telemetry2 as any).sessionId).toBeDefined();
      expect((telemetry1 as any).sessionId).not.toBe((telemetry2 as any).sessionId);
    });
    
    it('sets up visibility change listener', () => {
      expect(mockWindow.addEventListener).toHaveBeenCalledWith(
        'visibilitychange',
        expect.any(Function)
      );
    });
    
    it('schedules periodic flush', () => {
      expect((telemetry as any).flushTimer).toBeDefined();
    });
  });

  describe('Code Analysis', () => {
    it('analyzes basic code metrics correctly', () => {
      const code = `
def calculate_sum(numbers: list) -> int:
    total = 0
    for num in numbers:
        if num > 0:
            total += num
    return total

def process_data(data: Dict[str, Any]) -> List[str]:
    result = []
    for key, value in data.items():
        if isinstance(value, str):
            result.append(value.upper())
    return result
      `;
      
      const metrics = (telemetry as any).analyzeCode(code);
      
      expect(metrics).toEqual({
        sizeBytes: expect.any(Number),
        numFunctions: 2,
        numLoops: 2,
        numConditionals: 2,
        maxNesting: expect.any(Number),
        usesAsyncAwait: false,
        usesComplexTypes: true, // Dict[str, Any] and List[str]
        hasAnnotations: false
      });
      
      expect(metrics.sizeBytes).toBeGreaterThan(0);
      expect(metrics.maxNesting).toBeGreaterThan(0);
    });
    
    it('detects async functions', () => {
      const asyncCode = `
async def fetch_data(url: str) -> dict:
    async with aiohttp.ClientSession() as session:
        async with session.get(url) as response:
            return await response.json()
      `;
      
      const metrics = (telemetry as any).analyzeCode(asyncCode);
      expect(metrics.usesAsyncAwait).toBe(true);
    });
    
    it('detects Depyler annotations', () => {
      const annotatedCode = `
# @depyler: optimize_energy=true, string_strategy=zero_copy
def optimized_function(text: str) -> str:
    return text.upper()
      `;
      
      const metrics = (telemetry as any).analyzeCode(annotatedCode);
      expect(metrics.hasAnnotations).toBe(true);
    });
    
    it('calculates nesting depth correctly', () => {
      const nestedCode = `
def deeply_nested():
    if True:
        for i in range(10):
            if i > 5:
                for j in range(i):
                    if j % 2 == 0:
                        print(j)
      `;
      
      const metrics = (telemetry as any).analyzeCode(nestedCode);
      expect(metrics.maxNesting).toBeGreaterThan(2);
    });
  });

  describe('Environment Capture', () => {
    it('captures browser environment correctly', () => {
      const environment = (telemetry as any).captureEnvironment();
      
      expect(environment).toEqual({
        browser: 'Mozilla/5.0 (Test Environment)',
        viewport: {
          width: 1920,
          height: 1080
        },
        connection: '4g',
        deviceMemory: 8
      });
    });
    
    it('handles missing connection API gracefully', () => {
      const originalConnection = mockNavigator.connection;
      delete (mockNavigator as any).connection;
      
      const environment = (telemetry as any).captureEnvironment();
      expect(environment.connection).toBe('unknown');
      
      mockNavigator.connection = originalConnection;
    });
  });

  describe('Quality Event Recording', () => {
    it('records quality events with complete payload', () => {
      const mockEvent = {
        type: 'transpilation_complete',
        timestamp: Date.now(),
        metrics: {
          transpile_time_ms: 45,
          energy_reduction: {
            joules: 0.001,
            wattsAverage: 1.0,
            co2Grams: 0.000475
          }
        },
        pmatScore: {
          productivity: 0.85,
          maintainability: 0.92,
          accessibility: 0.88,
          testability: 0.91,
          tdg: 0.89
        },
        severity: 'info' as const
      };
      
      const codeContext = 'def add(a: int, b: int) -> int:\n    return a + b';
      
      telemetry.recordQualityEvent(mockEvent, codeContext);
      
      const buffer = (telemetry as any).buffer;
      expect(buffer).toHaveLength(1);
      
      const payload = buffer[0];
      expect(payload).toEqual({
        sessionId: expect.any(String),
        timestamp: expect.any(Number),
        metrics: mockEvent.metrics,
        pmatScore: mockEvent.pmatScore,
        codeMetrics: expect.objectContaining({
          sizeBytes: expect.any(Number),
          numFunctions: 1,
          numLoops: 0,
          numConditionals: 0
        }),
        environment: expect.objectContaining({
          browser: expect.any(String),
          viewport: expect.any(Object)
        }),
        qualityEvents: [mockEvent]
      });
    });
    
    it('immediately flushes critical events', () => {
      const criticalEvent = {
        type: 'critical_error',
        timestamp: Date.now(),
        metrics: {},
        pmatScore: {},
        severity: 'critical' as const
      };
      
      const flushSpy = vi.spyOn(telemetry as any, 'flush');
      
      telemetry.recordQualityEvent(criticalEvent, 'test code');
      
      expect(flushSpy).toHaveBeenCalled();
    });
  });

  describe('Batching and Flushing', () => {
    it('batches multiple events before flushing', () => {
      const events = [
        { type: 'event1', severity: 'info', timestamp: Date.now(), metrics: {}, pmatScore: {} },
        { type: 'event2', severity: 'info', timestamp: Date.now(), metrics: {}, pmatScore: {} },
        { type: 'event3', severity: 'info', timestamp: Date.now(), metrics: {}, pmatScore: {} }
      ];
      
      events.forEach(event => {
        telemetry.recordQualityEvent(event as any, 'test code');
      });
      
      const buffer = (telemetry as any).buffer;
      expect(buffer).toHaveLength(3);
    });
    
    it('uses sendBeacon when available', () => {
      const event = {
        type: 'test_event',
        severity: 'info' as const,
        timestamp: Date.now(),
        metrics: {},
        pmatScore: {}
      };
      
      telemetry.recordQualityEvent(event, 'test code');
      (telemetry as any).flush();
      
      expect(mockNavigator.sendBeacon).toHaveBeenCalledWith(
        '/api/telemetry',
        expect.any(String)
      );
    });
    
    it('falls back to fetch when sendBeacon unavailable', () => {
      const originalSendBeacon = mockNavigator.sendBeacon;
      delete (mockNavigator as any).sendBeacon;
      
      const event = {
        type: 'test_event',
        severity: 'info' as const,
        timestamp: Date.now(),
        metrics: {},
        pmatScore: {}
      };
      
      telemetry.recordQualityEvent(event, 'test code');
      (telemetry as any).flush();
      
      expect(global.fetch).toHaveBeenCalledWith(
        '/api/telemetry',
        expect.objectContaining({
          method: 'POST',
          body: expect.any(String),
          keepalive: true
        })
      );
      
      mockNavigator.sendBeacon = originalSendBeacon;
    });
    
    it('clears buffer after successful flush', () => {
      const event = {
        type: 'test_event',
        severity: 'info' as const,
        timestamp: Date.now(),
        metrics: {},
        pmatScore: {}
      };
      
      telemetry.recordQualityEvent(event, 'test code');
      expect((telemetry as any).buffer).toHaveLength(1);
      
      (telemetry as any).flush();
      expect((telemetry as any).buffer).toHaveLength(0);
    });
    
    it('handles flush errors gracefully', () => {
      mockNavigator.sendBeacon.mockReturnValue(false);
      global.fetch = vi.fn(() => Promise.reject(new Error('Network error'))) as any;
      
      const event = {
        type: 'test_event',
        severity: 'info' as const,
        timestamp: Date.now(),
        metrics: {},
        pmatScore: {}
      };
      
      telemetry.recordQualityEvent(event, 'test code');
      
      expect(() => (telemetry as any).flush()).not.toThrow();
    });
  });

  describe('Periodic Flushing', () => {
    it('schedules periodic flush', () => {
      expect((telemetry as any).flushTimer).toBeDefined();
    });
    
    it('flushes on visibility change to hidden', () => {
      const flushSpy = vi.spyOn(telemetry as any, 'flush');
      
      // Simulate visibility change
      mockDocument.visibilityState = 'hidden';
      const visibilityHandler = mockWindow.addEventListener.mock.calls
        .find(call => call[0] === 'visibilitychange')?.[1];
      
      if (visibilityHandler) {
        (visibilityHandler as Function)();
        expect(flushSpy).toHaveBeenCalled();
      }
    });
  });

  describe('Performance', () => {
    it('handles large code analysis efficiently', () => {
      const largeCode = 'def function():\n    pass\n'.repeat(1000);
      
      const startTime = performance.now();
      const metrics = (telemetry as any).analyzeCode(largeCode);
      const analysisTime = performance.now() - startTime;
      
      expect(analysisTime).toBeLessThan(100); // Should analyze within 100ms
      expect(metrics.numFunctions).toBe(1000);
    });
    
    it('batches telemetry efficiently', () => {
      const events = Array.from({ length: 100 }, (_, i) => ({
        type: `event_${i}`,
        severity: 'info' as const,
        timestamp: Date.now(),
        metrics: {},
        pmatScore: {}
      }));
      
      const startTime = performance.now();
      
      events.forEach(event => {
        telemetry.recordQualityEvent(event, 'test code');
      });
      
      const recordingTime = performance.now() - startTime;
      
      expect(recordingTime).toBeLessThan(50); // Should record 100 events within 50ms
      expect((telemetry as any).buffer).toHaveLength(100);
    });
  });

  describe('Session Management', () => {
    it('generates valid UUID for session ID', () => {
      const sessionId = (telemetry as any).sessionId;
      
      expect(sessionId).toMatch(/^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i);
    });
    
    it('maintains consistent session ID throughout lifecycle', () => {
      const initialSessionId = (telemetry as any).sessionId;
      
      telemetry.recordQualityEvent({
        type: 'test',
        severity: 'info',
        timestamp: Date.now(),
        metrics: {},
        pmatScore: {}
      }, 'test');
      
      const eventSessionId = (telemetry as any).buffer[0].sessionId;
      expect(eventSessionId).toBe(initialSessionId);
    });
  });
});