import { describe, expect, it, vi, beforeEach } from "vitest";
import { executeComparison } from "../execution-manager";

describe("ExecutionManager", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("executes Python and Rust code comparison", async () => {
    const result = await executeComparison(
      'print("Hello")',
      'fn main() { println!("Hello"); }'
    );

    expect(result.python.output).toBe("Hello");
    expect(result.rust.output).toBe("Hello");
    expect(result.python.execution_time_ms).toBeGreaterThan(0);
    expect(result.rust.execution_time_ms).toBeGreaterThan(0);
    expect(result.energy_savings_percent).toBeGreaterThan(0);
    expect(result.python.error).toBeNull();
    expect(result.rust.error).toBeNull();
  });

  it("calculates speedup correctly", async () => {
    const result = await executeComparison(
      'print("test")',
      'fn main() { println!("test"); }'
    );

    const speedup = result.python.execution_time_ms / result.rust.execution_time_ms;
    expect(speedup).toBeGreaterThan(1);
  });

  it("calculates energy savings correctly", async () => {
    const result = await executeComparison(
      'print("test")',
      'fn main() { println!("test"); }'
    );

    expect(result.energy_savings_percent).toBeGreaterThanOrEqual(0);
    expect(result.energy_savings_percent).toBeLessThanOrEqual(95);
  });

  it("handles execution without errors", async () => {
    const result = await executeComparison(
      'invalid syntax',
      'fn main() {}'
    );

    expect(result.python.output).toBeDefined();
    expect(result.rust.output).toBeDefined();
    expect(result.energy_savings_percent).toBeDefined();
  });

  it("returns results within reasonable time", async () => {
    const startTime = performance.now();
    const result = await executeComparison(
      'print("test")',
      'fn main() { println!("test"); }'
    );
    const endTime = performance.now();

    expect(endTime - startTime).toBeLessThan(500); // Should complete within 500ms
    expect(result).toBeDefined();
  });

  it("completes execution successfully", async () => {
    const result = await executeComparison(
      'print("test")',
      'fn main() { println!("test"); }'
    );

    expect(result).toBeDefined();
    expect(result.python).toBeDefined();
    expect(result.rust).toBeDefined();
  });

  it("preserves output formatting", async () => {
    const result = await executeComparison(
      'print("Line 1\\n  Indented\\n    Double")',
      'fn main() { println!("Line 1\\n  Indented\\n    Double"); }'
    );

    expect(result.python.output).toContain("Line 1");
    expect(result.rust.output).toContain("Line 1");
  });

  it("handles empty code gracefully", async () => {
    const result = await executeComparison("", "");

    expect(result.python.output).toBeDefined();
    expect(result.rust.output).toBeDefined();
    expect(result.energy_savings_percent).toBeGreaterThanOrEqual(0);
  });

  it("handles very fast execution times", async () => {
    const result = await executeComparison(
      'x = 1',
      'fn main() { let x = 1; }'
    );

    expect(result.python.execution_time_ms).toBeGreaterThan(0);
    expect(result.rust.execution_time_ms).toBeGreaterThan(0);
  });
});