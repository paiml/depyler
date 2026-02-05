use super::*;
use crate::hir::ExceptionScope;

#[test]
fn test_default_context_creates_one_scope() {
    let ctx = CodeGenContext::default();
    assert_eq!(ctx.declared_vars.len(), 1);
}

#[test]
fn test_enter_scope_adds_scope() {
    let mut ctx = CodeGenContext::default();
    let initial = ctx.declared_vars.len();
    ctx.enter_scope();
    assert_eq!(ctx.declared_vars.len(), initial + 1);
}

#[test]
fn test_exit_scope_removes_scope() {
    let mut ctx = CodeGenContext::default();
    ctx.enter_scope();
    let before = ctx.declared_vars.len();
    ctx.exit_scope();
    assert_eq!(ctx.declared_vars.len(), before - 1);
}

#[test]
fn test_declare_var_in_current_scope() {
    let mut ctx = CodeGenContext::default();
    ctx.declare_var("x");
    assert!(ctx.is_declared("x"));
}

#[test]
fn test_is_declared_false_for_unknown() {
    let ctx = CodeGenContext::default();
    assert!(!ctx.is_declared("nonexistent"));
}

#[test]
fn test_var_visible_in_outer_scope() {
    let mut ctx = CodeGenContext::default();
    ctx.declare_var("outer");
    ctx.enter_scope();
    assert!(ctx.is_declared("outer"));
}

#[test]
fn test_var_not_visible_after_scope_exit() {
    let mut ctx = CodeGenContext::default();
    ctx.enter_scope();
    ctx.declare_var("inner");
    assert!(ctx.is_declared("inner"));
    ctx.exit_scope();
    assert!(!ctx.is_declared("inner"));
}

#[test]
fn test_nested_scopes() {
    let mut ctx = CodeGenContext::default();
    ctx.declare_var("level0");
    ctx.enter_scope();
    ctx.declare_var("level1");
    ctx.enter_scope();
    ctx.declare_var("level2");

    assert!(ctx.is_declared("level0"));
    assert!(ctx.is_declared("level1"));
    assert!(ctx.is_declared("level2"));

    ctx.exit_scope();
    assert!(ctx.is_declared("level0"));
    assert!(ctx.is_declared("level1"));
    assert!(!ctx.is_declared("level2"));

    ctx.exit_scope();
    assert!(ctx.is_declared("level0"));
    assert!(!ctx.is_declared("level1"));
}

#[test]
fn test_exception_scope_default_is_unhandled() {
    let ctx = CodeGenContext::default();
    assert!(matches!(
        ctx.current_exception_scope(),
        ExceptionScope::Unhandled
    ));
}

#[test]
fn test_not_in_try_block_by_default() {
    let ctx = CodeGenContext::default();
    assert!(!ctx.is_in_try_block());
}

#[test]
fn test_enter_try_scope() {
    let mut ctx = CodeGenContext::default();
    ctx.enter_try_scope(vec!["ValueError".to_string()]);
    assert!(ctx.is_in_try_block());
}

#[test]
fn test_is_exception_handled_specific_type() {
    let mut ctx = CodeGenContext::default();
    ctx.enter_try_scope(vec!["ValueError".to_string(), "KeyError".to_string()]);
    assert!(ctx.is_exception_handled("ValueError"));
    assert!(ctx.is_exception_handled("KeyError"));
    assert!(!ctx.is_exception_handled("TypeError"));
}

#[test]
fn test_bare_except_catches_all() {
    let mut ctx = CodeGenContext::default();
    ctx.enter_try_scope(vec![]); // empty = bare except
    assert!(ctx.is_exception_handled("ValueError"));
    assert!(ctx.is_exception_handled("AnyError"));
}

#[test]
fn test_exception_not_handled_outside_try() {
    let ctx = CodeGenContext::default();
    assert!(!ctx.is_exception_handled("ValueError"));
}

#[test]
fn test_enter_handler_scope() {
    let mut ctx = CodeGenContext::default();
    ctx.enter_handler_scope();
    assert!(matches!(
        ctx.current_exception_scope(),
        ExceptionScope::Handler
    ));
    assert!(!ctx.is_in_try_block());
}

#[test]
fn test_exit_exception_scope() {
    let mut ctx = CodeGenContext::default();
    ctx.enter_try_scope(vec![]);
    assert!(ctx.is_in_try_block());
    ctx.exit_exception_scope();
    assert!(!ctx.is_in_try_block());
}

