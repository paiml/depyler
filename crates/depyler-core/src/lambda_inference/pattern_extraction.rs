//! Pattern extraction from Python AST for Lambda event type inference
//!
//! This module extracts access patterns from Python AST expressions to enable
//! AWS Lambda event type inference.

use crate::lambda_inference::{InferenceError, Pattern, PatternType};
use rustpython_ast::{Expr, ExprAttribute, ExprSubscript, Stmt, StmtFunctionDef};

/// Extract access patterns from a list of statements
pub fn extract_access_patterns(statements: &[Stmt]) -> Result<Vec<Pattern>, InferenceError> {
    let mut patterns = Vec::new();

    for stmt in statements {
        if let Stmt::FunctionDef(func_def) = stmt {
            patterns.extend(extract_patterns_from_function(func_def)?);
        }
    }

    Ok(patterns)
}

/// Extract patterns from a function definition
pub fn extract_patterns_from_function(
    func_def: &StmtFunctionDef,
) -> Result<Vec<Pattern>, InferenceError> {
    let mut patterns = Vec::new();

    for stmt in &func_def.body {
        patterns.extend(extract_patterns_from_stmt(stmt)?);
    }

    Ok(patterns)
}

/// Extract patterns from a single statement
pub fn extract_patterns_from_stmt(stmt: &Stmt) -> Result<Vec<Pattern>, InferenceError> {
    let mut patterns = Vec::new();

    match stmt {
        Stmt::Assign(assign) => {
            for target in &assign.targets {
                patterns.extend(extract_patterns_from_expr(&assign.value)?);
                patterns.extend(extract_patterns_from_expr(target)?);
            }
        }
        Stmt::AnnAssign(ann_assign) => {
            if let Some(ref value) = ann_assign.value {
                patterns.extend(extract_patterns_from_expr(value)?);
            } else {
                patterns.extend(extract_patterns_from_expr(&ann_assign.target)?);
            }
        }
        Stmt::Return(ret) => {
            if let Some(value) = &ret.value {
                patterns.extend(extract_patterns_from_expr(value)?);
            }
        }
        Stmt::If(if_stmt) => {
            patterns.extend(extract_patterns_from_expr(&if_stmt.test)?);
            for stmt in &if_stmt.body {
                patterns.extend(extract_patterns_from_stmt(stmt)?);
            }
            for stmt in &if_stmt.orelse {
                patterns.extend(extract_patterns_from_stmt(stmt)?);
            }
        }
        Stmt::For(for_stmt) => {
            patterns.extend(extract_patterns_from_expr(&for_stmt.iter)?);
            for stmt in &for_stmt.body {
                patterns.extend(extract_patterns_from_stmt(stmt)?);
            }
        }
        Stmt::While(while_stmt) => {
            patterns.extend(extract_patterns_from_expr(&while_stmt.test)?);
            for stmt in &while_stmt.body {
                patterns.extend(extract_patterns_from_stmt(stmt)?);
            }
        }
        Stmt::With(with_stmt) => {
            for item in &with_stmt.items {
                patterns.extend(extract_patterns_from_expr(&item.context_expr)?);
            }
            for stmt in &with_stmt.body {
                patterns.extend(extract_patterns_from_stmt(stmt)?);
            }
        }
        Stmt::Expr(expr_stmt) => {
            patterns.extend(extract_patterns_from_expr(&expr_stmt.value)?);
        }
        _ => {}
    }

    Ok(patterns)
}

