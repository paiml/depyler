//! Ruchy transpiler for immediate execution
//!
//! Converts Depyler HIR to Ruchy source code for interpreter execution

use crate::hir::{
    AssignTarget, BinOp, HirClass, HirExpr, HirFunction, HirModule, HirStmt, Literal, Type,
    UnaryOp,
};
use anyhow::{anyhow, Result};
use std::fmt::Write;

/// Transpiler from HIR to Ruchy source code
pub struct RuchyTranspiler {
    /// Current indentation level
    indent: usize,
    /// Output buffer
    output: String,
}

impl RuchyTranspiler {
    /// Create a new Ruchy transpiler
    pub fn new() -> Self {
        Self {
            indent: 0,
            output: String::new(),
        }
    }

    /// Transpile HIR module to Ruchy source
    pub fn transpile(&mut self, module: &HirModule) -> Result<String> {
        self.output.clear();
        self.indent = 0;

        // Transpile imports
        for import in &module.imports {
            self.transpile_import(&import.module)?;
        }

        if !module.imports.is_empty() {
            writeln!(self.output)?;
        }

        // Transpile functions
        for func in &module.functions {
            self.transpile_function(func)?;
            writeln!(self.output)?;
        }

        // Transpile classes
        for class in &module.classes {
            self.transpile_class(class)?;
            writeln!(self.output)?;
        }

        Ok(self.output.clone())
    }

    fn transpile_import(&mut self, import: &str) -> Result<()> {
        // Map Python imports to Ruchy imports
        let ruchy_import = match import {
            "typing" => return Ok(()), // Skip typing imports
            "collections" => "std.collections",
            "itertools" => "std.iter",
            "functools" => "std.func",
            "math" => "std.math",
            other => other,
        };

        writeln!(self.output, "import {}", ruchy_import)?;
        Ok(())
    }

    fn transpile_function(&mut self, func: &HirFunction) -> Result<()> {
        write!(self.output, "fun {}(", func.name)?;

        // Parameters
        for (i, (name, ty)) in func.params.iter().enumerate() {
            if i > 0 {
                write!(self.output, ", ")?;
            }
            write!(self.output, "{}: {}", name, self.transpile_type(ty)?)?;
        }

        writeln!(
            self.output,
            ") -> {} {{",
            self.transpile_type(&func.ret_type)?
        )?;

        // Function body
        self.indent += 1;
        for stmt in &func.body {
            self.write_indent()?;
            self.transpile_statement(stmt)?;
        }
        self.indent -= 1;

        writeln!(self.output, "}}")?;
        Ok(())
    }

    fn transpile_class(&mut self, class: &HirClass) -> Result<()> {
        writeln!(self.output, "struct {} {{", class.name)?;
        
        self.indent += 1;
        for field in &class.fields {
            self.write_indent()?;
            writeln!(
                self.output,
                "{}: {},",
                field.name,
                self.transpile_type(&field.field_type)?
            )?;
        }
        self.indent -= 1;
        
        writeln!(self.output, "}}")?;
        
        // Methods as impl block
        if !class.methods.is_empty() {
            writeln!(self.output)?;
            writeln!(self.output, "impl {} {{", class.name)?;
            
            self.indent += 1;
            for method in &class.methods {
                self.write_indent()?;
                self.transpile_method(method)?;
            }
            self.indent -= 1;
            
            writeln!(self.output, "}}")?;
        }
        
        Ok(())
    }

    fn transpile_method(&mut self, method: &crate::hir::HirMethod) -> Result<()> {
        write!(self.output, "fun {}(", method.name)?;

        // Parameters (skip self for now)
        let params_iter = method.params.iter().skip_while(|(name, _)| name == "self");
        for (i, (name, ty)) in params_iter.enumerate() {
            if i > 0 {
                write!(self.output, ", ")?;
            }
            write!(self.output, "{}: {}", name, self.transpile_type(ty)?)?;
        }

        writeln!(
            self.output,
            ") -> {} {{",
            self.transpile_type(&method.ret_type)?
        )?;

        // Method body
        self.indent += 1;
        for stmt in &method.body {
            self.write_indent()?;
            self.transpile_statement(stmt)?;
        }
        self.indent -= 1;

        self.write_indent()?;
        writeln!(self.output, "}}")?;
        Ok(())
    }

