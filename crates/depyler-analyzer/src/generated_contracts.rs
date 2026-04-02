// Auto-generated contract assertions from YAML — DO NOT EDIT.
// Zero cost in release builds (debug_assert!).
// Regenerate: pv codegen contracts/ -o src/generated_contracts.rs
// Include:   #[macro_use] #[allow(unused_macros)] mod generated_contracts;

// Auto-generated from contracts/absolute-position-v1.yaml — DO NOT EDIT
// Contract: absolute-position-v1

/// Preconditions for equation `absolute_position_add`.
/// Domain-specific. Call: `contract_pre_absolute_position_add!(slice_expr)`
macro_rules! contract_pre_absolute_position_add {
    () => {{}};
    ($input:expr) => {{
        let indices = &$input;
        debug_assert!(indices.len() > 0,
            "Contract absolute_position_add: precondition violated — indices.len() > 0");
    }};
}

// Auto-generated from contracts/activation-kernel-v1.yaml — DO NOT EDIT
// Contract: activation-kernel-v1

/// Preconditions for equation `gelu`.
/// Domain-specific. Call: `contract_pre_gelu!(slice_expr)`
macro_rules! contract_pre_gelu {
    () => {{}};
    ($input:expr) => {{
        let x = &$input;
        debug_assert!(x.iter().all(|v| v.is_finite()),
            "Contract gelu: precondition violated — x.iter().all(|v| v.is_finite())");
        debug_assert!(x.len() > 0,
            "Contract gelu: precondition violated — x.len() > 0");
    }};
}

/// Preconditions for equation `relu`.
/// Domain-specific. Call: `contract_pre_relu!(slice_expr)`
macro_rules! contract_pre_relu {
    () => {{}};
    ($input:expr) => {{
        let x = &$input;
        debug_assert!(x.iter().all(|v| v.is_finite()),
            "Contract relu: precondition violated — x.iter().all(|v| v.is_finite())");
        debug_assert!(x.len() > 0,
            "Contract relu: precondition violated — x.len() > 0");
    }};
}

/// Preconditions for equation `silu`.
/// Domain-specific. Call: `contract_pre_silu!(slice_expr)`
macro_rules! contract_pre_silu {
    () => {{}};
    ($input:expr) => {{
        let x = &$input;
        debug_assert!(x.iter().all(|v| v.is_finite()),
            "Contract silu: precondition violated — x.iter().all(|v| v.is_finite())");
        debug_assert!(x.len() > 0,
            "Contract silu: precondition violated — x.len() > 0");
    }};
}

// Auto-generated from contracts/active-learning-v1.yaml — DO NOT EDIT
// Contract: active-learning-v1

