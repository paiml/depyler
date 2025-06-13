// Global type definitions
// Use number for browser timeout IDs
declare namespace NodeJS {
  type Timeout = number;
}

// Extend PerformanceNavigationTiming for compatibility
interface PerformanceNavigationTiming {
  navigationStart?: number;
}

// Ensure this file is treated as a module
export {};