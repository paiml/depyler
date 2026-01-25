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
        Self {
            include_private: false,
        }
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
        let parsed =
            parse(source, Mode::Module, filename).map_err(|e| KnowledgeError::StubParseError {
                file: filename.to_string(),
                message: e.to_string(),
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

        Some(TypeFact::method(
            module,
            class_name,
            &method.name,
            &signature,
            &return_type,
        ))
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
        let type_str = arg
            .def
            .annotation
            .as_ref()
            .map(|a| self.expr_to_string(a))
            .unwrap_or_default();

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
        let type_str = arg
            .annotation
            .as_ref()
            .map(|a| self.expr_to_string(a))
            .unwrap_or_default();

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
        let facts = extractor
            .extract_source(source, "requests", "test.pyi")
            .unwrap();

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
        let facts = extractor
            .extract_source(source, "requests", "test.pyi")
            .unwrap();

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
        let facts = extractor
            .extract_source(source, "requests.models", "test.pyi")
            .unwrap();

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
        let facts = extractor
            .extract_source(source, "test", "test.pyi")
            .unwrap();

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
        let facts = extractor
            .extract_source(source, "test", "test.pyi")
            .unwrap();

        assert_eq!(facts.len(), 2);
    }

    #[test]
    fn test_extract_kwargs() {
        let source = r#"
def get(url: str, **kwargs) -> Response: ...
"#;
        let extractor = Extractor::new();
        let facts = extractor
            .extract_source(source, "requests", "test.pyi")
            .unwrap();

        assert!(facts[0].signature.contains("**kwargs"));
    }
}