/// Preconditions for equation `entropy_score`.
/// Domain-specific. Call: `contract_pre_entropy_score!(slice_expr)`
macro_rules! contract_pre_entropy_score {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract entropy_score: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract entropy_score: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `margin_score`.
/// Domain-specific. Call: `contract_pre_margin_score!(slice_expr)`
macro_rules! contract_pre_margin_score {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract margin_score: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract margin_score: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `qbc_score`.
/// Domain-specific. Call: `contract_pre_qbc_score!(slice_expr)`
macro_rules! contract_pre_qbc_score {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract qbc_score: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract qbc_score: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `uncertainty_score`.
/// Domain-specific. Call: `contract_pre_uncertainty_score!(slice_expr)`
macro_rules! contract_pre_uncertainty_score {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract uncertainty_score: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract uncertainty_score: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

// Auto-generated from contracts/adamw-kernel-v1.yaml — DO NOT EDIT
// Contract: adamw-kernel-v1

/// Preconditions for equation `adam_moments`.
/// Domain-specific. Call: `contract_pre_adam_moments!(slice_expr)`
macro_rules! contract_pre_adam_moments {
    () => {{}};
    ($input:expr) => {{
        let params = &$input;
        debug_assert!(params.len() > 0,
            "Contract adam_moments: precondition violated — params.len() > 0");
    }};
}

/// Preconditions for equation `adam_variance`.
/// Domain-specific. Call: `contract_pre_adam_variance!(slice_expr)`
macro_rules! contract_pre_adam_variance {
    () => {{}};
    ($input:expr) => {{
        let params = &$input;
        debug_assert!(params.len() > 0,
            "Contract adam_variance: precondition violated — params.len() > 0");
    }};
}

/// Preconditions for equation `bias_correction`.
/// Domain-specific. Call: `contract_pre_bias_correction!(slice_expr)`
macro_rules! contract_pre_bias_correction {
    () => {{}};
    ($input:expr) => {{
        let params = &$input;
        debug_assert!(params.len() > 0,
            "Contract bias_correction: precondition violated — params.len() > 0");
    }};
}

/// Preconditions for equation `weight_update`.
/// Domain-specific. Call: `contract_pre_weight_update!(slice_expr)`
macro_rules! contract_pre_weight_update {
    () => {{}};
    ($input:expr) => {{
        let params = &$input;
        debug_assert!(params.len() > 0,
            "Contract weight_update: precondition violated — params.len() > 0");
    }};
}

// Auto-generated from contracts/alibi-kernel-v1.yaml — DO NOT EDIT
// Contract: alibi-kernel-v1

/// Preconditions for equation `alibi_bias`.
/// Domain-specific. Call: `contract_pre_alibi_bias!(slice_expr)`
macro_rules! contract_pre_alibi_bias {
    () => {{}};
    ($input:expr) => {{
        let indices = &$input;
        debug_assert!(indices.len() > 0,
            "Contract alibi_bias: precondition violated — indices.len() > 0");
    }};
}

/// Preconditions for equation `alibi_slopes`.
/// Domain-specific. Call: `contract_pre_alibi_slopes!(slice_expr)`
macro_rules! contract_pre_alibi_slopes {
    () => {{}};
    ($input:expr) => {{
        let indices = &$input;
        debug_assert!(indices.len() > 0,
            "Contract alibi_slopes: precondition violated — indices.len() > 0");
    }};
}

// Auto-generated from contracts/apr-checkpoint-v1.yaml — DO NOT EDIT
// Contract: apr-checkpoint-v1

/// Preconditions for equation `identity`.
/// Domain-specific. Call: `contract_pre_identity!(slice_expr)`
macro_rules! contract_pre_identity {
    () => {{}};
    ($input:expr) => {{
        let data = &$input;
        debug_assert!(!data.is_empty(),
            "Contract identity: precondition violated — !data.is_empty()");
        debug_assert!(data.len() > 0,
            "Contract identity: precondition violated — data.len() > 0");
    }};
}

// Auto-generated from contracts/apr-format-invariants-v1.yaml — DO NOT EDIT
// Contract: apr-format-invariants-v1

/// Preconditions for equation `detect_regression`.
/// Domain-specific. Call: `contract_pre_detect_regression!(slice_expr)`
macro_rules! contract_pre_detect_regression {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract detect_regression: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `format_report`.
/// Domain-specific. Call: `contract_pre_format_report!(slice_expr)`
macro_rules! contract_pre_format_report {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract format_report: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `parse_playbook`.
/// Domain-specific. Call: `contract_pre_parse_playbook!(slice_expr)`
macro_rules! contract_pre_parse_playbook {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract parse_playbook: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `serialize_roundtrip`.
/// Domain-specific. Call: `contract_pre_serialize_roundtrip!(slice_expr)`
macro_rules! contract_pre_serialize_roundtrip {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract serialize_roundtrip: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `validate_schema`.
/// Domain-specific. Call: `contract_pre_validate_schema!(slice_expr)`
macro_rules! contract_pre_validate_schema {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract validate_schema: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/arch-constraints-v1.yaml — DO NOT EDIT
// Contract: arch-constraints-v1

/// Preconditions for equation `arch_constraint_lookup`.
/// Domain-specific. Call: `contract_pre_arch_constraint_lookup!(slice_expr)`
macro_rules! contract_pre_arch_constraint_lookup {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract arch_constraint_lookup: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/architecture-requirements-v1.yaml — DO NOT EDIT
// Contract: architecture-requirements-v1

/// Preconditions for equation `constraint_matrix_exhaustiveness`.
/// Domain-specific. Call: `contract_pre_constraint_matrix_exhaustiveness!(slice_expr)`
macro_rules! contract_pre_constraint_matrix_exhaustiveness {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract constraint_matrix_exhaustiveness: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `role_mapping`.
/// Domain-specific. Call: `contract_pre_role_mapping!(slice_expr)`
macro_rules! contract_pre_role_mapping {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract role_mapping: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `weight_completeness`.
/// Domain-specific. Call: `contract_pre_weight_completeness!(slice_expr)`
macro_rules! contract_pre_weight_completeness {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract weight_completeness: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/arima-v1.yaml — DO NOT EDIT
// Contract: arima-v1

/// Preconditions for equation `ar_forecast`.
/// Domain-specific. Call: `contract_pre_ar_forecast!(slice_expr)`
macro_rules! contract_pre_ar_forecast {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract ar_forecast: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract ar_forecast: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `differencing`.
/// Domain-specific. Call: `contract_pre_differencing!(slice_expr)`
macro_rules! contract_pre_differencing {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract differencing: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract differencing: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `forecast_finite`.
/// Domain-specific. Call: `contract_pre_forecast_finite!(slice_expr)`
macro_rules! contract_pre_forecast_finite {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract forecast_finite: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract forecast_finite: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `ma_filter`.
/// Domain-specific. Call: `contract_pre_ma_filter!(slice_expr)`
macro_rules! contract_pre_ma_filter {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract ma_filter: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract ma_filter: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

// Auto-generated from contracts/attention-head-extraction-v1.yaml — DO NOT EDIT
// Contract: attention-head-extraction-v1

/// Preconditions for equation `identity`.
/// Domain-specific. Call: `contract_pre_identity!(slice_expr)`
macro_rules! contract_pre_identity {
    () => {{}};
    ($input:expr) => {{
        let q = &$input;
        debug_assert!(q.len() > 0,
            "Contract identity: precondition violated — q.len() > 0");
    }};
}

// Auto-generated from contracts/attention-kernel-v1.yaml — DO NOT EDIT
// Contract: attention-kernel-v1

/// Preconditions for equation `attention`.
/// Domain-specific. Call: `contract_pre_attention!(slice_expr)`
macro_rules! contract_pre_attention {
    () => {{}};
    ($input:expr) => {{
        let query = &$input;
        debug_assert!(query.len() > 0,
            "Contract attention: precondition violated — query.len() > 0");
    }};
}

/// Postconditions for equation `attention`.
/// Call before return: `contract_post_attention!(result_expr)`
macro_rules! contract_post_attention {
    ($result:expr) => {{
        let _contract_result = &$result;
        debug_assert!(_contract_result.iter().all(|v| v.is_finite()), "Contract attention: postcondition violated — result.iter().all(|v| v.is_finite())");
    }};
}

/// Combined pre+post contract for equation `attention`.
macro_rules! contract_attention {
    ($input:expr, $body:expr) => {{
        contract_pre_attention!($input);
        let _contract_result = $body;
        contract_post_attention!(_contract_result);
        _contract_result
    }};
}

// Auto-generated from contracts/attention-scaling-v1.yaml — DO NOT EDIT
// Contract: attention-scaling-v1

/// Preconditions for equation `attention_entropy`.
/// Domain-specific. Call: `contract_pre_attention_entropy!(slice_expr)`
macro_rules! contract_pre_attention_entropy {
    () => {{}};
    ($input:expr) => {{
        let q = &$input;
        debug_assert!(q.len() > 0,
            "Contract attention_entropy: precondition violated — q.len() > 0");
    }};
}

/// Preconditions for equation `numerical_stability`.
/// Domain-specific. Call: `contract_pre_numerical_stability!(slice_expr)`
macro_rules! contract_pre_numerical_stability {
    () => {{}};
    ($input:expr) => {{
        let q = &$input;
        debug_assert!(q.len() > 0,
            "Contract numerical_stability: precondition violated — q.len() > 0");
    }};
}

/// Preconditions for equation `scaled_dot_product`.
/// Domain-specific. Call: `contract_pre_scaled_dot_product!(slice_expr)`
macro_rules! contract_pre_scaled_dot_product {
    () => {{}};
    ($input:expr) => {{
        let a = &$input;
        debug_assert!(a.len() > 0,
            "Contract scaled_dot_product: precondition violated — a.len() > 0");
    }};
}

/// Preconditions for equation `score_bound_with_qknorm`.
/// Domain-specific. Call: `contract_pre_score_bound_with_qknorm!(slice_expr)`
macro_rules! contract_pre_score_bound_with_qknorm {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract score_bound_with_qknorm: precondition violated — input.iter().all(|v| v.is_finite())");
        debug_assert!(input.len() > 0,
            "Contract score_bound_with_qknorm: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `softmax_saturation`.
/// Domain-specific. Call: `contract_pre_softmax_saturation!(slice_expr)`
macro_rules! contract_pre_softmax_saturation {
    () => {{}};
    ($input:expr) => {{
        let x = &$input;
        debug_assert!(x.iter().all(|v| v.is_finite()),
            "Contract softmax_saturation: precondition violated — x.iter().all(|v| v.is_finite())");
        debug_assert!(x.len() > 0,
            "Contract softmax_saturation: precondition violated — x.len() > 0");
    }};
}

/// Preconditions for equation `variance_preservation`.
/// Domain-specific. Call: `contract_pre_variance_preservation!(slice_expr)`
macro_rules! contract_pre_variance_preservation {
    () => {{}};
    ($input:expr) => {{
        let q = &$input;
        debug_assert!(q.len() > 0,
            "Contract variance_preservation: precondition violated — q.len() > 0");
    }};
}

// Auto-generated from contracts/avx2-fma-dot-v1.yaml — DO NOT EDIT
// Contract: avx2-fma-dot-v1

/// Preconditions for equation `dot_product`.
/// Domain-specific. Call: `contract_pre_dot_product!(slice_expr)`
macro_rules! contract_pre_dot_product {
    () => {{}};
    ($input:expr) => {{
        let a = &$input;
        debug_assert!(a.len() > 0,
            "Contract dot_product: precondition violated — a.len() > 0");
    }};
}

/// Preconditions for equation `fma_accumulation`.
/// Domain-specific. Call: `contract_pre_fma_accumulation!(slice_expr)`
macro_rules! contract_pre_fma_accumulation {
    () => {{}};
    ($input:expr) => {{
        let a = &$input;
        debug_assert!(a.len() > 0,
            "Contract fma_accumulation: precondition violated — a.len() > 0");
    }};
}

// Auto-generated from contracts/backend-dispatch-v1.yaml — DO NOT EDIT
// Contract: backend-dispatch-v1

/// Preconditions for equation `garbage_oracle`.
/// Domain-specific. Call: `contract_pre_garbage_oracle!(slice_expr)`
macro_rules! contract_pre_garbage_oracle {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract garbage_oracle: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract garbage_oracle: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `gpu_threshold`.
/// Domain-specific. Call: `contract_pre_gpu_threshold!(slice_expr)`
macro_rules! contract_pre_gpu_threshold {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract gpu_threshold: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract gpu_threshold: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `qk_norm_score_bound`.
/// Domain-specific. Call: `contract_pre_qk_norm_score_bound!(slice_expr)`
macro_rules! contract_pre_qk_norm_score_bound {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract qk_norm_score_bound: precondition violated — input.iter().all(|v| v.is_finite())");
        debug_assert!(input.len() > 0,
            "Contract qk_norm_score_bound: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `simd_only_threshold`.
/// Domain-specific. Call: `contract_pre_simd_only_threshold!(slice_expr)`
macro_rules! contract_pre_simd_only_threshold {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract simd_only_threshold: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract simd_only_threshold: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

// Auto-generated from contracts/batch-training-v1.yaml — DO NOT EDIT
// Contract: batch-training-v1

/// Preconditions for equation `batch_loss`.
/// Domain-specific. Call: `contract_pre_batch_loss!(slice_expr)`
macro_rules! contract_pre_batch_loss {
    () => {{}};
    ($input:expr) => {{
        let predicted = &$input;
        debug_assert!(predicted.len() > 0,
            "Contract batch_loss: precondition violated — predicted.len() > 0");
    }};
}

/// Preconditions for equation `gradient_accumulation`.
/// Domain-specific. Call: `contract_pre_gradient_accumulation!(slice_expr)`
macro_rules! contract_pre_gradient_accumulation {
    () => {{}};
    ($input:expr) => {{
        let params = &$input;
        debug_assert!(params.len() > 0,
            "Contract gradient_accumulation: precondition violated — params.len() > 0");
    }};
}

/// Preconditions for equation `gradient_clipping`.
/// Domain-specific. Call: `contract_pre_gradient_clipping!(slice_expr)`
macro_rules! contract_pre_gradient_clipping {
    () => {{}};
    ($input:expr) => {{
        let params = &$input;
        debug_assert!(params.len() > 0,
            "Contract gradient_clipping: precondition violated — params.len() > 0");
    }};
}

// Auto-generated from contracts/batched-beam-search-v1.yaml — DO NOT EDIT
// Contract: batched-beam-search-v1

/// Preconditions for equation `batched_beam_projection`.
/// Domain-specific. Call: `contract_pre_batched_beam_projection!(slice_expr)`
macro_rules! contract_pre_batched_beam_projection {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract batched_beam_projection: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `beam_selection`.
/// Domain-specific. Call: `contract_pre_beam_selection!(slice_expr)`
macro_rules! contract_pre_beam_selection {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract beam_selection: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `sequential_beam_projection`.
/// Domain-specific. Call: `contract_pre_sequential_beam_projection!(slice_expr)`
macro_rules! contract_pre_sequential_beam_projection {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract sequential_beam_projection: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `termination`.
/// Domain-specific. Call: `contract_pre_termination!(slice_expr)`
macro_rules! contract_pre_termination {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract termination: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/batchnorm-kernel-v1.yaml — DO NOT EDIT
// Contract: batchnorm-kernel-v1

/// Preconditions for equation `batchnorm_eval`.
/// Domain-specific. Call: `contract_pre_batchnorm_eval!(slice_expr)`
macro_rules! contract_pre_batchnorm_eval {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract batchnorm_eval: precondition violated — input.iter().all(|v| v.is_finite())");
        debug_assert!(input.len() > 0,
            "Contract batchnorm_eval: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `batchnorm_train`.
/// Domain-specific. Call: `contract_pre_batchnorm_train!(slice_expr)`
macro_rules! contract_pre_batchnorm_train {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract batchnorm_train: precondition violated — input.iter().all(|v| v.is_finite())");
        debug_assert!(input.len() > 0,
            "Contract batchnorm_train: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `running_stats`.
/// Domain-specific. Call: `contract_pre_running_stats!(slice_expr)`
macro_rules! contract_pre_running_stats {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract running_stats: precondition violated — input.iter().all(|v| v.is_finite())");
        debug_assert!(input.len() > 0,
            "Contract running_stats: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/bayesian-v1.yaml — DO NOT EDIT
// Contract: bayesian-v1

/// Preconditions for equation `blr_predict`.
/// Domain-specific. Call: `contract_pre_blr_predict!(slice_expr)`
macro_rules! contract_pre_blr_predict {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract blr_predict: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract blr_predict: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `conjugate_update`.
/// Domain-specific. Call: `contract_pre_conjugate_update!(slice_expr)`
macro_rules! contract_pre_conjugate_update {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract conjugate_update: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract conjugate_update: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `posterior_predictive`.
/// Domain-specific. Call: `contract_pre_posterior_predictive!(slice_expr)`
macro_rules! contract_pre_posterior_predictive {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract posterior_predictive: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract posterior_predictive: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `posterior_valid`.
/// Domain-specific. Call: `contract_pre_posterior_valid!(slice_expr)`
macro_rules! contract_pre_posterior_valid {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract posterior_valid: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract posterior_valid: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

// Auto-generated from contracts/bias-add-v1.yaml — DO NOT EDIT
// Contract: bias-add-v1

/// Preconditions for equation `bias_add`.
/// Domain-specific. Call: `contract_pre_bias_add!(slice_expr)`
macro_rules! contract_pre_bias_add {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract bias_add: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract bias_add: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

// Auto-generated from contracts/bidirectional-attention-v1.yaml — DO NOT EDIT
// Contract: bidirectional-attention-v1

/// Preconditions for equation `bidirectional_attention`.
/// Domain-specific. Call: `contract_pre_bidirectional_attention!(slice_expr)`
macro_rules! contract_pre_bidirectional_attention {
    () => {{}};
    ($input:expr) => {{
        let q = &$input;
        debug_assert!(q.len() > 0,
            "Contract bidirectional_attention: precondition violated — q.len() > 0");
    }};
}

// Auto-generated from contracts/blake3-state-v1.yaml — DO NOT EDIT
// Contract: blake3-state-v1

/// Preconditions for equation `composite_hash`.
/// Domain-specific. Call: `contract_pre_composite_hash!(slice_expr)`
macro_rules! contract_pre_composite_hash {
    () => {{}};
    ($input:expr) => {{
        let parts = &$input;
        debug_assert!(parts.len() > 0,
            "Contract composite_hash: precondition violated — parts.len() > 0");
    }};
}

/// Preconditions for equation `hash_file`.
/// Call at function entry: `contract_pre_hash_file!(input_expr)`
macro_rules! contract_pre_hash_file {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `hash_string`.
/// Domain-specific. Call: `contract_pre_hash_string!(slice_expr)`
macro_rules! contract_pre_hash_string {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(!input.is_empty(),
            "Contract hash_string: precondition violated — !input.is_empty()");
        debug_assert!(input.len() <= 1_073_741_824,
            "Contract hash_string: precondition violated — input.len() <= 1_073_741_824");
    }};
}

// Auto-generated from contracts/bpe-tokenization-v1.yaml — DO NOT EDIT
// Contract: bpe-tokenization-v1

/// Preconditions for equation `decode`.
/// Domain-specific. Call: `contract_pre_decode!(slice_expr)`
macro_rules! contract_pre_decode {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract decode: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `encode`.
/// Domain-specific. Call: `contract_pre_encode!(slice_expr)`
macro_rules! contract_pre_encode {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract encode: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `merge_rule`.
/// Domain-specific. Call: `contract_pre_merge_rule!(slice_expr)`
macro_rules! contract_pre_merge_rule {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract merge_rule: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/builder-pattern-v1.yaml — DO NOT EDIT
// Contract: builder-pattern-v1

/// Preconditions for equation `builder_pattern`.
/// Domain-specific. Call: `contract_pre_builder_pattern!(slice_expr)`
macro_rules! contract_pre_builder_pattern {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract builder_pattern: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/calibration-v1.yaml — DO NOT EDIT
// Contract: calibration-v1

/// Preconditions for equation `expected_calibration_error`.
/// Domain-specific. Call: `contract_pre_expected_calibration_error!(slice_expr)`
macro_rules! contract_pre_expected_calibration_error {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract expected_calibration_error: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract expected_calibration_error: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `isotonic_regression`.
/// Domain-specific. Call: `contract_pre_isotonic_regression!(slice_expr)`
macro_rules! contract_pre_isotonic_regression {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract isotonic_regression: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract isotonic_regression: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `maximum_calibration_error`.
/// Domain-specific. Call: `contract_pre_maximum_calibration_error!(slice_expr)`
macro_rules! contract_pre_maximum_calibration_error {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract maximum_calibration_error: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract maximum_calibration_error: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `platt_scaling`.
/// Domain-specific. Call: `contract_pre_platt_scaling!(slice_expr)`
macro_rules! contract_pre_platt_scaling {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract platt_scaling: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract platt_scaling: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `reliability_diagram`.
/// Domain-specific. Call: `contract_pre_reliability_diagram!(slice_expr)`
macro_rules! contract_pre_reliability_diagram {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract reliability_diagram: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract reliability_diagram: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

// Auto-generated from contracts/classification-finetune-v1.yaml — DO NOT EDIT
// Contract: classification-finetune-v1

/// Preconditions for equation `classifier_weight_shape`.
/// Domain-specific. Call: `contract_pre_classifier_weight_shape!(slice_expr)`
macro_rules! contract_pre_classifier_weight_shape {
    () => {{}};
    ($input:expr) => {{
        let a = &$input;
        debug_assert!(a.len() > 0,
            "Contract classifier_weight_shape: precondition violated — a.len() > 0");
    }};
}

/// Preconditions for equation `label_bounds`.
/// Domain-specific. Call: `contract_pre_label_bounds!(slice_expr)`
macro_rules! contract_pre_label_bounds {
    () => {{}};
    ($input:expr) => {{
        let a = &$input;
        debug_assert!(a.len() > 0,
            "Contract label_bounds: precondition violated — a.len() > 0");
    }};
}

/// Preconditions for equation `logit_shape`.
/// Domain-specific. Call: `contract_pre_logit_shape!(slice_expr)`
macro_rules! contract_pre_logit_shape {
    () => {{}};
    ($input:expr) => {{
        let a = &$input;
        debug_assert!(a.len() > 0,
            "Contract logit_shape: precondition violated — a.len() > 0");
    }};
}

/// Preconditions for equation `softmax_sum`.
/// Domain-specific. Call: `contract_pre_softmax_sum!(slice_expr)`
macro_rules! contract_pre_softmax_sum {
    () => {{}};
    ($input:expr) => {{
        let x = &$input;
        debug_assert!(x.iter().all(|v| v.is_finite()),
            "Contract softmax_sum: precondition violated — x.iter().all(|v| v.is_finite())");
        debug_assert!(x.len() > 0,
            "Contract softmax_sum: precondition violated — x.len() > 0");
    }};
}

// Auto-generated from contracts/classifier-pipeline-v1.yaml — DO NOT EDIT
// Contract: classifier-pipeline-v1

/// Preconditions for equation `embedding_extraction`.
/// Domain-specific. Call: `contract_pre_embedding_extraction!(slice_expr)`
macro_rules! contract_pre_embedding_extraction {
    () => {{}};
    ($input:expr) => {{
        let indices = &$input;
        debug_assert!(indices.len() > 0,
            "Contract embedding_extraction: precondition violated — indices.len() > 0");
    }};
}

/// Preconditions for equation `evaluation`.
/// Call at function entry: `contract_pre_evaluation!(input_expr)`
macro_rules! contract_pre_evaluation {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `linear_probe`.
/// Call at function entry: `contract_pre_linear_probe!(input_expr)`
macro_rules! contract_pre_linear_probe {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

// Auto-generated from contracts/cma-es-kernel-v1.yaml — DO NOT EDIT
// Contract: cma-es-kernel-v1

/// Preconditions for equation `covariance_update`.
/// Domain-specific. Call: `contract_pre_covariance_update!(slice_expr)`
macro_rules! contract_pre_covariance_update {
    () => {{}};
    ($input:expr) => {{
        let params = &$input;
        debug_assert!(params.len() > 0,
            "Contract covariance_update: precondition violated — params.len() > 0");
    }};
}

/// Preconditions for equation `mean_update`.
/// Domain-specific. Call: `contract_pre_mean_update!(slice_expr)`
macro_rules! contract_pre_mean_update {
    () => {{}};
    ($input:expr) => {{
        let params = &$input;
        debug_assert!(params.len() > 0,
            "Contract mean_update: precondition violated — params.len() > 0");
    }};
}

/// Preconditions for equation `sample`.
/// Domain-specific. Call: `contract_pre_sample!(slice_expr)`
macro_rules! contract_pre_sample {
    () => {{}};
    ($input:expr) => {{
        let params = &$input;
        debug_assert!(params.len() > 0,
            "Contract sample: precondition violated — params.len() > 0");
    }};
}

// Auto-generated from contracts/codebert-tokenizer-validation-v1.yaml — DO NOT EDIT
// Contract: codebert-tokenizer-validation-v1

/// Preconditions for equation `tokenizer_adequacy`.
/// Domain-specific. Call: `contract_pre_tokenizer_adequacy!(slice_expr)`
macro_rules! contract_pre_tokenizer_adequacy {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract tokenizer_adequacy: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/codegen-dispatch-v1.yaml — DO NOT EDIT
// Contract: codegen-dispatch-v1

/// Preconditions for equation `apply_script`.
/// Call at function entry: `contract_pre_apply_script!(input_expr)`
macro_rules! contract_pre_apply_script {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `check_script`.
/// Call at function entry: `contract_pre_check_script!(input_expr)`
macro_rules! contract_pre_check_script {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `state_query_script`.
/// Call at function entry: `contract_pre_state_query_script!(input_expr)`
macro_rules! contract_pre_state_query_script {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

// Auto-generated from contracts/comply-check-v1.yaml — DO NOT EDIT
// Contract: comply-check-v1

/// Preconditions for equation `aggregate_score`.
/// Domain-specific. Call: `contract_pre_aggregate_score!(slice_expr)`
macro_rules! contract_pre_aggregate_score {
    () => {{}};
    ($input:expr) => {{
        let checks = &$input;
        debug_assert!(checks.len() > 0,
            "Contract aggregate_score: precondition violated — checks.len() > 0");
    }};
}

/// Preconditions for equation `run_checks`.
/// Call at function entry: `contract_pre_run_checks!(input_expr)`
macro_rules! contract_pre_run_checks {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

// Auto-generated from contracts/configuration-v1.yaml — DO NOT EDIT
// Contract: configuration-v1

/// Preconditions for equation `configuration`.
/// Domain-specific. Call: `contract_pre_configuration!(slice_expr)`
macro_rules! contract_pre_configuration {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract configuration: precondition violated — input.len() > 0");
    }};
}

/// Postconditions for equation `configuration`.
/// Call before return: `contract_post_configuration!(result_expr)`
macro_rules! contract_post_configuration {
    ($result:expr) => {{
        let _contract_result = &$result;
        debug_assert!(_contract_result.len() > 0, "Contract configuration: postcondition violated — result.len() > 0");
    }};
}

/// Combined pre+post contract for equation `configuration`.
macro_rules! contract_configuration {
    ($input:expr, $body:expr) => {{
        contract_pre_configuration!($input);
        let _contract_result = $body;
        contract_post_configuration!(_contract_result);
        _contract_result
    }};
}

// Auto-generated from contracts/context-generation-v1.yaml — DO NOT EDIT
// Contract: context-generation-v1

/// Preconditions for equation `generate_context`.
/// Call at function entry: `contract_pre_generate_context!(input_expr)`
macro_rules! contract_pre_generate_context {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `index_persistence`.
/// Call at function entry: `contract_pre_index_persistence!(input_expr)`
macro_rules! contract_pre_index_persistence {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

// Auto-generated from contracts/continuous-batching-v1.yaml — DO NOT EDIT
// Contract: continuous-batching-v1

/// Preconditions for equation `chunked_prefill`.
/// Call at function entry: `contract_pre_chunked_prefill!(input_expr)`
macro_rules! contract_pre_chunked_prefill {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `correctness_under_batching`.
/// Call at function entry: `contract_pre_correctness_under_batching!(input_expr)`
macro_rules! contract_pre_correctness_under_batching {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `decode_degradation`.
/// Domain-specific. Call: `contract_pre_decode_degradation!(slice_expr)`
macro_rules! contract_pre_decode_degradation {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract decode_degradation: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `request_state`.
/// Call at function entry: `contract_pre_request_state!(input_expr)`
macro_rules! contract_pre_request_state {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `scheduling_fairness`.
/// Call at function entry: `contract_pre_scheduling_fairness!(input_expr)`
macro_rules! contract_pre_scheduling_fairness {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `throughput_scaling`.
/// Call at function entry: `contract_pre_throughput_scaling!(input_expr)`
macro_rules! contract_pre_throughput_scaling {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `token_budget`.
/// Domain-specific. Call: `contract_pre_token_budget!(slice_expr)`
macro_rules! contract_pre_token_budget {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract token_budget: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/conv1d-kernel-v1.yaml — DO NOT EDIT
// Contract: conv1d-kernel-v1

/// Preconditions for equation `conv1d`.
/// Domain-specific. Call: `contract_pre_conv1d!(slice_expr)`
macro_rules! contract_pre_conv1d {
    () => {{}};
    ($input:expr) => {{
        let a = &$input;
        debug_assert!(a.len() > 0,
            "Contract conv1d: precondition violated — a.len() > 0");
    }};
}

// Auto-generated from contracts/conversation-generation-v1.yaml — DO NOT EDIT
// Contract: conversation-generation-v1

/// Preconditions for equation `chatml_format`.
/// Domain-specific. Call: `contract_pre_chatml_format!(slice_expr)`
macro_rules! contract_pre_chatml_format {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract chatml_format: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `conversation_types`.
/// Domain-specific. Call: `contract_pre_conversation_types!(slice_expr)`
macro_rules! contract_pre_conversation_types {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract conversation_types: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `quality_gate`.
/// Domain-specific. Call: `contract_pre_quality_gate!(slice_expr)`
macro_rules! contract_pre_quality_gate {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract quality_gate: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/cpu-q4k-activation-quant-v1.yaml — DO NOT EDIT
// Contract: cpu-q4k-activation-quant-v1

/// Preconditions for equation `current_path`.
/// Domain-specific. Call: `contract_pre_current_path!(slice_expr)`
macro_rules! contract_pre_current_path {
    () => {{}};
    ($input:expr) => {{
        let x = &$input;
        debug_assert!(x.iter().all(|v| v.is_finite()),
            "Contract current_path: precondition violated — x.iter().all(|v| v.is_finite())");
        debug_assert!(x.len() > 0,
            "Contract current_path: precondition violated — x.len() > 0");
    }};
}

/// Preconditions for equation `speedup_bound`.
/// Domain-specific. Call: `contract_pre_speedup_bound!(slice_expr)`
macro_rules! contract_pre_speedup_bound {
    () => {{}};
    ($input:expr) => {{
        let x = &$input;
        debug_assert!(x.iter().all(|v| v.is_finite()),
            "Contract speedup_bound: precondition violated — x.iter().all(|v| v.is_finite())");
        debug_assert!(x.len() > 0,
            "Contract speedup_bound: precondition violated — x.len() > 0");
    }};
}

/// Preconditions for equation `target_path`.
/// Domain-specific. Call: `contract_pre_target_path!(slice_expr)`
macro_rules! contract_pre_target_path {
    () => {{}};
    ($input:expr) => {{
        let x = &$input;
        debug_assert!(x.iter().all(|v| v.is_finite()),
            "Contract target_path: precondition violated — x.iter().all(|v| v.is_finite())");
        debug_assert!(x.len() > 0,
            "Contract target_path: precondition violated — x.len() > 0");
    }};
}

// Auto-generated from contracts/cpu-work-stealing-v1.yaml — DO NOT EDIT
// Contract: cpu-work-stealing-v1

/// Preconditions for equation `l1_tiling`.
/// Domain-specific. Call: `contract_pre_l1_tiling!(slice_expr)`
macro_rules! contract_pre_l1_tiling {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract l1_tiling: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract l1_tiling: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `rayon_overhead`.
/// Domain-specific. Call: `contract_pre_rayon_overhead!(slice_expr)`
macro_rules! contract_pre_rayon_overhead {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract rayon_overhead: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract rayon_overhead: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

// Auto-generated from contracts/cross-entropy-kernel-v1.yaml — DO NOT EDIT
// Contract: cross-entropy-kernel-v1

/// Preconditions for equation `cross_entropy`.
/// Domain-specific. Call: `contract_pre_cross_entropy!(slice_expr)`
macro_rules! contract_pre_cross_entropy {
    () => {{}};
    ($input:expr) => {{
        let logits = &$input;
        debug_assert!(logits.len() > 0,
            "Contract cross_entropy: precondition violated — logits.len() > 0");
        debug_assert!(logits.iter().all(|v| v.is_finite()),
            "Contract cross_entropy: precondition violated — logits.iter().all(|v| v.is_finite())");
    }};
}

/// Postconditions for equation `cross_entropy`.
/// Call before return: `contract_post_cross_entropy!(result_expr)`
macro_rules! contract_post_cross_entropy {
    ($result:expr) => {{
        let _contract_result = &$result;
        debug_assert!(_contract_result.is_finite(), "Contract cross_entropy: postcondition violated — result.is_finite()");
        debug_assert!(*_contract_result >= 0.0, "Contract cross_entropy: postcondition violated — result >= 0.0");
    }};
}

/// Combined pre+post contract for equation `cross_entropy`.
macro_rules! contract_cross_entropy {
    ($input:expr, $body:expr) => {{
        contract_pre_cross_entropy!($input);
        let _contract_result = $body;
        contract_post_cross_entropy!(_contract_result);
        _contract_result
    }};
}

/// Preconditions for equation `log_softmax`.
/// Domain-specific. Call: `contract_pre_log_softmax!(slice_expr)`
macro_rules! contract_pre_log_softmax {
    () => {{}};
    ($input:expr) => {{
        let x = &$input;
        debug_assert!(x.iter().all(|v| v.is_finite()),
            "Contract log_softmax: precondition violated — x.iter().all(|v| v.is_finite())");
        debug_assert!(x.len() > 0,
            "Contract log_softmax: precondition violated — x.len() > 0");
    }};
}

// Auto-generated from contracts/cuda-classify-training-v1.yaml — DO NOT EDIT
// Contract: cuda-classify-training-v1

/// Preconditions for equation `device_dispatch`.
/// Call at function entry: `contract_pre_device_dispatch!(input_expr)`
macro_rules! contract_pre_device_dispatch {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `gpu_forward`.
/// Call at function entry: `contract_pre_gpu_forward!(input_expr)`
macro_rules! contract_pre_gpu_forward {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `weight_roundtrip`.
/// Domain-specific. Call: `contract_pre_weight_roundtrip!(slice_expr)`
macro_rules! contract_pre_weight_roundtrip {
    () => {{}};
    ($input:expr) => {{
        let weights = &$input;
        debug_assert!(weights.len() > 0,
            "Contract weight_roundtrip: precondition violated — weights.len() > 0");
    }};
}

// Auto-generated from contracts/dag-ordering-v1.yaml — DO NOT EDIT
// Contract: dag-ordering-v1

/// Preconditions for equation `kahn_sort`.
/// Call at function entry: `contract_pre_kahn_sort!(input_expr)`
macro_rules! contract_pre_kahn_sort {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `topological_sort`.
/// Call at function entry: `contract_pre_topological_sort!(input_expr)`
macro_rules! contract_pre_topological_sort {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

// Auto-generated from contracts/decision-tree-v1.yaml — DO NOT EDIT
// Contract: decision-tree-v1

/// Preconditions for equation `gini_impurity`.
/// Domain-specific. Call: `contract_pre_gini_impurity!(slice_expr)`
macro_rules! contract_pre_gini_impurity {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract gini_impurity: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract gini_impurity: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `gini_split`.
/// Domain-specific. Call: `contract_pre_gini_split!(slice_expr)`
macro_rules! contract_pre_gini_split {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract gini_split: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract gini_split: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `mse_split`.
/// Domain-specific. Call: `contract_pre_mse_split!(slice_expr)`
macro_rules! contract_pre_mse_split {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract mse_split: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract mse_split: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `prediction`.
/// Domain-specific. Call: `contract_pre_prediction!(slice_expr)`
macro_rules! contract_pre_prediction {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract prediction: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract prediction: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

// Auto-generated from contracts/display-format-v1.yaml — DO NOT EDIT
// Contract: display-format-v1

/// Preconditions for equation `display_format`.
/// Domain-specific. Call: `contract_pre_display_format!(slice_expr)`
macro_rules! contract_pre_display_format {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract display_format: precondition violated — input.len() > 0");
    }};
}

/// Postconditions for equation `display_format`.
/// Call before return: `contract_post_display_format!(result_expr)`
macro_rules! contract_post_display_format {
    ($result:expr) => {{
        let _contract_result = &$result;
        debug_assert!(_contract_result.len() > 0, "Contract display_format: postcondition violated — result.len() > 0");
    }};
}

/// Combined pre+post contract for equation `display_format`.
macro_rules! contract_display_format {
    ($input:expr, $body:expr) => {{
        contract_pre_display_format!($input);
        let _contract_result = $body;
        contract_post_display_format!(_contract_result);
        _contract_result
    }};
}

/// Preconditions for equation `render`.
/// Domain-specific. Call: `contract_pre_render!(slice_expr)`
macro_rules! contract_pre_render {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract render: precondition violated — input.len() > 0");
    }};
}

/// Postconditions for equation `render`.
/// Call before return: `contract_post_render!(result_expr)`
macro_rules! contract_post_render {
    ($result:expr) => {{
        let _contract_result = &$result;
        debug_assert!(_contract_result.len() > 0, "Contract render: postcondition violated — result.len() > 0");
    }};
}

/// Combined pre+post contract for equation `render`.
macro_rules! contract_render {
    ($input:expr, $body:expr) => {{
        contract_pre_render!($input);
        let _contract_result = $body;
        contract_post_render!(_contract_result);
        _contract_result
    }};
}

// Auto-generated from contracts/distributed-training-v1.yaml — DO NOT EDIT
// Contract: distributed-training-v1

/// Preconditions for equation `gradient_allreduce`.
/// Domain-specific. Call: `contract_pre_gradient_allreduce!(slice_expr)`
macro_rules! contract_pre_gradient_allreduce {
    () => {{}};
    ($input:expr) => {{
        let params = &$input;
        debug_assert!(params.len() > 0,
            "Contract gradient_allreduce: precondition violated — params.len() > 0");
    }};
}

/// Preconditions for equation `lora_gradient_size`.
/// Domain-specific. Call: `contract_pre_lora_gradient_size!(slice_expr)`
macro_rules! contract_pre_lora_gradient_size {
    () => {{}};
    ($input:expr) => {{
        let grad_output = &$input;
        debug_assert!(grad_output.len() > 0,
            "Contract lora_gradient_size: precondition violated — grad_output.len() > 0");
        debug_assert!(grad_output.iter().all(|v| v.is_finite()),
            "Contract lora_gradient_size: precondition violated — grad_output.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `sharding`.
/// Domain-specific. Call: `contract_pre_sharding!(slice_expr)`
macro_rules! contract_pre_sharding {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract sharding: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract sharding: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `swiglu_ffn`.
/// Domain-specific. Call: `contract_pre_swiglu_ffn!(slice_expr)`
macro_rules! contract_pre_swiglu_ffn {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract swiglu_ffn: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract swiglu_ffn: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `weighted_loss`.
/// Domain-specific. Call: `contract_pre_weighted_loss!(slice_expr)`
macro_rules! contract_pre_weighted_loss {
    () => {{}};
    ($input:expr) => {{
        let predicted = &$input;
        debug_assert!(predicted.len() > 0,
            "Contract weighted_loss: precondition violated — predicted.len() > 0");
    }};
}

// Auto-generated from contracts/dpo-loss-v1.yaml — DO NOT EDIT
// Contract: dpo-loss-v1

/// Preconditions for equation `dpo_loss`.
/// Domain-specific. Call: `contract_pre_dpo_loss!(slice_expr)`
macro_rules! contract_pre_dpo_loss {
    () => {{}};
    ($input:expr) => {{
        let predicted = &$input;
        debug_assert!(predicted.len() > 0,
            "Contract dpo_loss: precondition violated — predicted.len() > 0");
    }};
}

/// Preconditions for equation `implicit_reward`.
/// Domain-specific. Call: `contract_pre_implicit_reward!(slice_expr)`
macro_rules! contract_pre_implicit_reward {
    () => {{}};
    ($input:expr) => {{
        let predicted = &$input;
        debug_assert!(predicted.len() > 0,
            "Contract implicit_reward: precondition violated — predicted.len() > 0");
    }};
}

/// Preconditions for equation `log_ratio`.
/// Domain-specific. Call: `contract_pre_log_ratio!(slice_expr)`
macro_rules! contract_pre_log_ratio {
    () => {{}};
    ($input:expr) => {{
        let predicted = &$input;
        debug_assert!(predicted.len() > 0,
            "Contract log_ratio: precondition violated — predicted.len() > 0");
    }};
}

// Auto-generated from contracts/drift-detection-v1.yaml — DO NOT EDIT
// Contract: drift-detection-v1

/// Preconditions for equation `classify_drift`.
/// Domain-specific. Call: `contract_pre_classify_drift!(slice_expr)`
macro_rules! contract_pre_classify_drift {
    () => {{}};
    ($input:expr) => {{
        let x = &$input;
    }};
}

/// Preconditions for equation `min_samples_guard`.
/// Domain-specific. Call: `contract_pre_min_samples_guard!(slice_expr)`
macro_rules! contract_pre_min_samples_guard {
    () => {{}};
    ($input:expr) => {{
        let params = &$input;
        debug_assert!(params.len() > 0,
            "Contract min_samples_guard: precondition violated — params.len() > 0");
    }};
}

/// Preconditions for equation `performance_drift`.
/// Domain-specific. Call: `contract_pre_performance_drift!(slice_expr)`
macro_rules! contract_pre_performance_drift {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract performance_drift: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `univariate_drift`.
/// Domain-specific. Call: `contract_pre_univariate_drift!(slice_expr)`
macro_rules! contract_pre_univariate_drift {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract univariate_drift: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/dropout-v1.yaml — DO NOT EDIT
// Contract: dropout-v1

/// Preconditions for equation `dropout_eval`.
/// Domain-specific. Call: `contract_pre_dropout_eval!(slice_expr)`
macro_rules! contract_pre_dropout_eval {
    () => {{}};
    ($input:expr) => {{
        let x = &$input;
        debug_assert!(x.iter().all(|v| v.is_finite()),
            "Contract dropout_eval: precondition violated — x.iter().all(|v| v.is_finite())");
        debug_assert!(x.len() > 0,
            "Contract dropout_eval: precondition violated — x.len() > 0");
    }};
}

/// Preconditions for equation `dropout_train`.
/// Domain-specific. Call: `contract_pre_dropout_train!(slice_expr)`
macro_rules! contract_pre_dropout_train {
    () => {{}};
    ($input:expr) => {{
        let x = &$input;
        debug_assert!(x.iter().all(|v| v.is_finite()),
            "Contract dropout_train: precondition violated — x.iter().all(|v| v.is_finite())");
        debug_assert!(x.len() > 0,
            "Contract dropout_train: precondition violated — x.len() > 0");
    }};
}

// Auto-generated from contracts/embedding-algebra-v1.yaml — DO NOT EDIT
// Contract: embedding-algebra-v1

/// Preconditions for equation `embedding_lookup`.
/// Domain-specific. Call: `contract_pre_embedding_lookup!(slice_expr)`
macro_rules! contract_pre_embedding_lookup {
    () => {{}};
    ($input:expr) => {{
        let indices = &$input;
        debug_assert!(indices.len() > 0,
            "Contract embedding_lookup: precondition violated — indices.len() > 0");
    }};
}

/// Preconditions for equation `embedding_norm`.
/// Domain-specific. Call: `contract_pre_embedding_norm!(slice_expr)`
macro_rules! contract_pre_embedding_norm {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract embedding_norm: precondition violated — input.iter().all(|v| v.is_finite())");
        debug_assert!(input.len() > 0,
            "Contract embedding_norm: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `logit_temperature`.
/// Domain-specific. Call: `contract_pre_logit_temperature!(slice_expr)`
macro_rules! contract_pre_logit_temperature {
    () => {{}};
    ($input:expr) => {{
        let indices = &$input;
        debug_assert!(indices.len() > 0,
            "Contract logit_temperature: precondition violated — indices.len() > 0");
    }};
}

/// Preconditions for equation `tied_weights`.
/// Domain-specific. Call: `contract_pre_tied_weights!(slice_expr)`
macro_rules! contract_pre_tied_weights {
    () => {{}};
    ($input:expr) => {{
        let indices = &$input;
        debug_assert!(indices.len() > 0,
            "Contract tied_weights: precondition violated — indices.len() > 0");
    }};
}

/// Preconditions for equation `unembedding_projection`.
/// Domain-specific. Call: `contract_pre_unembedding_projection!(slice_expr)`
macro_rules! contract_pre_unembedding_projection {
    () => {{}};
    ($input:expr) => {{
        let indices = &$input;
        debug_assert!(indices.len() > 0,
            "Contract unembedding_projection: precondition violated — indices.len() > 0");
    }};
}

/// Preconditions for equation `vocabulary_bounds`.
/// Domain-specific. Call: `contract_pre_vocabulary_bounds!(slice_expr)`
macro_rules! contract_pre_vocabulary_bounds {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract vocabulary_bounds: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/embedding-lookup-v1.yaml — DO NOT EDIT
// Contract: embedding-lookup-v1

/// Preconditions for equation `embedding_lookup`.
/// Domain-specific. Call: `contract_pre_embedding_lookup!(slice_expr)`
macro_rules! contract_pre_embedding_lookup {
    () => {{}};
    ($input:expr) => {{
        let token_ids = &$input;
    }};
}

/// Postconditions for equation `embedding_lookup`.
/// Call before return: `contract_post_embedding_lookup!(result_expr)`
macro_rules! contract_post_embedding_lookup {
    ($result:expr) => {{
        let _contract_result = &$result;
        debug_assert!(_contract_result.iter().all(|v| v.is_finite()), "Contract embedding_lookup: postcondition violated — result.iter().all(|v| v.is_finite())");
    }};
}

/// Combined pre+post contract for equation `embedding_lookup`.
macro_rules! contract_embedding_lookup {
    ($input:expr, $body:expr) => {{
        contract_pre_embedding_lookup!($input);
        let _contract_result = $body;
        contract_post_embedding_lookup!(_contract_result);
        _contract_result
    }};
}

// Auto-generated from contracts/encoder-forward-v1.yaml — DO NOT EDIT
// Contract: encoder-forward-v1

/// Preconditions for equation `cls_pooling`.
/// Domain-specific. Call: `contract_pre_cls_pooling!(slice_expr)`
macro_rules! contract_pre_cls_pooling {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract cls_pooling: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `encoder_layer`.
/// Domain-specific. Call: `contract_pre_encoder_layer!(slice_expr)`
macro_rules! contract_pre_encoder_layer {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract encoder_layer: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/encoder-roundtrip-v1.yaml — DO NOT EDIT
// Contract: encoder-roundtrip-v1

/// Preconditions for equation `emit_posix`.
/// Call at function entry: `contract_pre_emit_posix!(input_expr)`
macro_rules! contract_pre_emit_posix {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(!_contract_input.is_empty(),
            "Contract emit_posix: precondition violated — !input.is_empty()");
    }};
}

/// Preconditions for equation `emit_purified`.
/// Call at function entry: `contract_pre_emit_purified!(input_expr)`
macro_rules! contract_pre_emit_purified {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(!_contract_input.is_empty(),
            "Contract emit_purified: precondition violated — !input.is_empty()");
    }};
}

/// Preconditions for equation `roundtrip`.
/// Call at function entry: `contract_pre_roundtrip!(input_expr)`
macro_rules! contract_pre_roundtrip {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(!_contract_input.is_empty(),
            "Contract roundtrip: precondition violated — !input.is_empty()");
    }};
}

// Auto-generated from contracts/error-handling-v1.yaml — DO NOT EDIT
// Contract: error-handling-v1

/// Preconditions for equation `error_handling`.
/// Domain-specific. Call: `contract_pre_error_handling!(slice_expr)`
macro_rules! contract_pre_error_handling {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract error_handling: precondition violated — input.len() > 0");
    }};
}

/// Postconditions for equation `error_handling`.
/// Call before return: `contract_post_error_handling!(result_expr)`
macro_rules! contract_post_error_handling {
    ($result:expr) => {{
        let _contract_result = &$result;
        debug_assert!(_contract_result.len() > 0, "Contract error_handling: postcondition violated — result.len() > 0");
    }};
}

/// Combined pre+post contract for equation `error_handling`.
macro_rules! contract_error_handling {
    ($input:expr, $body:expr) => {{
        contract_pre_error_handling!($input);
        let _contract_result = $body;
        contract_post_error_handling!(_contract_result);
        _contract_result
    }};
}

// Auto-generated from contracts/execution-safety-v1.yaml — DO NOT EDIT
// Contract: execution-safety-v1

/// Preconditions for equation `atomic_write`.
/// Call at function entry: `contract_pre_atomic_write!(input_expr)`
macro_rules! contract_pre_atomic_write {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `jidoka_stop`.
/// Call at function entry: `contract_pre_jidoka_stop!(input_expr)`
macro_rules! contract_pre_jidoka_stop {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

// Auto-generated from contracts/f16-conversion-v1.yaml — DO NOT EDIT
// Contract: f16-conversion-v1

/// Preconditions for equation `f16_to_f32_bias`.
/// Domain-specific. Call: `contract_pre_f16_to_f32_bias!(slice_expr)`
macro_rules! contract_pre_f16_to_f32_bias {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract f16_to_f32_bias: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `roundtrip`.
/// Domain-specific. Call: `contract_pre_roundtrip!(slice_expr)`
macro_rules! contract_pre_roundtrip {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract roundtrip: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/flash-attention-v1.yaml — DO NOT EDIT
// Contract: flash-attention-v1

/// Preconditions for equation `flash_attention`.
/// Domain-specific. Call: `contract_pre_flash_attention!(slice_expr)`
macro_rules! contract_pre_flash_attention {
    () => {{}};
    ($input:expr) => {{
        let q = &$input;
        debug_assert!(q.len() > 0,
            "Contract flash_attention: precondition violated — q.len() > 0");
    }};
}

// Auto-generated from contracts/format-parity-v1.yaml — DO NOT EDIT
// Contract: format-parity-v1

/// Preconditions for equation `element_count`.
/// Domain-specific. Call: `contract_pre_element_count!(slice_expr)`
macro_rules! contract_pre_element_count {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract element_count: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `identity_1d`.
/// Domain-specific. Call: `contract_pre_identity_1d!(slice_expr)`
macro_rules! contract_pre_identity_1d {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract identity_1d: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract identity_1d: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `name_bijection`.
/// Domain-specific. Call: `contract_pre_name_bijection!(slice_expr)`
macro_rules! contract_pre_name_bijection {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract name_bijection: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `transpose_involution`.
/// Domain-specific. Call: `contract_pre_transpose_involution!(slice_expr)`
macro_rules! contract_pre_transpose_involution {
    () => {{}};
    ($input:expr) => {{
        let a = &$input;
        debug_assert!(a.len() > 0,
            "Contract transpose_involution: precondition violated — a.len() > 0");
    }};
}

// Auto-generated from contracts/fp8-interchange-v1.yaml — DO NOT EDIT
// Contract: fp8-interchange-v1

/// Preconditions for equation `e4m3_encode`.
/// Domain-specific. Call: `contract_pre_e4m3_encode!(slice_expr)`
macro_rules! contract_pre_e4m3_encode {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract e4m3_encode: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `e5m2_encode`.
/// Domain-specific. Call: `contract_pre_e5m2_encode!(slice_expr)`
macro_rules! contract_pre_e5m2_encode {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract e5m2_encode: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `roundtrip`.
/// Domain-specific. Call: `contract_pre_roundtrip!(slice_expr)`
macro_rules! contract_pre_roundtrip {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract roundtrip: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/fused-qkv-projection-v1.yaml — DO NOT EDIT
// Contract: fused-qkv-projection-v1

/// Preconditions for equation `fused_qkv`.
/// Domain-specific. Call: `contract_pre_fused_qkv!(slice_expr)`
macro_rules! contract_pre_fused_qkv {
    () => {{}};
    ($input:expr) => {{
        let a = &$input;
        debug_assert!(a.len() > 0,
            "Contract fused_qkv: precondition violated — a.len() > 0");
    }};
}

/// Preconditions for equation `separate_qkv`.
/// Domain-specific. Call: `contract_pre_separate_qkv!(slice_expr)`
macro_rules! contract_pre_separate_qkv {
    () => {{}};
    ($input:expr) => {{
        let a = &$input;
        debug_assert!(a.len() > 0,
            "Contract separate_qkv: precondition violated — a.len() > 0");
    }};
}

/// Preconditions for equation `shared_q8_qkv`.
/// Domain-specific. Call: `contract_pre_shared_q8_qkv!(slice_expr)`
macro_rules! contract_pre_shared_q8_qkv {
    () => {{}};
    ($input:expr) => {{
        let a = &$input;
        debug_assert!(a.len() > 0,
            "Contract shared_q8_qkv: precondition violated — a.len() > 0");
    }};
}

// Auto-generated from contracts/gated-delta-net-v1.yaml — DO NOT EDIT
// Contract: gated-delta-net-v1

/// Preconditions for equation `decay`.
/// Domain-specific. Call: `contract_pre_decay!(slice_expr)`
macro_rules! contract_pre_decay {
    () => {{}};
    ($input:expr) => {{
        let x = &$input;
        debug_assert!(x.iter().all(|v| v.is_finite()),
            "Contract decay: precondition violated — x.iter().all(|v| v.is_finite())");
        debug_assert!(x.len() > 0,
            "Contract decay: precondition violated — x.len() > 0");
    }};
}

/// Preconditions for equation `delta`.
/// Domain-specific. Call: `contract_pre_delta!(slice_expr)`
macro_rules! contract_pre_delta {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract delta: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract delta: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `output`.
/// Domain-specific. Call: `contract_pre_output!(slice_expr)`
macro_rules! contract_pre_output {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract output: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract output: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `read`.
/// Domain-specific. Call: `contract_pre_read!(slice_expr)`
macro_rules! contract_pre_read {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract read: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract read: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `write`.
/// Domain-specific. Call: `contract_pre_write!(slice_expr)`
macro_rules! contract_pre_write {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract write: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract write: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

// Auto-generated from contracts/gbm-v1.yaml — DO NOT EDIT
// Contract: gbm-v1

/// Preconditions for equation `gradient_boost`.
/// Domain-specific. Call: `contract_pre_gradient_boost!(slice_expr)`
macro_rules! contract_pre_gradient_boost {
    () => {{}};
    ($input:expr) => {{
        let grad_output = &$input;
        debug_assert!(grad_output.len() > 0,
            "Contract gradient_boost: precondition violated — grad_output.len() > 0");
        debug_assert!(grad_output.iter().all(|v| v.is_finite()),
            "Contract gradient_boost: precondition violated — grad_output.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `negative_gradient`.
/// Domain-specific. Call: `contract_pre_negative_gradient!(slice_expr)`
macro_rules! contract_pre_negative_gradient {
    () => {{}};
    ($input:expr) => {{
        let grad_output = &$input;
        debug_assert!(grad_output.len() > 0,
            "Contract negative_gradient: precondition violated — grad_output.len() > 0");
        debug_assert!(grad_output.iter().all(|v| v.is_finite()),
            "Contract negative_gradient: precondition violated — grad_output.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `predict`.
/// Domain-specific. Call: `contract_pre_predict!(slice_expr)`
macro_rules! contract_pre_predict {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract predict: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract predict: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `training_loss`.
/// Domain-specific. Call: `contract_pre_training_loss!(slice_expr)`
macro_rules! contract_pre_training_loss {
    () => {{}};
    ($input:expr) => {{
        let predicted = &$input;
        debug_assert!(predicted.len() > 0,
            "Contract training_loss: precondition violated — predicted.len() > 0");
    }};
}

// Auto-generated from contracts/gelu-kernel-v1.yaml — DO NOT EDIT
// Contract: gelu-kernel-v1

/// Preconditions for equation `gelu`.
/// Domain-specific. Call: `contract_pre_gelu!(slice_expr)`
macro_rules! contract_pre_gelu {
    () => {{}};
    ($input:expr) => {{
        let x = &$input;
        debug_assert!(x.iter().all(|v| v.is_finite()),
            "Contract gelu: precondition violated — x.iter().all(|v| v.is_finite())");
        debug_assert!(x.len() > 0,
            "Contract gelu: precondition violated — x.len() > 0");
    }};
}

/// Preconditions for equation `gelu_tanh_approx`.
/// Domain-specific. Call: `contract_pre_gelu_tanh_approx!(slice_expr)`
macro_rules! contract_pre_gelu_tanh_approx {
    () => {{}};
    ($input:expr) => {{
        let x = &$input;
        debug_assert!(x.iter().all(|v| v.is_finite()),
            "Contract gelu_tanh_approx: precondition violated — x.iter().all(|v| v.is_finite())");
        debug_assert!(x.len() > 0,
            "Contract gelu_tanh_approx: precondition violated — x.len() > 0");
    }};
}

// Auto-generated from contracts/gemm-backward-tiled-v1.yaml — DO NOT EDIT
// Contract: gemm-backward-tiled-v1

/// Preconditions for equation `backward_a_gemm`.
/// Domain-specific. Call: `contract_pre_backward_a_gemm!(slice_expr)`
macro_rules! contract_pre_backward_a_gemm {
    () => {{}};
    ($input:expr) => {{
        let a = &$input;
        debug_assert!(a.len() > 0,
            "Contract backward_a_gemm: precondition violated — a.len() > 0");
    }};
}

/// Preconditions for equation `backward_b_gemm`.
/// Domain-specific. Call: `contract_pre_backward_b_gemm!(slice_expr)`
macro_rules! contract_pre_backward_b_gemm {
    () => {{}};
    ($input:expr) => {{
        let a = &$input;
        debug_assert!(a.len() > 0,
            "Contract backward_b_gemm: precondition violated — a.len() > 0");
    }};
}

/// Preconditions for equation `shared_memory_per_tile`.
/// Domain-specific. Call: `contract_pre_shared_memory_per_tile!(slice_expr)`
macro_rules! contract_pre_shared_memory_per_tile {
    () => {{}};
    ($input:expr) => {{
        let a = &$input;
        debug_assert!(a.len() > 0,
            "Contract shared_memory_per_tile: precondition violated — a.len() > 0");
    }};
}

/// Preconditions for equation `tiled_gemm_arithmetic_intensity`.
/// Domain-specific. Call: `contract_pre_tiled_gemm_arithmetic_intensity!(slice_expr)`
macro_rules! contract_pre_tiled_gemm_arithmetic_intensity {
    () => {{}};
    ($input:expr) => {{
        let a = &$input;
        debug_assert!(a.len() > 0,
            "Contract tiled_gemm_arithmetic_intensity: precondition violated — a.len() > 0");
    }};
}

/// Preconditions for equation `unrolled_instruction_ratio`.
/// Domain-specific. Call: `contract_pre_unrolled_instruction_ratio!(slice_expr)`
macro_rules! contract_pre_unrolled_instruction_ratio {
    () => {{}};
    ($input:expr) => {{
        let a = &$input;
        debug_assert!(a.len() > 0,
            "Contract unrolled_instruction_ratio: precondition violated — a.len() > 0");
    }};
}

// Auto-generated from contracts/gguf-cpu-cache-v1.yaml — DO NOT EDIT
// Contract: gguf-cpu-cache-v1

/// Preconditions for equation `autoregressive_generation`.
/// Call at function entry: `contract_pre_autoregressive_generation!(input_expr)`
macro_rules! contract_pre_autoregressive_generation {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

// Auto-generated from contracts/glm-v1.yaml — DO NOT EDIT
// Contract: glm-v1

/// Preconditions for equation `binomial_link`.
/// Domain-specific. Call: `contract_pre_binomial_link!(slice_expr)`
macro_rules! contract_pre_binomial_link {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract binomial_link: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract binomial_link: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `gamma_link`.
/// Domain-specific. Call: `contract_pre_gamma_link!(slice_expr)`
macro_rules! contract_pre_gamma_link {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract gamma_link: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract gamma_link: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `irls_fit`.
/// Domain-specific. Call: `contract_pre_irls_fit!(slice_expr)`
macro_rules! contract_pre_irls_fit {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract irls_fit: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract irls_fit: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `poisson_link`.
/// Domain-specific. Call: `contract_pre_poisson_link!(slice_expr)`
macro_rules! contract_pre_poisson_link {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract poisson_link: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract poisson_link: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

// Auto-generated from contracts/gnn-v1.yaml — DO NOT EDIT
// Contract: gnn-v1

/// Preconditions for equation `gcn_aggregate`.
/// Call at function entry: `contract_pre_gcn_aggregate!(input_expr)`
macro_rules! contract_pre_gcn_aggregate {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `global_max_pool`.
/// Call at function entry: `contract_pre_global_max_pool!(input_expr)`
macro_rules! contract_pre_global_max_pool {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `global_mean_pool`.
/// Call at function entry: `contract_pre_global_mean_pool!(input_expr)`
macro_rules! contract_pre_global_mean_pool {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `message_passing`.
/// Call at function entry: `contract_pre_message_passing!(input_expr)`
macro_rules! contract_pre_message_passing {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

// Auto-generated from contracts/gpu-context-health-v1.yaml — DO NOT EDIT
// Contract: gpu-context-health-v1

/// Preconditions for equation `context_health`.
/// Domain-specific. Call: `contract_pre_context_health!(slice_expr)`
macro_rules! contract_pre_context_health {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract context_health: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `cuda_graph_guard`.
/// Call at function entry: `contract_pre_cuda_graph_guard!(input_expr)`
macro_rules! contract_pre_cuda_graph_guard {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `culink_skip`.
/// Domain-specific. Call: `contract_pre_culink_skip!(slice_expr)`
macro_rules! contract_pre_culink_skip {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract culink_skip: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `fp8_architecture_guard`.
/// Domain-specific. Call: `contract_pre_fp8_architecture_guard!(slice_expr)`
macro_rules! contract_pre_fp8_architecture_guard {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract fp8_architecture_guard: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/gpu-decode-profiling-v1.yaml — DO NOT EDIT
// Contract: gpu-decode-profiling-v1

/// Preconditions for equation `brick_ordering`.
/// Domain-specific. Call: `contract_pre_brick_ordering!(slice_expr)`
macro_rules! contract_pre_brick_ordering {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract brick_ordering: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `graph_disable`.
/// Domain-specific. Call: `contract_pre_graph_disable!(slice_expr)`
macro_rules! contract_pre_graph_disable {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract graph_disable: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `report_completeness`.
/// Domain-specific. Call: `contract_pre_report_completeness!(slice_expr)`
macro_rules! contract_pre_report_completeness {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract report_completeness: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `report_denominator`.
/// Domain-specific. Call: `contract_pre_report_denominator!(slice_expr)`
macro_rules! contract_pre_report_denominator {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract report_denominator: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `report_fidelity`.
/// Domain-specific. Call: `contract_pre_report_fidelity!(slice_expr)`
macro_rules! contract_pre_report_fidelity {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract report_fidelity: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `report_metadata`.
/// Domain-specific. Call: `contract_pre_report_metadata!(slice_expr)`
macro_rules! contract_pre_report_metadata {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract report_metadata: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `sync_verification`.
/// Domain-specific. Call: `contract_pre_sync_verification!(slice_expr)`
macro_rules! contract_pre_sync_verification {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract sync_verification: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `token_accounting`.
/// Domain-specific. Call: `contract_pre_token_accounting!(slice_expr)`
macro_rules! contract_pre_token_accounting {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract token_accounting: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `wall_coverage`.
/// Domain-specific. Call: `contract_pre_wall_coverage!(slice_expr)`
macro_rules! contract_pre_wall_coverage {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract wall_coverage: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/gpu-multi-backend-parity-v1.yaml — DO NOT EDIT
// Contract: gpu-multi-backend-parity-v1

/// Preconditions for equation `backend_priority`.
/// Domain-specific. Call: `contract_pre_backend_priority!(slice_expr)`
macro_rules! contract_pre_backend_priority {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract backend_priority: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `bandwidth_bound_theorem`.
/// Domain-specific. Call: `contract_pre_bandwidth_bound_theorem!(slice_expr)`
macro_rules! contract_pre_bandwidth_bound_theorem {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract bandwidth_bound_theorem: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `jit_compilation_correctness`.
/// Domain-specific. Call: `contract_pre_jit_compilation_correctness!(slice_expr)`
macro_rules! contract_pre_jit_compilation_correctness {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract jit_compilation_correctness: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `multi_backend_parity`.
/// Domain-specific. Call: `contract_pre_multi_backend_parity!(slice_expr)`
macro_rules! contract_pre_multi_backend_parity {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract multi_backend_parity: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/gpu-weight-residency-v1.yaml — DO NOT EDIT
// Contract: gpu-weight-residency-v1

/// Preconditions for equation `pcie_overhead`.
/// Domain-specific. Call: `contract_pre_pcie_overhead!(slice_expr)`
macro_rules! contract_pre_pcie_overhead {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract pcie_overhead: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `throughput_target`.
/// Domain-specific. Call: `contract_pre_throughput_target!(slice_expr)`
macro_rules! contract_pre_throughput_target {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract throughput_target: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/gqa-kernel-v1.yaml — DO NOT EDIT
// Contract: gqa-kernel-v1

/// Preconditions for equation `gqa`.
/// Domain-specific. Call: `contract_pre_gqa!(slice_expr)`
macro_rules! contract_pre_gqa {
    () => {{}};
    ($input:expr) => {{
        let q = &$input;
        debug_assert!(q.len() > 0,
            "Contract gqa: precondition violated — q.len() > 0");
    }};
}

// Auto-generated from contracts/graph-centrality-v1.yaml — DO NOT EDIT
// Contract: graph-centrality-v1

/// Preconditions for equation `betweenness`.
/// Call at function entry: `contract_pre_betweenness!(input_expr)`
macro_rules! contract_pre_betweenness {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `closeness`.
/// Call at function entry: `contract_pre_closeness!(input_expr)`
macro_rules! contract_pre_closeness {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `degree`.
/// Call at function entry: `contract_pre_degree!(input_expr)`
macro_rules! contract_pre_degree {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `eigenvector`.
/// Call at function entry: `contract_pre_eigenvector!(input_expr)`
macro_rules! contract_pre_eigenvector {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `harmonic`.
/// Call at function entry: `contract_pre_harmonic!(input_expr)`
macro_rules! contract_pre_harmonic {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `katz`.
/// Call at function entry: `contract_pre_katz!(input_expr)`
macro_rules! contract_pre_katz {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

// Auto-generated from contracts/hybrid-layer-dispatch-v1.yaml — DO NOT EDIT
// Contract: hybrid-layer-dispatch-v1

/// Preconditions for equation `conv1d_causal`.
/// Domain-specific. Call: `contract_pre_conv1d_causal!(slice_expr)`
macro_rules! contract_pre_conv1d_causal {
    () => {{}};
    ($input:expr) => {{
        let a = &$input;
        debug_assert!(a.len() > 0,
            "Contract conv1d_causal: precondition violated — a.len() > 0");
    }};
}

/// Preconditions for equation `head_grouping`.
/// Domain-specific. Call: `contract_pre_head_grouping!(slice_expr)`
macro_rules! contract_pre_head_grouping {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract head_grouping: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract head_grouping: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `hybrid_dispatch`.
/// Domain-specific. Call: `contract_pre_hybrid_dispatch!(slice_expr)`
macro_rules! contract_pre_hybrid_dispatch {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract hybrid_dispatch: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract hybrid_dispatch: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `linear_associativity`.
/// Domain-specific. Call: `contract_pre_linear_associativity!(slice_expr)`
macro_rules! contract_pre_linear_associativity {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract linear_associativity: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract linear_associativity: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `linear_no_softmax`.
/// Domain-specific. Call: `contract_pre_linear_no_softmax!(slice_expr)`
macro_rules! contract_pre_linear_no_softmax {
    () => {{}};
    ($input:expr) => {{
        let x = &$input;
        debug_assert!(x.iter().all(|v| v.is_finite()),
            "Contract linear_no_softmax: precondition violated — x.iter().all(|v| v.is_finite())");
        debug_assert!(x.len() > 0,
            "Contract linear_no_softmax: precondition violated — x.len() > 0");
    }};
}

/// Preconditions for equation `linear_shapes`.
/// Domain-specific. Call: `contract_pre_linear_shapes!(slice_expr)`
macro_rules! contract_pre_linear_shapes {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract linear_shapes: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract linear_shapes: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

// Auto-generated from contracts/ica-v1.yaml — DO NOT EDIT
// Contract: ica-v1

/// Preconditions for equation `fastica`.
/// Domain-specific. Call: `contract_pre_fastica!(slice_expr)`
macro_rules! contract_pre_fastica {
    () => {{}};
    ($input:expr) => {{
        let a = &$input;
        debug_assert!(a.len() > 0,
            "Contract fastica: precondition violated — a.len() > 0");
    }};
}

/// Preconditions for equation `mixing`.
/// Domain-specific. Call: `contract_pre_mixing!(slice_expr)`
macro_rules! contract_pre_mixing {
    () => {{}};
    ($input:expr) => {{
        let a = &$input;
        debug_assert!(a.len() > 0,
            "Contract mixing: precondition violated — a.len() > 0");
    }};
}

/// Preconditions for equation `unmixing`.
/// Domain-specific. Call: `contract_pre_unmixing!(slice_expr)`
macro_rules! contract_pre_unmixing {
    () => {{}};
    ($input:expr) => {{
        let a = &$input;
        debug_assert!(a.len() > 0,
            "Contract unmixing: precondition violated — a.len() > 0");
    }};
}

// Auto-generated from contracts/inference-pipeline-v1.yaml — DO NOT EDIT
// Contract: inference-pipeline-v1

/// Preconditions for equation `decode_step`.
/// Domain-specific. Call: `contract_pre_decode_step!(slice_expr)`
macro_rules! contract_pre_decode_step {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract decode_step: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `hybrid_layer_schedule`.
/// Call at function entry: `contract_pre_hybrid_layer_schedule!(input_expr)`
macro_rules! contract_pre_hybrid_layer_schedule {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `kv_cache_growth`.
/// Domain-specific. Call: `contract_pre_kv_cache_growth!(slice_expr)`
macro_rules! contract_pre_kv_cache_growth {
    () => {{}};
    ($input:expr) => {{
        let q = &$input;
        debug_assert!(q.len() > 0,
            "Contract kv_cache_growth: precondition violated — q.len() > 0");
    }};
}

/// Preconditions for equation `layer_composition`.
/// Domain-specific. Call: `contract_pre_layer_composition!(slice_expr)`
macro_rules! contract_pre_layer_composition {
    () => {{}};
    ($input:expr) => {{
        let indices = &$input;
        debug_assert!(indices.len() > 0,
            "Contract layer_composition: precondition violated — indices.len() > 0");
    }};
}

/// Preconditions for equation `prefill_phase`.
/// Call at function entry: `contract_pre_prefill_phase!(input_expr)`
macro_rules! contract_pre_prefill_phase {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `residual_stream`.
/// Call at function entry: `contract_pre_residual_stream!(input_expr)`
macro_rules! contract_pre_residual_stream {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

// Auto-generated from contracts/int8-symmetric-quant-v1.yaml — DO NOT EDIT
// Contract: int8-symmetric-quant-v1

/// Preconditions for equation `dequant_dot`.
/// Domain-specific. Call: `contract_pre_dequant_dot!(slice_expr)`
macro_rules! contract_pre_dequant_dot {
    () => {{}};
    ($input:expr) => {{
        let a = &$input;
        debug_assert!(a.len() > 0,
            "Contract dequant_dot: precondition violated — a.len() > 0");
    }};
}

/// Preconditions for equation `per_row_scale`.
/// Domain-specific. Call: `contract_pre_per_row_scale!(slice_expr)`
macro_rules! contract_pre_per_row_scale {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract per_row_scale: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `quantize`.
/// Domain-specific. Call: `contract_pre_quantize!(slice_expr)`
macro_rules! contract_pre_quantize {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract quantize: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/iterator-v1.yaml — DO NOT EDIT
// Contract: iterator-v1

/// Preconditions for equation `iterator`.
/// Domain-specific. Call: `contract_pre_iterator!(slice_expr)`
macro_rules! contract_pre_iterator {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract iterator: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/kernel-fusion-v1.yaml — DO NOT EDIT
// Contract: kernel-fusion-v1

/// Preconditions for equation `identity`.
/// Domain-specific. Call: `contract_pre_identity!(slice_expr)`
macro_rules! contract_pre_identity {
    () => {{}};
    ($input:expr) => {{
        let q = &$input;
        debug_assert!(q.len() > 0,
            "Contract identity: precondition violated — q.len() > 0");
    }};
}

// Auto-generated from contracts/kernel-launch-budget-v1.yaml — DO NOT EDIT
// Contract: kernel-launch-budget-v1

/// Preconditions for equation `bsum_budget`.
/// Domain-specific. Call: `contract_pre_bsum_budget!(slice_expr)`
macro_rules! contract_pre_bsum_budget {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract bsum_budget: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `per_layer_decomposition`.
/// Domain-specific. Call: `contract_pre_per_layer_decomposition!(slice_expr)`
macro_rules! contract_pre_per_layer_decomposition {
    () => {{}};
    ($input:expr) => {{
        let indices = &$input;
        debug_assert!(indices.len() > 0,
            "Contract per_layer_decomposition: precondition violated — indices.len() > 0");
    }};
}

/// Preconditions for equation `per_token_launches`.
/// Domain-specific. Call: `contract_pre_per_token_launches!(slice_expr)`
macro_rules! contract_pre_per_token_launches {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract per_token_launches: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/kmeans-kernel-v1.yaml — DO NOT EDIT
// Contract: kmeans-kernel-v1

/// Preconditions for equation `assignment`.
/// Domain-specific. Call: `contract_pre_assignment!(slice_expr)`
macro_rules! contract_pre_assignment {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract assignment: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract assignment: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `objective`.
/// Domain-specific. Call: `contract_pre_objective!(slice_expr)`
macro_rules! contract_pre_objective {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract objective: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract objective: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `update`.
/// Domain-specific. Call: `contract_pre_update!(slice_expr)`
macro_rules! contract_pre_update {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract update: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract update: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

// Auto-generated from contracts/kv-cache-equivalence-v1.yaml — DO NOT EDIT
// Contract: kv-cache-equivalence-v1

/// Preconditions for equation `batched_serial_equivalence`.
/// Call at function entry: `contract_pre_batched_serial_equivalence!(input_expr)`
macro_rules! contract_pre_batched_serial_equivalence {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `fused_kernel`.
/// Call at function entry: `contract_pre_fused_kernel!(input_expr)`
macro_rules! contract_pre_fused_kernel {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `page_shape`.
/// Call at function entry: `contract_pre_page_shape!(input_expr)`
macro_rules! contract_pre_page_shape {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `prefill_incremental`.
/// Call at function entry: `contract_pre_prefill_incremental!(input_expr)`
macro_rules! contract_pre_prefill_incremental {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

// Auto-generated from contracts/kv-cache-sizing-v1.yaml — DO NOT EDIT
// Contract: kv-cache-sizing-v1

/// Preconditions for equation `bias_absence`.
/// Call at function entry: `contract_pre_bias_absence!(input_expr)`
macro_rules! contract_pre_bias_absence {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `hybrid_accounting`.
/// Call at function entry: `contract_pre_hybrid_accounting!(input_expr)`
macro_rules! contract_pre_hybrid_accounting {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `per_token_per_layer`.
/// Domain-specific. Call: `contract_pre_per_token_per_layer!(slice_expr)`
macro_rules! contract_pre_per_token_per_layer {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract per_token_per_layer: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `total_kv_memory`.
/// Call at function entry: `contract_pre_total_kv_memory!(input_expr)`
macro_rules! contract_pre_total_kv_memory {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `zero_input_identity`.
/// Call at function entry: `contract_pre_zero_input_identity!(input_expr)`
macro_rules! contract_pre_zero_input_identity {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

// Auto-generated from contracts/layer-parity-v1.yaml — DO NOT EDIT
// Contract: layer-parity-v1

/// Preconditions for equation `identity`.
/// Domain-specific. Call: `contract_pre_identity!(slice_expr)`
macro_rules! contract_pre_identity {
    () => {{}};
    ($input:expr) => {{
        let x = &$input;
    }};
}

// Auto-generated from contracts/layernorm-kernel-v1.yaml — DO NOT EDIT
// Contract: layernorm-kernel-v1

/// Preconditions for equation `layernorm`.
/// Domain-specific. Call: `contract_pre_layernorm!(slice_expr)`
macro_rules! contract_pre_layernorm {
    () => {{}};
    ($input:expr) => {{
        let x = &$input;
        debug_assert!(x.len() > 0,
            "Contract layernorm: precondition violated — x.len() > 0");
        debug_assert!(x.iter().all(|v| v.is_finite()),
            "Contract layernorm: precondition violated — x.iter().all(|v| v.is_finite())");
    }};
}

/// Postconditions for equation `layernorm`.
/// Call before return: `contract_post_layernorm!(result_expr)`
macro_rules! contract_post_layernorm {
    ($result:expr) => {{
        let _contract_result = &$result;
        debug_assert!(_contract_result.iter().all(|v| v.is_finite()), "Contract layernorm: postcondition violated — result.iter().all(|v| v.is_finite())");
    }};
}

/// Combined pre+post contract for equation `layernorm`.
macro_rules! contract_layernorm {
    ($input:expr, $body:expr) => {{
        contract_pre_layernorm!($input);
        let _contract_result = $body;
        contract_post_layernorm!(_contract_result);
        _contract_result
    }};
}

/// Preconditions for equation `statistics`.
/// Domain-specific. Call: `contract_pre_statistics!(slice_expr)`
macro_rules! contract_pre_statistics {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract statistics: precondition violated — input.iter().all(|v| v.is_finite())");
        debug_assert!(input.len() > 0,
            "Contract statistics: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/lbfgs-kernel-v1.yaml — DO NOT EDIT
// Contract: lbfgs-kernel-v1

/// Preconditions for equation `line_search`.
/// Domain-specific. Call: `contract_pre_line_search!(slice_expr)`
macro_rules! contract_pre_line_search {
    () => {{}};
    ($input:expr) => {{
        let params = &$input;
        debug_assert!(params.len() > 0,
            "Contract line_search: precondition violated — params.len() > 0");
    }};
}

/// Preconditions for equation `secant_condition`.
/// Domain-specific. Call: `contract_pre_secant_condition!(slice_expr)`
macro_rules! contract_pre_secant_condition {
    () => {{}};
    ($input:expr) => {{
        let params = &$input;
        debug_assert!(params.len() > 0,
            "Contract secant_condition: precondition violated — params.len() > 0");
    }};
}

/// Preconditions for equation `two_loop_recursion`.
/// Domain-specific. Call: `contract_pre_two_loop_recursion!(slice_expr)`
macro_rules! contract_pre_two_loop_recursion {
    () => {{}};
    ($input:expr) => {{
        let params = &$input;
        debug_assert!(params.len() > 0,
            "Contract two_loop_recursion: precondition violated — params.len() > 0");
    }};
}

// Auto-generated from contracts/learned-position-embedding-v1.yaml — DO NOT EDIT
// Contract: learned-position-embedding-v1

/// Preconditions for equation `position_embedding`.
/// Domain-specific. Call: `contract_pre_position_embedding!(slice_expr)`
macro_rules! contract_pre_position_embedding {
    () => {{}};
    ($input:expr) => {{
        let indices = &$input;
        debug_assert!(indices.len() > 0,
            "Contract position_embedding: precondition violated — indices.len() > 0");
    }};
}

// Auto-generated from contracts/linear-models-v1.yaml — DO NOT EDIT
// Contract: linear-models-v1

/// Preconditions for equation `logistic_predict_proba`.
/// Domain-specific. Call: `contract_pre_logistic_predict_proba!(slice_expr)`
macro_rules! contract_pre_logistic_predict_proba {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract logistic_predict_proba: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract logistic_predict_proba: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `ols_fit`.
/// Domain-specific. Call: `contract_pre_ols_fit!(slice_expr)`
macro_rules! contract_pre_ols_fit {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract ols_fit: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract ols_fit: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `ols_predict`.
/// Domain-specific. Call: `contract_pre_ols_predict!(slice_expr)`
macro_rules! contract_pre_ols_predict {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract ols_predict: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract ols_predict: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `r_squared_training`.
/// Domain-specific. Call: `contract_pre_r_squared_training!(slice_expr)`
macro_rules! contract_pre_r_squared_training {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract r_squared_training: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract r_squared_training: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

// Auto-generated from contracts/linear-probe-classifier-v1.yaml — DO NOT EDIT
// Contract: linear-probe-classifier-v1

/// Preconditions for equation `linear_probe`.
/// Domain-specific. Call: `contract_pre_linear_probe!(slice_expr)`
macro_rules! contract_pre_linear_probe {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract linear_probe: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/linear-projection-v1.yaml — DO NOT EDIT
// Contract: linear-projection-v1

/// Preconditions for equation `linear_forward`.
/// Domain-specific. Call: `contract_pre_linear_forward!(slice_expr)`
macro_rules! contract_pre_linear_forward {
    () => {{}};
    ($input:expr) => {{
        let a = &$input;
        debug_assert!(a.len() > 0,
            "Contract linear_forward: precondition violated — a.len() > 0");
    }};
}

/// Preconditions for equation `linear_no_bias`.
/// Domain-specific. Call: `contract_pre_linear_no_bias!(slice_expr)`
macro_rules! contract_pre_linear_no_bias {
    () => {{}};
    ($input:expr) => {{
        let a = &$input;
        debug_assert!(a.len() > 0,
            "Contract linear_no_bias: precondition violated — a.len() > 0");
    }};
}

// Auto-generated from contracts/lora-algebra-v1.yaml — DO NOT EDIT
// Contract: lora-algebra-v1

/// Preconditions for equation `dare_unbiased`.
/// Domain-specific. Call: `contract_pre_dare_unbiased!(slice_expr)`
macro_rules! contract_pre_dare_unbiased {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract dare_unbiased: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract dare_unbiased: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `eckart_young`.
/// Domain-specific. Call: `contract_pre_eckart_young!(slice_expr)`
macro_rules! contract_pre_eckart_young {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract eckart_young: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract eckart_young: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `lora_shape`.
/// Domain-specific. Call: `contract_pre_lora_shape!(slice_expr)`
macro_rules! contract_pre_lora_shape {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract lora_shape: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract lora_shape: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `shape_preservation`.
/// Domain-specific. Call: `contract_pre_shape_preservation!(slice_expr)`
macro_rules! contract_pre_shape_preservation {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract shape_preservation: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract shape_preservation: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `task_vector`.
/// Domain-specific. Call: `contract_pre_task_vector!(slice_expr)`
macro_rules! contract_pre_task_vector {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract task_vector: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract task_vector: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

// Auto-generated from contracts/lora-gradient-flow-v1.yaml — DO NOT EDIT
// Contract: lora-gradient-flow-v1

/// Preconditions for equation `identity`.
/// Domain-specific. Call: `contract_pre_identity!(slice_expr)`
macro_rules! contract_pre_identity {
    () => {{}};
    ($input:expr) => {{
        let x = &$input;
    }};
}

// Auto-generated from contracts/loss-functions-v1.yaml — DO NOT EDIT
// Contract: loss-functions-v1

/// Preconditions for equation `bce`.
/// Domain-specific. Call: `contract_pre_bce!(slice_expr)`
macro_rules! contract_pre_bce {
    () => {{}};
    ($input:expr) => {{
        let predicted = &$input;
        debug_assert!(predicted.len() > 0,
            "Contract bce: precondition violated — predicted.len() > 0");
    }};
}

/// Preconditions for equation `huber`.
/// Domain-specific. Call: `contract_pre_huber!(slice_expr)`
macro_rules! contract_pre_huber {
    () => {{}};
    ($input:expr) => {{
        let predicted = &$input;
        debug_assert!(predicted.len() > 0,
            "Contract huber: precondition violated — predicted.len() > 0");
    }};
}

/// Preconditions for equation `l1_loss`.
/// Domain-specific. Call: `contract_pre_l1_loss!(slice_expr)`
macro_rules! contract_pre_l1_loss {
    () => {{}};
    ($input:expr) => {{
        let predicted = &$input;
        debug_assert!(predicted.len() > 0,
            "Contract l1_loss: precondition violated — predicted.len() > 0");
    }};
}

/// Preconditions for equation `mse_loss`.
/// Domain-specific. Call: `contract_pre_mse_loss!(slice_expr)`
macro_rules! contract_pre_mse_loss {
    () => {{}};
    ($input:expr) => {{
        let predicted = &$input;
        debug_assert!(predicted.len() > 0,
            "Contract mse_loss: precondition violated — predicted.len() > 0");
    }};
}

/// Preconditions for equation `nll`.
/// Domain-specific. Call: `contract_pre_nll!(slice_expr)`
macro_rules! contract_pre_nll {
    () => {{}};
    ($input:expr) => {{
        let predicted = &$input;
        debug_assert!(predicted.len() > 0,
            "Contract nll: precondition violated — predicted.len() > 0");
    }};
}

/// Preconditions for equation `smooth_l1`.
/// Domain-specific. Call: `contract_pre_smooth_l1!(slice_expr)`
macro_rules! contract_pre_smooth_l1 {
    () => {{}};
    ($input:expr) => {{
        let predicted = &$input;
        debug_assert!(predicted.len() > 0,
            "Contract smooth_l1: precondition violated — predicted.len() > 0");
    }};
}

// Auto-generated from contracts/matmul-kernel-v1.yaml — DO NOT EDIT
// Contract: matmul-kernel-v1

/// Preconditions for equation `matmul`.
/// Domain-specific. Call: `contract_pre_matmul!(slice_expr)`
macro_rules! contract_pre_matmul {
    () => {{}};
    ($input:expr) => {{
        let a = &$input;
    }};
}

/// Postconditions for equation `matmul`.
/// Call before return: `contract_post_matmul!(result_expr)`
macro_rules! contract_post_matmul {
    ($result:expr) => {{
        let _contract_result = &$result;
        debug_assert!(_contract_result.iter().all(|v| v.is_finite()), "Contract matmul: postcondition violated — result.iter().all(|v| v.is_finite())");
    }};
}

/// Combined pre+post contract for equation `matmul`.
macro_rules! contract_matmul {
    ($input:expr, $body:expr) => {{
        contract_pre_matmul!($input);
        let _contract_result = $body;
        contract_post_matmul!(_contract_result);
        _contract_result
    }};
}

/// Preconditions for equation `quantized_dot`.
/// Domain-specific. Call: `contract_pre_quantized_dot!(slice_expr)`
macro_rules! contract_pre_quantized_dot {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract quantized_dot: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/memory-safety-v1.yaml — DO NOT EDIT
// Contract: memory-safety-v1

/// Preconditions for equation `bounds_safety`.
/// Domain-specific. Call: `contract_pre_bounds_safety!(slice_expr)`
macro_rules! contract_pre_bounds_safety {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract bounds_safety: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `drop_safety`.
/// Domain-specific. Call: `contract_pre_drop_safety!(slice_expr)`
macro_rules! contract_pre_drop_safety {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract drop_safety: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `escape_analysis`.
/// Domain-specific. Call: `contract_pre_escape_analysis!(slice_expr)`
macro_rules! contract_pre_escape_analysis {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract escape_analysis: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `lifetime_safety`.
/// Domain-specific. Call: `contract_pre_lifetime_safety!(slice_expr)`
macro_rules! contract_pre_lifetime_safety {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract lifetime_safety: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `ownership_invariant`.
/// Domain-specific. Call: `contract_pre_ownership_invariant!(slice_expr)`
macro_rules! contract_pre_ownership_invariant {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract ownership_invariant: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `use_after_move`.
/// Domain-specific. Call: `contract_pre_use_after_move!(slice_expr)`
macro_rules! contract_pre_use_after_move {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract use_after_move: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/metaheuristics-v1.yaml — DO NOT EDIT
// Contract: metaheuristics-v1

/// Preconditions for equation `best_monotone`.
/// Domain-specific. Call: `contract_pre_best_monotone!(slice_expr)`
macro_rules! contract_pre_best_monotone {
    () => {{}};
    ($input:expr) => {{
        let params = &$input;
        debug_assert!(params.len() > 0,
            "Contract best_monotone: precondition violated — params.len() > 0");
    }};
}

/// Preconditions for equation `ga_crossover`.
/// Domain-specific. Call: `contract_pre_ga_crossover!(slice_expr)`
macro_rules! contract_pre_ga_crossover {
    () => {{}};
    ($input:expr) => {{
        let params = &$input;
        debug_assert!(params.len() > 0,
            "Contract ga_crossover: precondition violated — params.len() > 0");
    }};
}

/// Preconditions for equation `pso_velocity`.
/// Domain-specific. Call: `contract_pre_pso_velocity!(slice_expr)`
macro_rules! contract_pre_pso_velocity {
    () => {{}};
    ($input:expr) => {{
        let params = &$input;
        debug_assert!(params.len() > 0,
            "Contract pso_velocity: precondition violated — params.len() > 0");
    }};
}

/// Preconditions for equation `sa_acceptance`.
/// Domain-specific. Call: `contract_pre_sa_acceptance!(slice_expr)`
macro_rules! contract_pre_sa_acceptance {
    () => {{}};
    ($input:expr) => {{
        let params = &$input;
        debug_assert!(params.len() > 0,
            "Contract sa_acceptance: precondition violated — params.len() > 0");
    }};
}

// Auto-generated from contracts/metrics-classification-v1.yaml — DO NOT EDIT
// Contract: metrics-classification-v1

/// Preconditions for equation `accuracy`.
/// Domain-specific. Call: `contract_pre_accuracy!(slice_expr)`
macro_rules! contract_pre_accuracy {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract accuracy: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract accuracy: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `confusion_matrix`.
/// Domain-specific. Call: `contract_pre_confusion_matrix!(slice_expr)`
macro_rules! contract_pre_confusion_matrix {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract confusion_matrix: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract confusion_matrix: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `f1_score`.
/// Domain-specific. Call: `contract_pre_f1_score!(slice_expr)`
macro_rules! contract_pre_f1_score {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract f1_score: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract f1_score: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `precision`.
/// Domain-specific. Call: `contract_pre_precision!(slice_expr)`
macro_rules! contract_pre_precision {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract precision: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract precision: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `recall`.
/// Domain-specific. Call: `contract_pre_recall!(slice_expr)`
macro_rules! contract_pre_recall {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract recall: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract recall: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

// Auto-generated from contracts/metrics-clustering-v1.yaml — DO NOT EDIT
// Contract: metrics-clustering-v1

/// Preconditions for equation `inertia`.
/// Call at function entry: `contract_pre_inertia!(input_expr)`
macro_rules! contract_pre_inertia {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `silhouette_coefficient`.
/// Call at function entry: `contract_pre_silhouette_coefficient!(input_expr)`
macro_rules! contract_pre_silhouette_coefficient {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `silhouette_score`.
/// Call at function entry: `contract_pre_silhouette_score!(input_expr)`
macro_rules! contract_pre_silhouette_score {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

// Auto-generated from contracts/metrics-ranking-v1.yaml — DO NOT EDIT
// Contract: metrics-ranking-v1

/// Preconditions for equation `hit_at_k`.
/// Domain-specific. Call: `contract_pre_hit_at_k!(slice_expr)`
macro_rules! contract_pre_hit_at_k {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract hit_at_k: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `mrr`.
/// Domain-specific. Call: `contract_pre_mrr!(slice_expr)`
macro_rules! contract_pre_mrr {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract mrr: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `ndcg_at_k`.
/// Domain-specific. Call: `contract_pre_ndcg_at_k!(slice_expr)`
macro_rules! contract_pre_ndcg_at_k {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract ndcg_at_k: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `reciprocal_rank`.
/// Domain-specific. Call: `contract_pre_reciprocal_rank!(slice_expr)`
macro_rules! contract_pre_reciprocal_rank {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract reciprocal_rank: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/metrics-regression-v1.yaml — DO NOT EDIT
// Contract: metrics-regression-v1

/// Preconditions for equation `mae`.
/// Domain-specific. Call: `contract_pre_mae!(slice_expr)`
macro_rules! contract_pre_mae {
    () => {{}};
    ($input:expr) => {{
        let predicted = &$input;
        debug_assert!(predicted.len() > 0,
            "Contract mae: precondition violated — predicted.len() > 0");
    }};
}

/// Preconditions for equation `mse`.
/// Domain-specific. Call: `contract_pre_mse!(slice_expr)`
macro_rules! contract_pre_mse {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract mse: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract mse: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `r_squared`.
/// Domain-specific. Call: `contract_pre_r_squared!(slice_expr)`
macro_rules! contract_pre_r_squared {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract r_squared: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract r_squared: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `rmse`.
/// Domain-specific. Call: `contract_pre_rmse!(slice_expr)`
macro_rules! contract_pre_rmse {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract rmse: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract rmse: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

// Auto-generated from contracts/model-config-algebra-v1.yaml — DO NOT EDIT
// Contract: model-config-algebra-v1

/// Preconditions for equation `bounds`.
/// Domain-specific. Call: `contract_pre_bounds!(slice_expr)`
macro_rules! contract_pre_bounds {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract bounds: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `cross_constraint`.
/// Domain-specific. Call: `contract_pre_cross_constraint!(slice_expr)`
macro_rules! contract_pre_cross_constraint {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract cross_constraint: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `divisibility`.
/// Domain-specific. Call: `contract_pre_divisibility!(slice_expr)`
macro_rules! contract_pre_divisibility {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract divisibility: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `non_degeneracy`.
/// Domain-specific. Call: `contract_pre_non_degeneracy!(slice_expr)`
macro_rules! contract_pre_non_degeneracy {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract non_degeneracy: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `ordering`.
/// Domain-specific. Call: `contract_pre_ordering!(slice_expr)`
macro_rules! contract_pre_ordering {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract ordering: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/model-metadata-bounds-v1.yaml — DO NOT EDIT
// Contract: model-metadata-bounds-v1

/// Preconditions for equation `config_bounds_check`.
/// Domain-specific. Call: `contract_pre_config_bounds_check!(slice_expr)`
macro_rules! contract_pre_config_bounds_check {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract config_bounds_check: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/mqs-scoring-v1.yaml — DO NOT EDIT
// Contract: mqs-scoring-v1

/// Preconditions for equation `mqs_composite`.
/// Domain-specific. Call: `contract_pre_mqs_composite!(slice_expr)`
macro_rules! contract_pre_mqs_composite {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract mqs_composite: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract mqs_composite: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `mqs_deterministic`.
/// Domain-specific. Call: `contract_pre_mqs_deterministic!(slice_expr)`
macro_rules! contract_pre_mqs_deterministic {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract mqs_deterministic: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract mqs_deterministic: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `mqs_grade`.
/// Domain-specific. Call: `contract_pre_mqs_grade!(slice_expr)`
macro_rules! contract_pre_mqs_grade {
    () => {{}};
    ($input:expr) => {{
        let grad_output = &$input;
        debug_assert!(grad_output.len() > 0,
            "Contract mqs_grade: precondition violated — grad_output.len() > 0");
        debug_assert!(grad_output.iter().all(|v| v.is_finite()),
            "Contract mqs_grade: precondition violated — grad_output.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `mqs_pass_rate`.
/// Domain-specific. Call: `contract_pre_mqs_pass_rate!(slice_expr)`
macro_rules! contract_pre_mqs_pass_rate {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract mqs_pass_rate: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract mqs_pass_rate: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

// Auto-generated from contracts/naive-bayes-v1.yaml — DO NOT EDIT
// Contract: naive-bayes-v1

/// Preconditions for equation `class_prior`.
/// Domain-specific. Call: `contract_pre_class_prior!(slice_expr)`
macro_rules! contract_pre_class_prior {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract class_prior: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract class_prior: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `gaussian_likelihood`.
/// Domain-specific. Call: `contract_pre_gaussian_likelihood!(slice_expr)`
macro_rules! contract_pre_gaussian_likelihood {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract gaussian_likelihood: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract gaussian_likelihood: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `log_posterior`.
/// Domain-specific. Call: `contract_pre_log_posterior!(slice_expr)`
macro_rules! contract_pre_log_posterior {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract log_posterior: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract log_posterior: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

// Auto-generated from contracts/online-softmax-v1.yaml — DO NOT EDIT
// Contract: online-softmax-v1

/// Preconditions for equation `online_normalizer`.
/// Domain-specific. Call: `contract_pre_online_normalizer!(slice_expr)`
macro_rules! contract_pre_online_normalizer {
    () => {{}};
    ($input:expr) => {{
        let x = &$input;
        debug_assert!(x.iter().all(|v| v.is_finite()),
            "Contract online_normalizer: precondition violated — x.iter().all(|v| v.is_finite())");
        debug_assert!(x.len() > 0,
            "Contract online_normalizer: precondition violated — x.len() > 0");
    }};
}

/// Preconditions for equation `standard_softmax`.
/// Domain-specific. Call: `contract_pre_standard_softmax!(slice_expr)`
macro_rules! contract_pre_standard_softmax {
    () => {{}};
    ($input:expr) => {{
        let x = &$input;
        debug_assert!(x.iter().all(|v| v.is_finite()),
            "Contract standard_softmax: precondition violated — x.iter().all(|v| v.is_finite())");
        debug_assert!(x.len() > 0,
            "Contract standard_softmax: precondition violated — x.len() > 0");
    }};
}

// Auto-generated from contracts/optimization-v1.yaml — DO NOT EDIT
// Contract: optimization-v1

/// Preconditions for equation `cg_minimize`.
/// Domain-specific. Call: `contract_pre_cg_minimize!(slice_expr)`
macro_rules! contract_pre_cg_minimize {
    () => {{}};
    ($input:expr) => {{
        let params = &$input;
        debug_assert!(params.len() > 0,
            "Contract cg_minimize: precondition violated — params.len() > 0");
    }};
}

/// Preconditions for equation `convergence`.
/// Domain-specific. Call: `contract_pre_convergence!(slice_expr)`
macro_rules! contract_pre_convergence {
    () => {{}};
    ($input:expr) => {{
        let params = &$input;
        debug_assert!(params.len() > 0,
            "Contract convergence: precondition violated — params.len() > 0");
    }};
}

/// Preconditions for equation `line_search`.
/// Domain-specific. Call: `contract_pre_line_search!(slice_expr)`
macro_rules! contract_pre_line_search {
    () => {{}};
    ($input:expr) => {{
        let params = &$input;
        debug_assert!(params.len() > 0,
            "Contract line_search: precondition violated — params.len() > 0");
    }};
}

// Auto-generated from contracts/paged-attention-v1.yaml — DO NOT EDIT
// Contract: paged-attention-v1

/// Preconditions for equation `block_allocation`.
/// Domain-specific. Call: `contract_pre_block_allocation!(slice_expr)`
macro_rules! contract_pre_block_allocation {
    () => {{}};
    ($input:expr) => {{
        let q = &$input;
        debug_assert!(q.len() > 0,
            "Contract block_allocation: precondition violated — q.len() > 0");
    }};
}

/// Preconditions for equation `block_table_lookup`.
/// Domain-specific. Call: `contract_pre_block_table_lookup!(slice_expr)`
macro_rules! contract_pre_block_table_lookup {
    () => {{}};
    ($input:expr) => {{
        let q = &$input;
        debug_assert!(q.len() > 0,
            "Contract block_table_lookup: precondition violated — q.len() > 0");
    }};
}

/// Preconditions for equation `copy_on_write`.
/// Domain-specific. Call: `contract_pre_copy_on_write!(slice_expr)`
macro_rules! contract_pre_copy_on_write {
    () => {{}};
    ($input:expr) => {{
        let q = &$input;
        debug_assert!(q.len() > 0,
            "Contract copy_on_write: precondition violated — q.len() > 0");
    }};
}

// Auto-generated from contracts/paged-kv-cache-v1.yaml — DO NOT EDIT
// Contract: paged-kv-cache-v1

/// Preconditions for equation `block_allocation`.
/// Domain-specific. Call: `contract_pre_block_allocation!(slice_expr)`
macro_rules! contract_pre_block_allocation {
    () => {{}};
    ($input:expr) => {{
        let q = &$input;
        debug_assert!(q.len() > 0,
            "Contract block_allocation: precondition violated — q.len() > 0");
    }};
}

/// Preconditions for equation `block_table_invariant`.
/// Domain-specific. Call: `contract_pre_block_table_invariant!(slice_expr)`
macro_rules! contract_pre_block_table_invariant {
    () => {{}};
    ($input:expr) => {{
        let q = &$input;
        debug_assert!(q.len() > 0,
            "Contract block_table_invariant: precondition violated — q.len() > 0");
    }};
}

/// Preconditions for equation `fragmentation_free`.
/// Domain-specific. Call: `contract_pre_fragmentation_free!(slice_expr)`
macro_rules! contract_pre_fragmentation_free {
    () => {{}};
    ($input:expr) => {{
        let q = &$input;
        debug_assert!(q.len() > 0,
            "Contract fragmentation_free: precondition violated — q.len() > 0");
    }};
}

/// Preconditions for equation `graph_compatibility`.
/// Call at function entry: `contract_pre_graph_compatibility!(input_expr)`
macro_rules! contract_pre_graph_compatibility {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `paged_contiguous_equivalence`.
/// Call at function entry: `contract_pre_paged_contiguous_equivalence!(input_expr)`
macro_rules! contract_pre_paged_contiguous_equivalence {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `slot_mapping`.
/// Domain-specific. Call: `contract_pre_slot_mapping!(slice_expr)`
macro_rules! contract_pre_slot_mapping {
    () => {{}};
    ($input:expr) => {{
        let q = &$input;
        debug_assert!(q.len() > 0,
            "Contract slot_mapping: precondition violated — q.len() > 0");
    }};
}

// Auto-generated from contracts/pagerank-kernel-v1.yaml — DO NOT EDIT
// Contract: pagerank-kernel-v1

/// Preconditions for equation `pagerank`.
/// Call at function entry: `contract_pre_pagerank!(input_expr)`
macro_rules! contract_pre_pagerank {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `power_iteration`.
/// Call at function entry: `contract_pre_power_iteration!(input_expr)`
macro_rules! contract_pre_power_iteration {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

// Auto-generated from contracts/parser-soundness-v1.yaml — DO NOT EDIT
// Contract: parser-soundness-v1

/// Preconditions for equation `lex`.
/// Domain-specific. Call: `contract_pre_lex!(slice_expr)`
macro_rules! contract_pre_lex {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract lex: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `parse`.
/// Domain-specific. Call: `contract_pre_parse!(slice_expr)`
macro_rules! contract_pre_parse {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract parse: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `semantic_analyze`.
/// Domain-specific. Call: `contract_pre_semantic_analyze!(slice_expr)`
macro_rules! contract_pre_semantic_analyze {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract semantic_analyze: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/pca-v1.yaml — DO NOT EDIT
// Contract: pca-v1

/// Preconditions for equation `explained_variance`.
/// Domain-specific. Call: `contract_pre_explained_variance!(slice_expr)`
macro_rules! contract_pre_explained_variance {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract explained_variance: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract explained_variance: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `pca_transform`.
/// Domain-specific. Call: `contract_pre_pca_transform!(slice_expr)`
macro_rules! contract_pre_pca_transform {
    () => {{}};
    ($input:expr) => {{
        let a = &$input;
        debug_assert!(a.len() > 0,
            "Contract pca_transform: precondition violated — a.len() > 0");
    }};
}

/// Preconditions for equation `reconstruction`.
/// Domain-specific. Call: `contract_pre_reconstruction!(slice_expr)`
macro_rules! contract_pre_reconstruction {
    () => {{}};
    ($input:expr) => {{
        let a = &$input;
        debug_assert!(a.len() > 0,
            "Contract reconstruction: precondition violated — a.len() > 0");
    }};
}

// Auto-generated from contracts/performance-grading-v1.yaml — DO NOT EDIT
// Contract: performance-grading-v1

/// Preconditions for equation `concrete_instance`.
/// Domain-specific. Call: `contract_pre_concrete_instance!(slice_expr)`
macro_rules! contract_pre_concrete_instance {
    () => {{}};
    ($input:expr) => {{
        let grad_output = &$input;
        debug_assert!(grad_output.len() > 0,
            "Contract concrete_instance: precondition violated — grad_output.len() > 0");
        debug_assert!(grad_output.iter().all(|v| v.is_finite()),
            "Contract concrete_instance: precondition violated — grad_output.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `efficiency_grade`.
/// Domain-specific. Call: `contract_pre_efficiency_grade!(slice_expr)`
macro_rules! contract_pre_efficiency_grade {
    () => {{}};
    ($input:expr) => {{
        let grad_output = &$input;
        debug_assert!(grad_output.len() > 0,
            "Contract efficiency_grade: precondition violated — grad_output.len() > 0");
        debug_assert!(grad_output.iter().all(|v| v.is_finite()),
            "Contract efficiency_grade: precondition violated — grad_output.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `llamacpp_parity`.
/// Domain-specific. Call: `contract_pre_llamacpp_parity!(slice_expr)`
macro_rules! contract_pre_llamacpp_parity {
    () => {{}};
    ($input:expr) => {{
        let grad_output = &$input;
        debug_assert!(grad_output.len() > 0,
            "Contract llamacpp_parity: precondition violated — grad_output.len() > 0");
        debug_assert!(grad_output.iter().all(|v| v.is_finite()),
            "Contract llamacpp_parity: precondition violated — grad_output.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `ollama_parity`.
/// Domain-specific. Call: `contract_pre_ollama_parity!(slice_expr)`
macro_rules! contract_pre_ollama_parity {
    () => {{}};
    ($input:expr) => {{
        let grad_output = &$input;
        debug_assert!(grad_output.len() > 0,
            "Contract ollama_parity: precondition violated — grad_output.len() > 0");
        debug_assert!(grad_output.iter().all(|v| v.is_finite()),
            "Contract ollama_parity: precondition violated — grad_output.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `vllm_parity`.
/// Domain-specific. Call: `contract_pre_vllm_parity!(slice_expr)`
macro_rules! contract_pre_vllm_parity {
    () => {{}};
    ($input:expr) => {{
        let grad_output = &$input;
        debug_assert!(grad_output.len() > 0,
            "Contract vllm_parity: precondition violated — grad_output.len() > 0");
        debug_assert!(grad_output.iter().all(|v| v.is_finite()),
            "Contract vllm_parity: precondition violated — grad_output.iter().all(|v| v.is_finite())");
    }};
}

// Auto-generated from contracts/pipeline-cache-v1.yaml — DO NOT EDIT
// Contract: pipeline-cache-v1

/// Preconditions for equation `identity`.
/// Call at function entry: `contract_pre_identity!(input_expr)`
macro_rules! contract_pre_identity {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

// Auto-generated from contracts/preprocessing-normalization-v1.yaml — DO NOT EDIT
// Contract: preprocessing-normalization-v1

/// Preconditions for equation `minmax_scaler`.
/// Domain-specific. Call: `contract_pre_minmax_scaler!(slice_expr)`
macro_rules! contract_pre_minmax_scaler {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract minmax_scaler: precondition violated — input.iter().all(|v| v.is_finite())");
        debug_assert!(input.len() > 0,
            "Contract minmax_scaler: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `robust_scaler`.
/// Domain-specific. Call: `contract_pre_robust_scaler!(slice_expr)`
macro_rules! contract_pre_robust_scaler {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract robust_scaler: precondition violated — input.iter().all(|v| v.is_finite())");
        debug_assert!(input.len() > 0,
            "Contract robust_scaler: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `standard_scaler`.
/// Domain-specific. Call: `contract_pre_standard_scaler!(slice_expr)`
macro_rules! contract_pre_standard_scaler {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract standard_scaler: precondition violated — input.iter().all(|v| v.is_finite())");
        debug_assert!(input.len() > 0,
            "Contract standard_scaler: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/ptx-target-parity-v1.yaml — DO NOT EDIT
// Contract: ptx-target-parity-v1

/// Preconditions for equation `jit_compilation_success`.
/// Domain-specific. Call: `contract_pre_jit_compilation_success!(slice_expr)`
macro_rules! contract_pre_jit_compilation_success {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract jit_compilation_success: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract jit_compilation_success: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `no_hardcoded_targets`.
/// Domain-specific. Call: `contract_pre_no_hardcoded_targets!(slice_expr)`
macro_rules! contract_pre_no_hardcoded_targets {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract no_hardcoded_targets: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract no_hardcoded_targets: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `target_parity`.
/// Domain-specific. Call: `contract_pre_target_parity!(slice_expr)`
macro_rules! contract_pre_target_parity {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract target_parity: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract target_parity: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

// Auto-generated from contracts/q4k-q6k-superblock-v1.yaml — DO NOT EDIT
// Contract: q4k-q6k-superblock-v1

/// Preconditions for equation `bsum`.
/// Domain-specific. Call: `contract_pre_bsum!(slice_expr)`
macro_rules! contract_pre_bsum {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract bsum: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `dequantization`.
/// Domain-specific. Call: `contract_pre_dequantization!(slice_expr)`
macro_rules! contract_pre_dequantization {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract dequantization: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `q4k_superblock`.
/// Domain-specific. Call: `contract_pre_q4k_superblock!(slice_expr)`
macro_rules! contract_pre_q4k_superblock {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract q4k_superblock: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `q6k_superblock`.
/// Domain-specific. Call: `contract_pre_q6k_superblock!(slice_expr)`
macro_rules! contract_pre_q6k_superblock {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract q6k_superblock: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `total_bytes`.
/// Domain-specific. Call: `contract_pre_total_bytes!(slice_expr)`
macro_rules! contract_pre_total_bytes {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract total_bytes: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/qk-norm-apr-loader-v1.yaml — DO NOT EDIT
// Contract: qk-norm-apr-loader-v1

/// Preconditions for equation `qk_norm_load`.
/// Domain-specific. Call: `contract_pre_qk_norm_load!(slice_expr)`
macro_rules! contract_pre_qk_norm_load {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract qk_norm_load: precondition violated — input.iter().all(|v| v.is_finite())");
        debug_assert!(input.len() > 0,
            "Contract qk_norm_load: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/qk-norm-v1.yaml — DO NOT EDIT
// Contract: qk-norm-v1

/// Preconditions for equation `qk_rmsnorm`.
/// Domain-specific. Call: `contract_pre_qk_rmsnorm!(slice_expr)`
macro_rules! contract_pre_qk_rmsnorm {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract qk_rmsnorm: precondition violated — input.iter().all(|v| v.is_finite())");
        debug_assert!(input.len() > 0,
            "Contract qk_rmsnorm: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/qlora-hyperparameters-v1.yaml — DO NOT EDIT
// Contract: qlora-hyperparameters-v1

/// Preconditions for equation `effective_batch_size`.
/// Domain-specific. Call: `contract_pre_effective_batch_size!(slice_expr)`
macro_rules! contract_pre_effective_batch_size {
    () => {{}};
    ($input:expr) => {{
        let params = &$input;
        debug_assert!(params.len() > 0,
            "Contract effective_batch_size: precondition violated — params.len() > 0");
    }};
}

/// Preconditions for equation `epoch_count_imbalanced`.
/// Domain-specific. Call: `contract_pre_epoch_count_imbalanced!(slice_expr)`
macro_rules! contract_pre_epoch_count_imbalanced {
    () => {{}};
    ($input:expr) => {{
        let params = &$input;
        debug_assert!(params.len() > 0,
            "Contract epoch_count_imbalanced: precondition violated — params.len() > 0");
    }};
}

/// Preconditions for equation `gradient_clip_bound`.
/// Domain-specific. Call: `contract_pre_gradient_clip_bound!(slice_expr)`
macro_rules! contract_pre_gradient_clip_bound {
    () => {{}};
    ($input:expr) => {{
        let params = &$input;
        debug_assert!(params.len() > 0,
            "Contract gradient_clip_bound: precondition violated — params.len() > 0");
    }};
}

/// Preconditions for equation `learning_rate_scaling`.
/// Domain-specific. Call: `contract_pre_learning_rate_scaling!(slice_expr)`
macro_rules! contract_pre_learning_rate_scaling {
    () => {{}};
    ($input:expr) => {{
        let params = &$input;
        debug_assert!(params.len() > 0,
            "Contract learning_rate_scaling: precondition violated — params.len() > 0");
    }};
}

/// Preconditions for equation `lora_alpha_ratio`.
/// Domain-specific. Call: `contract_pre_lora_alpha_ratio!(slice_expr)`
macro_rules! contract_pre_lora_alpha_ratio {
    () => {{}};
    ($input:expr) => {{
        let params = &$input;
        debug_assert!(params.len() > 0,
            "Contract lora_alpha_ratio: precondition violated — params.len() > 0");
    }};
}

/// Preconditions for equation `seq_len_from_data`.
/// Domain-specific. Call: `contract_pre_seq_len_from_data!(slice_expr)`
macro_rules! contract_pre_seq_len_from_data {
    () => {{}};
    ($input:expr) => {{
        let params = &$input;
        debug_assert!(params.len() > 0,
            "Contract seq_len_from_data: precondition violated — params.len() > 0");
    }};
}

/// Preconditions for equation `warmup_fraction`.
/// Domain-specific. Call: `contract_pre_warmup_fraction!(slice_expr)`
macro_rules! contract_pre_warmup_fraction {
    () => {{}};
    ($input:expr) => {{
        let params = &$input;
        debug_assert!(params.len() > 0,
            "Contract warmup_fraction: precondition violated — params.len() > 0");
    }};
}

// Auto-generated from contracts/quantization-ordering-v1.yaml — DO NOT EDIT
// Contract: quantization-ordering-v1

/// Preconditions for equation `alpha_scaling`.
/// Domain-specific. Call: `contract_pre_alpha_scaling!(slice_expr)`
macro_rules! contract_pre_alpha_scaling {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract alpha_scaling: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `bytes_per_param`.
/// Domain-specific. Call: `contract_pre_bytes_per_param!(slice_expr)`
macro_rules! contract_pre_bytes_per_param {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract bytes_per_param: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `dropout_expectation`.
/// Domain-specific. Call: `contract_pre_dropout_expectation!(slice_expr)`
macro_rules! contract_pre_dropout_expectation {
    () => {{}};
    ($input:expr) => {{
        let x = &$input;
        debug_assert!(x.iter().all(|v| v.is_finite()),
            "Contract dropout_expectation: precondition violated — x.iter().all(|v| v.is_finite())");
        debug_assert!(x.len() > 0,
            "Contract dropout_expectation: precondition violated — x.len() > 0");
    }};
}

/// Preconditions for equation `size_ordering`.
/// Domain-specific. Call: `contract_pre_size_ordering!(slice_expr)`
macro_rules! contract_pre_size_ordering {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract size_ordering: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/quantized-dot-product-v1.yaml — DO NOT EDIT
// Contract: quantized-dot-product-v1

/// Preconditions for equation `identity`.
/// Domain-specific. Call: `contract_pre_identity!(slice_expr)`
macro_rules! contract_pre_identity {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract identity: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/qwen2-e2e-verification-v1.yaml — DO NOT EDIT
// Contract: qwen2-e2e-verification-v1

/// Preconditions for equation `contract_composition`.
/// Domain-specific. Call: `contract_pre_contract_composition!(slice_expr)`
macro_rules! contract_pre_contract_composition {
    () => {{}};
    ($input:expr) => {{
        let indices = &$input;
        debug_assert!(indices.len() > 0,
            "Contract contract_composition: precondition violated — indices.len() > 0");
    }};
}

/// Preconditions for equation `flops_per_token`.
/// Domain-specific. Call: `contract_pre_flops_per_token!(slice_expr)`
macro_rules! contract_pre_flops_per_token {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract flops_per_token: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `memory_breakdown`.
/// Domain-specific. Call: `contract_pre_memory_breakdown!(slice_expr)`
macro_rules! contract_pre_memory_breakdown {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract memory_breakdown: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `model_parameter_count`.
/// Domain-specific. Call: `contract_pre_model_parameter_count!(slice_expr)`
macro_rules! contract_pre_model_parameter_count {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract model_parameter_count: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `throughput_model`.
/// Domain-specific. Call: `contract_pre_throughput_model!(slice_expr)`
macro_rules! contract_pre_throughput_model {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract throughput_model: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `verification_ladder`.
/// Domain-specific. Call: `contract_pre_verification_ladder!(slice_expr)`
macro_rules! contract_pre_verification_ladder {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract verification_ladder: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/qwen2-shapes-v1.yaml — DO NOT EDIT
// Contract: qwen2-shapes-v1

/// Preconditions for equation `head_dim_consistency`.
/// Domain-specific. Call: `contract_pre_head_dim_consistency!(slice_expr)`
macro_rules! contract_pre_head_dim_consistency {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract head_dim_consistency: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `kv_projection_shape`.
/// Domain-specific. Call: `contract_pre_kv_projection_shape!(slice_expr)`
macro_rules! contract_pre_kv_projection_shape {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract kv_projection_shape: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `o_projection_transpose`.
/// Domain-specific. Call: `contract_pre_o_projection_transpose!(slice_expr)`
macro_rules! contract_pre_o_projection_transpose {
    () => {{}};
    ($input:expr) => {{
        let a = &$input;
        debug_assert!(a.len() > 0,
            "Contract o_projection_transpose: precondition violated — a.len() > 0");
    }};
}

/// Preconditions for equation `q_projection_shape`.
/// Domain-specific. Call: `contract_pre_q_projection_shape!(slice_expr)`
macro_rules! contract_pre_q_projection_shape {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract q_projection_shape: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `rope_frequency`.
/// Domain-specific. Call: `contract_pre_rope_frequency!(slice_expr)`
macro_rules! contract_pre_rope_frequency {
    () => {{}};
    ($input:expr) => {{
        let indices = &$input;
        debug_assert!(indices.len() > 0,
            "Contract rope_frequency: precondition violated — indices.len() > 0");
    }};
}

/// Preconditions for equation `swiglu_ratio`.
/// Domain-specific. Call: `contract_pre_swiglu_ratio!(slice_expr)`
macro_rules! contract_pre_swiglu_ratio {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract swiglu_ratio: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/qwen2-weight-loading-v1.yaml — DO NOT EDIT
// Contract: qwen2-weight-loading-v1

/// Preconditions for equation `kv_projection`.
/// Call at function entry: `contract_pre_kv_projection!(input_expr)`
macro_rules! contract_pre_kv_projection {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `q_projection`.
/// Call at function entry: `contract_pre_q_projection!(input_expr)`
macro_rules! contract_pre_q_projection {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `swiglu_expansion`.
/// Call at function entry: `contract_pre_swiglu_expansion!(input_expr)`
macro_rules! contract_pre_swiglu_expansion {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `total_parameters`.
/// Call at function entry: `contract_pre_total_parameters!(input_expr)`
macro_rules! contract_pre_total_parameters {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

// Auto-generated from contracts/qwen3-e2e-verification-v1.yaml — DO NOT EDIT
// Contract: qwen3-e2e-verification-v1

/// Preconditions for equation `contract_composition`.
/// Domain-specific. Call: `contract_pre_contract_composition!(slice_expr)`
macro_rules! contract_pre_contract_composition {
    () => {{}};
    ($input:expr) => {{
        let indices = &$input;
        debug_assert!(indices.len() > 0,
            "Contract contract_composition: precondition violated — indices.len() > 0");
    }};
}

/// Preconditions for equation `flops_per_token`.
/// Domain-specific. Call: `contract_pre_flops_per_token!(slice_expr)`
macro_rules! contract_pre_flops_per_token {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract flops_per_token: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `memory_breakdown`.
/// Domain-specific. Call: `contract_pre_memory_breakdown!(slice_expr)`
macro_rules! contract_pre_memory_breakdown {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract memory_breakdown: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `model_parameter_count`.
/// Domain-specific. Call: `contract_pre_model_parameter_count!(slice_expr)`
macro_rules! contract_pre_model_parameter_count {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract model_parameter_count: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `throughput_model`.
/// Domain-specific. Call: `contract_pre_throughput_model!(slice_expr)`
macro_rules! contract_pre_throughput_model {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract throughput_model: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `verification_ladder`.
/// Domain-specific. Call: `contract_pre_verification_ladder!(slice_expr)`
macro_rules! contract_pre_verification_ladder {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract verification_ladder: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/qwen3-shapes-v1.yaml — DO NOT EDIT
// Contract: qwen3-shapes-v1

/// Preconditions for equation `head_dim_consistency`.
/// Domain-specific. Call: `contract_pre_head_dim_consistency!(slice_expr)`
macro_rules! contract_pre_head_dim_consistency {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract head_dim_consistency: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `kv_projection_shape`.
/// Domain-specific. Call: `contract_pre_kv_projection_shape!(slice_expr)`
macro_rules! contract_pre_kv_projection_shape {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract kv_projection_shape: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `o_projection_transpose`.
/// Domain-specific. Call: `contract_pre_o_projection_transpose!(slice_expr)`
macro_rules! contract_pre_o_projection_transpose {
    () => {{}};
    ($input:expr) => {{
        let a = &$input;
        debug_assert!(a.len() > 0,
            "Contract o_projection_transpose: precondition violated — a.len() > 0");
    }};
}

/// Preconditions for equation `q_projection_shape`.
/// Domain-specific. Call: `contract_pre_q_projection_shape!(slice_expr)`
macro_rules! contract_pre_q_projection_shape {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract q_projection_shape: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `rope_frequency`.
/// Domain-specific. Call: `contract_pre_rope_frequency!(slice_expr)`
macro_rules! contract_pre_rope_frequency {
    () => {{}};
    ($input:expr) => {{
        let indices = &$input;
        debug_assert!(indices.len() > 0,
            "Contract rope_frequency: precondition violated — indices.len() > 0");
    }};
}

/// Preconditions for equation `swiglu_ratio`.
/// Domain-specific. Call: `contract_pre_swiglu_ratio!(slice_expr)`
macro_rules! contract_pre_swiglu_ratio {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract swiglu_ratio: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/qwen35-e2e-verification-v1.yaml — DO NOT EDIT
// Contract: qwen35-e2e-verification-v1

/// Preconditions for equation `contract_composition`.
/// Domain-specific. Call: `contract_pre_contract_composition!(slice_expr)`
macro_rules! contract_pre_contract_composition {
    () => {{}};
    ($input:expr) => {{
        let indices = &$input;
        debug_assert!(indices.len() > 0,
            "Contract contract_composition: precondition violated — indices.len() > 0");
    }};
}

/// Preconditions for equation `flops_per_token`.
/// Domain-specific. Call: `contract_pre_flops_per_token!(slice_expr)`
macro_rules! contract_pre_flops_per_token {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract flops_per_token: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `memory_breakdown`.
/// Domain-specific. Call: `contract_pre_memory_breakdown!(slice_expr)`
macro_rules! contract_pre_memory_breakdown {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract memory_breakdown: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `model_parameter_count`.
/// Domain-specific. Call: `contract_pre_model_parameter_count!(slice_expr)`
macro_rules! contract_pre_model_parameter_count {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract model_parameter_count: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `throughput_model`.
/// Domain-specific. Call: `contract_pre_throughput_model!(slice_expr)`
macro_rules! contract_pre_throughput_model {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract throughput_model: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `verification_ladder`.
/// Domain-specific. Call: `contract_pre_verification_ladder!(slice_expr)`
macro_rules! contract_pre_verification_ladder {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract verification_ladder: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/qwen35-hybrid-forward-v1.yaml — DO NOT EDIT
// Contract: qwen35-hybrid-forward-v1

/// Preconditions for equation `activation_magnitude`.
/// Domain-specific. Call: `contract_pre_activation_magnitude!(slice_expr)`
macro_rules! contract_pre_activation_magnitude {
    () => {{}};
    ($input:expr) => {{
        let x = &$input;
        debug_assert!(x.iter().all(|v| v.is_finite()),
            "Contract activation_magnitude: precondition violated — x.iter().all(|v| v.is_finite())");
        debug_assert!(x.len() > 0,
            "Contract activation_magnitude: precondition violated — x.len() > 0");
    }};
}

/// Preconditions for equation `attention_sublayer`.
/// Domain-specific. Call: `contract_pre_attention_sublayer!(slice_expr)`
macro_rules! contract_pre_attention_sublayer {
    () => {{}};
    ($input:expr) => {{
        let q = &$input;
        debug_assert!(q.len() > 0,
            "Contract attention_sublayer: precondition violated — q.len() > 0");
    }};
}

/// Preconditions for equation `ffn_sublayer`.
/// Domain-specific. Call: `contract_pre_ffn_sublayer!(slice_expr)`
macro_rules! contract_pre_ffn_sublayer {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract ffn_sublayer: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `gdn_sublayer`.
/// Domain-specific. Call: `contract_pre_gdn_sublayer!(slice_expr)`
macro_rules! contract_pre_gdn_sublayer {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract gdn_sublayer: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `gradient_flow`.
/// Domain-specific. Call: `contract_pre_gradient_flow!(slice_expr)`
macro_rules! contract_pre_gradient_flow {
    () => {{}};
    ($input:expr) => {{
        let grad_output = &$input;
        debug_assert!(grad_output.len() > 0,
            "Contract gradient_flow: precondition violated — grad_output.len() > 0");
        debug_assert!(grad_output.iter().all(|v| v.is_finite()),
            "Contract gradient_flow: precondition violated — grad_output.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `hybrid_block`.
/// Domain-specific. Call: `contract_pre_hybrid_block!(slice_expr)`
macro_rules! contract_pre_hybrid_block {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract hybrid_block: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/qwen35-shapes-v1.yaml — DO NOT EDIT
// Contract: qwen35-shapes-v1

/// Preconditions for equation `kv_projection_shape`.
/// Domain-specific. Call: `contract_pre_kv_projection_shape!(slice_expr)`
macro_rules! contract_pre_kv_projection_shape {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract kv_projection_shape: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `o_projection_transpose`.
/// Domain-specific. Call: `contract_pre_o_projection_transpose!(slice_expr)`
macro_rules! contract_pre_o_projection_transpose {
    () => {{}};
    ($input:expr) => {{
        let a = &$input;
        debug_assert!(a.len() > 0,
            "Contract o_projection_transpose: precondition violated — a.len() > 0");
    }};
}

/// Preconditions for equation `q_projection_shape`.
/// Domain-specific. Call: `contract_pre_q_projection_shape!(slice_expr)`
macro_rules! contract_pre_q_projection_shape {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract q_projection_shape: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `rope_frequency`.
/// Domain-specific. Call: `contract_pre_rope_frequency!(slice_expr)`
macro_rules! contract_pre_rope_frequency {
    () => {{}};
    ($input:expr) => {{
        let indices = &$input;
        debug_assert!(indices.len() > 0,
            "Contract rope_frequency: precondition violated — indices.len() > 0");
    }};
}

/// Preconditions for equation `swiglu_ratio`.
/// Domain-specific. Call: `contract_pre_swiglu_ratio!(slice_expr)`
macro_rules! contract_pre_swiglu_ratio {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract swiglu_ratio: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/qwen3moe-e2e-verification-v1.yaml — DO NOT EDIT
// Contract: qwen3moe-e2e-verification-v1

/// Preconditions for equation `active_parameter_count`.
/// Domain-specific. Call: `contract_pre_active_parameter_count!(slice_expr)`
macro_rules! contract_pre_active_parameter_count {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract active_parameter_count: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `contract_composition`.
/// Domain-specific. Call: `contract_pre_contract_composition!(slice_expr)`
macro_rules! contract_pre_contract_composition {
    () => {{}};
    ($input:expr) => {{
        let indices = &$input;
        debug_assert!(indices.len() > 0,
            "Contract contract_composition: precondition violated — indices.len() > 0");
    }};
}

/// Preconditions for equation `flops_per_token`.
/// Domain-specific. Call: `contract_pre_flops_per_token!(slice_expr)`
macro_rules! contract_pre_flops_per_token {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract flops_per_token: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `memory_breakdown`.
/// Domain-specific. Call: `contract_pre_memory_breakdown!(slice_expr)`
macro_rules! contract_pre_memory_breakdown {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract memory_breakdown: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `model_parameter_count`.
/// Domain-specific. Call: `contract_pre_model_parameter_count!(slice_expr)`
macro_rules! contract_pre_model_parameter_count {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract model_parameter_count: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `throughput_model`.
/// Domain-specific. Call: `contract_pre_throughput_model!(slice_expr)`
macro_rules! contract_pre_throughput_model {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract throughput_model: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `verification_ladder`.
/// Domain-specific. Call: `contract_pre_verification_ladder!(slice_expr)`
macro_rules! contract_pre_verification_ladder {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract verification_ladder: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/qwen3moe-shapes-v1.yaml — DO NOT EDIT
// Contract: qwen3moe-shapes-v1

/// Preconditions for equation `kv_projection_shape`.
/// Domain-specific. Call: `contract_pre_kv_projection_shape!(slice_expr)`
macro_rules! contract_pre_kv_projection_shape {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract kv_projection_shape: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `moe_expert_shape`.
/// Domain-specific. Call: `contract_pre_moe_expert_shape!(slice_expr)`
macro_rules! contract_pre_moe_expert_shape {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract moe_expert_shape: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `moe_router_shape`.
/// Domain-specific. Call: `contract_pre_moe_router_shape!(slice_expr)`
macro_rules! contract_pre_moe_router_shape {
    () => {{}};
    ($input:expr) => {{
        let a = &$input;
        debug_assert!(a.len() > 0,
            "Contract moe_router_shape: precondition violated — a.len() > 0");
    }};
}

/// Preconditions for equation `o_projection_transpose`.
/// Domain-specific. Call: `contract_pre_o_projection_transpose!(slice_expr)`
macro_rules! contract_pre_o_projection_transpose {
    () => {{}};
    ($input:expr) => {{
        let a = &$input;
        debug_assert!(a.len() > 0,
            "Contract o_projection_transpose: precondition violated — a.len() > 0");
    }};
}

/// Preconditions for equation `q_projection_shape`.
/// Domain-specific. Call: `contract_pre_q_projection_shape!(slice_expr)`
macro_rules! contract_pre_q_projection_shape {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract q_projection_shape: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `rope_frequency`.
/// Domain-specific. Call: `contract_pre_rope_frequency!(slice_expr)`
macro_rules! contract_pre_rope_frequency {
    () => {{}};
    ($input:expr) => {{
        let indices = &$input;
        debug_assert!(indices.len() > 0,
            "Contract rope_frequency: precondition violated — indices.len() > 0");
    }};
}

/// Preconditions for equation `swiglu_ratio`.
/// Domain-specific. Call: `contract_pre_swiglu_ratio!(slice_expr)`
macro_rules! contract_pre_swiglu_ratio {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract swiglu_ratio: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/random-forest-v1.yaml — DO NOT EDIT
// Contract: random-forest-v1

/// Preconditions for equation `bootstrap_sample`.
/// Domain-specific. Call: `contract_pre_bootstrap_sample!(slice_expr)`
macro_rules! contract_pre_bootstrap_sample {
    () => {{}};
    ($input:expr) => {{
        let params = &$input;
        debug_assert!(params.len() > 0,
            "Contract bootstrap_sample: precondition violated — params.len() > 0");
    }};
}

/// Preconditions for equation `ensemble_size`.
/// Domain-specific. Call: `contract_pre_ensemble_size!(slice_expr)`
macro_rules! contract_pre_ensemble_size {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract ensemble_size: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract ensemble_size: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `majority_vote`.
/// Domain-specific. Call: `contract_pre_majority_vote!(slice_expr)`
macro_rules! contract_pre_majority_vote {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract majority_vote: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract majority_vote: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `predict`.
/// Domain-specific. Call: `contract_pre_predict!(slice_expr)`
macro_rules! contract_pre_predict {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract predict: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract predict: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

// Auto-generated from contracts/recipe-determinism-v1.yaml — DO NOT EDIT
// Contract: recipe-determinism-v1

/// Preconditions for equation `expand_recipe`.
/// Call at function entry: `contract_pre_expand_recipe!(input_expr)`
macro_rules! contract_pre_expand_recipe {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `validate_input_type`.
/// Call at function entry: `contract_pre_validate_input_type!(input_expr)`
macro_rules! contract_pre_validate_input_type {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `validate_inputs`.
/// Domain-specific. Call: `contract_pre_validate_inputs!(slice_expr)`
macro_rules! contract_pre_validate_inputs {
    () => {{}};
    ($input:expr) => {{
        let inputs = &$input;
        debug_assert!(inputs.len() > 0,
            "Contract validate_inputs: precondition violated — inputs.len() > 0");
    }};
}

// Auto-generated from contracts/rmsnorm-kernel-v1.yaml — DO NOT EDIT
// Contract: rmsnorm-kernel-v1

/// Preconditions for equation `rmsnorm`.
/// Domain-specific. Call: `contract_pre_rmsnorm!(slice_expr)`
macro_rules! contract_pre_rmsnorm {
    () => {{}};
    ($input:expr) => {{
        let x = &$input;
        debug_assert!(x.len() > 0,
            "Contract rmsnorm: precondition violated — x.len() > 0");
        debug_assert!(x.iter().all(|v| v.is_finite()),
            "Contract rmsnorm: precondition violated — x.iter().all(|v| v.is_finite())");
    }};
}

/// Postconditions for equation `rmsnorm`.
/// Call before return: `contract_post_rmsnorm!(result_expr)`
macro_rules! contract_post_rmsnorm {
    ($result:expr) => {{
        let _contract_result = &$result;
        debug_assert!(_contract_result.iter().all(|v| v.is_finite()), "Contract rmsnorm: postcondition violated — result.iter().all(|v| v.is_finite())");
    }};
}

/// Combined pre+post contract for equation `rmsnorm`.
macro_rules! contract_rmsnorm {
    ($input:expr, $body:expr) => {{
        contract_pre_rmsnorm!($input);
        let _contract_result = $body;
        contract_post_rmsnorm!(_contract_result);
        _contract_result
    }};
}

// Auto-generated from contracts/roofline-model-v1.yaml — DO NOT EDIT
// Contract: roofline-model-v1

/// Preconditions for equation `bandwidth_ceiling`.
/// Domain-specific. Call: `contract_pre_bandwidth_ceiling!(slice_expr)`
macro_rules! contract_pre_bandwidth_ceiling {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract bandwidth_ceiling: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `compute_ceiling`.
/// Domain-specific. Call: `contract_pre_compute_ceiling!(slice_expr)`
macro_rules! contract_pre_compute_ceiling {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract compute_ceiling: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `model_bytes`.
/// Domain-specific. Call: `contract_pre_model_bytes!(slice_expr)`
macro_rules! contract_pre_model_bytes {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract model_bytes: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `throughput_bound`.
/// Domain-specific. Call: `contract_pre_throughput_bound!(slice_expr)`
macro_rules! contract_pre_throughput_bound {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract throughput_bound: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/rope-extrapolation-v1.yaml — DO NOT EDIT
// Contract: rope-extrapolation-v1

/// Preconditions for equation `base_frequency`.
/// Domain-specific. Call: `contract_pre_base_frequency!(slice_expr)`
macro_rules! contract_pre_base_frequency {
    () => {{}};
    ($input:expr) => {{
        let indices = &$input;
        debug_assert!(indices.len() > 0,
            "Contract base_frequency: precondition violated — indices.len() > 0");
    }};
}

/// Preconditions for equation `linear_interpolation`.
/// Domain-specific. Call: `contract_pre_linear_interpolation!(slice_expr)`
macro_rules! contract_pre_linear_interpolation {
    () => {{}};
    ($input:expr) => {{
        let indices = &$input;
        debug_assert!(indices.len() > 0,
            "Contract linear_interpolation: precondition violated — indices.len() > 0");
    }};
}

/// Preconditions for equation `ntk_scaled_base`.
/// Domain-specific. Call: `contract_pre_ntk_scaled_base!(slice_expr)`
macro_rules! contract_pre_ntk_scaled_base {
    () => {{}};
    ($input:expr) => {{
        let indices = &$input;
        debug_assert!(indices.len() > 0,
            "Contract ntk_scaled_base: precondition violated — indices.len() > 0");
    }};
}

/// Preconditions for equation `rotation_matrix`.
/// Domain-specific. Call: `contract_pre_rotation_matrix!(slice_expr)`
macro_rules! contract_pre_rotation_matrix {
    () => {{}};
    ($input:expr) => {{
        let indices = &$input;
        debug_assert!(indices.len() > 0,
            "Contract rotation_matrix: precondition violated — indices.len() > 0");
    }};
}

/// Preconditions for equation `yarn_mixed_frequency`.
/// Domain-specific. Call: `contract_pre_yarn_mixed_frequency!(slice_expr)`
macro_rules! contract_pre_yarn_mixed_frequency {
    () => {{}};
    ($input:expr) => {{
        let indices = &$input;
        debug_assert!(indices.len() > 0,
            "Contract yarn_mixed_frequency: precondition violated — indices.len() > 0");
    }};
}

/// Preconditions for equation `yarn_ramp`.
/// Domain-specific. Call: `contract_pre_yarn_ramp!(slice_expr)`
macro_rules! contract_pre_yarn_ramp {
    () => {{}};
    ($input:expr) => {{
        let indices = &$input;
        debug_assert!(indices.len() > 0,
            "Contract yarn_ramp: precondition violated — indices.len() > 0");
    }};
}

// Auto-generated from contracts/rope-kernel-v1.yaml — DO NOT EDIT
// Contract: rope-kernel-v1

/// Preconditions for equation `rope`.
/// Domain-specific. Call: `contract_pre_rope!(slice_expr)`
macro_rules! contract_pre_rope {
    () => {{}};
    ($input:expr) => {{
        let x = &$input;
        debug_assert!(x.len() > 0,
            "Contract rope: precondition violated — x.len() > 0");
        debug_assert!(x.len() % 2 == 0,
            "Contract rope: precondition violated — x.len() % 2 == 0");
    }};
}

/// Postconditions for equation `rope`.
/// Call before return: `contract_post_rope!(result_expr)`
macro_rules! contract_post_rope {
    ($result:expr) => {{
        let _contract_result = &$result;
        debug_assert!(_contract_result.iter().all(|v| v.is_finite()), "Contract rope: postcondition violated — result.iter().all(|v| v.is_finite())");
    }};
}

/// Combined pre+post contract for equation `rope`.
macro_rules! contract_rope {
    ($input:expr, $body:expr) => {{
        contract_pre_rope!($input);
        let _contract_result = $body;
        contract_post_rope!(_contract_result);
        _contract_result
    }};
}

// Auto-generated from contracts/safetensors-cpu-dispatch-v1.yaml — DO NOT EDIT
// Contract: safetensors-cpu-dispatch-v1

/// Preconditions for equation `format_parity`.
/// Domain-specific. Call: `contract_pre_format_parity!(slice_expr)`
macro_rules! contract_pre_format_parity {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract format_parity: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/safety-classifier-v1.yaml — DO NOT EDIT
// Contract: safety-classifier-v1

/// Preconditions for equation `classify_filesystem`.
/// Domain-specific. Call: `contract_pre_classify_filesystem!(slice_expr)`
macro_rules! contract_pre_classify_filesystem {
    () => {{}};
    ($input:expr) => {{
        let source = &$input;
        debug_assert!(!source.is_empty(),
            "Contract classify_filesystem: precondition violated — !source.is_empty()");
        debug_assert!(source.len() <= 1_000_000,
            "Contract classify_filesystem: precondition violated — source.len() <= 1_000_000");
    }};
}

/// Preconditions for equation `classify_injection`.
/// Domain-specific. Call: `contract_pre_classify_injection!(slice_expr)`
macro_rules! contract_pre_classify_injection {
    () => {{}};
    ($input:expr) => {{
        let source = &$input;
        debug_assert!(!source.is_empty(),
            "Contract classify_injection: precondition violated — !source.is_empty()");
        debug_assert!(source.len() <= 1_000_000,
            "Contract classify_injection: precondition violated — source.len() <= 1_000_000");
    }};
}

/// Preconditions for equation `classify_secrets`.
/// Domain-specific. Call: `contract_pre_classify_secrets!(slice_expr)`
macro_rules! contract_pre_classify_secrets {
    () => {{}};
    ($input:expr) => {{
        let source = &$input;
        debug_assert!(!source.is_empty(),
            "Contract classify_secrets: precondition violated — !source.is_empty()");
        debug_assert!(source.len() <= 1_000_000,
            "Contract classify_secrets: precondition violated — source.len() <= 1_000_000");
    }};
}

/// Preconditions for equation `lint_shell`.
/// Call at function entry: `contract_pre_lint_shell!(input_expr)`
macro_rules! contract_pre_lint_shell {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

// Auto-generated from contracts/sampling-algorithms-v1.yaml — DO NOT EDIT
// Contract: sampling-algorithms-v1

/// Preconditions for equation `greedy`.
/// Domain-specific. Call: `contract_pre_greedy!(slice_expr)`
macro_rules! contract_pre_greedy {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract greedy: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `temperature`.
/// Domain-specific. Call: `contract_pre_temperature!(slice_expr)`
macro_rules! contract_pre_temperature {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract temperature: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `top_k`.
/// Domain-specific. Call: `contract_pre_top_k!(slice_expr)`
macro_rules! contract_pre_top_k {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract top_k: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `top_p`.
/// Domain-specific. Call: `contract_pre_top_p!(slice_expr)`
macro_rules! contract_pre_top_p {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract top_p: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/score-composite-v1.yaml — DO NOT EDIT
// Contract: score-composite-v1

/// Preconditions for equation `geometric_mean`.
/// Domain-specific. Call: `contract_pre_geometric_mean!(slice_expr)`
macro_rules! contract_pre_geometric_mean {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract geometric_mean: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract geometric_mean: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `grade_from_score`.
/// Domain-specific. Call: `contract_pre_grade_from_score!(slice_expr)`
macro_rules! contract_pre_grade_from_score {
    () => {{}};
    ($input:expr) => {{
        let grad_output = &$input;
        debug_assert!(grad_output.len() > 0,
            "Contract grade_from_score: precondition violated — grad_output.len() > 0");
        debug_assert!(grad_output.iter().all(|v| v.is_finite()),
            "Contract grade_from_score: precondition violated — grad_output.iter().all(|v| v.is_finite())");
    }};
}

// Auto-generated from contracts/semantic-equivalence-v1.yaml — DO NOT EDIT
// Contract: semantic-equivalence-v1

/// Preconditions for equation `comprehension_equivalence`.
/// Domain-specific. Call: `contract_pre_comprehension_equivalence!(slice_expr)`
macro_rules! contract_pre_comprehension_equivalence {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract comprehension_equivalence: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `control_flow_equivalence`.
/// Domain-specific. Call: `contract_pre_control_flow_equivalence!(slice_expr)`
macro_rules! contract_pre_control_flow_equivalence {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract control_flow_equivalence: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `expression_equivalence`.
/// Domain-specific. Call: `contract_pre_expression_equivalence!(slice_expr)`
macro_rules! contract_pre_expression_equivalence {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract expression_equivalence: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `observational_equivalence`.
/// Domain-specific. Call: `contract_pre_observational_equivalence!(slice_expr)`
macro_rules! contract_pre_observational_equivalence {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract observational_equivalence: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `statement_equivalence`.
/// Domain-specific. Call: `contract_pre_statement_equivalence!(slice_expr)`
macro_rules! contract_pre_statement_equivalence {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract statement_equivalence: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/serialization-v1.yaml — DO NOT EDIT
// Contract: serialization-v1

/// Preconditions for equation `serialization`.
/// Domain-specific. Call: `contract_pre_serialization!(slice_expr)`
macro_rules! contract_pre_serialization {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract serialization: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract serialization: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

// Auto-generated from contracts/shannon-entropy-v1.yaml — DO NOT EDIT
// Contract: shannon-entropy-v1

/// Preconditions for equation `entropy`.
/// Domain-specific. Call: `contract_pre_entropy!(slice_expr)`
macro_rules! contract_pre_entropy {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract entropy: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract entropy: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `uniform_entropy`.
/// Domain-specific. Call: `contract_pre_uniform_entropy!(slice_expr)`
macro_rules! contract_pre_uniform_entropy {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract uniform_entropy: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract uniform_entropy: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

// Auto-generated from contracts/silu-kernel-v1.yaml — DO NOT EDIT
// Contract: silu-kernel-v1

/// Preconditions for equation `sigmoid`.
/// Domain-specific. Call: `contract_pre_sigmoid!(slice_expr)`
macro_rules! contract_pre_sigmoid {
    () => {{}};
    ($input:expr) => {{
        let x = &$input;
        debug_assert!(x.iter().all(|v| v.is_finite()),
            "Contract sigmoid: precondition violated — x.iter().all(|v| v.is_finite())");
        debug_assert!(x.len() > 0,
            "Contract sigmoid: precondition violated — x.len() > 0");
    }};
}

/// Preconditions for equation `silu`.
/// Domain-specific. Call: `contract_pre_silu!(slice_expr)`
macro_rules! contract_pre_silu {
    () => {{}};
    ($input:expr) => {{
        let x = &$input;
        debug_assert!(x.iter().all(|v| v.is_finite()),
            "Contract silu: precondition violated — x.iter().all(|v| v.is_finite())");
        debug_assert!(x.len() > 0,
            "Contract silu: precondition violated — x.len() > 0");
    }};
}

// Auto-generated from contracts/sliding-window-attention-v1.yaml — DO NOT EDIT
// Contract: sliding-window-attention-v1

/// Preconditions for equation `attention_sparsity`.
/// Domain-specific. Call: `contract_pre_attention_sparsity!(slice_expr)`
macro_rules! contract_pre_attention_sparsity {
    () => {{}};
    ($input:expr) => {{
        let q = &$input;
        debug_assert!(q.len() > 0,
            "Contract attention_sparsity: precondition violated — q.len() > 0");
    }};
}

/// Preconditions for equation `causal_window_mask`.
/// Domain-specific. Call: `contract_pre_causal_window_mask!(slice_expr)`
macro_rules! contract_pre_causal_window_mask {
    () => {{}};
    ($input:expr) => {{
        let q = &$input;
        debug_assert!(q.len() > 0,
            "Contract causal_window_mask: precondition violated — q.len() > 0");
    }};
}

/// Preconditions for equation `effective_context`.
/// Domain-specific. Call: `contract_pre_effective_context!(slice_expr)`
macro_rules! contract_pre_effective_context {
    () => {{}};
    ($input:expr) => {{
        let q = &$input;
        debug_assert!(q.len() > 0,
            "Contract effective_context: precondition violated — q.len() > 0");
    }};
}

/// Preconditions for equation `multi_layer_receptive_field`.
/// Domain-specific. Call: `contract_pre_multi_layer_receptive_field!(slice_expr)`
macro_rules! contract_pre_multi_layer_receptive_field {
    () => {{}};
    ($input:expr) => {{
        let q = &$input;
        debug_assert!(q.len() > 0,
            "Contract multi_layer_receptive_field: precondition violated — q.len() > 0");
    }};
}

/// Preconditions for equation `window_mask`.
/// Domain-specific. Call: `contract_pre_window_mask!(slice_expr)`
macro_rules! contract_pre_window_mask {
    () => {{}};
    ($input:expr) => {{
        let q = &$input;
        debug_assert!(q.len() > 0,
            "Contract window_mask: precondition violated — q.len() > 0");
    }};
}

// Auto-generated from contracts/softmax-kernel-v1.yaml — DO NOT EDIT
// Contract: softmax-kernel-v1

/// Preconditions for equation `softmax`.
/// Domain-specific. Call: `contract_pre_softmax!(slice_expr)`
macro_rules! contract_pre_softmax {
    () => {{}};
    ($input:expr) => {{
        let x = &$input;
        debug_assert!(x.len() > 0,
            "Contract softmax: precondition violated — x.len() > 0");
        debug_assert!(x.iter().all(|v| v.is_finite()),
            "Contract softmax: precondition violated — x.iter().all(|v| v.is_finite())");
    }};
}

/// Postconditions for equation `softmax`.
/// Call before return: `contract_post_softmax!(result_expr)`
macro_rules! contract_post_softmax {
    ($result:expr) => {{
        let _contract_result = &$result;
        debug_assert!(_contract_result.iter().all(|v| *v > 0.0), "Contract softmax: postcondition violated — result.iter().all(|v| *v > 0.0)");
        debug_assert!((_contract_result.iter().sum::<f32>() - 1.0).abs() < 1e-5, "Contract softmax: postcondition violated — (result.iter().sum::<f32>() - 1.0).abs() < 1e-5");
    }};
}

/// Combined pre+post contract for equation `softmax`.
macro_rules! contract_softmax {
    ($input:expr, $body:expr) => {{
        contract_pre_softmax!($input);
        let _contract_result = $body;
        contract_post_softmax!(_contract_result);
        _contract_result
    }};
}

// Auto-generated from contracts/special-tokens-registry-v1.yaml — DO NOT EDIT
// Contract: special-tokens-registry-v1

/// Preconditions for equation `token_bounds`.
/// Domain-specific. Call: `contract_pre_token_bounds!(slice_expr)`
macro_rules! contract_pre_token_bounds {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract token_bounds: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/speculative-decoding-v1.yaml — DO NOT EDIT
// Contract: speculative-decoding-v1

/// Preconditions for equation `acceptance_probability`.
/// Domain-specific. Call: `contract_pre_acceptance_probability!(slice_expr)`
macro_rules! contract_pre_acceptance_probability {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract acceptance_probability: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `output_equivalence`.
/// Domain-specific. Call: `contract_pre_output_equivalence!(slice_expr)`
macro_rules! contract_pre_output_equivalence {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract output_equivalence: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `token_acceptance`.
/// Domain-specific. Call: `contract_pre_token_acceptance!(slice_expr)`
macro_rules! contract_pre_token_acceptance {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract token_acceptance: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/ssm-kernel-v1.yaml — DO NOT EDIT
// Contract: ssm-kernel-v1

/// Preconditions for equation `selective_gate`.
/// Domain-specific. Call: `contract_pre_selective_gate!(slice_expr)`
macro_rules! contract_pre_selective_gate {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract selective_gate: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract selective_gate: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `ssm_discretize`.
/// Domain-specific. Call: `contract_pre_ssm_discretize!(slice_expr)`
macro_rules! contract_pre_ssm_discretize {
    () => {{}};
    ($input:expr) => {{
        let x = &$input;
        debug_assert!(x.iter().all(|v| v.is_finite()),
            "Contract ssm_discretize: precondition violated — x.iter().all(|v| v.is_finite())");
        debug_assert!(x.len() > 0,
            "Contract ssm_discretize: precondition violated — x.len() > 0");
    }};
}

/// Preconditions for equation `ssm_scan`.
/// Domain-specific. Call: `contract_pre_ssm_scan!(slice_expr)`
macro_rules! contract_pre_ssm_scan {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract ssm_scan: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract ssm_scan: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

// Auto-generated from contracts/streaming-tpot-v1.yaml — DO NOT EDIT
// Contract: streaming-tpot-v1

/// Preconditions for equation `tpot_definition`.
/// Domain-specific. Call: `contract_pre_tpot_definition!(slice_expr)`
macro_rules! contract_pre_tpot_definition {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract tpot_definition: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract tpot_definition: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

// Auto-generated from contracts/svm-v1.yaml — DO NOT EDIT
// Contract: svm-v1

/// Preconditions for equation `decision_function`.
/// Domain-specific. Call: `contract_pre_decision_function!(slice_expr)`
macro_rules! contract_pre_decision_function {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract decision_function: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract decision_function: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `hinge_loss`.
/// Domain-specific. Call: `contract_pre_hinge_loss!(slice_expr)`
macro_rules! contract_pre_hinge_loss {
    () => {{}};
    ($input:expr) => {{
        let predicted = &$input;
        debug_assert!(predicted.len() > 0,
            "Contract hinge_loss: precondition violated — predicted.len() > 0");
    }};
}

/// Preconditions for equation `margin`.
/// Domain-specific. Call: `contract_pre_margin!(slice_expr)`
macro_rules! contract_pre_margin {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract margin: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract margin: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `svm_predict`.
/// Domain-specific. Call: `contract_pre_svm_predict!(slice_expr)`
macro_rules! contract_pre_svm_predict {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract svm_predict: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract svm_predict: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

// Auto-generated from contracts/swiglu-kernel-v1.yaml — DO NOT EDIT
// Contract: swiglu-kernel-v1

/// Preconditions for equation `silu`.
/// Domain-specific. Call: `contract_pre_silu!(slice_expr)`
macro_rules! contract_pre_silu {
    () => {{}};
    ($input:expr) => {{
        let x = &$input;
        debug_assert!(x.iter().all(|v| v.is_finite()),
            "Contract silu: precondition violated — x.iter().all(|v| v.is_finite())");
        debug_assert!(x.len() > 0,
            "Contract silu: precondition violated — x.len() > 0");
    }};
}

/// Preconditions for equation `swiglu`.
/// Domain-specific. Call: `contract_pre_swiglu!(slice_expr)`
macro_rules! contract_pre_swiglu {
    () => {{}};
    ($input:expr) => {{
        let x = &$input;
        debug_assert!(x.len() > 0,
            "Contract swiglu: precondition violated — x.len() > 0");
        debug_assert!(x.iter().all(|v| v.is_finite()),
            "Contract swiglu: precondition violated — x.iter().all(|v| v.is_finite())");
    }};
}

/// Postconditions for equation `swiglu`.
/// Call before return: `contract_post_swiglu!(result_expr)`
macro_rules! contract_post_swiglu {
    ($result:expr) => {{
        let _contract_result = &$result;
        debug_assert!(_contract_result.iter().all(|v| v.is_finite()), "Contract swiglu: postcondition violated — result.iter().all(|v| v.is_finite())");
    }};
}

/// Combined pre+post contract for equation `swiglu`.
macro_rules! contract_swiglu {
    ($input:expr, $body:expr) => {{
        contract_pre_swiglu!($input);
        let _contract_result = $body;
        contract_post_swiglu!(_contract_result);
        _contract_result
    }};
}

// Auto-generated from contracts/tdg-scoring-v1.yaml — DO NOT EDIT
// Contract: tdg-scoring-v1

/// Preconditions for equation `calculate_tdg`.
/// Call at function entry: `contract_pre_calculate_tdg!(input_expr)`
macro_rules! contract_pre_calculate_tdg {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Preconditions for equation `letter_grade`.
/// Domain-specific. Call: `contract_pre_letter_grade!(slice_expr)`
macro_rules! contract_pre_letter_grade {
    () => {{}};
    ($input:expr) => {{
        let grad_output = &$input;
        debug_assert!(grad_output.len() > 0,
            "Contract letter_grade: precondition violated — grad_output.len() > 0");
        debug_assert!(grad_output.iter().all(|v| v.is_finite()),
            "Contract letter_grade: precondition violated — grad_output.iter().all(|v| v.is_finite())");
    }};
}

// Auto-generated from contracts/tensor-inventory-v1.yaml — DO NOT EDIT
// Contract: tensor-inventory-v1

/// Preconditions for equation `architecture_delta`.
/// Domain-specific. Call: `contract_pre_architecture_delta!(slice_expr)`
macro_rules! contract_pre_architecture_delta {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract architecture_delta: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract architecture_delta: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `parameter_decomposition`.
/// Domain-specific. Call: `contract_pre_parameter_decomposition!(slice_expr)`
macro_rules! contract_pre_parameter_decomposition {
    () => {{}};
    ($input:expr) => {{
        let indices = &$input;
        debug_assert!(indices.len() > 0,
            "Contract parameter_decomposition: precondition violated — indices.len() > 0");
    }};
}

/// Preconditions for equation `quantization_bytes`.
/// Domain-specific. Call: `contract_pre_quantization_bytes!(slice_expr)`
macro_rules! contract_pre_quantization_bytes {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract quantization_bytes: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `tensor_count`.
/// Domain-specific. Call: `contract_pre_tensor_count!(slice_expr)`
macro_rules! contract_pre_tensor_count {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract tensor_count: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract tensor_count: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `tied_embeddings`.
/// Domain-specific. Call: `contract_pre_tied_embeddings!(slice_expr)`
macro_rules! contract_pre_tied_embeddings {
    () => {{}};
    ($input:expr) => {{
        let indices = &$input;
        debug_assert!(indices.len() > 0,
            "Contract tied_embeddings: precondition violated — indices.len() > 0");
    }};
}

// Auto-generated from contracts/tensor-layout-v1.yaml — DO NOT EDIT
// Contract: tensor-layout-v1

/// Preconditions for equation `identity`.
/// Domain-specific. Call: `contract_pre_identity!(slice_expr)`
macro_rules! contract_pre_identity {
    () => {{}};
    ($input:expr) => {{
        let a = &$input;
        debug_assert!(a.len() > 0,
            "Contract identity: precondition violated — a.len() > 0");
    }};
}

// Auto-generated from contracts/tensor-names-v1.yaml — DO NOT EDIT
// Contract: tensor-names-v1

/// Preconditions for equation `architecture_normalization`.
/// Domain-specific. Call: `contract_pre_architecture_normalization!(slice_expr)`
macro_rules! contract_pre_architecture_normalization {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract architecture_normalization: precondition violated — input.iter().all(|v| v.is_finite())");
        debug_assert!(input.len() > 0,
            "Contract architecture_normalization: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `name_resolution`.
/// Domain-specific. Call: `contract_pre_name_resolution!(slice_expr)`
macro_rules! contract_pre_name_resolution {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract name_resolution: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract name_resolution: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

// Auto-generated from contracts/tensor-rc-data-v1.yaml — DO NOT EDIT
// Contract: tensor-rc-data-v1

/// Preconditions for equation `identity`.
/// Domain-specific. Call: `contract_pre_identity!(slice_expr)`
macro_rules! contract_pre_identity {
    () => {{}};
    ($input:expr) => {{
        let a = &$input;
        debug_assert!(a.len() > 0,
            "Contract identity: precondition violated — a.len() > 0");
    }};
}

// Auto-generated from contracts/tensor-shape-flow-v1.yaml — DO NOT EDIT
// Contract: tensor-shape-flow-v1

/// Preconditions for equation `gqa_grouping`.
/// Domain-specific. Call: `contract_pre_gqa_grouping!(slice_expr)`
macro_rules! contract_pre_gqa_grouping {
    () => {{}};
    ($input:expr) => {{
        let q = &$input;
        debug_assert!(q.len() > 0,
            "Contract gqa_grouping: precondition violated — q.len() > 0");
    }};
}

/// Preconditions for equation `lm_head`.
/// Domain-specific. Call: `contract_pre_lm_head!(slice_expr)`
macro_rules! contract_pre_lm_head {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract lm_head: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract lm_head: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `qkv_projection`.
/// Domain-specific. Call: `contract_pre_qkv_projection!(slice_expr)`
macro_rules! contract_pre_qkv_projection {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract qkv_projection: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract qkv_projection: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `residual`.
/// Domain-specific. Call: `contract_pre_residual!(slice_expr)`
macro_rules! contract_pre_residual {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract residual: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract residual: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `swiglu_shape`.
/// Domain-specific. Call: `contract_pre_swiglu_shape!(slice_expr)`
macro_rules! contract_pre_swiglu_shape {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract swiglu_shape: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract swiglu_shape: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

// Auto-generated from contracts/tied-embeddings-v1.yaml — DO NOT EDIT
// Contract: tied-embeddings-v1

/// Preconditions for equation `tied_lm_head`.
/// Domain-specific. Call: `contract_pre_tied_lm_head!(slice_expr)`
macro_rules! contract_pre_tied_lm_head {
    () => {{}};
    ($input:expr) => {{
        let indices = &$input;
        debug_assert!(indices.len() > 0,
            "Contract tied_lm_head: precondition violated — indices.len() > 0");
    }};
}

// Auto-generated from contracts/tiled-matmul-shader-v1.yaml — DO NOT EDIT
// Contract: tiled-matmul-shader-v1

/// Preconditions for equation `identity`.
/// Domain-specific. Call: `contract_pre_identity!(slice_expr)`
macro_rules! contract_pre_identity {
    () => {{}};
    ($input:expr) => {{
        let a = &$input;
        debug_assert!(a.len() > 0,
            "Contract identity: precondition violated — a.len() > 0");
    }};
}

// Auto-generated from contracts/tokenizer-loading-v1.yaml — DO NOT EDIT
// Contract: tokenizer-loading-v1

/// Preconditions for equation `identity`.
/// Call at function entry: `contract_pre_identity!(input_expr)`
macro_rules! contract_pre_identity {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(!_contract_input.is_empty(),
            "Contract identity: precondition violated — !input.is_empty()");
    }};
}

// Auto-generated from contracts/training-loop-v1.yaml — DO NOT EDIT
// Contract: training-loop-v1

/// Preconditions for equation `ema_loss`.
/// Domain-specific. Call: `contract_pre_ema_loss!(slice_expr)`
macro_rules! contract_pre_ema_loss {
    () => {{}};
    ($input:expr) => {{
        let predicted = &$input;
        debug_assert!(predicted.len() > 0,
            "Contract ema_loss: precondition violated — predicted.len() > 0");
    }};
}

/// Preconditions for equation `val_split`.
/// Domain-specific. Call: `contract_pre_val_split!(slice_expr)`
macro_rules! contract_pre_val_split {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract val_split: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract val_split: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `warmup_lr`.
/// Domain-specific. Call: `contract_pre_warmup_lr!(slice_expr)`
macro_rules! contract_pre_warmup_lr {
    () => {{}};
    ($input:expr) => {{
        let params = &$input;
        debug_assert!(params.len() > 0,
            "Contract warmup_lr: precondition violated — params.len() > 0");
    }};
}

// Auto-generated from contracts/transpose-kernel-v1.yaml — DO NOT EDIT
// Contract: transpose-kernel-v1

/// Preconditions for equation `transpose`.
/// Domain-specific. Call: `contract_pre_transpose!(slice_expr)`
macro_rules! contract_pre_transpose {
    () => {{}};
    ($input:expr) => {{
        let a = &$input;
        debug_assert!(a.len() > 0,
            "Contract transpose: precondition violated — a.len() > 0");
    }};
}

// Auto-generated from contracts/type-preservation-v1.yaml — DO NOT EDIT
// Contract: type-preservation-v1

/// Preconditions for equation `container_preservation`.
/// Domain-specific. Call: `contract_pre_container_preservation!(slice_expr)`
macro_rules! contract_pre_container_preservation {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract container_preservation: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `copy_semantics`.
/// Domain-specific. Call: `contract_pre_copy_semantics!(slice_expr)`
macro_rules! contract_pre_copy_semantics {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract copy_semantics: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `numeric_semantics`.
/// Domain-specific. Call: `contract_pre_numeric_semantics!(slice_expr)`
macro_rules! contract_pre_numeric_semantics {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract numeric_semantics: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `type_inference`.
/// Domain-specific. Call: `contract_pre_type_inference!(slice_expr)`
macro_rules! contract_pre_type_inference {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract type_inference: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `type_map`.
/// Domain-specific. Call: `contract_pre_type_map!(slice_expr)`
macro_rules! contract_pre_type_map {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract type_map: precondition violated — input.len() > 0");
    }};
}

// Auto-generated from contracts/validated-tensor-v1.yaml — DO NOT EDIT
// Contract: validated-tensor-v1

/// Preconditions for equation `density_gate`.
/// Domain-specific. Call: `contract_pre_density_gate!(slice_expr)`
macro_rules! contract_pre_density_gate {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract density_gate: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract density_gate: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

/// Preconditions for equation `l2_norm_nondegeneracy`.
/// Domain-specific. Call: `contract_pre_l2_norm_nondegeneracy!(slice_expr)`
macro_rules! contract_pre_l2_norm_nondegeneracy {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract l2_norm_nondegeneracy: precondition violated — input.iter().all(|v| v.is_finite())");
        debug_assert!(input.len() > 0,
            "Contract l2_norm_nondegeneracy: precondition violated — input.len() > 0");
    }};
}

/// Preconditions for equation `nan_inf_rejection`.
/// Domain-specific. Call: `contract_pre_nan_inf_rejection!(slice_expr)`
macro_rules! contract_pre_nan_inf_rejection {
    () => {{}};
    ($input:expr) => {{
        let input = &$input;
        debug_assert!(input.len() > 0,
            "Contract nan_inf_rejection: precondition violated — input.len() > 0");
        debug_assert!(input.iter().all(|v| v.is_finite()),
            "Contract nan_inf_rejection: precondition violated — input.iter().all(|v| v.is_finite())");
    }};
}

// Auto-generated from contracts/wgpu-resident-weights-v1.yaml — DO NOT EDIT
// Contract: wgpu-resident-weights-v1

/// Preconditions for equation `identity`.
/// Domain-specific. Call: `contract_pre_identity!(slice_expr)`
macro_rules! contract_pre_identity {
    () => {{}};
    ($input:expr) => {{
        let x = &$input;
    }};
}

// Total: 608 preconditions, 15 postconditions from 165 contracts
