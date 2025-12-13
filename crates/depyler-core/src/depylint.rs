//! DEPYLER-0951: Depylint - Proactive detection of unsupported Python features
//!
//! This module provides static analysis to identify Python constructs that are
//! not supported by Depyler's transpilation pipeline. Running depylint before
//! transpilation helps users understand what needs to be refactored.
//!
//! ## Unsupported Features
//! - `eval()` and `exec()` - Dynamic code execution
//! - `globals()` and `locals()` - Dynamic namespace access
//! - Metaclasses - Dynamic class creation
//! - `__getattr__` / `__setattr__` - Dynamic attribute access
//! - Monkey patching patterns

use rustpython_parser::ast::{self, Expr, Stmt};
use std::collections::HashSet;

/// A lint warning about an unsupported Python feature
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LintWarning {
    /// Byte offset in source
    pub offset: usize,
    /// Warning code (e.g., "DPL001")
    pub code: String,
    /// Human-readable message
    pub message: String,
    /// Severity level
    pub severity: Severity,
    /// Suggested fix or workaround
    pub suggestion: Option<String>,
}

/// Severity level for lint warnings
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// Will definitely cause transpilation failure
    Error,
    /// May cause issues depending on usage
    Warning,
    /// Informational - suboptimal but works
    Info,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Error => write!(f, "error"),
            Severity::Warning => write!(f, "warning"),
            Severity::Info => write!(f, "info"),
        }
    }
}

/// Depylint analyzer for Python source code
pub struct DepylintAnalyzer {
    warnings: Vec<LintWarning>,
    /// Track which features have been reported to avoid duplicates
    reported: HashSet<String>,
}

impl Default for DepylintAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl DepylintAnalyzer {
    /// Create a new analyzer
    pub fn new() -> Self {
        Self {
            warnings: Vec::new(),
            reported: HashSet::new(),
        }
    }

    /// Analyze Python source code and return warnings
    pub fn analyze(&mut self, source: &str) -> Vec<LintWarning> {
        self.warnings.clear();
        self.reported.clear();

        // Parse the Python source
        let parsed = rustpython_parser::parse(source, rustpython_parser::Mode::Module, "<input>");
        if let Ok(ast::Mod::Module(module)) = parsed {
            for stmt in &module.body {
                self.visit_stmt(stmt);
            }
        }

        std::mem::take(&mut self.warnings)
    }

    /// Get offset from a range
    fn get_offset(range: &ast::text_size::TextRange) -> usize {
        range.start().into()
    }

