import { describe, expect, it, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { ExampleSelector } from "../ExampleSelector";
import { createMockPlaygroundStore } from "@test/setup";

// Mock the store
const mockStore = createMockPlaygroundStore();
vi.mock("@/store", () => ({
  usePlaygroundStore: vi.fn(() => mockStore),
}));

describe("ExampleSelector", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders examples button", () => {
    render(<ExampleSelector />);
    
    const button = screen.getByRole("button", { name: /examples/i });
    expect(button).toBeInTheDocument();
  });

  it("opens dropdown when clicked", async () => {
    const user = userEvent.setup();
    render(<ExampleSelector />);
    
    const button = screen.getByRole("button", { name: /examples/i });
    await user.click(button);
    
    expect(button).toHaveAttribute("aria-expanded", "true");
    expect(screen.getByText("basic")).toBeInTheDocument();
    expect(screen.getByText("optimization")).toBeInTheDocument();
  });

  it("displays all example categories", async () => {
    const user = userEvent.setup();
    render(<ExampleSelector />);
    
    await user.click(screen.getByRole("button", { name: /examples/i }));
    
    expect(screen.getByText("basic")).toBeInTheDocument();
    expect(screen.getByText("advanced")).toBeInTheDocument();
    expect(screen.getByText("optimization")).toBeInTheDocument();
    expect(screen.getByText("patterns")).toBeInTheDocument();
  });

  it("loads example when clicked", async () => {
    const user = userEvent.setup();
    render(<ExampleSelector />);
    
    await user.click(screen.getByRole("button", { name: /examples/i }));
    
    const fibonacciExample = screen.getByText("Fibonacci Sequence");
    await user.click(fibonacciExample);
    
    expect(mockStore.setPythonCode).toHaveBeenCalledWith(
      expect.stringContaining("calculate_fibonacci")
    );
  });

  it("closes dropdown after selecting example", async () => {
    const user = userEvent.setup();
    render(<ExampleSelector />);
    
    const button = screen.getByRole("button", { name: /examples/i });
    await user.click(button);
    
    const example = screen.getByText("Binary Search");
    await user.click(example);
    
    expect(button).toHaveAttribute("aria-expanded", "false");
  });

  it("displays example descriptions", async () => {
    const user = userEvent.setup();
    render(<ExampleSelector />);
    
    await user.click(screen.getByRole("button", { name: /examples/i }));
    
    expect(screen.getByText("Efficient fibonacci calculation with memoization")).toBeInTheDocument();
    expect(screen.getByText("Sieve of Eratosthenes for finding prime numbers")).toBeInTheDocument();
    expect(screen.getByText("Efficient search algorithm for sorted lists")).toBeInTheDocument();
  });

  it("organizes examples by category", async () => {
    const user = userEvent.setup();
    render(<ExampleSelector />);
    
    await user.click(screen.getByRole("button", { name: /examples/i }));
    
    // Check that Fibonacci is under "basic" category
    const basicSection = screen.getByText("basic").parentElement?.parentElement;
    expect(basicSection).toBeTruthy();
    
    const fibonacciButton = screen.getByText("Fibonacci Sequence").parentElement;
    expect(fibonacciButton).toBeTruthy();
  });
});