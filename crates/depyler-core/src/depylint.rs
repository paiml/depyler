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
                    if keyword
                        .arg
                        .as_ref()
                        .is_some_and(|a| a.as_str() == "metaclass")
                    {
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
                // DEPYLER-PHASE2: Check for self-referential patterns (Poka-Yoke)
                self.check_self_reference(stmt);
                self.visit_expr(&expr_stmt.value);
            }
            Stmt::Assign(assign) => {
                // DEPYLER-PHASE2: Check for self-referential patterns (Poka-Yoke)
                self.check_self_reference(stmt);
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
                // DEPYLER-PHASE2: Check for mutation-while-iterating (Poka-Yoke)
                self.check_mutation_while_iterating(for_stmt);
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
                                &format!(
                                    "{}() with dynamic attribute names is not fully supported",
                                    func_name
                                ),
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
            (
                "__getattribute__",
                "DPL011",
                "Attribute access interception",
            ),
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

    // ============================================================================
    // DEPYLER-PHASE2: Poka-Yoke Rejection Patterns
    //
    // These patterns cannot be safely transpiled to Rust's ownership model and
    // must be rejected at lint time with clear error messages.
    // ============================================================================

    /// Check for mutation-while-iterating pattern (Poka-Yoke DPL100)
    ///
    /// Detects patterns like:
    /// ```python
    /// for x in items:
    ///     items.append(x * 2)  # INVALID: mutating while iterating
    /// ```
    pub fn check_mutation_while_iterating(&mut self, for_stmt: &ast::StmtFor) {
        // Extract iterator variable name
        let iter_name = match &*for_stmt.iter {
            Expr::Name(name) => Some(name.id.as_str().to_string()),
            _ => None,
        };

        if let Some(iter_var) = iter_name {
            // Check body for mutations to the iterator
            for stmt in &for_stmt.body {
                if self.stmt_mutates_var(stmt, &iter_var) {
                    self.add_warning(
                        Self::get_offset(&for_stmt.range),
                        "DPL100",
                        &format!(
                            "Mutating '{}' while iterating over it is not supported - \
                             Rust's borrow checker prevents this pattern",
                            iter_var
                        ),
                        Severity::Error,
                        Some(format!(
                            "Collect items to add/remove in a separate list, then modify '{}' after the loop",
                            iter_var
                        )),
                    );
                    break; // One warning per loop
                }
            }
        }
    }

    /// Check if a statement mutates a specific variable
    fn stmt_mutates_var(&self, stmt: &Stmt, var_name: &str) -> bool {
        match stmt {
            Stmt::Expr(expr_stmt) => self.expr_mutates_var(&expr_stmt.value, var_name),
            Stmt::If(if_stmt) => {
                if_stmt
                    .body
                    .iter()
                    .any(|s| self.stmt_mutates_var(s, var_name))
                    || if_stmt
                        .orelse
                        .iter()
                        .any(|s| self.stmt_mutates_var(s, var_name))
            }
            _ => false,
        }
    }

    /// Check if an expression mutates a specific variable
    fn expr_mutates_var(&self, expr: &Expr, var_name: &str) -> bool {
        match expr {
            Expr::Call(call) => {
                // Check for method calls like items.append(), items.remove(), etc.
                if let Expr::Attribute(attr) = call.func.as_ref() {
                    if let Expr::Name(name) = attr.value.as_ref() {
                        if name.id.as_str() == var_name {
                            let method = attr.attr.as_str();
                            let mutating_methods = [
                                "append",
                                "extend",
                                "insert",
                                "remove",
                                "pop",
                                "clear",
                                "add",
                                "discard",
                                "update",
                                "setdefault",
                                "__setitem__",
                            ];
                            return mutating_methods.contains(&method);
                        }
                    }
                }
                false
            }
            _ => false,
        }
    }

    /// Check for self-referential structures (Poka-Yoke DPL101)
    ///
    /// Detects patterns like:
    /// ```python
    /// d["self"] = d  # INVALID: self-reference
    /// lst.append(lst)  # INVALID: list contains itself
    /// ```
    pub fn check_self_reference(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Assign(assign) => {
                // Check for d[key] = d pattern
                for target in &assign.targets {
                    if let Expr::Subscript(subscript) = target {
                        if let Expr::Name(target_name) = subscript.value.as_ref() {
                            if let Expr::Name(value_name) = assign.value.as_ref() {
                                if target_name.id == value_name.id {
                                    self.add_warning(
                                        Self::get_offset(&assign.range),
                                        "DPL101",
                                        &format!(
                                            "Self-referential assignment to '{}' is not supported - \
                                             Rust cannot represent cyclic data structures without Rc/RefCell",
                                            target_name.id
                                        ),
                                        Severity::Error,
                                        Some(
                                            "Use indices or separate data structures to represent relationships".to_string(),
                                        ),
                                    );
                                }
                            }
                        }
                    }
                }
            }
            Stmt::Expr(expr_stmt) => {
                // Check for lst.append(lst) pattern
                if let Expr::Call(call) = expr_stmt.value.as_ref() {
                    if let Expr::Attribute(attr) = call.func.as_ref() {
                        let method = attr.attr.as_str();
                        if method == "append" || method == "add" || method == "extend" {
                            if let Expr::Name(receiver_name) = attr.value.as_ref() {
                                for arg in &call.args {
                                    if let Expr::Name(arg_name) = arg {
                                        if receiver_name.id == arg_name.id {
                                            self.add_warning(
                                                Self::get_offset(&call.range),
                                                "DPL101",
                                                &format!(
                                                    "Adding '{}' to itself creates a cyclic reference - \
                                                     Rust cannot represent this without Rc/RefCell",
                                                    receiver_name.id
                                                ),
                                                Severity::Error,
                                                Some(
                                                    "Use a copy if you need to store duplicate data".to_string(),
                                                ),
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    /// Check for cyclic reference patterns (Poka-Yoke DPL102)
    ///
    /// Detects patterns like:
    /// ```python
    /// a.next = b
    /// b.next = a  # INVALID: cyclic reference
    /// ```
    ///
    /// Note: This is a simplified check that detects immediate cycles within
    /// a single scope. Full cycle detection would require dataflow analysis.
    pub fn check_cyclic_assignment(&mut self, assigns: &[&Stmt]) {
        // Track attribute assignments: (object, attribute) -> value
        let mut attr_assigns: Vec<(String, String, String)> = Vec::new();

        for stmt in assigns {
            if let Stmt::Assign(assign) = stmt {
                for target in &assign.targets {
                    if let Expr::Attribute(attr) = target {
                        if let Expr::Name(obj_name) = attr.value.as_ref() {
                            if let Expr::Name(value_name) = assign.value.as_ref() {
                                let obj = obj_name.id.as_str().to_string();
                                let field = attr.attr.as_str().to_string();
                                let value = value_name.id.as_str().to_string();
                                attr_assigns.push((obj, field, value));
                            }
                        }
                    }
                }
            }
        }

        // Detect simple cycles: a.x = b and b.y = a
        for (i, (obj1, _field1, val1)) in attr_assigns.iter().enumerate() {
            for (obj2, _field2, val2) in attr_assigns.iter().skip(i + 1) {
                if obj1 == val2 && obj2 == val1 {
                    // Found a cycle: obj1 points to val1 (obj2) and obj2 points to val2 (obj1)
                    self.add_warning(
                        0, // Would need offset tracking
                        "DPL102",
                        &format!(
                            "Cyclic reference detected: '{}' and '{}' reference each other - \
                             Rust cannot represent cyclic structures without Rc/RefCell",
                            obj1, obj2
                        ),
                        Severity::Error,
                        Some("Use weak references or restructure to avoid cycles".to_string()),
                    );
                }
            }
        }
    }
}

/// Poka-Yoke violation error
#[derive(Debug, Clone)]
pub struct PokaYokeViolation {
    /// Error code
    pub code: String,
    /// Description of the violation
    pub description: String,
    /// Location in source
    pub offset: usize,
    /// Suggested fix
    pub suggestion: String,
}

impl std::fmt::Display for PokaYokeViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code, self.description)
    }
}

impl std::error::Error for PokaYokeViolation {}

/// Check a function for Poka-Yoke violations (HIR-level)
///
/// This function operates on the HIR and can be called during transpilation
/// to reject patterns that cannot be safely transpiled.
pub fn check_poka_yoke(func: &crate::hir::HirFunction) -> Result<(), PokaYokeViolation> {
    for stmt in &func.body {
        if let Some(violation) = detect_hir_mutation_while_iterating(stmt) {
            return Err(violation);
        }
        if let Some(violation) = detect_hir_self_reference(stmt) {
            return Err(violation);
        }
    }
    Ok(())
}

/// Detect mutation-while-iterating in HIR
fn detect_hir_mutation_while_iterating(stmt: &crate::hir::HirStmt) -> Option<PokaYokeViolation> {
    use crate::hir::{HirExpr, HirStmt};

    if let HirStmt::For { iter, body, .. } = stmt {
        // Get iterator variable name - HirExpr::Var(Symbol) where Symbol = String
        let iter_name = if let HirExpr::Var(name) = iter {
            Some(name.clone())
        } else {
            None
        };

        if let Some(iter_var) = iter_name {
            // Check body for mutations
            for body_stmt in body {
                if hir_stmt_mutates_var(body_stmt, &iter_var) {
                    return Some(PokaYokeViolation {
                        code: "DPL100".to_string(),
                        description: format!(
                            "Cannot mutate '{}' while iterating over it",
                            iter_var
                        ),
                        offset: 0,
                        suggestion: "Collect modifications and apply after loop".to_string(),
                    });
                }
            }
        }
    }

    None
}

/// Check if an HIR statement mutates a variable
fn hir_stmt_mutates_var(stmt: &crate::hir::HirStmt, var_name: &str) -> bool {
    use crate::hir::HirStmt;

    match stmt {
        HirStmt::Expr(expr) => hir_expr_mutates_var(expr, var_name),
        HirStmt::If {
            then_body,
            else_body,
            ..
        } => {
            let then_mutates = then_body.iter().any(|s| hir_stmt_mutates_var(s, var_name));
            let else_mutates = else_body
                .as_ref()
                .map(|stmts| stmts.iter().any(|s| hir_stmt_mutates_var(s, var_name)))
                .unwrap_or(false);
            then_mutates || else_mutates
        }
        HirStmt::Block(stmts) => stmts.iter().any(|s| hir_stmt_mutates_var(s, var_name)),
        _ => false,
    }
}

/// Check if an HIR expression mutates a variable
fn hir_expr_mutates_var(expr: &crate::hir::HirExpr, var_name: &str) -> bool {
    use crate::hir::HirExpr;

    match expr {
        HirExpr::MethodCall { object, method, .. } => {
            if let HirExpr::Var(name) = object.as_ref() {
                if name == var_name {
                    let mutating_methods = [
                        "append",
                        "extend",
                        "insert",
                        "remove",
                        "pop",
                        "clear",
                        "add",
                        "discard",
                        "update",
                        "setdefault",
                    ];
                    return mutating_methods.contains(&method.as_str());
                }
            }
            false
        }
        _ => false,
    }
}

/// Detect self-referential patterns in HIR
fn detect_hir_self_reference(stmt: &crate::hir::HirStmt) -> Option<PokaYokeViolation> {
    use crate::hir::{HirExpr, HirStmt};

    match stmt {
        HirStmt::Assign { target, value, .. } => {
            // Check for self-reference through index: d["x"] = d
            if let Some(target_var) = find_base_var_in_assign(target) {
                if let HirExpr::Var(value_var) = value {
                    if target_var == value_var {
                        return Some(PokaYokeViolation {
                            code: "DPL101".to_string(),
                            description: format!(
                                "Self-referential assignment to '{}' is not allowed",
                                target_var
                            ),
                            offset: 0,
                            suggestion: "Use indices or separate structures".to_string(),
                        });
                    }
                }
            }
        }
        HirStmt::Expr(HirExpr::MethodCall {
            object,
            method,
            args,
            ..
        }) => {
            // Check for self-appending: lst.append(lst)
            if method == "append" || method == "add" || method == "extend" {
                if let HirExpr::Var(recv_name) = object.as_ref() {
                    for arg in args {
                        if let HirExpr::Var(arg_name) = arg {
                            if recv_name == arg_name {
                                return Some(PokaYokeViolation {
                                    code: "DPL101".to_string(),
                                    description: format!(
                                        "Adding '{}' to itself creates a cycle",
                                        recv_name
                                    ),
                                    offset: 0,
                                    suggestion: "Use a copy to store duplicate data".to_string(),
                                });
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }

    None
}

/// Helper to find the base variable in an assignment target
fn find_base_var_in_assign(target: &crate::hir::AssignTarget) -> Option<&str> {
    use crate::hir::{AssignTarget, HirExpr};

    match target {
        AssignTarget::Symbol(name) => Some(name.as_str()),
        AssignTarget::Index { base, .. } => {
            // Recursively find the base variable in index expression
            if let HirExpr::Var(name) = base.as_ref() {
                Some(name.as_str())
            } else {
                None
            }
        }
        AssignTarget::Attribute { value, .. } => {
            if let HirExpr::Var(name) = value.as_ref() {
                Some(name.as_str())
            } else {
                None
            }
        }
        AssignTarget::Tuple(_) => None, // Tuple unpacking - skip for now
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

    // === LintWarning tests ===

    #[test]
    fn test_lint_warning_new() {
        let warning = LintWarning {
            offset: 10,
            code: "DPL001".to_string(),
            message: "test message".to_string(),
            severity: Severity::Error,
            suggestion: Some("fix it".to_string()),
        };
        assert_eq!(warning.offset, 10);
        assert_eq!(warning.code, "DPL001");
        assert_eq!(warning.message, "test message");
        assert_eq!(warning.severity, Severity::Error);
        assert_eq!(warning.suggestion, Some("fix it".to_string()));
    }

    #[test]
    fn test_lint_warning_no_suggestion() {
        let warning = LintWarning {
            offset: 0,
            code: "DPL000".to_string(),
            message: "msg".to_string(),
            severity: Severity::Info,
            suggestion: None,
        };
        assert!(warning.suggestion.is_none());
    }

    #[test]
    fn test_lint_warning_clone() {
        let warning = LintWarning {
            offset: 5,
            code: "DPL002".to_string(),
            message: "exec".to_string(),
            severity: Severity::Warning,
            suggestion: None,
        };
        let cloned = warning.clone();
        assert_eq!(warning, cloned);
    }

    #[test]
    fn test_lint_warning_partial_eq() {
        let w1 = LintWarning {
            offset: 1,
            code: "A".to_string(),
            message: "m".to_string(),
            severity: Severity::Error,
            suggestion: None,
        };
        let w2 = LintWarning {
            offset: 1,
            code: "A".to_string(),
            message: "m".to_string(),
            severity: Severity::Error,
            suggestion: None,
        };
        assert_eq!(w1, w2);
    }

    #[test]
    fn test_lint_warning_debug() {
        let warning = LintWarning {
            offset: 0,
            code: "DPL001".to_string(),
            message: "test".to_string(),
            severity: Severity::Error,
            suggestion: None,
        };
        let debug = format!("{:?}", warning);
        assert!(debug.contains("LintWarning"));
        assert!(debug.contains("DPL001"));
    }

    // === Severity tests ===

    #[test]
    fn test_severity_display_error() {
        assert_eq!(format!("{}", Severity::Error), "error");
    }

    #[test]
    fn test_severity_display_warning() {
        assert_eq!(format!("{}", Severity::Warning), "warning");
    }

    #[test]
    fn test_severity_display_info() {
        assert_eq!(format!("{}", Severity::Info), "info");
    }

    #[test]
    fn test_severity_clone() {
        let s = Severity::Error;
        let cloned = s;
        assert_eq!(s, cloned);
    }

    #[test]
    fn test_severity_copy() {
        let s1 = Severity::Warning;
        let s2 = s1;
        assert_eq!(s1, s2);
    }

    #[test]
    fn test_severity_debug() {
        let debug = format!("{:?}", Severity::Info);
        assert!(debug.contains("Info"));
    }

    #[test]
    fn test_severity_partial_eq() {
        assert_eq!(Severity::Error, Severity::Error);
        assert_ne!(Severity::Error, Severity::Warning);
        assert_ne!(Severity::Warning, Severity::Info);
    }

    // === DepylintAnalyzer tests ===

    #[test]
    fn test_analyzer_new() {
        let analyzer = DepylintAnalyzer::new();
        assert!(analyzer.warnings.is_empty());
        assert!(analyzer.reported.is_empty());
    }

    #[test]
    fn test_analyzer_default() {
        let analyzer = DepylintAnalyzer::default();
        assert!(analyzer.warnings.is_empty());
    }

    #[test]
    fn test_analyzer_reuse() {
        let mut analyzer = DepylintAnalyzer::new();
        let w1 = analyzer.analyze("eval('1')");
        assert_eq!(w1.len(), 1);
        let w2 = analyzer.analyze("x = 1");
        assert!(w2.is_empty());
    }

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
    fn test_detect_locals() {
        let mut analyzer = DepylintAnalyzer::new();
        let warnings = analyzer.analyze("x = locals()");
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].code, "DPL005");
        assert_eq!(warnings[0].severity, Severity::Warning);
    }

    #[test]
    fn test_detect_setattr() {
        let mut analyzer = DepylintAnalyzer::new();
        let warnings = analyzer.analyze("setattr(obj, 'name', value)");
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].code, "DPL006");
    }

    #[test]
    fn test_detect_getattr_func() {
        let mut analyzer = DepylintAnalyzer::new();
        let warnings = analyzer.analyze("x = getattr(obj, 'name')");
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].code, "DPL006");
    }

    #[test]
    fn test_detect_delattr() {
        let mut analyzer = DepylintAnalyzer::new();
        let warnings = analyzer.analyze("delattr(obj, 'name')");
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].code, "DPL006");
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
    fn test_detect_setattr_dunder() {
        let mut analyzer = DepylintAnalyzer::new();
        let warnings = analyzer.analyze(
            r#"
class Foo:
    def __setattr__(self, name, value):
        pass
"#,
        );
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].code, "DPL009");
    }

    #[test]
    fn test_detect_delattr_dunder() {
        let mut analyzer = DepylintAnalyzer::new();
        let warnings = analyzer.analyze(
            r#"
class Foo:
    def __delattr__(self, name):
        pass
"#,
        );
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].code, "DPL010");
    }

    #[test]
    fn test_detect_getattribute_dunder() {
        let mut analyzer = DepylintAnalyzer::new();
        let warnings = analyzer.analyze(
            r#"
class Foo:
    def __getattribute__(self, name):
        return object.__getattribute__(self, name)
"#,
        );
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].code, "DPL011");
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
    fn test_type_single_arg_ok() {
        let mut analyzer = DepylintAnalyzer::new();
        let warnings = analyzer.analyze("t = type(obj)");
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_no_duplicate_warnings() {
        let mut analyzer = DepylintAnalyzer::new();
        // Same eval at same offset should not be duplicated
        let warnings = analyzer.analyze("eval('1')");
        assert_eq!(warnings.len(), 1);
    }

    #[test]
    fn test_multiple_different_warnings() {
        let mut analyzer = DepylintAnalyzer::new();
        let warnings = analyzer.analyze(
            r#"
eval('1')
exec('2')
globals()
"#,
        );
        assert_eq!(warnings.len(), 3);
    }

    #[test]
    fn test_nested_in_function() {
        let mut analyzer = DepylintAnalyzer::new();
        let warnings = analyzer.analyze(
            r#"
def foo():
    return eval('x')
"#,
        );
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].code, "DPL001");
    }

    #[test]
    fn test_nested_in_class() {
        let mut analyzer = DepylintAnalyzer::new();
        let warnings = analyzer.analyze(
            r#"
class Foo:
    def method(self):
        exec('pass')
"#,
        );
        assert_eq!(warnings.len(), 1);
    }

    #[test]
    fn test_in_if_statement() {
        let mut analyzer = DepylintAnalyzer::new();
        let warnings = analyzer.analyze(
            r#"
if True:
    eval('1')
"#,
        );
        assert_eq!(warnings.len(), 1);
    }

    #[test]
    fn test_in_for_loop() {
        let mut analyzer = DepylintAnalyzer::new();
        let warnings = analyzer.analyze(
            r#"
for i in range(10):
    eval(str(i))
"#,
        );
        assert_eq!(warnings.len(), 1);
    }

    #[test]
    fn test_in_while_loop() {
        let mut analyzer = DepylintAnalyzer::new();
        let warnings = analyzer.analyze(
            r#"
while True:
    exec('break')
"#,
        );
        assert_eq!(warnings.len(), 1);
    }

    #[test]
    fn test_in_try_block() {
        let mut analyzer = DepylintAnalyzer::new();
        let warnings = analyzer.analyze(
            r#"
try:
    eval('bad')
except:
    pass
"#,
        );
        assert_eq!(warnings.len(), 1);
    }

    #[test]
    fn test_in_with_statement() {
        let mut analyzer = DepylintAnalyzer::new();
        let warnings = analyzer.analyze(
            r#"
with open('f') as f:
    eval(f.read())
"#,
        );
        assert_eq!(warnings.len(), 1);
    }

    #[test]
    fn test_in_lambda() {
        let mut analyzer = DepylintAnalyzer::new();
        let warnings = analyzer.analyze("f = lambda x: eval(x)");
        assert_eq!(warnings.len(), 1);
    }

    #[test]
    fn test_in_list_comprehension() {
        let mut analyzer = DepylintAnalyzer::new();
        let warnings = analyzer.analyze("[eval(x) for x in items]");
        assert_eq!(warnings.len(), 1);
    }

    #[test]
    fn test_in_dict_comprehension() {
        let mut analyzer = DepylintAnalyzer::new();
        let warnings = analyzer.analyze("{k: eval(v) for k, v in items}");
        assert_eq!(warnings.len(), 1);
    }

    #[test]
    fn test_in_ternary() {
        let mut analyzer = DepylintAnalyzer::new();
        let warnings = analyzer.analyze("x = eval('1') if cond else 0");
        assert_eq!(warnings.len(), 1);
    }

    #[test]
    fn test_async_function() {
        let mut analyzer = DepylintAnalyzer::new();
        let warnings = analyzer.analyze(
            r#"
async def foo():
    return eval('x')
"#,
        );
        assert_eq!(warnings.len(), 1);
    }

    // === format_warnings tests ===

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
    fn test_format_warnings_no_suggestion() {
        let source = "exec('1')";
        let warnings = vec![LintWarning {
            offset: 0,
            code: "DPL002".to_string(),
            message: "exec".to_string(),
            severity: Severity::Error,
            suggestion: None,
        }];
        let output = format_warnings(&warnings, source, "file.py");
        assert!(!output.contains("suggestion:"));
    }

    #[test]
    fn test_format_warnings_empty() {
        let output = format_warnings(&[], "x = 1", "file.py");
        assert!(output.is_empty());
    }

    #[test]
    fn test_format_warnings_multiple() {
        let source = "eval('1')\nexec('2')";
        let warnings = vec![
            LintWarning {
                offset: 0,
                code: "DPL001".to_string(),
                message: "eval".to_string(),
                severity: Severity::Error,
                suggestion: None,
            },
            LintWarning {
                offset: 10,
                code: "DPL002".to_string(),
                message: "exec".to_string(),
                severity: Severity::Error,
                suggestion: None,
            },
        ];
        let output = format_warnings(&warnings, source, "test.py");
        assert!(output.contains("DPL001"));
        assert!(output.contains("DPL002"));
    }

    // === offset_to_line_col tests ===

    #[test]
    fn test_offset_to_line_col() {
        let source = "line1\nline2\nline3";
        assert_eq!(offset_to_line_col(source, 0), (1, 1)); // 'l' in line1
        assert_eq!(offset_to_line_col(source, 6), (2, 1)); // 'l' in line2
        assert_eq!(offset_to_line_col(source, 12), (3, 1)); // 'l' in line3
    }

    #[test]
    fn test_offset_to_line_col_empty() {
        assert_eq!(offset_to_line_col("", 0), (1, 1));
    }

    #[test]
    fn test_offset_to_line_col_single_line() {
        let source = "hello world";
        assert_eq!(offset_to_line_col(source, 0), (1, 1));
        assert_eq!(offset_to_line_col(source, 5), (1, 6));
    }

    #[test]
    fn test_offset_to_line_col_end_of_line() {
        let source = "abc\ndef";
        assert_eq!(offset_to_line_col(source, 3), (1, 4)); // newline char
        assert_eq!(offset_to_line_col(source, 4), (2, 1)); // 'd'
    }

    #[test]
    fn test_offset_to_line_col_beyond_end() {
        let source = "ab";
        assert_eq!(offset_to_line_col(source, 100), (1, 3));
    }

    // === Edge cases ===

    #[test]
    fn test_parse_error_graceful() {
        let mut analyzer = DepylintAnalyzer::new();
        let warnings = analyzer.analyze("def broken(");
        assert!(warnings.is_empty()); // Parse error, no warnings
    }

    #[test]
    fn test_empty_source() {
        let mut analyzer = DepylintAnalyzer::new();
        let warnings = analyzer.analyze("");
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_normal_dunder_ok() {
        let mut analyzer = DepylintAnalyzer::new();
        let warnings = analyzer.analyze(
            r#"
class Foo:
    def __init__(self):
        pass
    def __str__(self):
        return ""
"#,
        );
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_suggestions_present() {
        let mut analyzer = DepylintAnalyzer::new();
        let warnings = analyzer.analyze("eval('1')");
        assert!(warnings[0].suggestion.is_some());
    }

    #[test]
    fn test_offset_is_nonzero() {
        let mut analyzer = DepylintAnalyzer::new();
        let warnings = analyzer.analyze("x = 1\neval('2')");
        assert!(warnings[0].offset > 0);
    }
}
