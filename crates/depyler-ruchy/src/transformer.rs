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
        assert!(matches!(
            result,
            RuchyExpr::If {
                else_branch: None,
                ..
            }
        ));
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
            params: vec![Param {
                name: "x".to_string(),
                typ: None,
                default: None,
            }],
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

    // ========================================================================
    // Additional tests for better coverage
    // ========================================================================

    #[test]
    fn test_parse_format_string_empty() {
        let transformer = PatternTransformer::new();
        let parts = transformer.parse_format_string("", &[]).unwrap();
        assert!(parts.is_empty());
    }

    #[test]
    fn test_parse_format_string_no_placeholders() {
        let transformer = PatternTransformer::new();
        let parts = transformer
            .parse_format_string("Hello, world!", &[])
            .unwrap();
        assert_eq!(parts.len(), 1);
        assert!(matches!(&parts[0], StringPart::Text(s) if s == "Hello, world!"));
    }

    #[test]
    fn test_parse_format_string_multiple_placeholders() {
        let transformer = PatternTransformer::new();
        let args = vec![
            RuchyExpr::Identifier("a".to_string()),
            RuchyExpr::Identifier("b".to_string()),
        ];
        let parts = transformer.parse_format_string("{} and {}", &args).unwrap();
        assert_eq!(parts.len(), 3); // expr, " and ", expr
    }

    #[test]
    fn test_parse_format_string_escaped_braces() {
        let transformer = PatternTransformer::new();
        let parts = transformer.parse_format_string("{{literal}}", &[]).unwrap();
        assert_eq!(parts.len(), 1);
        assert!(matches!(&parts[0], StringPart::Text(s) if s == "{literal}"));
    }

    #[test]
    fn test_parse_format_string_escaped_closing_braces() {
        let transformer = PatternTransformer::new();
        let parts = transformer.parse_format_string("end}}", &[]).unwrap();
        assert_eq!(parts.len(), 1);
        assert!(matches!(&parts[0], StringPart::Text(s) if s == "end}"));
    }

    #[test]
    fn test_parse_format_string_mixed() {
        let transformer = PatternTransformer::new();
        let args = vec![RuchyExpr::Literal(Literal::Integer(42))];
        let parts = transformer
            .parse_format_string("Value: {} end", &args)
            .unwrap();
        assert_eq!(parts.len(), 3);
        assert!(matches!(&parts[0], StringPart::Text(s) if s == "Value: "));
        assert!(matches!(&parts[1], StringPart::Expr(_)));
        assert!(matches!(&parts[2], StringPart::Text(s) if s == " end"));
    }

    #[test]
    fn test_is_list_comprehension_disabled() {
        let config = crate::RuchyConfig {
            use_pipelines: false,
            ..Default::default()
        };
        let transformer = PatternTransformer::with_config(&config);
        let func = Box::new(RuchyExpr::Identifier("list_comp".to_string()));
        assert!(!transformer.is_list_comprehension(&func, &[]));
    }

    #[test]
    fn test_is_list_comprehension_wrong_name() {
        let transformer = PatternTransformer::new();
        let func = Box::new(RuchyExpr::Identifier("not_list_comp".to_string()));
        assert!(!transformer.is_list_comprehension(&func, &[]));
    }

    #[test]
    fn test_is_string_format_disabled() {
        let config = crate::RuchyConfig {
            use_string_interpolation: false,
            ..Default::default()
        };
        let transformer = PatternTransformer::with_config(&config);
        let func = RuchyExpr::FieldAccess {
            object: Box::new(RuchyExpr::Literal(Literal::String("test".to_string()))),
            field: "format".to_string(),
        };
        assert!(!transformer.is_string_format(&func, &[]));
    }

    #[test]
    fn test_is_string_format_wrong_method() {
        let transformer = PatternTransformer::new();
        let func = RuchyExpr::FieldAccess {
            object: Box::new(RuchyExpr::Literal(Literal::String("test".to_string()))),
            field: "upper".to_string(),
        };
        assert!(!transformer.is_string_format(&func, &[]));
    }

    #[test]
    fn test_is_filter_map_pattern_disabled() {
        let config = crate::RuchyConfig {
            use_pipelines: false,
            ..Default::default()
        };
        let transformer = PatternTransformer::with_config(&config);
        assert!(!transformer.is_filter_map_pattern(&RuchyExpr::Identifier("map".to_string()), &[]));
    }

    #[test]
    fn test_is_filter_map_pattern_wrong_args_count() {
        let transformer = PatternTransformer::new();
        let func = RuchyExpr::Identifier("map".to_string());
        let args = vec![RuchyExpr::Identifier("x".to_string())];
        assert!(!transformer.is_filter_map_pattern(&func, &args));
    }

    #[test]
    fn test_is_filter_map_pattern_not_filter() {
        let transformer = PatternTransformer::new();
        let func = RuchyExpr::Identifier("map".to_string());
        let args = vec![
            RuchyExpr::Lambda {
                params: vec![],
                body: Box::new(RuchyExpr::Identifier("x".to_string())),
            },
            RuchyExpr::List(vec![]),
        ];
        assert!(!transformer.is_filter_map_pattern(&func, &args));
    }

    #[test]
    fn test_parse_comprehension_too_few_args() {
        let transformer = PatternTransformer::new();
        let args = vec![
            RuchyExpr::Identifier("x".to_string()),
            RuchyExpr::Identifier("y".to_string()),
        ];
        let result = transformer.parse_comprehension(&args);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_comprehension_invalid_var() {
        let transformer = PatternTransformer::new();
        let args = vec![
            RuchyExpr::Literal(Literal::Integer(1)),
            RuchyExpr::Literal(Literal::Integer(2)), // Not an identifier
            RuchyExpr::List(vec![]),
        ];
        let result = transformer.parse_comprehension(&args);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_comprehension_with_condition() {
        let transformer = PatternTransformer::new();
        let args = vec![
            RuchyExpr::Identifier("x".to_string()),
            RuchyExpr::Identifier("i".to_string()),
            RuchyExpr::List(vec![]),
            RuchyExpr::Literal(Literal::Bool(true)), // condition
        ];
        let result = transformer.parse_comprehension(&args).unwrap();
        assert!(result.3.is_some());
    }

    #[test]
    fn test_parse_comprehension_no_condition() {
        let transformer = PatternTransformer::new();
        let args = vec![
            RuchyExpr::Identifier("x".to_string()),
            RuchyExpr::Identifier("i".to_string()),
            RuchyExpr::List(vec![]),
        ];
        let result = transformer.parse_comprehension(&args).unwrap();
        assert!(result.3.is_none());
    }

    #[test]
    fn test_is_dataframe_operation_block_true() {
        let transformer = PatternTransformer::new();
        let expr = RuchyExpr::Block(vec![RuchyExpr::Call {
            func: Box::new(RuchyExpr::FieldAccess {
                object: Box::new(RuchyExpr::Identifier("df".to_string())),
                field: "append".to_string(),
            }),
            args: vec![],
        }]);
        assert!(transformer.is_dataframe_operation(&expr));
    }

    #[test]
    fn test_is_dataframe_operation_block_false() {
        let transformer = PatternTransformer::new();
        let expr = RuchyExpr::Block(vec![RuchyExpr::Literal(Literal::Integer(1))]);
        assert!(!transformer.is_dataframe_operation(&expr));
    }

    #[test]
    fn test_is_dataframe_operation_sum() {
        let transformer = PatternTransformer::new();
        let expr = RuchyExpr::Call {
            func: Box::new(RuchyExpr::FieldAccess {
                object: Box::new(RuchyExpr::Identifier("df".to_string())),
                field: "sum".to_string(),
            }),
            args: vec![],
        };
        assert!(transformer.is_dataframe_operation(&expr));
    }

    #[test]
    fn test_is_dataframe_operation_mean() {
        let transformer = PatternTransformer::new();
        let expr = RuchyExpr::Call {
            func: Box::new(RuchyExpr::FieldAccess {
                object: Box::new(RuchyExpr::Identifier("df".to_string())),
                field: "mean".to_string(),
            }),
            args: vec![],
        };
        assert!(transformer.is_dataframe_operation(&expr));
    }

    #[test]
    fn test_is_dataframe_operation_groupby() {
        let transformer = PatternTransformer::new();
        let expr = RuchyExpr::Call {
            func: Box::new(RuchyExpr::FieldAccess {
                object: Box::new(RuchyExpr::Identifier("df".to_string())),
                field: "groupby".to_string(),
            }),
            args: vec![],
        };
        assert!(transformer.is_dataframe_operation(&expr));
    }

    #[test]
    fn test_is_dataframe_operation_sort() {
        let transformer = PatternTransformer::new();
        let expr = RuchyExpr::Call {
            func: Box::new(RuchyExpr::FieldAccess {
                object: Box::new(RuchyExpr::Identifier("df".to_string())),
                field: "sort".to_string(),
            }),
            args: vec![],
        };
        assert!(transformer.is_dataframe_operation(&expr));
    }

    #[test]
    fn test_is_dataframe_operation_aggregate() {
        let transformer = PatternTransformer::new();
        let expr = RuchyExpr::Call {
            func: Box::new(RuchyExpr::FieldAccess {
                object: Box::new(RuchyExpr::Identifier("df".to_string())),
                field: "aggregate".to_string(),
            }),
            args: vec![],
        };
        assert!(transformer.is_dataframe_operation(&expr));
    }

    #[test]
    fn test_is_dataframe_operation_not_df_method() {
        let transformer = PatternTransformer::new();
        let expr = RuchyExpr::Call {
            func: Box::new(RuchyExpr::FieldAccess {
                object: Box::new(RuchyExpr::Identifier("df".to_string())),
                field: "custom_method".to_string(),
            }),
            args: vec![],
        };
        assert!(!transformer.is_dataframe_operation(&expr));
    }

    #[test]
    fn test_is_dataframe_operation_not_field_access() {
        let transformer = PatternTransformer::new();
        let expr = RuchyExpr::Call {
            func: Box::new(RuchyExpr::Identifier("sum".to_string())),
            args: vec![],
        };
        assert!(!transformer.is_dataframe_operation(&expr));
    }

    #[test]
    fn test_is_dataframe_operation_literal() {
        let transformer = PatternTransformer::new();
        let expr = RuchyExpr::Literal(Literal::Integer(42));
        assert!(!transformer.is_dataframe_operation(&expr));
    }

    #[test]
    fn test_extract_dataframe_ops_empty_block() {
        let transformer = PatternTransformer::new();
        let expr = RuchyExpr::Block(vec![]);
        let ops = transformer.extract_dataframe_ops(&expr).unwrap();
        assert!(ops.is_empty());
    }

    #[test]
    fn test_extract_dataframe_ops_non_call() {
        let transformer = PatternTransformer::new();
        let expr = RuchyExpr::Identifier("x".to_string());
        let ops = transformer.extract_dataframe_ops(&expr).unwrap();
        assert!(ops.is_empty());
    }

    #[test]
    fn test_extract_dataframe_ops_call_with_field_access() {
        let transformer = PatternTransformer::new();
        let expr = RuchyExpr::Call {
            func: Box::new(RuchyExpr::FieldAccess {
                object: Box::new(RuchyExpr::Identifier("df".to_string())),
                field: "sum".to_string(),
            }),
            args: vec![RuchyExpr::Literal(Literal::Integer(1))],
        };
        let ops = transformer.extract_dataframe_ops(&expr).unwrap();
        assert_eq!(ops.len(), 1);
        assert_eq!(ops[0].method, "sum");
    }

    #[test]
    fn test_extract_dataframe_ops_call_without_field_access() {
        let transformer = PatternTransformer::new();
        let expr = RuchyExpr::Call {
            func: Box::new(RuchyExpr::Identifier("sum".to_string())),
            args: vec![],
        };
        let ops = transformer.extract_dataframe_ops(&expr).unwrap();
        assert!(ops.is_empty());
    }

    #[test]
    fn test_extract_dataframe_ops_block_with_ops() {
        let transformer = PatternTransformer::new();
        let expr = RuchyExpr::Block(vec![
            RuchyExpr::Call {
                func: Box::new(RuchyExpr::FieldAccess {
                    object: Box::new(RuchyExpr::Identifier("df".to_string())),
                    field: "sum".to_string(),
                }),
                args: vec![],
            },
            RuchyExpr::Call {
                func: Box::new(RuchyExpr::FieldAccess {
                    object: Box::new(RuchyExpr::Identifier("df".to_string())),
                    field: "mean".to_string(),
                }),
                args: vec![],
            },
        ]);
        let ops = transformer.extract_dataframe_ops(&expr).unwrap();
        assert_eq!(ops.len(), 2);
    }

    #[test]
    fn test_transform_for_loop_no_dataframe() {
        let transformer = PatternTransformer::new();
        let for_expr = RuchyExpr::For {
            var: "x".to_string(),
            iter: Box::new(RuchyExpr::List(vec![])),
            body: Box::new(RuchyExpr::Literal(Literal::Integer(1))),
        };
        let result = transformer.transform(for_expr).unwrap();
        assert!(matches!(result, RuchyExpr::For { .. }));
    }

    #[test]
    fn test_transform_for_loop_dataframe_enabled() {
        let transformer = PatternTransformer::new();
        let for_expr = RuchyExpr::For {
            var: "row".to_string(),
            iter: Box::new(RuchyExpr::Identifier("data".to_string())),
            body: Box::new(RuchyExpr::Block(vec![RuchyExpr::Call {
                func: Box::new(RuchyExpr::FieldAccess {
                    object: Box::new(RuchyExpr::Identifier("result".to_string())),
                    field: "append".to_string(),
                }),
                args: vec![],
            }])),
        };
        let result = transformer.transform(for_expr).unwrap();
        // Should transform to DataFrame operations
        assert!(matches!(
            result,
            RuchyExpr::MethodCall { .. } | RuchyExpr::Call { .. }
        ));
    }

    #[test]
    fn test_transform_for_loop_dataframe_disabled() {
        let config = crate::RuchyConfig {
            optimize_dataframes: false,
            ..Default::default()
        };
        let transformer = PatternTransformer::with_config(&config);
        let for_expr = RuchyExpr::For {
            var: "row".to_string(),
            iter: Box::new(RuchyExpr::Identifier("data".to_string())),
            body: Box::new(RuchyExpr::Block(vec![RuchyExpr::Call {
                func: Box::new(RuchyExpr::FieldAccess {
                    object: Box::new(RuchyExpr::Identifier("result".to_string())),
                    field: "append".to_string(),
                }),
                args: vec![],
            }])),
        };
        let result = transformer.transform(for_expr).unwrap();
        // Should remain as for loop since dataframe optimization is disabled
        assert!(matches!(result, RuchyExpr::For { .. }));
    }

    #[test]
    fn test_transform_filter_map_pattern() {
        let transformer = PatternTransformer::new();
        let expr = RuchyExpr::Call {
            func: Box::new(RuchyExpr::Identifier("map".to_string())),
            args: vec![
                RuchyExpr::Lambda {
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
                },
                RuchyExpr::Call {
                    func: Box::new(RuchyExpr::Identifier("filter".to_string())),
                    args: vec![
                        RuchyExpr::Lambda {
                            params: vec![Param {
                                name: "x".to_string(),
                                typ: None,
                                default: None,
                            }],
                            body: Box::new(RuchyExpr::Binary {
                                left: Box::new(RuchyExpr::Identifier("x".to_string())),
                                op: BinaryOp::Greater,
                                right: Box::new(RuchyExpr::Literal(Literal::Integer(0))),
                            }),
                        },
                        RuchyExpr::List(vec![
                            RuchyExpr::Literal(Literal::Integer(1)),
                            RuchyExpr::Literal(Literal::Integer(2)),
                        ]),
                    ],
                },
            ],
        };
        let result = transformer.transform(expr).unwrap();
        assert!(matches!(result, RuchyExpr::Pipeline { .. }));
    }

    #[test]
    fn test_transform_filter_map_insufficient_args() {
        let transformer = PatternTransformer::new();
        // Call transform_filter_map directly with insufficient args
        let result = transformer
            .transform_filter_map(
                RuchyExpr::Identifier("map".to_string()),
                vec![RuchyExpr::Identifier("x".to_string())], // Only 1 arg instead of 2
            )
            .unwrap();
        // Should return a Call since args were insufficient
        assert!(matches!(result, RuchyExpr::Call { .. }));
    }

    #[test]
    fn test_transform_filter_map_not_filter_call() {
        let transformer = PatternTransformer::new();
        // Second arg is not a filter call
        let result = transformer
            .transform_filter_map(
                RuchyExpr::Identifier("map".to_string()),
                vec![
                    RuchyExpr::Lambda {
                        params: vec![],
                        body: Box::new(RuchyExpr::Identifier("x".to_string())),
                    },
                    RuchyExpr::List(vec![]), // Not a Call with filter
                ],
            )
            .unwrap();
        // Should return a Call since filter call wasn't found
        assert!(matches!(result, RuchyExpr::Call { .. }));
    }

    #[test]
    fn test_transform_string_format_non_string_object() {
        let transformer = PatternTransformer::new();
        let format_expr = RuchyExpr::Call {
            func: Box::new(RuchyExpr::FieldAccess {
                object: Box::new(RuchyExpr::Identifier("var".to_string())), // Not a string literal
                field: "format".to_string(),
            }),
            args: vec![],
        };
        let result = transformer.transform(format_expr).unwrap();
        // Should return Call unchanged since object isn't a string literal
        assert!(matches!(result, RuchyExpr::Call { .. }));
    }

    #[test]
    fn test_transform_string_format_non_field_access() {
        let transformer = PatternTransformer::new();
        // The func isn't a FieldAccess
        let call = RuchyExpr::Call {
            func: Box::new(RuchyExpr::Identifier("format".to_string())),
            args: vec![RuchyExpr::Literal(Literal::String("test".to_string()))],
        };
        let result = transformer.transform(call).unwrap();
        // Should return Call unchanged
        assert!(matches!(result, RuchyExpr::Call { .. }));
    }

    #[test]
    fn test_transform_list_comp_no_condition() {
        let transformer = PatternTransformer::new();
        let comp_expr = RuchyExpr::Call {
            func: Box::new(RuchyExpr::Identifier("list_comp".to_string())),
            args: vec![
                RuchyExpr::Identifier("x".to_string()),
                RuchyExpr::Identifier("x".to_string()),
                RuchyExpr::List(vec![
                    RuchyExpr::Literal(Literal::Integer(1)),
                    RuchyExpr::Literal(Literal::Integer(2)),
                ]),
            ],
        };
        let result = transformer.transform(comp_expr).unwrap();
        match result {
            RuchyExpr::Pipeline { stages, .. } => {
                // Without condition: only Map and Call stages (no Filter)
                assert_eq!(stages.len(), 2);
                assert!(matches!(stages[0], PipelineStage::Map(_)));
                assert!(matches!(stages[1], PipelineStage::Call(_, _)));
            }
            _ => panic!("Expected Pipeline"),
        }
    }

    #[test]
    fn test_transform_pass_through_expressions() {
        let transformer = PatternTransformer::new();

        // Test identifier
        let id = RuchyExpr::Identifier("x".to_string());
        let result = transformer.transform(id.clone()).unwrap();
        assert!(matches!(result, RuchyExpr::Identifier(_)));

        // Test literals
        let int_lit = RuchyExpr::Literal(Literal::Integer(42));
        let result = transformer.transform(int_lit).unwrap();
        assert!(matches!(result, RuchyExpr::Literal(Literal::Integer(42))));

        let float_lit = RuchyExpr::Literal(Literal::Float(3.14));
        let result = transformer.transform(float_lit).unwrap();
        assert!(matches!(result, RuchyExpr::Literal(Literal::Float(_))));

        let str_lit = RuchyExpr::Literal(Literal::String("hello".to_string()));
        let result = transformer.transform(str_lit).unwrap();
        assert!(matches!(result, RuchyExpr::Literal(Literal::String(_))));

        let bool_lit = RuchyExpr::Literal(Literal::Bool(true));
        let result = transformer.transform(bool_lit).unwrap();
        assert!(matches!(result, RuchyExpr::Literal(Literal::Bool(true))));

        let unit_lit = RuchyExpr::Literal(Literal::Unit);
        let result = transformer.transform(unit_lit).unwrap();
        assert!(matches!(result, RuchyExpr::Literal(Literal::Unit)));

        // Test list
        let list = RuchyExpr::List(vec![RuchyExpr::Literal(Literal::Integer(1))]);
        let result = transformer.transform(list).unwrap();
        assert!(matches!(result, RuchyExpr::List(_)));

        // Test field access
        let field_access = RuchyExpr::FieldAccess {
            object: Box::new(RuchyExpr::Identifier("obj".to_string())),
            field: "field".to_string(),
        };
        let result = transformer.transform(field_access).unwrap();
        assert!(matches!(result, RuchyExpr::FieldAccess { .. }));

        // Test range
        let range = RuchyExpr::Range {
            start: Box::new(RuchyExpr::Literal(Literal::Integer(0))),
            end: Box::new(RuchyExpr::Literal(Literal::Integer(10))),
            inclusive: false,
        };
        let result = transformer.transform(range).unwrap();
        assert!(matches!(result, RuchyExpr::Range { .. }));
    }

    #[test]
    fn test_transform_async_function() {
        use crate::ast::RuchyType;
        let transformer = PatternTransformer::new();
        let func = RuchyExpr::Function {
            name: "async_test".to_string(),
            params: vec![],
            body: Box::new(RuchyExpr::Literal(Literal::Integer(42))),
            is_async: true,
            return_type: Some(RuchyType::Named("i32".to_string())),
        };
        let result = transformer.transform(func).unwrap();
        match result {
            RuchyExpr::Function {
                is_async,
                return_type,
                ..
            } => {
                assert!(is_async);
                assert!(return_type.is_some());
            }
            _ => panic!("Expected Function"),
        }
    }

    #[test]
    fn test_transform_let_mutable() {
        let transformer = PatternTransformer::new();
        let let_expr = RuchyExpr::Let {
            name: "x".to_string(),
            value: Box::new(RuchyExpr::Literal(Literal::Integer(0))),
            body: Box::new(RuchyExpr::Identifier("x".to_string())),
            is_mutable: true,
        };
        let result = transformer.transform(let_expr).unwrap();
        match result {
            RuchyExpr::Let { is_mutable, .. } => {
                assert!(is_mutable);
            }
            _ => panic!("Expected Let"),
        }
    }

    #[test]
    fn test_transform_nested_if() {
        let transformer = PatternTransformer::new();
        let nested_if = RuchyExpr::If {
            condition: Box::new(RuchyExpr::Literal(Literal::Bool(true))),
            then_branch: Box::new(RuchyExpr::If {
                condition: Box::new(RuchyExpr::Literal(Literal::Bool(false))),
                then_branch: Box::new(RuchyExpr::Literal(Literal::Integer(1))),
                else_branch: Some(Box::new(RuchyExpr::Literal(Literal::Integer(2)))),
            }),
            else_branch: Some(Box::new(RuchyExpr::Literal(Literal::Integer(3)))),
        };
        let result = transformer.transform(nested_if).unwrap();
        assert!(matches!(result, RuchyExpr::If { .. }));
    }
}
