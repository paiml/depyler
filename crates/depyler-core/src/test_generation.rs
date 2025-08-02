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

    /// Generate tests for a function if applicable
    pub fn generate_tests(&self, func: &HirFunction) -> Result<Option<proc_macro2::TokenStream>> {
        // Only generate tests for pure functions
        if !func.properties.is_pure {
            return Ok(None);
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

        if test_functions.is_empty() {
            return Ok(None);
        }

        Ok(Some(quote! {
            #[cfg(test)]
            mod tests {
                use super::*;
                use quickcheck::{quickcheck, TestResult};

                #(#test_functions)*
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
            .map(|(_, ty)| self.type_to_quickcheck_type(ty))
            .collect();

        let param_names: Vec<_> = func
            .params
            .iter()
            .map(|(name, _)| syn::Ident::new(name, proc_macro2::Span::call_site()))
            .collect();

        let property_checks: Vec<_> = properties
            .iter()
            .map(|prop| self.property_to_assertion(prop, &func_name, &param_names))
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
                return name == &func.params[0].0;
            }
        }
        false
    }

    /// Check if function is commutative (like addition)
    fn is_commutative(&self, func: &HirFunction) -> bool {
        if func.params.len() == 2 && func.body.len() == 1 {
            if let HirStmt::Return(Some(HirExpr::Binary { op, left, right })) = &func.body[0] {
                // Check if it's a commutative operation
                matches!(
                    op,
                    BinOp::Add | BinOp::Mul | BinOp::BitAnd | BinOp::BitOr | BinOp::BitXor
                ) && self.is_simple_param_reference(left, &func.params[0].0)
                    && self.is_simple_param_reference(right, &func.params[1].0)
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
            if let (Type::List(_), Type::List(_)) = (&func.params[0].1, &func.ret_type) {
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
        func.name.contains("sort")
    }

    /// Convert Python type to quickcheck-compatible type
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

    /// Convert property to assertion code
    fn property_to_assertion(
        &self,
        prop: &TestProperty,
        func_name: &syn::Ident,
        params: &[syn::Ident],
    ) -> proc_macro2::TokenStream {
        match prop {
            TestProperty::Identity => {
                let param = &params[0];
                quote! {
                    let result = #func_name(#param.clone());
                    if result != #param {
                        return TestResult::failed();
                    }
                }
            }
            TestProperty::Commutative => {
                let (a, b) = (&params[0], &params[1]);
                quote! {
                    let result1 = #func_name(#a.clone(), #b.clone());
                    let result2 = #func_name(#b.clone(), #a.clone());
                    if result1 != result2 {
                        return TestResult::failed();
                    }
                }
            }
            TestProperty::NonNegative => {
                quote! {
                    let result = #func_name(#(#params.clone()),*);
                    if result < 0 {
                        return TestResult::failed();
                    }
                }
            }
            TestProperty::LengthPreserving => {
                let param = &params[0];
                quote! {
                    let input_len = #param.len();
                    let result = #func_name(#param.clone());
                    if result.len() != input_len {
                        return TestResult::failed();
                    }
                }
            }
            TestProperty::Sorted => {
                quote! {
                    let result = #func_name(#(#params.clone()),*);
                    for i in 1..result.len() {
                        if result[i-1] > result[i] {
                            return TestResult::failed();
                        }
                    }
                }
            }
            TestProperty::SameElements => {
                let param = &params[0];
                quote! {
                    let mut input_sorted = #param.clone();
                    input_sorted.sort();
                    let mut result = #func_name(#param.clone());
                    result.sort();
                    if input_sorted != result {
                        return TestResult::failed();
                    }
                }
            }
            TestProperty::Idempotent => {
                quote! {
                    let once = #func_name(#(#params.clone()),*);
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
            (Type::Int, 2)
                if matches!(&func.params[0].1, Type::Int)
                    && matches!(&func.params[1].1, Type::Int) =>
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
                if func.params.len() == 1 && matches!(&func.params[0].1, Type::List(_)) {
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
    Distributive,
    Monotonic,
}
