import { beforeEach, describe, expect, it, vi } from "vitest";
import { act, fireEvent, render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { ExecutionButton } from "../ExecutionButton";
import { createMockPlaygroundStore } from "@test/setup";

// Mock the store
const mockStore = createMockPlaygroundStore();
vi.mock("@/store", () => ({
  usePlaygroundStore: vi.fn(() => mockStore),
}));

describe("ExecutionButton", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockStore.isExecuting = false;
    mockStore.isTranspiling = false;
    mockStore.pythonCode = "def test(): pass";
    mockStore.rustCode = "";
    // Reset mock implementations
    mockStore.executeCode.mockResolvedValue(undefined);
    mockStore.transpileCode.mockResolvedValue(undefined);
  });

  it("renders with correct default text", () => {
    render(<ExecutionButton />);

    expect(screen.getByRole("button", { name: /run comparison/i })).toBeInTheDocument();
  });

  it("calls executeCode when clicked", async () => {
    mockStore.rustCode = "fn test() {}";
    const user = userEvent.setup();
    render(<ExecutionButton />);

    const button = screen.getByRole("button", { name: /run comparison/i });
    
    await act(async () => {
      await user.click(button);
    });

    await waitFor(() => {
      expect(mockStore.executeCode).toHaveBeenCalledOnce();
    });
  });

  it("shows loading state during transpilation", () => {
    mockStore.isTranspiling = true;
    render(<ExecutionButton />);

    const button = screen.getByRole("button");
    expect(button).toBeDisabled();
    expect(button).toHaveAttribute("aria-busy", "true");
    expect(screen.getByText(/transpiling/i)).toBeInTheDocument();
  });

  it("is disabled when no Python code", () => {
    mockStore.pythonCode = "";
    render(<ExecutionButton />);

    const button = screen.getByRole("button");
    expect(button).toBeDisabled();
  });

  it("transpiles first if no Rust code exists", async () => {
    mockStore.pythonCode = "def hello(): pass";
    mockStore.rustCode = "";
    
    const user = userEvent.setup();
    render(<ExecutionButton />);

    const button = screen.getByRole("button");
    
    await act(async () => {
      await user.click(button);
    });

    await waitFor(() => {
      expect(mockStore.transpileCode).toHaveBeenCalled();
    });
  });

  it("calls executeCode after transpilation", async () => {
    mockStore.pythonCode = "def hello(): pass";
    mockStore.rustCode = "fn hello() {}";
    
    const user = userEvent.setup();
    render(<ExecutionButton />);

    const button = screen.getByRole("button");
    
    await act(async () => {
      await user.click(button);
    });

    await waitFor(() => {
      expect(mockStore.executeCode).toHaveBeenCalled();
    });
  });

  it("has proper accessibility attributes", () => {
    render(<ExecutionButton />);

    const button = screen.getByRole("button");
    expect(button).toHaveAttribute("type", "button");
    expect(button).toHaveAccessibleName();
  });

  it("prevents multiple simultaneous executions", async () => {
    mockStore.rustCode = "fn test() {}";
    mockStore.executeCode.mockImplementation(() => {
      // Simulate execution state
      mockStore.isExecuting = true;
      return Promise.resolve();
    });
    
    const user = userEvent.setup();
    const { rerender } = render(<ExecutionButton />);

    const button = screen.getByRole("button");

    // First click starts execution
    await user.click(button);
    expect(mockStore.executeCode).toHaveBeenCalledTimes(1);
    
    // Update component to reflect executing state
    mockStore.isExecuting = true;
    rerender(<ExecutionButton />);
    
    // Button should now be disabled
    const disabledButton = screen.getByRole("button");
    expect(disabledButton).toBeDisabled();
  });
});

describe("ExecutionButton States", () => {
  it("renders correctly in idle state", () => {
    mockStore.isExecuting = false;
    mockStore.isTranspiling = false;

    render(<ExecutionButton />);

    expect(screen.getByText("Run Comparison")).toBeInTheDocument();

    const button = screen.getByRole("button");
    expect(button).toBeEnabled();
  });

  it("renders correctly when transpiling", () => {
    mockStore.isTranspiling = true;

    render(<ExecutionButton />);

    expect(screen.getByText("Transpiling...")).toBeInTheDocument();

    const button = screen.getByRole("button");
    expect(button).toBeDisabled();
  });

  it("renders correctly when executing", () => {
    mockStore.isExecuting = true;

    render(<ExecutionButton />);

    // When isExecuting is true, the button is disabled but shows default text
    const button = screen.getByRole("button");
    expect(button).toBeDisabled();
    expect(button).toHaveAttribute("aria-busy", "true");
  });
});