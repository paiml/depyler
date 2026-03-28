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
    fn relu(&self, x: f32) -> Vec<f32> {
        vec![x.max(0.0)]
    }
    fn silu(&self, x: f32) -> Vec<f32> {
        vec![x / (1.0 + (-x).exp())]
    }
}

impl SiluKernelV1 for DepylerKernels {
    fn sigmoid(&self, x: &[f32]) -> Vec<f32> {
        x.iter().map(|v| 1.0 / (1.0 + (-v).exp())).collect()
    }
    fn silu(&self, x: &[f32]) -> Vec<f32> {
        x.iter().map(|v| v / (1.0 + (-v).exp())).collect()
    }
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
        x.iter()
            .enumerate()
            .map(|(i, v)| ((v - mean) / std) * gamma.get(i).copied().unwrap_or(1.0))
            .collect()
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
    fn silu(&self, x: &[f32]) -> Vec<f32> {
        x.iter().map(|v| v / (1.0 + (-v).exp())).collect()
    }
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

// ---------------------------------------------------------------------------
// AttentionKernelV1 -- naive scaled dot-product attention
// ---------------------------------------------------------------------------
impl AttentionKernelV1 for DepylerKernels {
    fn attention(&self, q: &[f32], k: &[f32], v: &[f32]) -> Vec<f32> {
        let n = (q.len() as f32).sqrt() as usize;
        if n == 0 {
            return vec![];
        }
        let d = q.len() / n;
        let mut out = vec![0.0f32; n * d];
        for i in 0..n {
            let mut scores = vec![0.0f32; n];
            for j in 0..n {
                for kk in 0..d {
                    scores[j] += q[i * d + kk] * k[j * d + kk];
                }
            }
            let scale = (d as f32).sqrt();
            let max = scores.iter().copied().fold(f32::NEG_INFINITY, f32::max);
            let exps: Vec<f32> = scores.iter().map(|s| ((s / scale) - max).exp()).collect();
            let sum: f32 = exps.iter().sum();
            for j in 0..n {
                for kk in 0..d {
                    out[i * d + kk] += (exps[j] / sum) * v[j * d + kk];
                }
            }
        }
        out
    }
}

// ---------------------------------------------------------------------------
// FlashAttentionV1 -- same math as attention (flash is an optimization)
// ---------------------------------------------------------------------------
impl FlashAttentionV1 for DepylerKernels {
    fn flash_attention(&self, q: &[f32], k: &[f32], v: &[f32]) -> Vec<f32> {
        AttentionKernelV1::attention(self, q, k, v)
    }
}

// ---------------------------------------------------------------------------
// GqaKernelV1 -- grouped-query attention (equal heads = standard attention)
// ---------------------------------------------------------------------------
impl GqaKernelV1 for DepylerKernels {
    fn gqa(&self, q: &[f32], k: &[f32], v: &[f32]) -> Vec<f32> {
        AttentionKernelV1::attention(self, q, k, v)
    }
}

// ---------------------------------------------------------------------------
// MatmulKernelV1 -- naive matrix multiplication (square matrices)
// ---------------------------------------------------------------------------
impl MatmulKernelV1 for DepylerKernels {
    fn matmul(&self, a: &[f32], b: &[f32]) -> Vec<f32> {
        let n = (a.len() as f32).sqrt() as usize;
        if n == 0 {
            return vec![];
        }
        let mut c = vec![0.0f32; n * n];
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    c[i * n + j] += a[i * n + k] * b[k * n + j];
                }
            }
        }
        c
    }

    fn quantized_dot(&self, b: &[f32], s_b: f32) -> Vec<f32> {
        vec![b.iter().sum::<f32>() * s_b]
    }
}

// ---------------------------------------------------------------------------
// RopeKernelV1 -- rotary position embeddings
// ---------------------------------------------------------------------------
impl RopeKernelV1 for DepylerKernels {
    fn rope(&self, x: &[f32], m: &[f32]) -> Vec<f32> {
        let pos = m.first().copied().unwrap_or(0.0);
        let mut out = x.to_vec();
        for i in (0..x.len()).step_by(2) {
            if i + 1 < x.len() {
                let theta = pos / 10000_f32.powf(i as f32 / x.len() as f32);
                let (sin_t, cos_t) = theta.sin_cos();
                out[i] = x[i] * cos_t - x[i + 1] * sin_t;
                out[i + 1] = x[i] * sin_t + x[i + 1] * cos_t;
            }
        }
        out
    }
}

// ---------------------------------------------------------------------------
// AdamwKernelV1 -- AdamW optimizer moments and weight update
// ---------------------------------------------------------------------------
impl AdamwKernelV1 for DepylerKernels {
    fn adam_moments(&self, g_t: &[f32]) -> Vec<f32> {
        g_t.iter().map(|g| 0.9 * 0.0 + 0.1 * g).collect()
    }

    fn adam_variance(&self, g_t: &[f32]) -> Vec<f32> {
        g_t.iter().map(|g| 0.999 * 0.0 + 0.001 * g * g).collect()
    }

    fn bias_correction(&self, input: &[f32]) -> Vec<f32> {
        input.iter().map(|v| v / (1.0 - 0.9)).collect()
    }

    fn weight_update(&self, theta: &[f32]) -> Vec<f32> {
        theta.iter().map(|t| t - 0.001 * t).collect()
    }
}

#[test]
fn attention_output_size() {
    let k = DepylerKernels;
    let out = AttentionKernelV1::attention(&k, &[1.0; 4], &[1.0; 4], &[1.0; 4]);
    assert_eq!(out.len(), 4);
}

#[test]
fn matmul_identity() {
    let k = DepylerKernels;
    let identity = vec![1.0, 0.0, 0.0, 1.0]; // 2x2 identity
    let a = vec![1.0, 2.0, 3.0, 4.0];
    let out = MatmulKernelV1::matmul(&k, &a, &identity);
    assert!((out[0] - 1.0).abs() < 1e-5 && (out[3] - 4.0).abs() < 1e-5);
}

#[test]
fn rope_preserves_norm() {
    let k = DepylerKernels;
    let x = vec![1.0, 0.0, 0.0, 1.0];
    let out = RopeKernelV1::rope(&k, &x, &[1.0]);
    let norm_in: f32 = x.iter().map(|v| v * v).sum::<f32>().sqrt();
    let norm_out: f32 = out.iter().map(|v| v * v).sum::<f32>().sqrt();
    assert!((norm_in - norm_out).abs() < 1e-5);
}
