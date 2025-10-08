use depyler_core::hir::{HirExpr, HirFunction, HirStmt, Type};
use serde::{Deserialize, Serialize};

use crate::contract_verification::{
    InvariantChecker, PostconditionVerifier, PreconditionChecker, VerificationResult,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    pub preconditions: Vec<Condition>,
    pub postconditions: Vec<Condition>,
    pub invariants: Vec<Condition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub name: String,
    pub expression: String,
    pub description: String,
}

pub struct ContractChecker;

impl ContractChecker {
    pub fn extract_contracts(func: &HirFunction) -> Contract {
        let mut contract = Contract {
            preconditions: vec![],
            postconditions: vec![],
            invariants: vec![],
        };

        Self::extract_docstring_into_contract(&mut contract, &func.docstring);
        Self::extract_param_preconditions(&mut contract, &func.params);
        Self::extract_return_postconditions(&mut contract, &func.ret_type);
        Self::extract_property_invariants(&mut contract, &func.properties);

        contract
    }

    fn extract_docstring_into_contract(contract: &mut Contract, docstring: &Option<String>) {
        if let Some(doc) = docstring {
            let extracted = Self::extract_docstring_contracts(doc);
            contract.preconditions.extend(extracted.preconditions);
            contract.postconditions.extend(extracted.postconditions);
            contract.invariants.extend(extracted.invariants);
        }
    }

    fn extract_param_preconditions(
        contract: &mut Contract,
        params: &[depyler_core::hir::HirParam],
    ) {
        for param in params {
            if let Type::List(_) = &param.ty {
                contract.preconditions.push(Condition {
                    name: format!("{}_not_null", param.name),
                    expression: format!("{} is not None", param.name),
                    description: format!("Parameter {} must not be null", param.name),
                });
            }
        }
    }

    fn extract_return_postconditions(contract: &mut Contract, ret_type: &Type) {
        match ret_type {
            Type::Optional(_) => {
                contract.postconditions.push(Condition {
                    name: "result_valid".to_string(),
                    expression: "result is None or result meets type constraints".to_string(),
                    description: "Result must be None or valid value".to_string(),
                });
            }
            Type::List(_) => {
                contract.postconditions.push(Condition {
                    name: "result_not_null".to_string(),
                    expression: "result is not None".to_string(),
                    description: "Result list must not be null".to_string(),
                });
            }
            _ => {}
        }
    }

    fn extract_property_invariants(
        contract: &mut Contract,
        properties: &depyler_core::hir::FunctionProperties,
    ) {
        if properties.panic_free {
            contract.invariants.push(Condition {
                name: "no_panics".to_string(),
                expression: "all array accesses are bounds-checked".to_string(),
                description: "Function must not panic on any input".to_string(),
            });
        }

        if properties.always_terminates {
            contract.invariants.push(Condition {
                name: "termination".to_string(),
                expression: "loop variants decrease monotonically".to_string(),
                description: "Function must terminate for all inputs".to_string(),
            });
        }
    }

    pub fn generate_contract_checks(contract: &Contract, func_name: &str) -> String {
        let mut checks = String::new();

        if !contract.preconditions.is_empty() {
            checks.push_str(&Self::generate_precondition_function(
                &contract.preconditions,
                func_name,
            ));
        }

        if !contract.postconditions.is_empty() {
            checks.push_str(&Self::generate_postcondition_function(
                &contract.postconditions,
                func_name,
            ));
        }

        checks
    }

    fn generate_precondition_function(preconditions: &[Condition], func_name: &str) -> String {
        let mut result = format!("fn check_{func_name}_preconditions(");
        result.push_str("/* params */) -> Result<(), &'static str> {\n");

        for pre in preconditions {
            result.push_str(&format!("    // {}\n", pre.description));
            result.push_str(&Self::generate_precondition_check(pre));
        }

        result.push_str("    Ok(())\n");
        result.push_str("}\n\n");
        result
    }

    fn generate_precondition_check(condition: &Condition) -> String {
        if condition.expression.contains("is not None") {
            let var_name = condition
                .expression
                .split_whitespace()
                .next()
                .unwrap_or("");
            format!(
                "    if {}.is_none() {{ return Err(\"Precondition failed: {}\"); }}\n",
                var_name, condition.description
            )
        } else {
            let check_expr = condition.expression.replace("self.", "");
            format!(
                "    debug_assert!({}, \"Precondition failed: {}\");\n",
                check_expr, condition.description
            )
        }
    }

