// Auto-generated contract assertions from YAML — DO NOT EDIT.
// Zero cost in release builds (debug_assert!).
// Regenerate: pv codegen contracts/ -o src/generated_contracts.rs
// Include:   #[macro_use] #[allow(unused_macros)] mod generated_contracts;

// Auto-generated from contracts/cli-transpile-v1.yaml — DO NOT EDIT
// Contract: cli-transpile-v1

/// Preconditions for equation `exit_code_dispatch`.
/// Domain-specific. Call: `contract_pre_exit_code_dispatch!(slice_expr)`
macro_rules! contract_pre_exit_code_dispatch {
    () => {{}};
    ($input:expr) => {{
        let _pv_args = &$input;
        debug_assert!(
            _pv_args.len() >= 2,
            "Contract exit_code_dispatch: precondition violated — args.len() >= 2"
        );
        debug_assert!(
            _pv_args[0] == "transpile",
            "Contract exit_code_dispatch: precondition violated — args[0] == \"transpile\""
        );
    }};
}

/// Invariants for equation `exit_code_dispatch`.
/// Check after computation: `contract_inv_exit_code_dispatch!(result_expr)`
macro_rules! contract_inv_exit_code_dispatch {
    () => {{}};
    ($result:expr) => {{
        let _contract_result = &$result;
    }};
}

