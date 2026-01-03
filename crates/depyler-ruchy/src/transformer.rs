//! Pattern-based transformations for Pythonic to functional style

use crate::ast::{Literal, Param, PipelineStage, RuchyExpr, StringPart};
use anyhow::{anyhow, Result};

/// Pattern transformer for Pythonic constructs to Ruchy functional style
pub struct PatternTransformer {
    /// Enable list comprehension to pipeline transformation
    enable_pipeline_transform: bool,

    /// Enable string formatting to interpolation
    enable_string_interpolation: bool,

    /// Enable async/await to actor transformation
    #[allow(dead_code)]
    enable_actor_transform: bool,

    /// Enable DataFrame optimizations
    enable_dataframe_opt: bool,
}

impl PatternTransformer {
    /// Creates a new transformer with default settings
    #[must_use]
    pub fn new() -> Self {
        Self {
            enable_pipeline_transform: true,
            enable_string_interpolation: true,
            enable_actor_transform: false,
            enable_dataframe_opt: true,
        }
    }

    /// Creates transformer with custom configuration
    #[must_use]
    pub fn with_config(config: &crate::RuchyConfig) -> Self {
        Self {
            enable_pipeline_transform: config.use_pipelines,
            enable_string_interpolation: config.use_string_interpolation,
            enable_actor_transform: config.use_actors,
            enable_dataframe_opt: config.optimize_dataframes,
        }
    }

    /// Transform a Ruchy AST with pattern-based optimizations
    pub fn transform(&self, expr: RuchyExpr) -> Result<RuchyExpr> {
        self.transform_expr(expr)
    }

    /// Transform an expression recursively
    fn transform_expr(&self, expr: RuchyExpr) -> Result<RuchyExpr> {
        match expr {
            // Transform list comprehensions to pipelines
            RuchyExpr::Call { func, args } if self.is_list_comprehension(&func, &args) => {
                self.transform_list_comprehension(func, args)
            }

            // Transform string formatting to interpolation
            RuchyExpr::Call { func, args } if self.is_string_format(&func, &args) => {
                self.transform_string_format(func, args)
            }

            // Transform nested for loops to DataFrame operations
            RuchyExpr::For { var, iter, body } if self.enable_dataframe_opt => {
                self.check_dataframe_pattern(var, iter, body)
            }

            // Transform filter + map patterns to single pipeline
            RuchyExpr::Call { func, args } if self.is_filter_map_pattern(&func, &args) => {
                self.transform_filter_map(*func, args)
            }

            // Recursively transform nested expressions
            RuchyExpr::Block(exprs) => {
                let transformed = exprs
                    .into_iter()
                    .map(|e| self.transform_expr(e))
                    .collect::<Result<Vec<_>>>()?;
                Ok(RuchyExpr::Block(transformed))
            }

            RuchyExpr::If {
                condition,
                then_branch,
                else_branch,
            } => Ok(RuchyExpr::If {
                condition: Box::new(self.transform_expr(*condition)?),
                then_branch: Box::new(self.transform_expr(*then_branch)?),
                else_branch: else_branch
                    .map(|e| self.transform_expr(*e).map(Box::new))
                    .transpose()?,
            }),

            RuchyExpr::Function {
                name,
                params,
                body,
                is_async,
                return_type,
            } => Ok(RuchyExpr::Function {
                name,
                params,
                body: Box::new(self.transform_expr(*body)?),
                is_async,
                return_type,
            }),

            RuchyExpr::Lambda { params, body } => Ok(RuchyExpr::Lambda {
                params,
                body: Box::new(self.transform_expr(*body)?),
            }),

            RuchyExpr::For { var, iter, body } => Ok(RuchyExpr::For {
                var,
                iter: Box::new(self.transform_expr(*iter)?),
                body: Box::new(self.transform_expr(*body)?),
            }),

            RuchyExpr::While { condition, body } => Ok(RuchyExpr::While {
                condition: Box::new(self.transform_expr(*condition)?),
                body: Box::new(self.transform_expr(*body)?),
            }),

            RuchyExpr::Let {
                name,
                value,
                body,
                is_mutable,
            } => Ok(RuchyExpr::Let {
                name,
                value: Box::new(self.transform_expr(*value)?),
                body: Box::new(self.transform_expr(*body)?),
                is_mutable,
            }),

            // Pass through other expressions unchanged
            _ => Ok(expr),
        }
    }

