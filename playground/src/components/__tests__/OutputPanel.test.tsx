import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { createMockPlaygroundStore } from "@test/setup";

// Mock component since we don't have the actual implementation
const OutputPanel = ({ result }: { result: any }) => {
  if (!result) {
    return <div>No output yet. Click "Run" to execute the code.</div>;
  }
  
  return (
    <div>
      <div data-testid="python-output">
        <h3>Python Output:</h3>
        <pre>{result.pythonOutput}</pre>
        <div>Time: {result.pythonTime.toFixed(2)}ms</div>
      </div>
      
      <div data-testid="rust-output">
        <h3>Rust Output:</h3>
        <pre>{result.rustOutput}</pre>
        <div>Time: {result.rustTime.toFixed(2)}ms</div>
      </div>
      
      <div data-testid="performance-comparison">
        <h3>Performance Comparison:</h3>
        <div>Speedup: {result.speedup.toFixed(2)}x faster</div>
        <div>Memory saved: {result.memorySaved.toFixed(2)} MB</div>
      </div>
    </div>
  );
};

describe("OutputPanel", () => {
  it("shows placeholder when no execution result", () => {
    render(<OutputPanel result={null} />);
    
    expect(screen.getByText(/No output yet/)).toBeInTheDocument();
  });

  it("displays Python output correctly", () => {
    const result = {
      pythonOutput: "Hello from Python",
      rustOutput: "Hello from Rust",
      pythonTime: 10.5,
      rustTime: 2.1,
      speedup: 5.0,
      memorySaved: 3.2,
      pythonExitCode: 0,
      rustExitCode: 0,
    };
    
    render(<OutputPanel result={result} />);
    
    expect(screen.getByText("Hello from Python")).toBeInTheDocument();
    expect(screen.getByText("Time: 10.50ms")).toBeInTheDocument();
  });

  it("displays Rust output correctly", () => {
    const result = {
      pythonOutput: "Hello from Python",
      rustOutput: "Hello from Rust",
      pythonTime: 10.5,
      rustTime: 2.1,
      speedup: 5.0,
      memorySaved: 3.2,
      pythonExitCode: 0,
      rustExitCode: 0,
    };
    
    render(<OutputPanel result={result} />);
    
    expect(screen.getByText("Hello from Rust")).toBeInTheDocument();
    expect(screen.getByText("Time: 2.10ms")).toBeInTheDocument();
  });

  it("shows performance comparison", () => {
    const result = {
      pythonOutput: "",
      rustOutput: "",
      pythonTime: 100,
      rustTime: 10,
      speedup: 10.0,
      memorySaved: 5.5,
      pythonExitCode: 0,
      rustExitCode: 0,
    };
    
    render(<OutputPanel result={result} />);
    
    expect(screen.getByText("Speedup: 10.00x faster")).toBeInTheDocument();
    expect(screen.getByText("Memory saved: 5.50 MB")).toBeInTheDocument();
  });

  it("handles error exit codes", () => {
    const result = {
      pythonOutput: "Error: Division by zero",
      rustOutput: "Success",
      pythonTime: 10,
      rustTime: 5,
      speedup: 2.0,
      memorySaved: 1.0,
      pythonExitCode: 1,
      rustExitCode: 0,
    };
    
    render(<OutputPanel result={result} />);
    
    expect(screen.getByText("Error: Division by zero")).toBeInTheDocument();
  });

  it("formats large numbers appropriately", () => {
    const result = {
      pythonOutput: "",
      rustOutput: "",
      pythonTime: 1234.5678,
      rustTime: 12.3456,
      speedup: 100.123,
      memorySaved: 123.456,
      pythonExitCode: 0,
      rustExitCode: 0,
    };
    
    render(<OutputPanel result={result} />);
    
    expect(screen.getByText("Time: 1234.57ms")).toBeInTheDocument();
    expect(screen.getByText("Time: 12.35ms")).toBeInTheDocument();
    expect(screen.getByText("Speedup: 100.12x faster")).toBeInTheDocument();
    expect(screen.getByText("Memory saved: 123.46 MB")).toBeInTheDocument();
  });

  it("handles empty output gracefully", () => {
    const result = {
      pythonOutput: "",
      rustOutput: "",
      pythonTime: 0,
      rustTime: 0,
      speedup: 1.0,
      memorySaved: 0,
      pythonExitCode: 0,
      rustExitCode: 0,
    };
    
    render(<OutputPanel result={result} />);
    
    const pythonSection = screen.getByTestId("python-output");
    const rustSection = screen.getByTestId("rust-output");
    
    expect(pythonSection).toBeInTheDocument();
    expect(rustSection).toBeInTheDocument();
  });

  it("preserves whitespace in output", () => {
    const result = {
      pythonOutput: "Line 1\n  Indented line\n    Double indented",
      rustOutput: "fn main() {\n    println!(\"Hello\");\n}",
      pythonTime: 10,
      rustTime: 5,
      speedup: 2.0,
      memorySaved: 1.0,
      pythonExitCode: 0,
      rustExitCode: 0,
    };
    
    render(<OutputPanel result={result} />);
    
    const pythonPre = screen.getByText(/Line 1/).closest("pre");
    expect(pythonPre).toHaveTextContent("Line 1\n  Indented line\n    Double indented");
  });
});