    fn visit_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::FunctionDef(func) => {
                // Check for dunder methods that indicate dynamic behavior
                if func.name.as_str().starts_with("__") && func.name.as_str().ends_with("__") {
                    self.check_dunder_method(&func.name, Self::get_offset(&func.range));
                }
                // Visit function body
                for s in &func.body {
                    self.visit_stmt(s);
                }
            }
            Stmt::AsyncFunctionDef(func) => {
                for s in &func.body {
                    self.visit_stmt(s);
                }
            }
            Stmt::ClassDef(class) => {
                // Check for metaclass
                for keyword in &class.keywords {
                    if keyword.arg.as_ref().is_some_and(|a| a.as_str() == "metaclass") {
                        self.add_warning(
                            Self::get_offset(&class.range),
                            "DPL003",
                            "Metaclasses are not supported",
                            Severity::Error,
                            Some("Use composition or factory patterns instead".to_string()),
                        );
                    }
                }
                // Visit class body
                for s in &class.body {
                    self.visit_stmt(s);
                }
            }
            Stmt::Expr(expr_stmt) => {
                self.visit_expr(&expr_stmt.value);
            }
            Stmt::Assign(assign) => {
                self.visit_expr(&assign.value);
                for target in &assign.targets {
                    self.visit_expr(target);
                }
            }
            Stmt::AnnAssign(ann_assign) => {
                if let Some(value) = &ann_assign.value {
                    self.visit_expr(value);
                }
            }
            Stmt::Return(ret) => {
                if let Some(value) = &ret.value {
                    self.visit_expr(value);
                }
            }
            Stmt::If(if_stmt) => {
                self.visit_expr(&if_stmt.test);
                for s in &if_stmt.body {
                    self.visit_stmt(s);
                }
                for s in &if_stmt.orelse {
                    self.visit_stmt(s);
                }
            }
            Stmt::For(for_stmt) => {
                self.visit_expr(&for_stmt.iter);
                for s in &for_stmt.body {
                    self.visit_stmt(s);
                }
            }
            Stmt::While(while_stmt) => {
                self.visit_expr(&while_stmt.test);
                for s in &while_stmt.body {
                    self.visit_stmt(s);
                }
            }
            Stmt::With(with_stmt) => {
                for item in &with_stmt.items {
                    self.visit_expr(&item.context_expr);
                }
                for s in &with_stmt.body {
                    self.visit_stmt(s);
                }
            }
            Stmt::Try(try_stmt) => {
                for s in &try_stmt.body {
                    self.visit_stmt(s);
                }
                for handler in &try_stmt.handlers {
                    let ast::ExceptHandler::ExceptHandler(h) = handler;
                    for s in &h.body {
                        self.visit_stmt(s);
                    }
                }
            }
            _ => {}
        }
    }

    fn visit_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Call(call) => {
                // Check for unsupported builtin calls
                if let Expr::Name(name) = call.func.as_ref() {
                    let func_name = name.id.as_str();
                    let offset = Self::get_offset(&call.range);
                    match func_name {
                        "eval" => {
                            self.add_warning(
                                offset,
                                "DPL001",
                                "eval() is not supported - dynamic code execution cannot be transpiled",
                                Severity::Error,
                                Some("Refactor to use explicit logic or data structures".to_string()),
                            );
                        }
                        "exec" => {
                            self.add_warning(
                                offset,
                                "DPL002",
                                "exec() is not supported - dynamic code execution cannot be transpiled",
                                Severity::Error,
                                Some("Refactor to use explicit function calls".to_string()),
                            );
                        }
                        "globals" => {
                            self.add_warning(
                                offset,
                                "DPL004",
                                "globals() is not supported - dynamic namespace access cannot be transpiled",
                                Severity::Error,
                                Some("Use explicit module imports or pass variables as arguments".to_string()),
                            );
                        }
                        "locals" => {
                            self.add_warning(
                                offset,
                                "DPL005",
                                "locals() is not supported - dynamic namespace access cannot be transpiled",
                                Severity::Warning,
                                Some("Use explicit variable references".to_string()),
                            );
                        }
                        "setattr" | "getattr" | "delattr" => {
                            self.add_warning(
                                offset,
                                "DPL006",
                                &format!("{}() with dynamic attribute names is not fully supported", func_name),
                                Severity::Warning,
                                Some("Use explicit attribute access when possible".to_string()),
                            );
                        }
                        "type" if call.args.len() == 3 => {
                            // type() with 3 args creates a class dynamically
                            self.add_warning(
                                offset,
                                "DPL007",
                                "Dynamic class creation with type() is not supported",
                                Severity::Error,
                                Some("Define classes statically".to_string()),
                            );
                        }
                        _ => {}
                    }
                }
                // Visit call arguments
                for arg in &call.args {
                    self.visit_expr(arg);
                }
            }
            Expr::BinOp(binop) => {
                self.visit_expr(&binop.left);
                self.visit_expr(&binop.right);
            }
            Expr::UnaryOp(unop) => {
                self.visit_expr(&unop.operand);
            }
            Expr::Lambda(lambda) => {
                self.visit_expr(&lambda.body);
            }
            Expr::IfExp(ifexp) => {
                self.visit_expr(&ifexp.test);
                self.visit_expr(&ifexp.body);
                self.visit_expr(&ifexp.orelse);
            }
            Expr::List(list) => {
                for elt in &list.elts {
                    self.visit_expr(elt);
                }
            }
            Expr::Dict(dict) => {
                for key in dict.keys.iter().flatten() {
                    self.visit_expr(key);
                }
                for value in &dict.values {
                    self.visit_expr(value);
                }
            }
            Expr::ListComp(comp) => {
                self.visit_expr(&comp.elt);
                for gen in &comp.generators {
                    self.visit_expr(&gen.iter);
                }
            }
            Expr::DictComp(comp) => {
                self.visit_expr(&comp.key);
                self.visit_expr(&comp.value);
                for gen in &comp.generators {
                    self.visit_expr(&gen.iter);
                }
            }
            Expr::Subscript(subscript) => {
                self.visit_expr(&subscript.value);
                self.visit_expr(&subscript.slice);
            }
            Expr::Attribute(attr) => {
                self.visit_expr(&attr.value);
            }
            Expr::Compare(compare) => {
                self.visit_expr(&compare.left);
                for comp in &compare.comparators {
                    self.visit_expr(comp);
                }
            }
            _ => {}
        }
    }

    fn check_dunder_method(&mut self, name: &str, offset: usize) {
        let problematic_dunders = [
            ("__getattr__", "DPL008", "Dynamic attribute access"),
            ("__setattr__", "DPL009", "Dynamic attribute setting"),
            ("__delattr__", "DPL010", "Dynamic attribute deletion"),
            ("__getattribute__", "DPL011", "Attribute access interception"),
        ];

        for (dunder, code, desc) in problematic_dunders {
            if name == dunder {
                self.add_warning(
                    offset,
                    code,
                    &format!("{} ({}) is not fully supported", dunder, desc),
                    Severity::Warning,
                    Some("Use explicit properties or methods".to_string()),
                );
            }
        }
    }

    fn add_warning(
        &mut self,
        offset: usize,
        code: &str,
        message: &str,
        severity: Severity,
        suggestion: Option<String>,
    ) {
        let key = format!("{}:{}", code, offset);
        if !self.reported.contains(&key) {
            self.reported.insert(key);
            self.warnings.push(LintWarning {
                offset,
                code: code.to_string(),
                message: message.to_string(),
                severity,
                suggestion,
            });
        }
    }
}

