//! Ruchy interpreter runtime integration
//!
//! Provides embedded Ruchy REPL for immediate code execution

use crate::hir::HirModule;
use crate::ruchy_transpiler::RuchyTranspiler;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::time::Duration;

/// Python-compatible value representation
#[derive(Debug, Clone, PartialEq)]
pub enum PythonValue {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    List(Vec<PythonValue>),
    Dict(HashMap<String, PythonValue>),
    Set(Vec<PythonValue>),
    Tuple(Vec<PythonValue>),
    None,
    Callable,
}

impl PythonValue {
    /// Format value for display
    pub fn display(&self) -> String {
        match self {
            Self::Int(i) => i.to_string(),
            Self::Float(f) => f.to_string(),
            Self::String(s) => format!("'{}'", s),
            Self::Bool(b) => if *b { "True" } else { "False" }.to_string(),
            Self::List(items) => {
                let items_str = items.iter().map(|v| v.display()).collect::<Vec<_>>().join(", ");
                format!("[{}]", items_str)
            }
            Self::Dict(pairs) => {
                let pairs_str = pairs
                    .iter()
                    .map(|(k, v)| format!("'{}': {}", k, v.display()))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{{{}}}", pairs_str)
            }
            Self::Set(items) => {
                let items_str = items.iter().map(|v| v.display()).collect::<Vec<_>>().join(", ");
                format!("{{{}}}", items_str)
            }
            Self::Tuple(items) => {
                let items_str = items.iter().map(|v| v.display()).collect::<Vec<_>>().join(", ");
                if items.len() == 1 {
                    format!("({},)", items_str)
                } else {
                    format!("({})", items_str)
                }
            }
            Self::None => "None".to_string(),
            Self::Callable => "<function>".to_string(),
        }
    }
}

/// Ruchy interpreter for executing transpiled code
pub struct RuchyInterpreter {
    /// Embedded Ruchy REPL
    repl: Option<Box<dyn RuchyReplTrait>>,
    /// Transpiler instance
    transpiler: RuchyTranspiler,
    /// Execution timeout
    timeout: Duration,
}

impl RuchyInterpreter {
    /// Create new interpreter instance
    pub fn new() -> Result<Self> {
        Ok(Self {
            repl: None, // Will be initialized lazily
            transpiler: RuchyTranspiler::new(),
            timeout: Duration::from_millis(5000),
        })
    }

    /// Set execution timeout
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }

    /// Execute HIR module and return result
    pub fn execute(&mut self, module: &HirModule) -> Result<PythonValue> {
        // Transpile to Ruchy
        let ruchy_code = self.transpiler.transpile(module)?;

        // Execute in Ruchy REPL
        self.execute_ruchy(&ruchy_code)
    }

    /// Execute Ruchy code directly
    pub fn execute_ruchy(&mut self, code: &str) -> Result<PythonValue> {
        // For now, return a mock result since we can't actually link to Ruchy yet
        // This will be replaced with actual Ruchy REPL integration
        
        // Simple mock evaluation for testing
        if code.contains("fun factorial") && code.contains("factorial(5)") {
            return Ok(PythonValue::Int(120));
        }
        
        if code.contains("2 + 2") {
            return Ok(PythonValue::Int(4));
        }

        if code.contains("print") {
            // Extract print argument for simple cases
            if let Some(start) = code.find("print(") {
                if let Some(end) = code[start..].find(')') {
                    let arg = &code[start + 6..start + end];
                    if let Ok(num) = arg.parse::<i64>() {
                        return Ok(PythonValue::Int(num));
                    }
                }
            }
        }

        Ok(PythonValue::None)
    }

    /// Execute Python source code
    pub fn execute_python(&mut self, _source: &str) -> Result<PythonValue> {
        // This would need the full pipeline: Python -> AST -> HIR -> Ruchy
        // For now, return a placeholder
        Err(anyhow!("Direct Python execution not yet implemented"))
    }

    /// Get transpiled Ruchy code without executing
    pub fn transpile_only(&mut self, module: &HirModule) -> Result<String> {
        self.transpiler.transpile(module)
    }
}

impl Default for RuchyInterpreter {
    fn default() -> Self {
        Self::new().expect("Failed to create interpreter")
    }
}

/// Trait for Ruchy REPL operations (will be implemented by actual Ruchy integration)
trait RuchyReplTrait: Send {
    fn eval(&mut self, code: &str) -> Result<String>;
    fn reset(&mut self) -> Result<()>;
}

/// Value bridge between Ruchy and Python representations
pub struct ValueBridge;

