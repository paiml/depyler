//! Contract-trait enforcement — compiler verifies bound functions exist.
//! Section 23 of provable-contracts spec.

use provable_contracts::traits::*;

struct DepylerKernels;

impl SoftmaxKernelV1 for DepylerKernels {
    fn softmax(&self, x: &[f32]) -> Vec<f32> {
        let max = x.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        let exps: Vec<f32> = x.iter().map(|v| (v - max).exp()).collect();
        let sum: f32 = exps.iter().sum();
        exps.iter().map(|e| e / sum).collect()
    }
}

impl ActivationKernelV1 for DepylerKernels {
    fn gelu(&self, x: f32) -> Vec<f32> {
        vec![0.5 * x * (1.0 + (0.7978845608 * (x + 0.044715 * x.powi(3))).tanh())]
    }
    fn relu(&self, x: f32) -> Vec<f32> { vec![x.max(0.0)] }
    fn silu(&self, x: f32) -> Vec<f32> { vec![x / (1.0 + (-x).exp())] }
}

impl SiluKernelV1 for DepylerKernels {
    fn sigmoid(&self, x: &[f32]) -> Vec<f32> { x.iter().map(|v| 1.0 / (1.0 + (-v).exp())).collect() }
    fn silu(&self, x: &[f32]) -> Vec<f32> { x.iter().map(|v| v / (1.0 + (-v).exp())).collect() }
}

impl RmsnormKernelV1 for DepylerKernels {
    fn rmsnorm(&self, x: &[f32]) -> Vec<f32> {
        let rms = (x.iter().map(|v| v * v).sum::<f32>() / x.len() as f32).sqrt();
        x.iter().map(|v| v / (rms + 1e-5)).collect()
    }
}

impl LayernormKernelV1 for DepylerKernels {
    fn layernorm(&self, x: &[f32], gamma: &[f32]) -> Vec<f32> {
        let mean = x.iter().sum::<f32>() / x.len() as f32;
        let var = x.iter().map(|v| (v - mean).powi(2)).sum::<f32>() / x.len() as f32;
        let std = (var + 1e-5).sqrt();
        x.iter().enumerate().map(|(i, v)| ((v - mean) / std) * gamma.get(i).copied().unwrap_or(1.0)).collect()
    }
    fn statistics(&self, x: &[f32]) -> Vec<f32> {
        let mean = x.iter().sum::<f32>() / x.len() as f32;
        let var = x.iter().map(|v| (v - mean).powi(2)).sum::<f32>() / x.len() as f32;
        vec![mean, var]
    }
}

impl CrossEntropyKernelV1 for DepylerKernels {
    fn cross_entropy(&self, targets: &[f32], logits: &[f32]) -> Vec<f32> {
        let log_sm = CrossEntropyKernelV1::log_softmax(self, logits);
        let loss = -targets.iter().zip(log_sm.iter()).map(|(t, l)| t * l).sum::<f32>();
        vec![loss]
    }
    fn log_softmax(&self, x: &[f32]) -> Vec<f32> {
        let max = x.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        let lse = x.iter().map(|v| (v - max).exp()).sum::<f32>().ln();
        x.iter().map(|v| v - max - lse).collect()
    }
}

impl SwigluKernelV1 for DepylerKernels {
    fn silu(&self, x: &[f32]) -> Vec<f32> { x.iter().map(|v| v / (1.0 + (-v).exp())).collect() }
    fn swiglu(&self, x: &[f32], w: &[f32], v: &[f32], _b: &[f32], _c: &[f32]) -> Vec<f32> {
        let gate: Vec<f32> = x.iter().zip(w.iter()).map(|(xi, wi)| xi * wi).collect();
        let silu: Vec<f32> = gate.iter().map(|g| g / (1.0 + (-g).exp())).collect();
        let val: Vec<f32> = x.iter().zip(v.iter()).map(|(xi, vi)| xi * vi).collect();
        silu.iter().zip(val.iter()).map(|(s, v)| s * v).collect()
    }
}

#[test]
fn softmax_sums_to_one() {
    let k = DepylerKernels;
    let out = SoftmaxKernelV1::softmax(&k, &[1.0, 2.0, 3.0]);
    assert!((out.iter().sum::<f32>() - 1.0).abs() < 1e-6);
}

#[test]
fn activation_properties() {
    let k = DepylerKernels;
    assert!(ActivationKernelV1::gelu(&k, 0.0)[0].abs() < 1e-6);
    assert!(ActivationKernelV1::relu(&k, -1.0)[0] == 0.0);
    assert!(ActivationKernelV1::silu(&k, 0.0)[0].abs() < 1e-6);
}

#[test]
fn sigmoid_range() {
    let k = DepylerKernels;
    let out = SiluKernelV1::sigmoid(&k, &[-10.0, 0.0, 10.0]);
    assert!(out.iter().all(|v| *v > 0.0 && *v < 1.0));
}

#[test]
fn rmsnorm_unit_rms() {
    let k = DepylerKernels;
    let out = RmsnormKernelV1::rmsnorm(&k, &[3.0, 4.0]);
    let rms = (out.iter().map(|v| v * v).sum::<f32>() / out.len() as f32).sqrt();
    assert!((rms - 1.0).abs() < 0.01);
}

#[test]
fn layernorm_zero_mean() {
    let k = DepylerKernels;
    let out = LayernormKernelV1::layernorm(&k, &[1.0, 2.0, 3.0, 4.0], &[1.0, 1.0, 1.0, 1.0]);
    let mean: f32 = out.iter().sum::<f32>() / out.len() as f32;
    assert!(mean.abs() < 1e-5);
}

#[test]
fn cross_entropy_positive() {
    let k = DepylerKernels;
    let loss = CrossEntropyKernelV1::cross_entropy(&k, &[1.0, 0.0, 0.0], &[2.0, 1.0, 0.1]);
    assert!(loss[0] > 0.0);
}
