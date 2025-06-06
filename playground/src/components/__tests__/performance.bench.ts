import { bench, describe } from 'vitest';
import { render, cleanup } from '@testing-library/react';
import { App } from '../App';
import { EnergyGauge } from '../visualizations/EnergyGauge';
import { ExecutionButton } from '../ExecutionButton';
import { mockWasmModule, createMockPlaygroundStore } from '@test/setup';

// Mock dependencies for benchmarks
vi.mock('@/lib/wasm-manager', () => ({
  WasmModuleManager: vi.fn(() => ({
    loadModule: vi.fn(() => Promise.resolve(mockWasmModule)),
    isLoaded: vi.fn(() => true)
  }))
}));

vi.mock('@/store', () => ({
  usePlaygroundStore: vi.fn(() => createMockPlaygroundStore())
}));

const mockBreakdown = {
  cpu: 0.0008,
  memory: 0.0002,
  network: 0,
  storage: 0
};

describe('Component Performance Benchmarks', () => {
  bench('App component render performance', () => {
    const { unmount } = render(<App />);
    unmount();
  }, {
    iterations: 100,
    time: 1000 // 1 second
  });
  
  bench('EnergyGauge render performance', () => {
    const { unmount } = render(
      <EnergyGauge 
        savings={75} 
        breakdown={mockBreakdown} 
        confidence={0.8} 
      />
    );
    unmount();
  }, {
    iterations: 200,
    time: 1000
  });
  
  bench('ExecutionButton render performance', () => {
    const { unmount } = render(<ExecutionButton />);
    unmount();
  }, {
    iterations: 300,
    time: 1000
  });
  
  bench('EnergyGauge prop updates', () => {
    const { rerender, unmount } = render(
      <EnergyGauge 
        savings={50} 
        breakdown={mockBreakdown} 
        confidence={0.7} 
      />
    );
    
    for (let i = 0; i < 10; i++) {
      rerender(
        <EnergyGauge 
          savings={50 + i * 5} 
          breakdown={mockBreakdown} 
          confidence={0.7 + i * 0.02} 
        />
      );
    }
    
    unmount();
  }, {
    iterations: 50,
    time: 1000
  });
  
  bench('Multiple component renders (stress test)', () => {
    const components = [];
    
    // Render multiple components simultaneously
    for (let i = 0; i < 10; i++) {
      components.push(render(
        <div key={i}>
          <ExecutionButton />
          <EnergyGauge 
            savings={Math.random() * 100} 
            breakdown={mockBreakdown} 
            confidence={Math.random()} 
          />
        </div>
      ));
    }
    
    // Cleanup all components
    components.forEach(({ unmount }) => unmount());
  }, {
    iterations: 20,
    time: 2000
  });
});

describe('Memory Performance Benchmarks', () => {
  bench('Memory usage during render cycles', () => {
    const components = [];
    
    // Create many components
    for (let i = 0; i < 50; i++) {
      components.push(render(
        <EnergyGauge 
          savings={i % 100} 
          breakdown={mockBreakdown} 
          confidence={0.5 + (i % 50) / 100} 
        />
      ));
    }
    
    // Update all components
    components.forEach(({ rerender }, index) => {
      rerender(
        <EnergyGauge 
          savings={(index + 25) % 100} 
          breakdown={mockBreakdown} 
          confidence={0.8} 
        />
      );
    });
    
    // Cleanup
    components.forEach(({ unmount }) => unmount());
    cleanup();
  }, {
    iterations: 10,
    time: 3000
  });
});

describe('State Management Performance', () => {
  bench('Store state updates', () => {
    const store = createMockPlaygroundStore();
    
    // Simulate rapid state updates
    for (let i = 0; i < 100; i++) {
      store.setPythonCode(`def func_${i}(): pass`);
    }
  }, {
    iterations: 100,
    time: 1000
  });
  
  bench('Complex metrics updates', () => {
    const store = createMockPlaygroundStore();
    
    // Simulate complex metrics updates
    for (let i = 0; i < 50; i++) {
      store.metrics = {
        transpile_time_ms: Math.random() * 100,
        energy_reduction: {
          joules: Math.random() * 0.01,
          wattsAverage: Math.random() * 10,
          co2Grams: Math.random() * 0.001,
          breakdown: {
            cpu: Math.random() * 0.005,
            memory: Math.random() * 0.005
          },
          confidence: Math.random(),
          equivalentTo: `operation ${i}`
        }
      };
    }
  }, {
    iterations: 200,
    time: 1000
  });
});