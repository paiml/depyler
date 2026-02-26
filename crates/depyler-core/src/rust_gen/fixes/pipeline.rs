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
    formatted_code =
        formatted_code.replace("std::io::Cursor::new()", "std::io::Cursor::new(Vec::<u8>::new())");
    formatted_code = formatted_code
        .replace(".getvalue()", ".get_ref().iter().map(|&b| b as char).collect::<String>()");

    // DEPYLER-CONVERGE-MULTI-ITER5: Fix TypeError::new pattern (E0425).
    formatted_code = formatted_code
        .replace("(TypeError::new(", "(std::io::Error::new(std::io::ErrorKind::InvalidInput, ");

    // DEPYLER-CONVERGE-MULTI-ITER5: Fix docstring-in-main syntax errors.
    formatted_code = fix_docstring_in_main(&formatted_code);

    // DEPYLER-CONVERGE-MULTI-ITER5: Fix operator.mul and similar operator references (E0425).
    formatted_code = formatted_code.replace("operator.mul", "|a, b| a * b");
    formatted_code = formatted_code.replace("operator.add", "|a, b| a + b");
    formatted_code = formatted_code.replace("operator.sub", "|a, b| a - b");

    // DEPYLER-1404: Fix DepylerValue `as` casts (E0605).
    // DepylerValue can't use `as f64` — replace with comparison without cast
    // since DepylerValue already implements PartialEq for numeric types.
    formatted_code = fix_depyler_value_casts(&formatted_code);

    // DEPYLER-1404: Fix struct field assignment with raw literals (E0308).
    // Wraps `p.x = 5.0;` → `p.x = DepylerValue::Float(5.0);` for stub struct fields.
    formatted_code = fix_struct_field_literal_assignment(&formatted_code);

    // DEPYLER-1404: Hoist variable declarations out of with-statement blocks (E0425).
    // Python `with` blocks transpile to `{ let mut var = ...; { body } }` but
    // assert statements after the block can't see `var`. Hoist the let binding.
    formatted_code = fix_with_block_scope(&formatted_code);

    // DEPYLER-1404: Add .clone() to arguments of stub struct instance methods (E0382/E0505).
    // When a struct instance is passed as an argument to its own method, or reused
    // after being consumed, we need .clone() to satisfy the borrow checker.
    formatted_code = fix_stub_method_ownership(&formatted_code);

    // DEPYLER-1404: Wrap nested HashMap block expressions as DepylerValue::Dict (E0308).
    // Inner maps `{ let mut map = HashMap::new(); map.insert(...); map }` inside
    // outer DepylerValue inserts need wrapping as DepylerValue::Dict.
    formatted_code = fix_nested_hashmap_to_depyler_value(&formatted_code);

    // DEPYLER-1404: Replace bare UnionType with DepylerValue (E0425).
    // Python Union[...] type aliases sometimes emit `UnionType` which doesn't exist.
    formatted_code = formatted_code
        .replace("pub type JsonValue = UnionType;", "pub type JsonValue = DepylerValue;");
    formatted_code = formatted_code.replace("= UnionType;", "= DepylerValue;");
    formatted_code = formatted_code.replace(": UnionType", ": DepylerValue");
    formatted_code = formatted_code.replace("<UnionType>", "<DepylerValue>");
    formatted_code = formatted_code.replace("(UnionType)", "(DepylerValue)");

    // DEPYLER-1404: Disambiguate empty vec![] comparisons with DepylerValue (E0283).
    // Multiple PartialEq<Vec<T>> impls make `== vec![]` ambiguous.
    formatted_code = formatted_code.replace("== vec![]", "== Vec::<DepylerValue>::new()");
    formatted_code = formatted_code.replace("!= vec![]", "!= Vec::<DepylerValue>::new()");
    // Also disambiguate empty HashMap::new() in comparisons (E0282).
    formatted_code = formatted_code.replace(
        "== {\n                let mut map = std::collections::HashMap::new();\n                map\n            }",
        "== std::collections::HashMap::<String, DepylerValue>::new()"
    );

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
    // DEPYLER-1404: Fix py_div on PathBuf — Python `/` operator for path joining.
    formatted_code = formatted_code.replace(".py_div(", ".join(");

    // DEPYLER-CONVERGE-MULTI-ITER6: Fix TypeError::new in broader contexts (E0433).
    formatted_code = formatted_code
        .replace(", TypeError::new(", ", std::io::Error::new(std::io::ErrorKind::InvalidInput, ");
    formatted_code = formatted_code
        .replace(" TypeError::new(", " std::io::Error::new(std::io::ErrorKind::InvalidInput, ");

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

    // DEPYLER-1404: Fix stub function arities to match call sites.
    // Must run AFTER fix_enum_dot_to_path_separator so class-type stubs
    // see `Color::` notation (not `Color.`) when scanning for usage.
    formatted_code = fix_stub_arities(&formatted_code);

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

    let mut struct_stubs_to_prepend: Vec<String> = Vec::new();

    for name in &stub_names {
        // DEPYLER-1404: Names used with `::` syntax are class imports.
        // Replace function stub with a stub struct that supports field/method access.
        let type_path = format!("{}::", name);
        if result.contains(&type_path) {
            if let Some(cleaned) = remove_stub_function(&result, name) {
                result = cleaned;
                let (struct_def, ctor_macro) = generate_stub_struct(name, &result);
                struct_stubs_to_prepend.push(struct_def);
                if let Some(mac) = ctor_macro {
                    macros_to_prepend.push(mac);
                    // Rewrite Name::new( to Name_new!( directly
                    let new_call = format!("{}::new(", name);
                    let macro_call = format!("{}_new!(", name);
                    result = result.replace(&new_call, &macro_call);
                }
            }
            continue;
        }
        if let Some((cleaned, macro_def)) = replace_stub_with_macro(&result, name) {
            result = cleaned;
            macros_to_prepend.push(macro_def);
            result = rewrite_call_sites(&result, name);
        }
    }

    if !macros_to_prepend.is_empty() || !struct_stubs_to_prepend.is_empty() {
        let mut prefix_parts = macros_to_prepend;
        prefix_parts.extend(struct_stubs_to_prepend);
        let prefix = prefix_parts.join("\n");
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
        for next_line in lines.iter().take(lines.len().min(i + 5)).skip(i + 1) {
            if let Some(fname) = extract_fn_name(next_line) {
                stub_names.push(fname);
                break;
            }
        }
    }
    stub_names
}