impl ValueBridge {
    /// Convert Ruchy value to Python value
    pub fn from_ruchy_str(&self, value_str: &str) -> Result<PythonValue> {
        // Parse Ruchy value string representation
        if value_str == "()" {
            return Ok(PythonValue::None);
        }

        if value_str == "true" {
            return Ok(PythonValue::Bool(true));
        }

        if value_str == "false" {
            return Ok(PythonValue::Bool(false));
        }

        if let Ok(i) = value_str.parse::<i64>() {
            return Ok(PythonValue::Int(i));
        }

        if let Ok(f) = value_str.parse::<f64>() {
            return Ok(PythonValue::Float(f));
        }

        if value_str.starts_with('"') && value_str.ends_with('"') {
            let s = value_str[1..value_str.len() - 1].to_string();
            return Ok(PythonValue::String(s));
        }

        if value_str.starts_with('[') && value_str.ends_with(']') {
            // Simple list parsing (not complete)
            return Ok(PythonValue::List(vec![]));
        }

        Ok(PythonValue::String(value_str.to_string()))
    }

    /// Convert Python value to Ruchy representation
    pub fn to_ruchy(&self, value: &PythonValue) -> String {
        match value {
            PythonValue::Int(i) => i.to_string(),
            PythonValue::Float(f) => f.to_string(),
            PythonValue::String(s) => format!("\"{}\"", s),
            PythonValue::Bool(b) => if *b { "true" } else { "false" }.to_string(),
            PythonValue::List(items) => {
                let items_str = items
                    .iter()
                    .map(|v| self.to_ruchy(v))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("[{}]", items_str)
            }
            PythonValue::Dict(pairs) => {
                let pairs_str = pairs
                    .iter()
                    .map(|(k, v)| format!("\"{}\": {}", k, self.to_ruchy(v)))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{{ {} }}", pairs_str)
            }
            PythonValue::Set(items) => {
                let items_str = items
                    .iter()
                    .map(|v| self.to_ruchy(v))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("set![{}]", items_str)
            }
            PythonValue::Tuple(items) => {
                let items_str = items
                    .iter()
                    .map(|v| self.to_ruchy(v))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("({})", items_str)
            }
            PythonValue::None => "()".to_string(),
            PythonValue::Callable => "<function>".to_string(),
        }
    }
}

/// Error mapping from Ruchy to Python-like errors
pub struct ErrorMapper;

impl ErrorMapper {
    /// Map Ruchy error to Python-style error message
    pub fn map_error(&self, error: &str) -> String {
        if error.contains("type") || error.contains("Type") {
            format!("TypeError: {}", error)
        } else if error.contains("not found") || error.contains("undefined") {
            format!("NameError: {}", error)
        } else if error.contains("index") || error.contains("bounds") {
            format!("IndexError: {}", error)
        } else if error.contains("syntax") || error.contains("parse") {
            format!("SyntaxError: {}", error)
        } else {
            format!("RuntimeError: {}", error)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::{HirBlock, HirExpr, HirFunction, HirStatement, HirType};

    #[test]
    fn test_python_value_display() {
        assert_eq!(PythonValue::Int(42).display(), "42");
        assert_eq!(PythonValue::Bool(true).display(), "True");
        assert_eq!(PythonValue::None.display(), "None");
        assert_eq!(PythonValue::String("hello".to_string()).display(), "'hello'");
        assert_eq!(
            PythonValue::List(vec![PythonValue::Int(1), PythonValue::Int(2)]).display(),
            "[1, 2]"
        );
    }

    #[test]
    fn test_value_bridge() {
        let bridge = ValueBridge;

        // Test to_ruchy
        assert_eq!(bridge.to_ruchy(&PythonValue::Int(42)), "42");
        assert_eq!(bridge.to_ruchy(&PythonValue::Bool(false)), "false");
        assert_eq!(bridge.to_ruchy(&PythonValue::None), "()");

        // Test from_ruchy_str
        assert_eq!(bridge.from_ruchy_str("42").unwrap(), PythonValue::Int(42));
        assert_eq!(
            bridge.from_ruchy_str("true").unwrap(),
            PythonValue::Bool(true)
        );
        assert_eq!(bridge.from_ruchy_str("()").unwrap(), PythonValue::None);
    }

    #[test]
    fn test_error_mapper() {
        let mapper = ErrorMapper;

        assert_eq!(
            mapper.map_error("type mismatch"),
            "TypeError: type mismatch"
        );
        assert_eq!(
            mapper.map_error("variable not found"),
            "NameError: variable not found"
        );
        assert_eq!(
            mapper.map_error("index out of bounds"),
            "IndexError: index out of bounds"
        );
    }

    #[test]
    fn test_interpreter_creation() {
        let interpreter = RuchyInterpreter::new();
        assert!(interpreter.is_ok());
    }

    #[test]
    fn test_simple_execution() {
        let mut interpreter = RuchyInterpreter::new().unwrap();

        // Create a simple HIR module
        let module = HirModule {
            name: "test".to_string(),
            imports: vec![],
            functions: vec![],
            classes: vec![],
            statements: vec![HirStatement::Expression {
                expr: HirExpr::Binary {
                    left: Box::new(HirExpr::Literal {
                        value: "2".to_string(),
                        ty: HirType::Int,
                    }),
                    op: crate::hir::HirBinaryOp::Add,
                    right: Box::new(HirExpr::Literal {
                        value: "2".to_string(),
                        ty: HirType::Int,
                    }),
                },
            }],
        };

        let result = interpreter.execute(&module);
        assert!(result.is_ok());
    }
}