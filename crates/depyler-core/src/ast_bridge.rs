use crate::hir::*;
use anyhow::{bail, Result};
use depyler_annotations::{AnnotationExtractor, AnnotationParser, TranspilationAnnotations};
use rustpython_ast::{self as ast};

mod converters;
mod properties;
mod type_extraction;

pub use converters::{ExprConverter, StmtConverter};
pub use properties::FunctionAnalyzer;
pub use type_extraction::TypeExtractor;

pub struct AstBridge {
    source_code: Option<String>,
    annotation_extractor: AnnotationExtractor,
    annotation_parser: AnnotationParser,
}

impl Default for AstBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl AstBridge {
    pub fn new() -> Self {
        Self {
            source_code: None,
            annotation_extractor: AnnotationExtractor::new(),
            annotation_parser: AnnotationParser::new(),
        }
    }

    pub fn with_source(mut self, source: String) -> Self {
        self.source_code = Some(source);
        self
    }

    pub fn python_to_hir(&self, module: ast::Mod) -> Result<HirModule> {
        match module {
            ast::Mod::Module(m) => self.convert_module(m),
            _ => bail!("Only module-level code is supported"),
        }
    }

    fn convert_module(&self, module: ast::ModModule) -> Result<HirModule> {
        let mut functions = Vec::new();
        let mut imports = Vec::new();

        for stmt in module.body {
            match stmt {
                ast::Stmt::FunctionDef(f) => {
                    functions.push(self.convert_function(f)?);
                }
                ast::Stmt::Import(i) => {
                    imports.extend(convert_import(i)?);
                }
                ast::Stmt::ImportFrom(i) => {
                    imports.extend(convert_import_from(i)?);
                }
                _ => {
                    // Skip other statements for now
                }
            }
        }

        Ok(HirModule { functions, imports })
    }

    fn convert_function(&self, func: ast::StmtFunctionDef) -> Result<HirFunction> {
        let name = func.name.to_string();
        let params = convert_parameters(&func.args)?;
        let ret_type = TypeExtractor::extract_return_type(&func.returns)?;

        // Extract annotations from source code if available
        let annotations = self.extract_function_annotations(&func);

        // Extract docstring and filter it from the body
        let (docstring, filtered_body) = extract_docstring_and_body(func.body)?;
        let properties = FunctionAnalyzer::analyze(&filtered_body);

        Ok(HirFunction {
            name,
            params: params.into(),
            ret_type,
            body: filtered_body,
            properties,
            annotations,
            docstring,
        })
    }

    fn extract_function_annotations(
        &self,
        func: &ast::StmtFunctionDef,
    ) -> TranspilationAnnotations {
        // Try to extract from source code comments first
        if let Some(source) = &self.source_code {
            if let Some(annotation_text) = self
                .annotation_extractor
                .extract_function_annotations(source, &func.name)
            {
                if let Ok(annotations) = self.annotation_parser.parse_annotations(&annotation_text)
                {
                    return annotations;
                }
            }
        }

        // Fallback: Try to extract from docstring if present
        if let Some(ast::Stmt::Expr(expr)) = func.body.first() {
            if let ast::Expr::Constant(constant) = expr.value.as_ref() {
                if let ast::Constant::Str(docstring) = &constant.value {
                    if let Ok(annotations) = self.annotation_parser.parse_annotations(docstring) {
                        return annotations;
                    }
                }
            }
        }

        TranspilationAnnotations::default()
    }
}

// Keep the old function for backwards compatibility
pub fn python_to_hir(module: ast::Mod) -> Result<HirModule> {
    AstBridge::new().python_to_hir(module)
}

fn convert_parameters(args: &ast::Arguments) -> Result<Vec<(Symbol, Type)>> {
    let mut params = Vec::new();

    for arg in args.args.iter() {
        let name = arg.def.arg.to_string();
        let ty = if let Some(annotation) = &arg.def.annotation {
            TypeExtractor::extract_type(annotation)?
        } else {
            Type::Unknown
        };
        params.push((name, ty));
    }

    Ok(params)
}

pub(crate) fn convert_body(body: Vec<ast::Stmt>) -> Result<Vec<HirStmt>> {
    body.into_iter().map(convert_stmt).collect()
}

fn convert_stmt(stmt: ast::Stmt) -> Result<HirStmt> {
    StmtConverter::convert(stmt)
}

