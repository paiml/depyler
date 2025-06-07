import { describe, expect, it, vi } from "vitest";
import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { SettingsDropdown } from "../SettingsDropdown";

describe("SettingsDropdown", () => {
  it("renders settings button", () => {
    render(<SettingsDropdown />);
    
    const button = screen.getByRole("button", { name: /settings/i });
    expect(button).toBeInTheDocument();
  });

  it("opens dropdown when clicked", async () => {
    const user = userEvent.setup();
    render(<SettingsDropdown />);
    
    const button = screen.getByRole("button", { name: /settings/i });
    await user.click(button);
    
    expect(screen.getByText("Transpiler Settings")).toBeInTheDocument();
    expect(button).toHaveAttribute("aria-expanded", "true");
  });

  it("displays all settings options", async () => {
    const user = userEvent.setup();
    render(<SettingsDropdown />);
    
    await user.click(screen.getByRole("button", { name: /settings/i }));
    
    expect(screen.getByText("Optimize for Energy")).toBeInTheDocument();
    expect(screen.getByText("Safety Level")).toBeInTheDocument();
    expect(screen.getByText("String Strategy")).toBeInTheDocument();
    expect(screen.getByText("Generate Documentation")).toBeInTheDocument();
    expect(screen.getByText("Verify Output")).toBeInTheDocument();
  });

  it("toggles boolean settings", async () => {
    const user = userEvent.setup();
    render(<SettingsDropdown />);
    
    await user.click(screen.getByRole("button", { name: /settings/i }));
    
    const optimizeToggle = screen.getByRole("switch", { name: /optimize for energy/i });
    expect(optimizeToggle).toHaveAttribute("aria-checked", "true");
    
    await user.click(optimizeToggle);
    expect(optimizeToggle).toHaveAttribute("aria-checked", "false");
  });

  it("changes select settings", async () => {
    const user = userEvent.setup();
    render(<SettingsDropdown />);
    
    await user.click(screen.getByRole("button", { name: /settings/i }));
    
    const safetySelect = screen.getByLabelText("Safety Level");
    expect(safetySelect).toHaveValue("strict");
    
    await user.selectOptions(safetySelect, "moderate");
    expect(safetySelect).toHaveValue("moderate");
  });

  it("closes when clicking outside", async () => {
    const user = userEvent.setup();
    render(
      <div>
        <SettingsDropdown />
        <button>Outside button</button>
      </div>
    );
    
    await user.click(screen.getByRole("button", { name: /settings/i }));
    expect(screen.getByText("Transpiler Settings")).toBeInTheDocument();
    
    await user.click(screen.getByText("Outside button"));
    
    await waitFor(() => {
      expect(screen.queryByText("Transpiler Settings")).not.toBeInTheDocument();
    });
  });

  it("resets to defaults", async () => {
    const user = userEvent.setup();
    render(<SettingsDropdown />);
    
    await user.click(screen.getByRole("button", { name: /settings/i }));
    
    // Change a setting
    const optimizeToggle = screen.getByRole("switch", { name: /optimize for energy/i });
    await user.click(optimizeToggle);
    expect(optimizeToggle).toHaveAttribute("aria-checked", "false");
    
    // Reset
    await user.click(screen.getByText("Reset to defaults"));
    
    // Should be back to true
    expect(optimizeToggle).toHaveAttribute("aria-checked", "true");
  });

  it("preserves settings after closing and reopening", async () => {
    const user = userEvent.setup();
    render(<SettingsDropdown />);
    
    // Open and change settings
    await user.click(screen.getByRole("button", { name: /settings/i }));
    const safetySelect = screen.getByLabelText("Safety Level");
    await user.selectOptions(safetySelect, "moderate");
    
    // Close by clicking button again
    await user.click(screen.getByRole("button", { name: /settings/i }));
    
    // Reopen
    await user.click(screen.getByRole("button", { name: /settings/i }));
    
    // Setting should be preserved
    expect(screen.getByLabelText("Safety Level")).toHaveValue("moderate");
  });

  it("handles keyboard navigation", async () => {
    const user = userEvent.setup();
    render(<SettingsDropdown />);
    
    const button = screen.getByRole("button", { name: /settings/i });
    button.focus();
    
    // Open with Space key
    await user.keyboard(" ");
    expect(screen.getByText("Transpiler Settings")).toBeInTheDocument();
    
    // Navigate with Tab
    await user.keyboard("{Tab}");
    expect(document.activeElement).toHaveAttribute("role", "switch");
  });

  it("updates all string strategy options", async () => {
    const user = userEvent.setup();
    render(<SettingsDropdown />);
    
    await user.click(screen.getByRole("button", { name: /settings/i }));
    
    const strategySelect = screen.getByLabelText("String Strategy");
    
    // Test all options
    await user.selectOptions(strategySelect, "zero_copy");
    expect(strategySelect).toHaveValue("zero_copy");
    
    await user.selectOptions(strategySelect, "cow");
    expect(strategySelect).toHaveValue("cow");
    
    await user.selectOptions(strategySelect, "owned");
    expect(strategySelect).toHaveValue("owned");
  });

  it("shows proper icons for settings", async () => {
    const user = userEvent.setup();
    render(<SettingsDropdown />);
    
    await user.click(screen.getByRole("button", { name: /settings/i }));
    
    // Check that SVG icon is present in button
    const button = screen.getByRole("button", { name: /settings/i });
    expect(button.querySelector("svg")).toBeInTheDocument();
  });

  it("has correct ARIA attributes when closed", () => {
    render(<SettingsDropdown />);
    
    const button = screen.getByRole("button", { name: /settings/i });
    expect(button).toHaveAttribute("aria-haspopup", "true");
    expect(button).toHaveAttribute("aria-expanded", "false");
  });

  it("updates multiple settings in sequence", async () => {
    const user = userEvent.setup();
    render(<SettingsDropdown />);
    
    await user.click(screen.getByRole("button", { name: /settings/i }));
    
    // Toggle multiple boolean settings
    const switches = screen.getAllByRole("switch");
    const generateDocsToggle = switches[1]; // Generate Documentation is second toggle
    const verifyOutputToggle = switches[2]; // Verify Output is third toggle
    
    await user.click(generateDocsToggle);
    await user.click(verifyOutputToggle);
    
    expect(generateDocsToggle).toHaveAttribute("aria-checked", "true");
    expect(verifyOutputToggle).toHaveAttribute("aria-checked", "false");
  });

  it("maintains focus management", async () => {
    const user = userEvent.setup();
    render(<SettingsDropdown />);
    
    const button = screen.getByRole("button", { name: /settings/i });
    await user.click(button);
    
    // The button should still have focus after opening
    expect(document.activeElement).toBe(button);
  });
});