    /// Check if a call is a list comprehension pattern
    fn is_list_comprehension(&self, func: &Box<RuchyExpr>, _args: &[RuchyExpr]) -> bool {
        if !self.enable_pipeline_transform {
            return false;
        }

        // Check for patterns like: [expr for x in iter if cond]
        matches!(**func, RuchyExpr::Identifier(ref name) if name == "list_comp")
    }

    /// Transform list comprehension to pipeline
    fn transform_list_comprehension(
        &self,
        _func: Box<RuchyExpr>,
        args: Vec<RuchyExpr>,
    ) -> Result<RuchyExpr> {
        // Extract comprehension parts
        let (element_expr, var_name, iterable, condition) = self.parse_comprehension(&args)?;

        let mut stages = Vec::new();

        // Add filter stage if there's a condition
        if let Some(cond) = condition {
            let filter_lambda = RuchyExpr::Lambda {
                params: vec![Param {
                    name: var_name.clone(),
                    typ: None,
                    default: None,
                }],
                body: Box::new(cond),
            };
            stages.push(PipelineStage::Filter(Box::new(filter_lambda)));
        }

        // Add map stage for transformation
        let map_lambda = RuchyExpr::Lambda {
            params: vec![Param {
                name: var_name,
                typ: None,
                default: None,
            }],
            body: Box::new(element_expr),
        };
        stages.push(PipelineStage::Map(Box::new(map_lambda)));

        // Add collect stage to create list
        stages.push(PipelineStage::Call("collect".to_string(), vec![]));

        Ok(RuchyExpr::Pipeline {
            expr: Box::new(iterable),
            stages,
        })
    }

    /// Parse comprehension arguments
    fn parse_comprehension(
        &self,
        args: &[RuchyExpr],
    ) -> Result<(RuchyExpr, String, RuchyExpr, Option<RuchyExpr>)> {
        if args.len() < 3 {
            return Err(anyhow!("Invalid comprehension structure"));
        }

        let element = args[0].clone();

        let var_name = match &args[1] {
            RuchyExpr::Identifier(name) => name.clone(),
            _ => return Err(anyhow!("Expected variable name in comprehension")),
        };

        let iterable = args[2].clone();
        let condition = args.get(3).cloned();

        Ok((element, var_name, iterable, condition))
    }

    /// Check if a call is string formatting
    fn is_string_format(&self, func: &RuchyExpr, _args: &[RuchyExpr]) -> bool {
        if !self.enable_string_interpolation {
            return false;
        }

        matches!(
            func,
            RuchyExpr::FieldAccess {
                field: ref method,
                ..
            } if method == "format"
        )
    }

    /// Transform string format to interpolation
    fn transform_string_format(
        &self,
        func: Box<RuchyExpr>,
        args: Vec<RuchyExpr>,
    ) -> Result<RuchyExpr> {
        // Extract the format string
        let format_str = match &*func {
            RuchyExpr::FieldAccess { object, .. } => match &**object {
                RuchyExpr::Literal(Literal::String(s)) => s.clone(),
                _ => return Ok(RuchyExpr::Call { func, args }),
            },
            _ => return Ok(RuchyExpr::Call { func, args }),
        };

        // Parse format string and create interpolation parts
        let parts = self.parse_format_string(&format_str, &args)?;

        Ok(RuchyExpr::StringInterpolation { parts })
    }

    /// Parse format string into interpolation parts
    fn parse_format_string(&self, format_str: &str, args: &[RuchyExpr]) -> Result<Vec<StringPart>> {
        let mut parts = Vec::new();
        let mut current = String::new();
        let mut arg_index = 0;
        let mut chars = format_str.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '{' {
                if chars.peek() == Some(&'{') {
                    // Escaped brace
                    chars.next();
                    current.push('{');
                } else {
                    // Start of format placeholder
                    if !current.is_empty() {
                        parts.push(StringPart::Text(current.clone()));
                        current.clear();
                    }

                    // Find closing brace
                    let mut placeholder = String::new();
                    for ch in chars.by_ref() {
                        if ch == '}' {
                            break;
                        }
                        placeholder.push(ch);
                    }

                    // Get corresponding argument
                    if arg_index < args.len() {
                        parts.push(StringPart::Expr(Box::new(args[arg_index].clone())));
                        arg_index += 1;
                    }
                }
            } else if ch == '}' {
                if chars.peek() == Some(&'}') {
                    // Escaped brace
                    chars.next();
                    current.push('}');
                } else {
                    current.push(ch);
                }
            } else {
                current.push(ch);
            }
        }

