use super::*;
use crate::hir::{ExceptionScope, Type};

// ============================================================================
// ErrorType enum tests
// ============================================================================

#[test]
fn test_error_type_concrete_equality() {
    let e1 = ErrorType::Concrete("ValueError".to_string());
    let e2 = ErrorType::Concrete("ValueError".to_string());
    let e3 = ErrorType::Concrete("TypeError".to_string());

    assert_eq!(e1, e2);
    assert_ne!(e1, e3);
}

#[test]
fn test_error_type_dyn_box_equality() {
    let e1 = ErrorType::DynBox;
    let e2 = ErrorType::DynBox;

    assert_eq!(e1, e2);
}

#[test]
fn test_error_type_concrete_vs_dyn_box() {
    let e1 = ErrorType::Concrete("ValueError".to_string());
    let e2 = ErrorType::DynBox;

    assert_ne!(e1, e2);
}

#[test]
fn test_error_type_clone() {
    let e1 = ErrorType::Concrete("ValueError".to_string());
    let cloned = e1.clone();
    assert_eq!(e1, cloned);

    let e2 = ErrorType::DynBox;
    let cloned2 = e2.clone();
    assert_eq!(e2, cloned2);
}

#[test]
fn test_error_type_debug() {
    let e1 = ErrorType::Concrete("ValueError".to_string());
    let debug_str = format!("{:?}", e1);
    assert!(debug_str.contains("Concrete"));
    assert!(debug_str.contains("ValueError"));

    let e2 = ErrorType::DynBox;
    let debug_str2 = format!("{:?}", e2);
    assert!(debug_str2.contains("DynBox"));
}

// ============================================================================
// CodeGenContext scope tests
// ============================================================================

#[test]
fn test_enter_scope() {
    let mut ctx = test_helpers::test_context();
    let initial_depth = ctx.declared_vars.len();

    ctx.enter_scope();
    assert_eq!(ctx.declared_vars.len(), initial_depth + 1);

    ctx.enter_scope();
    assert_eq!(ctx.declared_vars.len(), initial_depth + 2);
}

#[test]
fn test_exit_scope() {
    let mut ctx = test_helpers::test_context();
    ctx.enter_scope();
    ctx.enter_scope();
    let depth_before = ctx.declared_vars.len();

    ctx.exit_scope();
    assert_eq!(ctx.declared_vars.len(), depth_before - 1);
}

#[test]
fn test_enter_exit_scope_balanced() {
    let mut ctx = test_helpers::test_context();
    let initial_depth = ctx.declared_vars.len();

    for _ in 0..5 {
        ctx.enter_scope();
    }
    assert_eq!(ctx.declared_vars.len(), initial_depth + 5);

    for _ in 0..5 {
        ctx.exit_scope();
    }
    assert_eq!(ctx.declared_vars.len(), initial_depth);
}

#[test]
fn test_declare_var_in_current_scope() {
    let mut ctx = test_helpers::test_context();

    ctx.declare_var("x");
    assert!(ctx.is_declared("x"));
}

#[test]
fn test_declare_var_not_in_other_scope() {
    let mut ctx = test_helpers::test_context();

    ctx.enter_scope();
    ctx.declare_var("inner_var");

    // Variable should be visible in current scope
    assert!(ctx.is_declared("inner_var"));

    ctx.exit_scope();

    // After exiting scope, variable should no longer be declared
    assert!(!ctx.is_declared("inner_var"));
}

#[test]
fn test_is_declared_outer_scope() {
    let mut ctx = test_helpers::test_context();

    ctx.declare_var("outer");
    ctx.enter_scope();

    // Should still see outer scope variables
    assert!(ctx.is_declared("outer"));
}

#[test]
fn test_is_declared_false_for_undeclared() {
    let ctx = test_helpers::test_context();
    assert!(!ctx.is_declared("nonexistent"));
}

#[test]
fn test_multiple_vars_same_scope() {
    let mut ctx = test_helpers::test_context();

    ctx.declare_var("a");
    ctx.declare_var("b");
    ctx.declare_var("c");

    assert!(ctx.is_declared("a"));
    assert!(ctx.is_declared("b"));
    assert!(ctx.is_declared("c"));
}