/// Format warnings for display
pub fn format_warnings(warnings: &[LintWarning], source: &str, source_path: &str) -> String {
    let mut output = String::new();
    for w in warnings {
        // Calculate line/column from offset
        let (line, col) = offset_to_line_col(source, w.offset);
        output.push_str(&format!(
            "{}:{}:{}: {} [{}]: {}\n",
            source_path, line, col, w.severity, w.code, w.message
        ));
        if let Some(ref suggestion) = w.suggestion {
            output.push_str(&format!("  suggestion: {}\n", suggestion));
        }
    }
    output
}

/// Convert byte offset to line and column (1-indexed)
fn offset_to_line_col(source: &str, offset: usize) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;
    for (i, c) in source.char_indices() {
        if i >= offset {
            break;
        }
        if c == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }
    (line, col)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_eval() {
        let mut analyzer = DepylintAnalyzer::new();
        let warnings = analyzer.analyze("result = eval('1 + 2')");
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].code, "DPL001");
        assert_eq!(warnings[0].severity, Severity::Error);
    }

    #[test]
    fn test_detect_exec() {
        let mut analyzer = DepylintAnalyzer::new();
        let warnings = analyzer.analyze("exec('print(1)')");
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].code, "DPL002");
    }

    #[test]
    fn test_detect_metaclass() {
        let mut analyzer = DepylintAnalyzer::new();
        let warnings = analyzer.analyze("class Foo(metaclass=ABCMeta): pass");
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].code, "DPL003");
    }

    #[test]
    fn test_detect_globals() {
        let mut analyzer = DepylintAnalyzer::new();
        let warnings = analyzer.analyze("x = globals()['foo']");
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].code, "DPL004");
    }

    #[test]
    fn test_detect_getattr_dunder() {
        let mut analyzer = DepylintAnalyzer::new();
        let warnings = analyzer.analyze(
            r#"
class Foo:
    def __getattr__(self, name):
        return None
"#,
        );
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].code, "DPL008");
    }

    #[test]
    fn test_clean_code_no_warnings() {
        let mut analyzer = DepylintAnalyzer::new();
        let warnings = analyzer.analyze(
            r#"
def add(a: int, b: int) -> int:
    return a + b

class Calculator:
    def multiply(self, x: float, y: float) -> float:
        return x * y
"#,
        );
        assert!(warnings.is_empty(), "Clean code should have no warnings");
    }

    #[test]
    fn test_dynamic_type_creation() {
        let mut analyzer = DepylintAnalyzer::new();
        let warnings = analyzer.analyze("MyClass = type('MyClass', (Base,), {'x': 1})");
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].code, "DPL007");
    }

    #[test]
    fn test_format_warnings() {
        let source = "result = eval('1 + 2')";
        let warnings = vec![LintWarning {
            offset: 9,
            code: "DPL001".to_string(),
            message: "eval() is not supported".to_string(),
            severity: Severity::Error,
            suggestion: Some("Use explicit logic".to_string()),
        }];
        let output = format_warnings(&warnings, source, "test.py");
        assert!(output.contains("test.py:1:"));
        assert!(output.contains("DPL001"));
        assert!(output.contains("suggestion:"));
    }

    #[test]
    fn test_offset_to_line_col() {
        let source = "line1\nline2\nline3";
        assert_eq!(offset_to_line_col(source, 0), (1, 1));  // 'l' in line1
        assert_eq!(offset_to_line_col(source, 6), (2, 1));  // 'l' in line2
        assert_eq!(offset_to_line_col(source, 12), (3, 1)); // 'l' in line3
    }
}