    fn generate_postcondition_function(postconditions: &[Condition], func_name: &str) -> String {
        let mut result = format!("fn check_{func_name}_postconditions(");
        result.push_str("/* result */) -> Result<(), &'static str> {\n");

        for post in postconditions {
            result.push_str(&format!("    // {}\n", post.description));
            result.push_str(&Self::generate_postcondition_check(post));
        }

        result.push_str("    Ok(())\n");
        result.push_str("}\n\n");
        result
    }

    fn generate_postcondition_check(condition: &Condition) -> String {
        let check_expr = condition.expression.replace("self.", "");
        if condition.expression.contains("result") {
            format!(
                "    if !({}) {{ return Err(\"Postcondition failed: {}\"); }}\n",
                check_expr, condition.description
            )
        } else {
            format!(
                "    debug_assert!({}, \"Postcondition failed: {}\");\n",
                check_expr, condition.description
            )
        }
    }

    pub fn check_contract_violations(func: &HirFunction) -> Vec<String> {
        let mut violations = Vec::new();

        // Check for potential contract violations in the function body
        for stmt in &func.body {
            violations.extend(check_stmt_contracts(stmt));
        }

        violations
    }

    /// Extract contracts from Python docstring annotations
    fn extract_docstring_contracts(docstring: &str) -> Contract {
        let mut contract = Contract {
            preconditions: vec![],
            postconditions: vec![],
            invariants: vec![],
        };

        for line in docstring.lines() {
            let trimmed = line.trim();
            Self::parse_docstring_line(trimmed, &mut contract);
        }

        contract
    }

    fn parse_docstring_line(line: &str, contract: &mut Contract) {
        if line.starts_with("@requires") {
            Self::parse_requires_line(line, contract);
        } else if line.starts_with("@ensures") {
            Self::parse_ensures_line(line, contract);
        } else if line.starts_with("@invariant") {
            Self::parse_invariant_line(line, contract);
        }
    }

    fn parse_requires_line(line: &str, contract: &mut Contract) {
        if let Some(annotation) = line.strip_prefix("@requires").map(str::trim) {
            if !annotation.is_empty() {
                contract.preconditions.push(Condition {
                    name: format!("requires_{}", contract.preconditions.len()),
                    expression: annotation.to_string(),
                    description: format!("Requires: {}", annotation),
                });
            }
        }
    }

    fn parse_ensures_line(line: &str, contract: &mut Contract) {
        if let Some(annotation) = line.strip_prefix("@ensures").map(str::trim) {
            if !annotation.is_empty() {
                contract.postconditions.push(Condition {
                    name: format!("ensures_{}", contract.postconditions.len()),
                    expression: annotation.to_string(),
                    description: format!("Ensures: {}", annotation),
                });
            }
        }
    }

    fn parse_invariant_line(line: &str, contract: &mut Contract) {
        if let Some(annotation) = line.strip_prefix("@invariant").map(str::trim) {
            if !annotation.is_empty() {
                contract.invariants.push(Condition {
                    name: format!("invariant_{}", contract.invariants.len()),
                    expression: annotation.to_string(),
                    description: format!("Invariant: {}", annotation),
                });
            }
        }
    }

    /// Verify contracts using the advanced verification framework
    pub fn verify_contracts(func: &HirFunction) -> VerificationResult {
        let mut precondition_checker = PreconditionChecker::new();
        let mut postcondition_verifier = PostconditionVerifier::new();
        let invariant_checker = InvariantChecker::new();

        // Extract and verify contracts
        let contract = Self::extract_contracts(func);

        // Parse and validate preconditions
        if let Some(docstring) = &func.docstring {
            let rules = precondition_checker.parse_requires_annotations(docstring);
            for rule in rules {
                precondition_checker.add_rule(rule);
            }
        }

        // Verify preconditions
        let mut result = precondition_checker.validate_preconditions(func);

        // Capture pre-state for postcondition verification
        postcondition_verifier.capture_pre_state(func);

        // Verify postconditions
        let post_result = postcondition_verifier.verify_postconditions(func, &contract);

        // Merge results
        result.violations.extend(post_result.violations);
        result.warnings.extend(post_result.warnings);
        result
            .proven_conditions
            .extend(post_result.proven_conditions);
        result
            .unproven_conditions
            .extend(post_result.unproven_conditions);
        result.success = result.success && post_result.success;

        // Check invariants
        let invariant_violations = invariant_checker.check_invariants(func);
        for violation in invariant_violations {
            result.violations.push(violation);
            result.success = false;
        }

        result
    }

