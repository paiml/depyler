import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/react";

// Mock component
const ErrorPanel = ({ errors, warnings }: { errors: string[]; warnings: string[] }) => {
  const hasErrors = errors.length > 0;
  const hasWarnings = warnings.length > 0;

  if (!hasErrors && !hasWarnings) {
    return <div>No errors or warnings.</div>;
  }

  return (
    <div>
      {hasErrors && (
        <div data-testid="errors-section">
          <h3>Errors:</h3>
          {errors.map((error, index) => (
            <div key={index} role="alert">
              <span>✗</span> {error}
            </div>
          ))}
        </div>
      )}
      
      {hasWarnings && (
        <div data-testid="warnings-section">
          <h3>Warnings:</h3>
          {warnings.map((warning, index) => (
            <div key={index}>
              <span>⚠</span> {warning}
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

describe("ErrorPanel", () => {
  it("shows placeholder when no errors or warnings", () => {
    render(<ErrorPanel errors={[]} warnings={[]} />);
    
    expect(screen.getByText("No errors or warnings.")).toBeInTheDocument();
  });

  it("displays errors correctly", () => {
    const errors = [
      "SyntaxError: invalid syntax",
      "NameError: name 'undefined_var' is not defined"
    ];
    
    render(<ErrorPanel errors={errors} warnings={[]} />);
    
    expect(screen.getByText(/SyntaxError: invalid syntax/)).toBeInTheDocument();
    expect(screen.getByText(/NameError: name 'undefined_var' is not defined/)).toBeInTheDocument();
    expect(screen.getAllByRole("alert")).toHaveLength(2);
  });

  it("displays warnings correctly", () => {
    const warnings = [
      "DeprecationWarning: Function is deprecated",
      "RuntimeWarning: Division by zero"
    ];
    
    render(<ErrorPanel errors={[]} warnings={warnings} />);
    
    expect(screen.getByText(/DeprecationWarning/)).toBeInTheDocument();
    expect(screen.getByText(/RuntimeWarning/)).toBeInTheDocument();
  });

  it("displays both errors and warnings", () => {
    const errors = ["Error 1", "Error 2"];
    const warnings = ["Warning 1", "Warning 2"];
    
    render(<ErrorPanel errors={errors} warnings={warnings} />);
    
    expect(screen.getByTestId("errors-section")).toBeInTheDocument();
    expect(screen.getByTestId("warnings-section")).toBeInTheDocument();
    expect(screen.getByText("Error 1")).toBeInTheDocument();
    expect(screen.getByText("Warning 1")).toBeInTheDocument();
  });

  it("shows error icon for errors", () => {
    const errors = ["Test error"];
    
    render(<ErrorPanel errors={errors} warnings={[]} />);
    
    expect(screen.getByText("✗")).toBeInTheDocument();
  });

  it("shows warning icon for warnings", () => {
    const warnings = ["Test warning"];
    
    render(<ErrorPanel errors={[]} warnings={warnings} />);
    
    expect(screen.getByText("⚠")).toBeInTheDocument();
  });

  it("handles long error messages", () => {
    const longError = "A".repeat(200) + " - This is a very long error message";
    
    render(<ErrorPanel errors={[longError]} warnings={[]} />);
    
    expect(screen.getByText(new RegExp(longError))).toBeInTheDocument();
  });

  it("preserves error formatting", () => {
    const formattedError = `File "test.py", line 10
    print(undefined_var)
          ^^^^^^^^^^^^^
NameError: name 'undefined_var' is not defined`;
    
    render(<ErrorPanel errors={[formattedError]} warnings={[]} />);
    
    expect(screen.getByText(new RegExp("line 10"))).toBeInTheDocument();
    expect(screen.getByText(new RegExp("NameError"))).toBeInTheDocument();
  });

  it("handles empty error strings gracefully", () => {
    render(<ErrorPanel errors={["", "Valid error"]} warnings={[]} />);
    
    const alerts = screen.getAllByRole("alert");
    expect(alerts).toHaveLength(2);
    expect(screen.getByText("Valid error")).toBeInTheDocument();
  });

  it("updates when errors change", () => {
    const { rerender } = render(<ErrorPanel errors={["Error 1"]} warnings={[]} />);
    
    expect(screen.getByText("Error 1")).toBeInTheDocument();
    
    rerender(<ErrorPanel errors={["Error 2", "Error 3"]} warnings={[]} />);
    
    expect(screen.queryByText("Error 1")).not.toBeInTheDocument();
    expect(screen.getByText("Error 2")).toBeInTheDocument();
    expect(screen.getByText("Error 3")).toBeInTheDocument();
  });
});