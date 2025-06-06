import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { ExecutionButton } from '../ExecutionButton';
import { createMockPlaygroundStore } from '@test/setup';

// Mock the store
const mockStore = createMockPlaygroundStore();
vi.mock('@/store', () => ({
  usePlaygroundStore: vi.fn(() => mockStore)
}));

// Mock download function
const mockDownloadToolchain = vi.fn();
vi.mock('@/lib/toolchain-downloader', () => ({
  downloadToolchain: mockDownloadToolchain
}));

describe('ExecutionButton', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockStore.isExecuting = false;
    mockStore.isToolchainCached = true;
  });

  it('renders with correct default text', () => {
    render(<ExecutionButton />);
    
    expect(screen.getByRole('button', { name: /run comparison/i })).toBeInTheDocument();
  });

  it('calls executeCode when clicked', async () => {
    const user = userEvent.setup();
    render(<ExecutionButton />);
    
    const button = screen.getByRole('button', { name: /run comparison/i });
    await user.click(button);
    
    expect(mockStore.executeCode).toHaveBeenCalledOnce();
  });

  it('shows loading state during execution', () => {
    mockStore.isExecuting = true;
    render(<ExecutionButton />);
    
    const button = screen.getByRole('button');
    expect(button).toBeDisabled();
    expect(button).toHaveAttribute('aria-busy', 'true');
    expect(screen.getByText(/executing/i)).toBeInTheDocument();
  });

  it('shows toolchain download progress when not cached', async () => {
    mockStore.isToolchainCached = false;
    mockDownloadToolchain.mockImplementation(({ onProgress }) => {
      // Simulate download progress
      setTimeout(() => onProgress(0.5), 10);
      setTimeout(() => onProgress(1.0), 20);
      return Promise.resolve();
    });
    
    const user = userEvent.setup();
    render(<ExecutionButton />);
    
    const button = screen.getByRole('button');
    await user.click(button);
    
    await waitFor(() => {
      expect(screen.getByText(/downloading rust toolchain/i)).toBeInTheDocument();
    });
  });

  it('handles download errors gracefully', async () => {
    mockStore.isToolchainCached = false;
    mockDownloadToolchain.mockRejectedValue(new Error('Download failed'));
    
    const user = userEvent.setup();
    render(<ExecutionButton />);
    
    const button = screen.getByRole('button');
    await user.click(button);
    
    await waitFor(() => {
      expect(screen.getByText(/error downloading/i)).toBeInTheDocument();
    });
  });

  it('shows progress indicator with correct value', async () => {
    mockStore.isToolchainCached = false;
    let progressCallback: (progress: number) => void;
    
    mockDownloadToolchain.mockImplementation(({ onProgress }) => {
      progressCallback = onProgress;
      return new Promise(() => {}); // Never resolves for this test
    });
    
    const user = userEvent.setup();
    render(<ExecutionButton />);
    
    await user.click(screen.getByRole('button'));
    
    await waitFor(() => {
      expect(screen.getByText(/downloading/i)).toBeInTheDocument();
    });
    
    // Simulate progress update
    progressCallback!(0.75);
    
    await waitFor(() => {
      const progressElement = screen.getByRole('progressbar');
      expect(progressElement).toHaveAttribute('aria-valuenow', '75');
    });
  });

  it('has proper accessibility attributes', () => {
    render(<ExecutionButton />);
    
    const button = screen.getByRole('button');
    expect(button).toHaveAttribute('type', 'button');
    expect(button).toHaveAccessibleName();
  });

  it('prevents multiple simultaneous executions', async () => {
    const user = userEvent.setup();
    render(<ExecutionButton />);
    
    const button = screen.getByRole('button');
    
    // Click multiple times quickly
    await user.click(button);
    await user.click(button);
    await user.click(button);
    
    // Should only call executeCode once
    expect(mockStore.executeCode).toHaveBeenCalledOnce();
  });
});

describe('ExecutionButton States', () => {
  it('renders correctly in idle state', () => {
    mockStore.isExecuting = false;
    mockStore.isTranspiling = false;
    
    render(<ExecutionButton />);
    
    expect(screen.getByText('Run Comparison')).toBeInTheDocument();
    
    const button = screen.getByRole('button');
    expect(button).toBeEnabled();
  });

  it('renders correctly when transpiling', () => {
    mockStore.isTranspiling = true;
    
    render(<ExecutionButton />);
    
    expect(screen.getByText('Transpiling...')).toBeInTheDocument();
    
    const button = screen.getByRole('button');
    expect(button).toBeDisabled();
  });

  it('renders correctly when executing', () => {
    mockStore.isExecuting = true;
    
    render(<ExecutionButton />);
    
    // When isExecuting is true, the button shows "Run Comparison" but is disabled
    expect(screen.getByText('Run Comparison')).toBeInTheDocument();
    
    const button = screen.getByRole('button');
    expect(button).toBeDisabled();
  });
});