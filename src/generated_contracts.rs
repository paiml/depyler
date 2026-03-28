// Auto-generated contract assertions from YAML — DO NOT EDIT.
// Zero cost in release builds (debug_assert!).
// Regenerate: pv codegen contracts/ -o src/generated_contracts.rs
// Include:   #[macro_use] #[allow(unused_macros)] mod generated_contracts;

// Auto-generated from contracts/absolute-position-v1.yaml — DO NOT EDIT
// Contract: absolute-position-v1

/// Preconditions for equation `absolute_position_add`.
/// Call at function entry: `contract_pre_absolute_position_add!(input_expr)`
macro_rules! contract_pre_absolute_position_add {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract absolute_position_add: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/activation-kernel-v1.yaml — DO NOT EDIT
// Contract: activation-kernel-v1

/// Preconditions for equation `gelu`.
/// Call at function entry: `contract_pre_gelu!(input_expr)`
macro_rules! contract_pre_gelu {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract gelu: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `relu`.
/// Call at function entry: `contract_pre_relu!(input_expr)`
macro_rules! contract_pre_relu {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract relu: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `silu`.
/// Call at function entry: `contract_pre_silu!(input_expr)`
macro_rules! contract_pre_silu {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract silu: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/active-learning-v1.yaml — DO NOT EDIT
// Contract: active-learning-v1

/// Preconditions for equation `entropy_score`.
/// Call at function entry: `contract_pre_entropy_score!(input_expr)`
macro_rules! contract_pre_entropy_score {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract entropy_score: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `margin_score`.
/// Call at function entry: `contract_pre_margin_score!(input_expr)`
macro_rules! contract_pre_margin_score {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract margin_score: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `qbc_score`.
/// Call at function entry: `contract_pre_qbc_score!(input_expr)`
macro_rules! contract_pre_qbc_score {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract qbc_score: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `uncertainty_score`.
/// Call at function entry: `contract_pre_uncertainty_score!(input_expr)`
macro_rules! contract_pre_uncertainty_score {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract uncertainty_score: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/adamw-kernel-v1.yaml — DO NOT EDIT
// Contract: adamw-kernel-v1

/// Preconditions for equation `adam_moments`.
/// Call at function entry: `contract_pre_adam_moments!(input_expr)`
macro_rules! contract_pre_adam_moments {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract adam_moments: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `adam_variance`.
/// Call at function entry: `contract_pre_adam_variance!(input_expr)`
macro_rules! contract_pre_adam_variance {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract adam_variance: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `bias_correction`.
/// Call at function entry: `contract_pre_bias_correction!(input_expr)`
macro_rules! contract_pre_bias_correction {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract bias_correction: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `weight_update`.
/// Call at function entry: `contract_pre_weight_update!(input_expr)`
macro_rules! contract_pre_weight_update {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract weight_update: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/alibi-kernel-v1.yaml — DO NOT EDIT
// Contract: alibi-kernel-v1

/// Preconditions for equation `alibi_bias`.
/// Call at function entry: `contract_pre_alibi_bias!(input_expr)`
macro_rules! contract_pre_alibi_bias {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract alibi_bias: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `alibi_slopes`.
/// Call at function entry: `contract_pre_alibi_slopes!(input_expr)`
macro_rules! contract_pre_alibi_slopes {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract alibi_slopes: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/apr-format-invariants-v1.yaml — DO NOT EDIT
// Contract: apr-format-invariants-v1

/// Preconditions for equation `detect_regression`.
/// Call at function entry: `contract_pre_detect_regression!(input_expr)`
macro_rules! contract_pre_detect_regression {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract detect_regression: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `format_report`.
/// Call at function entry: `contract_pre_format_report!(input_expr)`
macro_rules! contract_pre_format_report {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract format_report: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `parse_playbook`.
/// Call at function entry: `contract_pre_parse_playbook!(input_expr)`
macro_rules! contract_pre_parse_playbook {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract parse_playbook: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `serialize_roundtrip`.
/// Call at function entry: `contract_pre_serialize_roundtrip!(input_expr)`
macro_rules! contract_pre_serialize_roundtrip {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract serialize_roundtrip: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `validate_schema`.
/// Call at function entry: `contract_pre_validate_schema!(input_expr)`
macro_rules! contract_pre_validate_schema {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract validate_schema: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/arch-constraints-v1.yaml — DO NOT EDIT
// Contract: arch-constraints-v1

/// Preconditions for equation `arch_constraint_lookup`.
/// Call at function entry: `contract_pre_arch_constraint_lookup!(input_expr)`
macro_rules! contract_pre_arch_constraint_lookup {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract arch_constraint_lookup: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/architecture-requirements-v1.yaml — DO NOT EDIT
// Contract: architecture-requirements-v1

/// Preconditions for equation `constraint_matrix_exhaustiveness`.
/// Call at function entry: `contract_pre_constraint_matrix_exhaustiveness!(input_expr)`
macro_rules! contract_pre_constraint_matrix_exhaustiveness {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(!_contract_input.is_empty(),
            "Contract constraint_matrix_exhaustiveness: precondition violated — input must not be empty");
    }};
}

/// Preconditions for equation `role_mapping`.
/// Call at function entry: `contract_pre_role_mapping!(input_expr)`
macro_rules! contract_pre_role_mapping {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract role_mapping: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `weight_completeness`.
/// Call at function entry: `contract_pre_weight_completeness!(input_expr)`
macro_rules! contract_pre_weight_completeness {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract weight_completeness: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/arima-v1.yaml — DO NOT EDIT
// Contract: arima-v1

/// Preconditions for equation `ar_forecast`.
/// Call at function entry: `contract_pre_ar_forecast!(input_expr)`
macro_rules! contract_pre_ar_forecast {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract ar_forecast: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `differencing`.
/// Call at function entry: `contract_pre_differencing!(input_expr)`
macro_rules! contract_pre_differencing {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract differencing: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `forecast_finite`.
/// Call at function entry: `contract_pre_forecast_finite!(input_expr)`
macro_rules! contract_pre_forecast_finite {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract forecast_finite: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `ma_filter`.
/// Call at function entry: `contract_pre_ma_filter!(input_expr)`
macro_rules! contract_pre_ma_filter {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract ma_filter: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/attention-kernel-v1.yaml — DO NOT EDIT
// Contract: attention-kernel-v1

/// Preconditions for equation `attention`.
/// Call at function entry: `contract_pre_attention!(input_expr)`
macro_rules! contract_pre_attention {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract attention: precondition violated — input must not be empty"
        );
    }};
}

/// Postconditions for equation `attention`.
/// Call before return: `contract_post_attention!(result_expr)`
macro_rules! contract_post_attention {
    ($result:expr) => {{
        let _contract_result = &$result;
        debug_assert!(
            _contract_result.iter().all(|v| v.is_finite()),
            "Contract attention: postcondition violated — result.iter().all(|v| v.is_finite())"
        );
    }};
}

// Auto-generated from contracts/attention-scaling-v1.yaml — DO NOT EDIT
// Contract: attention-scaling-v1

/// Preconditions for equation `attention_entropy`.
/// Call at function entry: `contract_pre_attention_entropy!(input_expr)`
macro_rules! contract_pre_attention_entropy {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract attention_entropy: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `numerical_stability`.
/// Call at function entry: `contract_pre_numerical_stability!(input_expr)`
macro_rules! contract_pre_numerical_stability {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract numerical_stability: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `scaled_dot_product`.
/// Call at function entry: `contract_pre_scaled_dot_product!(input_expr)`
macro_rules! contract_pre_scaled_dot_product {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract scaled_dot_product: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `score_bound_with_qknorm`.
/// Call at function entry: `contract_pre_score_bound_with_qknorm!(input_expr)`
macro_rules! contract_pre_score_bound_with_qknorm {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract score_bound_with_qknorm: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `softmax_saturation`.
/// Call at function entry: `contract_pre_softmax_saturation!(input_expr)`
macro_rules! contract_pre_softmax_saturation {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract softmax_saturation: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `variance_preservation`.
/// Call at function entry: `contract_pre_variance_preservation!(input_expr)`
macro_rules! contract_pre_variance_preservation {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract variance_preservation: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/avx2-fma-dot-v1.yaml — DO NOT EDIT
// Contract: avx2-fma-dot-v1

/// Preconditions for equation `dot_product`.
/// Call at function entry: `contract_pre_dot_product!(input_expr)`
macro_rules! contract_pre_dot_product {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract dot_product: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `fma_accumulation`.
/// Call at function entry: `contract_pre_fma_accumulation!(input_expr)`
macro_rules! contract_pre_fma_accumulation {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract fma_accumulation: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/backend-dispatch-v1.yaml — DO NOT EDIT
// Contract: backend-dispatch-v1

/// Preconditions for equation `garbage_oracle`.
/// Call at function entry: `contract_pre_garbage_oracle!(input_expr)`
macro_rules! contract_pre_garbage_oracle {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract garbage_oracle: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `gpu_threshold`.
/// Call at function entry: `contract_pre_gpu_threshold!(input_expr)`
macro_rules! contract_pre_gpu_threshold {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract gpu_threshold: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `qk_norm_score_bound`.
/// Call at function entry: `contract_pre_qk_norm_score_bound!(input_expr)`
macro_rules! contract_pre_qk_norm_score_bound {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract qk_norm_score_bound: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `simd_only_threshold`.
/// Call at function entry: `contract_pre_simd_only_threshold!(input_expr)`
macro_rules! contract_pre_simd_only_threshold {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract simd_only_threshold: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/batched-beam-search-v1.yaml — DO NOT EDIT
// Contract: batched-beam-search-v1

/// Preconditions for equation `batched_beam_projection`.
/// Call at function entry: `contract_pre_batched_beam_projection!(input_expr)`
macro_rules! contract_pre_batched_beam_projection {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract batched_beam_projection: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `beam_selection`.
/// Call at function entry: `contract_pre_beam_selection!(input_expr)`
macro_rules! contract_pre_beam_selection {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract beam_selection: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `sequential_beam_projection`.
/// Call at function entry: `contract_pre_sequential_beam_projection!(input_expr)`
macro_rules! contract_pre_sequential_beam_projection {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract sequential_beam_projection: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `termination`.
/// Call at function entry: `contract_pre_termination!(input_expr)`
macro_rules! contract_pre_termination {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract termination: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/batchnorm-kernel-v1.yaml — DO NOT EDIT
// Contract: batchnorm-kernel-v1

/// Preconditions for equation `batchnorm_eval`.
/// Call at function entry: `contract_pre_batchnorm_eval!(input_expr)`
macro_rules! contract_pre_batchnorm_eval {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract batchnorm_eval: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `batchnorm_train`.
/// Call at function entry: `contract_pre_batchnorm_train!(input_expr)`
macro_rules! contract_pre_batchnorm_train {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract batchnorm_train: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `running_stats`.
/// Call at function entry: `contract_pre_running_stats!(input_expr)`
macro_rules! contract_pre_running_stats {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract running_stats: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/bayesian-v1.yaml — DO NOT EDIT
// Contract: bayesian-v1

/// Preconditions for equation `blr_predict`.
/// Call at function entry: `contract_pre_blr_predict!(input_expr)`
macro_rules! contract_pre_blr_predict {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract blr_predict: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `conjugate_update`.
/// Call at function entry: `contract_pre_conjugate_update!(input_expr)`
macro_rules! contract_pre_conjugate_update {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract conjugate_update: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `posterior_predictive`.
/// Call at function entry: `contract_pre_posterior_predictive!(input_expr)`
macro_rules! contract_pre_posterior_predictive {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract posterior_predictive: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `posterior_valid`.
/// Call at function entry: `contract_pre_posterior_valid!(input_expr)`
macro_rules! contract_pre_posterior_valid {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract posterior_valid: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/bias-add-v1.yaml — DO NOT EDIT
// Contract: bias-add-v1

/// Preconditions for equation `bias_add`.
/// Call at function entry: `contract_pre_bias_add!(input_expr)`
macro_rules! contract_pre_bias_add {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract bias_add: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/bidirectional-attention-v1.yaml — DO NOT EDIT
// Contract: bidirectional-attention-v1

/// Preconditions for equation `bidirectional_attention`.
/// Call at function entry: `contract_pre_bidirectional_attention!(input_expr)`
macro_rules! contract_pre_bidirectional_attention {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract bidirectional_attention: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/bpe-tokenization-v1.yaml — DO NOT EDIT
// Contract: bpe-tokenization-v1

/// Preconditions for equation `decode`.
/// Call at function entry: `contract_pre_decode!(input_expr)`
macro_rules! contract_pre_decode {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract decode: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `encode`.
/// Call at function entry: `contract_pre_encode!(input_expr)`
macro_rules! contract_pre_encode {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract encode: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `merge_rule`.
/// Call at function entry: `contract_pre_merge_rule!(input_expr)`
macro_rules! contract_pre_merge_rule {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract merge_rule: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/builder-pattern-v1.yaml — DO NOT EDIT
// Contract: builder-pattern-v1

/// Preconditions for equation `builder_pattern`.
/// Call at function entry: `contract_pre_builder_pattern!(input_expr)`
macro_rules! contract_pre_builder_pattern {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract builder_pattern: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/calibration-v1.yaml — DO NOT EDIT
// Contract: calibration-v1

/// Preconditions for equation `expected_calibration_error`.
/// Call at function entry: `contract_pre_expected_calibration_error!(input_expr)`
macro_rules! contract_pre_expected_calibration_error {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract expected_calibration_error: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `isotonic_regression`.
/// Call at function entry: `contract_pre_isotonic_regression!(input_expr)`
macro_rules! contract_pre_isotonic_regression {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract isotonic_regression: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `maximum_calibration_error`.
/// Call at function entry: `contract_pre_maximum_calibration_error!(input_expr)`
macro_rules! contract_pre_maximum_calibration_error {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract maximum_calibration_error: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `platt_scaling`.
/// Call at function entry: `contract_pre_platt_scaling!(input_expr)`
macro_rules! contract_pre_platt_scaling {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract platt_scaling: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `reliability_diagram`.
/// Call at function entry: `contract_pre_reliability_diagram!(input_expr)`
macro_rules! contract_pre_reliability_diagram {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract reliability_diagram: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/classification-finetune-v1.yaml — DO NOT EDIT
// Contract: classification-finetune-v1

/// Preconditions for equation `classifier_weight_shape`.
/// Call at function entry: `contract_pre_classifier_weight_shape!(input_expr)`
macro_rules! contract_pre_classifier_weight_shape {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract classifier_weight_shape: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `label_bounds`.
/// Call at function entry: `contract_pre_label_bounds!(input_expr)`
macro_rules! contract_pre_label_bounds {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract label_bounds: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `logit_shape`.
/// Call at function entry: `contract_pre_logit_shape!(input_expr)`
macro_rules! contract_pre_logit_shape {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract logit_shape: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `softmax_sum`.
/// Call at function entry: `contract_pre_softmax_sum!(input_expr)`
macro_rules! contract_pre_softmax_sum {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract softmax_sum: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/classifier-pipeline-v1.yaml — DO NOT EDIT
// Contract: classifier-pipeline-v1

/// Preconditions for equation `embedding_extraction`.
/// Call at function entry: `contract_pre_embedding_extraction!(input_expr)`
macro_rules! contract_pre_embedding_extraction {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract embedding_extraction: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `evaluation`.
/// Call at function entry: `contract_pre_evaluation!(input_expr)`
macro_rules! contract_pre_evaluation {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract evaluation: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `linear_probe`.
/// Call at function entry: `contract_pre_linear_probe!(input_expr)`
macro_rules! contract_pre_linear_probe {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract linear_probe: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/cma-es-kernel-v1.yaml — DO NOT EDIT
// Contract: cma-es-kernel-v1

/// Preconditions for equation `covariance_update`.
/// Call at function entry: `contract_pre_covariance_update!(input_expr)`
macro_rules! contract_pre_covariance_update {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract covariance_update: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `mean_update`.
/// Call at function entry: `contract_pre_mean_update!(input_expr)`
macro_rules! contract_pre_mean_update {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract mean_update: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `sample`.
/// Call at function entry: `contract_pre_sample!(input_expr)`
macro_rules! contract_pre_sample {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract sample: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/codebert-tokenizer-validation-v1.yaml — DO NOT EDIT
// Contract: codebert-tokenizer-validation-v1

/// Preconditions for equation `tokenizer_adequacy`.
/// Call at function entry: `contract_pre_tokenizer_adequacy!(input_expr)`
macro_rules! contract_pre_tokenizer_adequacy {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract tokenizer_adequacy: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/configuration-v1.yaml — DO NOT EDIT
// Contract: configuration-v1

/// Preconditions for equation `configuration`.
/// Call at function entry: `contract_pre_configuration!(input_expr)`
macro_rules! contract_pre_configuration {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract configuration: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/continuous-batching-v1.yaml — DO NOT EDIT
// Contract: continuous-batching-v1

/// Preconditions for equation `chunked_prefill`.
/// Call at function entry: `contract_pre_chunked_prefill!(input_expr)`
macro_rules! contract_pre_chunked_prefill {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract chunked_prefill: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `correctness_under_batching`.
/// Call at function entry: `contract_pre_correctness_under_batching!(input_expr)`
macro_rules! contract_pre_correctness_under_batching {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract correctness_under_batching: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `decode_degradation`.
/// Call at function entry: `contract_pre_decode_degradation!(input_expr)`
macro_rules! contract_pre_decode_degradation {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract decode_degradation: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `request_state`.
/// Call at function entry: `contract_pre_request_state!(input_expr)`
macro_rules! contract_pre_request_state {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract request_state: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `scheduling_fairness`.
/// Call at function entry: `contract_pre_scheduling_fairness!(input_expr)`
macro_rules! contract_pre_scheduling_fairness {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract scheduling_fairness: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `throughput_scaling`.
/// Call at function entry: `contract_pre_throughput_scaling!(input_expr)`
macro_rules! contract_pre_throughput_scaling {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract throughput_scaling: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `token_budget`.
/// Call at function entry: `contract_pre_token_budget!(input_expr)`
macro_rules! contract_pre_token_budget {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract token_budget: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/conv1d-kernel-v1.yaml — DO NOT EDIT
// Contract: conv1d-kernel-v1

/// Preconditions for equation `conv1d`.
/// Call at function entry: `contract_pre_conv1d!(input_expr)`
macro_rules! contract_pre_conv1d {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract conv1d: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/conversation-generation-v1.yaml — DO NOT EDIT
// Contract: conversation-generation-v1

/// Preconditions for equation `chatml_format`.
/// Call at function entry: `contract_pre_chatml_format!(input_expr)`
macro_rules! contract_pre_chatml_format {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract chatml_format: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `conversation_types`.
/// Call at function entry: `contract_pre_conversation_types!(input_expr)`
macro_rules! contract_pre_conversation_types {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract conversation_types: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `quality_gate`.
/// Call at function entry: `contract_pre_quality_gate!(input_expr)`
macro_rules! contract_pre_quality_gate {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract quality_gate: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/cpu-q4k-activation-quant-v1.yaml — DO NOT EDIT
// Contract: cpu-q4k-activation-quant-v1

/// Preconditions for equation `current_path`.
/// Call at function entry: `contract_pre_current_path!(input_expr)`
macro_rules! contract_pre_current_path {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract current_path: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `speedup_bound`.
/// Call at function entry: `contract_pre_speedup_bound!(input_expr)`
macro_rules! contract_pre_speedup_bound {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract speedup_bound: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `target_path`.
/// Call at function entry: `contract_pre_target_path!(input_expr)`
macro_rules! contract_pre_target_path {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract target_path: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/cpu-work-stealing-v1.yaml — DO NOT EDIT
// Contract: cpu-work-stealing-v1

/// Preconditions for equation `l1_tiling`.
/// Call at function entry: `contract_pre_l1_tiling!(input_expr)`
macro_rules! contract_pre_l1_tiling {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract l1_tiling: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `rayon_overhead`.
/// Call at function entry: `contract_pre_rayon_overhead!(input_expr)`
macro_rules! contract_pre_rayon_overhead {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract rayon_overhead: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/cross-entropy-kernel-v1.yaml — DO NOT EDIT
// Contract: cross-entropy-kernel-v1

/// Preconditions for equation `cross_entropy`.
/// Call at function entry: `contract_pre_cross_entropy!(input_expr)`
macro_rules! contract_pre_cross_entropy {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract cross_entropy: precondition violated — input must not be empty"
        );
    }};
}

/// Postconditions for equation `cross_entropy`.
/// Call before return: `contract_post_cross_entropy!(result_expr)`
macro_rules! contract_post_cross_entropy {
    ($result:expr) => {{
        let _contract_result = &$result;
        debug_assert!(
            _contract_result.is_finite(),
            "Contract cross_entropy: postcondition violated — result.is_finite()"
        );
        debug_assert!(
            _contract_result >= 0.0,
            "Contract cross_entropy: postcondition violated — result >= 0.0"
        );
    }};
}

/// Preconditions for equation `log_softmax`.
/// Call at function entry: `contract_pre_log_softmax!(input_expr)`
macro_rules! contract_pre_log_softmax {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract log_softmax: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/decision-tree-v1.yaml — DO NOT EDIT
// Contract: decision-tree-v1

/// Preconditions for equation `gini_impurity`.
/// Call at function entry: `contract_pre_gini_impurity!(input_expr)`
macro_rules! contract_pre_gini_impurity {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract gini_impurity: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `gini_split`.
/// Call at function entry: `contract_pre_gini_split!(input_expr)`
macro_rules! contract_pre_gini_split {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract gini_split: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `mse_split`.
/// Call at function entry: `contract_pre_mse_split!(input_expr)`
macro_rules! contract_pre_mse_split {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract mse_split: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `prediction`.
/// Call at function entry: `contract_pre_prediction!(input_expr)`
macro_rules! contract_pre_prediction {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract prediction: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/display-format-v1.yaml — DO NOT EDIT
// Contract: display-format-v1

/// Preconditions for equation `display_format`.
/// Call at function entry: `contract_pre_display_format!(input_expr)`
macro_rules! contract_pre_display_format {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract display_format: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/dpo-loss-v1.yaml — DO NOT EDIT
// Contract: dpo-loss-v1

/// Preconditions for equation `dpo_loss`.
/// Call at function entry: `contract_pre_dpo_loss!(input_expr)`
macro_rules! contract_pre_dpo_loss {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract dpo_loss: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `implicit_reward`.
/// Call at function entry: `contract_pre_implicit_reward!(input_expr)`
macro_rules! contract_pre_implicit_reward {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract implicit_reward: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `log_ratio`.
/// Call at function entry: `contract_pre_log_ratio!(input_expr)`
macro_rules! contract_pre_log_ratio {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract log_ratio: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/drift-detection-v1.yaml — DO NOT EDIT
// Contract: drift-detection-v1

/// Preconditions for equation `classify_drift`.
/// Call at function entry: `contract_pre_classify_drift!(input_expr)`
macro_rules! contract_pre_classify_drift {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract classify_drift: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `min_samples_guard`.
/// Call at function entry: `contract_pre_min_samples_guard!(input_expr)`
macro_rules! contract_pre_min_samples_guard {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract min_samples_guard: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `performance_drift`.
/// Call at function entry: `contract_pre_performance_drift!(input_expr)`
macro_rules! contract_pre_performance_drift {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract performance_drift: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `univariate_drift`.
/// Call at function entry: `contract_pre_univariate_drift!(input_expr)`
macro_rules! contract_pre_univariate_drift {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract univariate_drift: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/dropout-v1.yaml — DO NOT EDIT
// Contract: dropout-v1

/// Preconditions for equation `dropout_eval`.
/// Call at function entry: `contract_pre_dropout_eval!(input_expr)`
macro_rules! contract_pre_dropout_eval {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract dropout_eval: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `dropout_train`.
/// Call at function entry: `contract_pre_dropout_train!(input_expr)`
macro_rules! contract_pre_dropout_train {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract dropout_train: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/embedding-algebra-v1.yaml — DO NOT EDIT
// Contract: embedding-algebra-v1

/// Preconditions for equation `embedding_lookup`.
/// Call at function entry: `contract_pre_embedding_lookup!(input_expr)`
macro_rules! contract_pre_embedding_lookup {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract embedding_lookup: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `embedding_norm`.
/// Call at function entry: `contract_pre_embedding_norm!(input_expr)`
macro_rules! contract_pre_embedding_norm {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract embedding_norm: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `logit_temperature`.
/// Call at function entry: `contract_pre_logit_temperature!(input_expr)`
macro_rules! contract_pre_logit_temperature {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract logit_temperature: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `tied_weights`.
/// Call at function entry: `contract_pre_tied_weights!(input_expr)`
macro_rules! contract_pre_tied_weights {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract tied_weights: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `unembedding_projection`.
/// Call at function entry: `contract_pre_unembedding_projection!(input_expr)`
macro_rules! contract_pre_unembedding_projection {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract unembedding_projection: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `vocabulary_bounds`.
/// Call at function entry: `contract_pre_vocabulary_bounds!(input_expr)`
macro_rules! contract_pre_vocabulary_bounds {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract vocabulary_bounds: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/embedding-lookup-v1.yaml — DO NOT EDIT
// Contract: embedding-lookup-v1

/// Preconditions for equation `embedding_lookup`.
/// Call at function entry: `contract_pre_embedding_lookup!(input_expr)`
macro_rules! contract_pre_embedding_lookup {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract embedding_lookup: precondition violated — input must not be empty"
        );
    }};
}

/// Postconditions for equation `embedding_lookup`.
/// Call before return: `contract_post_embedding_lookup!(result_expr)`
macro_rules! contract_post_embedding_lookup {
    ($result:expr) => {{
        let _contract_result = &$result;
        debug_assert!(_contract_result.len() == token_ids.len() * embed_dim, "Contract embedding_lookup: postcondition violated — result.len() == token_ids.len() * embed_dim");
        debug_assert!(_contract_result.iter().all(|v| v.is_finite()), "Contract embedding_lookup: postcondition violated — result.iter().all(|v| v.is_finite())");
    }};
}

// Auto-generated from contracts/encoder-forward-v1.yaml — DO NOT EDIT
// Contract: encoder-forward-v1

/// Preconditions for equation `cls_pooling`.
/// Call at function entry: `contract_pre_cls_pooling!(input_expr)`
macro_rules! contract_pre_cls_pooling {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract cls_pooling: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `encoder_layer`.
/// Call at function entry: `contract_pre_encoder_layer!(input_expr)`
macro_rules! contract_pre_encoder_layer {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract encoder_layer: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/error-handling-v1.yaml — DO NOT EDIT
// Contract: error-handling-v1

/// Preconditions for equation `error_handling`.
/// Call at function entry: `contract_pre_error_handling!(input_expr)`
macro_rules! contract_pre_error_handling {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract error_handling: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/f16-conversion-v1.yaml — DO NOT EDIT
// Contract: f16-conversion-v1

/// Preconditions for equation `f16_to_f32_bias`.
/// Call at function entry: `contract_pre_f16_to_f32_bias!(input_expr)`
macro_rules! contract_pre_f16_to_f32_bias {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract f16_to_f32_bias: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `roundtrip`.
/// Call at function entry: `contract_pre_roundtrip!(input_expr)`
macro_rules! contract_pre_roundtrip {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract roundtrip: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/flash-attention-v1.yaml — DO NOT EDIT
// Contract: flash-attention-v1

/// Preconditions for equation `flash_attention`.
/// Call at function entry: `contract_pre_flash_attention!(input_expr)`
macro_rules! contract_pre_flash_attention {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract flash_attention: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/format-parity-v1.yaml — DO NOT EDIT
// Contract: format-parity-v1

/// Preconditions for equation `element_count`.
/// Call at function entry: `contract_pre_element_count!(input_expr)`
macro_rules! contract_pre_element_count {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract element_count: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `identity_1d`.
/// Call at function entry: `contract_pre_identity_1d!(input_expr)`
macro_rules! contract_pre_identity_1d {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract identity_1d: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `name_bijection`.
/// Call at function entry: `contract_pre_name_bijection!(input_expr)`
macro_rules! contract_pre_name_bijection {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract name_bijection: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `transpose_involution`.
/// Call at function entry: `contract_pre_transpose_involution!(input_expr)`
macro_rules! contract_pre_transpose_involution {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract transpose_involution: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/fp8-interchange-v1.yaml — DO NOT EDIT
// Contract: fp8-interchange-v1

/// Preconditions for equation `e4m3_encode`.
/// Call at function entry: `contract_pre_e4m3_encode!(input_expr)`
macro_rules! contract_pre_e4m3_encode {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract e4m3_encode: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `e5m2_encode`.
/// Call at function entry: `contract_pre_e5m2_encode!(input_expr)`
macro_rules! contract_pre_e5m2_encode {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract e5m2_encode: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `roundtrip`.
/// Call at function entry: `contract_pre_roundtrip!(input_expr)`
macro_rules! contract_pre_roundtrip {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract roundtrip: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/fused-qkv-projection-v1.yaml — DO NOT EDIT
// Contract: fused-qkv-projection-v1

/// Preconditions for equation `fused_qkv`.
/// Call at function entry: `contract_pre_fused_qkv!(input_expr)`
macro_rules! contract_pre_fused_qkv {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract fused_qkv: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `separate_qkv`.
/// Call at function entry: `contract_pre_separate_qkv!(input_expr)`
macro_rules! contract_pre_separate_qkv {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract separate_qkv: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `shared_q8_qkv`.
/// Call at function entry: `contract_pre_shared_q8_qkv!(input_expr)`
macro_rules! contract_pre_shared_q8_qkv {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract shared_q8_qkv: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/gated-delta-net-v1.yaml — DO NOT EDIT
// Contract: gated-delta-net-v1

/// Preconditions for equation `decay`.
/// Call at function entry: `contract_pre_decay!(input_expr)`
macro_rules! contract_pre_decay {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract decay: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `delta`.
/// Call at function entry: `contract_pre_delta!(input_expr)`
macro_rules! contract_pre_delta {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract delta: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `output`.
/// Call at function entry: `contract_pre_output!(input_expr)`
macro_rules! contract_pre_output {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract output: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `read`.
/// Call at function entry: `contract_pre_read!(input_expr)`
macro_rules! contract_pre_read {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract read: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `write`.
/// Call at function entry: `contract_pre_write!(input_expr)`
macro_rules! contract_pre_write {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract write: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/gbm-v1.yaml — DO NOT EDIT
// Contract: gbm-v1

/// Preconditions for equation `gradient_boost`.
/// Call at function entry: `contract_pre_gradient_boost!(input_expr)`
macro_rules! contract_pre_gradient_boost {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract gradient_boost: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `negative_gradient`.
/// Call at function entry: `contract_pre_negative_gradient!(input_expr)`
macro_rules! contract_pre_negative_gradient {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract negative_gradient: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `predict`.
/// Call at function entry: `contract_pre_predict!(input_expr)`
macro_rules! contract_pre_predict {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract predict: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `training_loss`.
/// Call at function entry: `contract_pre_training_loss!(input_expr)`
macro_rules! contract_pre_training_loss {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract training_loss: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/gelu-kernel-v1.yaml — DO NOT EDIT
// Contract: gelu-kernel-v1

/// Preconditions for equation `gelu`.
/// Call at function entry: `contract_pre_gelu!(input_expr)`
macro_rules! contract_pre_gelu {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract gelu: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `gelu_tanh_approx`.
/// Call at function entry: `contract_pre_gelu_tanh_approx!(input_expr)`
macro_rules! contract_pre_gelu_tanh_approx {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract gelu_tanh_approx: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/gguf-cpu-cache-v1.yaml — DO NOT EDIT
// Contract: gguf-cpu-cache-v1

/// Preconditions for equation `autoregressive_generation`.
/// Call at function entry: `contract_pre_autoregressive_generation!(input_expr)`
macro_rules! contract_pre_autoregressive_generation {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract autoregressive_generation: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/glm-v1.yaml — DO NOT EDIT
// Contract: glm-v1

/// Preconditions for equation `binomial_link`.
/// Call at function entry: `contract_pre_binomial_link!(input_expr)`
macro_rules! contract_pre_binomial_link {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract binomial_link: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `gamma_link`.
/// Call at function entry: `contract_pre_gamma_link!(input_expr)`
macro_rules! contract_pre_gamma_link {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract gamma_link: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `irls_fit`.
/// Call at function entry: `contract_pre_irls_fit!(input_expr)`
macro_rules! contract_pre_irls_fit {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract irls_fit: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `poisson_link`.
/// Call at function entry: `contract_pre_poisson_link!(input_expr)`
macro_rules! contract_pre_poisson_link {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract poisson_link: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/gnn-v1.yaml — DO NOT EDIT
// Contract: gnn-v1

/// Preconditions for equation `gcn_aggregate`.
/// Call at function entry: `contract_pre_gcn_aggregate!(input_expr)`
macro_rules! contract_pre_gcn_aggregate {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract gcn_aggregate: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `global_max_pool`.
/// Call at function entry: `contract_pre_global_max_pool!(input_expr)`
macro_rules! contract_pre_global_max_pool {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract global_max_pool: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `global_mean_pool`.
/// Call at function entry: `contract_pre_global_mean_pool!(input_expr)`
macro_rules! contract_pre_global_mean_pool {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract global_mean_pool: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `message_passing`.
/// Call at function entry: `contract_pre_message_passing!(input_expr)`
macro_rules! contract_pre_message_passing {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract message_passing: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/gpu-context-health-v1.yaml — DO NOT EDIT
// Contract: gpu-context-health-v1

/// Preconditions for equation `context_health`.
/// Call at function entry: `contract_pre_context_health!(input_expr)`
macro_rules! contract_pre_context_health {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract context_health: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `cuda_graph_guard`.
/// Call at function entry: `contract_pre_cuda_graph_guard!(input_expr)`
macro_rules! contract_pre_cuda_graph_guard {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract cuda_graph_guard: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `culink_skip`.
/// Call at function entry: `contract_pre_culink_skip!(input_expr)`
macro_rules! contract_pre_culink_skip {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract culink_skip: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `fp8_architecture_guard`.
/// Call at function entry: `contract_pre_fp8_architecture_guard!(input_expr)`
macro_rules! contract_pre_fp8_architecture_guard {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract fp8_architecture_guard: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/gpu-decode-profiling-v1.yaml — DO NOT EDIT
// Contract: gpu-decode-profiling-v1

/// Preconditions for equation `brick_ordering`.
/// Call at function entry: `contract_pre_brick_ordering!(input_expr)`
macro_rules! contract_pre_brick_ordering {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract brick_ordering: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `graph_disable`.
/// Call at function entry: `contract_pre_graph_disable!(input_expr)`
macro_rules! contract_pre_graph_disable {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract graph_disable: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `report_completeness`.
/// Call at function entry: `contract_pre_report_completeness!(input_expr)`
macro_rules! contract_pre_report_completeness {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract report_completeness: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `report_denominator`.
/// Call at function entry: `contract_pre_report_denominator!(input_expr)`
macro_rules! contract_pre_report_denominator {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract report_denominator: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `report_fidelity`.
/// Call at function entry: `contract_pre_report_fidelity!(input_expr)`
macro_rules! contract_pre_report_fidelity {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract report_fidelity: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `report_metadata`.
/// Call at function entry: `contract_pre_report_metadata!(input_expr)`
macro_rules! contract_pre_report_metadata {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract report_metadata: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `sync_verification`.
/// Call at function entry: `contract_pre_sync_verification!(input_expr)`
macro_rules! contract_pre_sync_verification {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract sync_verification: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `token_accounting`.
/// Call at function entry: `contract_pre_token_accounting!(input_expr)`
macro_rules! contract_pre_token_accounting {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract token_accounting: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `wall_coverage`.
/// Call at function entry: `contract_pre_wall_coverage!(input_expr)`
macro_rules! contract_pre_wall_coverage {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract wall_coverage: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/gpu-multi-backend-parity-v1.yaml — DO NOT EDIT
// Contract: gpu-multi-backend-parity-v1

/// Preconditions for equation `backend_priority`.
/// Call at function entry: `contract_pre_backend_priority!(input_expr)`
macro_rules! contract_pre_backend_priority {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract backend_priority: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `bandwidth_bound_theorem`.
/// Call at function entry: `contract_pre_bandwidth_bound_theorem!(input_expr)`
macro_rules! contract_pre_bandwidth_bound_theorem {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract bandwidth_bound_theorem: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `jit_compilation_correctness`.
/// Call at function entry: `contract_pre_jit_compilation_correctness!(input_expr)`
macro_rules! contract_pre_jit_compilation_correctness {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(!_contract_input.is_empty(),
            "Contract jit_compilation_correctness: precondition violated — input must not be empty");
    }};
}

/// Preconditions for equation `multi_backend_parity`.
/// Call at function entry: `contract_pre_multi_backend_parity!(input_expr)`
macro_rules! contract_pre_multi_backend_parity {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract multi_backend_parity: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/gpu-weight-residency-v1.yaml — DO NOT EDIT
// Contract: gpu-weight-residency-v1

/// Preconditions for equation `pcie_overhead`.
/// Call at function entry: `contract_pre_pcie_overhead!(input_expr)`
macro_rules! contract_pre_pcie_overhead {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract pcie_overhead: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `throughput_target`.
/// Call at function entry: `contract_pre_throughput_target!(input_expr)`
macro_rules! contract_pre_throughput_target {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract throughput_target: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/gqa-kernel-v1.yaml — DO NOT EDIT
// Contract: gqa-kernel-v1

/// Preconditions for equation `gqa`.
/// Call at function entry: `contract_pre_gqa!(input_expr)`
macro_rules! contract_pre_gqa {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract gqa: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/graph-centrality-v1.yaml — DO NOT EDIT
// Contract: graph-centrality-v1

/// Preconditions for equation `betweenness`.
/// Call at function entry: `contract_pre_betweenness!(input_expr)`
macro_rules! contract_pre_betweenness {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract betweenness: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `closeness`.
/// Call at function entry: `contract_pre_closeness!(input_expr)`
macro_rules! contract_pre_closeness {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract closeness: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `degree`.
/// Call at function entry: `contract_pre_degree!(input_expr)`
macro_rules! contract_pre_degree {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract degree: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `eigenvector`.
/// Call at function entry: `contract_pre_eigenvector!(input_expr)`
macro_rules! contract_pre_eigenvector {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract eigenvector: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `harmonic`.
/// Call at function entry: `contract_pre_harmonic!(input_expr)`
macro_rules! contract_pre_harmonic {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract harmonic: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `katz`.
/// Call at function entry: `contract_pre_katz!(input_expr)`
macro_rules! contract_pre_katz {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract katz: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/hybrid-layer-dispatch-v1.yaml — DO NOT EDIT
// Contract: hybrid-layer-dispatch-v1

/// Preconditions for equation `conv1d_causal`.
/// Call at function entry: `contract_pre_conv1d_causal!(input_expr)`
macro_rules! contract_pre_conv1d_causal {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract conv1d_causal: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `head_grouping`.
/// Call at function entry: `contract_pre_head_grouping!(input_expr)`
macro_rules! contract_pre_head_grouping {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract head_grouping: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `hybrid_dispatch`.
/// Call at function entry: `contract_pre_hybrid_dispatch!(input_expr)`
macro_rules! contract_pre_hybrid_dispatch {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract hybrid_dispatch: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `linear_associativity`.
/// Call at function entry: `contract_pre_linear_associativity!(input_expr)`
macro_rules! contract_pre_linear_associativity {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract linear_associativity: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `linear_no_softmax`.
/// Call at function entry: `contract_pre_linear_no_softmax!(input_expr)`
macro_rules! contract_pre_linear_no_softmax {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract linear_no_softmax: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `linear_shapes`.
/// Call at function entry: `contract_pre_linear_shapes!(input_expr)`
macro_rules! contract_pre_linear_shapes {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract linear_shapes: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/ica-v1.yaml — DO NOT EDIT
// Contract: ica-v1

/// Preconditions for equation `fastica`.
/// Call at function entry: `contract_pre_fastica!(input_expr)`
macro_rules! contract_pre_fastica {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract fastica: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `mixing`.
/// Call at function entry: `contract_pre_mixing!(input_expr)`
macro_rules! contract_pre_mixing {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract mixing: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `unmixing`.
/// Call at function entry: `contract_pre_unmixing!(input_expr)`
macro_rules! contract_pre_unmixing {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract unmixing: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/inference-pipeline-v1.yaml — DO NOT EDIT
// Contract: inference-pipeline-v1

/// Preconditions for equation `decode_step`.
/// Call at function entry: `contract_pre_decode_step!(input_expr)`
macro_rules! contract_pre_decode_step {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract decode_step: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `hybrid_layer_schedule`.
/// Call at function entry: `contract_pre_hybrid_layer_schedule!(input_expr)`
macro_rules! contract_pre_hybrid_layer_schedule {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract hybrid_layer_schedule: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `kv_cache_growth`.
/// Call at function entry: `contract_pre_kv_cache_growth!(input_expr)`
macro_rules! contract_pre_kv_cache_growth {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract kv_cache_growth: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `layer_composition`.
/// Call at function entry: `contract_pre_layer_composition!(input_expr)`
macro_rules! contract_pre_layer_composition {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract layer_composition: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `prefill_phase`.
/// Call at function entry: `contract_pre_prefill_phase!(input_expr)`
macro_rules! contract_pre_prefill_phase {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract prefill_phase: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `residual_stream`.
/// Call at function entry: `contract_pre_residual_stream!(input_expr)`
macro_rules! contract_pre_residual_stream {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract residual_stream: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/int8-symmetric-quant-v1.yaml — DO NOT EDIT
// Contract: int8-symmetric-quant-v1

/// Preconditions for equation `dequant_dot`.
/// Call at function entry: `contract_pre_dequant_dot!(input_expr)`
macro_rules! contract_pre_dequant_dot {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract dequant_dot: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `per_row_scale`.
/// Call at function entry: `contract_pre_per_row_scale!(input_expr)`
macro_rules! contract_pre_per_row_scale {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract per_row_scale: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `quantize`.
/// Call at function entry: `contract_pre_quantize!(input_expr)`
macro_rules! contract_pre_quantize {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract quantize: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/iterator-v1.yaml — DO NOT EDIT
// Contract: iterator-v1

/// Preconditions for equation `iterator`.
/// Call at function entry: `contract_pre_iterator!(input_expr)`
macro_rules! contract_pre_iterator {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract iterator: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/kernel-launch-budget-v1.yaml — DO NOT EDIT
// Contract: kernel-launch-budget-v1

/// Preconditions for equation `bsum_budget`.
/// Call at function entry: `contract_pre_bsum_budget!(input_expr)`
macro_rules! contract_pre_bsum_budget {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract bsum_budget: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `per_layer_decomposition`.
/// Call at function entry: `contract_pre_per_layer_decomposition!(input_expr)`
macro_rules! contract_pre_per_layer_decomposition {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract per_layer_decomposition: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `per_token_launches`.
/// Call at function entry: `contract_pre_per_token_launches!(input_expr)`
macro_rules! contract_pre_per_token_launches {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract per_token_launches: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/kmeans-kernel-v1.yaml — DO NOT EDIT
// Contract: kmeans-kernel-v1

/// Preconditions for equation `assignment`.
/// Call at function entry: `contract_pre_assignment!(input_expr)`
macro_rules! contract_pre_assignment {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract assignment: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `objective`.
/// Call at function entry: `contract_pre_objective!(input_expr)`
macro_rules! contract_pre_objective {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract objective: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `update`.
/// Call at function entry: `contract_pre_update!(input_expr)`
macro_rules! contract_pre_update {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract update: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/kv-cache-equivalence-v1.yaml — DO NOT EDIT
// Contract: kv-cache-equivalence-v1

/// Preconditions for equation `batched_serial_equivalence`.
/// Call at function entry: `contract_pre_batched_serial_equivalence!(input_expr)`
macro_rules! contract_pre_batched_serial_equivalence {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract batched_serial_equivalence: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `fused_kernel`.
/// Call at function entry: `contract_pre_fused_kernel!(input_expr)`
macro_rules! contract_pre_fused_kernel {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract fused_kernel: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `page_shape`.
/// Call at function entry: `contract_pre_page_shape!(input_expr)`
macro_rules! contract_pre_page_shape {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract page_shape: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `prefill_incremental`.
/// Call at function entry: `contract_pre_prefill_incremental!(input_expr)`
macro_rules! contract_pre_prefill_incremental {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract prefill_incremental: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/kv-cache-sizing-v1.yaml — DO NOT EDIT
// Contract: kv-cache-sizing-v1

/// Preconditions for equation `bias_absence`.
/// Call at function entry: `contract_pre_bias_absence!(input_expr)`
macro_rules! contract_pre_bias_absence {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract bias_absence: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `hybrid_accounting`.
/// Call at function entry: `contract_pre_hybrid_accounting!(input_expr)`
macro_rules! contract_pre_hybrid_accounting {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract hybrid_accounting: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `per_token_per_layer`.
/// Call at function entry: `contract_pre_per_token_per_layer!(input_expr)`
macro_rules! contract_pre_per_token_per_layer {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract per_token_per_layer: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `total_kv_memory`.
/// Call at function entry: `contract_pre_total_kv_memory!(input_expr)`
macro_rules! contract_pre_total_kv_memory {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract total_kv_memory: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `zero_input_identity`.
/// Call at function entry: `contract_pre_zero_input_identity!(input_expr)`
macro_rules! contract_pre_zero_input_identity {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract zero_input_identity: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/layernorm-kernel-v1.yaml — DO NOT EDIT
// Contract: layernorm-kernel-v1

/// Preconditions for equation `layernorm`.
/// Call at function entry: `contract_pre_layernorm!(input_expr)`
macro_rules! contract_pre_layernorm {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract layernorm: precondition violated — input must not be empty"
        );
    }};
}

/// Postconditions for equation `layernorm`.
/// Call before return: `contract_post_layernorm!(result_expr)`
macro_rules! contract_post_layernorm {
    ($result:expr) => {{
        let _contract_result = &$result;
        debug_assert!(
            _contract_result.len() == x.len(),
            "Contract layernorm: postcondition violated — result.len() == x.len()"
        );
        debug_assert!(
            _contract_result.iter().all(|v| v.is_finite()),
            "Contract layernorm: postcondition violated — result.iter().all(|v| v.is_finite())"
        );
    }};
}

/// Preconditions for equation `statistics`.
/// Call at function entry: `contract_pre_statistics!(input_expr)`
macro_rules! contract_pre_statistics {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract statistics: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/lbfgs-kernel-v1.yaml — DO NOT EDIT
// Contract: lbfgs-kernel-v1

/// Preconditions for equation `line_search`.
/// Call at function entry: `contract_pre_line_search!(input_expr)`
macro_rules! contract_pre_line_search {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract line_search: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `secant_condition`.
/// Call at function entry: `contract_pre_secant_condition!(input_expr)`
macro_rules! contract_pre_secant_condition {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract secant_condition: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `two_loop_recursion`.
/// Call at function entry: `contract_pre_two_loop_recursion!(input_expr)`
macro_rules! contract_pre_two_loop_recursion {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract two_loop_recursion: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/learned-position-embedding-v1.yaml — DO NOT EDIT
// Contract: learned-position-embedding-v1

/// Preconditions for equation `position_embedding`.
/// Call at function entry: `contract_pre_position_embedding!(input_expr)`
macro_rules! contract_pre_position_embedding {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract position_embedding: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/linear-models-v1.yaml — DO NOT EDIT
// Contract: linear-models-v1

/// Preconditions for equation `logistic_predict_proba`.
/// Call at function entry: `contract_pre_logistic_predict_proba!(input_expr)`
macro_rules! contract_pre_logistic_predict_proba {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract logistic_predict_proba: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `ols_fit`.
/// Call at function entry: `contract_pre_ols_fit!(input_expr)`
macro_rules! contract_pre_ols_fit {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract ols_fit: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `ols_predict`.
/// Call at function entry: `contract_pre_ols_predict!(input_expr)`
macro_rules! contract_pre_ols_predict {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract ols_predict: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `r_squared_training`.
/// Call at function entry: `contract_pre_r_squared_training!(input_expr)`
macro_rules! contract_pre_r_squared_training {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract r_squared_training: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/linear-probe-classifier-v1.yaml — DO NOT EDIT
// Contract: linear-probe-classifier-v1

/// Preconditions for equation `linear_probe`.
/// Call at function entry: `contract_pre_linear_probe!(input_expr)`
macro_rules! contract_pre_linear_probe {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract linear_probe: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/linear-projection-v1.yaml — DO NOT EDIT
// Contract: linear-projection-v1

/// Preconditions for equation `linear_forward`.
/// Call at function entry: `contract_pre_linear_forward!(input_expr)`
macro_rules! contract_pre_linear_forward {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract linear_forward: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `linear_no_bias`.
/// Call at function entry: `contract_pre_linear_no_bias!(input_expr)`
macro_rules! contract_pre_linear_no_bias {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract linear_no_bias: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/lora-algebra-v1.yaml — DO NOT EDIT
// Contract: lora-algebra-v1

/// Preconditions for equation `dare_unbiased`.
/// Call at function entry: `contract_pre_dare_unbiased!(input_expr)`
macro_rules! contract_pre_dare_unbiased {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract dare_unbiased: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `eckart_young`.
/// Call at function entry: `contract_pre_eckart_young!(input_expr)`
macro_rules! contract_pre_eckart_young {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract eckart_young: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `lora_shape`.
/// Call at function entry: `contract_pre_lora_shape!(input_expr)`
macro_rules! contract_pre_lora_shape {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract lora_shape: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `shape_preservation`.
/// Call at function entry: `contract_pre_shape_preservation!(input_expr)`
macro_rules! contract_pre_shape_preservation {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract shape_preservation: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `task_vector`.
/// Call at function entry: `contract_pre_task_vector!(input_expr)`
macro_rules! contract_pre_task_vector {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract task_vector: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/loss-functions-v1.yaml — DO NOT EDIT
// Contract: loss-functions-v1

/// Preconditions for equation `bce`.
/// Call at function entry: `contract_pre_bce!(input_expr)`
macro_rules! contract_pre_bce {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract bce: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `huber`.
/// Call at function entry: `contract_pre_huber!(input_expr)`
macro_rules! contract_pre_huber {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract huber: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `l1_loss`.
/// Call at function entry: `contract_pre_l1_loss!(input_expr)`
macro_rules! contract_pre_l1_loss {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract l1_loss: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `mse_loss`.
/// Call at function entry: `contract_pre_mse_loss!(input_expr)`
macro_rules! contract_pre_mse_loss {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract mse_loss: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `nll`.
/// Call at function entry: `contract_pre_nll!(input_expr)`
macro_rules! contract_pre_nll {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract nll: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `smooth_l1`.
/// Call at function entry: `contract_pre_smooth_l1!(input_expr)`
macro_rules! contract_pre_smooth_l1 {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract smooth_l1: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/matmul-kernel-v1.yaml — DO NOT EDIT
// Contract: matmul-kernel-v1

/// Preconditions for equation `matmul`.
/// Call at function entry: `contract_pre_matmul!(input_expr)`
macro_rules! contract_pre_matmul {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract matmul: precondition violated — input must not be empty"
        );
    }};
}

/// Postconditions for equation `matmul`.
/// Call before return: `contract_post_matmul!(result_expr)`
macro_rules! contract_post_matmul {
    ($result:expr) => {{
        let _contract_result = &$result;
        debug_assert!(
            _contract_result.len() == m * n,
            "Contract matmul: postcondition violated — result.len() == m * n"
        );
        debug_assert!(
            _contract_result.iter().all(|v| v.is_finite()),
            "Contract matmul: postcondition violated — result.iter().all(|v| v.is_finite())"
        );
    }};
}

/// Preconditions for equation `quantized_dot`.
/// Call at function entry: `contract_pre_quantized_dot!(input_expr)`
macro_rules! contract_pre_quantized_dot {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract quantized_dot: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/metaheuristics-v1.yaml — DO NOT EDIT
// Contract: metaheuristics-v1

/// Preconditions for equation `best_monotone`.
/// Call at function entry: `contract_pre_best_monotone!(input_expr)`
macro_rules! contract_pre_best_monotone {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract best_monotone: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `ga_crossover`.
/// Call at function entry: `contract_pre_ga_crossover!(input_expr)`
macro_rules! contract_pre_ga_crossover {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract ga_crossover: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `pso_velocity`.
/// Call at function entry: `contract_pre_pso_velocity!(input_expr)`
macro_rules! contract_pre_pso_velocity {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract pso_velocity: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `sa_acceptance`.
/// Call at function entry: `contract_pre_sa_acceptance!(input_expr)`
macro_rules! contract_pre_sa_acceptance {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract sa_acceptance: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/metrics-classification-v1.yaml — DO NOT EDIT
// Contract: metrics-classification-v1

/// Preconditions for equation `accuracy`.
/// Call at function entry: `contract_pre_accuracy!(input_expr)`
macro_rules! contract_pre_accuracy {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract accuracy: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `confusion_matrix`.
/// Call at function entry: `contract_pre_confusion_matrix!(input_expr)`
macro_rules! contract_pre_confusion_matrix {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract confusion_matrix: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `f1_score`.
/// Call at function entry: `contract_pre_f1_score!(input_expr)`
macro_rules! contract_pre_f1_score {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract f1_score: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `precision`.
/// Call at function entry: `contract_pre_precision!(input_expr)`
macro_rules! contract_pre_precision {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract precision: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `recall`.
/// Call at function entry: `contract_pre_recall!(input_expr)`
macro_rules! contract_pre_recall {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract recall: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/metrics-clustering-v1.yaml — DO NOT EDIT
// Contract: metrics-clustering-v1

/// Preconditions for equation `inertia`.
/// Call at function entry: `contract_pre_inertia!(input_expr)`
macro_rules! contract_pre_inertia {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract inertia: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `silhouette_coefficient`.
/// Call at function entry: `contract_pre_silhouette_coefficient!(input_expr)`
macro_rules! contract_pre_silhouette_coefficient {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract silhouette_coefficient: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `silhouette_score`.
/// Call at function entry: `contract_pre_silhouette_score!(input_expr)`
macro_rules! contract_pre_silhouette_score {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract silhouette_score: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/metrics-ranking-v1.yaml — DO NOT EDIT
// Contract: metrics-ranking-v1

/// Preconditions for equation `hit_at_k`.
/// Call at function entry: `contract_pre_hit_at_k!(input_expr)`
macro_rules! contract_pre_hit_at_k {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract hit_at_k: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `mrr`.
/// Call at function entry: `contract_pre_mrr!(input_expr)`
macro_rules! contract_pre_mrr {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract mrr: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `ndcg_at_k`.
/// Call at function entry: `contract_pre_ndcg_at_k!(input_expr)`
macro_rules! contract_pre_ndcg_at_k {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract ndcg_at_k: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `reciprocal_rank`.
/// Call at function entry: `contract_pre_reciprocal_rank!(input_expr)`
macro_rules! contract_pre_reciprocal_rank {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract reciprocal_rank: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/metrics-regression-v1.yaml — DO NOT EDIT
// Contract: metrics-regression-v1

/// Preconditions for equation `mae`.
/// Call at function entry: `contract_pre_mae!(input_expr)`
macro_rules! contract_pre_mae {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract mae: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `mse`.
/// Call at function entry: `contract_pre_mse!(input_expr)`
macro_rules! contract_pre_mse {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract mse: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `r_squared`.
/// Call at function entry: `contract_pre_r_squared!(input_expr)`
macro_rules! contract_pre_r_squared {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract r_squared: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `rmse`.
/// Call at function entry: `contract_pre_rmse!(input_expr)`
macro_rules! contract_pre_rmse {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract rmse: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/model-config-algebra-v1.yaml — DO NOT EDIT
// Contract: model-config-algebra-v1

/// Preconditions for equation `bounds`.
/// Call at function entry: `contract_pre_bounds!(input_expr)`
macro_rules! contract_pre_bounds {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract bounds: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `cross_constraint`.
/// Call at function entry: `contract_pre_cross_constraint!(input_expr)`
macro_rules! contract_pre_cross_constraint {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract cross_constraint: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `divisibility`.
/// Call at function entry: `contract_pre_divisibility!(input_expr)`
macro_rules! contract_pre_divisibility {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract divisibility: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `non_degeneracy`.
/// Call at function entry: `contract_pre_non_degeneracy!(input_expr)`
macro_rules! contract_pre_non_degeneracy {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract non_degeneracy: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `ordering`.
/// Call at function entry: `contract_pre_ordering!(input_expr)`
macro_rules! contract_pre_ordering {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract ordering: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/model-metadata-bounds-v1.yaml — DO NOT EDIT
// Contract: model-metadata-bounds-v1

/// Preconditions for equation `config_bounds_check`.
/// Call at function entry: `contract_pre_config_bounds_check!(input_expr)`
macro_rules! contract_pre_config_bounds_check {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract config_bounds_check: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/mqs-scoring-v1.yaml — DO NOT EDIT
// Contract: mqs-scoring-v1

/// Preconditions for equation `mqs_composite`.
/// Call at function entry: `contract_pre_mqs_composite!(input_expr)`
macro_rules! contract_pre_mqs_composite {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract mqs_composite: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `mqs_deterministic`.
/// Call at function entry: `contract_pre_mqs_deterministic!(input_expr)`
macro_rules! contract_pre_mqs_deterministic {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract mqs_deterministic: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `mqs_grade`.
/// Call at function entry: `contract_pre_mqs_grade!(input_expr)`
macro_rules! contract_pre_mqs_grade {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract mqs_grade: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `mqs_pass_rate`.
/// Call at function entry: `contract_pre_mqs_pass_rate!(input_expr)`
macro_rules! contract_pre_mqs_pass_rate {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract mqs_pass_rate: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/naive-bayes-v1.yaml — DO NOT EDIT
// Contract: naive-bayes-v1

/// Preconditions for equation `class_prior`.
/// Call at function entry: `contract_pre_class_prior!(input_expr)`
macro_rules! contract_pre_class_prior {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract class_prior: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `gaussian_likelihood`.
/// Call at function entry: `contract_pre_gaussian_likelihood!(input_expr)`
macro_rules! contract_pre_gaussian_likelihood {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract gaussian_likelihood: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `log_posterior`.
/// Call at function entry: `contract_pre_log_posterior!(input_expr)`
macro_rules! contract_pre_log_posterior {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract log_posterior: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/online-softmax-v1.yaml — DO NOT EDIT
// Contract: online-softmax-v1

/// Preconditions for equation `online_normalizer`.
/// Call at function entry: `contract_pre_online_normalizer!(input_expr)`
macro_rules! contract_pre_online_normalizer {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract online_normalizer: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `standard_softmax`.
/// Call at function entry: `contract_pre_standard_softmax!(input_expr)`
macro_rules! contract_pre_standard_softmax {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract standard_softmax: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/optimization-v1.yaml — DO NOT EDIT
// Contract: optimization-v1

/// Preconditions for equation `cg_minimize`.
/// Call at function entry: `contract_pre_cg_minimize!(input_expr)`
macro_rules! contract_pre_cg_minimize {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract cg_minimize: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `convergence`.
/// Call at function entry: `contract_pre_convergence!(input_expr)`
macro_rules! contract_pre_convergence {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract convergence: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `line_search`.
/// Call at function entry: `contract_pre_line_search!(input_expr)`
macro_rules! contract_pre_line_search {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract line_search: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/paged-attention-v1.yaml — DO NOT EDIT
// Contract: paged-attention-v1

/// Preconditions for equation `block_allocation`.
/// Call at function entry: `contract_pre_block_allocation!(input_expr)`
macro_rules! contract_pre_block_allocation {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract block_allocation: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `block_table_lookup`.
/// Call at function entry: `contract_pre_block_table_lookup!(input_expr)`
macro_rules! contract_pre_block_table_lookup {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract block_table_lookup: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `copy_on_write`.
/// Call at function entry: `contract_pre_copy_on_write!(input_expr)`
macro_rules! contract_pre_copy_on_write {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract copy_on_write: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/paged-kv-cache-v1.yaml — DO NOT EDIT
// Contract: paged-kv-cache-v1

/// Preconditions for equation `block_allocation`.
/// Call at function entry: `contract_pre_block_allocation!(input_expr)`
macro_rules! contract_pre_block_allocation {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract block_allocation: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `block_table_invariant`.
/// Call at function entry: `contract_pre_block_table_invariant!(input_expr)`
macro_rules! contract_pre_block_table_invariant {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract block_table_invariant: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `fragmentation_free`.
/// Call at function entry: `contract_pre_fragmentation_free!(input_expr)`
macro_rules! contract_pre_fragmentation_free {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract fragmentation_free: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `graph_compatibility`.
/// Call at function entry: `contract_pre_graph_compatibility!(input_expr)`
macro_rules! contract_pre_graph_compatibility {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract graph_compatibility: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `paged_contiguous_equivalence`.
/// Call at function entry: `contract_pre_paged_contiguous_equivalence!(input_expr)`
macro_rules! contract_pre_paged_contiguous_equivalence {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(!_contract_input.is_empty(),
            "Contract paged_contiguous_equivalence: precondition violated — input must not be empty");
    }};
}

/// Preconditions for equation `slot_mapping`.
/// Call at function entry: `contract_pre_slot_mapping!(input_expr)`
macro_rules! contract_pre_slot_mapping {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract slot_mapping: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/pagerank-kernel-v1.yaml — DO NOT EDIT
// Contract: pagerank-kernel-v1

/// Preconditions for equation `pagerank`.
/// Call at function entry: `contract_pre_pagerank!(input_expr)`
macro_rules! contract_pre_pagerank {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract pagerank: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `power_iteration`.
/// Call at function entry: `contract_pre_power_iteration!(input_expr)`
macro_rules! contract_pre_power_iteration {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract power_iteration: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/pca-v1.yaml — DO NOT EDIT
// Contract: pca-v1

/// Preconditions for equation `explained_variance`.
/// Call at function entry: `contract_pre_explained_variance!(input_expr)`
macro_rules! contract_pre_explained_variance {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract explained_variance: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `pca_transform`.
/// Call at function entry: `contract_pre_pca_transform!(input_expr)`
macro_rules! contract_pre_pca_transform {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract pca_transform: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `reconstruction`.
/// Call at function entry: `contract_pre_reconstruction!(input_expr)`
macro_rules! contract_pre_reconstruction {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract reconstruction: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/performance-grading-v1.yaml — DO NOT EDIT
// Contract: performance-grading-v1

/// Preconditions for equation `concrete_instance`.
/// Call at function entry: `contract_pre_concrete_instance!(input_expr)`
macro_rules! contract_pre_concrete_instance {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract concrete_instance: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `efficiency_grade`.
/// Call at function entry: `contract_pre_efficiency_grade!(input_expr)`
macro_rules! contract_pre_efficiency_grade {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract efficiency_grade: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `llamacpp_parity`.
/// Call at function entry: `contract_pre_llamacpp_parity!(input_expr)`
macro_rules! contract_pre_llamacpp_parity {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract llamacpp_parity: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `ollama_parity`.
/// Call at function entry: `contract_pre_ollama_parity!(input_expr)`
macro_rules! contract_pre_ollama_parity {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract ollama_parity: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `vllm_parity`.
/// Call at function entry: `contract_pre_vllm_parity!(input_expr)`
macro_rules! contract_pre_vllm_parity {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract vllm_parity: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/preprocessing-normalization-v1.yaml — DO NOT EDIT
// Contract: preprocessing-normalization-v1

/// Preconditions for equation `minmax_scaler`.
/// Call at function entry: `contract_pre_minmax_scaler!(input_expr)`
macro_rules! contract_pre_minmax_scaler {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract minmax_scaler: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `robust_scaler`.
/// Call at function entry: `contract_pre_robust_scaler!(input_expr)`
macro_rules! contract_pre_robust_scaler {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract robust_scaler: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `standard_scaler`.
/// Call at function entry: `contract_pre_standard_scaler!(input_expr)`
macro_rules! contract_pre_standard_scaler {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract standard_scaler: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/ptx-target-parity-v1.yaml — DO NOT EDIT
// Contract: ptx-target-parity-v1

/// Preconditions for equation `jit_compilation_success`.
/// Call at function entry: `contract_pre_jit_compilation_success!(input_expr)`
macro_rules! contract_pre_jit_compilation_success {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract jit_compilation_success: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `no_hardcoded_targets`.
/// Call at function entry: `contract_pre_no_hardcoded_targets!(input_expr)`
macro_rules! contract_pre_no_hardcoded_targets {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract no_hardcoded_targets: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `target_parity`.
/// Call at function entry: `contract_pre_target_parity!(input_expr)`
macro_rules! contract_pre_target_parity {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract target_parity: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/q4k-q6k-superblock-v1.yaml — DO NOT EDIT
// Contract: q4k-q6k-superblock-v1

/// Preconditions for equation `bsum`.
/// Call at function entry: `contract_pre_bsum!(input_expr)`
macro_rules! contract_pre_bsum {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract bsum: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `dequantization`.
/// Call at function entry: `contract_pre_dequantization!(input_expr)`
macro_rules! contract_pre_dequantization {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract dequantization: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `q4k_superblock`.
/// Call at function entry: `contract_pre_q4k_superblock!(input_expr)`
macro_rules! contract_pre_q4k_superblock {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract q4k_superblock: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `q6k_superblock`.
/// Call at function entry: `contract_pre_q6k_superblock!(input_expr)`
macro_rules! contract_pre_q6k_superblock {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract q6k_superblock: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `total_bytes`.
/// Call at function entry: `contract_pre_total_bytes!(input_expr)`
macro_rules! contract_pre_total_bytes {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract total_bytes: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/qk-norm-apr-loader-v1.yaml — DO NOT EDIT
// Contract: qk-norm-apr-loader-v1

/// Preconditions for equation `qk_norm_load`.
/// Call at function entry: `contract_pre_qk_norm_load!(input_expr)`
macro_rules! contract_pre_qk_norm_load {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract qk_norm_load: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/qk-norm-v1.yaml — DO NOT EDIT
// Contract: qk-norm-v1

/// Preconditions for equation `qk_rmsnorm`.
/// Call at function entry: `contract_pre_qk_rmsnorm!(input_expr)`
macro_rules! contract_pre_qk_rmsnorm {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract qk_rmsnorm: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/quantization-ordering-v1.yaml — DO NOT EDIT
// Contract: quantization-ordering-v1

/// Preconditions for equation `alpha_scaling`.
/// Call at function entry: `contract_pre_alpha_scaling!(input_expr)`
macro_rules! contract_pre_alpha_scaling {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract alpha_scaling: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `bytes_per_param`.
/// Call at function entry: `contract_pre_bytes_per_param!(input_expr)`
macro_rules! contract_pre_bytes_per_param {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract bytes_per_param: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `dropout_expectation`.
/// Call at function entry: `contract_pre_dropout_expectation!(input_expr)`
macro_rules! contract_pre_dropout_expectation {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract dropout_expectation: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `size_ordering`.
/// Call at function entry: `contract_pre_size_ordering!(input_expr)`
macro_rules! contract_pre_size_ordering {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract size_ordering: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/qwen2-e2e-verification-v1.yaml — DO NOT EDIT
// Contract: qwen2-e2e-verification-v1

/// Preconditions for equation `contract_composition`.
/// Call at function entry: `contract_pre_contract_composition!(input_expr)`
macro_rules! contract_pre_contract_composition {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract contract_composition: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `flops_per_token`.
/// Call at function entry: `contract_pre_flops_per_token!(input_expr)`
macro_rules! contract_pre_flops_per_token {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract flops_per_token: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `memory_breakdown`.
/// Call at function entry: `contract_pre_memory_breakdown!(input_expr)`
macro_rules! contract_pre_memory_breakdown {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract memory_breakdown: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `model_parameter_count`.
/// Call at function entry: `contract_pre_model_parameter_count!(input_expr)`
macro_rules! contract_pre_model_parameter_count {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract model_parameter_count: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `throughput_model`.
/// Call at function entry: `contract_pre_throughput_model!(input_expr)`
macro_rules! contract_pre_throughput_model {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract throughput_model: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `verification_ladder`.
/// Call at function entry: `contract_pre_verification_ladder!(input_expr)`
macro_rules! contract_pre_verification_ladder {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract verification_ladder: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/qwen2-shapes-v1.yaml — DO NOT EDIT
// Contract: qwen2-shapes-v1

/// Preconditions for equation `head_dim_consistency`.
/// Call at function entry: `contract_pre_head_dim_consistency!(input_expr)`
macro_rules! contract_pre_head_dim_consistency {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract head_dim_consistency: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `kv_projection_shape`.
/// Call at function entry: `contract_pre_kv_projection_shape!(input_expr)`
macro_rules! contract_pre_kv_projection_shape {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract kv_projection_shape: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `o_projection_transpose`.
/// Call at function entry: `contract_pre_o_projection_transpose!(input_expr)`
macro_rules! contract_pre_o_projection_transpose {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract o_projection_transpose: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `q_projection_shape`.
/// Call at function entry: `contract_pre_q_projection_shape!(input_expr)`
macro_rules! contract_pre_q_projection_shape {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract q_projection_shape: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `rope_frequency`.
/// Call at function entry: `contract_pre_rope_frequency!(input_expr)`
macro_rules! contract_pre_rope_frequency {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract rope_frequency: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `swiglu_ratio`.
/// Call at function entry: `contract_pre_swiglu_ratio!(input_expr)`
macro_rules! contract_pre_swiglu_ratio {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract swiglu_ratio: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/qwen3-e2e-verification-v1.yaml — DO NOT EDIT
// Contract: qwen3-e2e-verification-v1

/// Preconditions for equation `contract_composition`.
/// Call at function entry: `contract_pre_contract_composition!(input_expr)`
macro_rules! contract_pre_contract_composition {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract contract_composition: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `flops_per_token`.
/// Call at function entry: `contract_pre_flops_per_token!(input_expr)`
macro_rules! contract_pre_flops_per_token {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract flops_per_token: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `memory_breakdown`.
/// Call at function entry: `contract_pre_memory_breakdown!(input_expr)`
macro_rules! contract_pre_memory_breakdown {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract memory_breakdown: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `model_parameter_count`.
/// Call at function entry: `contract_pre_model_parameter_count!(input_expr)`
macro_rules! contract_pre_model_parameter_count {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract model_parameter_count: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `throughput_model`.
/// Call at function entry: `contract_pre_throughput_model!(input_expr)`
macro_rules! contract_pre_throughput_model {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract throughput_model: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `verification_ladder`.
/// Call at function entry: `contract_pre_verification_ladder!(input_expr)`
macro_rules! contract_pre_verification_ladder {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract verification_ladder: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/qwen3-shapes-v1.yaml — DO NOT EDIT
// Contract: qwen3-shapes-v1

/// Preconditions for equation `head_dim_consistency`.
/// Call at function entry: `contract_pre_head_dim_consistency!(input_expr)`
macro_rules! contract_pre_head_dim_consistency {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract head_dim_consistency: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `kv_projection_shape`.
/// Call at function entry: `contract_pre_kv_projection_shape!(input_expr)`
macro_rules! contract_pre_kv_projection_shape {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract kv_projection_shape: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `o_projection_transpose`.
/// Call at function entry: `contract_pre_o_projection_transpose!(input_expr)`
macro_rules! contract_pre_o_projection_transpose {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract o_projection_transpose: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `q_projection_shape`.
/// Call at function entry: `contract_pre_q_projection_shape!(input_expr)`
macro_rules! contract_pre_q_projection_shape {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract q_projection_shape: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `rope_frequency`.
/// Call at function entry: `contract_pre_rope_frequency!(input_expr)`
macro_rules! contract_pre_rope_frequency {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract rope_frequency: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `swiglu_ratio`.
/// Call at function entry: `contract_pre_swiglu_ratio!(input_expr)`
macro_rules! contract_pre_swiglu_ratio {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract swiglu_ratio: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/qwen35-e2e-verification-v1.yaml — DO NOT EDIT
// Contract: qwen35-e2e-verification-v1

/// Preconditions for equation `contract_composition`.
/// Call at function entry: `contract_pre_contract_composition!(input_expr)`
macro_rules! contract_pre_contract_composition {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract contract_composition: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `flops_per_token`.
/// Call at function entry: `contract_pre_flops_per_token!(input_expr)`
macro_rules! contract_pre_flops_per_token {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract flops_per_token: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `memory_breakdown`.
/// Call at function entry: `contract_pre_memory_breakdown!(input_expr)`
macro_rules! contract_pre_memory_breakdown {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract memory_breakdown: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `model_parameter_count`.
/// Call at function entry: `contract_pre_model_parameter_count!(input_expr)`
macro_rules! contract_pre_model_parameter_count {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract model_parameter_count: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `throughput_model`.
/// Call at function entry: `contract_pre_throughput_model!(input_expr)`
macro_rules! contract_pre_throughput_model {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract throughput_model: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `verification_ladder`.
/// Call at function entry: `contract_pre_verification_ladder!(input_expr)`
macro_rules! contract_pre_verification_ladder {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract verification_ladder: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/qwen35-hybrid-forward-v1.yaml — DO NOT EDIT
// Contract: qwen35-hybrid-forward-v1

/// Preconditions for equation `activation_magnitude`.
/// Call at function entry: `contract_pre_activation_magnitude!(input_expr)`
macro_rules! contract_pre_activation_magnitude {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract activation_magnitude: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `attention_sublayer`.
/// Call at function entry: `contract_pre_attention_sublayer!(input_expr)`
macro_rules! contract_pre_attention_sublayer {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract attention_sublayer: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `ffn_sublayer`.
/// Call at function entry: `contract_pre_ffn_sublayer!(input_expr)`
macro_rules! contract_pre_ffn_sublayer {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract ffn_sublayer: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `gdn_sublayer`.
/// Call at function entry: `contract_pre_gdn_sublayer!(input_expr)`
macro_rules! contract_pre_gdn_sublayer {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract gdn_sublayer: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `gradient_flow`.
/// Call at function entry: `contract_pre_gradient_flow!(input_expr)`
macro_rules! contract_pre_gradient_flow {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract gradient_flow: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `hybrid_block`.
/// Call at function entry: `contract_pre_hybrid_block!(input_expr)`
macro_rules! contract_pre_hybrid_block {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract hybrid_block: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/qwen35-shapes-v1.yaml — DO NOT EDIT
// Contract: qwen35-shapes-v1

/// Preconditions for equation `kv_projection_shape`.
/// Call at function entry: `contract_pre_kv_projection_shape!(input_expr)`
macro_rules! contract_pre_kv_projection_shape {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract kv_projection_shape: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `o_projection_transpose`.
/// Call at function entry: `contract_pre_o_projection_transpose!(input_expr)`
macro_rules! contract_pre_o_projection_transpose {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract o_projection_transpose: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `q_projection_shape`.
/// Call at function entry: `contract_pre_q_projection_shape!(input_expr)`
macro_rules! contract_pre_q_projection_shape {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract q_projection_shape: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `rope_frequency`.
/// Call at function entry: `contract_pre_rope_frequency!(input_expr)`
macro_rules! contract_pre_rope_frequency {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract rope_frequency: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `swiglu_ratio`.
/// Call at function entry: `contract_pre_swiglu_ratio!(input_expr)`
macro_rules! contract_pre_swiglu_ratio {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract swiglu_ratio: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/qwen3moe-e2e-verification-v1.yaml — DO NOT EDIT
// Contract: qwen3moe-e2e-verification-v1

/// Preconditions for equation `active_parameter_count`.
/// Call at function entry: `contract_pre_active_parameter_count!(input_expr)`
macro_rules! contract_pre_active_parameter_count {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract active_parameter_count: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `contract_composition`.
/// Call at function entry: `contract_pre_contract_composition!(input_expr)`
macro_rules! contract_pre_contract_composition {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract contract_composition: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `flops_per_token`.
/// Call at function entry: `contract_pre_flops_per_token!(input_expr)`
macro_rules! contract_pre_flops_per_token {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract flops_per_token: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `memory_breakdown`.
/// Call at function entry: `contract_pre_memory_breakdown!(input_expr)`
macro_rules! contract_pre_memory_breakdown {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract memory_breakdown: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `model_parameter_count`.
/// Call at function entry: `contract_pre_model_parameter_count!(input_expr)`
macro_rules! contract_pre_model_parameter_count {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract model_parameter_count: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `throughput_model`.
/// Call at function entry: `contract_pre_throughput_model!(input_expr)`
macro_rules! contract_pre_throughput_model {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract throughput_model: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `verification_ladder`.
/// Call at function entry: `contract_pre_verification_ladder!(input_expr)`
macro_rules! contract_pre_verification_ladder {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract verification_ladder: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/qwen3moe-shapes-v1.yaml — DO NOT EDIT
// Contract: qwen3moe-shapes-v1

/// Preconditions for equation `kv_projection_shape`.
/// Call at function entry: `contract_pre_kv_projection_shape!(input_expr)`
macro_rules! contract_pre_kv_projection_shape {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract kv_projection_shape: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `moe_expert_shape`.
/// Call at function entry: `contract_pre_moe_expert_shape!(input_expr)`
macro_rules! contract_pre_moe_expert_shape {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract moe_expert_shape: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `moe_router_shape`.
/// Call at function entry: `contract_pre_moe_router_shape!(input_expr)`
macro_rules! contract_pre_moe_router_shape {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract moe_router_shape: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `o_projection_transpose`.
/// Call at function entry: `contract_pre_o_projection_transpose!(input_expr)`
macro_rules! contract_pre_o_projection_transpose {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract o_projection_transpose: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `q_projection_shape`.
/// Call at function entry: `contract_pre_q_projection_shape!(input_expr)`
macro_rules! contract_pre_q_projection_shape {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract q_projection_shape: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `rope_frequency`.
/// Call at function entry: `contract_pre_rope_frequency!(input_expr)`
macro_rules! contract_pre_rope_frequency {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract rope_frequency: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `swiglu_ratio`.
/// Call at function entry: `contract_pre_swiglu_ratio!(input_expr)`
macro_rules! contract_pre_swiglu_ratio {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract swiglu_ratio: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/random-forest-v1.yaml — DO NOT EDIT
// Contract: random-forest-v1

/// Preconditions for equation `bootstrap_sample`.
/// Call at function entry: `contract_pre_bootstrap_sample!(input_expr)`
macro_rules! contract_pre_bootstrap_sample {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract bootstrap_sample: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `ensemble_size`.
/// Call at function entry: `contract_pre_ensemble_size!(input_expr)`
macro_rules! contract_pre_ensemble_size {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract ensemble_size: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `majority_vote`.
/// Call at function entry: `contract_pre_majority_vote!(input_expr)`
macro_rules! contract_pre_majority_vote {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract majority_vote: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `predict`.
/// Call at function entry: `contract_pre_predict!(input_expr)`
macro_rules! contract_pre_predict {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract predict: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/rmsnorm-kernel-v1.yaml — DO NOT EDIT
// Contract: rmsnorm-kernel-v1

/// Preconditions for equation `rmsnorm`.
/// Call at function entry: `contract_pre_rmsnorm!(input_expr)`
macro_rules! contract_pre_rmsnorm {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract rmsnorm: precondition violated — input must not be empty"
        );
    }};
}

/// Postconditions for equation `rmsnorm`.
/// Call before return: `contract_post_rmsnorm!(result_expr)`
macro_rules! contract_post_rmsnorm {
    ($result:expr) => {{
        let _contract_result = &$result;
        debug_assert!(
            _contract_result.len() == x.len(),
            "Contract rmsnorm: postcondition violated — result.len() == x.len()"
        );
        debug_assert!(
            _contract_result.iter().all(|v| v.is_finite()),
            "Contract rmsnorm: postcondition violated — result.iter().all(|v| v.is_finite())"
        );
    }};
}

// Auto-generated from contracts/roofline-model-v1.yaml — DO NOT EDIT
// Contract: roofline-model-v1

/// Preconditions for equation `bandwidth_ceiling`.
/// Call at function entry: `contract_pre_bandwidth_ceiling!(input_expr)`
macro_rules! contract_pre_bandwidth_ceiling {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract bandwidth_ceiling: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `compute_ceiling`.
/// Call at function entry: `contract_pre_compute_ceiling!(input_expr)`
macro_rules! contract_pre_compute_ceiling {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract compute_ceiling: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `model_bytes`.
/// Call at function entry: `contract_pre_model_bytes!(input_expr)`
macro_rules! contract_pre_model_bytes {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract model_bytes: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `throughput_bound`.
/// Call at function entry: `contract_pre_throughput_bound!(input_expr)`
macro_rules! contract_pre_throughput_bound {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract throughput_bound: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/rope-extrapolation-v1.yaml — DO NOT EDIT
// Contract: rope-extrapolation-v1

/// Preconditions for equation `base_frequency`.
/// Call at function entry: `contract_pre_base_frequency!(input_expr)`
macro_rules! contract_pre_base_frequency {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract base_frequency: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `linear_interpolation`.
/// Call at function entry: `contract_pre_linear_interpolation!(input_expr)`
macro_rules! contract_pre_linear_interpolation {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract linear_interpolation: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `ntk_scaled_base`.
/// Call at function entry: `contract_pre_ntk_scaled_base!(input_expr)`
macro_rules! contract_pre_ntk_scaled_base {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract ntk_scaled_base: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `rotation_matrix`.
/// Call at function entry: `contract_pre_rotation_matrix!(input_expr)`
macro_rules! contract_pre_rotation_matrix {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract rotation_matrix: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `yarn_mixed_frequency`.
/// Call at function entry: `contract_pre_yarn_mixed_frequency!(input_expr)`
macro_rules! contract_pre_yarn_mixed_frequency {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract yarn_mixed_frequency: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `yarn_ramp`.
/// Call at function entry: `contract_pre_yarn_ramp!(input_expr)`
macro_rules! contract_pre_yarn_ramp {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract yarn_ramp: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/rope-kernel-v1.yaml — DO NOT EDIT
// Contract: rope-kernel-v1

/// Preconditions for equation `rope`.
/// Call at function entry: `contract_pre_rope!(input_expr)`
macro_rules! contract_pre_rope {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract rope: precondition violated — input must not be empty"
        );
    }};
}

/// Postconditions for equation `rope`.
/// Call before return: `contract_post_rope!(result_expr)`
macro_rules! contract_post_rope {
    ($result:expr) => {{
        let _contract_result = &$result;
        debug_assert!(
            _contract_result.len() == x.len(),
            "Contract rope: postcondition violated — result.len() == x.len()"
        );
        debug_assert!(
            _contract_result.iter().all(|v| v.is_finite()),
            "Contract rope: postcondition violated — result.iter().all(|v| v.is_finite())"
        );
    }};
}

// Auto-generated from contracts/safetensors-cpu-dispatch-v1.yaml — DO NOT EDIT
// Contract: safetensors-cpu-dispatch-v1

/// Preconditions for equation `format_parity`.
/// Call at function entry: `contract_pre_format_parity!(input_expr)`
macro_rules! contract_pre_format_parity {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract format_parity: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/sampling-algorithms-v1.yaml — DO NOT EDIT
// Contract: sampling-algorithms-v1

/// Preconditions for equation `greedy`.
/// Call at function entry: `contract_pre_greedy!(input_expr)`
macro_rules! contract_pre_greedy {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract greedy: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `temperature`.
/// Call at function entry: `contract_pre_temperature!(input_expr)`
macro_rules! contract_pre_temperature {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract temperature: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `top_k`.
/// Call at function entry: `contract_pre_top_k!(input_expr)`
macro_rules! contract_pre_top_k {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract top_k: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `top_p`.
/// Call at function entry: `contract_pre_top_p!(input_expr)`
macro_rules! contract_pre_top_p {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract top_p: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/serialization-v1.yaml — DO NOT EDIT
// Contract: serialization-v1

/// Preconditions for equation `serialization`.
/// Call at function entry: `contract_pre_serialization!(input_expr)`
macro_rules! contract_pre_serialization {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract serialization: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/shannon-entropy-v1.yaml — DO NOT EDIT
// Contract: shannon-entropy-v1

/// Preconditions for equation `entropy`.
/// Call at function entry: `contract_pre_entropy!(input_expr)`
macro_rules! contract_pre_entropy {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract entropy: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `uniform_entropy`.
/// Call at function entry: `contract_pre_uniform_entropy!(input_expr)`
macro_rules! contract_pre_uniform_entropy {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract uniform_entropy: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/silu-kernel-v1.yaml — DO NOT EDIT
// Contract: silu-kernel-v1

/// Preconditions for equation `sigmoid`.
/// Call at function entry: `contract_pre_sigmoid!(input_expr)`
macro_rules! contract_pre_sigmoid {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract sigmoid: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `silu`.
/// Call at function entry: `contract_pre_silu!(input_expr)`
macro_rules! contract_pre_silu {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract silu: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/sliding-window-attention-v1.yaml — DO NOT EDIT
// Contract: sliding-window-attention-v1

/// Preconditions for equation `attention_sparsity`.
/// Call at function entry: `contract_pre_attention_sparsity!(input_expr)`
macro_rules! contract_pre_attention_sparsity {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract attention_sparsity: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `causal_window_mask`.
/// Call at function entry: `contract_pre_causal_window_mask!(input_expr)`
macro_rules! contract_pre_causal_window_mask {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract causal_window_mask: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `effective_context`.
/// Call at function entry: `contract_pre_effective_context!(input_expr)`
macro_rules! contract_pre_effective_context {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract effective_context: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `multi_layer_receptive_field`.
/// Call at function entry: `contract_pre_multi_layer_receptive_field!(input_expr)`
macro_rules! contract_pre_multi_layer_receptive_field {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(!_contract_input.is_empty(),
            "Contract multi_layer_receptive_field: precondition violated — input must not be empty");
    }};
}

/// Preconditions for equation `window_mask`.
/// Call at function entry: `contract_pre_window_mask!(input_expr)`
macro_rules! contract_pre_window_mask {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract window_mask: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/softmax-kernel-v1.yaml — DO NOT EDIT
// Contract: softmax-kernel-v1

/// Preconditions for equation `softmax`.
/// Call at function entry: `contract_pre_softmax!(input_expr)`
macro_rules! contract_pre_softmax {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract softmax: precondition violated — input must not be empty"
        );
    }};
}

/// Postconditions for equation `softmax`.
/// Call before return: `contract_post_softmax!(result_expr)`
macro_rules! contract_post_softmax {
    ($result:expr) => {{
        let _contract_result = &$result;
        debug_assert!(_contract_result.len() == x.len(), "Contract softmax: postcondition violated — result.len() == x.len()");
        debug_assert!(_contract_result.iter().all(|v| *v > 0.0), "Contract softmax: postcondition violated — result.iter().all(|v| *v > 0.0)");
        debug_assert!((_contract_result.iter().sum::<f32>() - 1.0).abs() < 1e-5, "Contract softmax: postcondition violated — (result.iter().sum::<f32>() - 1.0).abs() < 1e-5");
    }};
}

// Auto-generated from contracts/special-tokens-registry-v1.yaml — DO NOT EDIT
// Contract: special-tokens-registry-v1

/// Preconditions for equation `token_bounds`.
/// Call at function entry: `contract_pre_token_bounds!(input_expr)`
macro_rules! contract_pre_token_bounds {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract token_bounds: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/speculative-decoding-v1.yaml — DO NOT EDIT
// Contract: speculative-decoding-v1

/// Preconditions for equation `acceptance_probability`.
/// Call at function entry: `contract_pre_acceptance_probability!(input_expr)`
macro_rules! contract_pre_acceptance_probability {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract acceptance_probability: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `output_equivalence`.
/// Call at function entry: `contract_pre_output_equivalence!(input_expr)`
macro_rules! contract_pre_output_equivalence {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract output_equivalence: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `token_acceptance`.
/// Call at function entry: `contract_pre_token_acceptance!(input_expr)`
macro_rules! contract_pre_token_acceptance {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract token_acceptance: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/ssm-kernel-v1.yaml — DO NOT EDIT
// Contract: ssm-kernel-v1

/// Preconditions for equation `selective_gate`.
/// Call at function entry: `contract_pre_selective_gate!(input_expr)`
macro_rules! contract_pre_selective_gate {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract selective_gate: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `ssm_discretize`.
/// Call at function entry: `contract_pre_ssm_discretize!(input_expr)`
macro_rules! contract_pre_ssm_discretize {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract ssm_discretize: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `ssm_scan`.
/// Call at function entry: `contract_pre_ssm_scan!(input_expr)`
macro_rules! contract_pre_ssm_scan {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract ssm_scan: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/streaming-tpot-v1.yaml — DO NOT EDIT
// Contract: streaming-tpot-v1

/// Preconditions for equation `tpot_definition`.
/// Call at function entry: `contract_pre_tpot_definition!(input_expr)`
macro_rules! contract_pre_tpot_definition {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract tpot_definition: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/svm-v1.yaml — DO NOT EDIT
// Contract: svm-v1

/// Preconditions for equation `decision_function`.
/// Call at function entry: `contract_pre_decision_function!(input_expr)`
macro_rules! contract_pre_decision_function {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract decision_function: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `hinge_loss`.
/// Call at function entry: `contract_pre_hinge_loss!(input_expr)`
macro_rules! contract_pre_hinge_loss {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract hinge_loss: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `margin`.
/// Call at function entry: `contract_pre_margin!(input_expr)`
macro_rules! contract_pre_margin {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract margin: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `svm_predict`.
/// Call at function entry: `contract_pre_svm_predict!(input_expr)`
macro_rules! contract_pre_svm_predict {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract svm_predict: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/swiglu-kernel-v1.yaml — DO NOT EDIT
// Contract: swiglu-kernel-v1

/// Preconditions for equation `silu`.
/// Call at function entry: `contract_pre_silu!(input_expr)`
macro_rules! contract_pre_silu {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract silu: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `swiglu`.
/// Call at function entry: `contract_pre_swiglu!(input_expr)`
macro_rules! contract_pre_swiglu {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract swiglu: precondition violated — input must not be empty"
        );
    }};
}

/// Postconditions for equation `swiglu`.
/// Call before return: `contract_post_swiglu!(result_expr)`
macro_rules! contract_post_swiglu {
    ($result:expr) => {{
        let _contract_result = &$result;
        debug_assert!(
            _contract_result.len() == x.len(),
            "Contract swiglu: postcondition violated — result.len() == x.len()"
        );
        debug_assert!(
            _contract_result.iter().all(|v| v.is_finite()),
            "Contract swiglu: postcondition violated — result.iter().all(|v| v.is_finite())"
        );
    }};
}

// Auto-generated from contracts/tensor-inventory-v1.yaml — DO NOT EDIT
// Contract: tensor-inventory-v1

/// Preconditions for equation `architecture_delta`.
/// Call at function entry: `contract_pre_architecture_delta!(input_expr)`
macro_rules! contract_pre_architecture_delta {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract architecture_delta: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `parameter_decomposition`.
/// Call at function entry: `contract_pre_parameter_decomposition!(input_expr)`
macro_rules! contract_pre_parameter_decomposition {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract parameter_decomposition: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `quantization_bytes`.
/// Call at function entry: `contract_pre_quantization_bytes!(input_expr)`
macro_rules! contract_pre_quantization_bytes {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract quantization_bytes: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `tensor_count`.
/// Call at function entry: `contract_pre_tensor_count!(input_expr)`
macro_rules! contract_pre_tensor_count {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract tensor_count: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `tied_embeddings`.
/// Call at function entry: `contract_pre_tied_embeddings!(input_expr)`
macro_rules! contract_pre_tied_embeddings {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract tied_embeddings: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/tensor-names-v1.yaml — DO NOT EDIT
// Contract: tensor-names-v1

/// Preconditions for equation `architecture_normalization`.
/// Call at function entry: `contract_pre_architecture_normalization!(input_expr)`
macro_rules! contract_pre_architecture_normalization {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract architecture_normalization: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `name_resolution`.
/// Call at function entry: `contract_pre_name_resolution!(input_expr)`
macro_rules! contract_pre_name_resolution {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract name_resolution: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/tensor-shape-flow-v1.yaml — DO NOT EDIT
// Contract: tensor-shape-flow-v1

/// Preconditions for equation `gqa_grouping`.
/// Call at function entry: `contract_pre_gqa_grouping!(input_expr)`
macro_rules! contract_pre_gqa_grouping {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract gqa_grouping: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `lm_head`.
/// Call at function entry: `contract_pre_lm_head!(input_expr)`
macro_rules! contract_pre_lm_head {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract lm_head: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `qkv_projection`.
/// Call at function entry: `contract_pre_qkv_projection!(input_expr)`
macro_rules! contract_pre_qkv_projection {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract qkv_projection: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `residual`.
/// Call at function entry: `contract_pre_residual!(input_expr)`
macro_rules! contract_pre_residual {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract residual: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `swiglu_shape`.
/// Call at function entry: `contract_pre_swiglu_shape!(input_expr)`
macro_rules! contract_pre_swiglu_shape {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract swiglu_shape: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/tied-embeddings-v1.yaml — DO NOT EDIT
// Contract: tied-embeddings-v1

/// Preconditions for equation `tied_lm_head`.
/// Call at function entry: `contract_pre_tied_lm_head!(input_expr)`
macro_rules! contract_pre_tied_lm_head {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract tied_lm_head: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/transpose-kernel-v1.yaml — DO NOT EDIT
// Contract: transpose-kernel-v1

/// Preconditions for equation `transpose`.
/// Call at function entry: `contract_pre_transpose!(input_expr)`
macro_rules! contract_pre_transpose {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract transpose: precondition violated — input must not be empty"
        );
    }};
}

// Auto-generated from contracts/validated-tensor-v1.yaml — DO NOT EDIT
// Contract: validated-tensor-v1

/// Preconditions for equation `density_gate`.
/// Call at function entry: `contract_pre_density_gate!(input_expr)`
macro_rules! contract_pre_density_gate {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract density_gate: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `l2_norm_nondegeneracy`.
/// Call at function entry: `contract_pre_l2_norm_nondegeneracy!(input_expr)`
macro_rules! contract_pre_l2_norm_nondegeneracy {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract l2_norm_nondegeneracy: precondition violated — input must not be empty"
        );
    }};
}

/// Preconditions for equation `nan_inf_rejection`.
/// Call at function entry: `contract_pre_nan_inf_rejection!(input_expr)`
macro_rules! contract_pre_nan_inf_rejection {
    ($input:expr) => {{
        let _contract_input = &$input;
        debug_assert!(
            !_contract_input.is_empty(),
            "Contract nan_inf_rejection: precondition violated — input must not be empty"
        );
    }};
}

// Total: 441 preconditions, 18 postconditions from 131 contracts
