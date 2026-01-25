//! Exception Handling Helpers
//!
//! This module contains helpers for extracting and analyzing exception types
//! during code generation. Extracted from stmt_gen.rs for better testability.
//!
//! DEPYLER-0333: Exception type extraction for error handling

use crate::hir::HirExpr;

/// DEPYLER-0333: Extract exception type from raise statement expression
///
/// Extracts the exception type name from various raise patterns:
/// - `raise ValueError("msg")` → "ValueError"
/// - `raise exc_var` → variable name
/// - `raise module.ExceptionType("msg")` → "ExceptionType"
/// - Other patterns → "Exception" (fallback)
///
/// # Complexity
/// 2 (match + clone)
pub fn extract_exception_type(exception: &HirExpr) -> String {
    match exception {
        // Pattern: raise ExceptionType("message")
        HirExpr::Call { func, .. } => func.clone(),
        // Pattern: raise exception_variable
        HirExpr::Var(name) => name.clone(),
        // Pattern: raise module.ExceptionType("message")
        HirExpr::MethodCall { method, .. } => method.clone(),
        // Fallback for unknown patterns
        _ => "Exception".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::{BinOp, Literal, UnaryOp};

    // ============ Call pattern tests ============

    #[test]
    fn test_extract_exception_type_valueerror_call() {
        let expr = HirExpr::Call {
            func: "ValueError".to_string(),
            args: vec![HirExpr::Literal(Literal::String("invalid".to_string()))],
            kwargs: vec![],
        };
        assert_eq!(extract_exception_type(&expr), "ValueError");
    }

    #[test]
    fn test_extract_exception_type_typeerror_call() {
        let expr = HirExpr::Call {
            func: "TypeError".to_string(),
            args: vec![HirExpr::Literal(Literal::String("wrong type".to_string()))],
            kwargs: vec![],
        };
        assert_eq!(extract_exception_type(&expr), "TypeError");
    }

    #[test]
    fn test_extract_exception_type_keyerror_call() {
        let expr = HirExpr::Call {
            func: "KeyError".to_string(),
            args: vec![HirExpr::Literal(Literal::String("missing key".to_string()))],
            kwargs: vec![],
        };
        assert_eq!(extract_exception_type(&expr), "KeyError");
    }

    #[test]
    fn test_extract_exception_type_indexerror_call() {
        let expr = HirExpr::Call {
            func: "IndexError".to_string(),
            args: vec![HirExpr::Literal(Literal::String(
                "out of range".to_string(),
            ))],
            kwargs: vec![],
        };
        assert_eq!(extract_exception_type(&expr), "IndexError");
    }

    #[test]
    fn test_extract_exception_type_runtimeerror_call() {
        let expr = HirExpr::Call {
            func: "RuntimeError".to_string(),
            args: vec![HirExpr::Literal(Literal::String(
                "runtime issue".to_string(),
            ))],
            kwargs: vec![],
        };
        assert_eq!(extract_exception_type(&expr), "RuntimeError");
    }

    #[test]
    fn test_extract_exception_type_filenotfounderror_call() {
        let expr = HirExpr::Call {
            func: "FileNotFoundError".to_string(),
            args: vec![HirExpr::Literal(Literal::String("no file".to_string()))],
            kwargs: vec![],
        };
        assert_eq!(extract_exception_type(&expr), "FileNotFoundError");
    }

    #[test]
    fn test_extract_exception_type_zerodivisionerror_call() {
        let expr = HirExpr::Call {
            func: "ZeroDivisionError".to_string(),
            args: vec![HirExpr::Literal(Literal::String("div by zero".to_string()))],
            kwargs: vec![],
        };
        assert_eq!(extract_exception_type(&expr), "ZeroDivisionError");
    }

    #[test]
    fn test_extract_exception_type_syntaxerror_call() {
        let expr = HirExpr::Call {
            func: "SyntaxError".to_string(),
            args: vec![HirExpr::Literal(Literal::String("bad syntax".to_string()))],
            kwargs: vec![],
        };
        assert_eq!(extract_exception_type(&expr), "SyntaxError");
    }

    #[test]
    fn test_extract_exception_type_ioerror_call() {
        let expr = HirExpr::Call {
            func: "IOError".to_string(),
            args: vec![HirExpr::Literal(Literal::String("io fail".to_string()))],
            kwargs: vec![],
        };
        assert_eq!(extract_exception_type(&expr), "IOError");
    }

    #[test]
    fn test_extract_exception_type_attributeerror_call() {
        let expr = HirExpr::Call {
            func: "AttributeError".to_string(),
            args: vec![HirExpr::Literal(Literal::String("no attr".to_string()))],
            kwargs: vec![],
        };
        assert_eq!(extract_exception_type(&expr), "AttributeError");
    }

    #[test]
    fn test_extract_exception_type_stopiteration_call() {
        let expr = HirExpr::Call {
            func: "StopIteration".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert_eq!(extract_exception_type(&expr), "StopIteration");
    }

    #[test]
    fn test_extract_exception_type_call_no_args() {
        let expr = HirExpr::Call {
            func: "Exception".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert_eq!(extract_exception_type(&expr), "Exception");
    }

    #[test]
    fn test_extract_exception_type_call_multiple_args() {
        let expr = HirExpr::Call {
            func: "CustomError".to_string(),
            args: vec![
                HirExpr::Literal(Literal::String("msg".to_string())),
                HirExpr::Literal(Literal::Int(42)),
            ],
            kwargs: vec![],
        };
        assert_eq!(extract_exception_type(&expr), "CustomError");
    }

    #[test]
    fn test_extract_exception_type_call_with_kwargs() {
        let expr = HirExpr::Call {
            func: "ValidationError".to_string(),
            args: vec![],
            kwargs: vec![(
                "message".to_string(),
                HirExpr::Literal(Literal::String("bad".to_string())),
            )],
        };
        assert_eq!(extract_exception_type(&expr), "ValidationError");
    }

    #[test]
    fn test_extract_exception_type_argumenttypeerror_call() {
        let expr = HirExpr::Call {
            func: "ArgumentTypeError".to_string(),
            args: vec![HirExpr::Literal(Literal::String("bad arg".to_string()))],
            kwargs: vec![],
        };
        assert_eq!(extract_exception_type(&expr), "ArgumentTypeError");
    }

    // ============ Variable pattern tests ============

    #[test]
    fn test_extract_exception_type_var_exc() {
        let expr = HirExpr::Var("exc".to_string());
        assert_eq!(extract_exception_type(&expr), "exc");
    }

    #[test]
    fn test_extract_exception_type_var_error() {
        let expr = HirExpr::Var("error".to_string());
        assert_eq!(extract_exception_type(&expr), "error");
    }

    #[test]
    fn test_extract_exception_type_var_e() {
        let expr = HirExpr::Var("e".to_string());
        assert_eq!(extract_exception_type(&expr), "e");
    }

    #[test]
    fn test_extract_exception_type_var_exception() {
        let expr = HirExpr::Var("exception".to_string());
        assert_eq!(extract_exception_type(&expr), "exception");
    }

    #[test]
    fn test_extract_exception_type_var_custom_name() {
        let expr = HirExpr::Var("my_custom_exception".to_string());
        assert_eq!(extract_exception_type(&expr), "my_custom_exception");
    }

    #[test]
    fn test_extract_exception_type_var_err() {
        let expr = HirExpr::Var("err".to_string());
        assert_eq!(extract_exception_type(&expr), "err");
    }

    // ============ MethodCall pattern tests ============

    #[test]
    fn test_extract_exception_type_method_call_argparse() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("argparse".to_string())),
            method: "ArgumentTypeError".to_string(),
            args: vec![HirExpr::Literal(Literal::String("bad arg".to_string()))],
            kwargs: vec![],
        };
        assert_eq!(extract_exception_type(&expr), "ArgumentTypeError");
    }

    #[test]
    fn test_extract_exception_type_method_call_module_error() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("mymodule".to_string())),
            method: "CustomError".to_string(),
            args: vec![HirExpr::Literal(Literal::String("msg".to_string()))],
            kwargs: vec![],
        };
        assert_eq!(extract_exception_type(&expr), "CustomError");
    }

    #[test]
    fn test_extract_exception_type_method_call_http_error() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("http".to_string())),
            method: "HTTPError".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(404))],
            kwargs: vec![],
        };
        assert_eq!(extract_exception_type(&expr), "HTTPError");
    }

    #[test]
    fn test_extract_exception_type_method_call_json_decode() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("json".to_string())),
            method: "JSONDecodeError".to_string(),
            args: vec![HirExpr::Literal(Literal::String("bad json".to_string()))],
            kwargs: vec![],
        };
        assert_eq!(extract_exception_type(&expr), "JSONDecodeError");
    }

    #[test]
    fn test_extract_exception_type_method_call_no_args() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("errors".to_string())),
            method: "NoDataError".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert_eq!(extract_exception_type(&expr), "NoDataError");
    }

    #[test]
    fn test_extract_exception_type_method_call_nested_object() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("pkg".to_string())),
                attr: "errors".to_string(),
            }),
            method: "PackageError".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert_eq!(extract_exception_type(&expr), "PackageError");
    }

    // ============ Fallback pattern tests ============

    #[test]
    fn test_extract_exception_type_literal_string() {
        let expr = HirExpr::Literal(Literal::String("error message".to_string()));
        assert_eq!(extract_exception_type(&expr), "Exception");
    }

    #[test]
    fn test_extract_exception_type_literal_int() {
        let expr = HirExpr::Literal(Literal::Int(42));
        assert_eq!(extract_exception_type(&expr), "Exception");
    }

    #[test]
    fn test_extract_exception_type_literal_bool() {
        let expr = HirExpr::Literal(Literal::Bool(true));
        assert_eq!(extract_exception_type(&expr), "Exception");
    }

    #[test]
    fn test_extract_exception_type_binary_expr() {
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        assert_eq!(extract_exception_type(&expr), "Exception");
    }

    #[test]
    fn test_extract_exception_type_unary_expr() {
        let expr = HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(HirExpr::Literal(Literal::Int(1))),
        };
        assert_eq!(extract_exception_type(&expr), "Exception");
    }

    #[test]
    fn test_extract_exception_type_list() {
        let expr = HirExpr::List(vec![HirExpr::Literal(Literal::Int(1))]);
        assert_eq!(extract_exception_type(&expr), "Exception");
    }

    #[test]
    fn test_extract_exception_type_tuple() {
        let expr = HirExpr::Tuple(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::Int(2)),
        ]);
        assert_eq!(extract_exception_type(&expr), "Exception");
    }

    #[test]
    fn test_extract_exception_type_dict() {
        let expr = HirExpr::Dict(vec![(
            HirExpr::Literal(Literal::String("key".to_string())),
            HirExpr::Literal(Literal::Int(1)),
        )]);
        assert_eq!(extract_exception_type(&expr), "Exception");
    }

    #[test]
    fn test_extract_exception_type_index() {
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("arr".to_string())),
            index: Box::new(HirExpr::Literal(Literal::Int(0))),
        };
        assert_eq!(extract_exception_type(&expr), "Exception");
    }

    #[test]
    fn test_extract_exception_type_attribute() {
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("obj".to_string())),
            attr: "error".to_string(),
        };
        assert_eq!(extract_exception_type(&expr), "Exception");
    }

    #[test]
    fn test_extract_exception_type_if_expr() {
        let expr = HirExpr::IfExpr {
            test: Box::new(HirExpr::Literal(Literal::Bool(true))),
            body: Box::new(HirExpr::Var("exc1".to_string())),
            orelse: Box::new(HirExpr::Var("exc2".to_string())),
        };
        assert_eq!(extract_exception_type(&expr), "Exception");
    }

    #[test]
    fn test_extract_exception_type_lambda() {
        let expr = HirExpr::Lambda {
            params: vec![],
            body: Box::new(HirExpr::Literal(Literal::Int(1))),
        };
        assert_eq!(extract_exception_type(&expr), "Exception");
    }

    #[test]
    fn test_extract_exception_type_set() {
        let expr = HirExpr::Set(vec![HirExpr::Literal(Literal::Int(1))]);
        assert_eq!(extract_exception_type(&expr), "Exception");
    }

    #[test]
    fn test_extract_exception_type_slice() {
        let expr = HirExpr::Slice {
            base: Box::new(HirExpr::Var("arr".to_string())),
            start: Some(Box::new(HirExpr::Literal(Literal::Int(0)))),
            stop: Some(Box::new(HirExpr::Literal(Literal::Int(10)))),
            step: None,
        };
        assert_eq!(extract_exception_type(&expr), "Exception");
    }

    #[test]
    fn test_extract_exception_type_await() {
        let expr = HirExpr::Await {
            value: Box::new(HirExpr::Var("coro".to_string())),
        };
        assert_eq!(extract_exception_type(&expr), "Exception");
    }

    #[test]
    fn test_extract_exception_type_fstring() {
        let expr = HirExpr::FString { parts: vec![] };
        assert_eq!(extract_exception_type(&expr), "Exception");
    }

    #[test]
    fn test_extract_exception_type_list_comp() {
        let expr = HirExpr::ListComp {
            element: Box::new(HirExpr::Var("x".to_string())),
            generators: vec![],
        };
        assert_eq!(extract_exception_type(&expr), "Exception");
    }

    #[test]
    fn test_extract_exception_type_dict_comp() {
        let expr = HirExpr::DictComp {
            key: Box::new(HirExpr::Var("k".to_string())),
            value: Box::new(HirExpr::Var("v".to_string())),
            generators: vec![],
        };
        assert_eq!(extract_exception_type(&expr), "Exception");
    }

    #[test]
    fn test_extract_exception_type_set_comp() {
        let expr = HirExpr::SetComp {
            element: Box::new(HirExpr::Var("x".to_string())),
            generators: vec![],
        };
        assert_eq!(extract_exception_type(&expr), "Exception");
    }

    #[test]
    fn test_extract_exception_type_generator_exp() {
        let expr = HirExpr::GeneratorExp {
            element: Box::new(HirExpr::Var("x".to_string())),
            generators: vec![],
        };
        assert_eq!(extract_exception_type(&expr), "Exception");
    }

    #[test]
    fn test_extract_exception_type_yield() {
        let expr = HirExpr::Yield {
            value: Some(Box::new(HirExpr::Literal(Literal::Int(1)))),
        };
        assert_eq!(extract_exception_type(&expr), "Exception");
    }

    #[test]
    fn test_extract_exception_type_yield_none() {
        let expr = HirExpr::Yield { value: None };
        assert_eq!(extract_exception_type(&expr), "Exception");
    }
}