    fn transpile_statement(&mut self, stmt: &HirStmt) -> Result<()> {
        match stmt {
            HirStmt::Assign { target, value } => {
                let target_str = self.transpile_assign_target(target)?;
                let value_str = self.transpile_expr(value)?;
                writeln!(self.output, "{} = {}", target_str, value_str)?;
            }
            HirStmt::Return(value) => {
                if let Some(val) = value {
                    let val_str = self.transpile_expr(val)?;
                    writeln!(self.output, "return {}", val_str)?;
                } else {
                    writeln!(self.output, "return")?;
                }
            }
            HirStmt::Expr(expr) => {
                let expr_str = self.transpile_expr(expr)?;
                writeln!(self.output, "{}", expr_str)?;
            }
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                let cond_str = self.transpile_expr(condition)?;
                writeln!(self.output, "if {} {{", cond_str)?;
                self.indent += 1;
                for s in then_body {
                    self.write_indent()?;
                    self.transpile_statement(s)?;
                }
                self.indent -= 1;
                if let Some(else_b) = else_body {
                    self.write_indent()?;
                    writeln!(self.output, "}} else {{")?;
                    self.indent += 1;
                    for s in else_b {
                        self.write_indent()?;
                        self.transpile_statement(s)?;
                    }
                    self.indent -= 1;
                }
                self.write_indent()?;
                writeln!(self.output, "}}")?;
            }
            HirStmt::While { condition, body } => {
                let cond_str = self.transpile_expr(condition)?;
                writeln!(self.output, "while {} {{", cond_str)?;
                self.indent += 1;
                for s in body {
                    self.write_indent()?;
                    self.transpile_statement(s)?;
                }
                self.indent -= 1;
                self.write_indent()?;
                writeln!(self.output, "}}")?;
            }
            HirStmt::For { target, iter, body } => {
                let iter_str = self.transpile_expr(iter)?;
                writeln!(self.output, "for {} in {} {{", target, iter_str)?;
                self.indent += 1;
                for s in body {
                    self.write_indent()?;
                    self.transpile_statement(s)?;
                }
                self.indent -= 1;
                self.write_indent()?;
                writeln!(self.output, "}}")?;
            }
            HirStmt::Break { .. } => writeln!(self.output, "break")?,
            HirStmt::Continue { .. } => writeln!(self.output, "continue")?,
            HirStmt::Raise { .. } => writeln!(self.output, "// raise not supported")?,
            HirStmt::With { .. } => writeln!(self.output, "// with not supported")?,
        }
        Ok(())
    }

    fn transpile_assign_target(&mut self, target: &AssignTarget) -> Result<String> {
        Ok(match target {
            AssignTarget::Symbol(name) => name.clone(),
            AssignTarget::Index { base, index } => {
                format!(
                    "{}[{}]",
                    self.transpile_expr(base)?,
                    self.transpile_expr(index)?
                )
            }
            AssignTarget::Attribute { value, attr } => {
                format!("{}.{}", self.transpile_expr(value)?, attr)
            }
        })
    }

    fn transpile_expr(&mut self, expr: &HirExpr) -> Result<String> {
        Ok(match expr {
            HirExpr::Literal(lit) => self.transpile_literal(lit),
            HirExpr::Var(name) => name.clone(),
            HirExpr::Binary { left, op, right } => {
                let left_str = self.transpile_expr(left)?;
                let op_str = self.transpile_binary_op(op);
                let right_str = self.transpile_expr(right)?;
                format!("({} {} {})", left_str, op_str, right_str)
            }
            HirExpr::Unary { op, operand } => {
                let op_str = self.transpile_unary_op(op);
                let operand_str = self.transpile_expr(operand)?;
                format!("({}{})", op_str, operand_str)
            }
            HirExpr::Call { func, args } => {
                let args_str = args
                    .iter()
                    .map(|arg| self.transpile_expr(arg))
                    .collect::<Result<Vec<_>>>()?
                    .join(", ");
                format!("{}({})", func, args_str)
            }
            HirExpr::MethodCall {
                object,
                method,
                args,
            } => {
                let object_str = self.transpile_expr(object)?;
                let args_str = args
                    .iter()
                    .map(|arg| self.transpile_expr(arg))
                    .collect::<Result<Vec<_>>>()?
                    .join(", ");
                format!("{}.{}({})", object_str, method, args_str)
            }
            HirExpr::List(elements) => {
                let elements_str = elements
                    .iter()
                    .map(|elem| self.transpile_expr(elem))
                    .collect::<Result<Vec<_>>>()?
                    .join(", ");
                format!("[{}]", elements_str)
            }
            HirExpr::Dict(pairs) => {
                let pairs_str = pairs
                    .iter()
                    .map(|(k, v)| {
                        Ok(format!(
                            "{}: {}",
                            self.transpile_expr(k)?,
                            self.transpile_expr(v)?
                        ))
                    })
                    .collect::<Result<Vec<_>>>()?
                    .join(", ");
                format!("{{ {} }}", pairs_str)
            }
            HirExpr::Set(elements) => {
                let elements_str = elements
                    .iter()
                    .map(|elem| self.transpile_expr(elem))
                    .collect::<Result<Vec<_>>>()?
                    .join(", ");
                format!("set![{}]", elements_str)
            }
            HirExpr::Tuple(elements) => {
                let elements_str = elements
                    .iter()
                    .map(|elem| self.transpile_expr(elem))
                    .collect::<Result<Vec<_>>>()?
                    .join(", ");
                format!("({})", elements_str)
            }
            HirExpr::Index { base, index } => {
                let base_str = self.transpile_expr(base)?;
                let index_str = self.transpile_expr(index)?;
                format!("{}[{}]", base_str, index_str)
            }
            HirExpr::Slice {
                base,
                start,
                stop,
                step,
            } => {
                let base_str = self.transpile_expr(base)?;
                let start_str = start
                    .as_ref()
                    .map(|s| self.transpile_expr(s))
                    .transpose()?
                    .unwrap_or_default();
                let stop_str = stop
                    .as_ref()
                    .map(|e| self.transpile_expr(e))
                    .transpose()?
                    .unwrap_or_default();
                if step.is_some() {
                    return Err(anyhow!("Step slicing not supported in Ruchy"));
                }
                format!("{}[{}..{}]", base_str, start_str, stop_str)
            }
            HirExpr::Attribute { value, attr } => {
                let value_str = self.transpile_expr(value)?;
                format!("{}.{}", value_str, attr)
            }
            HirExpr::ListComp {
                element,
                target,
                iter,
                condition,
            } => {
                let elem_str = self.transpile_expr(element)?;
                let iter_str = self.transpile_expr(iter)?;
                if let Some(cond) = condition {
                    let cond_str = self.transpile_expr(cond)?;
                    format!("[{} for {} in {} if {}]", elem_str, target, iter_str, cond_str)
                } else {
                    format!("[{} for {} in {}]", elem_str, target, iter_str)
                }
            }
            HirExpr::SetComp { .. } => return Err(anyhow!("Set comprehensions not yet supported")),
            HirExpr::FrozenSet(_) => return Err(anyhow!("Frozen sets not yet supported")),
            HirExpr::Borrow { expr, .. } => self.transpile_expr(expr)?,
            HirExpr::Lambda { params, body } => {
                let params_str = params.join(", ");
                let body_str = self.transpile_expr(body)?;
                format!("|{}| {}", params_str, body_str)
            }
            HirExpr::Await { .. } => return Err(anyhow!("Await expressions not yet supported")),
        })
    }

    fn transpile_literal(&self, lit: &Literal) -> String {
        match lit {
            Literal::Int(i) => i.to_string(),
            Literal::Float(f) => f.to_string(),
            Literal::String(s) => format!("\"{}\"", s.escape_default()),
            Literal::Bool(b) => if *b { "true" } else { "false" }.to_string(),
            Literal::None => "()".to_string(),
        }
    }

    fn transpile_type(&self, ty: &Type) -> Result<String> {
        Ok(match ty {
            Type::Int => "i64".to_string(),
            Type::Float => "f64".to_string(),
            Type::String => "String".to_string(),
            Type::Bool => "bool".to_string(),
            Type::None => "()".to_string(),
            Type::List(inner) => format!("Vec<{}>", self.transpile_type(inner)?),
            Type::Dict(k, v) => {
                format!("HashMap<{}, {}>", self.transpile_type(k)?, self.transpile_type(v)?)
            }
            Type::Set(inner) => format!("HashSet<{}>", self.transpile_type(inner)?),
            Type::Tuple(types) => {
                let types_str = types
                    .iter()
                    .map(|t| self.transpile_type(t))
                    .collect::<Result<Vec<_>>>()?
                    .join(", ");
                format!("({})", types_str)
            }
            Type::Optional(inner) => format!("Option<{}>", self.transpile_type(inner)?),
            Type::Function { params, ret } => {
                let params_str = params
                    .iter()
                    .map(|t| self.transpile_type(t))
                    .collect::<Result<Vec<_>>>()?
                    .join(", ");
                let ret_str = self.transpile_type(ret)?;
                format!("fun({}) -> {}", params_str, ret_str)
            }
            Type::Custom(name) => name.clone(),
            Type::Unknown => "_".to_string(),
            Type::Union(types) => {
                // For now, just use the first type
                if types.is_empty() {
                    "_".to_string()
                } else {
                    self.transpile_type(&types[0])?
                }
            }
            Type::TypeVar(name) => name.clone(),
            Type::Generic { base, params } => {
                let params_str = params
                    .iter()
                    .map(|t| self.transpile_type(t))
                    .collect::<Result<Vec<_>>>()?
                    .join(", ");
                format!("{}<{}>", base, params_str)
            }
            Type::Array { element_type, .. } => {
                format!("Vec<{}>", self.transpile_type(element_type)?)
            }
        })
    }

    fn transpile_binary_op(&self, op: &BinOp) -> &'static str {
        match op {
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Mul => "*",
            BinOp::Div => "/",
            BinOp::FloorDiv => "//",
            BinOp::Mod => "%",
            BinOp::Pow => "**",
            BinOp::Lt => "<",
            BinOp::LtEq => "<=",
            BinOp::Gt => ">",
            BinOp::GtEq => ">=",
            BinOp::Eq => "==",
            BinOp::NotEq => "!=",
            BinOp::And => "&&",
            BinOp::Or => "||",
            BinOp::BitAnd => "&",
            BinOp::BitOr => "|",
            BinOp::BitXor => "^",
            BinOp::LShift => "<<",
            BinOp::RShift => ">>",
            BinOp::In => "in",
            BinOp::NotIn => "not in",
        }
    }

    fn transpile_unary_op(&self, op: &UnaryOp) -> &'static str {
        match op {
            UnaryOp::Not => "!",
            UnaryOp::Neg => "-",
            UnaryOp::Pos => "+",
            UnaryOp::BitNot => "~",
        }
    }

    fn write_indent(&mut self) -> Result<()> {
        for _ in 0..self.indent {
            write!(self.output, "    ")?;
        }
        Ok(())
    }
}

