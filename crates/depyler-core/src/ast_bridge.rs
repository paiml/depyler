use crate::hir::*;
use anyhow::{bail, Result};
use rustpython_ast::{self as ast};

pub fn python_to_hir(module: ast::Mod) -> Result<HirModule> {
    match module {
        ast::Mod::Module(m) => convert_module(m),
        _ => bail!("Only module-level code is supported"),
    }
}

fn convert_module(module: ast::ModModule) -> Result<HirModule> {
    let mut functions = Vec::new();
    let mut imports = Vec::new();

    for stmt in module.body {
        match stmt {
            ast::Stmt::FunctionDef(f) => {
                functions.push(convert_function(f)?);
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

fn convert_function(func: ast::StmtFunctionDef) -> Result<HirFunction> {
    let name = func.name.to_string();
    let params = convert_parameters(&func.args)?;
    let ret_type = extract_return_type(&func.returns)?;
    let body = convert_body(func.body)?;

    let properties = analyze_function_properties(&body);

    Ok(HirFunction {
        name,
        params: params.into(),
        ret_type,
        body,
        properties,
    })
}

fn convert_parameters(args: &ast::Arguments) -> Result<Vec<(Symbol, Type)>> {
    let mut params = Vec::new();

    for arg in args.args.iter() {
        let name = arg.def.arg.to_string();
        let ty = if let Some(annotation) = &arg.def.annotation {
            extract_type(annotation)?
        } else {
            Type::Unknown
        };
        params.push((name, ty));
    }

    Ok(params)
}

fn extract_return_type(returns: &Option<Box<ast::Expr>>) -> Result<Type> {
    if let Some(ret) = returns {
        extract_type(ret)
    } else {
        Ok(Type::Unknown)
    }
}

fn extract_type(expr: &ast::Expr) -> Result<Type> {
    match expr {
        ast::Expr::Name(n) => match n.id.as_str() {
            "int" => Ok(Type::Int),
            "float" => Ok(Type::Float),
            "str" => Ok(Type::String),
            "bool" => Ok(Type::Bool),
            "None" => Ok(Type::None),
            name => Ok(Type::Custom(name.to_string())),
        },
        ast::Expr::Subscript(s) => {
            if let ast::Expr::Name(n) = s.value.as_ref() {
                match n.id.as_str() {
                    "List" => {
                        let inner = extract_type(s.slice.as_ref())?;
                        Ok(Type::List(Box::new(inner)))
                    }
                    "Dict" => {
                        if let ast::Expr::Tuple(t) = s.slice.as_ref() {
                            if t.elts.len() == 2 {
                                let key = extract_type(&t.elts[0])?;
                                let value = extract_type(&t.elts[1])?;
                                Ok(Type::Dict(Box::new(key), Box::new(value)))
                            } else {
                                bail!("Dict type requires exactly 2 type parameters")
                            }
                        } else {
                            bail!("Invalid Dict type annotation")
                        }
                    }
                    "Optional" => {
                        let inner = extract_type(s.slice.as_ref())?;
                        Ok(Type::Optional(Box::new(inner)))
                    }
                    _ => Ok(Type::Custom(n.id.to_string())),
                }
            } else {
                bail!("Complex type annotations not yet supported")
            }
        }
        _ => bail!("Unsupported type annotation"),
    }
}

fn convert_body(body: Vec<ast::Stmt>) -> Result<Vec<HirStmt>> {
    body.into_iter().map(convert_stmt).collect()
}

/// Statement converter to reduce complexity
struct StmtConverter;

impl StmtConverter {
    fn convert(stmt: ast::Stmt) -> Result<HirStmt> {
        match stmt {
            ast::Stmt::Assign(a) => Self::convert_assign(a),
            ast::Stmt::AnnAssign(a) => Self::convert_ann_assign(a),
            ast::Stmt::AugAssign(a) => Self::convert_aug_assign(a),
            ast::Stmt::Return(r) => Self::convert_return(r),
            ast::Stmt::If(i) => Self::convert_if(i),
            ast::Stmt::While(w) => Self::convert_while(w),
            ast::Stmt::For(f) => Self::convert_for(f),
            ast::Stmt::Expr(e) => Self::convert_expr_stmt(e),
            _ => bail!("Statement type not yet supported"),
        }
    }

    fn convert_assign(a: ast::StmtAssign) -> Result<HirStmt> {
        if a.targets.len() != 1 {
            bail!("Multiple assignment targets not supported");
        }
        let target = extract_assign_target(&a.targets[0])?;
        let value = convert_expr(*a.value)?;
        Ok(HirStmt::Assign { target, value })
    }

    fn convert_ann_assign(a: ast::StmtAnnAssign) -> Result<HirStmt> {
        let target = extract_assign_target(&a.target)?;
        let value = if let Some(v) = a.value {
            convert_expr(*v)?
        } else {
            bail!("Annotated assignment without value not supported")
        };
        Ok(HirStmt::Assign { target, value })
    }

    fn convert_return(r: ast::StmtReturn) -> Result<HirStmt> {
        let value = r.value.map(|v| convert_expr(*v)).transpose()?;
        Ok(HirStmt::Return(value))
    }

    fn convert_if(i: ast::StmtIf) -> Result<HirStmt> {
        let condition = convert_expr(*i.test)?;
        let then_body = convert_body(i.body)?;
        let else_body = if i.orelse.is_empty() {
            None
        } else {
            Some(convert_body(i.orelse)?)
        };
        Ok(HirStmt::If {
            condition,
            then_body,
            else_body,
        })
    }

    fn convert_while(w: ast::StmtWhile) -> Result<HirStmt> {
        let condition = convert_expr(*w.test)?;
        let body = convert_body(w.body)?;
        Ok(HirStmt::While { condition, body })
    }

    fn convert_for(f: ast::StmtFor) -> Result<HirStmt> {
        let target = extract_assign_target(&f.target)?;
        let iter = convert_expr(*f.iter)?;
        let body = convert_body(f.body)?;
        Ok(HirStmt::For { target, iter, body })
    }

    fn convert_expr_stmt(e: ast::StmtExpr) -> Result<HirStmt> {
        let expr = convert_expr(*e.value)?;
        Ok(HirStmt::Expr(expr))
    }

    fn convert_aug_assign(a: ast::StmtAugAssign) -> Result<HirStmt> {
        let target = extract_assign_target(&a.target)?;
        let op = convert_aug_op(&a.op)?;
        let left = Box::new(HirExpr::Var(target.clone()));
        let right = Box::new(convert_expr(*a.value)?);
        let value = HirExpr::Binary { op, left, right };
        Ok(HirStmt::Assign { target, value })
    }
}

fn convert_stmt(stmt: ast::Stmt) -> Result<HirStmt> {
    StmtConverter::convert(stmt)
}

fn extract_assign_target(expr: &ast::Expr) -> Result<Symbol> {
    match expr {
        ast::Expr::Name(n) => Ok(n.id.to_string()),
        _ => bail!("Only simple name targets supported for assignment"),
    }
}

/// Expression converter to reduce complexity
struct ExprConverter;

impl ExprConverter {
    fn convert(expr: ast::Expr) -> Result<HirExpr> {
        match expr {
            ast::Expr::Constant(c) => Self::convert_constant(c),
            ast::Expr::Name(n) => Self::convert_name(n),
            ast::Expr::BinOp(b) => Self::convert_binop_expr(b),
            ast::Expr::UnaryOp(u) => Self::convert_unaryop_expr(u),
            ast::Expr::Call(c) => Self::convert_call(c),
            ast::Expr::Subscript(s) => Self::convert_subscript(s),
            ast::Expr::List(l) => Self::convert_list(l),
            ast::Expr::Dict(d) => Self::convert_dict(d),
            ast::Expr::Tuple(t) => Self::convert_tuple(t),
            ast::Expr::Compare(c) => Self::convert_compare(c),
            _ => bail!("Expression type not yet supported"),
        }
    }

    fn convert_constant(c: ast::ExprConstant) -> Result<HirExpr> {
        let lit = match &c.value {
            ast::Constant::Int(i) => {
                // Convert BigInt to i64, with overflow handling
                let int_val = i.try_into().unwrap_or(0i64);
                Literal::Int(int_val)
            }
            ast::Constant::Float(f) => Literal::Float(*f),
            ast::Constant::Str(s) => Literal::String(s.to_string()),
            ast::Constant::Bool(b) => Literal::Bool(*b),
            ast::Constant::None => Literal::None,
            _ => bail!("Unsupported constant type"),
        };
        Ok(HirExpr::Literal(lit))
    }

    fn convert_name(n: ast::ExprName) -> Result<HirExpr> {
        Ok(HirExpr::Var(n.id.to_string()))
    }

    fn convert_binop_expr(b: ast::ExprBinOp) -> Result<HirExpr> {
        let op = convert_binop(&b.op)?;
        let left = Box::new(Self::convert(*b.left)?);
        let right = Box::new(Self::convert(*b.right)?);
        Ok(HirExpr::Binary { op, left, right })
    }

    fn convert_unaryop_expr(u: ast::ExprUnaryOp) -> Result<HirExpr> {
        let op = convert_unaryop(&u.op)?;
        let operand = Box::new(Self::convert(*u.operand)?);
        Ok(HirExpr::Unary { op, operand })
    }

    fn convert_call(c: ast::ExprCall) -> Result<HirExpr> {
        let func = if let ast::Expr::Name(n) = c.func.as_ref() {
            n.id.to_string()
        } else {
            bail!("Only simple function calls supported")
        };
        let args = c
            .args
            .into_iter()
            .map(Self::convert)
            .collect::<Result<Vec<_>>>()?;
        Ok(HirExpr::Call { func, args })
    }

    fn convert_subscript(s: ast::ExprSubscript) -> Result<HirExpr> {
        let base = Box::new(Self::convert(*s.value)?);
        let index = Box::new(Self::convert(*s.slice)?);
        Ok(HirExpr::Index { base, index })
    }

    fn convert_list(l: ast::ExprList) -> Result<HirExpr> {
        let elts = l
            .elts
            .into_iter()
            .map(Self::convert)
            .collect::<Result<Vec<_>>>()?;
        Ok(HirExpr::List(elts))
    }

    fn convert_dict(d: ast::ExprDict) -> Result<HirExpr> {
        let mut items = Vec::new();
        for (k, v) in d.keys.into_iter().zip(d.values.into_iter()) {
            if let Some(key) = k {
                let key_expr = Self::convert(key)?;
                let val_expr = Self::convert(v)?;
                items.push((key_expr, val_expr));
            } else {
                bail!("Dict unpacking not supported");
            }
        }
        Ok(HirExpr::Dict(items))
    }

    fn convert_tuple(t: ast::ExprTuple) -> Result<HirExpr> {
        let elts = t
            .elts
            .into_iter()
            .map(Self::convert)
            .collect::<Result<Vec<_>>>()?;
        Ok(HirExpr::Tuple(elts))
    }

    fn convert_compare(c: ast::ExprCompare) -> Result<HirExpr> {
        // Convert simple comparisons to binary ops
        if c.ops.len() != 1 || c.comparators.len() != 1 {
            bail!("Chained comparisons not yet supported");
        }
        let op = convert_cmpop(&c.ops[0])?;
        let left = Box::new(Self::convert(*c.left)?);
        let right = Box::new(Self::convert(c.comparators.into_iter().next().unwrap())?);
        Ok(HirExpr::Binary { op, left, right })
    }
}

fn convert_expr(expr: ast::Expr) -> Result<HirExpr> {
    ExprConverter::convert(expr)
}

fn convert_binop(op: &ast::Operator) -> Result<BinOp> {
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

fn convert_aug_op(op: &ast::Operator) -> Result<BinOp> {
    // Augmented assignment operators map to the same binary operators
    convert_binop(op)
}

fn convert_unaryop(op: &ast::UnaryOp) -> Result<UnaryOp> {
    Ok(match op {
        ast::UnaryOp::Not => UnaryOp::Not,
        ast::UnaryOp::UAdd => UnaryOp::Pos,
        ast::UnaryOp::USub => UnaryOp::Neg,
        ast::UnaryOp::Invert => UnaryOp::BitNot,
    })
}

fn convert_cmpop(op: &ast::CmpOp) -> Result<BinOp> {
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

fn analyze_function_properties(body: &[HirStmt]) -> FunctionProperties {
    FunctionProperties {
        is_pure: check_pure(body),
        always_terminates: check_termination(body),
        panic_free: check_panic_free(body),
        max_stack_depth: calculate_max_stack_depth(body),
    }
}

fn check_pure(body: &[HirStmt]) -> bool {
    // V1: Conservative - only if no calls to unknown functions
    for stmt in body {
        if has_side_effects(stmt) {
            return false;
        }
    }
    true
}

fn has_side_effects(stmt: &HirStmt) -> bool {
    match stmt {
        HirStmt::Expr(HirExpr::Call { func, .. }) => {
            // Whitelist of pure functions
            !matches!(func.as_str(), "len" | "max" | "min" | "sum" | "abs")
        }
        HirStmt::If {
            then_body,
            else_body,
            ..
        } => {
            then_body.iter().any(has_side_effects)
                || else_body
                    .as_ref()
                    .is_some_and(|b| b.iter().any(has_side_effects))
        }
        HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
            body.iter().any(has_side_effects)
        }
        _ => false,
    }
}

fn check_termination(body: &[HirStmt]) -> bool {
    // V1: Only guarantee for simple cases
    for stmt in body {
        if let HirStmt::While { .. } = stmt {
            return false; // Can't guarantee termination with while loops
        }
        if let HirStmt::For { iter, .. } = stmt {
            // Only guarantee for finite iterators
            if !is_finite_iterator(iter) {
                return false;
            }
        }
    }
    true
}

fn is_finite_iterator(expr: &HirExpr) -> bool {
    match expr {
        HirExpr::List(_) | HirExpr::Tuple(_) | HirExpr::Dict(_) => true,
        HirExpr::Call { func, .. } => {
            matches!(func.as_str(), "range" | "enumerate" | "zip")
        }
        _ => false,
    }
}

fn check_panic_free(body: &[HirStmt]) -> bool {
    // V1: Check for obvious panic cases
    for stmt in body {
        if has_panic_risk(stmt) {
            return false;
        }
    }
    true
}

fn has_panic_risk(stmt: &HirStmt) -> bool {
    match stmt {
        HirStmt::Expr(expr) | HirStmt::Assign { value: expr, .. } => expr_has_panic_risk(expr),
        HirStmt::Return(Some(expr)) => expr_has_panic_risk(expr),
        HirStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            expr_has_panic_risk(condition)
                || then_body.iter().any(has_panic_risk)
                || else_body
                    .as_ref()
                    .is_some_and(|b| b.iter().any(has_panic_risk))
        }
        HirStmt::While { condition, body } => {
            expr_has_panic_risk(condition) || body.iter().any(has_panic_risk)
        }
        HirStmt::For { iter, body, .. } => {
            expr_has_panic_risk(iter) || body.iter().any(has_panic_risk)
        }
        _ => false,
    }
}

fn expr_has_panic_risk(expr: &HirExpr) -> bool {
    match expr {
        HirExpr::Index { .. } => true, // Array bounds
        HirExpr::Binary {
            op: BinOp::Div | BinOp::FloorDiv | BinOp::Mod,
            ..
        } => true, // Division by zero
        HirExpr::Binary { left, right, .. } => {
            expr_has_panic_risk(left) || expr_has_panic_risk(right)
        }
        HirExpr::Call { args, .. } => args.iter().any(expr_has_panic_risk),
        _ => false,
    }
}

fn calculate_max_stack_depth(body: &[HirStmt]) -> Option<usize> {
    // Simple estimation for V1
    Some(estimate_stack_depth(body, 0))
}

fn estimate_stack_depth(body: &[HirStmt], current: usize) -> usize {
    body.iter().fold(current, |max_depth, stmt| {
        let stmt_depth = match stmt {
            HirStmt::If {
                then_body,
                else_body,
                ..
            } => {
                let then_depth = estimate_stack_depth(then_body, current + 1);
                let else_depth = else_body
                    .as_ref()
                    .map(|b| estimate_stack_depth(b, current + 1))
                    .unwrap_or(current);
                then_depth.max(else_depth)
            }
            HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
                estimate_stack_depth(body, current + 1)
            }
            _ => current,
        };
        max_depth.max(stmt_depth)
    })
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
        python_to_hir(ast).unwrap()
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
}
