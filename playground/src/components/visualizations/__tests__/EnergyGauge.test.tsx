import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/react';
import { EnergyGauge } from '../EnergyGauge';

// Mock D3 more thoroughly for this component
const mockD3 = {
  select: vi.fn(() => ({
    selectAll: vi.fn(() => ({
      data: vi.fn(() => ({
        join: vi.fn(() => ({
          attr: vi.fn().mockReturnThis(),
          style: vi.fn().mockReturnThis(),
          text: vi.fn().mockReturnThis(),
          datum: vi.fn().mockReturnThis(),
          transition: vi.fn(() => ({
            duration: vi.fn().mockReturnThis(),
            ease: vi.fn().mockReturnThis(),
            attrTween: vi.fn().mockReturnThis(),
            tween: vi.fn().mockReturnThis()
          }))
        }))
      }))
    })),
    attr: vi.fn().mockReturnThis(),
    style: vi.fn().mockReturnThis()
  })),
  scaleLinear: vi.fn(() => ({
    domain: vi.fn().mockReturnThis(),
    range: vi.fn().mockReturnThis()
  })),
  scaleSequential: vi.fn(() => ({
    domain: vi.fn().mockReturnThis(),
    interpolator: vi.fn().mockReturnThis()
  })),
  interpolateRdYlGn: vi.fn(t => `hsl(${t * 120}, 80%, 50%)`),
  arc: vi.fn(() => vi.fn(() => 'M0,0')),
  easeCubicInOut: vi.fn(),
  interpolate: vi.fn(() => vi.fn(t => ({ endAngle: t * Math.PI })))
};

vi.mock('d3', () => mockD3);

// Mock ResizeObserver
const mockResizeObserver = vi.fn(() => ({
  observe: vi.fn(),
  unobserve: vi.fn(),
  disconnect: vi.fn()
}));
global.ResizeObserver = mockResizeObserver;

const mockBreakdown = {
  cpu: 0.0008,
  memory: 0.0002,
  network: 0,
  storage: 0
};

describe('EnergyGauge', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders without crashing', () => {
    render(
      <EnergyGauge 
        savings={75} 
        breakdown={mockBreakdown} 
        confidence={0.8} 
      />
    );
    
    expect(screen.getByTestId('energy-gauge')).toBeInTheDocument();
  });

  it('displays the SVG element', () => {
    render(
      <EnergyGauge 
        savings={50} 
        breakdown={mockBreakdown} 
        confidence={0.9} 
      />
    );
    
    const svg = screen.getByRole('img', { hidden: true });
    expect(svg).toBeInTheDocument();
    expect(svg.tagName).toBe('svg');
  });

  it('calls D3 functions for visualization', () => {
    render(
      <EnergyGauge 
        savings={85} 
        breakdown={mockBreakdown} 
        confidence={0.7} 
      />
    );
    
    expect(mockD3.select).toHaveBeenCalled();
    expect(mockD3.scaleLinear).toHaveBeenCalled();
    expect(mockD3.scaleSequential).toHaveBeenCalled();
  });

  it('updates when savings prop changes', () => {
    const { rerender } = render(
      <EnergyGauge 
        savings={25} 
        breakdown={mockBreakdown} 
        confidence={0.6} 
      />
    );
    
    rerender(
      <EnergyGauge 
        savings={90} 
        breakdown={mockBreakdown} 
        confidence={0.9} 
      />
    );
    
    // D3 select should be called multiple times for updates
    expect(mockD3.select).toHaveBeenCalledTimes(2);
  });

  it('handles zero savings correctly', () => {
    render(
      <EnergyGauge 
        savings={0} 
        breakdown={mockBreakdown} 
        confidence={0.5} 
      />
    );
    
    expect(screen.getByTestId('energy-gauge')).toBeInTheDocument();
  });

  it('handles maximum savings correctly', () => {
    render(
      <EnergyGauge 
        savings={100} 
        breakdown={mockBreakdown} 
        confidence={1.0} 
      />
    );
    
    expect(screen.getByTestId('energy-gauge')).toBeInTheDocument();
  });

  it('displays breakdown details', () => {
    render(
      <EnergyGauge 
        savings={60} 
        breakdown={mockBreakdown} 
        confidence={0.8} 
      />
    );
    
    expect(screen.getByTestId('energy-breakdown')).toBeInTheDocument();
  });

  it('has proper accessibility attributes', () => {
    render(
      <EnergyGauge 
        savings={75} 
        breakdown={mockBreakdown} 
        confidence={0.8} 
      />
    );
    
    const gauge = screen.getByTestId('energy-gauge');
    expect(gauge).toHaveAttribute('role', 'img');
    expect(gauge).toHaveAttribute('aria-label');
  });

  it('sets up ResizeObserver for responsive behavior', () => {
    render(
      <EnergyGauge 
        savings={50} 
        breakdown={mockBreakdown} 
        confidence={0.7} 
      />
    );
    
    expect(mockResizeObserver).toHaveBeenCalled();
  });
});

describe('EnergyGauge Performance', () => {
  it('renders within performance budget', () => {
    const startTime = performance.now();
    
    render(
      <EnergyGauge 
        savings={75} 
        breakdown={mockBreakdown} 
        confidence={0.8} 
      />
    );
    
    const renderTime = performance.now() - startTime;
    
    // Should render within 16ms for 60fps
    expect(renderTime).toBeLessThan(16);
  });

  it('handles rapid prop changes efficiently', () => {
    const { rerender } = render(
      <EnergyGauge 
        savings={10} 
        breakdown={mockBreakdown} 
        confidence={0.5} 
      />
    );
    
    const startTime = performance.now();
    
    // Simulate rapid updates
    for (let i = 20; i <= 90; i += 10) {
      rerender(
        <EnergyGauge 
          savings={i} 
          breakdown={mockBreakdown} 
          confidence={i / 100} 
        />
      );
    }
    
    const updateTime = performance.now() - startTime;
    
    // Should handle 8 updates within 100ms
    expect(updateTime).toBeLessThan(100);
  });

  it('memoizes scales correctly', () => {
    const { rerender } = render(
      <EnergyGauge 
        savings={50} 
        breakdown={mockBreakdown} 
        confidence={0.7} 
      />
    );
    
    const initialScaleCalls = mockD3.scaleLinear.mock.calls.length;
    
    // Rerender with different savings but same other props
    rerender(
      <EnergyGauge 
        savings={60} 
        breakdown={mockBreakdown} 
        confidence={0.7} 
      />
    );
    
    // Scales should be memoized and not recreated
    expect(mockD3.scaleLinear.mock.calls.length).toBe(initialScaleCalls);
  });
});