#[test]
fn test_exception_nesting_depth_empty() {
    let ctx = CodeGenContext::default();
    assert_eq!(ctx.exception_nesting_depth(), 0);
}

#[test]
fn test_exception_nesting_depth_nested() {
    let mut ctx = CodeGenContext::default();
    ctx.enter_try_scope(vec!["ValueError".to_string()]);
    assert_eq!(ctx.exception_nesting_depth(), 1);
    ctx.enter_try_scope(vec!["KeyError".to_string()]);
    assert_eq!(ctx.exception_nesting_depth(), 2);
    ctx.exit_exception_scope();
    assert_eq!(ctx.exception_nesting_depth(), 1);
}

#[test]
fn test_fallback_type_annotation_nasa_mode() {
    let mut ctx = CodeGenContext::default();
    // Default TypeMapper has nasa_mode = true
    let tokens = ctx.fallback_type_annotation();
    let s = tokens.to_string();
    assert!(s.contains("String"), "NASA mode should return `: String`, got: {}", s);
}

#[test]
fn test_fallback_type_nasa_mode() {
    let mut ctx = CodeGenContext::default();
    let tokens = ctx.fallback_type();
    let s = tokens.to_string();
    assert!(s.contains("String"), "NASA mode should return `String`, got: {}", s);
}

#[test]
fn test_lookup_external_return_type_none_without_feature() {
    let ctx = CodeGenContext::default();
    assert!(ctx.lookup_external_return_type("requests", "get").is_none());
}

#[test]
fn test_has_external_symbol_false_without_feature() {
    let ctx = CodeGenContext::default();
    assert!(!ctx.has_external_symbol("requests", "get"));
}

#[test]
fn test_error_type_concrete_variant() {
    let e = ErrorType::Concrete("ValueError".to_string());
    assert_eq!(e, ErrorType::Concrete("ValueError".to_string()));
}

#[test]
fn test_error_type_dynbox_variant() {
    let e = ErrorType::DynBox;
    assert_eq!(e, ErrorType::DynBox);
}

#[test]
fn test_error_type_debug() {
    let e = ErrorType::Concrete("KeyError".to_string());
    let debug = format!("{:?}", e);
    assert!(debug.contains("KeyError"));
}

#[test]
fn test_error_type_clone() {
    let e = ErrorType::DynBox;
    let cloned = e.clone();
    assert_eq!(e, cloned);
}

#[test]
fn test_default_context_flags_false() {
    let ctx = CodeGenContext::default();
    assert!(!ctx.needs_hashmap);
    assert!(!ctx.needs_hashset);
    assert!(!ctx.needs_rand);
    assert!(!ctx.needs_serde_json);
    assert!(!ctx.needs_regex);
    assert!(!ctx.needs_chrono);
    assert!(!ctx.needs_clap);
    assert!(!ctx.needs_tokio);
    assert!(!ctx.is_classmethod);
    assert!(!ctx.in_generator);
    assert!(!ctx.is_final_statement);
    assert!(!ctx.is_main_function);
    assert!(!ctx.in_json_context);
    assert!(!ctx.in_cmd_handler);
    assert!(!ctx.in_subcommand_match_arm);
    assert!(!ctx.function_returns_boxed_write);
    assert!(!ctx.force_dict_value_option_wrap);
    assert!(!ctx.returns_impl_iterator);
}

#[test]
fn test_default_context_collections_empty() {
    let ctx = CodeGenContext::default();
    assert!(ctx.mutable_vars.is_empty());
    assert!(ctx.var_types.is_empty());
    assert!(ctx.class_names.is_empty());
    assert!(ctx.function_return_types.is_empty());
    assert!(ctx.function_param_borrows.is_empty());
    assert!(ctx.tuple_iter_vars.is_empty());
    assert!(ctx.iterator_vars.is_empty());
    assert!(ctx.ref_params.is_empty());
    assert!(ctx.result_bool_functions.is_empty());
    assert!(ctx.result_returning_functions.is_empty());
    assert!(ctx.option_returning_functions.is_empty());
    assert!(ctx.generated_enums.is_empty());
    assert!(ctx.exception_scopes.is_empty());
    assert!(ctx.type_overrides.is_empty());
    assert!(ctx.vars_used_later.is_empty());
}

