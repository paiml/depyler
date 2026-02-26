//! Extractor: Parse .pyi stub files to extract type facts.
//!
//! Uses `rustpython_parser` to parse Python stub files and extract
//! function signatures, class definitions, and method types.

use crate::{KnowledgeError, Result, TypeFact, TypeFactKind};
use rustpython_ast::{self as ast, Stmt};
use rustpython_parser::{parse, Mode};
use std::path::Path;
use tracing::{debug, warn};

/// Extractor for parsing Python stub files.
pub struct Extractor {
    /// Whether to include private symbols (starting with _)
    include_private: bool,
}

impl Default for Extractor {
    fn default() -> Self {
        Self::new()
    }
}

impl Extractor {
    /// Create a new Extractor.
    pub fn new() -> Self {
        Self { include_private: false }
    }

    /// Include private symbols (starting with _).
    pub fn with_private(mut self) -> Self {
        self.include_private = true;
        self
    }

    /// Extract type facts from a single .pyi or .py file.
    pub fn extract_file(&self, path: &Path, module: &str) -> Result<Vec<TypeFact>> {
        let source = std::fs::read_to_string(path)?;
        self.extract_source(&source, module, path.to_string_lossy().as_ref())
    }

    /// Extract type facts from source code.
    pub fn extract_source(
        &self,
        source: &str,
        module: &str,
        filename: &str,
    ) -> Result<Vec<TypeFact>> {
        let parsed = parse(source, Mode::Module, filename).map_err(|e| {
            KnowledgeError::StubParseError { file: filename.to_string(), message: e.to_string() }
        })?;

        let mut facts = Vec::new();

        // The parsed result is a Module containing statements
        if let ast::Mod::Module(module_ast) = parsed {
            for stmt in module_ast.body {
                self.extract_stmt(&stmt, module, &mut facts);
            }
        }

        debug!(
            module = %module,
            facts = facts.len(),
            "Extracted type facts"
        );

        Ok(facts)
    }

    /// Extract type facts from a statement.
    fn extract_stmt(&self, stmt: &Stmt, module: &str, facts: &mut Vec<TypeFact>) {
        match stmt {
            Stmt::FunctionDef(func) => {
                if self.should_include(&func.name) {
                    if let Some(fact) = self.extract_function(func, module) {
                        facts.push(fact);
                    }
                }
            }
            Stmt::AsyncFunctionDef(func) => {
                if self.should_include(&func.name) {
                    if let Some(fact) = self.extract_async_function(func, module) {
                        facts.push(fact);
                    }
                }
            }
            Stmt::ClassDef(class) => {
                if self.should_include(&class.name) {
                    self.extract_class(class, module, facts);
                }
            }
            Stmt::AnnAssign(assign) => {
                if let Some(fact) = self.extract_annotated_assign(assign, module) {
                    facts.push(fact);
                }
            }
            _ => {}
        }
    }

    /// Check if a symbol should be included based on privacy settings.
    fn should_include(&self, name: &str) -> bool {
        self.include_private || !name.starts_with('_')
    }

    /// Extract a function definition.
    fn extract_function(&self, func: &ast::StmtFunctionDef, module: &str) -> Option<TypeFact> {
        let signature = self.build_signature(&func.args, &func.returns);
        let return_type = self.type_to_string(&func.returns);

        Some(TypeFact {
            module: module.to_string(),
            symbol: func.name.to_string(),
            kind: TypeFactKind::Function,
            signature,
            return_type,
        })
    }

    /// Extract an async function definition.
    fn extract_async_function(
        &self,
        func: &ast::StmtAsyncFunctionDef,
        module: &str,
    ) -> Option<TypeFact> {
        let signature = self.build_signature(&func.args, &func.returns);
        let return_type = self.type_to_string(&func.returns);

        Some(TypeFact {
            module: module.to_string(),
            symbol: func.name.to_string(),
            kind: TypeFactKind::Function,
            signature: format!("async {signature}"),
            return_type,
        })
    }

