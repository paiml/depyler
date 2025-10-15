//! Code formatter for Ruchy script output

use crate::ast::{
    BinaryOp, DataFrameColumn, Literal, MatchArm, Param, Pattern, PipelineStage, RuchyExpr,
    RuchyType, StringPart, StructField, UnaryOp,
};

/// Ruchy code formatter with configurable style
pub struct RuchyFormatter {
    /// Indentation width
    indent_width: usize,

    /// Maximum line length
    max_line_length: usize,

    /// Current indentation level
    current_indent: usize,

    /// Output buffer
    output: String,
}

impl RuchyFormatter {
    /// Creates a new formatter with default settings
    #[must_use]
    pub fn new() -> Self {
        Self {
            indent_width: 4,
            max_line_length: 100,
            current_indent: 0,
            output: String::new(),
        }
    }

    /// Creates a formatter with custom configuration
    #[must_use]
    pub fn with_config(config: &crate::RuchyConfig) -> Self {
        Self {
            indent_width: config.indent_width,
            max_line_length: config.max_line_length,
            current_indent: 0,
            output: String::new(),
        }
    }

    /// Formats a Ruchy AST into a string
    pub fn format(&self, expr: &RuchyExpr) -> String {
        let mut formatter = Self {
            indent_width: self.indent_width,
            max_line_length: self.max_line_length,
            current_indent: 0,
            output: String::new(),
        };

        formatter.format_expr(expr, false);
        formatter.output
    }

    /// Format an expression
    fn format_expr(&mut self, expr: &RuchyExpr, needs_semicolon: bool) {
        match expr {
            RuchyExpr::Literal(lit) => self.format_literal(lit),

            RuchyExpr::Identifier(name) => {
                self.write(name);
            }

            RuchyExpr::Binary { left, op, right } => {
                self.format_expr(left, false);
                self.write(&format!(" {} ", self.format_binary_op(*op)));
                self.format_expr(right, false);
            }

            RuchyExpr::Unary { op, operand } => {
                self.write(self.format_unary_op(*op));
                self.format_expr(operand, false);
            }

            RuchyExpr::Function {
                name,
                params,
                body,
                is_async,
                return_type,
            } => {
                self.format_function(name, params, body, *is_async, return_type.as_ref());
            }

            RuchyExpr::Lambda { params, body } => {
                self.format_lambda(params, body);
            }

            RuchyExpr::Call { func, args } => {
                self.format_expr(func, false);
                self.write("(");
                self.format_comma_separated(args, |f, arg| f.format_expr(arg, false));
                self.write(")");
            }

            RuchyExpr::MethodCall {
                receiver,
                method,
                args,
            } => {
                self.format_expr(receiver, false);
                self.write(&format!(".{method}("));
                self.format_comma_separated(args, |f, arg| f.format_expr(arg, false));
                self.write(")");
            }

            RuchyExpr::Pipeline { expr, stages } => {
                self.format_pipeline(expr, stages);
            }

            RuchyExpr::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.format_if(condition, then_branch, else_branch.as_deref());
            }

            RuchyExpr::Match { expr, arms } => {
                self.format_match(expr, arms);
            }

            RuchyExpr::For { var, iter, body } => {
                self.write(&format!("for {var} in "));
                self.format_expr(iter, false);
                self.write(" ");
                self.format_block_or_expr(body);
            }

            RuchyExpr::While { condition, body } => {
                self.write("while ");
                self.format_expr(condition, false);
                self.write(" ");
                self.format_block_or_expr(body);
            }

            RuchyExpr::Block(exprs) => {
                self.format_block(exprs);
            }

            RuchyExpr::Let {
                name,
                value,
                body,
                is_mutable,
            } => {
                if *is_mutable {
                    self.write(&format!("let mut {name} = "));
                } else {
                    self.write(&format!("let {name} = "));
                }
                self.format_expr(value, false);
                self.writeln("");
                self.format_expr(body, false);
            }

            RuchyExpr::List(elements) => {
                self.write("[");
                self.format_comma_separated(elements, |f, e| f.format_expr(e, false));
                self.write("]");
            }

            RuchyExpr::StringInterpolation { parts } => {
                self.format_string_interpolation(parts);
            }

            RuchyExpr::Struct { name, fields } => {
                self.format_struct(name, fields);
            }

            RuchyExpr::StructLiteral { name, fields } => {
                self.write(&format!("{name} {{ "));
                self.format_comma_separated(fields, |f, (field_name, field_expr)| {
                    f.write(&format!("{field_name}: "));
                    f.format_expr(field_expr, false);
                });
                self.write(" }");
            }

            RuchyExpr::FieldAccess { object, field } => {
                self.format_expr(object, false);
                self.write(&format!(".{field}"));
            }

            RuchyExpr::Await { expr } => {
                self.format_expr(expr, false);
                self.write(".await");
            }

            RuchyExpr::Try { expr } => {
                self.format_expr(expr, false);
                self.write("?");
            }

            RuchyExpr::DataFrame { columns } => {
                self.format_dataframe(columns);
            }

            RuchyExpr::Range {
                start,
                end,
                inclusive,
            } => {
                self.format_expr(start, false);
                if *inclusive {
                    self.write("..=");
                } else {
                    self.write("..");
                }
                self.format_expr(end, false);
            }

            RuchyExpr::Break { label } => {
                if let Some(lbl) = label {
                    self.write(&format!("break '{lbl}"));
                } else {
                    self.write("break");
                }
            }

            RuchyExpr::Continue { label } => {
                if let Some(lbl) = label {
                    self.write(&format!("continue '{lbl}"));
                } else {
                    self.write("continue");
                }
            }

            RuchyExpr::Return { value } => {
                if let Some(val) = value {
                    self.write("return ");
                    self.format_expr(val, false);
                } else {
                    self.write("return");
                }
            }
        }

