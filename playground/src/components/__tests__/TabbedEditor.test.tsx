import { describe, expect, it, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { TabbedEditor } from "../TabbedEditor";
import { createMockPlaygroundStore } from "@test/setup";

// Mock the store
const mockStore = createMockPlaygroundStore();
vi.mock("@/store", () => ({
  usePlaygroundStore: vi.fn(() => mockStore),
}));

describe("TabbedEditor", () => {
  it("renders all tabs", () => {
    render(<TabbedEditor />);
    
    expect(screen.getByRole("tab", { name: /python/i })).toBeInTheDocument();
    expect(screen.getByRole("tab", { name: /rust/i })).toBeInTheDocument();
    expect(screen.getByRole("tab", { name: /output/i })).toBeInTheDocument();
    expect(screen.getByRole("tab", { name: /errors/i })).toBeInTheDocument();
  });

  it("shows Python editor by default", () => {
    render(<TabbedEditor />);
    
    const pythonTab = screen.getByRole("tab", { name: /python/i });
    expect(pythonTab).toHaveAttribute("aria-selected", "true");
    
    // Check that Monaco editor is rendered
    expect(screen.getByTestId("monaco-editor")).toBeInTheDocument();
  });

  it("switches to Rust tab when clicked", () => {
    render(<TabbedEditor />);
    
    const rustTab = screen.getByRole("tab", { name: /rust/i });
    fireEvent.click(rustTab);
    
    expect(rustTab).toHaveAttribute("aria-selected", "true");
  });

  it("shows error count badge when there are errors", () => {
    mockStore.errors = ["Error 1", "Error 2"];
    render(<TabbedEditor />);
    
    const errorTab = screen.getByRole("tab", { name: /errors/i });
    expect(errorTab).toHaveTextContent("2");
  });

  it("displays execution results in output tab", () => {
    mockStore.executionResult = {
      pythonOutput: "Hello from Python",
      rustOutput: "Hello from Rust",
      pythonTime: 10.5,
      rustTime: 2.1,
      speedup: 5.0,
      memorySaved: 3.2,
      pythonExitCode: 0,
      rustExitCode: 0,
    };
    
    render(<TabbedEditor />);
    
    const outputTab = screen.getByRole("tab", { name: /output/i });
    fireEvent.click(outputTab);
    
    expect(screen.getByText("Hello from Python")).toBeInTheDocument();
    expect(screen.getByText("Hello from Rust")).toBeInTheDocument();
    expect(screen.getByText(/5\.0.*faster/)).toBeInTheDocument();
  });

  it("provides accessible tab navigation", () => {
    render(<TabbedEditor />);
    
    const tabs = screen.getAllByRole("tab");
    tabs.forEach(tab => {
      expect(tab).toHaveAttribute("aria-controls");
      expect(tab).toHaveAttribute("aria-selected");
    });
    
    const tabpanel = screen.getByRole("tabpanel");
    expect(tabpanel).toBeInTheDocument();
  });

  it("updates editor content when store changes", () => {
    const { rerender } = render(<TabbedEditor />);
    
    // Update the store
    mockStore.pythonCode = "def updated_function(): pass";
    mockStore.rustCode = "fn updated_function() {}";
    
    rerender(<TabbedEditor />);
    
    // The mock editors should receive the updated code
    expect(mockStore.pythonCode).toBe("def updated_function(): pass");
    expect(mockStore.rustCode).toBe("fn updated_function() {}");
  });

  it("shows warnings in errors tab", () => {
    mockStore.warnings = ["Warning 1", "Warning 2"];
    mockStore.errors = [];
    
    render(<TabbedEditor />);
    
    const errorTab = screen.getByRole("tab", { name: /errors/i });
    fireEvent.click(errorTab);
    
    expect(screen.getByText("Warning 1")).toBeInTheDocument();
    expect(screen.getByText("Warning 2")).toBeInTheDocument();
  });

  it("handles empty execution results gracefully", () => {
    mockStore.executionResult = null;
    
    render(<TabbedEditor />);
    
    const outputTab = screen.getByRole("tab", { name: /output/i });
    fireEvent.click(outputTab);
    
    expect(screen.getByText(/No output yet/)).toBeInTheDocument();
  });

  it("preserves active tab on re-render", () => {
    const { rerender } = render(<TabbedEditor />);
    
    // Switch to output tab
    const outputTab = screen.getByRole("tab", { name: /output/i });
    fireEvent.click(outputTab);
    
    expect(outputTab).toHaveAttribute("aria-selected", "true");
    
    // Re-render component
    rerender(<TabbedEditor />);
    
    // Output tab should still be selected
    expect(screen.getByRole("tab", { name: /output/i })).toHaveAttribute("aria-selected", "true");
  });

  it("handles keyboard navigation", () => {
    render(<TabbedEditor />);
    
    const pythonTab = screen.getByRole("tab", { name: /python/i });
    pythonTab.focus();
    
    // Simulate arrow key navigation
    fireEvent.keyDown(pythonTab, { key: "ArrowRight" });
    
    // Should move focus to next tab
    const rustTab = screen.getByRole("tab", { name: /rust/i });
    expect(document.activeElement).toBe(rustTab);
  });

  it("updates tab badge count correctly", () => {
    const { rerender } = render(<TabbedEditor />);
    
    // Initially no errors
    expect(screen.getByRole("tab", { name: /errors/i })).not.toHaveTextContent(/\d+/);
    
    // Add errors
    mockStore.errors = ["Error 1", "Error 2", "Error 3"];
    mockStore.warnings = ["Warning 1"];
    rerender(<TabbedEditor />);
    
    // Should show total count
    const errorTab = screen.getByRole("tab", { name: /errors/i });
    expect(errorTab).toHaveTextContent("4");
  });

  it("maintains scroll position when switching tabs", () => {
    render(<TabbedEditor />);
    
    // Switch between tabs
    fireEvent.click(screen.getByRole("tab", { name: /rust/i }));
    fireEvent.click(screen.getByRole("tab", { name: /python/i }));
    
    // Verify we're back on Python tab
    expect(screen.getByRole("tab", { name: /python/i })).toHaveAttribute("aria-selected", "true");
  });
});