    /// Extract a class and its methods.
    fn extract_class(&self, class: &ast::StmtClassDef, module: &str, facts: &mut Vec<TypeFact>) {
        // Add the class itself
        facts.push(TypeFact::class(module, &class.name));

        // Extract methods
        for stmt in &class.body {
            match stmt {
                Stmt::FunctionDef(method) => {
                    if self.should_include(&method.name) {
                        if let Some(fact) = self.extract_method(method, module, &class.name) {
                            facts.push(fact);
                        }
                    }
                }
                Stmt::AsyncFunctionDef(method) => {
                    if self.should_include(&method.name) {
                        if let Some(fact) = self.extract_async_method(method, module, &class.name) {
                            facts.push(fact);
                        }
                    }
                }
                Stmt::AnnAssign(assign) => {
                    if let Some(fact) = self.extract_class_attribute(assign, module, &class.name) {
                        facts.push(fact);
                    }
                }
                _ => {}
            }
        }
    }

    /// Extract a method from a class.
    fn extract_method(
        &self,
        method: &ast::StmtFunctionDef,
        module: &str,
        class_name: &str,
    ) -> Option<TypeFact> {
        let signature = self.build_signature(&method.args, &method.returns);
        let return_type = self.type_to_string(&method.returns);

        Some(TypeFact::method(module, class_name, &method.name, &signature, &return_type))
    }

    /// Extract an async method from a class.
    fn extract_async_method(
        &self,
        method: &ast::StmtAsyncFunctionDef,
        module: &str,
        class_name: &str,
    ) -> Option<TypeFact> {
        let signature = self.build_signature(&method.args, &method.returns);
        let return_type = self.type_to_string(&method.returns);

        Some(TypeFact::method(
            module,
            class_name,
            &method.name,
            &format!("async {signature}"),
            &return_type,
        ))
    }

    /// Extract an annotated assignment (module-level attribute).
    fn extract_annotated_assign(
        &self,
        assign: &ast::StmtAnnAssign,
        module: &str,
    ) -> Option<TypeFact> {
        let target = match assign.target.as_ref() {
            ast::Expr::Name(name) => name.id.to_string(),
            _ => return None,
        };

        if !self.should_include(&target) {
            return None;
        }

        let type_str = self.expr_to_string(&assign.annotation);

        Some(TypeFact {
            module: module.to_string(),
            symbol: target,
            kind: TypeFactKind::Attribute,
            signature: String::new(),
            return_type: type_str,
        })
    }

    /// Extract a class attribute.
    fn extract_class_attribute(
        &self,
        assign: &ast::StmtAnnAssign,
        module: &str,
        class_name: &str,
    ) -> Option<TypeFact> {
        let target = match assign.target.as_ref() {
            ast::Expr::Name(name) => name.id.to_string(),
            _ => return None,
        };

        if !self.should_include(&target) {
            return None;
        }

        let type_str = self.expr_to_string(&assign.annotation);

        Some(TypeFact {
            module: module.to_string(),
            symbol: format!("{class_name}.{target}"),
            kind: TypeFactKind::Attribute,
            signature: String::new(),
            return_type: type_str,
        })
    }

    /// Build a signature string from arguments and return type.
    fn build_signature(&self, args: &ast::Arguments, returns: &Option<Box<ast::Expr>>) -> String {
        let mut parts = Vec::new();

        // Positional-only params
        for param in &args.posonlyargs {
            parts.push(self.arg_with_default_to_string(param));
        }

        if !args.posonlyargs.is_empty() && !args.args.is_empty() {
            parts.push("/".to_string());
        }

        // Regular args
        for param in &args.args {
            parts.push(self.arg_with_default_to_string(param));
        }

        // *args
        if let Some(vararg) = &args.vararg {
            parts.push(format!("*{}", self.arg_to_string(vararg)));
        }

        // Keyword-only args
        for param in &args.kwonlyargs {
            parts.push(self.arg_with_default_to_string(param));
        }

        // **kwargs
        if let Some(kwarg) = &args.kwarg {
            parts.push(format!("**{}", self.arg_to_string(kwarg)));
        }

        let params_str = parts.join(", ");
        let return_str = self.type_to_string(returns);

        format!("({params_str}) -> {return_str}")
    }