        if needs_semicolon {
            self.write(";");
        }
    }

    /// Format a literal
    fn format_literal(&mut self, lit: &Literal) {
        match lit {
            Literal::Integer(n) => self.write(&n.to_string()),
            Literal::Float(f) => {
                let s = f.to_string();
                if !s.contains('.') && !s.contains('e') {
                    self.write(&format!("{s}.0"));
                } else {
                    self.write(&s);
                }
            }
            Literal::String(s) => self.write(&format!("\"{s}\"")),
            Literal::Bool(b) => self.write(&b.to_string()),
            Literal::Char(c) => self.write(&format!("'{c}'")),
            Literal::Unit => self.write("()"),
        }
    }

    /// Format a function definition
    fn format_function(
        &mut self,
        name: &str,
        params: &[Param],
        body: &RuchyExpr,
        is_async: bool,
        return_type: Option<&RuchyType>,
    ) {
        if is_async {
            self.write("async ");
        }
        self.write(&format!("fun {name}("));
        self.format_params(params);
        self.write(")");

        if let Some(ret_type) = return_type {
            self.write(" -> ");
            self.format_type(ret_type);
        }

        self.write(" ");
        self.format_block_or_expr(body);
    }

    /// Format function parameters
    fn format_params(&mut self, params: &[Param]) {
        self.format_comma_separated(params, |f, param| {
            f.write(&param.name);
            if let Some(typ) = &param.typ {
                f.write(": ");
                f.format_type(typ);
            }
            if let Some(default) = &param.default {
                f.write(" = ");
                f.format_expr(default, false);
            }
        });
    }

    /// Format a lambda expression
    fn format_lambda(&mut self, params: &[Param], body: &RuchyExpr) {
        self.write("|");
        self.format_params(params);
        self.write("| ");

        // Single expression lambdas don't need braces
        if !matches!(body, RuchyExpr::Block(_)) {
            self.format_expr(body, false);
        } else {
            self.format_block_or_expr(body);
        }
    }

    /// Format a pipeline expression
    fn format_pipeline(&mut self, expr: &RuchyExpr, stages: &[PipelineStage]) {
        self.format_expr(expr, false);

        for stage in stages {
            if self.current_line_length() > self.max_line_length / 2 {
                self.writeln("");
                self.indent();
                self.write("|> ");
            } else {
                self.write(" |> ");
            }

            match stage {
                PipelineStage::Map(f) => {
                    self.write("map(");
                    self.format_expr(f, false);
                    self.write(")");
                }
                PipelineStage::Filter(f) => {
                    self.write("filter(");
                    self.format_expr(f, false);
                    self.write(")");
                }
                PipelineStage::FlatMap(f) => {
                    self.write("flat_map(");
                    self.format_expr(f, false);
                    self.write(")");
                }
                PipelineStage::Reduce(f) => {
                    self.write("reduce(");
                    self.format_expr(f, false);
                    self.write(")");
                }
                PipelineStage::Call(method, args) => {
                    self.write(&format!("{method}("));
                    self.format_comma_separated(args, |f, arg| f.format_expr(arg, false));
                    self.write(")");
                }
            }
        }
    }

    /// Format an if expression
    fn format_if(
        &mut self,
        condition: &RuchyExpr,
        then_branch: &RuchyExpr,
        else_branch: Option<&RuchyExpr>,
    ) {
        self.write("if ");
        self.format_expr(condition, false);
        self.write(" ");
        self.format_block_or_expr(then_branch);

        if let Some(else_expr) = else_branch {
            self.write(" else ");
            if matches!(else_expr, RuchyExpr::If { .. }) {
                self.format_expr(else_expr, false);
            } else {
                self.format_block_or_expr(else_expr);
            }
        }
    }

    /// Format a match expression
    fn format_match(&mut self, expr: &RuchyExpr, arms: &[MatchArm]) {
        self.write("match ");
        self.format_expr(expr, false);
        self.write(" {");
        self.increase_indent();

        for arm in arms {
            self.writeln("");
            self.indent();
            self.format_pattern(&arm.pattern);

            if let Some(guard) = &arm.guard {
                self.write(" if ");
                self.format_expr(guard, false);
            }

            self.write(" => ");
            self.format_expr(&arm.body, false);
            self.write(",");
        }

        self.decrease_indent();
        self.writeln("");
        self.indent();
        self.write("}");
    }

    /// Format a pattern
    fn format_pattern(&mut self, pattern: &Pattern) {
        match pattern {
            Pattern::Wildcard => self.write("_"),
            Pattern::Literal(lit) => self.format_literal(lit),
            Pattern::Identifier(name) => self.write(name),
            Pattern::Tuple(patterns) => {
                self.write("(");
                self.format_comma_separated(patterns, |f, p| f.format_pattern(p));
                self.write(")");
            }
            Pattern::Struct { name, fields } => {
                self.write(&format!("{name} {{ "));
                self.format_comma_separated(fields, |f, (field_name, field_pattern)| {
                    f.write(&format!("{field_name}: "));
                    f.format_pattern(field_pattern);
                });
                self.write(" }");
            }
            Pattern::List(patterns) => {
                self.write("[");
                self.format_comma_separated(patterns, |f, p| f.format_pattern(p));
                self.write("]");
            }
        }
    }

    /// Format a block of expressions
    fn format_block(&mut self, exprs: &[RuchyExpr]) {
        self.write("{");

        if !exprs.is_empty() {
            self.increase_indent();

            for (i, expr) in exprs.iter().enumerate() {
                self.writeln("");
                self.indent();

                // Last expression in block doesn't get semicolon unless it's a statement
                let needs_semi = i < exprs.len() - 1 || self.is_statement(expr);
                self.format_expr(expr, needs_semi);
            }

            self.decrease_indent();
            self.writeln("");
            self.indent();
        }

        self.write("}");
    }

    /// Format block or single expression
    fn format_block_or_expr(&mut self, expr: &RuchyExpr) {
        if matches!(expr, RuchyExpr::Block(_)) {
            self.format_expr(expr, false);
        } else {
            self.write("{");
            self.increase_indent();
            self.writeln("");
            self.indent();
            self.format_expr(expr, false);
            self.decrease_indent();
            self.writeln("");
            self.indent();
            self.write("}");
        }
    }

    /// Format string interpolation
    fn format_string_interpolation(&mut self, parts: &[StringPart]) {
        self.write("f\"");

        for part in parts {
            match part {
                StringPart::Text(text) => {
                    // Escape special characters
                    let escaped = text
                        .replace('\\', "\\\\")
                        .replace('"', "\\\"")
                        .replace('{', "{{")
                        .replace('}', "}}");
                    self.write(&escaped);
                }
                StringPart::Expr(expr) => {
                    self.write("{");
                    self.format_expr(expr, false);
                    self.write("}");
                }
            }
        }

        self.write("\"");
    }

    /// Format a struct definition
    fn format_struct(&mut self, name: &str, fields: &[StructField]) {
        self.write(&format!("struct {name} {{"));

        if !fields.is_empty() {
            self.increase_indent();

            for field in fields {
                self.writeln("");
                self.indent();

                if field.is_public {
                    self.write("pub ");
                }

                self.write(&format!("{}: ", field.name));
                self.format_type(&field.typ);
                self.write(",");
            }

            self.decrease_indent();
            self.writeln("");
            self.indent();
        }

        self.write("}");
    }

    /// Format a DataFrame literal
    fn format_dataframe(&mut self, columns: &[DataFrameColumn]) {
        self.write("df![");

        if !columns.is_empty() {
            self.increase_indent();

            for (i, col) in columns.iter().enumerate() {
                if i > 0 {
                    self.write(",");
                }
                self.writeln("");
                self.indent();
                self.write(&format!("\"{}\": [", col.name));

                self.format_comma_separated(&col.values, |f, val| f.format_expr(val, false));

                self.write("]");
            }

            self.decrease_indent();
            self.writeln("");
            self.indent();
        }

        self.write("]");
    }

    /// Format a type
    fn format_type(&mut self, typ: &RuchyType) {
        match typ {
            RuchyType::I8 => self.write("i8"),
            RuchyType::I16 => self.write("i16"),
            RuchyType::I32 => self.write("i32"),
            RuchyType::I64 => self.write("i64"),
            RuchyType::I128 => self.write("i128"),
            RuchyType::ISize => self.write("isize"),
            RuchyType::U8 => self.write("u8"),
            RuchyType::U16 => self.write("u16"),
            RuchyType::U32 => self.write("u32"),
            RuchyType::U64 => self.write("u64"),
            RuchyType::U128 => self.write("u128"),
            RuchyType::USize => self.write("usize"),
            RuchyType::F32 => self.write("f32"),
            RuchyType::F64 => self.write("f64"),
            RuchyType::Bool => self.write("bool"),
            RuchyType::Char => self.write("char"),
            RuchyType::String => self.write("String"),

            RuchyType::Vec(inner) => {
                self.write("Vec<");
                self.format_type(inner);
                self.write(">");
            }

            RuchyType::Array(inner, size) => {
                self.write("[");
                self.format_type(inner);
                self.write(&format!("; {size}]"));
            }

            RuchyType::Tuple(types) => {
                self.write("(");
                self.format_comma_separated(types, |f, t| f.format_type(t));
                self.write(")");
            }

            RuchyType::Option(inner) => {
                self.format_type(inner);
                self.write("?");
            }

            RuchyType::Result(ok, err) => {
                self.write("Result<");
                self.format_type(ok);
                self.write(", ");
                self.format_type(err);
                self.write(">");
            }

            RuchyType::Function { params, returns } => {
                self.write("fun(");
                self.format_comma_separated(params, |f, t| f.format_type(t));
                self.write(") -> ");
                self.format_type(returns);
            }

            RuchyType::Named(name) => self.write(name),

            RuchyType::Generic(name) => self.write(name),

            RuchyType::Reference { typ, is_mutable } => {
                self.write("&");
                if *is_mutable {
                    self.write("mut ");
                }
                self.format_type(typ);
            }

            RuchyType::Dynamic => self.write("dyn Any"),
        }
    }

    /// Format binary operator
    fn format_binary_op(&self, op: BinaryOp) -> &'static str {
        match op {
            BinaryOp::Add => "+",
            BinaryOp::Subtract => "-",
            BinaryOp::Multiply => "*",
            BinaryOp::Divide => "/",
            BinaryOp::Modulo => "%",
            BinaryOp::Power => "**",
            BinaryOp::Equal => "==",
            BinaryOp::NotEqual => "!=",
            BinaryOp::Less => "<",
            BinaryOp::LessEqual => "<=",
            BinaryOp::Greater => ">",
            BinaryOp::GreaterEqual => ">=",
            BinaryOp::And => "&&",
            BinaryOp::Or => "||",
            BinaryOp::BitwiseAnd => "&",
            BinaryOp::BitwiseOr => "|",
            BinaryOp::BitwiseXor => "^",
            BinaryOp::LeftShift => "<<",
            BinaryOp::RightShift => ">>",
        }
    }

    /// Format unary operator
    fn format_unary_op(&self, op: UnaryOp) -> &'static str {
        match op {
            UnaryOp::Not => "!",
            UnaryOp::Negate => "-",
            UnaryOp::BitwiseNot => "!",
        }
    }

    /// Check if expression is a statement (needs semicolon)
    fn is_statement(&self, expr: &RuchyExpr) -> bool {
        matches!(
            expr,
            RuchyExpr::Let { .. }
                | RuchyExpr::Break { .. }
                | RuchyExpr::Continue { .. }
                | RuchyExpr::Return { .. }
        )
    }

    /// Format comma-separated items
    fn format_comma_separated<T, F>(&mut self, items: &[T], mut format_fn: F)
    where
        F: FnMut(&mut Self, &T),
    {
        for (i, item) in items.iter().enumerate() {
            if i > 0 {
                self.write(", ");
            }
            format_fn(self, item);
        }
    }

    /// Write string to output
    fn write(&mut self, s: &str) {
        self.output.push_str(s);
    }

    /// Write string with newline
    fn writeln(&mut self, s: &str) {
        self.output.push_str(s);
        self.output.push('\n');
    }

    /// Write current indentation
    fn indent(&mut self) {
        for _ in 0..self.current_indent {
            self.output.push(' ');
        }
    }

    /// Increase indentation level
    fn increase_indent(&mut self) {
        self.current_indent += self.indent_width;
    }

    /// Decrease indentation level
    fn decrease_indent(&mut self) {
        self.current_indent = self.current_indent.saturating_sub(self.indent_width);
    }

    /// Get current line length
    fn current_line_length(&self) -> usize {
        self.output.lines().last().map_or(0, |line| line.len())
    }
}

