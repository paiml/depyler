//! Automatic test generation for transpiled functions
//!
//! This module generates property-based tests using quickcheck
//! for pure functions with appropriate properties.

use crate::hir::{BinOp, HirExpr, HirFunction, HirStmt, Type};
use anyhow::Result;
use quote::quote;
use syn;

/// Configuration for test generation
#[derive(Debug, Clone)]
pub struct TestGenConfig {
    /// Generate property-based tests
    pub generate_property_tests: bool,
    /// Generate example-based tests
    pub generate_example_tests: bool,
    /// Maximum number of test cases for quickcheck
    pub max_test_cases: usize,
    /// Generate shrinking tests
    pub enable_shrinking: bool,
}

impl Default for TestGenConfig {
    fn default() -> Self {
        Self {
            generate_property_tests: true,
            generate_example_tests: true,
            max_test_cases: 100,
            enable_shrinking: true,
        }
    }
}

/// Test generator for HIR functions
pub struct TestGenerator {
    config: TestGenConfig,
}

impl TestGenerator {
    pub fn new(config: TestGenConfig) -> Self {
        Self { config }
    }

    /// Generate test items for a single function (without mod tests wrapper)
    ///
    /// DEPYLER-0280 FIX: This generates test functions only, not the module wrapper.
    /// The module wrapper should be added once at the file level.
    pub fn generate_test_items_for_function(
        &self,
        func: &HirFunction,
    ) -> Result<Vec<proc_macro2::TokenStream>> {
        // Only generate tests for pure functions
        if !func.properties.is_pure {
            return Ok(Vec::new());
        }

        let mut test_functions = Vec::new();

        // Generate property-based tests
        if self.config.generate_property_tests {
            if let Some(prop_test) = self.generate_property_test(func)? {
                test_functions.push(prop_test);
            }
        }

        // Generate example-based tests
        if self.config.generate_example_tests {
            if let Some(example_test) = self.generate_example_test(func)? {
                test_functions.push(example_test);
            }
        }

        Ok(test_functions)
    }

    /// Generate a complete test module for multiple functions
    ///
    /// DEPYLER-0280 FIX: Wraps all test items in a single `mod tests {}` block.
    /// This prevents "the name `tests` is defined multiple times" errors.
    pub fn generate_tests_module(
        &self,
        functions: &[HirFunction],
    ) -> Result<Option<proc_macro2::TokenStream>> {
        let mut all_test_items = Vec::new();

        // Collect test items from all functions
        for func in functions {
            let test_items = self.generate_test_items_for_function(func)?;
            all_test_items.extend(test_items);
        }

        // If no tests were generated, return None
        if all_test_items.is_empty() {
            return Ok(None);
        }

        // Wrap all tests in a single mod tests block
        Ok(Some(quote! {
            #[cfg(test)]
            mod tests {
                use super::*;
                use quickcheck::{quickcheck, TestResult};

                #(#all_test_items)*
            }
        }))
    }