    /// Generate runtime assertions using the verification framework
    pub fn generate_advanced_contract_checks(contract: &Contract, _func: &HirFunction) -> String {
        let mut checks = String::new();
        let precondition_checker = PreconditionChecker::new();
        let postcondition_verifier = PostconditionVerifier::new();

        // Generate precondition runtime assertions
        if !contract.preconditions.is_empty() {
            checks.push_str("    // Precondition checks\n");
            let assertions = precondition_checker.generate_runtime_assertions(contract);
            checks.push_str(&assertions);
        }

        // Generate postcondition runtime checks
        if !contract.postconditions.is_empty() {
            checks.push_str("\n    // Postcondition checks\n");
            let post_checks = postcondition_verifier.generate_postcondition_checks(contract);
            checks.push_str(&post_checks);
        }

        checks
    }

    /// Generate complete function with contract checks
    pub fn generate_function_with_contracts(
        func: &HirFunction,
        body_code: &str,
        include_runtime_checks: bool,
    ) -> String {
        let contract = Self::extract_contracts(func);
        let mut result = String::new();

        Self::append_contract_documentation(&mut result, &contract);
        Self::append_function_signature(&mut result, func);
        result.push_str(" {\n");

        if include_runtime_checks {
            Self::append_precondition_checks(&mut result, &contract);
            Self::append_old_value_storage(&mut result, &contract);
        }

        result.push_str(body_code);

        if include_runtime_checks {
            Self::append_postcondition_checks(&mut result, &contract);
        }

        result.push_str("}\n");
        result
    }

    fn append_contract_documentation(result: &mut String, contract: &Contract) {
        if contract.preconditions.is_empty()
            && contract.postconditions.is_empty()
            && contract.invariants.is_empty()
        {
            return;
        }

        result.push_str("/// Contract specifications:\n");
        for pre in &contract.preconditions {
            result.push_str(&format!("/// @requires {}\n", pre.expression));
        }
        for post in &contract.postconditions {
            result.push_str(&format!("/// @ensures {}\n", post.expression));
        }
        for inv in &contract.invariants {
            result.push_str(&format!("/// @invariant {}\n", inv.expression));
        }
    }

    fn append_function_signature(result: &mut String, func: &HirFunction) {
        result.push_str(&format!("pub fn {}", func.name));
        result.push('(');

        let params: Vec<String> = func
            .params
            .iter()
            .map(|param| format!("{}: {}", param.name, type_to_rust_string(&param.ty)))
            .collect();
        result.push_str(&params.join(", "));
        result.push(')');

        if !matches!(func.ret_type, Type::None) {
            result.push_str(&format!(" -> {}", type_to_rust_string(&func.ret_type)));
        }
    }

    fn append_precondition_checks(result: &mut String, contract: &Contract) {
        if contract.preconditions.is_empty() {
            return;
        }

        result.push_str("    // Contract precondition validation\n");
        for pre in &contract.preconditions {
            result.push_str(&format!(
                "    assert!({}, \"Precondition violated: {}\");\n",
                pre.expression.replace("is not None", "is_some()"),
                pre.description
            ));
        }
        result.push('\n');
    }

    fn append_old_value_storage(result: &mut String, contract: &Contract) {
        if contract
            .postconditions
            .iter()
            .any(|p| p.expression.contains("old("))
        {
            result.push_str("    // Store old values for postcondition checks\n");
            result.push('\n');
        }
    }

    fn append_postcondition_checks(result: &mut String, contract: &Contract) {
        if contract.postconditions.is_empty() {
            return;
        }

        result.push_str("\n    // Contract postcondition validation\n");
        for post in &contract.postconditions {
            if post.expression.contains("result") {
                result.push_str(&format!("    // Postcondition: {}\n", post.description));
            }
        }
    }
}

/// Convert HIR type to Rust type string
fn type_to_rust_string(ty: &Type) -> String {
    if let Some(simple) = convert_simple_type(ty) {
        return simple;
    }
    if let Some(container) = convert_container_type(ty) {
        return container;
    }
    format_complex_type(ty)
}