    /// Convert an ArgWithDefault to string.
    fn arg_with_default_to_string(&self, arg: &ast::ArgWithDefault) -> String {
        let name = &arg.def.arg;
        let type_str =
            arg.def.annotation.as_ref().map(|a| self.expr_to_string(a)).unwrap_or_default();

        if type_str.is_empty() {
            if arg.default.is_some() {
                format!("{name} = ...")
            } else {
                name.to_string()
            }
        } else if arg.default.is_some() {
            format!("{name}: {type_str} = ...")
        } else {
            format!("{name}: {type_str}")
        }
    }

    /// Convert an Arg to string (for vararg/kwarg).
    fn arg_to_string(&self, arg: &ast::Arg) -> String {
        let name = &arg.arg;
        let type_str = arg.annotation.as_ref().map(|a| self.expr_to_string(a)).unwrap_or_default();

        if type_str.is_empty() {
            name.to_string()
        } else {
            format!("{name}: {type_str}")
        }
    }

    /// Convert return type to string.
    fn type_to_string(&self, returns: &Option<Box<ast::Expr>>) -> String {
        match returns {
            Some(expr) => self.expr_to_string(expr),
            None => "None".to_string(),
        }
    }

    /// Convert an expression to a type string.
    fn expr_to_string(&self, expr: &ast::Expr) -> String {
        match expr {
            ast::Expr::Name(name) => name.id.to_string(),
            ast::Expr::Attribute(attr) => {
                let value = self.expr_to_string(&attr.value);
                format!("{value}.{}", attr.attr)
            }
            ast::Expr::Subscript(sub) => {
                let value = self.expr_to_string(&sub.value);
                let slice = self.expr_to_string(&sub.slice);
                format!("{value}[{slice}]")
            }
            ast::Expr::Tuple(tuple) => {
                let elts: Vec<_> = tuple.elts.iter().map(|e| self.expr_to_string(e)).collect();
                elts.join(", ")
            }
            ast::Expr::BinOp(binop) => {
                // Handle Union types written as X | Y
                if matches!(binop.op, ast::Operator::BitOr) {
                    let left = self.expr_to_string(&binop.left);
                    let right = self.expr_to_string(&binop.right);
                    format!("{left} | {right}")
                } else {
                    "Unknown".to_string()
                }
            }
            ast::Expr::Constant(c) => match &c.value {
                ast::Constant::None => "None".to_string(),
                ast::Constant::Str(s) => format!("\"{s}\""),
                ast::Constant::Int(i) => i.to_string(),
                ast::Constant::Float(f) => f.to_string(),
                ast::Constant::Bool(b) => b.to_string(),
                ast::Constant::Ellipsis => "...".to_string(),
                _ => "Unknown".to_string(),
            },
            ast::Expr::List(list) => {
                let elts: Vec<_> = list.elts.iter().map(|e| self.expr_to_string(e)).collect();
                format!("[{}]", elts.join(", "))
            }
            _ => {
                warn!("Unknown expression type in type annotation");
                "Unknown".to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_simple_function() {
        let source = r#"
def get(url: str) -> Response: ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "requests", "test.pyi").unwrap();

        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].symbol, "get");
        assert_eq!(facts[0].kind, TypeFactKind::Function);
        assert!(facts[0].signature.contains("url: str"));
        assert_eq!(facts[0].return_type, "Response");
    }

    #[test]
    fn test_extract_function_with_optional() {
        let source = r#"
def get(url: str, params: dict | None = ...) -> Response: ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "requests", "test.pyi").unwrap();