/// Preconditions for equation `input_validation`.
/// Call at function entry: `contract_pre_input_validation!(input_expr)`
macro_rules! contract_pre_input_validation {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Invariants for equation `input_validation`.
/// Check after computation: `contract_inv_input_validation!(result_expr)`
macro_rules! contract_inv_input_validation {
    () => {{}};
    ($result:expr) => {{
        let _contract_result = &$result;
    }};
}

/// Preconditions for equation `output_validity`.
/// Domain-specific. Call: `contract_pre_output_validity!(slice_expr)`
macro_rules! contract_pre_output_validity {
    () => {{}};
    ($input:expr) => {{
        let _pv_rust_source = &$input;
        debug_assert!(
            !_pv_rust_source.is_empty(),
            "Contract output_validity: precondition violated — !rust_source.is_empty()"
        );
        debug_assert!(
            _pv_rust_source.len() <= 10_000_000,
            "Contract output_validity: precondition violated — rust_source.len() <= 10_000_000"
        );
    }};
}

/// Invariants for equation `output_validity`.
/// Check after computation: `contract_inv_output_validity!(result_expr)`
macro_rules! contract_inv_output_validity {
    () => {{}};
    ($result:expr) => {{
        let _contract_result = &$result;
    }};
}

/// Preconditions for equation `transpilation_determinism`.
/// Call at function entry: `contract_pre_transpilation_determinism!(input_expr)`
macro_rules! contract_pre_transpilation_determinism {
    () => {{}};
    ($input:expr) => {{
        let _contract_input = &$input;
    }};
}

/// Invariants for equation `transpilation_determinism`.
/// Check after computation: `contract_inv_transpilation_determinism!(result_expr)`
macro_rules! contract_inv_transpilation_determinism {
    () => {{}};
    ($result:expr) => {{
        let _contract_result = &$result;
    }};
}

// Auto-generated from contracts/configuration-v1.yaml — DO NOT EDIT
// Contract: configuration-v1

/// Preconditions for equation `configuration`.
/// Domain-specific. Call: `contract_pre_configuration!(slice_expr)`
macro_rules! contract_pre_configuration {
    () => {{}};
    ($input:expr) => {{
        let _pv_path = &$input;
    }};
}

/// Postconditions for equation `configuration`.
/// Call before return: `contract_post_configuration!(result_expr)`
macro_rules! contract_post_configuration {
    ($result:expr) => {{
        let _contract_result = &$result;
    }};
}

/// Invariants for equation `configuration`.
/// Check after computation: `contract_inv_configuration!(result_expr)`
macro_rules! contract_inv_configuration {
    () => {{}};
    ($result:expr) => {{
        let _contract_result = &$result;
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

// Auto-generated from contracts/memory-safety-v1.yaml — DO NOT EDIT
// Contract: memory-safety-v1

/// Preconditions for equation `bounds_safety`.
/// Domain-specific. Call: `contract_pre_bounds_safety!(slice_expr)`
macro_rules! contract_pre_bounds_safety {
    () => {{}};
    ($input:expr) => {{
        let _pv_input = &$input;
        debug_assert!(
            _pv_input.len() > 0,
            "Contract bounds_safety: precondition violated — input.len() > 0"
        );
    }};
}

/// Invariants for equation `bounds_safety`.
/// Check after computation: `contract_inv_bounds_safety!(result_expr)`
macro_rules! contract_inv_bounds_safety {
    () => {{}};
    ($result:expr) => {{
        let _contract_result = &$result;
    }};
}

/// Preconditions for equation `drop_safety`.
/// Domain-specific. Call: `contract_pre_drop_safety!(slice_expr)`
macro_rules! contract_pre_drop_safety {
    () => {{}};
    ($input:expr) => {{
        let _pv_input = &$input;
        debug_assert!(
            _pv_input.len() > 0,
            "Contract drop_safety: precondition violated — input.len() > 0"
        );
    }};
}

/// Invariants for equation `drop_safety`.
/// Check after computation: `contract_inv_drop_safety!(result_expr)`
macro_rules! contract_inv_drop_safety {
    () => {{}};
    ($result:expr) => {{
        let _contract_result = &$result;
    }};
}

/// Preconditions for equation `escape_analysis`.
/// Domain-specific. Call: `contract_pre_escape_analysis!(slice_expr)`
macro_rules! contract_pre_escape_analysis {
    () => {{}};
    ($input:expr) => {{
        let _pv_input = &$input;
        debug_assert!(
            _pv_input.len() > 0,
            "Contract escape_analysis: precondition violated — input.len() > 0"
        );
    }};
}

/// Invariants for equation `escape_analysis`.
/// Check after computation: `contract_inv_escape_analysis!(result_expr)`
macro_rules! contract_inv_escape_analysis {
    () => {{}};
    ($result:expr) => {{
        let _contract_result = &$result;
    }};
}

/// Preconditions for equation `lifetime_safety`.
/// Domain-specific. Call: `contract_pre_lifetime_safety!(slice_expr)`
macro_rules! contract_pre_lifetime_safety {
    () => {{}};
    ($input:expr) => {{
        let _pv_input = &$input;
        debug_assert!(
            _pv_input.len() > 0,
            "Contract lifetime_safety: precondition violated — input.len() > 0"
        );
    }};
}

/// Invariants for equation `lifetime_safety`.
/// Check after computation: `contract_inv_lifetime_safety!(result_expr)`
macro_rules! contract_inv_lifetime_safety {
    () => {{}};
    ($result:expr) => {{
        let _contract_result = &$result;
    }};
}

/// Preconditions for equation `ownership_invariant`.
/// Domain-specific. Call: `contract_pre_ownership_invariant!(slice_expr)`
macro_rules! contract_pre_ownership_invariant {
    () => {{}};
    ($input:expr) => {{
        let _pv_input = &$input;
        debug_assert!(
            _pv_input.len() > 0,
            "Contract ownership_invariant: precondition violated — input.len() > 0"
        );
    }};
}

/// Invariants for equation `ownership_invariant`.
/// Check after computation: `contract_inv_ownership_invariant!(result_expr)`
macro_rules! contract_inv_ownership_invariant {
    () => {{}};
    ($result:expr) => {{
        let _contract_result = &$result;
    }};
}

/// Preconditions for equation `use_after_move`.
/// Domain-specific. Call: `contract_pre_use_after_move!(slice_expr)`
macro_rules! contract_pre_use_after_move {
    () => {{}};
    ($input:expr) => {{
        let _pv_input = &$input;
        debug_assert!(
            _pv_input.len() > 0,
            "Contract use_after_move: precondition violated — input.len() > 0"
        );
    }};
}

/// Invariants for equation `use_after_move`.
/// Check after computation: `contract_inv_use_after_move!(result_expr)`
macro_rules! contract_inv_use_after_move {
    () => {{}};
    ($result:expr) => {{
        let _contract_result = &$result;
    }};
}

// Auto-generated from contracts/semantic-equivalence-v1.yaml — DO NOT EDIT
// Contract: semantic-equivalence-v1

/// Preconditions for equation `comprehension_equivalence`.
/// Domain-specific. Call: `contract_pre_comprehension_equivalence!(slice_expr)`
macro_rules! contract_pre_comprehension_equivalence {
    () => {{}};
    ($input:expr) => {{
        let _pv_input = &$input;
        debug_assert!(
            _pv_input.len() > 0,
            "Contract comprehension_equivalence: precondition violated — input.len() > 0"
        );
    }};
}

/// Invariants for equation `comprehension_equivalence`.
/// Check after computation: `contract_inv_comprehension_equivalence!(result_expr)`
macro_rules! contract_inv_comprehension_equivalence {
    () => {{}};
    ($result:expr) => {{
        let _contract_result = &$result;
    }};
}

/// Preconditions for equation `control_flow_equivalence`.
/// Domain-specific. Call: `contract_pre_control_flow_equivalence!(slice_expr)`
macro_rules! contract_pre_control_flow_equivalence {
    () => {{}};
    ($input:expr) => {{
        let _pv_input = &$input;
        debug_assert!(
            _pv_input.len() > 0,
            "Contract control_flow_equivalence: precondition violated — input.len() > 0"
        );
    }};
}

/// Invariants for equation `control_flow_equivalence`.
/// Check after computation: `contract_inv_control_flow_equivalence!(result_expr)`
macro_rules! contract_inv_control_flow_equivalence {
    () => {{}};
    ($result:expr) => {{
        let _contract_result = &$result;
    }};
}

/// Preconditions for equation `expression_equivalence`.
/// Domain-specific. Call: `contract_pre_expression_equivalence!(slice_expr)`
macro_rules! contract_pre_expression_equivalence {
    () => {{}};
    ($input:expr) => {{
        let _pv_input = &$input;
        debug_assert!(
            _pv_input.len() > 0,
            "Contract expression_equivalence: precondition violated — input.len() > 0"
        );
    }};
}

/// Invariants for equation `expression_equivalence`.
/// Check after computation: `contract_inv_expression_equivalence!(result_expr)`
macro_rules! contract_inv_expression_equivalence {
    () => {{}};
    ($result:expr) => {{
        let _contract_result = &$result;
    }};
}

/// Preconditions for equation `observational_equivalence`.
/// Domain-specific. Call: `contract_pre_observational_equivalence!(slice_expr)`
macro_rules! contract_pre_observational_equivalence {
    () => {{}};
    ($input:expr) => {{
        let _pv_input = &$input;
        debug_assert!(
            _pv_input.len() > 0,
            "Contract observational_equivalence: precondition violated — input.len() > 0"
        );
    }};
}

/// Invariants for equation `observational_equivalence`.
/// Check after computation: `contract_inv_observational_equivalence!(result_expr)`
macro_rules! contract_inv_observational_equivalence {
    () => {{}};
    ($result:expr) => {{
        let _contract_result = &$result;
    }};
}

/// Preconditions for equation `statement_equivalence`.
/// Domain-specific. Call: `contract_pre_statement_equivalence!(slice_expr)`
macro_rules! contract_pre_statement_equivalence {
    () => {{}};
    ($input:expr) => {{
        let _pv_input = &$input;
        debug_assert!(
            _pv_input.len() > 0,
            "Contract statement_equivalence: precondition violated — input.len() > 0"
        );
    }};
}

/// Invariants for equation `statement_equivalence`.
/// Check after computation: `contract_inv_statement_equivalence!(result_expr)`
macro_rules! contract_inv_statement_equivalence {
    () => {{}};
    ($result:expr) => {{
        let _contract_result = &$result;
    }};
}

// Auto-generated from contracts/type-preservation-v1.yaml — DO NOT EDIT
// Contract: type-preservation-v1

/// Preconditions for equation `container_preservation`.
/// Domain-specific. Call: `contract_pre_container_preservation!(slice_expr)`
macro_rules! contract_pre_container_preservation {
    () => {{}};
    ($input:expr) => {{
        let _pv_input = &$input;
        debug_assert!(
            _pv_input.len() > 0,
            "Contract container_preservation: precondition violated — input.len() > 0"
        );
    }};
}

/// Invariants for equation `container_preservation`.
/// Check after computation: `contract_inv_container_preservation!(result_expr)`
macro_rules! contract_inv_container_preservation {
    () => {{}};
    ($result:expr) => {{
        let _contract_result = &$result;
    }};
}

/// Preconditions for equation `copy_semantics`.
/// Domain-specific. Call: `contract_pre_copy_semantics!(slice_expr)`
macro_rules! contract_pre_copy_semantics {
    () => {{}};
    ($input:expr) => {{
        let _pv_input = &$input;
        debug_assert!(
            _pv_input.len() > 0,
            "Contract copy_semantics: precondition violated — input.len() > 0"
        );
    }};
}

/// Invariants for equation `copy_semantics`.
/// Check after computation: `contract_inv_copy_semantics!(result_expr)`
macro_rules! contract_inv_copy_semantics {
    () => {{}};
    ($result:expr) => {{
        let _contract_result = &$result;
    }};
}

/// Preconditions for equation `numeric_semantics`.
/// Domain-specific. Call: `contract_pre_numeric_semantics!(slice_expr)`
macro_rules! contract_pre_numeric_semantics {
    () => {{}};
    ($input:expr) => {{
        let _pv_input = &$input;
        debug_assert!(
            _pv_input.len() > 0,
            "Contract numeric_semantics: precondition violated — input.len() > 0"
        );
    }};
}

/// Invariants for equation `numeric_semantics`.
/// Check after computation: `contract_inv_numeric_semantics!(result_expr)`
macro_rules! contract_inv_numeric_semantics {
    () => {{}};
    ($result:expr) => {{
        let _contract_result = &$result;
    }};
}

/// Preconditions for equation `type_inference`.
/// Domain-specific. Call: `contract_pre_type_inference!(slice_expr)`
macro_rules! contract_pre_type_inference {
    () => {{}};
    ($input:expr) => {{
        let _pv_input = &$input;
        debug_assert!(
            _pv_input.len() > 0,
            "Contract type_inference: precondition violated — input.len() > 0"
        );
    }};
}

/// Invariants for equation `type_inference`.
/// Check after computation: `contract_inv_type_inference!(result_expr)`
macro_rules! contract_inv_type_inference {
    () => {{}};
    ($result:expr) => {{
        let _contract_result = &$result;
    }};
}

/// Preconditions for equation `type_map`.
/// Domain-specific. Call: `contract_pre_type_map!(slice_expr)`
macro_rules! contract_pre_type_map {
    () => {{}};
    ($input:expr) => {{
        let _pv_input = &$input;
        debug_assert!(
            _pv_input.len() > 0,
            "Contract type_map: precondition violated — input.len() > 0"
        );
    }};
}

/// Invariants for equation `type_map`.
/// Check after computation: `contract_inv_type_map!(result_expr)`
macro_rules! contract_inv_type_map {
    () => {{}};
    ($result:expr) => {{
        let _contract_result = &$result;
    }};
}

// Total: 20 preconditions, 0 postconditions, 0 invariants from 5 contracts