impl Default for RuchyTranspiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smallvec::smallvec;

    #[test]
    fn test_transpile_function() {
        let mut transpiler = RuchyTranspiler::new();
        let func = HirFunction {
            name: "add".to_string(),
            params: smallvec![
                ("x".to_string(), Type::Int),
                ("y".to_string(), Type::Int)
            ],
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Binary {
                left: Box::new(HirExpr::Var("x".to_string())),
                op: BinOp::Add,
                right: Box::new(HirExpr::Var("y".to_string())),
            }))],
            properties: Default::default(),
            annotations: Default::default(),
            docstring: None,
        };

        let module = HirModule {
            functions: vec![func],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            classes: vec![],
        };

        let output = transpiler.transpile(&module).unwrap();

        assert!(output.contains("fun add(x: i64, y: i64) -> i64"));
        assert!(output.contains("return (x + y)"));
    }

    #[test]
    fn test_transpile_list_comprehension() {
        let mut transpiler = RuchyTranspiler::new();
        let expr = HirExpr::ListComp {
            element: Box::new(HirExpr::Binary {
                left: Box::new(HirExpr::Var("x".to_string())),
                op: BinOp::Mul,
                right: Box::new(HirExpr::Literal(Literal::Int(2))),
            }),
            target: "x".to_string(),
            iter: Box::new(HirExpr::List(vec![
                HirExpr::Literal(Literal::Int(1)),
                HirExpr::Literal(Literal::Int(2)),
            ])),
            condition: None,
        };

        let result = transpiler.transpile_expr(&expr).unwrap();
        assert_eq!(result, "[(x * 2) for x in [1, 2]]");
    }
}