fn convert_simple_type(ty: &Type) -> Option<String> {
    match ty {
        Type::Int => Some("i32".to_string()),
        Type::Float => Some("f64".to_string()),
        Type::String => Some("String".to_string()),
        Type::Bool => Some("bool".to_string()),
        Type::None => Some("()".to_string()),
        Type::Unknown => Some("_".to_string()),
        Type::Custom(name) | Type::TypeVar(name) => Some(name.clone()),
        _ => None,
    }
}

fn convert_container_type(ty: &Type) -> Option<String> {
    match ty {
        Type::List(inner) => Some(format_container_type("Vec", inner)),
        Type::Set(inner) => Some(format_container_type("HashSet", inner)),
        Type::Optional(inner) => Some(format_container_type("Option", inner)),
        _ => None,
    }
}

fn format_complex_type(ty: &Type) -> String {
    match ty {
        Type::Dict(k, v) => format_dict_type(k, v),
        Type::Tuple(types) => format_tuple_type(types),
        Type::Function { params, ret } => format_function_type(params, ret),
        Type::Generic { base, params } => format_generic_type(base, params),
        Type::Union(types) => format_union_type(types),
        Type::Array { element_type, size } => format_array_type(element_type, size),
        _ => "_".to_string(),
    }
}

fn format_container_type(container: &str, inner: &Type) -> String {
    format!("{}<{}>", container, type_to_rust_string(inner))
}

fn format_dict_type(key: &Type, value: &Type) -> String {
    format!(
        "HashMap<{}, {}>",
        type_to_rust_string(key),
        type_to_rust_string(value)
    )
}

fn format_tuple_type(types: &[Type]) -> String {
    let type_strs: Vec<String> = types.iter().map(type_to_rust_string).collect();
    format!("({})", type_strs.join(", "))
}

fn format_function_type(params: &[Type], ret: &Type) -> String {
    let param_strs: Vec<String> = params.iter().map(type_to_rust_string).collect();
    format!("fn({}) -> {}", param_strs.join(", "), type_to_rust_string(ret))
}

fn format_generic_type(base: &str, params: &[Type]) -> String {
    let param_strs: Vec<String> = params.iter().map(type_to_rust_string).collect();
    format!("{}<{}>", base, param_strs.join(", "))
}

fn format_union_type(types: &[Type]) -> String {
    let type_strs: Vec<String> = types.iter().map(type_to_rust_string).collect();
    format!("Union<{}>", type_strs.join(", "))
}

fn format_array_type(element_type: &Type, size: &depyler_core::hir::ConstGeneric) -> String {
    let element_str = type_to_rust_string(element_type);
    let size_str = match size {
        depyler_core::hir::ConstGeneric::Literal(n) => n.to_string(),
        depyler_core::hir::ConstGeneric::Parameter(name) => name.clone(),
        depyler_core::hir::ConstGeneric::Expression(expr) => expr.clone(),
    };
    format!("[{}; {}]", element_str, size_str)
}

fn check_stmt_contracts(stmt: &HirStmt) -> Vec<String> {
    match stmt {
        HirStmt::Assign { value, .. } => check_expr_contracts(value),
        HirStmt::Return(Some(expr)) => check_expr_contracts(expr),
        HirStmt::If {
            condition,
            then_body,
            else_body,
        } => check_if_contracts(condition, then_body, else_body),
        HirStmt::While { condition, body } => check_while_contracts(condition, body),
        HirStmt::For { iter, body, .. } => check_for_contracts(iter, body),
        HirStmt::Expr(expr) => check_expr_contracts(expr),
        _ => vec![],
    }
}

fn check_if_contracts(
    condition: &HirExpr,
    then_body: &[HirStmt],
    else_body: &Option<Vec<HirStmt>>,
) -> Vec<String> {
    let mut violations = check_expr_contracts(condition);

    for stmt in then_body {
        violations.extend(check_stmt_contracts(stmt));
    }

    if let Some(else_stmts) = else_body {
        for stmt in else_stmts {
            violations.extend(check_stmt_contracts(stmt));
        }
    }

    violations
}

fn check_while_contracts(condition: &HirExpr, body: &[HirStmt]) -> Vec<String> {
    let mut violations = check_expr_contracts(condition);
    for stmt in body {
        violations.extend(check_stmt_contracts(stmt));
    }
    violations
}