#[test]
fn test_default_context_options_none() {
    let ctx = CodeGenContext::default();
    assert!(ctx.current_return_type.is_none());
    assert!(ctx.current_error_type.is_none());
    assert!(ctx.generated_args_struct.is_none());
    assert!(ctx.generated_commands_enum.is_none());
    assert!(ctx.current_subcommand_fields.is_none());
    assert!(ctx.current_assign_type.is_none());
    assert!(ctx.last_external_call_return_type.is_none());
}

#[test]
fn test_mutable_vars_tracking() {
    let mut ctx = CodeGenContext::default();
    ctx.mutable_vars.insert("x".to_string());
    assert!(ctx.mutable_vars.contains("x"));
    assert!(!ctx.mutable_vars.contains("y"));
}

#[test]
fn test_var_types_tracking() {
    let mut ctx = CodeGenContext::default();
    ctx.var_types.insert("count".to_string(), crate::hir::Type::Int);
    assert_eq!(ctx.var_types.get("count"), Some(&crate::hir::Type::Int));
}

#[test]
fn test_class_names_tracking() {
    let mut ctx = CodeGenContext::default();
    ctx.class_names.insert("MyClass".to_string());
    assert!(ctx.class_names.contains("MyClass"));
}

#[test]
fn test_generator_state_vars() {
    let mut ctx = CodeGenContext::default();
    ctx.in_generator = true;
    ctx.generator_state_vars.insert("gen".to_string());
    assert!(ctx.in_generator);
    assert!(ctx.generator_state_vars.contains("gen"));
}

#[test]
fn test_function_return_types_tracking() {
    let mut ctx = CodeGenContext::default();
    ctx.function_return_types
        .insert("compute".to_string(), crate::hir::Type::Float);
    assert_eq!(
        ctx.function_return_types.get("compute"),
        Some(&crate::hir::Type::Float)
    );
}

#[test]
fn test_ref_params_tracking() {
    let mut ctx = CodeGenContext::default();
    ctx.ref_params.insert("data".to_string());
    assert!(ctx.ref_params.contains("data"));
}

#[test]
fn test_result_returning_functions() {
    let mut ctx = CodeGenContext::default();
    ctx.result_returning_functions.insert("open_file".to_string());
    assert!(ctx.result_returning_functions.contains("open_file"));
}

#[test]
fn test_option_returning_functions() {
    let mut ctx = CodeGenContext::default();
    ctx.option_returning_functions.insert("find".to_string());
    assert!(ctx.option_returning_functions.contains("find"));
}

#[test]
fn test_char_iter_vars_tracking() {
    let mut ctx = CodeGenContext::default();
    ctx.char_iter_vars.insert("ch".to_string());
    assert!(ctx.char_iter_vars.contains("ch"));
}

#[test]
fn test_module_constant_types() {
    let mut ctx = CodeGenContext::default();
    let dict_type = crate::hir::Type::Dict(
        Box::new(crate::hir::Type::String),
        Box::new(crate::hir::Type::Int),
    );
    ctx.module_constant_types
        .insert("CONFIG".to_string(), dict_type.clone());
    assert_eq!(
        ctx.module_constant_types.get("CONFIG"),
        Some(&dict_type)
    );
}

#[test]
fn test_type_overrides() {
    let mut ctx = CodeGenContext::default();
    ctx.type_overrides
        .insert("result".to_string(), crate::hir::Type::String);
    assert_eq!(
        ctx.type_overrides.get("result"),
        Some(&crate::hir::Type::String)
    );
}

#[test]
fn test_all_imported_modules() {
    let mut ctx = CodeGenContext::default();
    ctx.all_imported_modules.insert("json".to_string());
    ctx.all_imported_modules.insert("os".to_string());
    assert!(ctx.all_imported_modules.contains("json"));
    assert!(ctx.all_imported_modules.contains("os"));
    assert!(!ctx.all_imported_modules.contains("sys"));
}

#[test]
fn test_module_aliases() {
    let mut ctx = CodeGenContext::default();
    ctx.module_aliases
        .insert("ET".to_string(), "xml.etree.ElementTree".to_string());
    assert_eq!(
        ctx.module_aliases.get("ET"),
        Some(&"xml.etree.ElementTree".to_string())
    );
}