#[test]
fn test_shadow_var_in_nested_scope() {
    let mut ctx = test_helpers::test_context();

    ctx.declare_var("x");
    ctx.enter_scope();
    ctx.declare_var("x"); // Shadow outer x

    assert!(ctx.is_declared("x")); // Should still be declared

    ctx.exit_scope();
    assert!(ctx.is_declared("x")); // Outer x should still exist
}

// ============================================================================
// Exception scope tests
// ============================================================================

#[test]
fn test_current_exception_scope_default() {
    let ctx = test_helpers::test_context();
    assert!(matches!(
        ctx.current_exception_scope(),
        ExceptionScope::Unhandled
    ));
}

#[test]
fn test_enter_try_scope() {
    let mut ctx = test_helpers::test_context();

    ctx.enter_try_scope(vec!["ValueError".to_string()]);

    match ctx.current_exception_scope() {
        ExceptionScope::TryCaught { handled_types } => {
            assert_eq!(handled_types.len(), 1);
            assert!(handled_types.contains(&"ValueError".to_string()));
        }
        _ => panic!("Expected TryCaught scope"),
    }
}

#[test]
fn test_enter_try_scope_multiple_handlers() {
    let mut ctx = test_helpers::test_context();

    ctx.enter_try_scope(vec![
        "ValueError".to_string(),
        "TypeError".to_string(),
        "RuntimeError".to_string(),
    ]);

    match ctx.current_exception_scope() {
        ExceptionScope::TryCaught { handled_types } => {
            assert_eq!(handled_types.len(), 3);
        }
        _ => panic!("Expected TryCaught scope"),
    }
}

#[test]
fn test_enter_try_scope_bare_except() {
    let mut ctx = test_helpers::test_context();

    // Empty vec = bare except (catches all)
    ctx.enter_try_scope(vec![]);

    match ctx.current_exception_scope() {
        ExceptionScope::TryCaught { handled_types } => {
            assert!(handled_types.is_empty());
        }
        _ => panic!("Expected TryCaught scope"),
    }
}

#[test]
fn test_enter_handler_scope() {
    let mut ctx = test_helpers::test_context();

    ctx.enter_handler_scope();
    assert!(matches!(
        ctx.current_exception_scope(),
        ExceptionScope::Handler
    ));
}

#[test]
fn test_exit_exception_scope() {
    let mut ctx = test_helpers::test_context();

    ctx.enter_try_scope(vec!["ValueError".to_string()]);
    ctx.exit_exception_scope();

    assert!(matches!(
        ctx.current_exception_scope(),
        ExceptionScope::Unhandled
    ));
}

#[test]
fn test_nested_exception_scopes() {
    let mut ctx = test_helpers::test_context();

    ctx.enter_try_scope(vec!["ValueError".to_string()]);
    ctx.enter_try_scope(vec!["TypeError".to_string()]);

    // Should see innermost scope
    match ctx.current_exception_scope() {
        ExceptionScope::TryCaught { handled_types } => {
            assert!(handled_types.contains(&"TypeError".to_string()));
        }
        _ => panic!("Expected TryCaught scope"),
    }

    ctx.exit_exception_scope();

    // Should now see outer scope
    match ctx.current_exception_scope() {
        ExceptionScope::TryCaught { handled_types } => {
            assert!(handled_types.contains(&"ValueError".to_string()));
        }
        _ => panic!("Expected TryCaught scope"),
    }
}

#[test]
fn test_is_in_try_block_false_initially() {
    let ctx = test_helpers::test_context();
    assert!(!ctx.is_in_try_block());
}

#[test]
fn test_is_in_try_block_true_after_enter() {
    let mut ctx = test_helpers::test_context();

    ctx.enter_try_scope(vec!["ValueError".to_string()]);
    assert!(ctx.is_in_try_block());
}

#[test]
fn test_is_in_try_block_false_in_handler() {
    let mut ctx = test_helpers::test_context();

    ctx.enter_handler_scope();
    assert!(!ctx.is_in_try_block());
}

#[test]
fn test_is_exception_handled_specific_type() {
    let mut ctx = test_helpers::test_context();

    ctx.enter_try_scope(vec!["ValueError".to_string(), "TypeError".to_string()]);

    assert!(ctx.is_exception_handled("ValueError"));
    assert!(ctx.is_exception_handled("TypeError"));
    assert!(!ctx.is_exception_handled("RuntimeError"));
}