fn check_for_contracts(iter: &HirExpr, body: &[HirStmt]) -> Vec<String> {
    let mut violations = check_expr_contracts(iter);
    for stmt in body {
        violations.extend(check_stmt_contracts(stmt));
    }
    violations
}

fn check_expr_contracts(expr: &HirExpr) -> Vec<String> {
    let mut violations = Vec::new();

    match expr {
        HirExpr::Index { base, .. } => {
            // Unchecked array access could violate panic-free contract
            violations.push("Potential array bounds violation".to_string());
            violations.extend(check_expr_contracts(base));
        }
        HirExpr::Binary { left, right, .. } => {
            violations.extend(check_expr_contracts(left));
            violations.extend(check_expr_contracts(right));
        }
        HirExpr::Call { args, .. } => {
            for arg in args {
                violations.extend(check_expr_contracts(arg));
            }
        }
        HirExpr::Attribute { value, .. } => {
            // Check contracts for the base value expression
            violations.extend(check_expr_contracts(value));
        }
        _ => {}
    }

    violations
}

#[cfg(test)]
mod tests {
    use super::*;
    use depyler_annotations::TranspilationAnnotations;
    use depyler_core::hir::{FunctionProperties, Literal};

    fn create_test_function(
        name: &str,
        params: Vec<depyler_core::hir::HirParam>,
        ret_type: Type,
        body: Vec<HirStmt>,
        properties: FunctionProperties,
    ) -> HirFunction {
        HirFunction {
            name: name.to_string(),
            params: params.into(),
            ret_type,
            body,
            properties,
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        }
    }

    #[test]
    fn test_contract_creation() {
        let contract = Contract {
            preconditions: vec![Condition {
                name: "param_valid".to_string(),
                expression: "param > 0".to_string(),
                description: "Parameter must be positive".to_string(),
            }],
            postconditions: vec![Condition {
                name: "result_valid".to_string(),
                expression: "result >= 0".to_string(),
                description: "Result must be non-negative".to_string(),
            }],
            invariants: vec![Condition {
                name: "no_overflow".to_string(),
                expression: "no arithmetic overflow".to_string(),
                description: "Arithmetic operations must not overflow".to_string(),
            }],
        };

        assert_eq!(contract.preconditions.len(), 1);
        assert_eq!(contract.postconditions.len(), 1);
        assert_eq!(contract.invariants.len(), 1);
        assert_eq!(contract.preconditions[0].name, "param_valid");
    }

    #[test]
    fn test_condition_creation() {
        let condition = Condition {
            name: "test_condition".to_string(),
            expression: "x > 0".to_string(),
            description: "x must be positive".to_string(),
        };

        assert_eq!(condition.name, "test_condition");
        assert_eq!(condition.expression, "x > 0");
        assert_eq!(condition.description, "x must be positive");
    }

    #[test]
    fn test_extract_contracts_with_list_param() {
        let func = create_test_function(
            "process_list",
            vec![depyler_core::hir::HirParam::new("items".to_string(), Type::List(Box::new(Type::Int)))],
            Type::Int,
            vec![],
            FunctionProperties::default(),
        );

        let contract = ContractChecker::extract_contracts(&func);

        assert_eq!(contract.preconditions.len(), 1);
        assert_eq!(contract.preconditions[0].name, "items_not_null");
        assert!(contract.preconditions[0]
            .expression
            .contains("items is not None"));
    }

    #[test]
    fn test_extract_contracts_with_int_param() {
        let func = create_test_function(
            "calculate",
            vec![depyler_core::hir::HirParam::new("num".to_string(), Type::Int)],
            Type::Int,
            vec![],
            FunctionProperties::default(),
        );

        let contract = ContractChecker::extract_contracts(&func);

        // Int parameters don't generate preconditions in the current implementation
        assert_eq!(contract.preconditions.len(), 0);
    }

    #[test]
    fn test_extract_contracts_with_optional_return() {
        let func = create_test_function(
            "find_item",
            vec![],
            Type::Optional(Box::new(Type::String)),
            vec![],
            FunctionProperties::default(),
        );

        let contract = ContractChecker::extract_contracts(&func);

        assert_eq!(contract.postconditions.len(), 1);
        assert_eq!(contract.postconditions[0].name, "result_valid");
        assert!(contract.postconditions[0]
            .expression
            .contains("result is None or result meets type constraints"));
    }