        assert_eq!(facts.len(), 1);
        assert!(facts[0].signature.contains("params: dict | None"));
    }

    #[test]
    fn test_extract_class_with_methods() {
        let source = r#"
class Response:
    status_code: int
    def json(self) -> dict: ...
    def text(self) -> str: ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "requests.models", "test.pyi").unwrap();

        // Should have: class, status_code attribute, json method, text method
        assert_eq!(facts.len(), 4);

        let class_fact = facts.iter().find(|f| f.symbol == "Response").unwrap();
        assert_eq!(class_fact.kind, TypeFactKind::Class);

        let json_fact = facts.iter().find(|f| f.symbol == "Response.json").unwrap();
        assert_eq!(json_fact.kind, TypeFactKind::Method);
        assert_eq!(json_fact.return_type, "dict");
    }

    #[test]
    fn test_excludes_private_by_default() {
        let source = r#"
def _private(): ...
def public(): ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();

        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].symbol, "public");
    }

    #[test]
    fn test_includes_private_when_enabled() {
        let source = r#"
def _private(): ...
def public(): ...
"#;
        let extractor = Extractor::new().with_private();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();

        assert_eq!(facts.len(), 2);
    }

    #[test]
    fn test_extract_kwargs() {
        let source = r#"
def get(url: str, **kwargs) -> Response: ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "requests", "test.pyi").unwrap();

        assert!(facts[0].signature.contains("**kwargs"));
    }

    #[test]
    fn test_extractor_default() {
        let extractor = Extractor::default();
        // Default should not include private
        let source = "def _hidden(): ...\ndef visible(): ...";
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].symbol, "visible");
    }

    #[test]
    fn test_extract_async_function() {
        let source = r#"
async def fetch(url: str) -> bytes: ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "aiohttp", "test.pyi").unwrap();

        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].symbol, "fetch");
        assert_eq!(facts[0].kind, TypeFactKind::Function);
        assert!(facts[0].signature.starts_with("async "));
        assert_eq!(facts[0].return_type, "bytes");
    }

    #[test]
    fn test_extract_function_no_return_type() {
        let source = r#"
def setup(): ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();

        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].return_type, "None");
    }

    #[test]
    fn test_extract_annotated_assign() {
        let source = r#"
VERSION: str
DEBUG: bool
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "config", "test.pyi").unwrap();

        assert_eq!(facts.len(), 2);
        assert_eq!(facts[0].symbol, "VERSION");
        assert_eq!(facts[0].kind, TypeFactKind::Attribute);
        assert_eq!(facts[0].return_type, "str");
        assert_eq!(facts[1].symbol, "DEBUG");
        assert_eq!(facts[1].return_type, "bool");
    }

    #[test]
    fn test_extract_private_annotated_assign_excluded() {
        let source = r#"
_internal: int
public: str
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();

        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].symbol, "public");
    }

    #[test]
    fn test_extract_generic_type() {
        let source = r#"
def values() -> List[int]: ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();

        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].return_type, "List[int]");
    }

    #[test]
    fn test_extract_nested_generic_type() {
        let source = r#"
def nested() -> Dict[str, List[int]]: ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();

        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].return_type, "Dict[str, List[int]]");
    }

    #[test]
    fn test_extract_union_type_pipe() {
        let source = r#"
def maybe(x: int) -> str | None: ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();

        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].return_type, "str | None");
    }

    #[test]
    fn test_extract_varargs() {
        let source = r#"
def concat(*args: str) -> str: ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();

        assert_eq!(facts.len(), 1);
        assert!(facts[0].signature.contains("*args: str"));
    }

    #[test]
    fn test_extract_class_attribute() {
        let source = r#"
class Config:
    timeout: int
    name: str
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "app", "test.pyi").unwrap();

        // class + 2 attributes
        assert_eq!(facts.len(), 3);

        let timeout = facts.iter().find(|f| f.symbol == "Config.timeout").unwrap();
        assert_eq!(timeout.kind, TypeFactKind::Attribute);
        assert_eq!(timeout.return_type, "int");
    }

    #[test]
    fn test_extract_class_private_method_excluded() {
        let source = r#"
class MyClass:
    def _private(self) -> None: ...
    def public(self) -> int: ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();

        // class + public method only (private excluded)
        assert_eq!(facts.len(), 2);
        let method = facts.iter().find(|f| f.kind == TypeFactKind::Method).unwrap();
        assert_eq!(method.symbol, "MyClass.public");
    }

    #[test]
    fn test_extract_class_private_method_included() {
        let source = r#"
class MyClass:
    def _private(self) -> None: ...
    def public(self) -> int: ...
"#;
        let extractor = Extractor::new().with_private();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();

        // class + 2 methods (both included)
        assert_eq!(facts.len(), 3);
    }

    #[test]
    fn test_extract_async_method() {
        let source = r#"
class Client:
    async def fetch(self, url: str) -> bytes: ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "http", "test.pyi").unwrap();

        // class + async method
        assert_eq!(facts.len(), 2);
        let method = facts.iter().find(|f| f.kind == TypeFactKind::Method).unwrap();
        assert_eq!(method.symbol, "Client.fetch");
        assert!(method.signature.starts_with("async "));
    }

    #[test]
    fn test_extract_invalid_syntax() {
        let source = "def invalid syntax here %%%: ...";
        let extractor = Extractor::new();
        let result = extractor.extract_source(source, "test", "test.pyi");
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_empty_source() {
        let source = "";
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "empty", "test.pyi").unwrap();
        assert!(facts.is_empty());
    }

    #[test]
    fn test_extract_multiple_functions() {
        let source = r#"
def add(a: int, b: int) -> int: ...
def sub(a: int, b: int) -> int: ...
def mul(a: int, b: int) -> int: ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "math_ops", "test.pyi").unwrap();

        assert_eq!(facts.len(), 3);
        let symbols: Vec<&str> = facts.iter().map(|f| f.symbol.as_str()).collect();
        assert!(symbols.contains(&"add"));
        assert!(symbols.contains(&"sub"));
        assert!(symbols.contains(&"mul"));
    }

    #[test]
    fn test_extract_function_with_default_no_type() {
        let source = r#"
def func(x, y = ...): ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();

        assert_eq!(facts.len(), 1);
        assert!(facts[0].signature.contains("y = ..."));
    }

    #[test]
    fn test_extract_dotted_return_type() {
        let source = r#"
def connect() -> http.client.HTTPConnection: ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();

        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].return_type, "http.client.HTTPConnection");
    }

    // ========================================================================
    // S9B7: Coverage tests for extractor
    // ========================================================================

    #[test]
    fn test_s9b7_extract_posonly_args() {
        let source = r#"
def func(a: int, b: int, /, c: int) -> int: ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        assert_eq!(facts.len(), 1);
        // Should contain the / separator
        assert!(facts[0].signature.contains("/"));
    }

    #[test]
    fn test_s9b7_extract_kwonly_args() {
        let source = r#"
def func(*, key: str) -> None: ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        assert_eq!(facts.len(), 1);
        assert!(facts[0].signature.contains("key: str"));
    }

    #[test]
    fn test_s9b7_extract_constant_type_in_annotation() {
        let source = r#"
def func() -> None: ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        assert_eq!(facts[0].return_type, "None");
    }

    #[test]
    fn test_s9b7_extract_list_type_annotation() {
        let source = r#"
def func() -> [int, str]: ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].return_type, "[int, str]");
    }

    #[test]
    fn test_s9b7_extract_class_private_attribute_excluded() {
        let source = r#"
class MyClass:
    _private: int
    public: str
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        // class + public attribute only
        assert_eq!(facts.len(), 2);
        assert!(!facts.iter().any(|f| f.symbol.contains("_private")));
    }

    #[test]
    fn test_s9b7_extract_class_private_attribute_included() {
        let source = r#"
class MyClass:
    _private: int
    public: str
"#;
        let extractor = Extractor::new().with_private();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        // class + both attributes
        assert_eq!(facts.len(), 3);
    }

    #[test]
    fn test_s9b7_extract_async_method_in_class() {
        let source = r#"
class Service:
    async def process(self, data: bytes) -> str: ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "svc", "test.pyi").unwrap();
        assert_eq!(facts.len(), 2); // class + method
        let method = facts.iter().find(|f| f.kind == TypeFactKind::Method).unwrap();
        assert!(method.signature.starts_with("async "));
        assert_eq!(method.return_type, "str");
    }

    #[test]
    fn test_s9b7_extract_function_with_typed_default() {
        let source = r#"
def func(x: int = ..., y: str = ...) -> None: ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        assert_eq!(facts.len(), 1);
        assert!(facts[0].signature.contains("x: int = ..."));
        assert!(facts[0].signature.contains("y: str = ..."));
    }

    #[test]
    fn test_s9b7_extractor_with_private_flag() {
        let extractor = Extractor::new().with_private();
        let source = "_hidden_func = 1\n";
        // This is not a function, just ensure private filter works on annotated assigns too
        let _ = extractor.extract_source(source, "test", "test.pyi");
    }

    #[test]
    fn test_s9b7_extract_multiple_classes() {
        let source = r#"
class A:
    def method_a(self) -> int: ...

class B:
    def method_b(self) -> str: ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        // 2 classes + 2 methods
        assert_eq!(facts.len(), 4);
    }

    #[test]
    fn test_extract_private_class_excluded() {
        let source = r#"
class _Internal: ...
class Public: ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();

        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].symbol, "Public");
    }

    // ========================================================================
    // DEPYLER-99MODE-S11: Coverage tests for expr_to_string branches,
    // build_signature edge cases, and annotation extraction
    // ========================================================================

    #[test]
    fn test_s11_expr_to_string_constant_int_in_annotation() {
        // Python allows integer literals in type annotations (e.g., Literal[42])
        let source = r#"
x: 42
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].return_type, "42");
    }

    #[test]
    fn test_s11_expr_to_string_constant_float_in_annotation() {
        // Use a value that won't be parsed as int (e.g. 3.5)
        let source = r#"
x: 3.5
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].return_type, "3.5");
    }

    #[test]
    fn test_s11_expr_to_string_constant_bool_true() {
        let source = r#"
x: True
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].return_type, "true");
    }

    #[test]
    fn test_s11_expr_to_string_constant_bool_false() {
        let source = r#"
x: False
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].return_type, "false");
    }

    #[test]
    fn test_s11_expr_to_string_constant_ellipsis() {
        let source = r#"
x: ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].return_type, "...");
    }

    #[test]
    fn test_s11_expr_to_string_constant_string_literal() {
        let source = "x: \"hello\"\n";
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].return_type, "\"hello\"");
    }

    #[test]
    fn test_s11_expr_to_string_constant_none() {
        let source = r#"
x: None
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].return_type, "None");
    }

    #[test]
    fn test_s11_expr_to_string_non_bitor_binop() {
        // Using + operator in annotation should produce "Unknown"
        let source = r#"
x: int + str
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].return_type, "Unknown");
    }

    #[test]
    fn test_s11_extract_tuple_return_type() {
        let source = r#"
def func() -> (int, str, float): ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].return_type, "int, str, float");
    }

    #[test]
    fn test_s11_extract_deeply_nested_subscript() {
        let source = r#"
def func() -> Dict[str, List[Optional[int]]]: ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].return_type, "Dict[str, List[Optional[int]]]");
    }

    #[test]
    fn test_s11_extract_attribute_type_nested() {
        let source = r#"
x: collections.abc.Mapping
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].return_type, "collections.abc.Mapping");
    }

    #[test]
    fn test_s11_extract_union_pipe_chained() {
        let source = r#"
def func() -> int | str | None: ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        assert_eq!(facts.len(), 1);
        // Should chain: (int | str) | None
        assert!(facts[0].return_type.contains("int | str | None"));
    }

    #[test]
    fn test_s11_extract_posonly_with_kwonly_combined() {
        let source = r#"
def func(a: int, b: int, /, c: int, *, d: str) -> None: ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        assert_eq!(facts.len(), 1);
        let sig = &facts[0].signature;
        assert!(sig.contains("a: int"));
        assert!(sig.contains("/"));
        assert!(sig.contains("c: int"));
        assert!(sig.contains("d: str"));
    }

    #[test]
    fn test_s11_extract_class_with_async_private_method_included() {
        let source = r#"
class Service:
    async def _internal(self) -> None: ...
    async def public(self) -> str: ...
"#;
        let extractor = Extractor::new().with_private();
        let facts = extractor.extract_source(source, "svc", "test.pyi").unwrap();
        // class + 2 methods (both included with private flag)
        assert_eq!(facts.len(), 3);
        assert!(facts.iter().any(|f| f.symbol == "Service._internal"));
        assert!(facts.iter().any(|f| f.symbol == "Service.public"));
    }

    #[test]
    fn test_s11_extract_annotated_assign_private_excluded() {
        let source = r#"
_PRIVATE_CONST: int
PUBLIC_CONST: str
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "config", "test.pyi").unwrap();
        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].symbol, "PUBLIC_CONST");
    }

    #[test]
    fn test_s11_extract_annotated_assign_private_included() {
        let source = r#"
_PRIVATE_CONST: int
PUBLIC_CONST: str
"#;
        let extractor = Extractor::new().with_private();
        let facts = extractor.extract_source(source, "config", "test.pyi").unwrap();
        assert_eq!(facts.len(), 2);
    }

    #[test]
    fn test_s11_extract_function_only_varargs() {
        let source = r#"
def func(*args: int, **kwargs: str) -> None: ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        assert_eq!(facts.len(), 1);
        let sig = &facts[0].signature;
        assert!(sig.contains("*args: int"));
        assert!(sig.contains("**kwargs: str"));
    }

    #[test]
    fn test_s11_extract_function_untyped_vararg() {
        let source = r#"
def func(*args) -> None: ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        assert_eq!(facts.len(), 1);
        assert!(facts[0].signature.contains("*args"));
    }

    #[test]
    fn test_s11_extract_function_untyped_kwarg() {
        let source = r#"
def func(**kw) -> None: ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        assert_eq!(facts.len(), 1);
        assert!(facts[0].signature.contains("**kw"));
    }

    #[test]
    fn test_s11_extract_class_with_mixed_statement_types() {
        // Class with methods, async methods, attributes, and ignored statements
        let source = r#"
class MyClass:
    name: str
    async def fetch(self) -> bytes: ...
    def process(self) -> int: ...
    x = 42
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        // class + name attr + fetch method + process method = 4
        // x = 42 is a plain Assign, not AnnAssign, so it's in the _ => {} branch
        assert_eq!(facts.len(), 4);
    }

    #[test]
    fn test_s11_extract_source_with_ignored_statement_types() {
        // Import statements and other non-function/class/annassign stmts are ignored
        let source = r#"
import os
from typing import List
def real_func() -> int: ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].symbol, "real_func");
    }

    #[test]
    fn test_s11_extract_file_nonexistent_path() {
        let extractor = Extractor::new();
        let result =
            extractor.extract_file(Path::new("/nonexistent/path/to/file.pyi"), "nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_s11_extract_list_type_with_nested_elements() {
        let source = r#"
def func() -> [List[int], Dict[str, str]]: ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        assert_eq!(facts.len(), 1);
        assert!(facts[0].return_type.starts_with('['));
        assert!(facts[0].return_type.contains("List[int]"));
    }

    // ========================================================================
    // S12: Deep coverage tests for extractor edge cases
    // ========================================================================

    #[test]
    fn test_s12_extract_function_no_params() {
        // Function with zero parameters
        let source = r#"
def no_args() -> int: ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        assert_eq!(facts.len(), 1);
        assert!(
            facts[0].signature.contains("()"),
            "Expected empty params, got: {}",
            facts[0].signature
        );
    }

    #[test]
    fn test_s12_extract_function_no_return_type() {
        // Function without return annotation - type_to_string returns "None" for missing returns
        let source = r#"
def no_return(x: int): ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].return_type, "None");
    }

    #[test]
    fn test_s12_extract_method_self_excluded_from_sig() {
        // Method with self parameter - self should be excluded from signature
        let source = r#"
class Foo:
    def bar(self, x: int) -> str: ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        let method = facts.iter().find(|f| f.kind == TypeFactKind::Method).unwrap();
        // Self should be filtered from the signature display
        assert!(method.signature.contains("x: int"));
    }

    #[test]
    fn test_s12_extract_class_attribute_fqn() {
        let source = r#"
class Config:
    debug: bool
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        let attr = facts.iter().find(|f| f.kind == TypeFactKind::Attribute).unwrap();
        assert_eq!(attr.symbol, "Config.debug");
        assert_eq!(attr.return_type, "bool");
    }

    #[test]
    fn test_s12_extract_module_attribute() {
        // Top-level annotated assign is a module attribute
        let source = r#"
VERSION: str
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].symbol, "VERSION");
        assert_eq!(facts[0].return_type, "str");
    }

    #[test]
    fn test_s12_extract_generic_subscript() {
        let source = r#"
def func() -> Callable[[int, str], bool]: ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        assert_eq!(facts.len(), 1);
        assert!(facts[0].return_type.contains("Callable"));
    }

    #[test]
    fn test_s12_extractor_default_no_private() {
        let extractor = Extractor::new();
        let source = r#"
def _private() -> None: ...
def public() -> None: ...
"#;
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].symbol, "public");
    }

    #[test]
    fn test_s12_extract_multiple_classes_and_functions() {
        let source = r#"
class A:
    def m(self) -> int: ...

class B:
    x: str
    def n(self) -> str: ...

def standalone(a: int, b: int) -> int: ...
"#;
        let extractor = Extractor::new();
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        // A(class) + A.m(method) + B(class) + B.x(attr) + B.n(method) + standalone(func) = 6
        assert_eq!(facts.len(), 6);
    }

    // ===== Session 12 Batch 30: Additional extractor tests =====

    #[test]
    fn test_s12_extract_file_valid() {
        let dir = tempfile::tempdir().unwrap();
        let stub_path = dir.path().join("test.pyi");
        std::fs::write(&stub_path, "def greet(name: str) -> str: ...\n").unwrap();

        let extractor = Extractor::new();
        let facts = extractor.extract_file(&stub_path, "test").unwrap();
        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].symbol, "greet");
    }

    #[test]
    fn test_s12_extract_file_missing() {
        let extractor = Extractor::new();
        let result = extractor.extract_file(Path::new("/nonexistent/file.pyi"), "test");
        assert!(result.is_err());
    }

    #[test]
    fn test_s12_extract_file_with_class() {
        let dir = tempfile::tempdir().unwrap();
        let stub_path = dir.path().join("mymodule.pyi");
        std::fs::write(
            &stub_path,
            r#"class MyClass:
    def method(self, x: int) -> str: ...
    name: str
"#,
        )
        .unwrap();

        let extractor = Extractor::new();
        let facts = extractor.extract_file(&stub_path, "mymodule").unwrap();
        // MyClass(class) + method(method) + name(attr) = 3
        assert_eq!(facts.len(), 3);
    }

    #[test]
    fn test_s12_extract_source_empty() {
        let extractor = Extractor::new();
        let facts = extractor.extract_source("", "empty", "empty.pyi").unwrap();
        assert!(facts.is_empty());
    }

    #[test]
    fn test_s12_extract_source_comments_only() {
        let extractor = Extractor::new();
        let facts =
            extractor.extract_source("# Just a comment\n", "comments", "comments.pyi").unwrap();
        assert!(facts.is_empty());
    }

    #[test]
    fn test_s12_extract_source_private_excluded() {
        let extractor = Extractor::new();
        let source = "def _private() -> None: ...\ndef public() -> int: ...\n";
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].symbol, "public");
    }

    #[test]
    fn test_s12_extract_source_private_included() {
        let extractor = Extractor::new().with_private();
        let source = "def _private() -> None: ...\ndef public() -> int: ...\n";
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        assert_eq!(facts.len(), 2);
    }

    #[test]
    fn test_s12_extract_source_nested_class() {
        let extractor = Extractor::new();
        let source = r#"class Outer:
    class Inner:
        def inner_method(self) -> int: ...
    def outer_method(self) -> str: ...
"#;
        let facts = extractor.extract_source(source, "test", "test.pyi").unwrap();
        assert!(facts.len() >= 2);
    }

    #[test]
    fn test_s12_extract_source_complex_signatures() {
        let extractor = Extractor::new();
        let source = r#"
def foo(a: int, b: str, c: float = 0.0) -> bool: ...
def bar(items: list, key: str) -> dict: ...
def baz(*args, **kwargs) -> None: ...
"#;
        let facts = extractor.extract_source(source, "sigs", "sigs.pyi").unwrap();
        assert_eq!(facts.len(), 3);
    }

    #[test]
    fn test_s12_extract_source_module_path() {
        let extractor = Extractor::new();
        let source = "def greet(name: str) -> str: ...\n";
        let facts = extractor.extract_source(source, "my.nested.module", "module.pyi").unwrap();
        assert_eq!(facts.len(), 1);
        // Check that fqn includes the module path
        let fqn = facts[0].fqn();
        assert!(fqn.contains("my.nested.module"));
    }
}