#[test]
fn test_narrowed_option_vars() {
    let mut ctx = CodeGenContext::default();
    ctx.narrowed_option_vars.insert("value".to_string());
    assert!(ctx.narrowed_option_vars.contains("value"));
}

#[test]
fn test_validator_functions() {
    let mut ctx = CodeGenContext::default();
    ctx.validator_functions.insert("check_port".to_string());
    assert!(ctx.validator_functions.contains("check_port"));
}

#[test]
fn test_fn_str_params() {
    let mut ctx = CodeGenContext::default();
    ctx.fn_str_params.insert("name".to_string());
    assert!(ctx.fn_str_params.contains("name"));
}

#[test]
fn test_slice_params() {
    let mut ctx = CodeGenContext::default();
    ctx.slice_params.insert("args".to_string());
    assert!(ctx.slice_params.contains("args"));
}

#[test]
fn test_hoisted_inference_vars() {
    let mut ctx = CodeGenContext::default();
    ctx.hoisted_inference_vars.insert("format".to_string());
    assert!(ctx.hoisted_inference_vars.contains("format"));
}

#[test]
fn test_none_placeholder_vars() {
    let mut ctx = CodeGenContext::default();
    ctx.none_placeholder_vars.insert("result".to_string());
    assert!(ctx.none_placeholder_vars.contains("result"));
}

#[test]
fn test_needs_flags_setting() {
    let mut ctx = CodeGenContext::default();
    ctx.needs_hashmap = true;
    ctx.needs_serde_json = true;
    ctx.needs_tokio = true;
    assert!(ctx.needs_hashmap);
    assert!(ctx.needs_serde_json);
    assert!(ctx.needs_tokio);
}

#[test]
fn test_current_function_can_fail() {
    let mut ctx = CodeGenContext::default();
    assert!(!ctx.current_function_can_fail);
    ctx.current_function_can_fail = true;
    assert!(ctx.current_function_can_fail);
}

#[test]
fn test_current_error_type_setting() {
    let mut ctx = CodeGenContext::default();
    ctx.current_error_type = Some(ErrorType::DynBox);
    assert_eq!(ctx.current_error_type, Some(ErrorType::DynBox));

    ctx.current_error_type = Some(ErrorType::Concrete("IOError".to_string()));
    assert_eq!(
        ctx.current_error_type,
        Some(ErrorType::Concrete("IOError".to_string()))
    );
}

#[test]
fn test_exception_scope_mixed_nesting() {
    let mut ctx = CodeGenContext::default();
    ctx.enter_try_scope(vec!["ValueError".to_string()]);
    assert!(ctx.is_in_try_block());
    assert_eq!(ctx.exception_nesting_depth(), 1);

    ctx.enter_handler_scope();
    assert!(!ctx.is_in_try_block());
    assert_eq!(ctx.exception_nesting_depth(), 2);

    ctx.exit_exception_scope(); // exit handler
    assert!(ctx.is_in_try_block());
    assert_eq!(ctx.exception_nesting_depth(), 1);

    ctx.exit_exception_scope(); // exit try
    assert!(!ctx.is_in_try_block());
    assert_eq!(ctx.exception_nesting_depth(), 0);
}

#[test]
fn test_process_union_type() {
    let mut ctx = CodeGenContext::default();
    let types = vec![crate::hir::Type::Int, crate::hir::Type::String];
    let name = ctx.process_union_type(&types);
    assert!(!name.is_empty());
}

#[test]
fn test_multiple_declare_var_same_scope() {
    let mut ctx = CodeGenContext::default();
    ctx.declare_var("x");
    ctx.declare_var("y");
    ctx.declare_var("z");
    assert!(ctx.is_declared("x"));
    assert!(ctx.is_declared("y"));
    assert!(ctx.is_declared("z"));
}

#[test]
fn test_declare_var_empty_scopes() {
    let mut ctx = CodeGenContext::default();
    // Pop the default scope
    ctx.declared_vars.clear();
    // declare_var should handle no scopes gracefully
    ctx.declare_var("orphan");
    assert!(!ctx.is_declared("orphan"));
}

#[test]
fn test_exit_scope_on_empty_does_not_panic() {
    let mut ctx = CodeGenContext::default();
    ctx.declared_vars.clear();
    ctx.exit_scope(); // should not panic
}