impl Default for RuchyFormatter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_literal() {
        let formatter = RuchyFormatter::new();

        let expr = RuchyExpr::Literal(Literal::Integer(42));
        assert_eq!(formatter.format(&expr), "42");

        let expr = RuchyExpr::Literal(Literal::String("hello".to_string()));
        assert_eq!(formatter.format(&expr), "\"hello\"");

        let expr = RuchyExpr::Literal(Literal::Bool(true));
        assert_eq!(formatter.format(&expr), "true");
    }

    #[test]
    fn test_format_function() {
        let formatter = RuchyFormatter::new();

        let expr = RuchyExpr::Function {
            name: "add".to_string(),
            params: vec![
                Param {
                    name: "x".to_string(),
                    typ: Some(RuchyType::I64),
                    default: None,
                },
                Param {
                    name: "y".to_string(),
                    typ: Some(RuchyType::I64),
                    default: None,
                },
            ],
            body: Box::new(RuchyExpr::Binary {
                left: Box::new(RuchyExpr::Identifier("x".to_string())),
                op: BinaryOp::Add,
                right: Box::new(RuchyExpr::Identifier("y".to_string())),
            }),
            is_async: false,
            return_type: Some(RuchyType::I64),
        };

        let formatted = formatter.format(&expr);
        assert!(formatted.contains("fun add(x: i64, y: i64) -> i64"));
        assert!(formatted.contains("x + y"));
    }

    #[test]
    fn test_format_pipeline() {
        let formatter = RuchyFormatter::new();

        let expr = RuchyExpr::Pipeline {
            expr: Box::new(RuchyExpr::List(vec![
                RuchyExpr::Literal(Literal::Integer(1)),
                RuchyExpr::Literal(Literal::Integer(2)),
                RuchyExpr::Literal(Literal::Integer(3)),
            ])),
            stages: vec![
                PipelineStage::Filter(Box::new(RuchyExpr::Lambda {
                    params: vec![Param {
                        name: "x".to_string(),
                        typ: None,
                        default: None,
                    }],
                    body: Box::new(RuchyExpr::Binary {
                        left: Box::new(RuchyExpr::Identifier("x".to_string())),
                        op: BinaryOp::Greater,
                        right: Box::new(RuchyExpr::Literal(Literal::Integer(1))),
                    }),
                })),
                PipelineStage::Map(Box::new(RuchyExpr::Lambda {
                    params: vec![Param {
                        name: "x".to_string(),
                        typ: None,
                        default: None,
                    }],
                    body: Box::new(RuchyExpr::Binary {
                        left: Box::new(RuchyExpr::Identifier("x".to_string())),
                        op: BinaryOp::Multiply,
                        right: Box::new(RuchyExpr::Literal(Literal::Integer(2))),
                    }),
                })),
            ],
        };

        let formatted = formatter.format(&expr);
        assert!(formatted.contains("[1, 2, 3]"));
        assert!(formatted.contains("|> filter(|x| x > 1)"));
        assert!(formatted.contains("|> map(|x| x * 2)"));
    }
}