/// Extract function name from a `pub fn name(...)` declaration line.
fn extract_fn_name(line: &str) -> Option<String> {
    let fname = line.trim().strip_prefix("pub fn ")?.split('(').next()?.split('<').next()?.trim();
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
    let fn_unit = format!("pub fn {}(_args: impl std::any::Any) -> () {{\n}}", name);

    let is_unit = code.contains(&fn_unit);
    let is_dv = code.contains(&fn_dv);
    if !is_unit && !is_dv {
        return None;
    }

    let mut result = if is_dv { code.replace(&fn_dv, "") } else { code.replace(&fn_unit, "") };
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

/// Rewrite call sites from `name(` to `name!(`, with word-boundary awareness.
///
/// Only rewrites when `name(` appears at a word boundary (not preceded by
/// an alphanumeric char or underscore), to avoid mangling identifiers like
/// `test_basic_product(` when the stub name is `product`.
fn rewrite_call_sites(code: &str, name: &str) -> String {
    let macro_def_marker = format!("macro_rules! {}", name);

    code.lines()
        .map(|line| {
            if line.contains(&macro_def_marker) {
                return line.to_string();
            }
            rewrite_line_call_sites(line, name)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Replace `name(` with `name!(` in a single line, respecting word boundaries.
fn rewrite_line_call_sites(line: &str, name: &str) -> String {
    let pattern = format!("{}(", name);
    let mut result = String::with_capacity(line.len());
    let mut remaining = line;

    while let Some(pos) = remaining.find(&pattern) {
        // Check character before match for word boundary
        let is_word_boundary = if pos == 0 {
            true
        } else {
            let prev = remaining.as_bytes()[pos - 1];
            !prev.is_ascii_alphanumeric() && prev != b'_'
        };

        if is_word_boundary {
            result.push_str(&remaining[..pos]);
            result.push_str(name);
            result.push('!');
            result.push('(');
            remaining = &remaining[pos + pattern.len()..];
        } else {
            result.push_str(&remaining[..pos + pattern.len()]);
            remaining = &remaining[pos + pattern.len()..];
        }
    }
    result.push_str(remaining);
    result
}

/// Remove a stub function declaration (for class-type stubs that will become structs).
fn remove_stub_function(code: &str, name: &str) -> Option<String> {
    let fn_dv = format!(
        "pub fn {}(_args: impl std::any::Any) -> DepylerValue {{\n    DepylerValue::default()\n}}",
        name
    );
    let fn_unit = format!("pub fn {}(_args: impl std::any::Any) -> () {{\n}}", name);

    let mut result = if code.contains(&fn_dv) {
        code.replace(&fn_dv, "")
    } else if code.contains(&fn_unit) {
        code.replace(&fn_unit, "")
    } else {
        return None;
    };
    // Clean up associated doc/attribute lines
    result = result.replace("/// DEPYLER-0615: Generated to allow standalone compilation", "");
    result = result.replace("#[allow(dead_code, unused_variables)]", "");
    Some(result)
}

/// Generate a stub struct for class-type imports.
///
/// Scans the code for `Name::CONSTANT`, `Name::method()`, and instance
/// method calls (`.method()` on constants) to generate a compilable stub.
fn generate_stub_struct(name: &str, code: &str) -> (String, Option<String>) {
    let (members, instance_methods) = scan_class_usage(name, code);

    // Detect fields (instance_methods entries that are field accesses, not method calls)
    let mut fields: Vec<String> = Vec::new();
    for method in &instance_methods {
        if method == "value" {
            continue;
        }
        let method_call = format!(".{}(", method);
        let field_assign = format!(".{} =", method);
        let is_method_call = code.contains(&method_call);
        let is_field_assign = code.contains(&field_assign);
        if is_field_assign || !is_method_call {
            fields.push(method.clone());
        }
    }

    let mut parts = Vec::new();
    let mut struct_fields = vec!["pub value: DepylerValue".to_string()];
    for field in &fields {
        struct_fields.push(format!("pub {}: DepylerValue", field));
    }
    parts.push(format!(
        "#[derive(Debug, Clone, PartialEq)] pub struct {} {{ {} }}",
        name,
        struct_fields.join(", ")
    ));

    let impl_items = build_struct_impl_items(name, code, &members, &instance_methods);
    parts.push(format!("impl {} {{ {} }}", name, impl_items.join("\n")));
    parts.push(format!(
        "impl std::ops::Deref for {} {{ type Target = DepylerValue; fn deref(&self) -> &Self::Target {{ &self.value }} }}",
        name
    ));

    // If `new` is used, generate a constructor macro for variadic args
    let ctor_macro = if members.contains(&"new".to_string()) {
        let field_defaults: Vec<String> =
            struct_fields.iter().map(|_| "DepylerValue::default()".to_string()).collect();
        let field_names: Vec<String> =
            vec!["value".to_string()].into_iter().chain(fields.iter().cloned()).collect();
        let field_inits: String = field_names
            .iter()
            .zip(field_defaults.iter())
            .map(|(n, d)| format!("{}: {}", n, d))
            .collect::<Vec<_>>()
            .join(", ");
        Some(format!(
            "macro_rules! {}_new {{ ($($args:expr),* $(,)?) => {{ {} {{ {} }} }}; }}",
            name, name, field_inits
        ))
    } else {
        None
    };

    (parts.join("\n"), ctor_macro)
}

/// Scan code for `Name::MEMBER` patterns, instance methods, and field accesses.
fn scan_class_usage(name: &str, code: &str) -> (Vec<String>, Vec<String>) {
    let prefix = format!("{}::", name);
    let ctor_macro = format!("{}_new!(", name);
    let mut members: Vec<String> = Vec::new();
    let mut instance_methods: Vec<String> = Vec::new();
    let mut instance_vars: Vec<String> = Vec::new();

    for line in code.lines() {
        scan_static_members(line, &prefix, &mut members, &mut instance_methods);
        scan_constructor_vars(line, name, &ctor_macro, &mut instance_vars);
    }

    scan_instance_var_methods(code, &instance_vars, &mut instance_methods);

    (members, instance_methods)
}

/// Extract an identifier (alphanumeric + underscore) from the start of `text`.
fn extract_identifier(text: &str) -> String {
    text.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect()
}

/// Scan a single line for `Name::MEMBER` patterns and chained `.method` accesses.
fn scan_static_members(
    line: &str,
    prefix: &str,
    members: &mut Vec<String>,
    instance_methods: &mut Vec<String>,
) {
    let mut search = line;
    while let Some(pos) = search.find(prefix) {
        let after = &search[pos + prefix.len()..];
        let member = extract_identifier(after);
        if !member.is_empty() && !members.contains(&member) {
            members.push(member.clone());
        }
        let after_member = &after[member.len()..];
        if let Some(rest) = after_member.strip_prefix('.') {
            let method = extract_identifier(rest);
            if !method.is_empty() && !instance_methods.contains(&method) {
                instance_methods.push(method);
            }
        }
        search = &search[pos + prefix.len()..];
    }
}

/// Scan a line for variables assigned from `Name::new()` or `Name_new!()` constructors.
fn scan_constructor_vars(
    line: &str,
    name: &str,
    ctor_macro: &str,
    instance_vars: &mut Vec<String>,
) {
    let trimmed = line.trim();
    let ctor_call = format!("{}::new(", name);
    let rest = trimmed.strip_prefix("let mut ").or_else(|| trimmed.strip_prefix("let "));
    if let Some(rest) = rest {
        if rest.contains(ctor_macro) || rest.contains(&ctor_call) {
            let var_name = extract_identifier(rest);
            if !var_name.is_empty() && !instance_vars.contains(&var_name) {
                instance_vars.push(var_name);
            }
        }
    }
}

/// Scan all lines for field/method access on instance variables (`var.field` or `var.method(`).
fn scan_instance_var_methods(
    code: &str,
    instance_vars: &[String],
    instance_methods: &mut Vec<String>,
) {
    for line in code.lines() {
        for var in instance_vars {
            let dot_prefix = format!("{}.", var);
            let mut search = line;
            while let Some(pos) = search.find(&dot_prefix) {
                let after = &search[pos + dot_prefix.len()..];
                let member = extract_identifier(after);
                if !member.is_empty() && !instance_methods.contains(&member) {
                    instance_methods.push(member);
                }
                search = &search[pos + dot_prefix.len()..];
            }
        }
    }
}

/// Build impl items for the stub struct (associated constants, methods, instance methods).
fn build_struct_impl_items(
    name: &str,
    code: &str,
    members: &[String],
    instance_methods: &[String],
) -> Vec<String> {
    let mut items = Vec::new();

    for member in members {
        if member == "new" {
            continue;
        }
        if let Some(item) = build_associated_member_item(name, code, member) {
            items.push(item);
        }
    }

    for method in instance_methods {
        if let Some(item) = build_instance_method_item(code, method) {
            items.push(item);
        }
    }

    items
}

/// Build a single associated constant or static method for a member.
///
/// Returns `None` for `new` (handled via constructor macro).
fn build_associated_member_item(name: &str, code: &str, member: &str) -> Option<String> {
    let call_pattern = format!("{}::{}(", name, member);
    let is_method = code.contains(&call_pattern);
    if !is_method {
        return Some(format!(
            "pub const {}: {} = {} {{ value: DepylerValue::None }};",
            member, name, name
        ));
    }
    let no_arg_pattern = format!("{}::{}()", name, member);
    let has_no_args = code.contains(&no_arg_pattern);
    if !has_no_args {
        return Some(format!(
            "pub fn {}(_args: impl std::any::Any) -> DepylerValue {{ DepylerValue::default() }}",
            member
        ));
    }
    let cast_pattern = format!("{}::{}() as ", name, member);
    let is_cast_to_numeric = code.contains(&cast_pattern);
    let ret_type = if is_cast_to_numeric { "usize" } else { "DepylerValue" };
    let ret_val = if is_cast_to_numeric { "0" } else { "DepylerValue::default()" };
    Some(format!("pub fn {}() -> {} {{ {} }}", member, ret_type, ret_val))
}

/// Build a single instance method item for a method name.
///
/// Returns `None` if the method is `value` (already a field) or is a field access.
fn build_instance_method_item(code: &str, method: &str) -> Option<String> {
    if method == "value" {
        return None;
    }
    let method_call = format!(".{}(", method);
    let is_method_call = code.contains(&method_call);
    let field_assign = format!(".{} =", method);
    let is_field_assign = code.contains(&field_assign);

    if is_field_assign || !is_method_call {
        return None;
    }
    let zero_arg_call = format!(".{}()", method);
    let has_zero_args = code.contains(&zero_arg_call);
    if has_zero_args {
        Some(format!("pub fn {}(&self) -> DepylerValue {{ DepylerValue::default() }}", method))
    } else {
        Some(format!(
            "pub fn {}(&self, _args: impl std::any::Any) -> DepylerValue {{ DepylerValue::default() }}",
            method
        ))
    }
}

/// Find the index of the line containing the matching closing brace.
///
/// Starts scanning from `start` with initial `depth`, tracking `{` and `}` characters.
/// Returns the index of the line where depth reaches zero.
fn find_closing_brace(lines: &[&str], start: usize, initial_depth: usize) -> Option<usize> {
    let mut depth = initial_depth;
    for (j, line) in lines.iter().enumerate().skip(start) {
        for ch in line.trim().chars() {
            match ch {
                '{' => depth += 1,
                '}' => depth -= 1,
                _ => {}
            }
        }
        if depth == 0 {
            return Some(j);
        }
    }
    None
}

/// Try to wrap a bare integer value inside a `.insert(...)` line with `DepylerValue::Int`.
///
/// Returns `Some(new_line)` if the line was an insert with a bare integer, `None` otherwise.
fn try_wrap_insert_integer(original_line: &str) -> Option<String> {
    let inner = original_line.trim();
    if !inner.contains(".insert(") || !inner.ends_with(");") {
        return None;
    }
    let cp = inner.rfind(", ")?;
    let val = &inner[cp + 2..inner.len() - 2];
    if val.parse::<i64>().is_ok() && !val.contains("DepylerValue") {
        Some(
            original_line
                .replace(&format!(", {});", val), &format!(", DepylerValue::Int({}));", val)),
        )
    } else {
        None
    }
}

/// Process a single inner line of a nested HashMap block.
///
/// Wraps bare integer inserts with `DepylerValue::Int` and wraps the final
/// bare `map` return with `DepylerValue::Dict(...)`.
fn process_inner_hashmap_line(original_line: &str, line_idx: usize, close_idx: usize) -> String {
    if let Some(wrapped) = try_wrap_insert_integer(original_line) {
        return wrapped;
    }

    let inner = original_line.trim();
    if inner == "map" && line_idx + 1 == close_idx {
        let indent = original_line.len() - original_line.trim_start().len();
        return format!(
            "{}DepylerValue::Dict(map.into_iter().map(|(k, v)| (DepylerValue::Str(k), v)).collect())",
            " ".repeat(indent)
        );
    }

    original_line.to_string()
}

/// Detect a nested HashMap insert pattern starting at line `i`.
///
/// Looks for `.insert("key", {` followed by `HashMap::new()` and returns the
/// index of the closing brace if found.
fn detect_nested_hashmap_insert(lines: &[&str], i: usize) -> Option<usize> {
    let trimmed = lines[i].trim();
    if !trimmed.contains(".insert(") || !trimmed.ends_with('{') || i + 1 >= lines.len() {
        return None;
    }
    if !lines[i + 1].trim().contains("HashMap::new()") {
        return None;
    }
    find_closing_brace(lines, i + 2, 1)
}

/// Wrap nested HashMap block expressions as DepylerValue::Dict.
///
/// When an outer HashMap<String, DepylerValue> inserts a value that's a block
/// expression building another HashMap, wrap the block result in DepylerValue::Dict.
fn fix_nested_hashmap_to_depyler_value(code: &str) -> String {
    if !code.contains("HashMap<String, DepylerValue>") {
        return code.to_string();
    }

    let lines: Vec<&str> = code.lines().collect();
    let mut result: Vec<String> = Vec::with_capacity(lines.len());
    let mut i = 0;

    while i < lines.len() {
        if let Some(close_idx) = detect_nested_hashmap_insert(&lines, i) {
            result.push(lines[i].to_string());
            for (j, line) in lines.iter().enumerate().take(close_idx).skip(i + 1) {
                result.push(process_inner_hashmap_line(line, j, close_idx));
            }
            result.push(lines[close_idx].to_string());
            i = close_idx + 1;
            continue;
        }

        result.push(lines[i].to_string());
        i += 1;
    }

    result.join("\n")
}

/// Extract the receiver identifier immediately before a dot position in a string.
///
/// Scans backwards from `dot_pos` collecting alphanumeric/underscore characters.
fn extract_receiver(text: &str, dot_pos: usize) -> String {
    text[..dot_pos]
        .chars()
        .rev()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect::<String>()
        .chars()
        .rev()
        .collect()
}

/// Extract the method name and argument from a `receiver.method(arg)` pattern.
///
/// Given text after the dot, returns `(method, arg, rest_after_arg)`
/// if the pattern is a valid identifier method call with an identifier argument.
fn extract_method_and_arg(after_dot: &str) -> Option<(&str, String, &str)> {
    let paren = after_dot.find('(')?;
    let method = &after_dot[..paren];
    if method.is_empty() || !method.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }
    let args_rest = &after_dot[paren + 1..];
    let arg: String = args_rest.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
    if arg.is_empty() {
        return None;
    }
    let after_arg = &args_rest[arg.len()..];
    Some((method, arg, after_arg))
}

/// Apply `.clone()` to a method call argument if ownership cloning is needed.
///
/// Handles two cases:
/// - Same variable as receiver and argument: `a.method(a)` -> `a.method(a.clone())`
/// - Argument reused later on the line: `a.f(b) == b.f(a)` -> `a.f(b.clone()) == b.f(a)`
fn apply_clone_to_method_call(line: &str, trimmed: &str) -> String {
    let dot_pos = match trimmed.find('.') {
        Some(p) => p,
        None => return line.to_string(),
    };

    let receiver = extract_receiver(trimmed, dot_pos);
    if receiver.is_empty() {
        return line.to_string();
    }

    let after_dot = &trimmed[dot_pos + 1..];
    let (method, arg, after_arg) = match extract_method_and_arg(after_dot) {
        Some(parts) => parts,
        None => return line.to_string(),
    };

    // Case 1: same variable as receiver and arg
    if arg == receiver && after_arg.starts_with(')') {
        let old = format!("{}.{}({})", receiver, method, arg);
        let new = format!("{}.{}({}.clone())", receiver, method, arg);
        return line.replace(&old, &new);
    }
    // Case 2: arg is used again on the same line (e.g., a.f(b) == b.f(a))
    if after_arg.contains(&format!("{}.{}", arg, method)) {
        let old = format!(".{}({})", method, arg);
        let new = format!(".{}({}.clone())", method, arg);
        return line.replacen(&old, &new, 1);
    }

    line.to_string()
}

/// Add `.clone()` to arguments of stub struct instance methods to fix ownership.
///
/// Only applies in assert contexts to avoid false positives in match arms, hash impls, etc.
/// Detects `a.method(b)` where `b` is reused, or `a.method(a)` same-variable patterns.
fn fix_stub_method_ownership(code: &str) -> String {
    if !code.contains("DEPYLER-0615") {
        return code.to_string();
    }

    let lines: Vec<&str> = code.lines().collect();
    let mut result: Vec<String> = Vec::with_capacity(lines.len());

    for line in &lines {
        let trimmed = line.trim();
        if trimmed.starts_with("assert") {
            result.push(apply_clone_to_method_call(line, trimmed));
        } else {
            result.push(line.to_string());
        }
    }

    result.join("\n")
}

/// Find the matching closing brace at the same indentation level.
///
/// Like `find_closing_brace` but also requires the closing line to have the
/// same indentation as `expected_indent`.
fn find_closing_brace_at_indent(
    lines: &[&str],
    start: usize,
    expected_indent: usize,
) -> Option<usize> {
    let mut depth: usize = 1;
    for (j, line) in lines.iter().enumerate().skip(start) {
        for ch in line.trim().chars() {
            match ch {
                '{' => depth += 1,
                '}' => depth -= 1,
                _ => {}
            }
        }
        if depth == 0 {
            let j_indent = line.len() - line.trim_start().len();
            return if j_indent == expected_indent { Some(j) } else { None };
        }
    }
    None
}

/// Extract the variable name from a `let [mut] VAR = EXPR;` line.
fn extract_let_var_name(let_trimmed: &str) -> String {
    let var_rest = let_trimmed
        .strip_prefix("let mut ")
        .or_else(|| let_trimmed.strip_prefix("let "))
        .unwrap_or("");
    var_rest.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect()
}

/// Check if `var_name` appears in any line after index `close` in the given lines.
fn is_var_used_after(lines: &[&str], close: usize, var_name: &str) -> bool {
    (close + 1..lines.len()).any(|k| lines[k].trim().contains(var_name))
}

/// Try to hoist a let-binding out of a with-block at line `i`.
///
/// If the pattern matches and hoisting is needed, pushes the hoisted let and
/// opening brace to `result`, and returns `Some(next_i)` to skip past them.
/// Returns `None` if no hoisting applies.
fn try_hoist_with_block(lines: &[&str], i: usize, result: &mut Vec<String>) -> Option<usize> {
    let trimmed = lines[i].trim();
    if trimmed != "{" || i + 1 >= lines.len() {
        return None;
    }

    let next_trimmed = lines[i + 1].trim();
    if !next_trimmed.starts_with("let mut ") && !next_trimmed.starts_with("let ") {
        return None;
    }

    let indent = lines[i].len() - lines[i].trim_start().len();
    let close = find_closing_brace_at_indent(lines, i + 2, indent)?;

    let var_name = extract_let_var_name(next_trimmed);
    if var_name.is_empty() || !is_var_used_after(lines, close, &var_name) {
        return None;
    }

    // Hoist: emit `let mut VAR = EXPR;` before `{`
    let let_indent = " ".repeat(indent + 4);
    result.push(format!("{}{}", let_indent, next_trimmed));
    result.push(lines[i].to_string()); // the `{`
    Some(i + 2)
}

/// Hoist variable declarations out of with-statement blocks.
///
/// Python `with` statements transpile to `{ let mut var = EXPR; { body } }`
/// but assertions after the block reference the variable, which is out of scope.
/// This hoists `let mut var = EXPR;` before the enclosing `{` block.
fn fix_with_block_scope(code: &str) -> String {
    let lines: Vec<&str> = code.lines().collect();
    let mut result: Vec<String> = Vec::with_capacity(lines.len());
    let mut i = 0;

    while i < lines.len() {
        if let Some(skip_to) = try_hoist_with_block(&lines, i, &mut result) {
            i = skip_to;
            continue;
        }
        result.push(lines[i].to_string());
        i += 1;
    }

    result.join("\n")
}

/// Fix struct field assignments where a raw literal is assigned to a DepylerValue field.
///
/// Wraps `p.field = 5.0;` → `p.field = DepylerValue::Float(5.0);` for stub struct fields.
fn fix_struct_field_literal_assignment(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    for line in code.lines() {
        if let Some(new_line) = try_wrap_field_literal(line) {
            result.push_str(&new_line);
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }
    result
}

/// Try to wrap a raw literal in a field assignment with the appropriate DepylerValue variant.
///
/// Returns `Some(new_line)` if the line matches `ident.field = LITERAL;` and was wrapped,
/// `None` if the line does not match or no wrapping is needed.
fn try_wrap_field_literal(line: &str) -> Option<String> {
    let trimmed = line.trim();
    let eq_pos = trimmed.find(" = ")?;
    let lhs = &trimmed[..eq_pos];
    let rhs = trimmed[eq_pos + 3..].trim_end_matches(';').trim();

    if !lhs.contains('.') || lhs.contains("::") {
        return None;
    }

    // Float literal (e.g., 5.0, -3.14)
    if rhs.parse::<f64>().is_ok() && rhs.contains('.') {
        return Some(
            line.replace(&format!("= {};", rhs), &format!("= DepylerValue::Float({});", rhs)),
        );
    }
    // Integer literal
    if rhs.parse::<i64>().is_ok() {
        return Some(
            line.replace(&format!("= {};", rhs), &format!("= DepylerValue::Int({});", rhs)),
        );
    }

    None
}

/// Fix `(expr as f64)` and `(expr as i32)` patterns involving DepylerValue.
///
/// Rewrites `(expr as f64) == literal` → `expr == literal` since DepylerValue
/// already implements PartialEq for numeric types.
fn fix_depyler_value_casts(code: &str) -> String {
    // Pattern: `(EXPR as f64)` or `(EXPR as i32)` in assert contexts
    // We can't know which exprs are DepylerValue, so only fix patterns where
    // the cast appears in a comparison with a literal.
    let mut result = code.to_string();
    for cast_type in &["f64", "i32", "i64", "usize"] {
        let cast_suffix = format!(" as {})", cast_type);
        // Only fix casts that are followed by == or != with a literal
        let lines: Vec<&str> = result.lines().collect();
        let mut new_lines: Vec<String> = Vec::with_capacity(lines.len());
        for line in &lines {
            if line.contains(&cast_suffix) && line.contains("assert!") {
                // Remove the `as TYPE` part: `(expr as f64)` → `(expr)`
                new_lines.push(line.replace(&cast_suffix, ")"));
            } else {
                new_lines.push(line.to_string());
            }
        }
        result = new_lines.join("\n");
    }
    result
}