/// Extract patterns from an expression
pub fn extract_patterns_from_expr(expr: &Expr) -> Result<Vec<Pattern>, InferenceError> {
    let mut patterns = Vec::new();

    match expr {
        Expr::Subscript(subscript) => {
            if let Some(pattern) = extract_subscript_pattern(subscript)? {
                patterns.push(pattern);
            }
            patterns.extend(extract_patterns_from_expr(&subscript.value)?);
        }
        Expr::Attribute(attr) => {
            if let Some(pattern) = extract_attribute_pattern(attr)? {
                patterns.push(pattern);
            }
            patterns.extend(extract_patterns_from_expr(&attr.value)?);
        }
        Expr::Call(call) => {
            patterns.extend(extract_patterns_from_expr(&call.func)?);
            for arg in &call.args {
                patterns.extend(extract_patterns_from_expr(arg)?);
            }
            for keyword in &call.keywords {
                patterns.extend(extract_patterns_from_expr(&keyword.value)?);
            }
        }
        Expr::BinOp(binop) => {
            patterns.extend(extract_patterns_from_expr(&binop.left)?);
            patterns.extend(extract_patterns_from_expr(&binop.right)?);
        }
        Expr::Compare(compare) => {
            patterns.extend(extract_patterns_from_expr(&compare.left)?);
            for comp in &compare.comparators {
                patterns.extend(extract_patterns_from_expr(comp)?);
            }
        }
        Expr::BoolOp(boolop) => {
            for value in &boolop.values {
                patterns.extend(extract_patterns_from_expr(value)?);
            }
        }
        Expr::UnaryOp(unaryop) => {
            patterns.extend(extract_patterns_from_expr(&unaryop.operand)?);
        }
        Expr::IfExp(ifexp) => {
            patterns.extend(extract_patterns_from_expr(&ifexp.test)?);
            patterns.extend(extract_patterns_from_expr(&ifexp.body)?);
            patterns.extend(extract_patterns_from_expr(&ifexp.orelse)?);
        }
        Expr::Dict(dict) => {
            for value in &dict.values {
                patterns.extend(extract_patterns_from_expr(value)?);
            }
        }
        Expr::List(list) => {
            for elt in &list.elts {
                patterns.extend(extract_patterns_from_expr(elt)?);
            }
        }
        Expr::Tuple(tuple) => {
            for elt in &tuple.elts {
                patterns.extend(extract_patterns_from_expr(elt)?);
            }
        }
        _ => {}
    }

    Ok(patterns)
}

/// Extract a pattern from a subscript expression like event['Records']
pub fn extract_subscript_pattern(
    subscript: &ExprSubscript,
) -> Result<Option<Pattern>, InferenceError> {
    let mut access_chain = Vec::new();
    let mut current_expr = &subscript.value;

    // Extract the subscript key
    if let Expr::Constant(constant) = &*subscript.slice {
        if let Some(key) = constant.value.as_str() {
            access_chain.insert(0, key.to_string());
        }
        // Skip numeric indices - they don't contribute to pattern matching
    }

    // Walk up the access chain
    loop {
        match &**current_expr {
            Expr::Subscript(inner_subscript) => {
                if let Expr::Constant(constant) = &*inner_subscript.slice {
                    if let Some(key) = constant.value.as_str() {
                        access_chain.insert(0, key.to_string());
                    }
                }
                current_expr = &inner_subscript.value;
            }
            Expr::Attribute(attr) => {
                access_chain.insert(0, attr.attr.to_string());
                current_expr = &attr.value;
            }
            Expr::Name(name) => {
                if name.id.as_str() == "event" {
                    return Ok(Some(Pattern {
                        access_chain,
                        pattern_type: PatternType::Mixed,
                    }));
                }
                break;
            }
            _ => break,
        }
    }

    Ok(None)
}