    #[test]
    fn test_extract_contracts_with_list_return() {
        let func = create_test_function(
            "get_items",
            vec![],
            Type::List(Box::new(Type::String)),
            vec![],
            FunctionProperties::default(),
        );

        let contract = ContractChecker::extract_contracts(&func);

        assert_eq!(contract.postconditions.len(), 1);
        assert_eq!(contract.postconditions[0].name, "result_not_null");
        assert_eq!(contract.postconditions[0].expression, "result is not None");
    }

    #[test]
    fn test_extract_contracts_with_panic_free_property() {
        let properties = FunctionProperties {
            is_pure: false,
            always_terminates: false,
            panic_free: true,
            max_stack_depth: Some(100),
            can_fail: false,
            error_types: vec![],
            is_async: false,
        };

        let func = create_test_function("safe_function", vec![], Type::Int, vec![], properties);

        let contract = ContractChecker::extract_contracts(&func);

        assert_eq!(contract.invariants.len(), 1);
        assert_eq!(contract.invariants[0].name, "no_panics");
        assert!(contract.invariants[0]
            .expression
            .contains("array accesses are bounds-checked"));
    }

    #[test]
    fn test_extract_contracts_with_termination_property() {
        let properties = FunctionProperties {
            is_pure: false,
            always_terminates: true,
            panic_free: false,
            max_stack_depth: Some(100),
            can_fail: false,
            error_types: vec![],
            is_async: false,
        };

        let func = create_test_function(
            "terminating_function",
            vec![],
            Type::Int,
            vec![],
            properties,
        );

        let contract = ContractChecker::extract_contracts(&func);

        assert_eq!(contract.invariants.len(), 1);
        assert_eq!(contract.invariants[0].name, "termination");
        assert!(contract.invariants[0]
            .expression
            .contains("loop variants decrease monotonically"));
    }

    #[test]
    fn test_extract_contracts_with_all_properties() {
        let properties = FunctionProperties {
            is_pure: true,
            always_terminates: true,
            panic_free: true,
            max_stack_depth: Some(100),
            can_fail: false,
            error_types: vec![],
            is_async: false,
        };

        let func = create_test_function(
            "perfect_function",
            vec![depyler_core::hir::HirParam::new("data".to_string(), Type::List(Box::new(Type::Int)))],
            Type::List(Box::new(Type::Int)),
            vec![],
            properties,
        );

        let contract = ContractChecker::extract_contracts(&func);

        // Should have preconditions for list param
        assert_eq!(contract.preconditions.len(), 1);
        // Should have postconditions for list return
        assert_eq!(contract.postconditions.len(), 1);
        // Should have invariants for both properties
        assert_eq!(contract.invariants.len(), 2);
    }

    #[test]
    fn test_generate_contract_checks() {
        let contract = Contract {
            preconditions: vec![Condition {
                name: "param_positive".to_string(),
                expression: "param > 0".to_string(),
                description: "Parameter must be positive".to_string(),
            }],
            postconditions: vec![Condition {
                name: "result_valid".to_string(),
                expression: "result >= 0".to_string(),
                description: "Result must be non-negative".to_string(),
            }],
            invariants: vec![],
        };

        let checks = ContractChecker::generate_contract_checks(&contract, "test_func");

        assert!(checks.contains("check_test_func_preconditions"));
        assert!(checks.contains("check_test_func_postconditions"));
        assert!(checks.contains("Parameter must be positive"));
        assert!(checks.contains("Result must be non-negative"));
        assert!(checks.contains("debug_assert!(param > 0"));
        // Postcondition format changed to use if statement
        assert!(
            checks.contains("if !(result >= 0)") || checks.contains("debug_assert!(result >= 0")
        );
    }

    #[test]
    fn test_generate_contract_checks_empty() {
        let contract = Contract {
            preconditions: vec![],
            postconditions: vec![],
            invariants: vec![],
        };

        let checks = ContractChecker::generate_contract_checks(&contract, "empty_func");

        // Should be empty for contract with no conditions
        assert!(checks.is_empty());
    }