#[test]
fn test_is_exception_handled_bare_except() {
    let mut ctx = test_helpers::test_context();

    // Bare except catches all
    ctx.enter_try_scope(vec![]);

    assert!(ctx.is_exception_handled("ValueError"));
    assert!(ctx.is_exception_handled("TypeError"));
    assert!(ctx.is_exception_handled("RuntimeError"));
    assert!(ctx.is_exception_handled("AnyException"));
}

#[test]
fn test_is_exception_handled_not_in_try() {
    let ctx = test_helpers::test_context();

    assert!(!ctx.is_exception_handled("ValueError"));
}

#[test]
fn test_exception_nesting_depth_initial() {
    let ctx = test_helpers::test_context();
    assert_eq!(ctx.exception_nesting_depth(), 0);
}

#[test]
fn test_exception_nesting_depth_after_enter() {
    let mut ctx = test_helpers::test_context();

    ctx.enter_try_scope(vec![]);
    assert_eq!(ctx.exception_nesting_depth(), 1);

    ctx.enter_handler_scope();
    assert_eq!(ctx.exception_nesting_depth(), 2);

    ctx.enter_try_scope(vec!["ValueError".to_string()]);
    assert_eq!(ctx.exception_nesting_depth(), 3);
}

#[test]
fn test_exception_nesting_depth_after_exit() {
    let mut ctx = test_helpers::test_context();

    ctx.enter_try_scope(vec![]);
    ctx.enter_try_scope(vec![]);
    assert_eq!(ctx.exception_nesting_depth(), 2);

    ctx.exit_exception_scope();
    assert_eq!(ctx.exception_nesting_depth(), 1);

    ctx.exit_exception_scope();
    assert_eq!(ctx.exception_nesting_depth(), 0);
}

// ============================================================================
// Test helper function tests
// ============================================================================

#[test]
fn test_context_default_values() {
    let ctx = test_helpers::test_context();

    // Check boolean flags default to false
    assert!(!ctx.needs_hashmap);
    assert!(!ctx.needs_hashset);
    assert!(!ctx.needs_rand);
    assert!(!ctx.needs_serde_json);
    assert!(!ctx.needs_regex);
    assert!(!ctx.needs_chrono);
    assert!(!ctx.needs_clap);
    assert!(!ctx.current_function_can_fail);
    assert!(!ctx.is_classmethod);
    assert!(!ctx.in_generator);
    assert!(!ctx.is_final_statement);
    assert!(!ctx.is_main_function);
    assert!(!ctx.in_json_context);
}

#[test]
fn test_context_collections_empty() {
    let ctx = test_helpers::test_context();

    // Check collections are empty
    assert!(ctx.mutable_vars.is_empty());
    assert!(ctx.generator_state_vars.is_empty());
    assert!(ctx.var_types.is_empty());
    assert!(ctx.class_names.is_empty());
    assert!(ctx.property_methods.is_empty());
    assert!(ctx.tuple_iter_vars.is_empty());
    assert!(ctx.iterator_vars.is_empty());
    assert!(ctx.exception_scopes.is_empty());
    assert!(ctx.generated_enums.is_empty());
}

#[test]
fn test_context_has_initial_scope() {
    let ctx = test_helpers::test_context();

    // Should have one initial scope
    assert_eq!(ctx.declared_vars.len(), 1);
}

#[test]
fn test_context_none_values() {
    let ctx = test_helpers::test_context();

    // Check Option values are None
    assert!(ctx.current_return_type.is_none());
    assert!(ctx.current_error_type.is_none());
    assert!(ctx.generated_args_struct.is_none());
    assert!(ctx.generated_commands_enum.is_none());
    assert!(ctx.current_subcommand_fields.is_none());
    assert!(ctx.current_assign_type.is_none());
}

#[test]
fn test_context_default_impl() {
    // Test that Default trait works (only available in test mode)
    let ctx = CodeGenContext::default();

    // Should be equivalent to test_context()
    assert!(!ctx.needs_hashmap);
    assert_eq!(ctx.declared_vars.len(), 1);
}

// ============================================================================
// Context mutability tests
// ============================================================================