/// Extract a pattern from an attribute expression like event.body
pub fn extract_attribute_pattern(attr: &ExprAttribute) -> Result<Option<Pattern>, InferenceError> {
    let mut access_chain = vec![attr.attr.to_string()];
    let mut current_expr = &attr.value;

    // Walk up the access chain
    loop {
        match &**current_expr {
            Expr::Attribute(inner_attr) => {
                access_chain.insert(0, inner_attr.attr.to_string());
                current_expr = &inner_attr.value;
            }
            Expr::Subscript(subscript) => {
                if let Expr::Constant(constant) = &*subscript.slice {
                    if let Some(key) = constant.value.as_str() {
                        access_chain.insert(0, key.to_string());
                    }
                }
                current_expr = &subscript.value;
            }
            Expr::Name(name) => {
                if name.id.as_str() == "event" {
                    return Ok(Some(Pattern {
                        access_chain,
                        pattern_type: PatternType::Attribute,
                    }));
                }
                break;
            }
            _ => break,
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustpython_ast::{Mod, ModModule};
    use rustpython_parser::Parse;

    fn parse_python(source: &str) -> Mod {
        rustpython_ast::Suite::parse(source, "<test>")
            .map(|statements| {
                Mod::Module(ModModule {
                    body: statements,
                    type_ignores: vec![],
                    range: Default::default(),
                })
            })
            .unwrap()
    }

    fn get_patterns(code: &str) -> Vec<Pattern> {
        let ast = parse_python(code);
        match ast {
            Mod::Module(module) => extract_access_patterns(&module.body).unwrap(),
            _ => vec![],
        }
    }

    // ========================================
    // extract_subscript_pattern tests
    // ========================================

    #[test]
    fn test_subscript_simple_event_access() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    x = event['Records']
"#,
        );
        assert!(patterns.iter().any(|p| p.access_chain == vec!["Records"]));
    }

    #[test]
    fn test_subscript_nested_event_access() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    x = event['Records']['s3']
"#,
        );
        assert!(patterns
            .iter()
            .any(|p| p.access_chain == vec!["Records", "s3"]));
    }

    #[test]
    fn test_subscript_deeply_nested_access() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    x = event['Records']['s3']['bucket']['name']
"#,
        );
        assert!(patterns
            .iter()
            .any(|p| p.access_chain == vec!["Records", "s3", "bucket", "name"]));
    }

    #[test]
    fn test_subscript_numeric_index_skipped() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    x = event['Records'][0]['s3']
"#,
        );
        // Numeric indices should be skipped
        assert!(patterns
            .iter()
            .any(|p| p.access_chain == vec!["Records", "s3"]));
    }

    #[test]
    fn test_subscript_non_event_ignored() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    data = {'foo': 'bar'}
    x = data['foo']
"#,
        );
        // Non-event subscripts should not produce patterns
        assert!(patterns.is_empty());
    }

    #[test]
    fn test_subscript_mixed_with_attribute() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    x = event['Records'][0].data
"#,
        );
        // Should capture mixed patterns
        assert!(!patterns.is_empty());
    }

    // ========================================
    // extract_attribute_pattern tests
    // ========================================

    #[test]
    fn test_attribute_simple_access() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    x = event.body
"#,
        );
        assert!(patterns.iter().any(|p| p.access_chain == vec!["body"]));
        assert!(patterns
            .iter()
            .any(|p| p.pattern_type == PatternType::Attribute));
    }

    #[test]
    fn test_attribute_nested_access() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    x = event.body.data
"#,
        );
        assert!(patterns
            .iter()
            .any(|p| p.access_chain == vec!["body", "data"]));
    }

    #[test]
    fn test_attribute_non_event_ignored() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    obj = SomeClass()
    x = obj.attribute
"#,
        );
        assert!(patterns.is_empty());
    }

    // ========================================
    // Statement extraction tests
    // ========================================

    #[test]
    fn test_extract_from_assign() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    x = event['body']
"#,
        );
        assert!(!patterns.is_empty());
    }

    #[test]
    fn test_extract_from_annotated_assign() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    x: str = event['body']
"#,
        );
        assert!(patterns.iter().any(|p| p.access_chain == vec!["body"]));
    }

    #[test]
    fn test_extract_from_annotated_assign_no_value() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    x: str
"#,
        );
        // No event access, should be empty
        assert!(patterns.is_empty());
    }

    #[test]
    fn test_extract_from_return() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    return event['data']
"#,
        );
        assert!(patterns.iter().any(|p| p.access_chain == vec!["data"]));
    }

    #[test]
    fn test_extract_from_return_none() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    return