pub(crate) fn extract_assign_target(expr: &ast::Expr) -> Result<Symbol> {
    match expr {
        ast::Expr::Name(n) => Ok(n.id.to_string()),
        _ => bail!("Only simple name targets supported for assignment"),
    }
}

pub(crate) fn convert_expr(expr: ast::Expr) -> Result<HirExpr> {
    ExprConverter::convert(expr)
}

pub(crate) fn convert_binop(op: &ast::Operator) -> Result<BinOp> {
    Ok(match op {
        ast::Operator::Add => BinOp::Add,
        ast::Operator::Sub => BinOp::Sub,
        ast::Operator::Mult => BinOp::Mul,
        ast::Operator::Div => BinOp::Div,
        ast::Operator::FloorDiv => BinOp::FloorDiv,
        ast::Operator::Mod => BinOp::Mod,
        ast::Operator::Pow => BinOp::Pow,
        ast::Operator::BitAnd => BinOp::BitAnd,
        ast::Operator::BitOr => BinOp::BitOr,
        ast::Operator::BitXor => BinOp::BitXor,
        ast::Operator::LShift => BinOp::LShift,
        ast::Operator::RShift => BinOp::RShift,
        _ => bail!("Unsupported binary operator"),
    })
}

pub(crate) fn convert_aug_op(op: &ast::Operator) -> Result<BinOp> {
    // Augmented assignment operators map to the same binary operators
    convert_binop(op)
}

pub(crate) fn convert_unaryop(op: &ast::UnaryOp) -> Result<UnaryOp> {
    Ok(match op {
        ast::UnaryOp::Not => UnaryOp::Not,
        ast::UnaryOp::UAdd => UnaryOp::Pos,
        ast::UnaryOp::USub => UnaryOp::Neg,
        ast::UnaryOp::Invert => UnaryOp::BitNot,
    })
}

pub(crate) fn convert_cmpop(op: &ast::CmpOp) -> Result<BinOp> {
    Ok(match op {
        ast::CmpOp::Eq => BinOp::Eq,
        ast::CmpOp::NotEq => BinOp::NotEq,
        ast::CmpOp::Lt => BinOp::Lt,
        ast::CmpOp::LtE => BinOp::LtEq,
        ast::CmpOp::Gt => BinOp::Gt,
        ast::CmpOp::GtE => BinOp::GtEq,
        ast::CmpOp::In => BinOp::In,
        ast::CmpOp::NotIn => BinOp::NotIn,
        _ => bail!("Unsupported comparison operator"),
    })
}

fn convert_import(import: ast::StmtImport) -> Result<Vec<Import>> {
    import
        .names
        .into_iter()
        .map(|alias| {
            let module = alias.name.to_string();
            let items = if let Some(asname) = alias.asname {
                vec![ImportItem::Aliased {
                    name: module.clone(),
                    alias: asname.to_string(),
                }]
            } else {
                vec![ImportItem::Named(module.clone())]
            };
            Ok(Import { module, items })
        })
        .collect()
}

fn convert_import_from(import: ast::StmtImportFrom) -> Result<Vec<Import>> {
    let module = import.module.map(|m| m.to_string()).unwrap_or_default();

    let items = import
        .names
        .into_iter()
        .map(|alias| {
            let name = alias.name.to_string();
            if let Some(asname) = alias.asname {
                ImportItem::Aliased {
                    name,
                    alias: asname.to_string(),
                }
            } else {
                ImportItem::Named(name)
            }
        })
        .collect();

    Ok(vec![Import { module, items }])
}