#[test]
fn test_modify_needs_flags() {
    let mut ctx = test_helpers::test_context();

    ctx.needs_hashmap = true;
    ctx.needs_serde_json = true;
    ctx.needs_rand = true;

    assert!(ctx.needs_hashmap);
    assert!(ctx.needs_serde_json);
    assert!(ctx.needs_rand);
}

#[test]
fn test_add_mutable_var() {
    let mut ctx = test_helpers::test_context();

    ctx.mutable_vars.insert("x".to_string());
    ctx.mutable_vars.insert("y".to_string());

    assert!(ctx.mutable_vars.contains("x"));
    assert!(ctx.mutable_vars.contains("y"));
    assert!(!ctx.mutable_vars.contains("z"));
}

#[test]
fn test_add_var_type() {
    let mut ctx = test_helpers::test_context();

    ctx.var_types.insert("count".to_string(), Type::Int);
    ctx.var_types.insert("name".to_string(), Type::String);

    assert!(matches!(ctx.var_types.get("count"), Some(Type::Int)));
    assert!(matches!(ctx.var_types.get("name"), Some(Type::String)));
}

#[test]
fn test_add_class_name() {
    let mut ctx = test_helpers::test_context();

    ctx.class_names.insert("MyClass".to_string());
    ctx.class_names.insert("OtherClass".to_string());

    assert!(ctx.class_names.contains("MyClass"));
    assert!(ctx.class_names.contains("OtherClass"));
}

#[test]
fn test_add_result_returning_function() {
    let mut ctx = test_helpers::test_context();

    ctx.result_returning_functions
        .insert("open_file".to_string());
    ctx.result_returning_functions
        .insert("parse_json".to_string());

    assert!(ctx.result_returning_functions.contains("open_file"));
    assert!(ctx.result_returning_functions.contains("parse_json"));
    assert!(!ctx.result_returning_functions.contains("other_func"));
}

#[test]
fn test_set_current_error_type() {
    let mut ctx = test_helpers::test_context();

    ctx.current_error_type = Some(ErrorType::Concrete("ValueError".to_string()));
    assert!(matches!(
        ctx.current_error_type,
        Some(ErrorType::Concrete(ref s)) if s == "ValueError"
    ));

    ctx.current_error_type = Some(ErrorType::DynBox);
    assert!(matches!(ctx.current_error_type, Some(ErrorType::DynBox)));

    ctx.current_error_type = None;
    assert!(ctx.current_error_type.is_none());
}

#[test]
fn test_function_param_borrows() {
    let mut ctx = test_helpers::test_context();

    ctx.function_param_borrows
        .insert("my_func".to_string(), vec![true, false, true]);

    let borrows = ctx.function_param_borrows.get("my_func").unwrap();
    assert_eq!(borrows.len(), 3);
    assert!(borrows[0]); // First param is borrowed
    assert!(!borrows[1]); // Second param is owned
    assert!(borrows[2]); // Third param is borrowed
}

#[test]
fn test_function_param_types() {
    let mut ctx = test_helpers::test_context();

    ctx.function_param_types.insert(
        "process".to_string(),
        vec![Type::Int, Type::String, Type::List(Box::new(Type::Float))],
    );

    let types = ctx.function_param_types.get("process").unwrap();
    assert_eq!(types.len(), 3);
    assert!(matches!(types[0], Type::Int));
    assert!(matches!(types[1], Type::String));
    assert!(matches!(types[2], Type::List(_)));
}

// ============================================================================
// Complex scenario tests
// ============================================================================

#[test]
fn test_nested_function_with_scopes() {
    let mut ctx = test_helpers::test_context();

    // Outer function scope
    ctx.declare_var("outer_var");
    ctx.enter_scope();

    // Inner block scope
    ctx.declare_var("inner_var");

    assert!(ctx.is_declared("outer_var"));
    assert!(ctx.is_declared("inner_var"));

    ctx.exit_scope();

    assert!(ctx.is_declared("outer_var"));
    assert!(!ctx.is_declared("inner_var"));
}