"#,
        );
        assert!(patterns.is_empty());
    }

    #[test]
    fn test_extract_from_if_test() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    if event['status']:
        pass
"#,
        );
        assert!(patterns.iter().any(|p| p.access_chain == vec!["status"]));
    }

    #[test]
    fn test_extract_from_if_body() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    if True:
        x = event['body']
"#,
        );
        assert!(patterns.iter().any(|p| p.access_chain == vec!["body"]));
    }

    #[test]
    fn test_extract_from_if_else() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    if True:
        pass
    else:
        x = event['data']
"#,
        );
        assert!(patterns.iter().any(|p| p.access_chain == vec!["data"]));
    }

    #[test]
    fn test_extract_from_for_iter() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    for record in event['Records']:
        pass
"#,
        );
        assert!(patterns.iter().any(|p| p.access_chain == vec!["Records"]));
    }

    #[test]
    fn test_extract_from_for_body() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    for i in range(10):
        x = event['body']
"#,
        );
        assert!(patterns.iter().any(|p| p.access_chain == vec!["body"]));
    }

    #[test]
    fn test_extract_from_while_test() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    while event['status']:
        pass
"#,
        );
        assert!(patterns.iter().any(|p| p.access_chain == vec!["status"]));
    }

    #[test]
    fn test_extract_from_while_body() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    while True:
        x = event['data']
        break
"#,
        );
        assert!(patterns.iter().any(|p| p.access_chain == vec!["data"]));
    }

    #[test]
    fn test_extract_from_with_context() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    with event['resource'] as r:
        pass
"#,
        );
        assert!(patterns.iter().any(|p| p.access_chain == vec!["resource"]));
    }

    #[test]
    fn test_extract_from_with_body() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    with open('file') as f:
        x = event['data']
"#,
        );
        assert!(patterns.iter().any(|p| p.access_chain == vec!["data"]));
    }

    #[test]
    fn test_extract_from_expr_stmt() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    event['action']
"#,
        );
        assert!(patterns.iter().any(|p| p.access_chain == vec!["action"]));
    }

    // ========================================
    // Expression extraction tests
    // ========================================

    #[test]
    fn test_extract_from_call_func() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    event['handler']()
"#,
        );
        assert!(patterns.iter().any(|p| p.access_chain == vec!["handler"]));
    }

    #[test]
    fn test_extract_from_call_args() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    process(event['data'])
"#,
        );
        assert!(patterns.iter().any(|p| p.access_chain == vec!["data"]));
    }

    #[test]
    fn test_extract_from_call_kwargs() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    process(data=event['body'])
"#,
        );
        assert!(patterns.iter().any(|p| p.access_chain == vec!["body"]));
    }

    #[test]
    fn test_extract_from_binop() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    x = event['a'] + event['b']
"#,
        );
        assert!(patterns.iter().any(|p| p.access_chain == vec!["a"]));
        assert!(patterns.iter().any(|p| p.access_chain == vec!["b"]));
    }

    #[test]
    fn test_extract_from_compare() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    if event['a'] == event['b']:
        pass
"#,
        );
        assert!(patterns.iter().any(|p| p.access_chain == vec!["a"]));
        assert!(patterns.iter().any(|p| p.access_chain == vec!["b"]));
    }

    #[test]
    fn test_extract_from_boolop() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    if event['a'] and event['b']:
        pass
"#,
        );
        assert!(patterns.iter().any(|p| p.access_chain == vec!["a"]));
        assert!(patterns.iter().any(|p| p.access_chain == vec!["b"]));
    }

    #[test]
    fn test_extract_from_unaryop() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    if not event['flag']:
        pass
"#,
        );
        assert!(patterns.iter().any(|p| p.access_chain == vec!["flag"]));
    }

    #[test]
    fn test_extract_from_ifexp() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    x = event['a'] if event['cond'] else event['b']