fn extract_docstring_and_body(body: Vec<ast::Stmt>) -> Result<(Option<String>, Vec<HirStmt>)> {
    if body.is_empty() {
        return Ok((None, vec![]));
    }

    // Check if the first statement is a string literal (docstring)
    let docstring = if let ast::Stmt::Expr(expr) = &body[0] {
        if let ast::Expr::Constant(constant) = expr.value.as_ref() {
            if let ast::Constant::Str(s) = &constant.value {
                Some(s.clone())
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    // Convert the body, skipping the docstring if it exists
    let start_index = if docstring.is_some() { 1 } else { 0 };
    let filtered_body = body
        .into_iter()
        .skip(start_index)
        .map(convert_stmt)
        .collect::<Result<Vec<_>>>()?;

    Ok((docstring, filtered_body))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustpython_ast::Suite;
    use rustpython_parser::Parse;

    fn parse_python_to_hir(source: &str) -> HirModule {
        let statements = Suite::parse(source, "<test>").unwrap();
        let ast = rustpython_ast::Mod::Module(rustpython_ast::ModModule {
            body: statements,
            type_ignores: vec![],
            range: Default::default(),
        });
        AstBridge::new()
            .with_source(source.to_string())
            .python_to_hir(ast)
            .unwrap()
    }

    #[test]
    fn test_simple_function_conversion() {
        let source = "def add(a: int, b: int) -> int:\n    return a + b";
        let hir = parse_python_to_hir(source);

        assert_eq!(hir.functions.len(), 1);
        let func = &hir.functions[0];
        assert_eq!(func.name, "add");
        assert_eq!(func.params.len(), 2);
        assert_eq!(func.params[0].0, "a");
        assert_eq!(func.params[0].1, Type::Int);
        assert_eq!(func.ret_type, Type::Int);
    }

    #[test]
    fn test_type_annotation_conversion() {
        let source = "def process(items: List[str]) -> Optional[int]:\n    return None";
        let hir = parse_python_to_hir(source);

        let func = &hir.functions[0];
        assert_eq!(func.params[0].1, Type::List(Box::new(Type::String)));
        assert_eq!(func.ret_type, Type::Optional(Box::new(Type::Int)));
    }

    #[test]
    fn test_import_conversion() {
        let source = "from typing import List, Dict\nimport sys";
        let hir = parse_python_to_hir(source);

        assert_eq!(hir.imports.len(), 2);
        assert_eq!(hir.imports[0].module, "typing");
        assert_eq!(hir.imports[1].module, "sys");
    }

    #[test]
    fn test_control_flow_conversion() {
        let source = r#"
def check(x: int) -> str:
    if x > 0:
        return "positive"
    else:
        return "negative"
"#;
        let hir = parse_python_to_hir(source);

        let func = &hir.functions[0];
        assert_eq!(func.body.len(), 1);
        if let HirStmt::If {
            condition,
            then_body,
            else_body,
        } = &func.body[0]
        {
            assert!(matches!(condition, HirExpr::Binary { op: BinOp::Gt, .. }));
            assert_eq!(then_body.len(), 1);
            assert!(else_body.is_some());
        } else {
            panic!("Expected if statement");
        }
    }

    #[test]
    fn test_binary_operations() {
        let source = "def calc() -> int:\n    return 1 + 2 * 3";
        let hir = parse_python_to_hir(source);

        let func = &hir.functions[0];
        if let HirStmt::Return(Some(HirExpr::Binary { op, .. })) = &func.body[0] {
            assert_eq!(*op, BinOp::Add);
        } else {
            panic!("Expected binary operation in return");
        }
    }

    #[test]
    fn test_function_properties_analysis() {
        let source = r#"
def pure_func(x: int) -> int:
    return x + 1

def impure_func(x: int):
    print(x)
"#;
        let hir = parse_python_to_hir(source);

        assert_eq!(hir.functions.len(), 2);
        assert!(hir.functions[0].properties.is_pure);
        assert!(!hir.functions[1].properties.is_pure);
    }

    #[test]
    fn test_for_loop_conversion() {
        let source = r#"
def iterate(items: list) -> int:
    total = 0
    for item in items:
        total = total + item
    return total
"#;
        let hir = parse_python_to_hir(source);

        let func = &hir.functions[0];
        assert_eq!(func.body.len(), 3); // assign, for, return
        if let HirStmt::For { target, iter, body } = &func.body[1] {
            assert_eq!(target, "item");
            assert!(matches!(iter, HirExpr::Var(_)));
            assert_eq!(body.len(), 1);
        } else {
            panic!("Expected for loop");
        }
    }

    #[test]
    fn test_expression_types() {
        let source = r#"
def expressions():
    x = [1, 2, 3]
    z = (1, 2, 3)
"#;
        let hir = parse_python_to_hir(source);

        let func = &hir.functions[0];
        assert_eq!(func.body.len(), 2);

        // Check list assignment
        if let HirStmt::Assign {
            value: HirExpr::List(_),
            ..
        } = &func.body[0]
        {
            // OK
        } else {
            panic!("Expected list assignment");
        }

        // Check tuple assignment
        if let HirStmt::Assign {
            value: HirExpr::Tuple(_),
            ..
        } = &func.body[1]
        {
            // OK
        } else {
            panic!("Expected tuple assignment");
        }
    }

    #[test]
    fn test_comparison_operators() {
        let source = r#"
def compare(a: int, b: int) -> bool:
    return a > b
"#;
        let hir = parse_python_to_hir(source);

        let func = &hir.functions[0];
        if let HirStmt::Return(Some(HirExpr::Binary { op: BinOp::Gt, .. })) = &func.body[0] {
            // OK - simple comparison works
        } else {
            panic!("Expected > comparison");
        }
    }

    #[test]
    fn test_unary_operations() {
        let source = r#"
def unary_ops(x: int) -> int:
    return -x + +x
"#;
        let hir = parse_python_to_hir(source);

        let func = &hir.functions[0];
        if let HirStmt::Return(Some(HirExpr::Binary {
            op: BinOp::Add,
            left,
            right,
        })) = &func.body[0]
        {
            assert!(matches!(
                left.as_ref(),
                HirExpr::Unary {
                    op: UnaryOp::Neg,
                    ..
                }
            ));
            assert!(matches!(
                right.as_ref(),
                HirExpr::Unary {
                    op: UnaryOp::Pos,
                    ..
                }
            ));
        } else {
            panic!("Expected unary operations");
        }
    }

    #[test]
    fn test_function_calls() {
        let source = r#"
def call_functions() -> int:
    return len([1, 2, 3])
"#;
        let hir = parse_python_to_hir(source);

        let func = &hir.functions[0];
        if let HirStmt::Return(Some(HirExpr::Call { func: fname, args })) = &func.body[0] {
            assert_eq!(fname, "len");
            assert_eq!(args.len(), 1);
            assert!(matches!(args[0], HirExpr::List(_)));
        } else {
            panic!("Expected function call");
        }
    }

    #[test]
    fn test_annotation_extraction() {
        let source = r#"
# @depyler: type_strategy = "aggressive"
# @depyler: optimization_level = "aggressive"
# @depyler: thread_safety = "required"
def process_data(items: List[int]) -> int:
    total = 0
    for x in items:
        total = total + x * 2
    return total
"#;
        let hir = parse_python_to_hir(source);

        let func = &hir.functions[0];
        assert_eq!(
            func.annotations.type_strategy,
            depyler_annotations::TypeStrategy::Aggressive
        );
        assert_eq!(
            func.annotations.optimization_level,
            depyler_annotations::OptimizationLevel::Aggressive
        );
        assert_eq!(
            func.annotations.thread_safety,
            depyler_annotations::ThreadSafety::Required
        );
    }

    #[test]
    fn test_annotation_with_performance_hints() {
        let source = r#"
# @depyler: performance_critical = "true"
# @depyler: vectorize = "true"
# @depyler: bounds_checking = "disabled"
def compute(data: List[float]) -> float:
    total = 0.0
    for x in data:
        total += x
    return total
"#;
        let hir = parse_python_to_hir(source);

        let func = &hir.functions[0];
        assert!(func
            .annotations
            .performance_hints
            .contains(&depyler_annotations::PerformanceHint::PerformanceCritical));
        assert!(func
            .annotations
            .performance_hints
            .contains(&depyler_annotations::PerformanceHint::Vectorize));
        assert_eq!(
            func.annotations.bounds_checking,
            depyler_annotations::BoundsChecking::Disabled
        );
    }

    #[test]
    fn test_docstring_extraction() {
        let source = r#"
def example_function(x: int) -> int:
    """This is a docstring that should become a comment"""
    return x * 2

def function_without_docstring(y: int) -> int:
    print("Not a docstring") 
    return y + 1
"#;
        let hir = parse_python_to_hir(source);

        assert_eq!(hir.functions.len(), 2);

        // First function should have a docstring
        let func_with_docstring = &hir.functions[0];
        assert_eq!(func_with_docstring.name, "example_function");
        assert_eq!(
            func_with_docstring.docstring,
            Some("This is a docstring that should become a comment".to_string())
        );
        assert_eq!(func_with_docstring.body.len(), 1); // Only the return statement

        // Second function should not have a docstring
        let func_without_docstring = &hir.functions[1];
        assert_eq!(func_without_docstring.name, "function_without_docstring");
        assert_eq!(func_without_docstring.docstring, None);
        assert_eq!(func_without_docstring.body.len(), 2); // print statement + return
    }
}