#[test]
fn test_try_except_with_nested_scopes() {
    let mut ctx = test_helpers::test_context();

    // Enter try block
    ctx.enter_try_scope(vec!["ValueError".to_string()]);
    ctx.enter_scope();
    ctx.declare_var("try_var");

    assert!(ctx.is_in_try_block());
    assert!(ctx.is_declared("try_var"));

    // Enter exception handler
    ctx.exit_scope();
    ctx.exit_exception_scope();
    ctx.enter_handler_scope();
    ctx.enter_scope();
    ctx.declare_var("handler_var");

    assert!(!ctx.is_in_try_block());
    assert!(ctx.is_declared("handler_var"));
    assert!(!ctx.is_declared("try_var"));
}

#[test]
fn test_generator_context_setup() {
    let mut ctx = test_helpers::test_context();

    ctx.in_generator = true;
    ctx.generator_state_vars.insert("current".to_string());
    ctx.generator_state_vars.insert("index".to_string());

    assert!(ctx.in_generator);
    assert_eq!(ctx.generator_state_vars.len(), 2);
    assert!(ctx.generator_state_vars.contains("current"));
}

#[test]
fn test_argparse_context_setup() {
    let mut ctx = test_helpers::test_context();

    ctx.needs_clap = true;
    ctx.validator_functions.insert("validate_path".to_string());
    ctx.validator_functions.insert("validate_int".to_string());

    assert!(ctx.needs_clap);
    assert_eq!(ctx.validator_functions.len(), 2);
}

#[test]
fn test_subcommand_match_context() {
    let mut ctx = test_helpers::test_context();

    ctx.in_subcommand_match_arm = true;
    ctx.subcommand_match_fields = vec!["input".to_string(), "output".to_string()];

    assert!(ctx.in_subcommand_match_arm);
    assert_eq!(ctx.subcommand_match_fields.len(), 2);
    assert_eq!(ctx.subcommand_match_fields[0], "input");
}

#[test]
fn test_json_context_flag() {
    let mut ctx = test_helpers::test_context();

    assert!(!ctx.in_json_context);

    ctx.in_json_context = true;
    assert!(ctx.in_json_context);

    ctx.in_json_context = false;
    assert!(!ctx.in_json_context);
}

#[test]
fn test_main_function_context() {
    let mut ctx = test_helpers::test_context();

    ctx.is_main_function = true;
    ctx.current_return_type = Some(Type::Int);

    assert!(ctx.is_main_function);
    assert!(matches!(ctx.current_return_type, Some(Type::Int)));
}

#[test]
fn test_boxed_dyn_write_vars() {
    let mut ctx = test_helpers::test_context();

    ctx.boxed_dyn_write_vars.insert("output".to_string());
    ctx.function_returns_boxed_write = true;

    assert!(ctx.boxed_dyn_write_vars.contains("output"));
    assert!(ctx.function_returns_boxed_write);
}

#[test]
fn test_option_unwrap_map() {
    let mut ctx = test_helpers::test_context();

    ctx.option_unwrap_map
        .insert("config".to_string(), "config_val".to_string());

    assert_eq!(
        ctx.option_unwrap_map.get("config"),
        Some(&"config_val".to_string())
    );
}

#[test]
fn test_adt_child_to_parent_mapping() {
    let mut ctx = test_helpers::test_context();

    ctx.adt_child_to_parent
        .insert("ListIter".to_string(), "Iter".to_string());
    ctx.adt_child_to_parent
        .insert("RangeIter".to_string(), "Iter".to_string());

    assert_eq!(
        ctx.adt_child_to_parent.get("ListIter"),
        Some(&"Iter".to_string())
    );
    assert_eq!(
        ctx.adt_child_to_parent.get("RangeIter"),
        Some(&"Iter".to_string())
    );
}

#[test]
fn test_char_iter_vars() {
    let mut ctx = test_helpers::test_context();

    ctx.char_iter_vars.insert("c".to_string());
    ctx.char_iter_vars.insert("ch".to_string());

    assert!(ctx.char_iter_vars.contains("c"));
    assert!(ctx.char_iter_vars.contains("ch"));
    assert!(!ctx.char_iter_vars.contains("x"));
}

#[test]
fn test_numpy_vars_tracking() {
    let mut ctx = test_helpers::test_context();

    ctx.needs_trueno = true;
    ctx.numpy_vars.insert("arr".to_string());
    ctx.numpy_vars.insert("matrix".to_string());

    assert!(ctx.needs_trueno);
    assert!(ctx.numpy_vars.contains("arr"));
    assert!(ctx.numpy_vars.contains("matrix"));
}
