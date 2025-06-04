use depyler_core::hir::{BinOp, HirExpr, HirStmt};

pub fn calculate_cyclomatic(body: &[HirStmt]) -> u32 {
    // Start with 1 for the function entry point
    let mut complexity = 1;

    for stmt in body {
        complexity += cyclomatic_stmt(stmt);
    }

    complexity
}

fn cyclomatic_stmt(stmt: &HirStmt) -> u32 {
    match stmt {
        HirStmt::If {
            then_body,
            else_body,
            ..
        } => {
            let mut complexity = 1; // +1 for the if condition
            complexity += cyclomatic_body(then_body);
            if let Some(else_stmts) = else_body {
                complexity += cyclomatic_body(else_stmts);
            }
            complexity
        }
        HirStmt::While { body, .. } => {
            1 + cyclomatic_body(body) // +1 for the loop condition
        }
        HirStmt::For { body, .. } => {
            1 + cyclomatic_body(body) // +1 for the loop
        }
        HirStmt::Expr(expr) => cyclomatic_expr(expr),
        _ => 0,
    }
}

fn cyclomatic_body(body: &[HirStmt]) -> u32 {
    body.iter().map(cyclomatic_stmt).sum()
}

fn cyclomatic_expr(expr: &HirExpr) -> u32 {
    match expr {
        HirExpr::Binary {
            op: BinOp::And | BinOp::Or,
            left,
            right,
        } => 1 + cyclomatic_expr(left) + cyclomatic_expr(right),
        _ => 0,
    }
}

pub fn calculate_cognitive(body: &[HirStmt]) -> u32 {
    cognitive_body(body, 0).0
}

fn cognitive_body(body: &[HirStmt], nesting: u32) -> (u32, u32) {
    let mut total_complexity = 0;
    let mut max_nesting = nesting;

    for stmt in body {
        let (stmt_complexity, stmt_nesting) = cognitive_stmt(stmt, nesting);
        total_complexity += stmt_complexity;
        max_nesting = max_nesting.max(stmt_nesting);
    }

    (total_complexity, max_nesting)
}

fn cognitive_stmt(stmt: &HirStmt, nesting: u32) -> (u32, u32) {
    match stmt {
        HirStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            let mut complexity = 1 + nesting; // Base complexity + nesting increment
            let mut max_nesting = nesting;

            // Add complexity for conditions
            complexity += cognitive_condition(condition);

            // Process then branch
            let (then_complexity, then_nesting) = cognitive_body(then_body, nesting + 1);
            complexity += then_complexity;
            max_nesting = max_nesting.max(then_nesting);

            // Process else branch if present
            if let Some(else_stmts) = else_body {
                complexity += 1; // +1 for else
                let (else_complexity, else_nesting) = cognitive_body(else_stmts, nesting + 1);
                complexity += else_complexity;
                max_nesting = max_nesting.max(else_nesting);
            }

            (complexity, max_nesting)
        }
        HirStmt::While { condition, body } => {
            let mut complexity = 1 + nesting;
            complexity += cognitive_condition(condition);

            let (body_complexity, body_nesting) = cognitive_body(body, nesting + 1);
            complexity += body_complexity;

            (complexity, body_nesting)
        }
        HirStmt::For { body, .. } => {
            let complexity = 1 + nesting;
            let (body_complexity, body_nesting) = cognitive_body(body, nesting + 1);

            (complexity + body_complexity, body_nesting)
        }
        _ => (0, nesting),
    }
}

fn cognitive_condition(expr: &HirExpr) -> u32 {
    match expr {
        HirExpr::Binary {
            op: BinOp::And | BinOp::Or,
            left,
            right,
        } => 1 + cognitive_condition(left) + cognitive_condition(right),
        HirExpr::Unary { operand, .. } => cognitive_condition(operand),
        _ => 0,
    }
}

pub fn calculate_max_nesting(body: &[HirStmt]) -> usize {
    cognitive_body(body, 0).1 as usize
}

pub fn count_statements(body: &[HirStmt]) -> usize {
    let mut count = 0;

    for stmt in body {
        count += 1;
        count += match stmt {
            HirStmt::If {
                then_body,
                else_body,
                ..
            } => {
                count_statements(then_body) + else_body.as_ref().map_or(0, |b| count_statements(b))
            }
            HirStmt::While { body, .. } | HirStmt::For { body, .. } => count_statements(body),
            _ => 0,
        };
    }

    count
}
