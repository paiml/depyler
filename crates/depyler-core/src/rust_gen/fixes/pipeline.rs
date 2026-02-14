//! Fix application pipeline — applies all text-level fixes in sequence.
//!
//! Extracted from `generate_rust_file_internal()` in rust_gen.rs.
//! Each fix targets a specific rustc error pattern.

use super::collections::*;
use super::depyler_value::*;
use super::enums::*;
use super::misc::*;
use super::numeric::*;
use super::options_results::*;
use super::ownership::*;
use super::strings::*;
use super::truthiness::*;

/// Apply all text-level fixes to generated Rust code.
///
/// This is the main entry point for the fix pipeline. It takes the formatted
/// Rust code from codegen and applies ~130 text-level fixes in sequence.
pub(in crate::rust_gen) fn apply_text_level_fixes(mut formatted_code: String) -> String {
    // DEPYLER-CONVERGE-MULTI: Strip `if TYPE_CHECKING {}` that leaks through codegen.
    // The ast_bridge skips top-level TYPE_CHECKING blocks, but they can appear in
    // synthesized main() or in function bodies processed via StmtConverter::convert_if.
    // Robust fallback: remove the statement from generated code at text level.
    // Use line-based filtering for robustness against varying indentation.
    formatted_code = formatted_code
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            trimmed != "if TYPE_CHECKING {}" && trimmed != "if TYPE_CHECKING { }"
        })
        .collect::<Vec<_>>()
        .join("\n");
    if !formatted_code.ends_with('\n') {
        formatted_code.push('\n');
    }

    // DEPYLER-CONVERGE-MULTI: Fix `type(x).__name__` pattern.
    // Python: type(n).__name__ returns the type name as a string.
    // Transpiler emits: std::any::type_name_of_val(&n).__name__
    // But type_name_of_val already returns &str, so .__name__ is invalid.
    // Strip the trailing .__name__ since the function already gives us what we need.
    // Also handle .__name as a field access (E0609).
    while formatted_code.contains(".__name__") {
        formatted_code = formatted_code.replace(".__name__", "");
    }

    // DEPYLER-CONVERGE-MULTI: Map typing.Sequence<T> to &[T] (slice reference).
    // Python's typing.Sequence is an abstract read-only sequence type.
    // In Rust, &[T] is the idiomatic equivalent for borrowed sequence data.
    formatted_code = formatted_code.replace("Sequence<i32>", "&[i32]");
    formatted_code = formatted_code.replace("Sequence<i64>", "&[i64]");
    formatted_code = formatted_code.replace("Sequence<f64>", "&[f64]");
    formatted_code = formatted_code.replace("Sequence<String>", "&[String]");
    formatted_code = formatted_code.replace("Sequence<bool>", "&[bool]");
    formatted_code = formatted_code.replace("Sequence<u8>", "&[u8]");

    // DEPYLER-CONVERGE-MULTI: Fix enum/class path separator (E0423).
    formatted_code = fix_enum_path_separator(&formatted_code);

    // DEPYLER-CONVERGE-MULTI: Fix Python truthiness on non-bool types (E0600).
    formatted_code = fix_python_truthiness(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER5: Fix io.StringIO patterns (E0061 + E0599).
    formatted_code = formatted_code.replace(
        "std::io::Cursor::new()",
        "std::io::Cursor::new(Vec::<u8>::new())",
    );
    formatted_code = formatted_code.replace(
        ".getvalue()",
        ".get_ref().iter().map(|&b| b as char).collect::<String>()",
    );

    // DEPYLER-CONVERGE-MULTI-ITER5: Fix TypeError::new pattern (E0425).
    formatted_code = formatted_code.replace(
        "(TypeError::new(",
        "(std::io::Error::new(std::io::ErrorKind::InvalidInput, ",
    );

    // DEPYLER-CONVERGE-MULTI-ITER5: Fix docstring-in-main syntax errors.
    formatted_code = fix_docstring_in_main(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER5: Fix operator.mul and similar operator references (E0425).
    formatted_code = formatted_code.replace("operator.mul", "|a, b| a * b");
    formatted_code = formatted_code.replace("operator.add", "|a, b| a + b");
    formatted_code = formatted_code.replace("operator.sub", "|a, b| a - b");

    // DEPYLER-1404: Fix stub function arities to match call sites.
    // Stubs generated with 1 param cause E0061 when called with multiple args.
    formatted_code = fix_stub_arities(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER5b: Inject missing std imports detected by usage.
    if formatted_code.contains(".write_all(") && !formatted_code.contains("use std::io::Write") {
        formatted_code = format!("use std::io::Write;\n{}", formatted_code);
    }
    if formatted_code.contains("HashMap")
        && !formatted_code.contains("use std::collections::HashMap")
    {
        formatted_code = format!("use std::collections::HashMap;\n{}", formatted_code);
    }
    formatted_code = formatted_code.replace(".py_sub(", " - (");

    // DEPYLER-CONVERGE-MULTI-ITER6: Fix TypeError::new in broader contexts (E0433).
    formatted_code = formatted_code.replace(
        ", TypeError::new(",
        ", std::io::Error::new(std::io::ErrorKind::InvalidInput, ",
    );
    formatted_code = formatted_code.replace(
        " TypeError::new(",
        " std::io::Error::new(std::io::ErrorKind::InvalidInput, ",
    );

    // DEPYLER-CONVERGE-MULTI-ITER6: Fix Python `type()` builtin (E0425).
    if formatted_code.contains("r#type(") {
        formatted_code = formatted_code.replace("r#type(", "py_type_name(&");
        let helper = "fn py_type_name<T: ?Sized>(_: &T) -> &'static str { \
                       std::any::type_name::<T>() }\n";
        formatted_code = format!("{}{}", helper, formatted_code);
    }

    // DEPYLER-CONVERGE-MULTI-ITER6: Fix `.into_iter()` on borrowed vecs (E0308).
    formatted_code = fix_borrow_into_iter_chain(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER6: Fix dot-access on enum variants (E0423/E0573).
    formatted_code = fix_enum_dot_to_path_separator(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER6: Fix pathlib::PurePosixPath (E0425).
    formatted_code = formatted_code.replace("pathlib::PurePosixPath(", "std::path::Path::new(");
    formatted_code = formatted_code.replace("pathlib::Path(", "std::path::Path::new(");

    // DEPYLER-CONVERGE-MULTI-ITER6: Fix `.days()` on datetime types (E0599).
    formatted_code = formatted_code.replace(".days()", ".day()");

    // DEPYLER-CONVERGE-MULTI-ITER6: Fix `DynDigest` trait reference (E0405).
    formatted_code =
        formatted_code.replace("as Box<dyn DynDigest>", "as Box<dyn std::hash::Hasher>");

    // DEPYLER-CONVERGE-MULTI-ITER6: Fix hex::encode references (E0433).
    if formatted_code.contains("hex::encode(") && !formatted_code.contains("fn hex_encode") {
        formatted_code = formatted_code.replace("hex::encode(", "hex_encode(");
        let helper = "fn hex_encode(bytes: impl AsRef<[u8]>) -> String { \
                       bytes.as_ref().iter().map(|b| format!(\"{:02x}\", b)).collect() }\n";
        formatted_code = format!("{}{}", helper, formatted_code);
    }

    // DEPYLER-CONVERGE-MULTI-ITER7: Fix generator yield scope (E0425 on `items`).
    formatted_code = fix_generator_yield_scope(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER7: Fix BufReader.deserialize() (E0599).
    formatted_code = fix_bufreader_deserialize(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER7: Fix checked_pow + sqrt type mismatch (E0277).
    formatted_code = fix_power_sqrt_types(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER7: Fix DepylerDateTime subtraction (E0369).
    formatted_code = fix_datetime_subtraction(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER7: Fix Hasher .update()/.finalize_reset() (E0599).
    formatted_code = fix_hasher_digest_methods(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER8: Fix HashMap<String, ()> empty dict default (E0308).
    formatted_code = fix_hashmap_empty_value_type(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER8: Fix String→PathOrStringUnion coercion (E0308).
    formatted_code = fix_path_or_string_union_coercion(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER8: Fix function stubs used as types (E0308/E0433).
    formatted_code = fix_function_stub_as_type(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER8: Fix heterogeneous dict inserts (E0308).
    formatted_code = fix_heterogeneous_dict_inserts(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER9: Fix LazyLock<String> static blocks used as types (E0573).
    formatted_code = fix_lazylock_static_as_type(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER9: Repair malformed LazyLock static initializers (E0599/E0605).
    formatted_code = fix_broken_lazylock_initializers(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER9: Fix `Literal.clone().py_index(...)` blocks (E0425/E0605).
    formatted_code = fix_literal_clone_pattern(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER9: Fix `frozenset<T>` → `HashSet<T>` (E0425).
    formatted_code = formatted_code.replace("frozenset<", "HashSet<");

    // DEPYLER-CONVERGE-MULTI-ITER9: Fix `!string_var` → `string_var.is_empty()` (E0600).
    formatted_code = fix_negation_on_non_bool(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER10: Fix `!config.field` → `config.field.is_empty()` (E0600).
    formatted_code = fix_field_access_truthiness(&formatted_code);

    // DEPYLER-99MODE-S9: Fix HashMap `.contains()` → `.contains_key()` (E0599).
    formatted_code = fix_hashmap_contains(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER9: Generalize DepylerValue insert wrapping (E0308).
    formatted_code = fix_depyler_value_inserts_generalized(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER9: Add Display impl for enums (E0599).
    formatted_code = fix_enum_display(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER9: Remove orphaned LazyLock initializer bodies.
    formatted_code = fix_orphaned_lazylock_bodies(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER9: Fix DepylerValue Str match arm missing .into_iter().
    formatted_code = fix_depyler_value_str_match_arm(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER9: Fix inline block expressions missing closing parens.
    formatted_code = fix_inline_block_expression_parens(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER9: Fix orphaned `;)` after for-loop closings.
    formatted_code = fix_orphaned_semicolon_paren(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER10: Fix sorted_vec reference pattern.
    formatted_code = fix_sorted_vec_reference(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER10: Fix Vec<String>.contains(&*str_var) → .iter().any()
    formatted_code = fix_vec_contains_deref(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER10: Fix Vec.get(&string_ref).is_some() → .iter().any()
    formatted_code = fix_vec_get_membership(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER10: Fix integer literals in f64 comparisons.
    formatted_code = fix_float_int_comparison(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER10b: Inject new() constructors for enums.
    formatted_code = fix_enum_new_constructor(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER10b: Strip extra args from enum new() calls.
    formatted_code = fix_enum_new_call_args(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER10b: Fix .is_none() on non-Option struct references.
    formatted_code = fix_is_none_on_non_option(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER10b: Fix Vec<char>.join("") → .collect::<String>().
    formatted_code = fix_vec_char_join(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER11: Fix borrowed type-alias params in ::new() calls.
    formatted_code = fix_borrowed_alias_in_new_calls(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER11: Fix (*var) == "literal" deref comparisons.
    formatted_code = fix_deref_string_comparison(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER11: Fix Vec<DepylerValue>.join("sep").
    formatted_code = fix_depyler_value_vec_join(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER12: Fix `!string_expr.trim().to_string()` truthiness.
    formatted_code = fix_not_string_truthiness(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER12: Fix `r#false` and `r#true` raw identifiers.
    formatted_code = fix_raw_identifier_booleans(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER12: Fix spurious dereference on unwrap results.
    formatted_code = fix_deref_unwrap_result(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER12: Fix &str params passed to ::new() constructors.
    formatted_code = fix_str_params_in_new_calls(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER12: Fix String inserted into HashMap<String, DepylerValue>.
    formatted_code = fix_string_to_depyler_value_insert(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER12b: Fix [String].contains(&str_param) membership tests.
    formatted_code = fix_string_array_contains(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER12b: Fix config.field move from &Config in DepylerValue::Str().
    formatted_code = fix_depyler_value_str_clone(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER12b: Fix &Option<T> params passed where Option<T> expected.
    formatted_code = fix_ref_option_in_new(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER12c: Fix (*ref_option.unwrap_or_default()) deref pattern.
    formatted_code = fix_deref_ref_option_unwrap(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER13: Replace DepylerValue::from(EnumType::X) with
    // DepylerValue::Str(format!("{:?}", EnumType::X)) for generated enum types.
    formatted_code = fix_depyler_value_from_enum(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER13: Add type annotations to CSE temps in py_mul/py_div chains.
    formatted_code = fix_cse_py_mul_type_annotation(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER13: Add From<EnumType> for DepylerValue impls.
    formatted_code = fix_add_enum_from_impls(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER13: Fix validate_not_none 2-arg calls vs 1-arg definition.
    formatted_code = fix_validate_not_none_args(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER14: Fix HashMap key type mismatch.
    formatted_code = fix_hashmap_key_type_mismatch(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER14: Fix tuple(T, DepylerValue) fields → Vec<T>.
    formatted_code = fix_tuple_to_vec_when_len_called(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER15: Fix CSE temps that compare i32 variables with f64 literals.
    formatted_code = fix_cse_int_float_comparison(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER15: Fix .to_string() on Vec<Struct> types that lack Display.
    formatted_code = fix_vec_to_string_debug(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER15: Fix HashMap<DepylerValue, DV> blocks.
    formatted_code = fix_depyler_value_hashmap_keys(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER15: Fix depyler_min/depyler_max with mixed i32/f64 args.
    formatted_code = fix_mixed_numeric_min_max(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER16: Fix bitwise AND used in boolean context.
    formatted_code = fix_bitwise_and_truthiness(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER16: Fix spurious .to_i64()/.as_i64() on i32 values.
    formatted_code = fix_spurious_i64_conversion(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER16: Fix Ok() double-wrapping of Result-returning function calls.
    formatted_code = fix_result_double_wrap(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER17: Fix trailing comma creating tuples in arithmetic.
    formatted_code = fix_trailing_comma_in_arith_parens(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER17: Fix &ref → &mut ref at call sites.
    formatted_code = fix_immutable_ref_to_mut(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER17: Fix .to_string() where &str expected.
    formatted_code = fix_regex_match_string_arg(&formatted_code);

    // DEPYLER-99MODE-S9: Fix `format!(...).expect(...)` on String (E0599).
    formatted_code = fix_format_expect(&formatted_code);

    // DEPYLER-99MODE-S9: Fix `(*pos).expect(...)` on non-Option types (E0614).
    formatted_code = fix_deref_expect_on_primitive(&formatted_code);

    // DEPYLER-99MODE-S9: Fix `self.day.to_string()` in DepylerDate::new() (E0308).
    formatted_code = fix_spurious_to_string_in_numeric_call(&formatted_code);

    // DEPYLER-99MODE-S9: Fix &str param returned where -> String expected (E0308).
    formatted_code = fix_str_param_return_as_string(&formatted_code);

    // DEPYLER-99MODE-S9: Fix bool.as_bool() and bool.unwrap_or_default() (E0599).
    formatted_code = fix_as_bool_on_bool(&formatted_code);

    // DEPYLER-99MODE-S9: Fix bare `Range` type annotation → `PyRange` (E0425).
    formatted_code = fix_range_type_annotation(&formatted_code);

    // DEPYLER-99MODE-S9: Fix HashMap .keys() producing &String in tuple push (E0308).
    formatted_code = fix_hashmap_keys_iter_clone(&formatted_code);

    // DEPYLER-99MODE-S9: Fix String::from_utf8_lossy(&string_var) (E0308).
    formatted_code = fix_from_utf8_lossy_string_arg(&formatted_code);

    // DEPYLER-99MODE-S9: Fix closure passed where &dyn Fn expected (E0308).
    formatted_code = fix_closure_to_dyn_fn_ref(&formatted_code);

    // DEPYLER-99MODE-S9: Fix &item passed to impl Fn(String) param (E0308).
    formatted_code = fix_ref_arg_to_fn_string_param(&formatted_code);

    // DEPYLER-99MODE-S9: Fix (*var.expect()).expect() double-unwrap (E0614).
    formatted_code = fix_double_expect_on_option_ref(&formatted_code);

    // DEPYLER-99MODE-S9: Fix usize.to_string() in constructor args (E0308).
    formatted_code = fix_usize_to_string_in_constructor(&formatted_code);

    // DEPYLER-99MODE-S9: Fix PyRange.iter().cloned() → start..stop (E0599).
    formatted_code = fix_pyrange_iteration(&formatted_code);

    // DEPYLER-99MODE-S9: Fix move closure capturing vars used later (E0382).
    // Disabled: current heuristic is too broad, cloning vars from other scopes.
    // formatted_code = fix_move_closure_capture(&formatted_code);

    // DEPYLER-99MODE-S9: Fix unclosed vec![] macro (syntax error).
    formatted_code = fix_unclosed_vec_macro(&formatted_code);

    // DEPYLER-99MODE-S9: Fix missing inherited fields in child structs (E0609).
    formatted_code = fix_missing_inherited_fields(&formatted_code);

    // DEPYLER-99MODE-S9: De-async: remove tokio dependency for single-file compilation (E0433).
    formatted_code = fix_remove_async_for_standalone(&formatted_code);

    // DEPYLER-99MODE-S9: Fix &var passed to fn expecting owned String (E0308).
    formatted_code = fix_ref_string_to_owned_in_call(&formatted_code);

    // DEPYLER-99MODE-S9: Fix let var → let mut var for &mut self methods (E0596).
    formatted_code = fix_missing_mut_for_method_calls(&formatted_code);

    // DEPYLER-99MODE-S9: Fix wrong type annotation on let bindings with .collect() (E0308).
    formatted_code = fix_collect_type_annotation_mismatch(&formatted_code);

    // DEPYLER-99MODE-S9: Fix negative integer literal without type annotation (E0277/E0689).
    formatted_code = fix_negative_literal_type_annotation(&formatted_code);

    // DEPYLER-99MODE-S9: Fix empty vec![] in assert_eq!/assert_ne! (E0282/E0283).
    formatted_code = fix_empty_vec_in_assert(&formatted_code);

    // DEPYLER-99MODE-S9: Fix DepylerValue→typed variable assignment missing .into() (E0308).
    formatted_code = fix_depyler_value_to_typed_assignment(&formatted_code);

    // DEPYLER-99MODE-S9: Fix format!("{:?}", N) in vec literal where i32 expected (E0308).
    formatted_code = fix_format_debug_in_int_vec(&formatted_code);

    // DEPYLER-99MODE-S9: Fix ambiguous .into() on chain for String context (E0282).
    formatted_code = fix_ambiguous_into_on_chain(&formatted_code);

    // DEPYLER-99MODE-S9: Fix .contains() → .contains_key() on HashMap (E0599).
    formatted_code = fix_hashmap_contains_to_contains_key(&formatted_code);

    // DEPYLER-99MODE-S9: Fix ambiguous .into() after DV chain (E0283).
    formatted_code = fix_ambiguous_into_type_annotation(&formatted_code);

    // DEPYLER-99MODE-S9: Fix `let q = a / b;` missing type annotation (E0282).
    formatted_code = fix_floor_div_type_annotation(&formatted_code);

    // DEPYLER-99MODE-S9: Fix -fn_call() when fn returns Result (E0600).
    formatted_code = fix_negate_result_fn_call(&formatted_code);

    // DEPYLER-99MODE-S9: Fix `return dv_param` when fn returns i32/f64/String (E0308).
    formatted_code = fix_return_depyler_value_param(&formatted_code);

    // DEPYLER-99MODE-S9: Fix &str.clone() → .to_string() in Vec<String> context (E0308).
    formatted_code = fix_str_clone_to_string(&formatted_code);

    // DEPYLER-99MODE-S9: Fix Option<i32> as u32 cast (E0605).
    formatted_code = fix_option_as_cast(&formatted_code);

    // DEPYLER-99MODE-S9: Fix .unwrap_or(0) on Option<DepylerValue> (E0308).
    formatted_code = fix_unwrap_or_depyler_value(&formatted_code);

    // DEPYLER-99MODE-S9: Fix (Ni32) as i64 in i32 context (E0308).
    formatted_code = fix_i32_as_i64_cast(&formatted_code);

    // DEPYLER-99MODE-S9: Fix Vec::<i32>::new() where nested vec expected (E0277).
    formatted_code = fix_nested_vec_type_in_assert(&formatted_code);

    // DEPYLER-99MODE-S9: Fix dict.get().cloned() return without unwrap (E0308).
    formatted_code = fix_dict_get_return(&formatted_code);

    // DEPYLER-99MODE-S9: Fix .as_ref() ambiguity for File::create (E0282).
    formatted_code = fix_string_as_ref_ambiguity(&formatted_code);

    // DEPYLER-99MODE-S9: Fix dequeue()/pop_front() Option unwrap (E0308).
    formatted_code = fix_option_dequeue_unwrap(&formatted_code);

    // DEPYLER-99MODE-S9: Fix Option<HashMap>.contains_key() (E0599).
    formatted_code = fix_option_hashmap_contains_key(&formatted_code);

    // DEPYLER-99MODE-S9: Fix char.as_str() → char.to_string() (E0599).
    formatted_code = fix_char_as_str(&formatted_code);

    // DEPYLER-99MODE-S9: Fix result.push(value) where value is Option after is_some() guard.
    formatted_code = fix_option_push_after_is_some(&formatted_code);

    // DEPYLER-99MODE-S9: Fix DepylerValue::Str("literal") → .to_string() (E0308).
    formatted_code = fix_depyler_value_str_literal(&formatted_code);

    // DEPYLER-99MODE-S9: Fix Vec arg type mismatch in assert fn calls (E0308).
    formatted_code = fix_vec_arg_type_in_assert(&formatted_code);

    // DEPYLER-99MODE-S9: Fix format!("{}", vec) → format!("{:?}", vec) (E0277).
    formatted_code = fix_format_vec_display(&formatted_code);

    // DEPYLER-99MODE-S9: Fix .iter() on impl Iterator (E0599).
    formatted_code = fix_iter_on_impl_iterator(&formatted_code);

    // DEPYLER-99MODE-S9: Fix *self.field.unwrap_or() deref on non-ref (E0614).
    formatted_code = fix_deref_on_unwrap_or(&formatted_code);

    // DEPYLER-99MODE-S9: Fix .py_index(DepylerValue::Int(EXPR)) → .py_index((EXPR) as i64) (E0277).
    formatted_code = fix_pyindex_depyler_value_wrapper(&formatted_code);

    // DEPYLER-99MODE-S9: Fix self.field = expr → Some(expr) for Option<T> fields (E0308).
    formatted_code = fix_option_field_assignment(&formatted_code);

    // DEPYLER-99MODE-S9: Fix option.clone().to_string() in is_some() guard → unwrap() (E0599).
    formatted_code = fix_option_to_string_in_is_some_guard(&formatted_code);

    // DEPYLER-99MODE-S9: Fix return &Option<T> param in is_some() guard → unwrap (E0308).
    formatted_code = fix_return_option_param_in_is_some(&formatted_code);

    // DEPYLER-99MODE-S9: Fix let _ = Ok(expr); → Ok(expr) as tail expression (E0308).
    formatted_code = fix_let_discard_ok_return(&formatted_code);

    // DEPYLER-99MODE-S9: Fix void fn with return value as i32 → add -> i32 (E0308).
    formatted_code = fix_void_fn_with_return_value(&formatted_code);

    // DEPYLER-99MODE-S9: Fix Vec<()> → Vec<(String, i32)> from tuple contents (E0609).
    formatted_code = fix_unit_vec_to_tuple_type(&formatted_code);

    // DEPYLER-99MODE-S9: Fix Vec::<T>::new() → vec![] in assert_eq (let Rust infer type) (E0277).
    // Disabled: DepylerValue PartialEq<i32> impl causes vec![] inference ambiguity.
    // formatted_code = fix_empty_vec_new_in_assert(&formatted_code);

    // DEPYLER-99MODE-S9: Fix bare return in Result fn → wrap in Ok() (E0308).
    formatted_code = fix_bare_return_in_result_fn(&formatted_code);

    // DEPYLER-99MODE-S9: Fix let x: () = expr → let x = expr (remove unit type annotation) (E0308).
    formatted_code = fix_let_unit_type_annotation(&formatted_code);

    // DEPYLER-99MODE-S9: Fix let x: WRONG = fn(args) where fn returns CORRECT (E0308).
    formatted_code = fix_let_type_from_fn_return(&formatted_code);

    // DEPYLER-99MODE-S9: Fix Vec::<WRONG>::new() in assert by inferring from fn return type (E0277).
    formatted_code = fix_assert_vec_type_from_fn_return(&formatted_code);

    // DEPYLER-99MODE-S9: Refine fn(&Vec<DepylerValue>) → fn(&Vec<concrete>) from call-site types (E0308).
    formatted_code = fix_vec_depyler_value_param_from_callsite(&formatted_code);

    // DEPYLER-99MODE-S9: Remove spurious .into() in middle of DepylerValue chains (E0282).
    // Disabled: DepylerValue.get() takes &DepylerValue not &str; .into() converts to HashMap
    // which accepts &str keys. Removing .into() breaks the chain.
    // formatted_code = fix_into_in_depyler_value_chain(&formatted_code);

    // DEPYLER-99MODE-S9: Replace DepylerValue::from(EXPR) with EXPR in concrete contexts (E0308).
    // Disabled: too aggressive, removes wrapping in DepylerValue contexts where it's needed.
    // formatted_code = fix_depyler_value_from_in_concrete_tuple(&formatted_code);

    // DEPYLER-99MODE-S9: Fix .push_back() → .push() (Vec has no push_back) (E0599).
    // Disabled: VecDeque legitimately uses .push_back(). Would need type tracking.
    // formatted_code = fix_push_back_to_push(&formatted_code);

    // DEPYLER-99MODE-S9: Fix HashMap<K,V> type when inserts have swapped arg order (E0308).
    // Disabled: too aggressive, swaps all occurrences including correct ones.
    // formatted_code = fix_hashmap_type_annotation_mismatch(&formatted_code);

    // DEPYLER-99MODE-S9: Fix .write_all(s.as_bytes()) → .write(s.to_string()) on custom struct (E0599).
    // Disabled: also affects std::fs::File.write_all() which should stay as-is.
    // formatted_code = fix_write_all_on_custom_struct(&formatted_code);

    // DEPYLER-99MODE-S9: Fix missing & on owned value passed to fn expecting reference (E0308).
    // Disabled: too aggressive, adds & in places where args are already correct types.
    // formatted_code = fix_missing_ref_in_fn_call(&formatted_code);

    formatted_code
}

/// DEPYLER-1404: Convert stub functions to macros and rewrite call sites.
///
/// Stub functions accept only 1 parameter, causing E0061 when called with more.
/// This converts stubs to `macro_rules!` (variadic args) and rewrites call sites
/// to add `!` for macro invocation syntax.
fn fix_stub_arities(code: &str) -> String {
    let stub_names = find_stub_names(code);
    if stub_names.is_empty() {
        return code.to_string();
    }

    let mut result = code.to_string();
    let mut macros_to_prepend: Vec<String> = Vec::new();

    for name in &stub_names {
        if let Some((cleaned, macro_def)) = replace_stub_with_macro(&result, name) {
            result = cleaned;
            macros_to_prepend.push(macro_def);
            result = rewrite_call_sites(&result, name);
        }
    }

    if !macros_to_prepend.is_empty() {
        let prefix = macros_to_prepend.join("\n");
        result = format!("{}\n{}", prefix, result);
    }

    result
}

/// Find stub function names by scanning for DEPYLER-0615 marker comments.
fn find_stub_names(code: &str) -> Vec<String> {
    let mut stub_names: Vec<String> = Vec::new();
    let lines: Vec<&str> = code.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        if !line.contains("DEPYLER-0615") {
            continue;
        }
        for j in (i + 1)..lines.len().min(i + 5) {
            if let Some(fname) = extract_fn_name(lines[j]) {
                stub_names.push(fname);
                break;
            }
        }
    }
    stub_names
}

/// Extract function name from a `pub fn name(...)` declaration line.
fn extract_fn_name(line: &str) -> Option<String> {
    let fname = line
        .trim()
        .strip_prefix("pub fn ")?
        .split('(')
        .next()?
        .split('<')
        .next()?
        .trim();
    if fname.is_empty() {
        return None;
    }
    Some(fname.to_string())
}

/// Remove a stub function and return the cleaned code + macro definition.
fn replace_stub_with_macro(code: &str, name: &str) -> Option<(String, String)> {
    let fn_dv = format!(
        "pub fn {}(_args: impl std::any::Any) -> DepylerValue {{\n    DepylerValue::default()\n}}",
        name
    );
    let fn_unit = format!(
        "pub fn {}(_args: impl std::any::Any) -> () {{\n}}",
        name
    );

    let is_unit = code.contains(&fn_unit);
    let is_dv = code.contains(&fn_dv);
    if !is_unit && !is_dv {
        return None;
    }

    let mut result = if is_dv {
        code.replace(&fn_dv, "")
    } else {
        code.replace(&fn_unit, "")
    };
    result = result.replace("/// DEPYLER-0615: Generated to allow standalone compilation", "");
    result = result.replace("#[allow(dead_code, unused_variables)]", "");

    let macro_def = if is_unit {
        format!("macro_rules! {} {{ ($($args:expr),* $(,)?) => {{ () }}; }}", name)
    } else {
        format!(
            "macro_rules! {} {{ ($($args:expr),* $(,)?) => {{ DepylerValue::default() }}; }}",
            name
        )
    };

    Some((result, macro_def))
}

/// Rewrite call sites from `name(` to `name!(`, skipping macro definitions.
fn rewrite_call_sites(code: &str, name: &str) -> String {
    let call_pattern = format!("{}(", name);
    let macro_call = format!("{}!(", name);
    let macro_def_marker = format!("macro_rules! {}", name);

    code.lines()
        .map(|line| {
            if line.contains(&macro_def_marker) || !line.contains(&call_pattern) {
                line.to_string()
            } else {
                line.replace(&call_pattern, &macro_call)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
