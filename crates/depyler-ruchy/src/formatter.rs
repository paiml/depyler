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

    #[test]
    fn test_format_identifier() {
        let formatter = RuchyFormatter::new();
        let expr = RuchyExpr::Identifier("my_variable".to_string());
        assert_eq!(formatter.format(&expr), "my_variable");
    }

    #[test]
    fn test_format_binary_operators() {
        let formatter = RuchyFormatter::new();

        // Test addition
        let add_expr = RuchyExpr::Binary {
            left: Box::new(RuchyExpr::Identifier("a".to_string())),
            op: BinaryOp::Add,
            right: Box::new(RuchyExpr::Identifier("b".to_string())),
        };
        assert_eq!(formatter.format(&add_expr), "a + b");

        // Test subtraction
        let sub_expr = RuchyExpr::Binary {
            left: Box::new(RuchyExpr::Identifier("a".to_string())),
            op: BinaryOp::Subtract,
            right: Box::new(RuchyExpr::Identifier("b".to_string())),
        };
        assert_eq!(formatter.format(&sub_expr), "a - b");

        // Test multiplication
        let mul_expr = RuchyExpr::Binary {
            left: Box::new(RuchyExpr::Identifier("a".to_string())),
            op: BinaryOp::Multiply,
            right: Box::new(RuchyExpr::Identifier("b".to_string())),
        };
        assert_eq!(formatter.format(&mul_expr), "a * b");

        // Test division
        let div_expr = RuchyExpr::Binary {
            left: Box::new(RuchyExpr::Identifier("a".to_string())),
            op: BinaryOp::Divide,
            right: Box::new(RuchyExpr::Identifier("b".to_string())),
        };
        assert_eq!(formatter.format(&div_expr), "a / b");
    }

    #[test]
    fn test_format_comparison_operators() {
        let formatter = RuchyFormatter::new();

        let eq_expr = RuchyExpr::Binary {
            left: Box::new(RuchyExpr::Identifier("a".to_string())),
            op: BinaryOp::Equal,
            right: Box::new(RuchyExpr::Identifier("b".to_string())),
        };
        assert_eq!(formatter.format(&eq_expr), "a == b");

        let ne_expr = RuchyExpr::Binary {
            left: Box::new(RuchyExpr::Identifier("a".to_string())),
            op: BinaryOp::NotEqual,
            right: Box::new(RuchyExpr::Identifier("b".to_string())),
        };
        assert_eq!(formatter.format(&ne_expr), "a != b");
    }

    #[test]
    fn test_format_unary_operators() {
        let formatter = RuchyFormatter::new();

        let not_expr = RuchyExpr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(RuchyExpr::Identifier("flag".to_string())),
        };
        assert_eq!(formatter.format(&not_expr), "!flag");

        let neg_expr = RuchyExpr::Unary {
            op: UnaryOp::Negate,
            operand: Box::new(RuchyExpr::Literal(Literal::Integer(5))),
        };
        assert_eq!(formatter.format(&neg_expr), "-5");
    }

    #[test]
    fn test_format_call() {
        let formatter = RuchyFormatter::new();

        let call_expr = RuchyExpr::Call {
            func: Box::new(RuchyExpr::Identifier("print".to_string())),
            args: vec![RuchyExpr::Literal(Literal::String("hello".to_string()))],
        };
        assert_eq!(formatter.format(&call_expr), "print(\"hello\")");
    }

    #[test]
    fn test_format_list() {
        let formatter = RuchyFormatter::new();

        let list_expr = RuchyExpr::List(vec![
            RuchyExpr::Literal(Literal::Integer(1)),
            RuchyExpr::Literal(Literal::Integer(2)),
            RuchyExpr::Literal(Literal::Integer(3)),
        ]);
        assert_eq!(formatter.format(&list_expr), "[1, 2, 3]");
    }

    #[test]
    fn test_format_if_else() {
        let formatter = RuchyFormatter::new();

        let if_expr = RuchyExpr::If {
            condition: Box::new(RuchyExpr::Binary {
                left: Box::new(RuchyExpr::Identifier("x".to_string())),
                op: BinaryOp::Greater,
                right: Box::new(RuchyExpr::Literal(Literal::Integer(0))),
            }),
            then_branch: Box::new(RuchyExpr::Literal(Literal::String("positive".to_string()))),
            else_branch: Some(Box::new(RuchyExpr::Literal(Literal::String("non-positive".to_string())))),
        };

        let formatted = formatter.format(&if_expr);
        assert!(formatted.contains("if"));
        assert!(formatted.contains("else"));
    }

    #[test]
    fn test_format_lambda() {
        let formatter = RuchyFormatter::new();

        let lambda_expr = RuchyExpr::Lambda {
            params: vec![Param {
                name: "x".to_string(),
                typ: Some(RuchyType::I64),
                default: None,
            }],
            body: Box::new(RuchyExpr::Binary {
                left: Box::new(RuchyExpr::Identifier("x".to_string())),
                op: BinaryOp::Multiply,
                right: Box::new(RuchyExpr::Literal(Literal::Integer(2))),
            }),
        };

        let formatted = formatter.format(&lambda_expr);
        assert!(formatted.contains("|x: i64|"));
        assert!(formatted.contains("x * 2"));
    }

    #[test]
    fn test_format_async_function() {
        let formatter = RuchyFormatter::new();

        let async_fn = RuchyExpr::Function {
            name: "fetch_data".to_string(),
            params: vec![],
            body: Box::new(RuchyExpr::Literal(Literal::Unit)),
            is_async: true,
            return_type: None,
        };

        let formatted = formatter.format(&async_fn);
        assert!(formatted.contains("async fun fetch_data"));
    }

    #[test]
    fn test_format_block() {
        let formatter = RuchyFormatter::new();

        let block_expr = RuchyExpr::Block(vec![
            RuchyExpr::Identifier("x".to_string()),
            RuchyExpr::Identifier("y".to_string()),
        ]);

        let formatted = formatter.format(&block_expr);
        assert!(formatted.contains("{"));
        assert!(formatted.contains("}"));
    }

    #[test]
    fn test_format_range() {
        let formatter = RuchyFormatter::new();

        // Exclusive range
        let range_expr = RuchyExpr::Range {
            start: Box::new(RuchyExpr::Literal(Literal::Integer(0))),
            end: Box::new(RuchyExpr::Literal(Literal::Integer(10))),
            inclusive: false,
        };
        assert_eq!(formatter.format(&range_expr), "0..10");

        // Inclusive range
        let inclusive_range = RuchyExpr::Range {
            start: Box::new(RuchyExpr::Literal(Literal::Integer(1))),
            end: Box::new(RuchyExpr::Literal(Literal::Integer(5))),
            inclusive: true,
        };
        assert_eq!(formatter.format(&inclusive_range), "1..=5");
    }

    #[test]
    fn test_format_break_continue() {
        let formatter = RuchyFormatter::new();

        let break_expr = RuchyExpr::Break { label: None };
        assert_eq!(formatter.format(&break_expr), "break");

        let break_with_label = RuchyExpr::Break { label: Some("outer".to_string()) };
        assert_eq!(formatter.format(&break_with_label), "break 'outer");

        let continue_expr = RuchyExpr::Continue { label: None };
        assert_eq!(formatter.format(&continue_expr), "continue");
    }

    #[test]
    fn test_format_return() {
        let formatter = RuchyFormatter::new();

        let return_nothing = RuchyExpr::Return { value: None };
        assert_eq!(formatter.format(&return_nothing), "return");

        let return_value = RuchyExpr::Return {
            value: Some(Box::new(RuchyExpr::Literal(Literal::Integer(42)))),
        };
        assert_eq!(formatter.format(&return_value), "return 42");
    }

    #[test]
    fn test_format_float_literal() {
        let formatter = RuchyFormatter::new();

        let float_expr = RuchyExpr::Literal(Literal::Float(3.14));
        let formatted = formatter.format(&float_expr);
        assert!(formatted.contains("3.14"));

        let whole_float = RuchyExpr::Literal(Literal::Float(5.0));
        let formatted = formatter.format(&whole_float);
        assert!(formatted.contains(".0") || formatted.contains("5"));
    }

    #[test]
    fn test_format_char_literal() {
        let formatter = RuchyFormatter::new();
        let char_expr = RuchyExpr::Literal(Literal::Char('a'));
        assert_eq!(formatter.format(&char_expr), "'a'");
    }

    #[test]
    fn test_format_unit_literal() {
        let formatter = RuchyFormatter::new();
        let unit_expr = RuchyExpr::Literal(Literal::Unit);
        assert_eq!(formatter.format(&unit_expr), "()");
    }

    #[test]
    fn test_formatter_default() {
        let formatter = RuchyFormatter::default();
        assert_eq!(formatter.indent_width, 4);
        assert_eq!(formatter.max_line_length, 100);
    }

    #[test]
    fn test_format_field_access() {
        let formatter = RuchyFormatter::new();

        let field_access = RuchyExpr::FieldAccess {
            object: Box::new(RuchyExpr::Identifier("person".to_string())),
            field: "name".to_string(),
        };
        assert_eq!(formatter.format(&field_access), "person.name");
    }

    #[test]
    fn test_format_await() {
        let formatter = RuchyFormatter::new();

        let await_expr = RuchyExpr::Await {
            expr: Box::new(RuchyExpr::Identifier("future".to_string())),
        };
        assert_eq!(formatter.format(&await_expr), "future.await");
    }

    #[test]
    fn test_format_try() {
        let formatter = RuchyFormatter::new();

        let try_expr = RuchyExpr::Try {
            expr: Box::new(RuchyExpr::Identifier("result".to_string())),
        };
        assert_eq!(formatter.format(&try_expr), "result?");
    }

    #[test]
    fn test_format_method_call() {
        let formatter = RuchyFormatter::new();

        let method_call = RuchyExpr::MethodCall {
            receiver: Box::new(RuchyExpr::Identifier("vec".to_string())),
            method: "push".to_string(),
            args: vec![RuchyExpr::Literal(Literal::Integer(42))],
        };
        assert_eq!(formatter.format(&method_call), "vec.push(42)");
    }

    #[test]
    fn test_format_for_loop() {
        let formatter = RuchyFormatter::new();

        let for_expr = RuchyExpr::For {
            var: "i".to_string(),
            iter: Box::new(RuchyExpr::Identifier("items".to_string())),
            body: Box::new(RuchyExpr::Block(vec![])),
        };
        let formatted = formatter.format(&for_expr);
        assert!(formatted.contains("for i in items"));
    }

    #[test]
    fn test_format_while_loop() {
        let formatter = RuchyFormatter::new();

        let while_expr = RuchyExpr::While {
            condition: Box::new(RuchyExpr::Literal(Literal::Bool(true))),
            body: Box::new(RuchyExpr::Block(vec![])),
        };
        let formatted = formatter.format(&while_expr);
        assert!(formatted.contains("while true"));
    }

    #[test]
    fn test_format_let_binding() {
        let formatter = RuchyFormatter::new();

        let let_expr = RuchyExpr::Let {
            name: "x".to_string(),
            value: Box::new(RuchyExpr::Literal(Literal::Integer(42))),
            body: Box::new(RuchyExpr::Identifier("x".to_string())),
            is_mutable: false,
        };
        let formatted = formatter.format(&let_expr);
        assert!(formatted.contains("let x = 42"));
    }

    #[test]
    fn test_format_let_mut_binding() {
        let formatter = RuchyFormatter::new();

        let let_expr = RuchyExpr::Let {
            name: "y".to_string(),
            value: Box::new(RuchyExpr::Literal(Literal::Integer(10))),
            body: Box::new(RuchyExpr::Identifier("y".to_string())),
            is_mutable: true,
        };
        let formatted = formatter.format(&let_expr);
        assert!(formatted.contains("let mut y = 10"));
    }

    #[test]
    fn test_format_match_expression() {
        use crate::ast::{MatchArm, Pattern};

        let formatter = RuchyFormatter::new();

        let match_expr = RuchyExpr::Match {
            expr: Box::new(RuchyExpr::Identifier("x".to_string())),
            arms: vec![
                MatchArm {
                    pattern: Pattern::Literal(Literal::Integer(1)),
                    guard: None,
                    body: Box::new(RuchyExpr::Literal(Literal::String("one".to_string()))),
                },
                MatchArm {
                    pattern: Pattern::Wildcard,
                    guard: None,
                    body: Box::new(RuchyExpr::Literal(Literal::String("other".to_string()))),
                },
            ],
        };
        let formatted = formatter.format(&match_expr);
        assert!(formatted.contains("match x"));
        assert!(formatted.contains("1 =>"));
        assert!(formatted.contains("_ =>"));
    }

    #[test]
    fn test_format_match_with_guard() {
        use crate::ast::{MatchArm, Pattern};

        let formatter = RuchyFormatter::new();

        let match_expr = RuchyExpr::Match {
            expr: Box::new(RuchyExpr::Identifier("n".to_string())),
            arms: vec![MatchArm {
                pattern: Pattern::Identifier("x".to_string()),
                guard: Some(Box::new(RuchyExpr::Binary {
                    left: Box::new(RuchyExpr::Identifier("x".to_string())),
                    op: BinaryOp::Greater,
                    right: Box::new(RuchyExpr::Literal(Literal::Integer(0))),
                })),
                body: Box::new(RuchyExpr::Literal(Literal::Bool(true))),
            }],
        };
        let formatted = formatter.format(&match_expr);
        assert!(formatted.contains("if x > 0"));
    }

    #[test]
    fn test_format_pattern_tuple() {
        use crate::ast::{MatchArm, Pattern};

        let formatter = RuchyFormatter::new();

        let match_expr = RuchyExpr::Match {
            expr: Box::new(RuchyExpr::Identifier("pair".to_string())),
            arms: vec![MatchArm {
                pattern: Pattern::Tuple(vec![
                    Pattern::Identifier("a".to_string()),
                    Pattern::Identifier("b".to_string()),
                ]),
                guard: None,
                body: Box::new(RuchyExpr::Identifier("a".to_string())),
            }],
        };
        let formatted = formatter.format(&match_expr);
        assert!(formatted.contains("(a, b)"));
    }

    #[test]
    fn test_format_pattern_struct() {
        use crate::ast::{MatchArm, Pattern};

        let formatter = RuchyFormatter::new();

        let match_expr = RuchyExpr::Match {
            expr: Box::new(RuchyExpr::Identifier("point".to_string())),
            arms: vec![MatchArm {
                pattern: Pattern::Struct {
                    name: "Point".to_string(),
                    fields: vec![("x".to_string(), Pattern::Identifier("px".to_string()))],
                },
                guard: None,
                body: Box::new(RuchyExpr::Identifier("px".to_string())),
            }],
        };
        let formatted = formatter.format(&match_expr);
        assert!(formatted.contains("Point { x: px }"));
    }

    #[test]
    fn test_format_pattern_list() {
        use crate::ast::{MatchArm, Pattern};

        let formatter = RuchyFormatter::new();

        let match_expr = RuchyExpr::Match {
            expr: Box::new(RuchyExpr::Identifier("list".to_string())),
            arms: vec![MatchArm {
                pattern: Pattern::List(vec![Pattern::Identifier("head".to_string())]),
                guard: None,
                body: Box::new(RuchyExpr::Identifier("head".to_string())),
            }],
        };
        let formatted = formatter.format(&match_expr);
        assert!(formatted.contains("[head]"));
    }

    #[test]
    fn test_format_struct_definition() {
        use crate::ast::StructField;

        let formatter = RuchyFormatter::new();

        let struct_expr = RuchyExpr::Struct {
            name: "Point".to_string(),
            fields: vec![
                StructField { name: "x".to_string(), typ: RuchyType::I64, is_public: true },
                StructField { name: "y".to_string(), typ: RuchyType::I64, is_public: false },
            ],
        };
        let formatted = formatter.format(&struct_expr);
        assert!(formatted.contains("struct Point"));
        assert!(formatted.contains("pub x: i64"));
        assert!(formatted.contains("y: i64"));
    }

    #[test]
    fn test_format_struct_literal() {
        let formatter = RuchyFormatter::new();

        let struct_lit = RuchyExpr::StructLiteral {
            name: "Point".to_string(),
            fields: vec![
                ("x".to_string(), RuchyExpr::Literal(Literal::Integer(10))),
                ("y".to_string(), RuchyExpr::Literal(Literal::Integer(20))),
            ],
        };
        let formatted = formatter.format(&struct_lit);
        assert!(formatted.contains("Point {"));
        assert!(formatted.contains("x: 10"));
        assert!(formatted.contains("y: 20"));
    }

    #[test]
    fn test_format_dataframe() {
        use crate::ast::DataFrameColumn;

        let formatter = RuchyFormatter::new();

        let df_expr = RuchyExpr::DataFrame {
            columns: vec![DataFrameColumn {
                name: "values".to_string(),
                values: vec![
                    RuchyExpr::Literal(Literal::Integer(1)),
                    RuchyExpr::Literal(Literal::Integer(2)),
                ],
            }],
        };
        let formatted = formatter.format(&df_expr);
        assert!(formatted.contains("df!["));
        assert!(formatted.contains("\"values\""));
    }

    #[test]
    fn test_format_string_interpolation() {
        let formatter = RuchyFormatter::new();

        let interp_expr = RuchyExpr::StringInterpolation {
            parts: vec![
                StringPart::Text("Hello, ".to_string()),
                StringPart::Expr(Box::new(RuchyExpr::Identifier("name".to_string()))),
                StringPart::Text("!".to_string()),
            ],
        };
        let formatted = formatter.format(&interp_expr);
        assert!(formatted.contains("f\"Hello, {name}!\""));
    }

    #[test]
    fn test_format_string_interpolation_escaping() {
        let formatter = RuchyFormatter::new();

        let interp_expr = RuchyExpr::StringInterpolation {
            parts: vec![StringPart::Text("test {brace}".to_string())],
        };
        let formatted = formatter.format(&interp_expr);
        assert!(formatted.contains("{{brace}}"));
    }

    #[test]
    fn test_format_type_primitives() {
        let formatter = RuchyFormatter::new();

        // Test via function with return type
        let fn_expr = RuchyExpr::Function {
            name: "test".to_string(),
            params: vec![],
            body: Box::new(RuchyExpr::Block(vec![])),
            is_async: false,
            return_type: Some(RuchyType::I8),
        };
        assert!(formatter.format(&fn_expr).contains("-> i8"));

        let fn_expr = RuchyExpr::Function {
            name: "test".to_string(),
            params: vec![],
            body: Box::new(RuchyExpr::Block(vec![])),
            is_async: false,
            return_type: Some(RuchyType::I16),
        };
        assert!(formatter.format(&fn_expr).contains("-> i16"));

        let fn_expr = RuchyExpr::Function {
            name: "test".to_string(),
            params: vec![],
            body: Box::new(RuchyExpr::Block(vec![])),
            is_async: false,
            return_type: Some(RuchyType::I32),
        };
        assert!(formatter.format(&fn_expr).contains("-> i32"));

        let fn_expr = RuchyExpr::Function {
            name: "test".to_string(),
            params: vec![],
            body: Box::new(RuchyExpr::Block(vec![])),
            is_async: false,
            return_type: Some(RuchyType::I128),
        };
        assert!(formatter.format(&fn_expr).contains("-> i128"));
    }

    #[test]
    fn test_format_type_unsigned() {
        let formatter = RuchyFormatter::new();

        let fn_expr = RuchyExpr::Function {
            name: "test".to_string(),
            params: vec![],
            body: Box::new(RuchyExpr::Block(vec![])),
            is_async: false,
            return_type: Some(RuchyType::U8),
        };
        assert!(formatter.format(&fn_expr).contains("-> u8"));

        let fn_expr = RuchyExpr::Function {
            name: "test".to_string(),
            params: vec![],
            body: Box::new(RuchyExpr::Block(vec![])),
            is_async: false,
            return_type: Some(RuchyType::U16),
        };
        assert!(formatter.format(&fn_expr).contains("-> u16"));

        let fn_expr = RuchyExpr::Function {
            name: "test".to_string(),
            params: vec![],
            body: Box::new(RuchyExpr::Block(vec![])),
            is_async: false,
            return_type: Some(RuchyType::U64),
        };
        assert!(formatter.format(&fn_expr).contains("-> u64"));

        let fn_expr = RuchyExpr::Function {
            name: "test".to_string(),
            params: vec![],
            body: Box::new(RuchyExpr::Block(vec![])),
            is_async: false,
            return_type: Some(RuchyType::U128),
        };
        assert!(formatter.format(&fn_expr).contains("-> u128"));
    }

    #[test]
    fn test_format_type_size() {
        let formatter = RuchyFormatter::new();

        let fn_expr = RuchyExpr::Function {
            name: "test".to_string(),
            params: vec![],
            body: Box::new(RuchyExpr::Block(vec![])),
            is_async: false,
            return_type: Some(RuchyType::ISize),
        };
        assert!(formatter.format(&fn_expr).contains("-> isize"));

        let fn_expr = RuchyExpr::Function {
            name: "test".to_string(),
            params: vec![],
            body: Box::new(RuchyExpr::Block(vec![])),
            is_async: false,
            return_type: Some(RuchyType::USize),
        };
        assert!(formatter.format(&fn_expr).contains("-> usize"));
    }

    #[test]
    fn test_format_type_float() {
        let formatter = RuchyFormatter::new();

        let fn_expr = RuchyExpr::Function {
            name: "test".to_string(),
            params: vec![],
            body: Box::new(RuchyExpr::Block(vec![])),
            is_async: false,
            return_type: Some(RuchyType::F32),
        };
        assert!(formatter.format(&fn_expr).contains("-> f32"));
    }

    #[test]
    fn test_format_type_char() {
        let formatter = RuchyFormatter::new();

        let fn_expr = RuchyExpr::Function {
            name: "test".to_string(),
            params: vec![],
            body: Box::new(RuchyExpr::Block(vec![])),
            is_async: false,
            return_type: Some(RuchyType::Char),
        };
        assert!(formatter.format(&fn_expr).contains("-> char"));
    }

    #[test]
    fn test_format_type_vec() {
        let formatter = RuchyFormatter::new();

        let fn_expr = RuchyExpr::Function {
            name: "test".to_string(),
            params: vec![],
            body: Box::new(RuchyExpr::Block(vec![])),
            is_async: false,
            return_type: Some(RuchyType::Vec(Box::new(RuchyType::I64))),
        };
        assert!(formatter.format(&fn_expr).contains("-> Vec<i64>"));
    }

    #[test]
    fn test_format_type_array() {
        let formatter = RuchyFormatter::new();

        let fn_expr = RuchyExpr::Function {
            name: "test".to_string(),
            params: vec![],
            body: Box::new(RuchyExpr::Block(vec![])),
            is_async: false,
            return_type: Some(RuchyType::Array(Box::new(RuchyType::I64), 10)),
        };
        assert!(formatter.format(&fn_expr).contains("-> [i64; 10]"));
    }

    #[test]
    fn test_format_type_tuple() {
        let formatter = RuchyFormatter::new();

        let fn_expr = RuchyExpr::Function {
            name: "test".to_string(),
            params: vec![],
            body: Box::new(RuchyExpr::Block(vec![])),
            is_async: false,
            return_type: Some(RuchyType::Tuple(vec![RuchyType::I64, RuchyType::String])),
        };
        assert!(formatter.format(&fn_expr).contains("-> (i64, String)"));
    }

    #[test]
    fn test_format_type_option() {
        let formatter = RuchyFormatter::new();

        let fn_expr = RuchyExpr::Function {
            name: "test".to_string(),
            params: vec![],
            body: Box::new(RuchyExpr::Block(vec![])),
            is_async: false,
            return_type: Some(RuchyType::Option(Box::new(RuchyType::I64))),
        };
        assert!(formatter.format(&fn_expr).contains("-> i64?"));
    }

    #[test]
    fn test_format_type_result() {
        let formatter = RuchyFormatter::new();

        let fn_expr = RuchyExpr::Function {
            name: "test".to_string(),
            params: vec![],
            body: Box::new(RuchyExpr::Block(vec![])),
            is_async: false,
            return_type: Some(RuchyType::Result(
                Box::new(RuchyType::I64),
                Box::new(RuchyType::String),
            )),
        };
        assert!(formatter.format(&fn_expr).contains("-> Result<i64, String>"));
    }

    #[test]
    fn test_format_type_function() {
        let formatter = RuchyFormatter::new();

        let fn_expr = RuchyExpr::Function {
            name: "test".to_string(),
            params: vec![],
            body: Box::new(RuchyExpr::Block(vec![])),
            is_async: false,
            return_type: Some(RuchyType::Function {
                params: vec![RuchyType::I64],
                returns: Box::new(RuchyType::Bool),
            }),
        };
        assert!(formatter.format(&fn_expr).contains("-> fun(i64) -> bool"));
    }

    #[test]
    fn test_format_type_named() {
        let formatter = RuchyFormatter::new();

        let fn_expr = RuchyExpr::Function {
            name: "test".to_string(),
            params: vec![],
            body: Box::new(RuchyExpr::Block(vec![])),
            is_async: false,
            return_type: Some(RuchyType::Named("MyType".to_string())),
        };
        assert!(formatter.format(&fn_expr).contains("-> MyType"));
    }

    #[test]
    fn test_format_type_generic() {
        let formatter = RuchyFormatter::new();

        let fn_expr = RuchyExpr::Function {
            name: "test".to_string(),
            params: vec![],
            body: Box::new(RuchyExpr::Block(vec![])),
            is_async: false,
            return_type: Some(RuchyType::Generic("T".to_string())),
        };
        assert!(formatter.format(&fn_expr).contains("-> T"));
    }

    #[test]
    fn test_format_type_reference() {
        let formatter = RuchyFormatter::new();

        let fn_expr = RuchyExpr::Function {
            name: "test".to_string(),
            params: vec![],
            body: Box::new(RuchyExpr::Block(vec![])),
            is_async: false,
            return_type: Some(RuchyType::Reference {
                typ: Box::new(RuchyType::I64),
                is_mutable: false,
            }),
        };
        assert!(formatter.format(&fn_expr).contains("-> &i64"));
    }

    #[test]
    fn test_format_type_mutable_reference() {
        let formatter = RuchyFormatter::new();

        let fn_expr = RuchyExpr::Function {
            name: "test".to_string(),
            params: vec![],
            body: Box::new(RuchyExpr::Block(vec![])),
            is_async: false,
            return_type: Some(RuchyType::Reference {
                typ: Box::new(RuchyType::String),
                is_mutable: true,
            }),
        };
        assert!(formatter.format(&fn_expr).contains("-> &mut String"));
    }

    #[test]
    fn test_format_type_dynamic() {
        let formatter = RuchyFormatter::new();

        let fn_expr = RuchyExpr::Function {
            name: "test".to_string(),
            params: vec![],
            body: Box::new(RuchyExpr::Block(vec![])),
            is_async: false,
            return_type: Some(RuchyType::Dynamic),
        };
        assert!(formatter.format(&fn_expr).contains("-> dyn Any"));
    }

    #[test]
    fn test_format_pipeline_all_stages() {
        let formatter = RuchyFormatter::new();

        let pipeline = RuchyExpr::Pipeline {
            expr: Box::new(RuchyExpr::Identifier("items".to_string())),
            stages: vec![
                PipelineStage::Map(Box::new(RuchyExpr::Identifier("f".to_string()))),
                PipelineStage::Filter(Box::new(RuchyExpr::Identifier("g".to_string()))),
                PipelineStage::FlatMap(Box::new(RuchyExpr::Identifier("h".to_string()))),
                PipelineStage::Reduce(Box::new(RuchyExpr::Identifier("r".to_string()))),
                PipelineStage::Call("collect".to_string(), vec![]),
            ],
        };
        let formatted = formatter.format(&pipeline);
        assert!(formatted.contains("map(f)"));
        assert!(formatted.contains("filter(g)"));
        assert!(formatted.contains("flat_map(h)"));
        assert!(formatted.contains("reduce(r)"));
        assert!(formatted.contains("collect()"));
    }

    #[test]
    fn test_format_bitwise_operators() {
        let formatter = RuchyFormatter::new();

        let bitand = RuchyExpr::Binary {
            left: Box::new(RuchyExpr::Identifier("a".to_string())),
            op: BinaryOp::BitwiseAnd,
            right: Box::new(RuchyExpr::Identifier("b".to_string())),
        };
        assert_eq!(formatter.format(&bitand), "a & b");

        let bitor = RuchyExpr::Binary {
            left: Box::new(RuchyExpr::Identifier("a".to_string())),
            op: BinaryOp::BitwiseOr,
            right: Box::new(RuchyExpr::Identifier("b".to_string())),
        };
        assert_eq!(formatter.format(&bitor), "a | b");

        let bitxor = RuchyExpr::Binary {
            left: Box::new(RuchyExpr::Identifier("a".to_string())),
            op: BinaryOp::BitwiseXor,
            right: Box::new(RuchyExpr::Identifier("b".to_string())),
        };
        assert_eq!(formatter.format(&bitxor), "a ^ b");

        let lshift = RuchyExpr::Binary {
            left: Box::new(RuchyExpr::Identifier("a".to_string())),
            op: BinaryOp::LeftShift,
            right: Box::new(RuchyExpr::Literal(Literal::Integer(2))),
        };
        assert_eq!(formatter.format(&lshift), "a << 2");

        let rshift = RuchyExpr::Binary {
            left: Box::new(RuchyExpr::Identifier("a".to_string())),
            op: BinaryOp::RightShift,
            right: Box::new(RuchyExpr::Literal(Literal::Integer(2))),
        };
        assert_eq!(formatter.format(&rshift), "a >> 2");
    }

    #[test]
    fn test_format_power_operator() {
        let formatter = RuchyFormatter::new();

        let pow = RuchyExpr::Binary {
            left: Box::new(RuchyExpr::Literal(Literal::Integer(2))),
            op: BinaryOp::Power,
            right: Box::new(RuchyExpr::Literal(Literal::Integer(8))),
        };
        assert_eq!(formatter.format(&pow), "2 ** 8");
    }

    #[test]
    fn test_format_bitwise_not() {
        let formatter = RuchyFormatter::new();

        let bitnot = RuchyExpr::Unary {
            op: UnaryOp::BitwiseNot,
            operand: Box::new(RuchyExpr::Identifier("x".to_string())),
        };
        assert_eq!(formatter.format(&bitnot), "!x");
    }

    #[test]
    fn test_format_function_with_params() {
        use crate::ast::Param;

        let formatter = RuchyFormatter::new();

        let fn_expr = RuchyExpr::Function {
            name: "add".to_string(),
            params: vec![
                Param { name: "x".to_string(), typ: Some(RuchyType::I64), default: None },
                Param { name: "y".to_string(), typ: Some(RuchyType::I64), default: None },
            ],
            body: Box::new(RuchyExpr::Binary {
                left: Box::new(RuchyExpr::Identifier("x".to_string())),
                op: BinaryOp::Add,
                right: Box::new(RuchyExpr::Identifier("y".to_string())),
            }),
            is_async: false,
            return_type: Some(RuchyType::I64),
        };
        let formatted = formatter.format(&fn_expr);
        assert!(formatted.contains("fun add(x: i64, y: i64) -> i64"));
    }

    #[test]
    fn test_format_function_with_default_param() {
        use crate::ast::Param;

        let formatter = RuchyFormatter::new();

        let fn_expr = RuchyExpr::Function {
            name: "greet".to_string(),
            params: vec![Param {
                name: "name".to_string(),
                typ: Some(RuchyType::String),
                default: Some(Box::new(RuchyExpr::Literal(Literal::String("World".to_string())))),
            }],
            body: Box::new(RuchyExpr::Block(vec![])),
            is_async: false,
            return_type: None,
        };
        let formatted = formatter.format(&fn_expr);
        assert!(formatted.contains("name: String = \"World\""));
    }

    #[test]
    fn test_format_lambda_with_block() {
        use crate::ast::Param;

        let formatter = RuchyFormatter::new();

        let lambda = RuchyExpr::Lambda {
            params: vec![Param { name: "x".to_string(), typ: None, default: None }],
            body: Box::new(RuchyExpr::Block(vec![RuchyExpr::Identifier("x".to_string())])),
        };
        let formatted = formatter.format(&lambda);
        assert!(formatted.contains("|x|"));
        assert!(formatted.contains("{"));
    }

    #[test]
    fn test_format_continue_with_label() {
        let formatter = RuchyFormatter::new();

        let cont = RuchyExpr::Continue { label: Some("loop1".to_string()) };
        assert_eq!(formatter.format(&cont), "continue 'loop1");
    }

    #[test]
    fn test_format_if_else_if() {
        let formatter = RuchyFormatter::new();

        let if_else_if = RuchyExpr::If {
            condition: Box::new(RuchyExpr::Literal(Literal::Bool(true))),
            then_branch: Box::new(RuchyExpr::Literal(Literal::Integer(1))),
            else_branch: Some(Box::new(RuchyExpr::If {
                condition: Box::new(RuchyExpr::Literal(Literal::Bool(false))),
                then_branch: Box::new(RuchyExpr::Literal(Literal::Integer(2))),
                else_branch: None,
            })),
        };
        let formatted = formatter.format(&if_else_if);
        assert!(formatted.contains("if true"));
        assert!(formatted.contains("else if false"));
    }

    #[test]
    fn test_formatter_with_config() {
        let mut config = crate::RuchyConfig::default();
        config.indent_width = 2;
        config.max_line_length = 80;
        let formatter = RuchyFormatter::with_config(&config);
        assert_eq!(formatter.indent_width, 2);
        assert_eq!(formatter.max_line_length, 80);
    }

    #[test]
    fn test_format_empty_struct() {
        let formatter = RuchyFormatter::new();

        let empty_struct = RuchyExpr::Struct {
            name: "Empty".to_string(),
            fields: vec![],
        };
        let formatted = formatter.format(&empty_struct);
        assert_eq!(formatted, "struct Empty {}");
    }

    #[test]
    fn test_format_empty_dataframe() {
        let formatter = RuchyFormatter::new();

        let empty_df = RuchyExpr::DataFrame { columns: vec![] };
        let formatted = formatter.format(&empty_df);
        assert_eq!(formatted, "df![]");
    }
}