        if !current.is_empty() {
            parts.push(StringPart::Text(current));
        }

        Ok(parts)
    }

    /// Check if expression matches filter + map pattern
    fn is_filter_map_pattern(&self, func: &RuchyExpr, args: &[RuchyExpr]) -> bool {
        if !self.enable_pipeline_transform {
            return false;
        }

        // Check for map(filter(...)) pattern
        matches!(
            func,
            RuchyExpr::Identifier(ref name) if name == "map"
        ) && args.len() == 2
            && matches!(
                args[1],
                RuchyExpr::Call {
                    func: ref inner_func,
                    ..
                } if matches!(**inner_func, RuchyExpr::Identifier(ref name) if name == "filter")
            )
    }

    /// Transform filter + map to single pipeline
    fn transform_filter_map(&self, _func: RuchyExpr, args: Vec<RuchyExpr>) -> Result<RuchyExpr> {
        if args.len() != 2 {
            return Ok(RuchyExpr::Call {
                func: Box::new(RuchyExpr::Identifier("map".to_string())),
                args,
            });
        }

        let map_fn = args[0].clone();

        // Extract filter call
        let (filter_fn, source) = match &args[1] {
            RuchyExpr::Call {
                args: filter_args, ..
            } if filter_args.len() == 2 => (filter_args[0].clone(), filter_args[1].clone()),
            _ => {
                return Ok(RuchyExpr::Call {
                    func: Box::new(RuchyExpr::Identifier("map".to_string())),
                    args,
                })
            }
        };

        // Create pipeline
        Ok(RuchyExpr::Pipeline {
            expr: Box::new(source),
            stages: vec![
                PipelineStage::Filter(Box::new(filter_fn)),
                PipelineStage::Map(Box::new(map_fn)),
            ],
        })
    }

    /// Check if for loop can be converted to DataFrame operation
    fn check_dataframe_pattern(
        &self,
        var: String,
        iter: Box<RuchyExpr>,
        body: Box<RuchyExpr>,
    ) -> Result<RuchyExpr> {
        // Check if body contains DataFrame-like operations
        if self.is_dataframe_operation(&body) {
            self.transform_to_dataframe(var, *iter, *body)
        } else {
            Ok(RuchyExpr::For { var, iter, body })
        }
    }

    /// Check if expression is a DataFrame-like operation
    #[allow(clippy::only_used_in_recursion)]
    fn is_dataframe_operation(&self, expr: &RuchyExpr) -> bool {
        match expr {
            RuchyExpr::Block(exprs) => exprs.iter().any(|e| self.is_dataframe_operation(e)),

            RuchyExpr::Call { func, .. } => match &**func {
                RuchyExpr::FieldAccess { field, .. } => {
                    matches!(
                        field.as_str(),
                        "append" | "sum" | "mean" | "groupby" | "aggregate" | "sort"
                    )
                }
                _ => false,
            },

            _ => false,
        }
    }

    /// Transform loop to DataFrame operation
    fn transform_to_dataframe(
        &self,
        _var: String,
        iter: RuchyExpr,
        body: RuchyExpr,
    ) -> Result<RuchyExpr> {
        // Extract operations from body
        let operations = self.extract_dataframe_ops(&body)?;

        // Create DataFrame from iterator
        let df_expr = RuchyExpr::Call {
            func: Box::new(RuchyExpr::Identifier("DataFrame".to_string())),
            args: vec![RuchyExpr::Call {
                func: Box::new(RuchyExpr::Identifier("from_iter".to_string())),
                args: vec![iter],
            }],
        };

        // Apply operations as pipeline
        let mut result = df_expr;
        for op in operations {
            result = RuchyExpr::MethodCall {
                receiver: Box::new(result),
                method: op.method,
                args: op.args,
            };
        }

        Ok(result)
    }

    /// Extract DataFrame operations from expression
    #[allow(clippy::only_used_in_recursion)]
    fn extract_dataframe_ops(&self, expr: &RuchyExpr) -> Result<Vec<DataFrameOp>> {
        let mut ops = Vec::new();

        match expr {
            RuchyExpr::Block(exprs) => {
                for e in exprs {
                    ops.extend(self.extract_dataframe_ops(e)?);
                }
            }

            RuchyExpr::Call { func, args } => {
                if let RuchyExpr::FieldAccess { field, .. } = &**func {
                    ops.push(DataFrameOp {
                        method: field.clone(),
                        args: args.clone(),
                    });
                }
            }

            _ => {}
        }

        Ok(ops)
    }
}

