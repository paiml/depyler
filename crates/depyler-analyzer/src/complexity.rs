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
        HirExpr::Attribute { value, .. } => cognitive_condition(value),
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

#[cfg(test)]
mod tests {
    use super::*;
    use depyler_core::hir::{HirExpr, HirStmt, Literal};

    #[test]
    fn test_cyclomatic_simple_function() {
        // Function with no control flow should have complexity 1
        let body = vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42))))];
        assert_eq!(calculate_cyclomatic(&body), 1);
    }

    #[test]
    fn test_cyclomatic_if_statement() {
        // Function with if statement should have complexity 2
        let body = vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(1))))],
            else_body: None,
        }];
        assert_eq!(calculate_cyclomatic(&body), 2);
    }

    #[test]
    fn test_cyclomatic_if_else_statement() {
        // Function with if-else should have complexity 2
        let body = vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(1))))],
            else_body: Some(vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(
                2,
            ))))]),
        }];
        assert_eq!(calculate_cyclomatic(&body), 2);
    }

    #[test]
    fn test_cyclomatic_while_loop() {
        // Function with while loop should have complexity 2
        let body = vec![HirStmt::While {
            condition: HirExpr::Literal(Literal::Bool(true)),
            body: vec![HirStmt::Return(None)],
        }];
        assert_eq!(calculate_cyclomatic(&body), 2);
    }

    #[test]
    fn test_cyclomatic_for_loop() {
        // Function with for loop should have complexity 2
        let body = vec![HirStmt::For {
            target: "i".to_string(),
            iter: HirExpr::Literal(Literal::Int(0)),
            body: vec![HirStmt::Return(None)],
        }];
        assert_eq!(calculate_cyclomatic(&body), 2);
    }

    #[test]
    fn test_cyclomatic_logical_operators() {
        // Binary logical operators should increase complexity
        let condition = HirExpr::Binary {
            op: BinOp::And,
            left: Box::new(HirExpr::Literal(Literal::Bool(true))),
            right: Box::new(HirExpr::Literal(Literal::Bool(false))),
        };

        let body = vec![HirStmt::Expr(condition)];
        assert_eq!(calculate_cyclomatic(&body), 2); // 1 base + 1 for And
    }

    #[test]
    fn test_cognitive_simple_function() {
        // Simple function should have cognitive complexity 0
        let body = vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42))))];
        assert_eq!(calculate_cognitive(&body), 0);
    }

    #[test]
    fn test_cognitive_nested_if() {
        // Nested if statements should have higher cognitive complexity
        let nested_if = HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(2))))],
            else_body: None,
        };

        let body = vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![nested_if],
            else_body: None,
        }];

        // First if: 1 + 0 (nesting), nested if: 1 + 1 (nesting) = 3
        assert_eq!(calculate_cognitive(&body), 3);
    }

    #[test]
    fn test_cognitive_logical_operators() {
        let complex_condition = HirExpr::Binary {
            op: BinOp::And,
            left: Box::new(HirExpr::Binary {
                op: BinOp::Or,
                left: Box::new(HirExpr::Literal(Literal::Bool(true))),
                right: Box::new(HirExpr::Literal(Literal::Bool(false))),
            }),
            right: Box::new(HirExpr::Literal(Literal::Bool(true))),
        };

        let body = vec![HirStmt::If {
            condition: complex_condition,
            then_body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(1))))],
            else_body: None,
        }];

        // If: 1, condition: 2 (for And and Or) = 3
        assert_eq!(calculate_cognitive(&body), 3);
    }

    #[test]
    fn test_count_statements() {
        let body = vec![
            HirStmt::Assign {
                target: depyler_core::hir::AssignTarget::Symbol("x".to_string()),
                value: HirExpr::Literal(Literal::Int(1)),
                type_annotation: None,
            },
            HirStmt::If {
                condition: HirExpr::Literal(Literal::Bool(true)),
                then_body: vec![
                    HirStmt::Return(Some(HirExpr::Literal(Literal::Int(1)))),
                    HirStmt::Return(None),
                ],
                else_body: Some(vec![HirStmt::Return(None)]),
            },
        ];
        // 2 top-level + 2 in then + 1 in else = 5
        assert_eq!(count_statements(&body), 5);
    }

    #[test]
    fn test_max_nesting() {
        let deeply_nested = vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::While {
                condition: HirExpr::Literal(Literal::Bool(true)),
                body: vec![HirStmt::For {
                    target: "i".to_string(),
                    iter: HirExpr::Literal(Literal::Int(0)),
                    body: vec![HirStmt::Return(None)],
                }],
            }],
            else_body: None,
        }];
        // if (1) -> while (2) -> for (3) = max nesting 3
        assert_eq!(calculate_max_nesting(&deeply_nested), 3);
    }
}
