# Generative Repair: MCTS-Guided Code Synthesis

Depyler's generative repair engine uses Monte Carlo Tree Search (MCTS) from the [entrenar](https://crates.io/crates/entrenar) library to explore code generation alternatives when standard transpilation encounters difficulties.

## Overview

The generative repair system provides:

- **MCTS Search**: Explores the space of possible AST transformations
- **Pattern-Guided Synthesis**: Uses HIR patterns to guide code generation
- **Feature-Gated**: Optional dependency via `--features generative`

## Architecture

```
HIR → CodeState → MCTS Search → Best Action → TokenStream
                      ↓
             Reward Function (pattern matching)
```

### Components

1. **CodeState**: Represents partial AST as token sequence
2. **CodeAction**: AST transformation actions (add fn, add let, etc.)
3. **CodeStateSpace**: Defines state transitions and reward calculation
4. **CodeActionSpace**: Provides available Rust token actions

## Usage

### Basic Example

```rust
use depyler_core::generative_repair::{GenerativeRepair, GenerativeRepairConfig};
use depyler_core::hir::HirModule;

// Create repair engine with default config
let repair = GenerativeRepair::new();

// Synthesize from HIR
let hir: HirModule = /* ... */;
let tokens = repair.synthesize(&hir)?;
```

### Custom Configuration

```rust
let config = GenerativeRepairConfig {
    max_iterations: 500,        // MCTS iterations
    exploration_constant: 2.0,  // UCB1 exploration parameter
    max_simulation_depth: 100,  // Max rollout depth
    use_discriminator: false,   // GAN validation (future)
    seed: 42,                   // For reproducibility
};

let repair = GenerativeRepair::with_config(config);
```

## Feature Flag

The generative repair engine requires the `generative` feature:

```bash
# Build with generative feature
cargo build --features generative

# Test with generative feature
cargo test --features generative
```

Without the feature, a stub implementation is provided that returns empty TokenStream.

## MCTS Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `max_iterations` | 100 | Number of MCTS iterations |
| `exploration_constant` | √2 | UCB1 exploration factor |
| `max_simulation_depth` | 50 | Maximum rollout depth |
| `use_discriminator` | false | Enable GAN validation |
| `seed` | 0 | Random seed (0 = non-deterministic) |

## How It Works

### 1. Pattern Extraction

The engine extracts target patterns from HIR:

```rust
fn extract_target_patterns(&self, hir: &HirModule) -> Vec<String> {
    let mut patterns = Vec::new();

    for func in &hir.functions {
        patterns.push(format!("fn {}", func.name));
        // Add parameter names, return type patterns, etc.
    }

    patterns
}
```

### 2. State Space Search

MCTS explores the code generation space:

1. **Selection**: Choose promising nodes via UCB1/PUCT
2. **Expansion**: Add new code tokens
3. **Simulation**: Random rollout to terminal state
4. **Backpropagation**: Update node statistics

### 3. Reward Function

Rewards are based on pattern matching:

```rust
fn evaluate(&self, state: &CodeState) -> Reward {
    let tokens_str = state.tokens.join(" ");
    let matches = self.target_patterns
        .iter()
        .filter(|p| tokens_str.contains(*p))
        .count();

    matches as f64 / self.target_patterns.len() as f64
}
```

## Integration with Hunt Mode

Hunt Mode can use generative repair as a fallback when standard transpilation fails:

```rust
// In hunt mode calibration
if standard_transpile_failed {
    let repair = GenerativeRepair::with_config(config);
    match repair.synthesize(&hir) {
        Ok(tokens) => /* use generated code */,
        Err(_) => /* escalate to LLM */,
    }
}
```

## Future Enhancements

- **GAN Discriminator**: Validate generated code against learned Rust patterns
- **Policy Network**: Neural-guided MCTS with learned priors
- **AST-Level Actions**: More sophisticated transformations
- **Incremental Repair**: Fix specific code regions rather than full regeneration

## Dependencies

The generative feature pulls in:

- `entrenar`: MCTS and GAN implementations
- `trueno`: SIMD-accelerated tensor operations (transitive)

## Performance Considerations

- MCTS is compute-intensive; tune `max_iterations` for your use case
- Use `seed` for reproducible builds
- Feature-gate in production to avoid unused dependencies