    /// Generate tests for a function if applicable (DEPRECATED - use generate_tests_module instead)
    ///
    /// DEPYLER-0280: This function is deprecated because it creates duplicate `mod tests {}` blocks.
    /// Use `generate_tests_module()` for module-level test generation instead.
    #[deprecated(
        since = "3.19.22",
        note = "Use generate_tests_module() to avoid duplicate mod tests blocks (DEPYLER-0280)"
    )]
    pub fn generate_tests(&self, func: &HirFunction) -> Result<Option<proc_macro2::TokenStream>> {
        let test_items = self.generate_test_items_for_function(func)?;

        if test_items.is_empty() {
            return Ok(None);
        }

        Ok(Some(quote! {
            #[cfg(test)]
            mod tests {
                use super::*;
                use quickcheck::{quickcheck, TestResult};

                #(#test_items)*
            }
        }))
    }

    /// Generate property-based test for a function
    fn generate_property_test(
        &self,
        func: &HirFunction,
    ) -> Result<Option<proc_macro2::TokenStream>> {
        let func_name = syn::Ident::new(&func.name, proc_macro2::Span::call_site());
        let test_name = syn::Ident::new(
            &format!("quickcheck_{}", func.name),
            proc_macro2::Span::call_site(),
        );

        // Determine properties to test based on function analysis
        let properties = self.analyze_function_properties(func);

        if properties.is_empty() {
            return Ok(None);
        }

        // Generate parameter types and names for quickcheck
        let param_types: Vec<_> = func
            .params
            .iter()
            .map(|param| self.type_to_quickcheck_type(&param.ty))
            .collect();

        let param_names: Vec<_> = func
            .params
            .iter()
            .map(|param| syn::Ident::new(&param.name, proc_macro2::Span::call_site()))
            .collect();

        // DEPYLER-0281: Pass function parameters for type-aware conversions
        let property_checks: Vec<_> = properties
            .iter()
            .map(|prop| self.property_to_assertion(prop, &func_name, &param_names, &func.params))
            .collect();

        Ok(Some(quote! {
            #[test]
            fn #test_name() {
                fn prop(#(#param_names: #param_types),*) -> TestResult {
                    #(#property_checks)*
                    TestResult::passed()
                }

                quickcheck(prop as fn(#(#param_types),*) -> TestResult);
            }
        }))
    }

    /// Generate example-based test
    fn generate_example_test(
        &self,
        func: &HirFunction,
    ) -> Result<Option<proc_macro2::TokenStream>> {
        let test_name = syn::Ident::new(
            &format!("test_{}_examples", func.name),
            proc_macro2::Span::call_site(),
        );

        // Generate test cases based on function type
        let test_cases = self.generate_test_cases(func);

        if test_cases.is_empty() {
            return Ok(None);
        }

        Ok(Some(quote! {
            #[test]
            fn #test_name() {
                #(#test_cases)*
            }
        }))
    }

    /// Analyze function to determine testable properties
    fn analyze_function_properties(&self, func: &HirFunction) -> Vec<TestProperty> {
        // DEPYLER-0282 FIXED: String parameters now correctly use Cow<'_, str> instead of
        // Cow<'static, str>, so property tests work properly with local String values.
        // The DEPYLER-0281 workaround has been removed.

        let mut properties = Vec::new();

        // Check for common patterns
        if self.is_identity_function(func) {
            properties.push(TestProperty::Identity);
        }

        if self.is_commutative(func) {
            properties.push(TestProperty::Commutative);
        }

        if self.is_associative(func) {
            properties.push(TestProperty::Associative);
        }

        if self.returns_non_negative(func) {
            properties.push(TestProperty::NonNegative);
        }

        if self.preserves_length(func) {
            properties.push(TestProperty::LengthPreserving);
        }

        if self.is_idempotent(func) {
            properties.push(TestProperty::Idempotent);
        }

        if self.is_sorting_function(func) {
            properties.push(TestProperty::Sorted);
            properties.push(TestProperty::SameElements);
        }

        properties
    }

    /// Check if function is an identity function
    fn is_identity_function(&self, func: &HirFunction) -> bool {
        // Simple case: function with one parameter that returns it unchanged
        if func.params.len() == 1 && func.body.len() == 1 {
            if let HirStmt::Return(Some(HirExpr::Var(name))) = &func.body[0] {
                return name == &func.params[0].name;
            }
        }
        false
    }

    /// Check if function is commutative (like addition)
    fn is_commutative(&self, func: &HirFunction) -> bool {
        if func.params.len() == 2 && func.body.len() == 1 {
            if let HirStmt::Return(Some(HirExpr::Binary { op, left, right })) = &func.body[0] {
                // DEPYLER-0286 FIX: String concatenation (BinOp::Add on strings) is NOT commutative!
                // "ab" + "cd" ≠ "cd" + "ab"
                // Only numeric addition is commutative, not string concatenation.

                // Check if this is string concatenation (Add with String parameters)
                let is_string_concat = matches!(op, BinOp::Add)
                    && (matches!(func.params[0].ty, Type::String)
                        || matches!(func.params[1].ty, Type::String));

                // If it's string concatenation, it's NOT commutative
                if is_string_concat {
                    return false;
                }

                // Check if it's a commutative operation
                matches!(
                    op,
                    BinOp::Add | BinOp::Mul | BinOp::BitAnd | BinOp::BitOr | BinOp::BitXor
                ) && self.is_simple_param_reference(left, &func.params[0].name)
                    && self.is_simple_param_reference(right, &func.params[1].name)
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Check if expression is a simple parameter reference
    fn is_simple_param_reference(&self, expr: &HirExpr, param_name: &str) -> bool {
        matches!(expr, HirExpr::Var(name) if name == param_name)
    }

    /// Check if function is associative
    fn is_associative(&self, _func: &HirFunction) -> bool {
        // This is more complex to detect automatically
        // For now, return false
        false
    }

    /// Check if function always returns non-negative values
    fn returns_non_negative(&self, func: &HirFunction) -> bool {
        // Check for abs-like patterns
        func.name.contains("abs") || func.name.contains("magnitude")
    }

    /// Check if function preserves collection length
    fn preserves_length(&self, func: &HirFunction) -> bool {
        // Check if input and output are both lists/arrays
        if func.params.len() == 1 {
            if let (Type::List(_), Type::List(_)) = (&func.params[0].ty, &func.ret_type) {
                // Simple heuristic: sorting and mapping functions preserve length
                return func.name.contains("sort") || func.name.contains("map");
            }
        }
        false
    }

    /// Check if function is idempotent
    fn is_idempotent(&self, func: &HirFunction) -> bool {
        func.name.contains("normalize") || func.name.contains("clean")
    }

    /// Check if function is a sorting function
    fn is_sorting_function(&self, func: &HirFunction) -> bool {
        // DEPYLER-0189: Must have at least one parameter to be a sorting function
        !func.params.is_empty() && func.name.contains("sort")
    }

    /// Convert Python type to quickcheck-compatible type
    #[allow(clippy::only_used_in_recursion)]
    fn type_to_quickcheck_type(&self, ty: &Type) -> proc_macro2::TokenStream {
        match ty {
            Type::Int => quote! { i32 },
            Type::Float => quote! { f64 },
            Type::String => quote! { String },
            Type::Bool => quote! { bool },
            Type::List(inner) => {
                let inner_type = self.type_to_quickcheck_type(inner);
                quote! { Vec<#inner_type> }
            }
            _ => quote! { () }, // Unsupported type
        }
    }

    /// Convert a property test argument to match function signature
    ///
    /// DEPYLER-0281 FIX: Property tests use simple QuickCheck types (String, Vec<T>),
    /// but functions may have complex signatures (Cow<'static, str>, &Vec<T>).
    /// This function generates conversion code to bridge the gap.
    fn convert_arg_for_property_test(
        &self,
        ty: &Type,
        arg_name: &syn::Ident,
    ) -> proc_macro2::TokenStream {
        match ty {
            Type::String => {
                // DEPYLER-0281 FIX: String parameters may become Cow<'static, str> or &str
                // Use (&*arg).into() which converts via type inference:
                //   - For `fn f(s: Cow<'static, str>)`: &str → Cow::Owned via From<String> after clone
                //   - For `fn f(s: &str)`: &str → &str (into() is no-op when T: Copy)
                // The &* dereferences String → str, then borrows as &str for inference.
                quote! { (&*#arg_name).into() }
            }
            Type::List(_) => {
                // List parameters become &Vec<T>
                quote! { &#arg_name }
            }
            Type::Dict(_, _) => {
                // Dict parameters become &HashMap<K, V>
                quote! { &#arg_name }
            }
            _ => {
                // Simple types (int, float, bool) - use directly with clone
                quote! { #arg_name.clone() }
            }
        }
    }

    /// Convert property to assertion code
    ///
    /// DEPYLER-0281: Now accepts func_params to enable type-aware argument conversions.
    /// This ensures property tests work with complex types like Cow<'static, str>.
    fn property_to_assertion(
        &self,
        prop: &TestProperty,
        func_name: &syn::Ident,
        params: &[syn::Ident],
        func_params: &[crate::hir::HirParam],
    ) -> proc_macro2::TokenStream {
        match prop {
            TestProperty::Identity => {
                // DEPYLER-0189: Bounds check before accessing params
                if params.is_empty() || func_params.is_empty() {
                    return quote! {};
                }
                let param = &params[0];

                // DEPYLER-0281: Convert argument to match function signature
                let param_converted = self.convert_arg_for_property_test(&func_params[0].ty, param);

                quote! {
                    let result = #func_name(#param_converted);
                    if result != #param {
                        return TestResult::failed();
                    }
                }
            }
            TestProperty::Commutative => {
                // DEPYLER-0189: Bounds check before accessing params
                if params.len() < 2 || func_params.len() < 2 {
                    return quote! {};
                }
                let (a, b) = (&params[0], &params[1]);

                // DEPYLER-0281 FIX: Convert arguments to match function signature
                // QuickCheck generates simple types (String), but functions may expect
                // complex types (Cow<'static, str>). Convert accordingly.
                let a_converted = self.convert_arg_for_property_test(&func_params[0].ty, a);
                let b_converted = self.convert_arg_for_property_test(&func_params[1].ty, b);

                // DEPYLER-0284 FIX: Check for potential overflow with integer addition
                // DEPYLER-0285 FIX: Check for NaN in float operations
                let special_value_check = if matches!(func_params[0].ty, Type::Int)
                    && matches!(func_params[1].ty, Type::Int)
                {
                    quote! {
                        // Skip test if values would overflow
                        if (#a > 0 && #b > i32::MAX - #a) || (#a < 0 && #b < i32::MIN - #a) {
                            return TestResult::discard();
                        }
                    }
                } else if matches!(func_params[0].ty, Type::Float)
                    && matches!(func_params[1].ty, Type::Float)
                {
                    quote! {
                        // Skip test if either value is NaN or infinite
                        if #a.is_nan() || #b.is_nan() || #a.is_infinite() || #b.is_infinite() {
                            return TestResult::discard();
                        }
                    }
                } else {
                    quote! {}
                };

                quote! {
                    #special_value_check
                    let result1 = #func_name(#a_converted, #b_converted);
                    let result2 = #func_name(#b_converted, #a_converted);
                    if result1 != result2 {
                        return TestResult::failed();
                    }
                }
            }
            TestProperty::NonNegative => {
                // DEPYLER-0281: Convert all arguments to match function signature
                let converted_args: Vec<_> = params
                    .iter()
                    .zip(func_params.iter())
                    .map(|(param, func_param)| {
                        self.convert_arg_for_property_test(&func_param.ty, param)
                    })
                    .collect();

                quote! {
                    let result = #func_name(#(#converted_args),*);
                    if result < 0 {
                        return TestResult::failed();
                    }
                }
            }
            TestProperty::LengthPreserving => {
                // DEPYLER-0189: Bounds check before accessing params
                if params.is_empty() || func_params.is_empty() {
                    return quote! {};
                }
                let param = &params[0];

                // DEPYLER-0281: Convert argument to match function signature
                let param_converted = self.convert_arg_for_property_test(&func_params[0].ty, param);

                quote! {
                    let input_len = #param.len();
                    let result = #func_name(#param_converted);
                    if result.len() != input_len {
                        return TestResult::failed();
                    }
                }
            }
            TestProperty::Sorted => {
                // DEPYLER-0281: Convert all arguments to match function signature
                let converted_args: Vec<_> = params
                    .iter()
                    .zip(func_params.iter())
                    .map(|(param, func_param)| {
                        self.convert_arg_for_property_test(&func_param.ty, param)
                    })
                    .collect();

                quote! {
                    let result = #func_name(#(#converted_args),*);
                    for i in 1..result.len() {
                        if result[i-1] > result[i] {
                            return TestResult::failed();
                        }
                    }
                }
            }
            TestProperty::SameElements => {
                // DEPYLER-0189: Bounds check before accessing params
                if params.is_empty() || func_params.is_empty() {
                    return quote! {};
                }
                let param = &params[0];

                // DEPYLER-0281: Convert argument to match function signature
                let param_converted = self.convert_arg_for_property_test(&func_params[0].ty, param);

                quote! {
                    let mut input_sorted = #param.clone();
                    input_sorted.sort();
                    let mut result = #func_name(#param_converted);
                    result.sort();
                    if input_sorted != result {
                        return TestResult::failed();
                    }
                }
            }
            TestProperty::Idempotent => {
                // DEPYLER-0281: Convert all arguments to match function signature
                let converted_args: Vec<_> = params
                    .iter()
                    .zip(func_params.iter())
                    .map(|(param, func_param)| {
                        self.convert_arg_for_property_test(&func_param.ty, param)
                    })
                    .collect();

                quote! {
                    let once = #func_name(#(#converted_args),*);
                    let twice = #func_name(once.clone());
                    if once != twice {
                        return TestResult::failed();
                    }
                }
            }
            _ => quote! {},
        }
    }

    /// Generate example test cases
    fn generate_test_cases(&self, func: &HirFunction) -> Vec<proc_macro2::TokenStream> {
        let func_name = syn::Ident::new(&func.name, proc_macro2::Span::call_site());
        let mut cases = Vec::new();

        // Generate basic test cases based on function type and parameters
        match (&func.ret_type, func.params.len()) {
            (Type::Int, 0) => {
                // No parameters - just call the function
                cases.push(quote! {
                    let _ = #func_name();
                });
            }
            (Type::Int, 1) => {
                // DEPYLER-0269: Check actual parameter type before generating test values
                let param_type = &func.params[0].ty;
                match param_type {
                    Type::Int => {
                        // Single integer parameter
                        if func.name.contains("abs") {
                            // Special case for absolute value functions
                            cases.push(quote! {
                                assert_eq!(#func_name(0), 0);
                                assert_eq!(#func_name(1), 1);
                                assert_eq!(#func_name(-1), 1);
                                assert_eq!(#func_name(i32::MIN + 1), i32::MAX);
                            });
                        } else {
                            // General case
                            cases.push(quote! {
                                assert_eq!(#func_name(0), 0);
                                assert_eq!(#func_name(1), 1);
                                assert_eq!(#func_name(-1), -1);
                            });
                        }
                    }
                    Type::List(_) => {
                        // DEPYLER-0283 FIX: List parameter returning int
                        // Detect if it's a sum function vs length function by name
                        if func.name.contains("sum") {
                            // Sum function - test sum of elements
                            cases.push(quote! {
                                assert_eq!(#func_name(&vec![]), 0);
                                assert_eq!(#func_name(&vec![1]), 1);
                                assert_eq!(#func_name(&vec![1, 2, 3]), 6);  // 1+2+3=6
                            });
                        } else if func.name.contains("len")
                            || func.name.contains("count")
                            || func.name.contains("size")
                        {
                            // Length/count function - test length
                            cases.push(quote! {
                                assert_eq!(#func_name(&vec![]), 0);
                                assert_eq!(#func_name(&vec![1]), 1);
                                assert_eq!(#func_name(&vec![1, 2, 3]), 3);
                            });
                        } else {
                            // Unknown function - use conservative length-based test
                            cases.push(quote! {
                                assert_eq!(#func_name(&vec![]), 0);
                                assert_eq!(#func_name(&vec![1]), 1);
                                assert_eq!(#func_name(&vec![1, 2, 3]), 3);
                            });
                        }
                    }
                    Type::String => {
                        // String parameter - generate string test cases
                        cases.push(quote! {
                            assert_eq!(#func_name(""), 0);
                            assert_eq!(#func_name("a"), 1);
                            assert_eq!(#func_name("abc"), 3);
                        });
                    }
                    _ => {
                        // Unsupported parameter type - skip test generation
                    }
                }
            }
            (Type::Int, 2)
                if matches!(&func.params[0].ty, Type::Int)
                    && matches!(&func.params[1].ty, Type::Int) =>
            {
                // Two integer parameters - test basic cases
                cases.push(quote! {
                    assert_eq!(#func_name(0, 0), 0);
                    assert_eq!(#func_name(1, 2), 3);
                    assert_eq!(#func_name(-1, 1), 0);
                });
            }
            (Type::Bool, _) => {
                // Test boolean functions
                if func.params.len() == 1 {
                    cases.push(quote! {
                        // Test with edge cases
                        let _ = #func_name(Default::default());
                    });
                }
            }
            (Type::List(_), _) => {
                // Test with empty and single-element lists
                if func.params.len() == 1 && matches!(&func.params[0].ty, Type::List(_)) {
                    cases.push(quote! {
                        assert_eq!(#func_name(vec![]), vec![]);
                        assert_eq!(#func_name(vec![1]), vec![1]);
                    });
                }
            }
            _ => {}
        }

        cases
    }
}

/// Properties that can be tested
#[derive(Debug, Clone, PartialEq)]
enum TestProperty {
    Identity,
    Commutative,
    Associative,
    NonNegative,
    LengthPreserving,
    Sorted,
    SameElements,
    Idempotent,
    #[allow(dead_code)]
    Distributive,
    #[allow(dead_code)]
    Monotonic,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::FunctionProperties;
    use depyler_annotations::TranspilationAnnotations;
    use smallvec::smallvec;

    fn make_pure_properties() -> FunctionProperties {
        let mut props = FunctionProperties::default();
        props.is_pure = true;
        props
    }

    fn make_impure_properties() -> FunctionProperties {
        let mut props = FunctionProperties::default();
        props.is_pure = false;
        props
    }

    fn make_param(name: &str, ty: Type) -> crate::hir::HirParam {
        crate::hir::HirParam::new(name.to_string(), ty)
    }

    // TestGenConfig tests
    #[test]
    fn test_testgen_config_default() {
        let config = TestGenConfig::default();
        assert!(config.generate_property_tests);
        assert!(config.generate_example_tests);
        assert_eq!(config.max_test_cases, 100);
        assert!(config.enable_shrinking);
    }

    #[test]
    fn test_testgen_config_custom() {
        let config = TestGenConfig {
            generate_property_tests: false,
            generate_example_tests: true,
            max_test_cases: 50,
            enable_shrinking: false,
        };
        assert!(!config.generate_property_tests);
        assert!(config.generate_example_tests);
        assert_eq!(config.max_test_cases, 50);
        assert!(!config.enable_shrinking);
    }

    #[test]
    fn test_testgen_config_clone() {
        let config = TestGenConfig::default();
        let cloned = config.clone();
        assert_eq!(config.max_test_cases, cloned.max_test_cases);
    }

    // TestGenerator tests
    #[test]
    fn test_test_generator_new() {
        let config = TestGenConfig::default();
        let _gen = TestGenerator::new(config);
    }

    #[test]
    fn test_generate_test_items_for_impure_function() {
        let gen = TestGenerator::new(TestGenConfig::default());
        let func = HirFunction {
            name: "side_effect".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![],
            properties: make_impure_properties(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };
        let result = gen.generate_test_items_for_function(&func).unwrap();
        assert!(result.is_empty(), "Impure functions should not generate tests");
    }

    #[test]
    fn test_generate_tests_module_empty() {
        let gen = TestGenerator::new(TestGenConfig::default());
        let result = gen.generate_tests_module(&[]).unwrap();
        assert!(result.is_none(), "Empty function list should return None");
    }

    #[test]
    fn test_generate_tests_module_impure_only() {
        let gen = TestGenerator::new(TestGenConfig::default());
        let func = HirFunction {
            name: "impure".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![],
            properties: make_impure_properties(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };
        let result = gen.generate_tests_module(&[func]).unwrap();
        assert!(result.is_none(), "Only impure functions should return None");
    }

    // is_identity_function tests
    #[test]
    fn test_is_identity_function_true() {
        let gen = TestGenerator::new(TestGenConfig::default());
        let func = HirFunction {
            name: "identity".to_string(),
            params: smallvec![make_param("x", Type::Int)],
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Var("x".to_string())))],
            properties: make_pure_properties(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };
        assert!(gen.is_identity_function(&func));
    }

    #[test]
    fn test_is_identity_function_false_different_return() {
        let gen = TestGenerator::new(TestGenConfig::default());
        let func = HirFunction {
            name: "not_identity".to_string(),
            params: smallvec![make_param("x", Type::Int)],
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Var("y".to_string())))],
            properties: make_pure_properties(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };
        assert!(!gen.is_identity_function(&func));
    }

    #[test]
    fn test_is_identity_function_false_multiple_params() {
        let gen = TestGenerator::new(TestGenConfig::default());
        let func = HirFunction {
            name: "two_params".to_string(),
            params: smallvec![make_param("x", Type::Int), make_param("y", Type::Int)],
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Var("x".to_string())))],
            properties: make_pure_properties(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };
        assert!(!gen.is_identity_function(&func));
    }

    #[test]
    fn test_is_identity_function_false_multiple_stmts() {
        let gen = TestGenerator::new(TestGenConfig::default());
        let func = HirFunction {
            name: "multiple_stmts".to_string(),
            params: smallvec![make_param("x", Type::Int)],
            ret_type: Type::Int,
            body: vec![
                HirStmt::Expr(HirExpr::Var("x".to_string())),
                HirStmt::Return(Some(HirExpr::Var("x".to_string()))),
            ],
            properties: make_pure_properties(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };
        assert!(!gen.is_identity_function(&func));
    }

    // is_commutative tests
    #[test]
    fn test_is_commutative_add() {
        let gen = TestGenerator::new(TestGenConfig::default());
        let func = HirFunction {
            name: "add".to_string(),
            params: smallvec![make_param("a", Type::Int), make_param("b", Type::Int)],
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Var("a".to_string())),
                right: Box::new(HirExpr::Var("b".to_string())),
            }))],
            properties: make_pure_properties(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };
        assert!(gen.is_commutative(&func));
    }

    #[test]
    fn test_is_commutative_mul() {
        let gen = TestGenerator::new(TestGenConfig::default());
        let func = HirFunction {
            name: "mul".to_string(),
            params: smallvec![make_param("a", Type::Int), make_param("b", Type::Int)],
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Binary {
                op: BinOp::Mul,
                left: Box::new(HirExpr::Var("a".to_string())),
                right: Box::new(HirExpr::Var("b".to_string())),
            }))],
            properties: make_pure_properties(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };
        assert!(gen.is_commutative(&func));
    }

    #[test]
    fn test_is_commutative_sub_false() {
        let gen = TestGenerator::new(TestGenConfig::default());
        let func = HirFunction {
            name: "sub".to_string(),
            params: smallvec![make_param("a", Type::Int), make_param("b", Type::Int)],
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Binary {
                op: BinOp::Sub,
                left: Box::new(HirExpr::Var("a".to_string())),
                right: Box::new(HirExpr::Var("b".to_string())),
            }))],
            properties: make_pure_properties(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };
        assert!(!gen.is_commutative(&func));
    }

    #[test]
    fn test_is_commutative_string_concat_false() {
        // DEPYLER-0286: String concatenation is NOT commutative
        let gen = TestGenerator::new(TestGenConfig::default());
        let func = HirFunction {
            name: "concat".to_string(),
            params: smallvec![make_param("a", Type::String), make_param("b", Type::String)],
            ret_type: Type::String,
            body: vec![HirStmt::Return(Some(HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Var("a".to_string())),
                right: Box::new(HirExpr::Var("b".to_string())),
            }))],
            properties: make_pure_properties(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };
        assert!(!gen.is_commutative(&func));
    }

    #[test]
    fn test_is_commutative_single_param_false() {
        let gen = TestGenerator::new(TestGenConfig::default());
        let func = HirFunction {
            name: "single".to_string(),
            params: smallvec![make_param("a", Type::Int)],
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Var("a".to_string())))],
            properties: make_pure_properties(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };
        assert!(!gen.is_commutative(&func));
    }

    // is_simple_param_reference tests
    #[test]
    fn test_is_simple_param_reference_true() {
        let gen = TestGenerator::new(TestGenConfig::default());
        let expr = HirExpr::Var("x".to_string());
        assert!(gen.is_simple_param_reference(&expr, "x"));
    }

    #[test]
    fn test_is_simple_param_reference_false_different_name() {
        let gen = TestGenerator::new(TestGenConfig::default());
        let expr = HirExpr::Var("x".to_string());
        assert!(!gen.is_simple_param_reference(&expr, "y"));
    }

    #[test]
    fn test_is_simple_param_reference_false_not_var() {
        let gen = TestGenerator::new(TestGenConfig::default());
        let expr = HirExpr::Literal(crate::hir::Literal::Int(1));
        assert!(!gen.is_simple_param_reference(&expr, "x"));
    }

    // is_associative tests
    #[test]
    fn test_is_associative_always_false() {
        let gen = TestGenerator::new(TestGenConfig::default());
        let func = HirFunction {
            name: "add".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![],
            properties: make_pure_properties(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };
        assert!(!gen.is_associative(&func));
    }

    // returns_non_negative tests
    #[test]
    fn test_returns_non_negative_abs() {
        let gen = TestGenerator::new(TestGenConfig::default());
        let func = HirFunction {
            name: "my_abs".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![],
            properties: make_pure_properties(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };
        assert!(gen.returns_non_negative(&func));
    }

    #[test]
    fn test_returns_non_negative_magnitude() {
        let gen = TestGenerator::new(TestGenConfig::default());
        let func = HirFunction {
            name: "magnitude".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![],
            properties: make_pure_properties(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };
        assert!(gen.returns_non_negative(&func));
    }

    #[test]
    fn test_returns_non_negative_false() {
        let gen = TestGenerator::new(TestGenConfig::default());
        let func = HirFunction {
            name: "subtract".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![],
            properties: make_pure_properties(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };
        assert!(!gen.returns_non_negative(&func));
    }

    // preserves_length tests
    #[test]
    fn test_preserves_length_sort() {
        let gen = TestGenerator::new(TestGenConfig::default());
        let func = HirFunction {
            name: "my_sort".to_string(),
            params: smallvec![make_param("arr", Type::List(Box::new(Type::Int)))],
            ret_type: Type::List(Box::new(Type::Int)),
            body: vec![],
            properties: make_pure_properties(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };
        assert!(gen.preserves_length(&func));
    }

    #[test]
    fn test_preserves_length_map() {
        let gen = TestGenerator::new(TestGenConfig::default());
        let func = HirFunction {
            name: "double_map".to_string(),
            params: smallvec![make_param("arr", Type::List(Box::new(Type::Int)))],
            ret_type: Type::List(Box::new(Type::Int)),
            body: vec![],
            properties: make_pure_properties(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };
        assert!(gen.preserves_length(&func));
    }

    #[test]
    fn test_preserves_length_false_no_list() {
        let gen = TestGenerator::new(TestGenConfig::default());
        let func = HirFunction {
            name: "my_sort".to_string(),
            params: smallvec![make_param("x", Type::Int)],
            ret_type: Type::Int,
            body: vec![],
            properties: make_pure_properties(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };
        assert!(!gen.preserves_length(&func));
    }

    // is_idempotent tests
    #[test]
    fn test_is_idempotent_normalize() {
        let gen = TestGenerator::new(TestGenConfig::default());
        let func = HirFunction {
            name: "normalize_path".to_string(),
            params: smallvec![],
            ret_type: Type::String,
            body: vec![],
            properties: make_pure_properties(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };
        assert!(gen.is_idempotent(&func));
    }

    #[test]
    fn test_is_idempotent_clean() {
        let gen = TestGenerator::new(TestGenConfig::default());
        let func = HirFunction {
            name: "clean_text".to_string(),
            params: smallvec![],
            ret_type: Type::String,
            body: vec![],
            properties: make_pure_properties(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };
        assert!(gen.is_idempotent(&func));
    }

    #[test]
    fn test_is_idempotent_false() {
        let gen = TestGenerator::new(TestGenConfig::default());
        let func = HirFunction {
            name: "increment".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![],
            properties: make_pure_properties(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };
        assert!(!gen.is_idempotent(&func));
    }

    // is_sorting_function tests
    #[test]
    fn test_is_sorting_function_true() {
        let gen = TestGenerator::new(TestGenConfig::default());
        let func = HirFunction {
            name: "bubble_sort".to_string(),
            params: smallvec![make_param("arr", Type::List(Box::new(Type::Int)))],
            ret_type: Type::List(Box::new(Type::Int)),
            body: vec![],
            properties: make_pure_properties(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };
        assert!(gen.is_sorting_function(&func));
    }

    #[test]
    fn test_is_sorting_function_no_params_false() {
        // DEPYLER-0189: Sorting function must have at least one param
        let gen = TestGenerator::new(TestGenConfig::default());
        let func = HirFunction {
            name: "get_sorted".to_string(),
            params: smallvec![],
            ret_type: Type::List(Box::new(Type::Int)),
            body: vec![],
            properties: make_pure_properties(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };
        assert!(!gen.is_sorting_function(&func));
    }

    #[test]
    fn test_is_sorting_function_no_sort_in_name() {
        let gen = TestGenerator::new(TestGenConfig::default());
        let func = HirFunction {
            name: "order_items".to_string(),
            params: smallvec![make_param("arr", Type::List(Box::new(Type::Int)))],
            ret_type: Type::List(Box::new(Type::Int)),
            body: vec![],
            properties: make_pure_properties(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };
        assert!(!gen.is_sorting_function(&func));
    }

    // type_to_quickcheck_type tests
    #[test]
    fn test_type_to_quickcheck_type_int() {
        let gen = TestGenerator::new(TestGenConfig::default());
        let result = gen.type_to_quickcheck_type(&Type::Int);
        assert_eq!(result.to_string(), "i32");
    }

    #[test]
    fn test_type_to_quickcheck_type_float() {
        let gen = TestGenerator::new(TestGenConfig::default());
        let result = gen.type_to_quickcheck_type(&Type::Float);
        assert_eq!(result.to_string(), "f64");
    }

    #[test]
    fn test_type_to_quickcheck_type_string() {
        let gen = TestGenerator::new(TestGenConfig::default());
        let result = gen.type_to_quickcheck_type(&Type::String);
        assert_eq!(result.to_string(), "String");
    }

    #[test]
    fn test_type_to_quickcheck_type_bool() {
        let gen = TestGenerator::new(TestGenConfig::default());
        let result = gen.type_to_quickcheck_type(&Type::Bool);
        assert_eq!(result.to_string(), "bool");
    }

    #[test]
    fn test_type_to_quickcheck_type_list() {
        let gen = TestGenerator::new(TestGenConfig::default());
        let result = gen.type_to_quickcheck_type(&Type::List(Box::new(Type::Int)));
        assert_eq!(result.to_string(), "Vec < i32 >");
    }

    #[test]
    fn test_type_to_quickcheck_type_unsupported() {
        let gen = TestGenerator::new(TestGenConfig::default());
        let result = gen.type_to_quickcheck_type(&Type::None);
        assert_eq!(result.to_string(), "()");
    }

    // analyze_function_properties tests
    #[test]
    fn test_analyze_function_properties_identity() {
        let gen = TestGenerator::new(TestGenConfig::default());
        let func = HirFunction {
            name: "identity".to_string(),
            params: smallvec![make_param("x", Type::Int)],
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Var("x".to_string())))],
            properties: make_pure_properties(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };
        let props = gen.analyze_function_properties(&func);
        assert!(props.contains(&TestProperty::Identity));
    }

    #[test]
    fn test_analyze_function_properties_commutative() {
        let gen = TestGenerator::new(TestGenConfig::default());
        let func = HirFunction {
            name: "add".to_string(),
            params: smallvec![make_param("a", Type::Int), make_param("b", Type::Int)],
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Var("a".to_string())),
                right: Box::new(HirExpr::Var("b".to_string())),
            }))],
            properties: make_pure_properties(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };
        let props = gen.analyze_function_properties(&func);
        assert!(props.contains(&TestProperty::Commutative));
    }

    #[test]
    fn test_analyze_function_properties_sorting() {
        let gen = TestGenerator::new(TestGenConfig::default());
        let func = HirFunction {
            name: "my_sort".to_string(),
            params: smallvec![make_param("arr", Type::List(Box::new(Type::Int)))],
            ret_type: Type::List(Box::new(Type::Int)),
            body: vec![],
            properties: make_pure_properties(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };
        let props = gen.analyze_function_properties(&func);
        assert!(props.contains(&TestProperty::Sorted));
        assert!(props.contains(&TestProperty::SameElements));
        assert!(props.contains(&TestProperty::LengthPreserving));
    }

    // TestProperty tests
    #[test]
    fn test_property_eq() {
        assert_eq!(TestProperty::Identity, TestProperty::Identity);
        assert_ne!(TestProperty::Identity, TestProperty::Commutative);
    }

    #[test]
    fn test_property_clone() {
        let prop = TestProperty::NonNegative;
        let cloned = prop.clone();
        assert_eq!(prop, cloned);
    }

    #[test]
    fn test_property_debug() {
        let prop = TestProperty::Idempotent;
        let debug_str = format!("{:?}", prop);
        assert_eq!(debug_str, "Idempotent");
    }
}
