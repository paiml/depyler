//! Pre-codegen analysis: validator detection and string optimization
//!
//! This module provides analysis passes that run BEFORE code generation begins.
//! It handles two concerns:
//!
//! 1. **String optimization analysis** - Scans all functions to determine optimal
//!    string representations (`String` vs `&str` vs `Cow<str>`).
//!
//! 2. **Validator function detection** (DEPYLER-0447) - Scans function bodies and
//!    constant expressions to find `add_argument(type=validator_func)` calls from
//!    argparse usage. This must run before function signature generation so that
//!    parameter types for validator functions can be corrected.

use super::context::CodeGenContext;
use crate::hir::*;

/// Analyze functions for string optimization
///
/// Performs string optimization analysis on all functions.
/// Complexity: 2 (well within <=10 target)
pub(super) fn analyze_string_optimization(ctx: &mut CodeGenContext, functions: &[HirFunction]) {
    for func in functions {
        ctx.string_optimizer.analyze_function(func);
    }
}

/// DEPYLER-0447: Analyze function bodies AND constants to find argparse validators
///
/// Scans all statements in function bodies and constant expressions to find
/// add_argument(type=validator_func) calls. Populates ctx.validator_functions
/// with function names used as type= parameters.
/// This must run BEFORE function signature generation so parameter types can be corrected.
///
/// Complexity: 8 (func loop + const loop + stmt loop + match + expr match + kwargs loop + filter)
pub(super) fn analyze_validators(
    ctx: &mut CodeGenContext,
    functions: &[HirFunction],
    constants: &[HirConstant],
) {
    // Scan function bodies
    for func in functions {
        scan_stmts_for_validators(&func.body, ctx);
    }

    // Scan constant expressions (module-level code)
    for constant in constants {
        scan_expr_for_validators(&constant.value, ctx);
    }
}

/// Helper: Recursively scan statements for add_argument(type=...) calls
pub(super) fn scan_stmts_for_validators(stmts: &[HirStmt], ctx: &mut CodeGenContext) {
    for stmt in stmts {
        match stmt {
            HirStmt::Expr(expr) => {
                scan_expr_for_validators(expr, ctx);
            }
            HirStmt::If { then_body, else_body, .. } => {
                scan_stmts_for_validators(then_body, ctx);
                if let Some(ref else_stmts) = else_body {
                    scan_stmts_for_validators(else_stmts, ctx);
                }
            }
            HirStmt::While { body, .. } => {
                scan_stmts_for_validators(body, ctx);
            }
            HirStmt::For { body, .. } => {
                scan_stmts_for_validators(body, ctx);
            }
            HirStmt::Try { body, handlers, orelse, finalbody } => {
                scan_stmts_for_validators(body, ctx);
                for handler in handlers {
                    scan_stmts_for_validators(&handler.body, ctx);
                }
                if let Some(ref else_stmts) = orelse {
                    scan_stmts_for_validators(else_stmts, ctx);
                }
                if let Some(ref final_stmts) = finalbody {
                    scan_stmts_for_validators(final_stmts, ctx);
                }
            }
            _ => {}
        }
    }
}

/// Helper: Scan expression for add_argument method calls
pub(super) fn scan_expr_for_validators(expr: &HirExpr, ctx: &mut CodeGenContext) {
    match expr {
        HirExpr::MethodCall { method, kwargs, .. } if method == "add_argument" => {
            // Check for type= parameter
            for (kw_name, kw_value) in kwargs {
                if kw_name == "type" {
                    if let HirExpr::Var(type_name) = kw_value {
                        // Skip built-in types
                        if !matches!(type_name.as_str(), "str" | "int" | "float" | "Path") {
                            ctx.validator_functions.insert(type_name.clone());
                        }
                    }
                }
            }
        }
        _ => {}
    }
}
