use crate::hir::{BinOp, FunctionProperties, HirExpr, HirStmt};

pub struct FunctionAnalyzer;

impl FunctionAnalyzer {
    pub fn analyze(body: &[HirStmt]) -> FunctionProperties {
        let (can_fail, error_types) = Self::check_can_fail(body);
        FunctionProperties {
            is_pure: Self::check_pure(body),
            always_terminates: Self::check_termination(body),
            panic_free: Self::check_panic_free(body),
            max_stack_depth: Self::calculate_max_stack_depth(body),
            can_fail,
            error_types,
            is_async: false, // Set by AST bridge when needed
            is_generator: Self::check_is_generator(body),
        }
    }

    fn check_pure(body: &[HirStmt]) -> bool {
        // V1: Conservative - only if no calls to unknown functions
        for stmt in body {
            if Self::has_side_effects(stmt) {
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
                then_body.iter().any(Self::has_side_effects)
                    || else_body
                        .as_ref()
                        .is_some_and(|b| b.iter().any(Self::has_side_effects))
            }
            HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
                body.iter().any(Self::has_side_effects)
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
                if !Self::is_finite_iterator(iter) {
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
            if Self::has_panic_risk(stmt) {
                return false;
            }
        }
        true
    }

    fn has_panic_risk(stmt: &HirStmt) -> bool {
        match stmt {
            HirStmt::Expr(expr) | HirStmt::Assign { value: expr, .. } => {
                Self::expr_has_panic_risk(expr)
            }
            HirStmt::Return(Some(expr)) => Self::expr_has_panic_risk(expr),
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                Self::expr_has_panic_risk(condition)
                    || then_body.iter().any(Self::has_panic_risk)
                    || else_body
                        .as_ref()
                        .is_some_and(|b| b.iter().any(Self::has_panic_risk))
            }
            HirStmt::While { condition, body } => {
                Self::expr_has_panic_risk(condition) || body.iter().any(Self::has_panic_risk)
            }
            HirStmt::For { iter, body, .. } => {
                Self::expr_has_panic_risk(iter) || body.iter().any(Self::has_panic_risk)
            }
            HirStmt::Raise { .. } => true, // Raise statements can fail
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
                Self::expr_has_panic_risk(left) || Self::expr_has_panic_risk(right)
            }
            HirExpr::Call { args, .. } => args.iter().any(Self::expr_has_panic_risk),
            _ => false,
        }
    }

    fn check_can_fail(body: &[HirStmt]) -> (bool, Vec<String>) {
        let mut error_types = Vec::new();
        let mut can_fail = false;

        for stmt in body {
            let (stmt_can_fail, mut stmt_errors) = Self::stmt_can_fail(stmt);
            if stmt_can_fail {
                can_fail = true;
            }
            // DEPYLER-0327 Fix #4: Always collect error types even if not can_fail
            // This ensures exception types used in try/except blocks are generated
            error_types.append(&mut stmt_errors);
        }

        // Remove duplicates
        error_types.sort();
        error_types.dedup();

        (can_fail, error_types)
    }

    fn stmt_can_fail(stmt: &HirStmt) -> (bool, Vec<String>) {
        match stmt {
            HirStmt::Raise { exception, .. } => {
                let error_type = Self::extract_exception_type(exception);
                (true, vec![error_type])
            }
            HirStmt::Expr(expr) | HirStmt::Assign { value: expr, .. } => Self::expr_can_fail(expr),
            HirStmt::Return(Some(expr)) => Self::expr_can_fail(expr),
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                let (cond_fail, cond_errors) = Self::expr_can_fail(condition);
                let (then_fail, mut then_errors) = Self::check_can_fail(then_body);
                let (else_fail, mut else_errors) = else_body
                    .as_ref()
                    .map(|b| Self::check_can_fail(b))
                    .unwrap_or((false, Vec::new()));

                let mut all_errors = cond_errors;
                all_errors.append(&mut then_errors);
                all_errors.append(&mut else_errors);

                (cond_fail || then_fail || else_fail, all_errors)
            }
            HirStmt::While { condition, body } => {
                let (cond_fail, cond_errors) = Self::expr_can_fail(condition);
                let (body_fail, mut body_errors) = Self::check_can_fail(body);

                let mut all_errors = cond_errors;
                all_errors.append(&mut body_errors);

                (cond_fail || body_fail, all_errors)
            }
            HirStmt::For { iter, body, .. } => {
                let (iter_fail, iter_errors) = Self::expr_can_fail(iter);
                let (body_fail, mut body_errors) = Self::check_can_fail(body);

                let mut all_errors = iter_errors;
                all_errors.append(&mut body_errors);

                (iter_fail || body_fail, all_errors)
            }
            // DEPYLER-0327 Fix #2: Analyze try/except blocks for error types
            // This ensures exception types used in try blocks are generated
            // even if they're caught internally (needed for type definitions)
            HirStmt::Try {
                body,
                handlers,
                finalbody,
                ..
            } => {
                // Collect error types from try body
                let (body_fail, mut body_errors) = Self::check_can_fail(body);

                // DEPYLER-0428: Track which exception types are CAUGHT vs RAISED
                let mut caught_exceptions: Vec<String> = Vec::new();
                let mut raised_in_handlers: Vec<String> = Vec::new();

                // Collect error types from except handlers
                let mut handler_errors = Vec::new();
                for handler in handlers {
                    // Track which exception this handler catches
                    if let Some(ref exc_type) = handler.exception_type {
                        caught_exceptions.push(exc_type.clone());
                        // DEPYLER-0327 Fix #3: Add exception types from handler signatures
                        // This ensures types like ValueError are generated even if only caught
                        body_errors.push(exc_type.clone());
                    }

                    // Track which exceptions are raised IN the handler body
                    let (handler_fail, mut h_errors) = Self::check_can_fail(&handler.body);
                    if handler_fail {
                        raised_in_handlers.append(&mut h_errors.clone());
                        handler_errors.append(&mut h_errors);
                    }
                }

                // Collect error types from finally block
                let finally_errors = if let Some(ref finally_body) = finalbody {
                    let (_, f_errors) = Self::check_can_fail(finally_body);
                    f_errors
                } else {
                    Vec::new()
                };

                let mut all_errors = body_errors;
                all_errors.append(&mut handler_errors);
                all_errors.extend(finally_errors);

                // DEPYLER-0428: Try block can_fail=true if handler raises UNCAUGHT exceptions
                // Example: try/except ValueError → raise ArgumentTypeError
                // - caught_exceptions = ["ValueError"]
                // - raised_in_handlers = ["ArgumentTypeError"]
                // - has_uncaught = true (ArgumentTypeError not in caught list)
                let has_uncaught_exceptions = raised_in_handlers
                    .iter()
                    .any(|raised| !caught_exceptions.contains(raised));

                // DEPYLER-0XXX: Result<> Inference Fix
                // If try body contains I/O operations (with open()), function must return Result<>
                // even if all exceptions are caught, because Rust version uses `?` operator
                // which propagates errors (unlike Python sys.exit() which terminates)
                let body_has_io = all_errors.iter().any(|e| e.contains("io::Error"));

                (
                    has_uncaught_exceptions || body_fail || body_has_io,
                    all_errors,
                )
            }
            // DEPYLER-0432: With statements using open() are fallible (file I/O)
            HirStmt::With { context, body, .. } => {
                // Check if context expression uses open() call
                let context_uses_open =
                    matches!(context, HirExpr::Call { func, .. } if func.as_str() == "open");

                let (context_fail, context_errors) = Self::expr_can_fail(context);
                let (body_fail, mut body_errors) = Self::check_can_fail(body);

                let mut all_errors = context_errors;
                all_errors.append(&mut body_errors);

                // File I/O operations can fail with IOError
                if context_uses_open {
                    all_errors.push("std::io::Error".to_string());
                }

                (context_uses_open || context_fail || body_fail, all_errors)
            }
            _ => (false, Vec::new()),
        }
    }

    fn expr_can_fail(expr: &HirExpr) -> (bool, Vec<String>) {
        match expr {
            HirExpr::Index { .. } => (true, vec!["IndexError".to_string()]),
            HirExpr::Binary {
                op: BinOp::Div | BinOp::FloorDiv | BinOp::Mod,
                ..
            } => (true, vec!["ZeroDivisionError".to_string()]),
            HirExpr::Call { func, args, .. } => {
                // DEPYLER-0217 FIX: Check if function can fail based on context
                // int() only fails when parsing strings, not when casting typed values
                let func_errors = match func.as_str() {
                    "int" => {
                        // Only mark as failable if parsing a string argument
                        // int(typed_value) → (value) as i32 (safe cast, cannot fail)
                        // int("123") → can fail with ValueError (parsing)
                        if args.len() == 1 {
                            match &args[0] {
                                // String literals being parsed can fail
                                HirExpr::Literal(crate::hir::Literal::String(_)) => {
                                    vec!["ValueError".to_string()]
                                }
                                // Variables or other expressions - safe cast, cannot fail
                                _ => Vec::new(),
                            }
                        } else if args.len() == 2 {
                            // int(string, base) - always can fail
                            vec!["ValueError".to_string()]
                        } else {
                            Vec::new()
                        }
                    }
                    // DEPYLER-0529: File I/O operations can fail with IOError
                    "open" => vec!["std::io::Error".to_string()],
                    _ => Vec::new(),
                };

                let (args_fail, mut args_errors) = Self::check_exprs_can_fail(args);
                let mut all_errors = func_errors.clone();
                all_errors.append(&mut args_errors);

                (!func_errors.is_empty() || args_fail, all_errors)
            }
            HirExpr::Binary { left, right, .. } => {
                let (left_fail, left_errors) = Self::expr_can_fail(left);
                let (right_fail, mut right_errors) = Self::expr_can_fail(right);

                let mut all_errors = left_errors;
                all_errors.append(&mut right_errors);

                (left_fail || right_fail, all_errors)
            }
            // DEPYLER-0624: Handle IfExpr (Python ternary) for can_fail analysis
            // Example: `out = sys.stdout if verbose else open("log.txt", "w")`
            // The open() in orelse can fail, so the whole expression can fail
            HirExpr::IfExpr { test, body, orelse } => {
                let (test_fail, test_errors) = Self::expr_can_fail(test);
                let (body_fail, mut body_errors) = Self::expr_can_fail(body);
                let (orelse_fail, mut orelse_errors) = Self::expr_can_fail(orelse);

                let mut all_errors = test_errors;
                all_errors.append(&mut body_errors);
                all_errors.append(&mut orelse_errors);

                (test_fail || body_fail || orelse_fail, all_errors)
            }
            _ => (false, Vec::new()),
        }
    }

    fn check_exprs_can_fail(exprs: &[HirExpr]) -> (bool, Vec<String>) {
        let mut can_fail = false;
        let mut all_errors = Vec::new();

        for expr in exprs {
            let (expr_fail, mut expr_errors) = Self::expr_can_fail(expr);
            if expr_fail {
                can_fail = true;
                all_errors.append(&mut expr_errors);
            }
        }

        (can_fail, all_errors)
    }

    fn extract_exception_type(exception: &Option<HirExpr>) -> String {
        match exception {
            Some(HirExpr::Call { func, .. }) => func.clone(),
            Some(HirExpr::Var(name)) => name.clone(),
            // DEPYLER-0428: Handle argparse.ArgumentTypeError pattern
            Some(HirExpr::MethodCall { method, .. }) => method.clone(),
            _ => "Exception".to_string(),
        }
    }

    fn calculate_max_stack_depth(body: &[HirStmt]) -> Option<usize> {
        // Simple estimation for V1
        Some(Self::estimate_stack_depth(body, 0))
    }

    fn estimate_stack_depth(body: &[HirStmt], current: usize) -> usize {
        body.iter().fold(current, |max_depth, stmt| {
            let stmt_depth = match stmt {
                HirStmt::If {
                    then_body,
                    else_body,
                    ..
                } => {
                    let then_depth = Self::estimate_stack_depth(then_body, current + 1);
                    let else_depth = else_body
                        .as_ref()
                        .map(|b| Self::estimate_stack_depth(b, current + 1))
                        .unwrap_or(current);
                    then_depth.max(else_depth)
                }
                HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
                    Self::estimate_stack_depth(body, current + 1)
                }
                HirStmt::Raise { .. } => current, // Raise doesn't add stack depth
                _ => current,
            };
            max_depth.max(stmt_depth)
        })
    }

    fn check_is_generator(body: &[HirStmt]) -> bool {
        body.iter().any(Self::stmt_has_yield)
    }

    fn stmt_has_yield(stmt: &HirStmt) -> bool {
        match stmt {
            HirStmt::Expr(expr)
            | HirStmt::Assign { value: expr, .. }
            | HirStmt::Return(Some(expr)) => Self::expr_has_yield(expr),
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => Self::check_if_for_yield(condition, then_body, else_body),
            HirStmt::While { condition, body }
            | HirStmt::For {
                iter: condition,
                body,
                ..
            } => Self::check_loop_for_yield(condition, body),
            HirStmt::Try {
                body,
                handlers,
                orelse,
                finalbody,
            } => Self::check_try_for_yield(body, handlers, orelse, finalbody),
            // DEPYLER-0561: Check for yield inside with statements (context managers)
            HirStmt::With { body, context, .. } => {
                Self::expr_has_yield(context) || body.iter().any(Self::stmt_has_yield)
            }
            _ => false,
        }
    }

    fn check_if_for_yield(
        condition: &HirExpr,
        then_body: &[HirStmt],
        else_body: &Option<Vec<HirStmt>>,
    ) -> bool {
        Self::expr_has_yield(condition)
            || then_body.iter().any(Self::stmt_has_yield)
            || else_body
                .as_ref()
                .is_some_and(|b| b.iter().any(Self::stmt_has_yield))
    }

    fn check_loop_for_yield(condition: &HirExpr, body: &[HirStmt]) -> bool {
        Self::expr_has_yield(condition) || body.iter().any(Self::stmt_has_yield)
    }

    fn check_try_for_yield(
        body: &[HirStmt],
        handlers: &[crate::hir::ExceptHandler],
        orelse: &Option<Vec<HirStmt>>,
        finalbody: &Option<Vec<HirStmt>>,
    ) -> bool {
        body.iter().any(Self::stmt_has_yield)
            || handlers
                .iter()
                .any(|h| h.body.iter().any(Self::stmt_has_yield))
            || orelse
                .as_ref()
                .is_some_and(|b| b.iter().any(Self::stmt_has_yield))
            || finalbody
                .as_ref()
                .is_some_and(|b| b.iter().any(Self::stmt_has_yield))
    }

    fn expr_has_yield(expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Yield { .. } => true,
            HirExpr::Binary { left, right, .. }
            | HirExpr::Index {
                base: left,
                index: right,
            } => Self::check_two_exprs_for_yield(left, right),
            HirExpr::Unary { operand, .. }
            | HirExpr::Attribute { value: operand, .. }
            | HirExpr::Await { value: operand }
            | HirExpr::Borrow { expr: operand, .. }
            | HirExpr::Lambda { body: operand, .. } => Self::expr_has_yield(operand),
            HirExpr::Call { args, .. }
            | HirExpr::List(args)
            | HirExpr::Tuple(args)
            | HirExpr::Set(args) => Self::check_exprs_for_yield(args),
            HirExpr::MethodCall { object, args, .. } => Self::check_method_for_yield(object, args),
            HirExpr::Dict(pairs) => Self::check_pairs_for_yield(pairs),
            HirExpr::ListComp {
                element,
                generators,
            }
            | HirExpr::SetComp {
                element,
                generators,
            } => Self::check_comp_for_yield(element, generators),
            _ => false,
        }
    }

    fn check_method_for_yield(object: &HirExpr, args: &[HirExpr]) -> bool {
        Self::expr_has_yield(object) || Self::check_exprs_for_yield(args)
    }

    fn check_two_exprs_for_yield(left: &HirExpr, right: &HirExpr) -> bool {
        Self::expr_has_yield(left) || Self::expr_has_yield(right)
    }

    fn check_exprs_for_yield(exprs: &[HirExpr]) -> bool {
        exprs.iter().any(Self::expr_has_yield)
    }

    fn check_pairs_for_yield(pairs: &[(HirExpr, HirExpr)]) -> bool {
        pairs
            .iter()
            .any(|(k, v)| Self::expr_has_yield(k) || Self::expr_has_yield(v))
    }

    fn check_comp_for_yield(
        element: &HirExpr,
        generators: &[crate::hir::HirComprehension],
    ) -> bool {
        // Check element for yield
        if Self::expr_has_yield(element) {
            return true;
        }
        // Check all generators (iter and conditions)
        for gen in generators {
            if Self::expr_has_yield(&gen.iter) {
                return true;
            }
            for cond in &gen.conditions {
                if Self::expr_has_yield(cond) {
                    return true;
                }
            }
        }
        false
    }
}