"#,
        );
        assert!(patterns.iter().any(|p| p.access_chain == vec!["a"]));
        assert!(patterns.iter().any(|p| p.access_chain == vec!["cond"]));
        assert!(patterns.iter().any(|p| p.access_chain == vec!["b"]));
    }

    #[test]
    fn test_extract_from_dict_values() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    return {'result': event['data']}
"#,
        );
        assert!(patterns.iter().any(|p| p.access_chain == vec!["data"]));
    }

    #[test]
    fn test_extract_from_list_elements() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    return [event['a'], event['b']]
"#,
        );
        assert!(patterns.iter().any(|p| p.access_chain == vec!["a"]));
        assert!(patterns.iter().any(|p| p.access_chain == vec!["b"]));
    }

    #[test]
    fn test_extract_from_tuple_elements() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    return (event['a'], event['b'])
"#,
        );
        assert!(patterns.iter().any(|p| p.access_chain == vec!["a"]));
        assert!(patterns.iter().any(|p| p.access_chain == vec!["b"]));
    }

    // ========================================
    // Edge cases and complex scenarios
    // ========================================

    #[test]
    fn test_multiple_functions_in_module() {
        let patterns = get_patterns(
            r#"
def helper():
    pass

def handler(event, context):
    x = event['data']
    return x

def another_helper():
    pass
"#,
        );
        assert!(patterns.iter().any(|p| p.access_chain == vec!["data"]));
    }

    #[test]
    fn test_empty_function() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    pass
"#,
        );
        assert!(patterns.is_empty());
    }

    #[test]
    fn test_no_function_def() {
        let patterns = get_patterns(
            r#"
x = 1
y = 2
"#,
        );
        assert!(patterns.is_empty());
    }

    #[test]
    fn test_nested_function() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    def inner():
        return event['inner_data']
    return event['outer_data']
"#,
        );
        // Only outer patterns are extracted
        assert!(patterns
            .iter()
            .any(|p| p.access_chain == vec!["outer_data"]));
    }

    #[test]
    fn test_complex_s3_pattern() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    bucket = event['Records'][0]['s3']['bucket']['name']
    key = event['Records'][0]['s3']['object']['key']
    return {'bucket': bucket, 'key': key}
"#,
        );
        assert!(patterns.len() >= 2);
        assert!(patterns
            .iter()
            .any(|p| p.access_chain.contains(&"bucket".to_string())));
        assert!(patterns
            .iter()
            .any(|p| p.access_chain.contains(&"object".to_string())));
    }

    #[test]
    fn test_api_gateway_pattern() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    method = event['requestContext']['http']['method']
    path = event['requestContext']['http']['path']
    body = event['body']
    return {'method': method, 'path': path}
"#,
        );
        assert!(patterns.len() >= 3);
        assert!(patterns
            .iter()
            .any(|p| p.access_chain.contains(&"requestContext".to_string())));
        assert!(patterns.iter().any(|p| p.access_chain == vec!["body"]));
    }

    #[test]
    fn test_eventbridge_pattern() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    detail_type = event['detail-type']
    detail = event['detail']
    source = event['source']
    return None
"#,
        );
        assert!(patterns
            .iter()
            .any(|p| p.access_chain == vec!["detail-type"]));
        assert!(patterns.iter().any(|p| p.access_chain == vec!["detail"]));
        assert!(patterns.iter().any(|p| p.access_chain == vec!["source"]));
    }

    #[test]
    fn test_pattern_type_is_mixed_for_subscript() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    x = event['data']
"#,
        );
        assert!(patterns
            .iter()
            .all(|p| p.pattern_type == PatternType::Mixed));
    }

    #[test]
    fn test_pattern_type_is_attribute_for_dot_access() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    x = event.data
"#,
        );
        assert!(patterns
            .iter()
            .any(|p| p.pattern_type == PatternType::Attribute));
    }

    #[test]
    fn test_multiple_targets_in_assign() {
        let patterns = get_patterns(
            r#"
def handler(event, context):
    x = y = event['data']
"#,
        );
        assert!(patterns.iter().any(|p| p.access_chain == vec!["data"]));
    }
}
