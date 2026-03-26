//! Contract trait enforcement -- compiler verifies provable-contracts trait compliance.
//!
//! Generated via provable-contracts Section 23 trait enforcement.
//!
//! Each `impl` below uses reference scalar implementations. The compile-time
//! check proves the trait signatures are satisfiable. If the contract traits
//! ever change shape, this file fails to compile.
//!
//! Run with: `cargo test --test contract_traits`

use provable_contracts::traits::{ActivationKernelV1, SoftmaxKernelV1};

/// Marker struct for reference scalar kernel implementations.
struct ReferenceKernels;

// ---------------------------------------------------------------------------
// SoftmaxKernelV1 -- numerically stable softmax
// ---------------------------------------------------------------------------
impl SoftmaxKernelV1 for ReferenceKernels {
    fn softmax(&self, x: &[f32]) -> Vec<f32> {
        if x.is_empty() {
            return vec![];
        }
        let max = x.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        let exps: Vec<f32> = x.iter().map(|&xi| (xi - max).exp()).collect();
        let sum: f32 = exps.iter().sum();
        exps.iter().map(|&e| e / sum).collect()
    }
}

// ---------------------------------------------------------------------------
// ActivationKernelV1 -- gelu, relu, silu
// ---------------------------------------------------------------------------
impl ActivationKernelV1 for ReferenceKernels {
    fn gelu(&self, x: f32) -> Vec<f32> {
        let inner = (2.0_f32 / std::f32::consts::PI).sqrt() * (x + 0.044715 * x * x * x);
        vec![0.5 * x * (1.0 + inner.tanh())]
    }

    fn relu(&self, x: f32) -> Vec<f32> {
        vec![x.max(0.0)]
    }

    fn silu(&self, x: f32) -> Vec<f32> {
        vec![x / (1.0 + (-x).exp())]
    }
}

// ---------------------------------------------------------------------------
// Compile-time enforcement test
// ---------------------------------------------------------------------------
#[test]
fn contract_traits_compile() {
    let k = ReferenceKernels;

    // SoftmaxKernelV1: verify normalization invariant
    let out = k.softmax(&[1.0, 2.0, 3.0]);
    let sum: f32 = out.iter().sum();
    assert!((sum - 1.0).abs() < 1e-6, "softmax must sum to 1.0");

    // ActivationKernelV1: verify basic properties
    let gelu_zero = k.gelu(0.0);
    assert!(gelu_zero[0].abs() < 1e-6, "GELU(0) = 0");

    let relu_neg = k.relu(-1.0);
    assert_eq!(relu_neg[0], 0.0, "ReLU(-1) = 0");

    let relu_pos = k.relu(2.0);
    assert_eq!(relu_pos[0], 2.0, "ReLU(2) = 2");

    let silu_zero = k.silu(0.0);
    assert!(silu_zero[0].abs() < 1e-6, "SiLU(0) = 0");
}