    #[test]
    fn test_check_contract_violations_with_index() {
        let body = vec![HirStmt::Assign {
            target: depyler_core::hir::AssignTarget::Symbol("result".to_string()),
            value: HirExpr::Index {
                base: Box::new(HirExpr::Var("arr".to_string())),
                index: Box::new(HirExpr::Literal(Literal::Int(0))),
            },
            type_annotation: None,
        }];

        let func = create_test_function(
            "access_array",
            vec![],
            Type::Int,
            body,
            FunctionProperties::default(),
        );

        let violations = ContractChecker::check_contract_violations(&func);

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0], "Potential array bounds violation");
    }

    #[test]
    fn test_check_contract_violations_with_nested_expressions() {
        let body = vec![HirStmt::Return(Some(HirExpr::Binary {
            op: depyler_core::hir::BinOp::Add,
            left: Box::new(HirExpr::Index {
                base: Box::new(HirExpr::Var("arr1".to_string())),
                index: Box::new(HirExpr::Literal(Literal::Int(0))),
            }),
            right: Box::new(HirExpr::Index {
                base: Box::new(HirExpr::Var("arr2".to_string())),
                index: Box::new(HirExpr::Literal(Literal::Int(1))),
            }),
        }))];

        let func = create_test_function(
            "add_array_elements",
            vec![],
            Type::Int,
            body,
            FunctionProperties::default(),
        );

        let violations = ContractChecker::check_contract_violations(&func);

        // Should detect two potential array bounds violations
        assert_eq!(violations.len(), 2);
        assert!(violations
            .iter()
            .all(|v| v == "Potential array bounds violation"));
    }

    #[test]
    fn test_check_contract_violations_with_control_flow() {
        let then_body = vec![HirStmt::Assign {
            target: depyler_core::hir::AssignTarget::Symbol("x".to_string()),
            value: HirExpr::Index {
                base: Box::new(HirExpr::Var("data".to_string())),
                index: Box::new(HirExpr::Literal(Literal::Int(0))),
            },
            type_annotation: None,
        }];

        let body = vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body,
            else_body: None,
        }];

        let func = create_test_function(
            "conditional_access",
            vec![],
            Type::Int,
            body,
            FunctionProperties::default(),
        );

        let violations = ContractChecker::check_contract_violations(&func);

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0], "Potential array bounds violation");
    }

    #[test]
    fn test_check_contract_violations_no_violations() {
        let body = vec![
            HirStmt::Assign {
                target: depyler_core::hir::AssignTarget::Symbol("result".to_string()),
                value: HirExpr::Literal(Literal::Int(42)),
                type_annotation: None,
            },
            HirStmt::Return(Some(HirExpr::Var("result".to_string()))),
        ];

        let func = create_test_function(
            "safe_function",
            vec![],
            Type::Int,
            body,
            FunctionProperties::default(),
        );

        let violations = ContractChecker::check_contract_violations(&func);

        assert!(violations.is_empty());
    }

    #[test]
    fn test_contract_serialization() {
        let contract = Contract {
            preconditions: vec![Condition {
                name: "test".to_string(),
                expression: "x > 0".to_string(),
                description: "Test condition".to_string(),
            }],
            postconditions: vec![],
            invariants: vec![],
        };

        // Test that contract can be serialized to JSON
        let json = serde_json::to_string(&contract).unwrap();
        assert!(json.contains("\"name\":\"test\""));
        assert!(json.contains("\"expression\":\"x > 0\""));

        // Test that it can be deserialized back
        let deserialized: Contract = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.preconditions.len(), 1);
        assert_eq!(deserialized.preconditions[0].name, "test");
    }

    #[test]
    fn test_extract_docstring_contracts() {
        let docstring = r#"
        Binary search implementation.
        
        @requires items is not None
        @requires low >= 0
        @requires high < len(items)
        @ensures result >= -1
        @ensures result < len(items)
        @invariant low <= high
        "#;

        let contract = ContractChecker::extract_docstring_contracts(docstring);

        assert_eq!(contract.preconditions.len(), 3);
        assert_eq!(contract.postconditions.len(), 2);
        assert_eq!(contract.invariants.len(), 1);

        assert_eq!(contract.preconditions[0].expression, "items is not None");
        assert_eq!(contract.preconditions[1].expression, "low >= 0");
        assert_eq!(contract.preconditions[2].expression, "high < len(items)");

        assert_eq!(contract.postconditions[0].expression, "result >= -1");
        assert_eq!(contract.postconditions[1].expression, "result < len(items)");

        assert_eq!(contract.invariants[0].expression, "low <= high");
    }

    #[test]
    fn test_verify_contracts() {
        let func = create_test_function(
            "safe_divide",
            vec![
                depyler_core::hir::HirParam::new("numerator".to_string(), Type::Float),
                depyler_core::hir::HirParam::new("denominator".to_string(), Type::Float),
            ],
            Type::Float,
            vec![],
            FunctionProperties::default(),
        );

        let result = ContractChecker::verify_contracts(&func);

        // Should have some unproven conditions since we can't statically verify everything
        assert!(!result.unproven_conditions.is_empty() || result.success);
    }

    #[test]
    fn test_generate_advanced_contract_checks() {
        let contract = Contract {
            preconditions: vec![
                Condition {
                    name: "items_not_null".to_string(),
                    expression: "items is not None".to_string(),
                    description: "Parameter items must not be null".to_string(),
                },
                Condition {
                    name: "index_bounds".to_string(),
                    expression: "index >= 0".to_string(),
                    description: "Index must be non-negative".to_string(),
                },
            ],
            postconditions: vec![Condition {
                name: "result_valid".to_string(),
                expression: "result is not None".to_string(),
                description: "Result must not be null".to_string(),
            }],
            invariants: vec![],
        };

        let func = create_test_function(
            "get_item",
            vec![
                depyler_core::hir::HirParam::new("items".to_string(), Type::List(Box::new(Type::Int))),
                depyler_core::hir::HirParam::new("index".to_string(), Type::Int),
            ],
            Type::Optional(Box::new(Type::Int)),
            vec![],
            FunctionProperties::default(),
        );

        let checks = ContractChecker::generate_advanced_contract_checks(&contract, &func);

        assert!(checks.contains("Precondition checks"));
        assert!(checks.contains("Postcondition checks"));
    }

    #[test]
    fn test_contract_with_function_annotations() {
        let mut func = create_test_function(
            "annotated_func",
            vec![depyler_core::hir::HirParam::new("x".to_string(), Type::Int)],
            Type::Int,
            vec![],
            FunctionProperties {
                is_pure: true,
                always_terminates: true,
                panic_free: true,
                max_stack_depth: Some(10),
                can_fail: false,
                error_types: vec![],
                is_async: false,
            },
        );

        func.docstring =
            Some("@requires x > 0\n@ensures result > x\n@invariant x <= result".to_string());

        let contract = ContractChecker::extract_contracts(&func);

        // Should have contracts from docstring
        assert_eq!(contract.preconditions.len(), 1);
        assert_eq!(contract.postconditions.len(), 1);
        // Should have invariants from both docstring and properties
        assert!(contract.invariants.len() >= 3); // docstring + panic_free + termination
    }

    #[test]
    fn test_generate_function_with_contracts() {
        let func = create_test_function(
            "safe_add",
            vec![depyler_core::hir::HirParam::new("a".to_string(), Type::Int), depyler_core::hir::HirParam::new("b".to_string(), Type::Int)],
            Type::Int,
            vec![],
            FunctionProperties::default(),
        );

        let body_code = "    let result = a + b;\n    result\n";
        let generated = ContractChecker::generate_function_with_contracts(&func, body_code, true);

        // Should have function signature
        assert!(generated.contains("pub fn safe_add(a: i32, b: i32) -> i32"));
        // Should have body
        assert!(generated.contains("let result = a + b"));
    }

    #[test]
    fn test_generate_function_with_contract_checks() {
        let mut func = create_test_function(
            "checked_divide",
            vec![
                depyler_core::hir::HirParam::new("num".to_string(), Type::Float),
                depyler_core::hir::HirParam::new("denom".to_string(), Type::Float),
            ],
            Type::Float,
            vec![],
            FunctionProperties::default(),
        );

        func.docstring = Some("@requires denom != 0\n@ensures result == num / denom".to_string());

        let body_code = "    num / denom\n";
        let generated = ContractChecker::generate_function_with_contracts(&func, body_code, true);

        // Should have contract documentation
        assert!(generated.contains("/// @requires denom != 0"));
        assert!(generated.contains("/// @ensures result == num / denom"));

        // Should have precondition check
        assert!(generated.contains("Contract precondition validation"));
        assert!(generated.contains("Precondition violated"));
    }
}