/// DataFrame operation descriptor
struct DataFrameOp {
    method: String,
    args: Vec<RuchyExpr>,
}

impl Default for PatternTransformer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{BinaryOp, UnaryOp};

    #[test]
    fn test_transformer_new() {
        let t = PatternTransformer::new();
        assert!(t.enable_pipeline_transform);
        assert!(t.enable_string_interpolation);
        assert!(t.enable_dataframe_opt);
    }

    #[test]
    fn test_transformer_default() {
        let t = PatternTransformer::default();
        assert!(t.enable_pipeline_transform);
    }

    #[test]
    fn test_transformer_with_config() {
        use crate::RuchyConfig;
        let config = RuchyConfig {
            use_pipelines: false,
            use_string_interpolation: false,
            use_actors: true,
            optimize_dataframes: false,
            ..Default::default()
        };
        let t = PatternTransformer::with_config(&config);
        assert!(!t.enable_pipeline_transform);
        assert!(!t.enable_string_interpolation);
    }

    #[test]
    fn test_transform_block() {
        let transformer = PatternTransformer::new();
        let block = RuchyExpr::Block(vec![
            RuchyExpr::Literal(Literal::Integer(1)),
            RuchyExpr::Literal(Literal::Integer(2)),
        ]);
        let result = transformer.transform(block).unwrap();
        assert!(matches!(result, RuchyExpr::Block(_)));
    }

    #[test]
    fn test_transform_if_expr() {
        let transformer = PatternTransformer::new();
        let if_expr = RuchyExpr::If {
            condition: Box::new(RuchyExpr::Literal(Literal::Bool(true))),
            then_branch: Box::new(RuchyExpr::Literal(Literal::Integer(1))),
            else_branch: Some(Box::new(RuchyExpr::Literal(Literal::Integer(2)))),
        };
        let result = transformer.transform(if_expr).unwrap();
        assert!(matches!(result, RuchyExpr::If { .. }));
    }

    #[test]
    fn test_transform_if_no_else() {
        let transformer = PatternTransformer::new();
        let if_expr = RuchyExpr::If {
            condition: Box::new(RuchyExpr::Literal(Literal::Bool(true))),
            then_branch: Box::new(RuchyExpr::Literal(Literal::Integer(1))),
            else_branch: None,
        };
        let result = transformer.transform(if_expr).unwrap();
        assert!(matches!(result, RuchyExpr::If { else_branch: None, .. }));
    }

    #[test]
    fn test_transform_function() {
        let transformer = PatternTransformer::new();
        let func = RuchyExpr::Function {
            name: "test".to_string(),
            params: vec![],
            body: Box::new(RuchyExpr::Literal(Literal::Integer(42))),
            is_async: false,
            return_type: None,
        };
        let result = transformer.transform(func).unwrap();
        assert!(matches!(result, RuchyExpr::Function { .. }));
    }

    #[test]
    fn test_transform_binary() {
        let transformer = PatternTransformer::new();
        let binary = RuchyExpr::Binary {
            left: Box::new(RuchyExpr::Literal(Literal::Integer(1))),
            op: BinaryOp::Add,
            right: Box::new(RuchyExpr::Literal(Literal::Integer(2))),
        };
        let result = transformer.transform(binary).unwrap();
        assert!(matches!(result, RuchyExpr::Binary { .. }));
    }

    #[test]
    fn test_transform_unary() {
        let transformer = PatternTransformer::new();
        let unary = RuchyExpr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(RuchyExpr::Literal(Literal::Bool(false))),
        };
        let result = transformer.transform(unary).unwrap();
        assert!(matches!(result, RuchyExpr::Unary { .. }));
    }

    #[test]
    fn test_transform_lambda() {
        let transformer = PatternTransformer::new();
        let lambda = RuchyExpr::Lambda {
            params: vec![Param { name: "x".to_string(), typ: None, default: None }],
            body: Box::new(RuchyExpr::Identifier("x".to_string())),
        };
        let result = transformer.transform(lambda).unwrap();
        assert!(matches!(result, RuchyExpr::Lambda { .. }));
    }

    #[test]
    fn test_transform_list() {
        let transformer = PatternTransformer::new();
        let list = RuchyExpr::List(vec![
            RuchyExpr::Literal(Literal::Integer(1)),
            RuchyExpr::Literal(Literal::Integer(2)),
        ]);
        let result = transformer.transform(list).unwrap();
        assert!(matches!(result, RuchyExpr::List(_)));
    }

    #[test]
    fn test_transform_method_call() {
        let transformer = PatternTransformer::new();
        let call = RuchyExpr::MethodCall {
            receiver: Box::new(RuchyExpr::Identifier("obj".to_string())),
            method: "foo".to_string(),
            args: vec![],
        };
        let result = transformer.transform(call).unwrap();
        assert!(matches!(result, RuchyExpr::MethodCall { .. }));
    }

    #[test]
    fn test_transform_match() {
        let transformer = PatternTransformer::new();
        let m = RuchyExpr::Match {
            expr: Box::new(RuchyExpr::Literal(Literal::Integer(1))),
            arms: vec![],
        };
        let result = transformer.transform(m).unwrap();
        assert!(matches!(result, RuchyExpr::Match { .. }));
    }

    #[test]
    fn test_transform_while() {
        let transformer = PatternTransformer::new();
        let w = RuchyExpr::While {
            condition: Box::new(RuchyExpr::Literal(Literal::Bool(true))),
            body: Box::new(RuchyExpr::Literal(Literal::Unit)),
        };
        let result = transformer.transform(w).unwrap();
        assert!(matches!(result, RuchyExpr::While { .. }));
    }

    #[test]
    fn test_transform_let() {
        let transformer = PatternTransformer::new();
        let l = RuchyExpr::Let {
            name: "x".to_string(),
            value: Box::new(RuchyExpr::Literal(Literal::Integer(1))),
            body: Box::new(RuchyExpr::Identifier("x".to_string())),
            is_mutable: false,
        };
        let result = transformer.transform(l).unwrap();
        assert!(matches!(result, RuchyExpr::Let { .. }));
    }

    #[test]
    fn test_pipeline_transformation() {
        let transformer = PatternTransformer::new();

        let comp_expr = RuchyExpr::Call {
            func: Box::new(RuchyExpr::Identifier("list_comp".to_string())),
            args: vec![
                RuchyExpr::Binary {
                    left: Box::new(RuchyExpr::Identifier("x".to_string())),
                    op: BinaryOp::Multiply,
                    right: Box::new(RuchyExpr::Literal(Literal::Integer(2))),
                },
                RuchyExpr::Identifier("x".to_string()),
                RuchyExpr::List(vec![
                    RuchyExpr::Literal(Literal::Integer(1)),
                    RuchyExpr::Literal(Literal::Integer(2)),
                    RuchyExpr::Literal(Literal::Integer(3)),
                ]),
                RuchyExpr::Binary {
                    left: Box::new(RuchyExpr::Identifier("x".to_string())),
                    op: BinaryOp::Greater,
                    right: Box::new(RuchyExpr::Literal(Literal::Integer(1))),
                },
            ],
        };

        let result = transformer.transform(comp_expr).unwrap();
        assert!(matches!(result, RuchyExpr::Pipeline { .. }));
    }

    #[test]
    fn test_string_interpolation_transformation() {
        let transformer = PatternTransformer::new();

        let format_expr = RuchyExpr::Call {
            func: Box::new(RuchyExpr::FieldAccess {
                object: Box::new(RuchyExpr::Literal(Literal::String(
                    "Hello, {}!".to_string(),
                ))),
                field: "format".to_string(),
            }),
            args: vec![RuchyExpr::Identifier("name".to_string())],
        };

        let result = transformer.transform(format_expr).unwrap();
        assert!(matches!(result, RuchyExpr::StringInterpolation { .. }));
    }
}
