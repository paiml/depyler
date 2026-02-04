use crate::hir::*;
use anyhow::{bail, Result};
use depyler_annotations::{AnnotationExtractor, AnnotationParser, TranspilationAnnotations};
use rustpython_ast::{self as ast};

mod converters;
mod properties;
mod type_extraction;

pub use converters::{ExprConverter, StmtConverter};
pub use properties::FunctionAnalyzer;
pub use type_extraction::TypeExtractor;

/// Bridge between Python AST and Depyler HIR
///
/// The `AstBridge` converts Python AST nodes to Depyler's High-level Intermediate Representation (HIR).
/// This is a critical component that handles the semantic analysis and type inference during transpilation.
///
/// # Examples
///
/// Basic usage with a simple function:
///
/// ```rust
/// use depyler_core::ast_bridge::AstBridge;
/// use rustpython_parser::{parse, Mode};
///
/// let python_code = r#"
/// def add(a: int, b: int) -> int:
///     return a + b
/// "#;
///
/// // Parse Python code to AST
/// let ast = parse(python_code, Mode::Module, "<test>").unwrap();
///
/// // Convert to HIR
/// let bridge = AstBridge::new();
/// let (hir, _type_env) = bridge.python_to_hir(ast).unwrap();
///
/// assert_eq!(hir.functions.len(), 1);
/// assert_eq!(hir.functions[0].name, "add");
/// assert_eq!(hir.functions[0].params.len(), 2);
/// ```
///
/// With source tracking for better error reporting:
///
/// ```rust
/// use depyler_core::ast_bridge::AstBridge;
/// use rustpython_parser::{parse, Mode};
///
/// let python_code = r#"
/// def greet(name: str) -> str:
///     return "Hello, " + name + "!"
/// "#;
///
/// let ast = parse(python_code, Mode::Module, "<example>").unwrap();
///
/// let bridge = AstBridge::new()
///     .with_source(python_code.to_string());
///
/// let (hir, _type_env) = bridge.python_to_hir(ast).unwrap();
/// assert_eq!(hir.functions[0].name, "greet");
/// ```
pub struct AstBridge {
    source_code: Option<String>,
    annotation_extractor: AnnotationExtractor,
    annotation_parser: AnnotationParser,
    type_env: crate::type_system::type_environment::TypeEnvironment,
}

impl Default for AstBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl AstBridge {
    /// Creates a new AST bridge with default configuration
    ///
    /// # Examples
    ///
    /// ```rust
    /// use depyler_core::ast_bridge::AstBridge;
    ///
    /// let bridge = AstBridge::new();
    /// // Ready to convert Python AST to HIR
    /// ```
    pub fn new() -> Self {
        Self {
            source_code: None,
            annotation_extractor: AnnotationExtractor::new(),
            annotation_parser: AnnotationParser::new(),
            type_env: crate::type_system::type_environment::TypeEnvironment::new(),
        }
    }

    /// Sets the source code for better error reporting and debugging
    ///
    /// # Arguments
    ///
    /// * `source` - The original Python source code
    ///
    /// # Examples
    ///
    /// ```rust
    /// use depyler_core::ast_bridge::AstBridge;
    ///
    /// let python_code = "def hello(): return 'world'";
    /// let bridge = AstBridge::new()
    ///     .with_source(python_code.to_string());
    /// ```
    pub fn with_source(mut self, source: String) -> Self {
        self.source_code = Some(source);
        self
    }

    /// Converts a Python AST module to Depyler HIR
    ///
    /// This is the main entry point for AST to HIR conversion. It handles semantic analysis,
    /// type inference, and creates the intermediate representation used by the transpiler.
    ///
    /// # Arguments
    ///
    /// * `module` - The Python AST module to convert
    ///
    /// # Returns
    ///
    /// Returns a `HirModule` containing functions, classes, and other declarations,
    /// or an error if the conversion fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use depyler_core::ast_bridge::AstBridge;
    /// use rustpython_parser::{parse, Mode};
    ///
    /// let python_code = r#"
    /// def fibonacci(n: int) -> int:
    ///     if n <= 1:
    ///         return n
    ///     return fibonacci(n - 1) + fibonacci(n - 2)
    /// "#;
    ///
    /// let ast = parse(python_code, Mode::Module, "<test>").unwrap();
    /// let bridge = AstBridge::new();
    /// let (hir, _type_env) = bridge.python_to_hir(ast).unwrap();
    ///
    /// assert_eq!(hir.functions.len(), 1);
    /// assert_eq!(hir.functions[0].name, "fibonacci");
    /// assert!(hir.functions[0].body.len() > 0);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The AST contains unsupported Python constructs
    /// - Type annotations are malformed
    /// - Function signatures are invalid
    ///
    /// Converts a Python AST module to Depyler HIR with type environment.
    ///
    /// # Returns
    ///
    /// Returns a tuple of (HirModule, TypeEnvironment) where TypeEnvironment contains
    /// all type annotations collected during HIR generation.
    pub fn python_to_hir(
        mut self,
        module: ast::Mod,
    ) -> Result<(
        HirModule,
        crate::type_system::type_environment::TypeEnvironment,
    )> {
        let hir = match module {
            ast::Mod::Module(m) => self.convert_module(m)?,
            _ => bail!("Only module-level code is supported"),
        };
        Ok((hir, self.type_env))
    }

    fn convert_module(&mut self, module: ast::ModModule) -> Result<HirModule> {
        let mut functions = Vec::new();
        let mut imports = Vec::new();
        let mut type_aliases = Vec::new();
        let mut protocols = Vec::new();
        let mut classes = Vec::new();
        let mut constants = Vec::new();
        // DEPYLER-1216: Capture top-level statements for script-style Python
        let mut top_level_stmts = Vec::new();

        for stmt in module.body {
            match stmt {
                ast::Stmt::FunctionDef(f) => {
                    functions.push(self.convert_function(f, false)?);
                }
                ast::Stmt::Import(i) => {
                    imports.extend(convert_import(i)?);
                }
                ast::Stmt::ImportFrom(i) => {
                    imports.extend(convert_import_from(i)?);
                }
                ast::Stmt::AsyncFunctionDef(f) => {
                    functions.push(self.convert_async_function(f)?);
                }
                ast::Stmt::ClassDef(class) => {
                    // Try to parse as protocol first
                    if let Some(protocol) = self.try_convert_protocol(&class)? {
                        protocols.push(protocol);
                    } else {
                        // Convert regular class
                        if let Some(hir_class) = self.try_convert_class(&class)? {
                            classes.push(hir_class);
                        }
                    }
                }
                ast::Stmt::Assign(assign) => {
                    // Try to parse as type alias first
                    if let Some(type_alias) = self.try_convert_type_alias(&assign)? {
                        type_aliases.push(type_alias);
                    } else {
                        // Otherwise, treat as module-level constant
                        if let Some(constant) = self.try_convert_constant(&assign)? {
                            constants.push(constant);
                        }
                    }
                }
                ast::Stmt::AnnAssign(ann_assign) => {
                    // Try to parse annotated assignment as type alias first
                    if let Some(type_alias) = self.try_convert_annotated_type_alias(&ann_assign)? {
                        type_aliases.push(type_alias);
                    } else {
                        // Otherwise, treat as annotated module-level constant
                        if let Some(constant) = self.try_convert_annotated_constant(&ann_assign)? {
                            constants.push(constant);
                        }
                    }
                }
                // DEPYLER-1155: Handle `if __name__ == "__main__":` pattern
                // Convert to a `fn main()` function that contains the block body
                // BUT only if there's no `def main():` already defined
                ast::Stmt::If(if_stmt) => {
                    // DEPYLER-CONVERGE-MULTI: Skip `if TYPE_CHECKING:` blocks.
                    // These contain import-time-only type hints that have no
                    // runtime meaning and produce E0425 in generated Rust.
                    if is_type_checking_guard(&if_stmt) {
                        continue;
                    }
                    let has_main_function = functions.iter().any(|f| f.name == "main");
                    if !has_main_function {
                        if let Some(main_fn) = self.try_convert_if_main(&if_stmt)? {
                            functions.push(main_fn);
                        } else {
                            // DEPYLER-1216: Not an `if __name__ == "__main__":` pattern,
                            // so capture as a top-level statement for script-style Python
                            if let Ok(hir_stmt) = convert_stmt(ast::Stmt::If(if_stmt)) {
                                top_level_stmts.push(hir_stmt);
                            }
                        }
                    }
                }
                // DEPYLER-1216: Capture executable top-level statements for script-style Python
                // These will be wrapped into a synthetic main() if no explicit main exists
                ast::Stmt::Expr(expr) => {
                    if let Ok(hir_stmt) = convert_stmt(ast::Stmt::Expr(expr)) {
                        top_level_stmts.push(hir_stmt);
                    }
                }
                ast::Stmt::For(for_stmt) => {
                    if let Ok(hir_stmt) = convert_stmt(ast::Stmt::For(for_stmt)) {
                        top_level_stmts.push(hir_stmt);
                    }
                }
                ast::Stmt::While(while_stmt) => {
                    if let Ok(hir_stmt) = convert_stmt(ast::Stmt::While(while_stmt)) {
                        top_level_stmts.push(hir_stmt);
                    }
                }
                ast::Stmt::Try(try_stmt) => {
                    if let Ok(hir_stmt) = convert_stmt(ast::Stmt::Try(try_stmt)) {
                        top_level_stmts.push(hir_stmt);
                    }
                }
                ast::Stmt::With(with_stmt) => {
                    if let Ok(hir_stmt) = convert_stmt(ast::Stmt::With(with_stmt)) {
                        top_level_stmts.push(hir_stmt);
                    }
                }
                ast::Stmt::Return(ret_stmt) => {
                    // Top-level return (unusual but valid in scripts executed via exec())
                    if let Ok(hir_stmt) = convert_stmt(ast::Stmt::Return(ret_stmt)) {
                        top_level_stmts.push(hir_stmt);
                    }
                }
                _ => {
                    // Skip other statements (Pass, Break, Continue, etc.)
                }
            }
        }

        // DEPYLER-0359: Propagate can_fail through function calls
        // If a function calls another function that can fail, mark it as can_fail too
        propagate_can_fail_through_calls(&mut functions);

        Ok(HirModule {
            functions,
            imports,
            type_aliases,
            protocols,
            classes,
            constants,
            top_level_stmts,
        })
    }

    fn convert_function(
        &mut self,
        func: ast::StmtFunctionDef,
        is_async: bool,
    ) -> Result<HirFunction> {
        let name = func.name.to_string();
        let params = convert_parameters(&func.args)?;

        // DEPYLER-0500: Collect parameter type annotations
        for param in &params {
            self.type_env.bind_var(&param.name, param.ty.clone());
        }

        let ret_type = TypeExtractor::extract_return_type(&func.returns)?;

        // Extract annotations from source code if available
        let annotations = self.extract_function_annotations(&func);

        // Extract docstring and filter it from the body
        let (docstring, filtered_body) = extract_docstring_and_body(func.body)?;
        let mut properties = FunctionAnalyzer::analyze(&filtered_body);
        properties.is_async = is_async;

        Ok(HirFunction {
            name,
            params: params.into(),
            ret_type,
            body: filtered_body,
            properties,
            annotations,
            docstring,
        })
    }

    fn convert_async_function(&mut self, func: ast::StmtAsyncFunctionDef) -> Result<HirFunction> {
        let name = func.name.to_string();
        let params = convert_parameters(&func.args)?;

        // DEPYLER-0500: Collect parameter type annotations
        for param in &params {
            self.type_env.bind_var(&param.name, param.ty.clone());
        }

        let ret_type = TypeExtractor::extract_return_type(&func.returns)?;

        // Extract annotations from source code if available
        let annotations = self.extract_async_function_annotations(&func);

        // Extract docstring and filter it from the body
        let (docstring, filtered_body) = extract_docstring_and_body(func.body)?;
        let mut properties = FunctionAnalyzer::analyze(&filtered_body);
        properties.is_async = true;

        Ok(HirFunction {
            name,
            params: params.into(),
            ret_type,
            body: filtered_body,
            properties,
            annotations,
            docstring,
        })
    }

    fn extract_function_annotations(
        &self,
        func: &ast::StmtFunctionDef,
    ) -> TranspilationAnnotations {
        // Try to extract from source code comments first
        if let Some(source) = &self.source_code {
            if let Some(annotation_text) = self
                .annotation_extractor
                .extract_function_annotations(source, &func.name)
            {
                if let Ok(annotations) = self.annotation_parser.parse_annotations(&annotation_text)
                {
                    return annotations;
                }
            }
        }

        // Fallback: Try to extract from docstring if present
        if let Some(ast::Stmt::Expr(expr)) = func.body.first() {
            if let ast::Expr::Constant(constant) = expr.value.as_ref() {
                if let ast::Constant::Str(docstring) = &constant.value {
                    if let Ok(annotations) = self.annotation_parser.parse_annotations(docstring) {
                        return annotations;
                    }
                }
            }
        }

        TranspilationAnnotations::default()
    }

    fn extract_async_function_annotations(
        &self,
        func: &ast::StmtAsyncFunctionDef,
    ) -> TranspilationAnnotations {
        // Try to extract from source code comments first
        if let Some(source) = &self.source_code {
            if let Some(annotation_text) = self
                .annotation_extractor
                .extract_function_annotations(source, &func.name)
            {
                if let Ok(annotations) = self.annotation_parser.parse_annotations(&annotation_text)
                {
                    return annotations;
                }
            }
        }

        // Fallback: Try to extract from docstring if present
        if let Some(ast::Stmt::Expr(expr)) = func.body.first() {
            if let ast::Expr::Constant(constant) = expr.value.as_ref() {
                if let ast::Constant::Str(docstring) = &constant.value {
                    if let Ok(annotations) = self.annotation_parser.parse_annotations(docstring) {
                        return annotations;
                    }
                }
            }
        }

        TranspilationAnnotations::default()
    }

    fn try_convert_type_alias(&mut self, assign: &ast::StmtAssign) -> Result<Option<TypeAlias>> {
        // Look for patterns like: UserId = int or UserId = NewType('UserId', int)
        if assign.targets.len() != 1 {
            return Ok(None); // Skip multiple assignment targets
        }

        let target = match &assign.targets[0] {
            ast::Expr::Name(name) => name.id.as_str(),
            _ => return Ok(None), // Skip complex assignment targets
        };

        // Check if this looks like a type alias (simple assignment of a type)
        let (target_type, is_newtype) = match assign.value.as_ref() {
            // Simple alias: UserId = int
            ast::Expr::Name(n) => {
                let type_name = n.id.as_str();
                if self.is_type_name(type_name) {
                    (TypeExtractor::extract_simple_type(type_name)?, false)
                } else {
                    return Ok(None); // Not a type name
                }
            }
            // Generic alias: UserId = Optional[int]
            // DEPYLER-0503: Only treat subscripts with type base (Optional, List, etc.) as type aliases
            // Regular value subscripts like items[0] should return None (not a type alias)
            ast::Expr::Subscript(s) => {
                // Check if the base is a type name
                if let ast::Expr::Name(base_name) = s.value.as_ref() {
                    if self.is_type_name(base_name.id.as_str()) {
                        (TypeExtractor::extract_type(&assign.value)?, false)
                    } else {
                        return Ok(None); // Base is variable, not a type - not a type alias
                    }
                } else {
                    return Ok(None); // Complex base expression - not a type alias
                }
            }
            // NewType pattern: UserId = NewType('UserId', int)
            ast::Expr::Call(call) => {
                if let ast::Expr::Name(func_name) = call.func.as_ref() {
                    if func_name.id.as_str() == "NewType" && call.args.len() == 2 {
                        // Second argument should be the base type
                        let base_type = TypeExtractor::extract_type(&call.args[1])?;
                        (base_type, true)
                    } else {
                        return Ok(None); // Not a NewType call
                    }
                } else {
                    return Ok(None); // Complex function call
                }
            }
            _ => return Ok(None), // Not a type alias pattern
        };

        Ok(Some(TypeAlias {
            name: target.to_string(),
            target_type,
            is_newtype,
        }))
    }

    fn try_convert_annotated_type_alias(
        &self,
        ann_assign: &ast::StmtAnnAssign,
    ) -> Result<Option<TypeAlias>> {
        // Look for patterns like: UserId: TypeAlias = int
        let target = match ann_assign.target.as_ref() {
            ast::Expr::Name(name) => name.id.as_str(),
            _ => return Ok(None), // Skip complex assignment targets
        };

        // Check if annotation is TypeAlias
        let is_type_alias = match ann_assign.annotation.as_ref() {
            ast::Expr::Name(n) => n.id.as_str() == "TypeAlias",
            _ => false,
        };

        if !is_type_alias {
            return Ok(None); // Not explicitly marked as TypeAlias
        }

        if let Some(value) = &ann_assign.value {
            let (target_type, is_newtype) = match value.as_ref() {
                // Simple alias: UserId: TypeAlias = int
                ast::Expr::Name(n) => {
                    let type_name = n.id.as_str();
                    (TypeExtractor::extract_simple_type(type_name)?, false)
                }
                // Generic alias: UserId: TypeAlias = Optional[int]
                ast::Expr::Subscript(_) => (TypeExtractor::extract_type(value)?, false),
                // NewType pattern: UserId: TypeAlias = NewType('UserId', int)
                ast::Expr::Call(call) => {
                    if let ast::Expr::Name(func_name) = call.func.as_ref() {
                        if func_name.id.as_str() == "NewType" && call.args.len() == 2 {
                            let base_type = TypeExtractor::extract_type(&call.args[1])?;
                            (base_type, true)
                        } else {
                            return Ok(None);
                        }
                    } else {
                        return Ok(None);
                    }
                }
                _ => return Ok(None),
            };

            Ok(Some(TypeAlias {
                name: target.to_string(),
                target_type,
                is_newtype,
            }))
        } else {
            Ok(None) // No value assigned
        }
    }

    /// Try to convert a simple assignment to a module-level constant
    fn try_convert_constant(&mut self, assign: &ast::StmtAssign) -> Result<Option<HirConstant>> {
        // Only handle single assignment targets
        if assign.targets.len() != 1 {
            return Ok(None);
        }

        let name = match &assign.targets[0] {
            ast::Expr::Name(n) => n.id.to_string(),
            _ => return Ok(None), // Skip complex assignment targets
        };

        // Convert the value expression
        let value = convert_expr(*assign.value.clone())?;

        Ok(Some(HirConstant {
            name,
            value,
            type_annotation: None,
        }))
    }

    /// Try to convert an annotated assignment to a module-level constant
    fn try_convert_annotated_constant(
        &mut self,
        ann_assign: &ast::StmtAnnAssign,
    ) -> Result<Option<HirConstant>> {
        let name = match ann_assign.target.as_ref() {
            ast::Expr::Name(n) => n.id.to_string(),
            _ => return Ok(None), // Skip complex assignment targets
        };

        // Extract type annotation
        let type_annotation = Some(TypeExtractor::extract_type(&ann_assign.annotation)?);

        // DEPYLER-0500 Phase 2: Bind type annotation to TypeEnvironment
        if let Some(ref ty) = type_annotation {
            self.type_env.bind_var(&name, ty.clone());
        }

        // Get the value (annotated assignments at module level should have values)
        if let Some(value_expr) = &ann_assign.value {
            let value = convert_expr(*value_expr.clone())?;

            Ok(Some(HirConstant {
                name,
                value,
                type_annotation,
            }))
        } else {
            Ok(None) // No value, skip it
        }
    }

    /// DEPYLER-1155: Handle `if __name__ == "__main__":` pattern
    ///
    /// This pattern is the standard Python entry point. We convert it to a `fn main()`
    /// function so that the transpiled Rust code can be executed as a binary.
    ///
    /// Python:
    /// ```python
    /// if __name__ == "__main__":
    ///     analyze_dataset()
    /// ```
    ///
    /// Rust:
    /// ```rust,ignore
    /// fn main() {
    ///     analyze_dataset();
    /// }
    /// ```
    fn try_convert_if_main(&mut self, if_stmt: &ast::StmtIf) -> Result<Option<HirFunction>> {
        // Check if condition is `__name__ == "__main__"`
        if !self.is_main_guard(&if_stmt.test) {
            return Ok(None);
        }

        // Convert the body of the if statement to HirStmts
        let body: Vec<HirStmt> = if_stmt
            .body
            .iter()
            .filter_map(|stmt| convert_stmt(stmt.clone()).ok())
            .collect();

        // Create a main function with the body
        Ok(Some(HirFunction {
            name: "main".to_string(),
            params: smallvec::smallvec![],
            ret_type: Type::None, // Rust main() returns ()
            body,
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        }))
    }

    /// DEPYLER-1155: Check if expression is `__name__ == "__main__"`
    fn is_main_guard(&self, expr: &ast::Expr) -> bool {
        match expr {
            ast::Expr::Compare(compare) => {
                // Check left side is `__name__`
                let left_is_name = match compare.left.as_ref() {
                    ast::Expr::Name(name) => name.id.as_str() == "__name__",
                    _ => false,
                };

                // Check operator is `==`
                let op_is_eq = compare.ops.len() == 1 && matches!(compare.ops[0], ast::CmpOp::Eq);

                // Check right side is `"__main__"`
                let right_is_main = compare.comparators.len() == 1
                    && match &compare.comparators[0] {
                        ast::Expr::Constant(c) => match &c.value {
                            ast::Constant::Str(s) => s.as_str() == "__main__",
                            _ => false,
                        },
                        _ => false,
                    };

                left_is_name && op_is_eq && right_is_main
            }
            _ => false,
        }
    }

    fn is_type_name(&self, name: &str) -> bool {
        matches!(
            name,
            "int"
                | "float"
                | "str"
                | "bool"
                | "None"
                | "list"
                | "dict"
                | "tuple"
                | "set"
                | "frozenset"
                | "List"
                | "Dict"
                | "Tuple"
                | "Set"
                | "FrozenSet"
                | "Optional"
                | "Union"
                | "Callable"
                | "Any"
                | "TypeVar"
        )
    }

    fn try_convert_protocol(&mut self, class: &ast::StmtClassDef) -> Result<Option<Protocol>> {
        // Check if this class inherits from Protocol
        let is_protocol = class
            .bases
            .iter()
            .any(|base| matches!(base, ast::Expr::Name(n) if n.id.as_str() == "Protocol"));

        if !is_protocol {
            return Ok(None);
        }

        let name = class.name.to_string();

        // Extract type parameters from class definition
        let type_params = self.extract_class_type_params(class);

        // Check for @runtime_checkable decorator
        let is_runtime_checkable = class.decorator_list.iter().any(|decorator| {
            matches!(decorator, ast::Expr::Name(n) if n.id.as_str() == "runtime_checkable")
        });

        // Extract methods from class body
        let mut methods = Vec::new();
        for stmt in &class.body {
            if let ast::Stmt::FunctionDef(func) = stmt {
                // Skip special methods like __init__, but include abstract methods
                if !func.name.as_str().starts_with("__") || func.name.as_str() == "__call__" {
                    let method = self.convert_protocol_method(func)?;
                    methods.push(method);
                }
            }
        }

        Ok(Some(Protocol {
            name,
            type_params,
            methods,
            is_runtime_checkable,
        }))
    }

    fn try_convert_class(&mut self, class: &ast::StmtClassDef) -> Result<Option<HirClass>> {
        // Extract docstring if present
        let docstring = self.extract_class_docstring(&class.body);

        // Check if it's a dataclass
        // DEPYLER-0839: Also check for @dataclass(frozen=True) and other parameterized forms
        let is_dataclass = class.decorator_list.iter().any(|d| {
            match d {
                // Simple @dataclass
                ast::Expr::Name(n) => n.id.as_str() == "dataclass",
                // Module-qualified @dataclasses.dataclass
                ast::Expr::Attribute(a) => a.attr.as_str() == "dataclass",
                // Parameterized @dataclass(frozen=True), @dataclass(slots=True), etc.
                ast::Expr::Call(c) => match c.func.as_ref() {
                    ast::Expr::Name(n) => n.id.as_str() == "dataclass",
                    ast::Expr::Attribute(a) => a.attr.as_str() == "dataclass",
                    _ => false,
                },
                _ => false,
            }
        });

        // DEPYLER-0841: Extract base classes including subscript expressions
        // For `class Left(Either[L, R])`, capture "Either[L, R]"
        // For `class Either(ABC, Generic[L, R])`, capture both "ABC" and "Generic[L, R]"
        let base_classes: Vec<String> = class
            .bases
            .iter()
            .filter_map(|base| match base {
                // Simple name like ABC
                ast::Expr::Name(n) => Some(n.id.to_string()),
                // Generic subscript like Generic[L, R] or Either[L, R]
                ast::Expr::Subscript(subscript) => {
                    if let ast::Expr::Name(n) = subscript.value.as_ref() {
                        // Format as "Name[params]" for full representation
                        let params = self.format_subscript_slice(&subscript.slice);
                        Some(format!("{}[{}]", n.id, params))
                    } else {
                        None
                    }
                }
                // DEPYLER-1403: Handle module.Class pattern (e.g., enum.Enum, abc.ABC)
                ast::Expr::Attribute(attr) => {
                    // Format as "module.Class" (e.g., "enum.Enum")
                    if let ast::Expr::Name(module) = attr.value.as_ref() {
                        Some(format!("{}.{}", module.id, attr.attr))
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect();

        // DEPYLER-0739: Extract type parameters from Generic[T, U, ...] base class
        let type_params = self.extract_class_type_params(class);

        // Convert methods and fields
        let mut methods = Vec::new();
        let mut fields = Vec::new();
        let mut init_method = None;
        // DEPYLER-0603: Collect ALL methods for field inference (not just __init__)
        let mut all_methods: Vec<&ast::StmtFunctionDef> = Vec::new();

        for stmt in &class.body {
            match stmt {
                ast::Stmt::FunctionDef(method) => {
                    if method.name.as_str() == "__init__" {
                        // Store __init__ for field inference (takes priority for type info)
                        init_method = Some(method);
                    }
                    // DEPYLER-0603: Store all methods for comprehensive field detection
                    all_methods.push(method);
                    if let Some(hir_method) = self.convert_method(method, false)? {
                        methods.push(hir_method);
                    }
                }
                ast::Stmt::AsyncFunctionDef(method) => {
                    if let Some(hir_method) = self.convert_async_method(method)? {
                        methods.push(hir_method);
                    }
                }
                ast::Stmt::AnnAssign(ann_assign) => {
                    // Handle annotated fields (class attributes)
                    if let ast::Expr::Name(target) = ann_assign.target.as_ref() {
                        let field_name = target.id.to_string();
                        let field_type = TypeExtractor::extract_type(&ann_assign.annotation)?;

                        // DEPYLER-0714: For dataclasses, fields with defaults are INSTANCE fields
                        // For regular classes, fields with defaults are class constants
                        // Only ClassVar[T] annotations should be class variables in dataclasses
                        let (is_class_var, default_value) = if let Some(value) = &ann_assign.value {
                            // Convert the default value expression
                            let converted_value = ExprConverter::convert(value.as_ref().clone())?;
                            // In dataclasses, annotated fields with defaults are instance fields
                            // In regular classes, they are class constants
                            let is_class_constant = !is_dataclass;
                            (is_class_constant, Some(converted_value))
                        } else {
                            // Instance attribute - no default value
                            (false, None)
                        };

                        fields.push(HirField {
                            name: field_name,
                            field_type,
                            default_value,
                            is_class_var,
                        });
                    }
                }
                // DEPYLER-0422 Fix #11: Handle simple assignments for IntEnum members
                // Five-Whys Root Cause:
                // 1. Why: No constants generated for IntEnum classes
                // 2. Why: IntEnum members like RED = 1 aren't being extracted
                // 3. Why: Code only handles AnnAssign, not Assign
                // 4. Why: IntEnum uses simple assignments without type annotations
                // 5. ROOT CAUSE: Missing ast::Stmt::Assign handling for enum classes
                ast::Stmt::Assign(assign) => {
                    // Check if this class inherits from IntEnum or Enum
                    // DEPYLER-1403: Also handle enum.Enum, enum.IntEnum module patterns
                    let is_enum_class = base_classes.iter().any(|b| {
                        matches!(
                            b.as_str(),
                            "IntEnum"
                                | "Enum"
                                | "StrEnum"
                                | "enum.Enum"
                                | "enum.IntEnum"
                                | "enum.StrEnum"
                        )
                    });

                    if is_enum_class {
                        // Extract enum members from simple assignments
                        for target in &assign.targets {
                            if let ast::Expr::Name(name) = target {
                                let field_name = name.id.to_string();

                                // Infer enum member type from value expression
                                let field_type = match assign.value.as_ref() {
                                    ast::Expr::Constant(c) => match &c.value {
                                        ast::Constant::Str(_) => Type::String,
                                        ast::Constant::Float(_) => Type::Float,
                                        _ => Type::Int,
                                    },
                                    _ => Type::Int,
                                };

                                // Convert the value expression
                                let converted_value =
                                    ExprConverter::convert(assign.value.as_ref().clone())?;

                                fields.push(HirField {
                                    name: field_name,
                                    field_type,
                                    default_value: Some(converted_value),
                                    is_class_var: true, // Enum members are class constants
                                });
                            }
                        }
                    }
                    // For non-enum classes, skip simple assignments (they're usually local to methods)
                }
                _ => {
                    // Skip other statements for now
                }
            }
        }

        // DEPYLER-0603: Infer instance fields from ALL methods, not just __init__
        // Check if we have any instance fields (non-class-var fields)
        let has_instance_fields = fields.iter().any(|f| !f.is_class_var);

        if !has_instance_fields && !is_dataclass {
            // First try __init__ (has parameter type info)
            if let Some(init) = init_method {
                let inferred_fields = self.infer_fields_from_init(init)?;
                fields.extend(inferred_fields);
            }

            // DEPYLER-0603: Also scan ALL other methods for self.field assignments
            // This catches fields assigned in __enter__, __exit__, and other methods
            let existing_field_names: std::collections::HashSet<String> =
                fields.iter().map(|f| f.name.clone()).collect();

            for method in &all_methods {
                // Skip __init__ since we already processed it
                if method.name.as_str() == "__init__" {
                    continue;
                }
                let method_fields = self.infer_fields_from_method(method)?;
                for field in method_fields {
                    // Deduplicate: only add if not already present
                    if !existing_field_names.contains(&field.name)
                        && !fields.iter().any(|f| f.name == field.name)
                    {
                        fields.push(field);
                    }
                }
            }
        }

        Ok(Some(HirClass {
            name: class.name.to_string(),
            base_classes,
            methods,
            fields,
            is_dataclass,
            docstring,
            type_params, // DEPYLER-0739: Generic type parameters
        }))
    }

    fn convert_method(
        &mut self,
        method: &ast::StmtFunctionDef,
        is_async: bool,
    ) -> Result<Option<HirMethod>> {
        use smallvec::smallvec;

        let name = method.name.to_string();

        // DEPYLER-0967: Skip dunder methods except commonly-needed ones
        // Allow: __init__, __iter__, __next__, __enter__, __exit__ (context managers)
        // Allow: __len__, __str__, __repr__, __getitem__, __setitem__, __contains__ (collections)
        // Allow: __eq__, __ne__, __lt__, __le__, __gt__, __ge__, __hash__ (comparisons)
        // Allow: __add__, __sub__, __mul__, __truediv__, __neg__ (operators)
        if name.starts_with("__")
            && name.ends_with("__")
            && !matches!(
                name.as_str(),
                "__init__"
                    | "__iter__"
                    | "__next__"
                    | "__enter__"
                    | "__exit__"
                    | "__len__"
                    | "__str__"
                    | "__repr__"
                    | "__getitem__"
                    | "__setitem__"
                    | "__contains__"
                    | "__eq__"
                    | "__ne__"
                    | "__lt__"
                    | "__le__"
                    | "__gt__"
                    | "__ge__"
                    | "__hash__"
                    | "__add__"
                    | "__sub__"
                    | "__mul__"
                    | "__truediv__"
                    | "__neg__"
            )
        {
            return Ok(None);
        }

        // Extract docstring
        let docstring = self.extract_class_docstring(&method.body);

        // Check decorators
        let is_static = method
            .decorator_list
            .iter()
            .any(|d| matches!(d, ast::Expr::Name(n) if n.id.as_str() == "staticmethod"));
        let is_classmethod = method
            .decorator_list
            .iter()
            .any(|d| matches!(d, ast::Expr::Name(n) if n.id.as_str() == "classmethod"));
        let is_property = method
            .decorator_list
            .iter()
            .any(|d| matches!(d, ast::Expr::Name(n) if n.id.as_str() == "property"));

        // Convert parameters (skip 'self' for regular methods, 'cls' for classmethods)
        let mut params = smallvec![];
        let skip_first = if is_static {
            false
        } else if is_classmethod {
            // Skip 'cls' parameter for classmethods
            method
                .args
                .args
                .first()
                .map(|arg| arg.def.arg.as_str() == "cls")
                .unwrap_or(false)
        } else {
            // Skip 'self' parameter for instance methods
            method
                .args
                .args
                .first()
                .map(|arg| arg.def.arg.as_str() == "self")
                .unwrap_or(false)
        };

        let args_to_process = if skip_first {
            &method.args.args[1..]
        } else {
            &method.args.args[..]
        };

        for arg in args_to_process {
            let param_name = arg.def.arg.to_string();
            let param_type = if let Some(ann) = &arg.def.annotation {
                TypeExtractor::extract_type(ann)?
            } else {
                Type::Unknown
            };
            params.push(HirParam {
                name: param_name,
                ty: param_type,
                default: None, // Note: Method defaults extraction requires AST alignment with convert_parameters()
                is_vararg: false, // DEPYLER-0477: Regular parameter
            });
        }

        // DEPYLER-0841: Extract varargs parameter (*args) for methods
        if let Some(vararg) = &method.args.vararg {
            let name = vararg.arg.to_string();
            // Start with List<String> as a reasonable default
            let ty = Type::List(Box::new(Type::String));
            params.push(HirParam {
                name,
                ty,
                default: None,
                is_vararg: true,
            });
        }

        // Convert return type
        let ret_type = if let Some(ret) = &method.returns {
            TypeExtractor::extract_type(ret)?
        } else if self.check_returns_self(&method.body) {
            // If method returns self without annotation, infer &Self
            Type::Custom("&Self".to_string())
        } else {
            Type::None
        };

        // Convert body (filter out docstring)
        let filtered_body = if docstring.is_some() && method.body.len() > 1 {
            // Skip first statement if it's the docstring
            method.body[1..].to_vec()
        } else if docstring.is_some() && method.body.len() == 1 {
            // Only docstring, no actual body
            vec![]
        } else {
            method.body.clone()
        };
        let body = convert_body(filtered_body)?;

        Ok(Some(HirMethod {
            name,
            params,
            ret_type,
            body,
            is_static,
            is_classmethod,
            is_property,
            is_async,
            docstring,
        }))
    }

    fn convert_async_method(
        &mut self,
        method: &ast::StmtAsyncFunctionDef,
    ) -> Result<Option<HirMethod>> {
        use smallvec::smallvec;

        let name = method.name.to_string();

        // DEPYLER-0967: Skip dunder methods except commonly-needed ones (async version)
        // Allow: context managers, async iterators, collections, comparisons, operators
        if name.starts_with("__")
            && name.ends_with("__")
            && !matches!(
                name.as_str(),
                "__init__"
                    | "__iter__"
                    | "__next__"
                    | "__enter__"
                    | "__exit__"
                    | "__aenter__"
                    | "__aexit__"
                    | "__anext__"
                    | "__aiter__"
                    | "__len__"
                    | "__str__"
                    | "__repr__"
                    | "__getitem__"
                    | "__setitem__"
                    | "__contains__"
                    | "__eq__"
                    | "__ne__"
                    | "__lt__"
                    | "__le__"
                    | "__gt__"
                    | "__ge__"
                    | "__hash__"
                    | "__add__"
                    | "__sub__"
                    | "__mul__"
                    | "__truediv__"
                    | "__neg__"
            )
        {
            return Ok(None);
        }

        // Extract docstring
        let docstring = self.extract_class_docstring(&method.body);

        // Check decorators
        let is_static = method
            .decorator_list
            .iter()
            .any(|d| matches!(d, ast::Expr::Name(n) if n.id.as_str() == "staticmethod"));
        let is_classmethod = method
            .decorator_list
            .iter()
            .any(|d| matches!(d, ast::Expr::Name(n) if n.id.as_str() == "classmethod"));
        let is_property = method
            .decorator_list
            .iter()
            .any(|d| matches!(d, ast::Expr::Name(n) if n.id.as_str() == "property"));

        // Convert parameters
        let mut params = smallvec![];
        let skip_first = if is_static {
            false
        } else if is_classmethod {
            // Skip 'cls' parameter for classmethods
            method
                .args
                .args
                .first()
                .map(|arg| arg.def.arg.as_str() == "cls")
                .unwrap_or(false)
        } else {
            // Skip 'self' parameter for instance methods
            method
                .args
                .args
                .first()
                .map(|arg| arg.def.arg.as_str() == "self")
                .unwrap_or(false)
        };

        let args_to_process = if skip_first {
            &method.args.args[1..]
        } else {
            &method.args.args[..]
        };

        for arg in args_to_process {
            let param_name = arg.def.arg.to_string();
            let param_type = if let Some(ann) = &arg.def.annotation {
                TypeExtractor::extract_type(ann)?
            } else {
                Type::Unknown
            };
            params.push(HirParam {
                name: param_name,
                ty: param_type,
                default: None, // Note: Method defaults extraction requires AST alignment with convert_parameters()
                is_vararg: false, // DEPYLER-0477: Regular parameter
            });
        }

        // DEPYLER-0841: Extract varargs parameter (*args) for async methods
        if let Some(vararg) = &method.args.vararg {
            let name = vararg.arg.to_string();
            // Start with List<String> as a reasonable default
            let ty = Type::List(Box::new(Type::String));
            params.push(HirParam {
                name,
                ty,
                default: None,
                is_vararg: true,
            });
        }

        // Convert return type
        let ret_type = if let Some(ret) = &method.returns {
            TypeExtractor::extract_type(ret)?
        } else if self.check_returns_self(&method.body) {
            // If method returns self without annotation, infer &Self
            Type::Custom("&Self".to_string())
        } else {
            Type::None
        };

        // Convert body (filter out docstring)
        let filtered_body = if docstring.is_some() && method.body.len() > 1 {
            // Skip first statement if it's the docstring
            method.body[1..].to_vec()
        } else if docstring.is_some() && method.body.len() == 1 {
            // Only docstring, no actual body
            vec![]
        } else {
            method.body.clone()
        };
        let body = convert_body(filtered_body)?;

        Ok(Some(HirMethod {
            name,
            params,
            ret_type,
            body,
            is_static,
            is_classmethod,
            is_property,
            is_async: true,
            docstring,
        }))
    }

    fn extract_class_docstring(&mut self, body: &[ast::Stmt]) -> Option<String> {
        if let Some(ast::Stmt::Expr(expr)) = body.first() {
            if let ast::Expr::Constant(c) = expr.value.as_ref() {
                if let ast::Constant::Str(s) = &c.value {
                    return Some(s.to_string());
                }
            }
        }
        None
    }

    fn extract_class_type_params(&mut self, class: &ast::StmtClassDef) -> Vec<String> {
        // DEPYLER-0759/0835: Extract type params from multiple sources:
        // 1. Explicit Generic[T, U] base class
        // 2. Type variables in parameterized bases like Iter[tuple[int, T]]
        // 3. Type variables used in field type annotations
        let mut type_params = Vec::new();

        // First, check for explicit Generic[T, U] declaration
        for base in &class.bases {
            if let ast::Expr::Subscript(subscript) = base {
                if let ast::Expr::Name(n) = subscript.value.as_ref() {
                    if n.id.as_str() == "Generic" {
                        // Explicit Generic[T, U] takes precedence - use these params
                        return self.extract_generic_params_recursive(&subscript.slice);
                    }
                }
            }
        }

        // DEPYLER-0835: Extract from all parameterized base classes recursively
        // Example: class EnumerateIter(Iter[tuple[int, T]]) -> extracts T
        for base in &class.bases {
            if let ast::Expr::Subscript(subscript) = base {
                let params = self.extract_generic_params_recursive(&subscript.slice);
                for p in params {
                    if self.is_type_variable(&p) && !type_params.contains(&p) {
                        type_params.push(p);
                    }
                }
            }
        }

        // DEPYLER-0835: Also extract from field type annotations
        // Example: source: Iter[T] with T not yet collected -> add T
        for stmt in &class.body {
            if let ast::Stmt::AnnAssign(ann_assign) = stmt {
                let field_type_vars =
                    self.extract_type_vars_from_annotation(&ann_assign.annotation);
                for tv in field_type_vars {
                    if !type_params.contains(&tv) {
                        type_params.push(tv);
                    }
                }
            }
        }

        type_params
    }

    /// DEPYLER-0835: Check if a name looks like a type variable
    /// Type variables are typically single uppercase letters (T, U, K, V)
    fn is_type_variable(&self, name: &str) -> bool {
        name.len() == 1 && name.chars().next().is_some_and(|c| c.is_uppercase())
    }

    /// DEPYLER-0841: Format the slice of a subscript expression
    /// Converts AST slice to string like "L, R" from Generic[L, R]
    fn format_subscript_slice(&self, slice: &ast::Expr) -> String {
        match slice {
            // Single type parameter: Generic[T]
            ast::Expr::Name(n) => n.id.to_string(),
            // Multiple type parameters: Generic[L, R] or tuple[int, str]
            ast::Expr::Tuple(tuple) => tuple
                .elts
                .iter()
                .map(|e| self.format_subscript_slice(e))
                .collect::<Vec<_>>()
                .join(", "),
            // Nested subscript: Dict[str, List[int]]
            ast::Expr::Subscript(sub) => {
                if let ast::Expr::Name(n) = sub.value.as_ref() {
                    format!("{}[{}]", n.id, self.format_subscript_slice(&sub.slice))
                } else {
                    "?".to_string()
                }
            }
            // Handle constants (None, strings, numbers)
            // Note: In rustpython_ast, these are typically wrapped in Constant
            _ => "?".to_string(),
        }
    }

    /// DEPYLER-0835: Recursively extract type variables from nested types
    /// Handles: T, tuple[int, T], list[T], dict[K, V], etc.
    fn extract_generic_params_recursive(&mut self, expr: &ast::Expr) -> Vec<String> {
        let mut params = Vec::new();
        self.collect_type_vars_from_expr(expr, &mut params);
        params
    }

    /// DEPYLER-0835: Collect type variables from any expression recursively
    fn collect_type_vars_from_expr(&self, expr: &ast::Expr, params: &mut Vec<String>) {
        match expr {
            ast::Expr::Name(n) => {
                let name = n.id.to_string();
                if self.is_type_variable(&name) && !params.contains(&name) {
                    params.push(name);
                }
            }
            ast::Expr::Tuple(tuple) => {
                for elt in &tuple.elts {
                    self.collect_type_vars_from_expr(elt, params);
                }
            }
            ast::Expr::Subscript(subscript) => {
                // Recurse into the slice (e.g., T in list[T])
                self.collect_type_vars_from_expr(&subscript.slice, params);
            }
            _ => {}
        }
    }

    /// DEPYLER-0835: Extract type variables from a field type annotation
    fn extract_type_vars_from_annotation(&self, annotation: &ast::Expr) -> Vec<String> {
        let mut params = Vec::new();
        self.collect_type_vars_from_expr(annotation, &mut params);
        params
    }

    fn convert_protocol_method(&self, func: &ast::StmtFunctionDef) -> Result<ProtocolMethod> {
        let name = func.name.to_string();
        let params = convert_parameters(&func.args)?;
        let ret_type = TypeExtractor::extract_return_type(&func.returns)?;

        // Check if method has @abstractmethod decorator
        let is_optional = !func.decorator_list.iter().any(|decorator| {
            matches!(decorator, ast::Expr::Name(n) if n.id.as_str() == "abstractmethod")
        });

        // Check if method has a default implementation (non-empty body beyond docstring)
        let has_default = self.method_has_default_implementation(&func.body);

        Ok(ProtocolMethod {
            name,
            params: params.into(),
            ret_type,
            is_optional,
            has_default,
        })
    }

    fn method_has_default_implementation(&self, body: &[ast::Stmt]) -> bool {
        // Filter out docstrings and ellipsis statements
        let meaningful_stmts: Vec<_> = body
            .iter()
            .filter(|stmt| {
                match stmt {
                    // Skip docstring
                    ast::Stmt::Expr(expr)
                        if matches!(expr.value.as_ref(),
                        ast::Expr::Constant(c) if matches!(c.value, ast::Constant::Str(_))) =>
                    {
                        false
                    }
                    // Skip ellipsis (...)
                    ast::Stmt::Expr(expr)
                        if matches!(expr.value.as_ref(),
                        ast::Expr::Constant(c) if matches!(c.value, ast::Constant::Ellipsis)) =>
                    {
                        false
                    }
                    _ => true,
                }
            })
            .collect();

        !meaningful_stmts.is_empty()
    }

    fn infer_fields_from_init(&self, init: &ast::StmtFunctionDef) -> Result<Vec<HirField>> {
        let mut fields = Vec::new();

        // Get parameter types from __init__ signature
        let mut param_types = std::collections::HashMap::new();
        for arg in &init.args.args {
            if arg.def.arg.as_str() != "self" {
                let param_name = arg.def.arg.to_string();
                let param_type = if let Some(annotation) = &arg.def.annotation {
                    TypeExtractor::extract_type(annotation)?
                } else {
                    Type::Unknown
                };
                param_types.insert(param_name, param_type);
            }
        }

        // DEPYLER-0637: Recursively collect all statements from body including nested blocks
        let all_stmts = Self::collect_all_statements_recursive(&init.body);

        // Look for self.field assignments in __init__ (including nested blocks)
        for stmt in all_stmts {
            // DEPYLER-0609: Handle both Assign and AnnAssign (annotated assignment)
            // Python: self._size: int = size  (AnnAssign)
            // Python: self._size = size       (Assign)
            match stmt {
                ast::Stmt::Assign(assign) => {
                    // Check if it's a self.field assignment
                    if assign.targets.len() == 1 {
                        if let ast::Expr::Attribute(attr) = &assign.targets[0] {
                            if let ast::Expr::Name(name) = attr.value.as_ref() {
                                if name.id.as_str() == "self" {
                                    let field_name = attr.attr.to_string();

                                    // Deduplicate: skip if field already exists
                                    if fields.iter().any(|f: &HirField| f.name == field_name) {
                                        continue;
                                    }

                                    // Try to infer type from the assigned value
                                    let field_type = if let ast::Expr::Name(value_name) =
                                        assign.value.as_ref()
                                    {
                                        // If assigning a parameter, use its type
                                        param_types
                                            .get(value_name.id.as_str())
                                            .cloned()
                                            .unwrap_or(Type::Unknown)
                                    } else {
                                        // Otherwise, try to infer from literal or default to Unknown
                                        self.infer_type_from_expr(assign.value.as_ref())
                                            .unwrap_or(Type::Unknown)
                                    };

                                    fields.push(HirField {
                                        name: field_name,
                                        field_type,
                                        default_value: None,
                                        is_class_var: false,
                                    });
                                }
                            }
                        }
                    }
                }
                ast::Stmt::AnnAssign(ann_assign) => {
                    // Handle annotated assignment: self._size: int = size
                    if let ast::Expr::Attribute(attr) = ann_assign.target.as_ref() {
                        if let ast::Expr::Name(name) = attr.value.as_ref() {
                            if name.id.as_str() == "self" {
                                let field_name = attr.attr.to_string();

                                // Deduplicate: skip if field already exists
                                if fields.iter().any(|f: &HirField| f.name == field_name) {
                                    continue;
                                }

                                // Use the annotation for the type
                                let field_type =
                                    TypeExtractor::extract_type(&ann_assign.annotation)
                                        .unwrap_or(Type::Unknown);

                                fields.push(HirField {
                                    name: field_name,
                                    field_type,
                                    default_value: None,
                                    is_class_var: false,
                                });
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(fields)
    }

    /// DEPYLER-0637: Recursively collect all statements from a body,
    /// including statements inside if/else/for/while/with/try blocks.
    /// This allows field inference to find self.X assignments in nested code.
    fn collect_all_statements_recursive(body: &[ast::Stmt]) -> Vec<&ast::Stmt> {
        let mut all_stmts = Vec::new();

        for stmt in body {
            // Add the statement itself
            all_stmts.push(stmt);

            // Recursively collect from nested blocks
            match stmt {
                ast::Stmt::If(if_stmt) => {
                    all_stmts.extend(Self::collect_all_statements_recursive(&if_stmt.body));
                    all_stmts.extend(Self::collect_all_statements_recursive(&if_stmt.orelse));
                }
                ast::Stmt::For(for_stmt) => {
                    all_stmts.extend(Self::collect_all_statements_recursive(&for_stmt.body));
                    all_stmts.extend(Self::collect_all_statements_recursive(&for_stmt.orelse));
                }
                ast::Stmt::While(while_stmt) => {
                    all_stmts.extend(Self::collect_all_statements_recursive(&while_stmt.body));
                    all_stmts.extend(Self::collect_all_statements_recursive(&while_stmt.orelse));
                }
                ast::Stmt::With(with_stmt) => {
                    all_stmts.extend(Self::collect_all_statements_recursive(&with_stmt.body));
                }
                ast::Stmt::Try(try_stmt) => {
                    all_stmts.extend(Self::collect_all_statements_recursive(&try_stmt.body));
                    // Note: handlers have a nested structure (ast::ExceptHandler enum variants)
                    // For simplicity, we skip handler bodies since field assignments in
                    // exception handlers are rare. We can extend this later if needed.
                    all_stmts.extend(Self::collect_all_statements_recursive(&try_stmt.orelse));
                    all_stmts.extend(Self::collect_all_statements_recursive(&try_stmt.finalbody));
                }
                _ => {}
            }
        }

        all_stmts
    }

    /// DEPYLER-0603: Infer fields from any method (not just __init__)
    /// This is simpler than infer_fields_from_init because we don't have
    /// parameter type information from the method signature.
    /// Fields inferred from non-__init__ methods get a synthetic default value
    /// so they don't become constructor parameters.
    fn infer_fields_from_method(&self, method: &ast::StmtFunctionDef) -> Result<Vec<HirField>> {
        let mut fields = Vec::new();

        // DEPYLER-0637: Recursively collect all statements including nested blocks
        let all_stmts = Self::collect_all_statements_recursive(&method.body);

        // Look for self.field assignments in method body (including nested blocks)
        for stmt in all_stmts {
            match stmt {
                ast::Stmt::Assign(assign) => {
                    // Check if it's a self.field assignment
                    if assign.targets.len() == 1 {
                        if let ast::Expr::Attribute(attr) = &assign.targets[0] {
                            if let ast::Expr::Name(name) = attr.value.as_ref() {
                                if name.id.as_str() == "self" {
                                    let field_name = attr.attr.to_string();

                                    // Infer type from the assigned value
                                    let field_type = self
                                        .infer_type_from_expr(assign.value.as_ref())
                                        .unwrap_or(Type::Unknown);

                                    // DEPYLER-0603: Create a default value based on the type
                                    // so this field doesn't become a constructor parameter
                                    let default_value =
                                        self.create_default_value_for_type(&field_type);

                                    // Deduplicate within this method
                                    if !fields.iter().any(|f: &HirField| f.name == field_name) {
                                        fields.push(HirField {
                                            name: field_name,
                                            field_type,
                                            default_value,
                                            is_class_var: false,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
                ast::Stmt::AnnAssign(ann_assign) => {
                    // Handle annotated assignment: self.field: int = value
                    if let ast::Expr::Attribute(attr) = ann_assign.target.as_ref() {
                        if let ast::Expr::Name(name) = attr.value.as_ref() {
                            if name.id.as_str() == "self" {
                                let field_name = attr.attr.to_string();

                                // Use the annotation for the type
                                let field_type =
                                    TypeExtractor::extract_type(&ann_assign.annotation)
                                        .unwrap_or(Type::Unknown);

                                // DEPYLER-0603: Create a default value based on the type
                                let default_value = self.create_default_value_for_type(&field_type);

                                // Deduplicate within this method
                                if !fields.iter().any(|f: &HirField| f.name == field_name) {
                                    fields.push(HirField {
                                        name: field_name,
                                        field_type,
                                        default_value,
                                        is_class_var: false,
                                    });
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(fields)
    }

    /// DEPYLER-0603: Create a default value for a type.
    /// Used for fields inferred from non-__init__ methods.
    fn create_default_value_for_type(&self, ty: &Type) -> Option<HirExpr> {
        match ty {
            Type::Int => Some(HirExpr::Literal(crate::hir::Literal::Int(0))),
            Type::Float => Some(HirExpr::Literal(crate::hir::Literal::Float(0.0))),
            Type::Bool => Some(HirExpr::Literal(crate::hir::Literal::Bool(false))),
            Type::String => Some(HirExpr::Literal(crate::hir::Literal::String(String::new()))),
            // For unknown types, use Int default (0) as fallback
            Type::Unknown => Some(HirExpr::Literal(crate::hir::Literal::Int(0))),
            _ => Some(HirExpr::Literal(crate::hir::Literal::Int(0))),
        }
    }

    fn infer_type_from_expr(&self, expr: &ast::Expr) -> Option<Type> {
        match expr {
            ast::Expr::Constant(c) => match &c.value {
                ast::Constant::Int(_) => Some(Type::Int),
                ast::Constant::Float(_) => Some(Type::Float),
                ast::Constant::Str(_) => Some(Type::String),
                ast::Constant::Bool(_) => Some(Type::Bool),
                ast::Constant::None => Some(Type::None),
                _ => None,
            },
            ast::Expr::List(_) => Some(Type::List(Box::new(Type::Unknown))),
            ast::Expr::Dict(_) => {
                Some(Type::Dict(Box::new(Type::Unknown), Box::new(Type::Unknown)))
            }
            ast::Expr::Set(_) => Some(Type::Set(Box::new(Type::Unknown))),
            _ => None,
        }
    }

    fn check_returns_self(&self, body: &[ast::Stmt]) -> bool {
        for stmt in body {
            if let ast::Stmt::Return(ret_stmt) = stmt {
                if let Some(value) = &ret_stmt.value {
                    if let ast::Expr::Name(n) = value.as_ref() {
                        if n.id.as_str() == "self" {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}

// Keep the old function for backwards compatibility
/// Convenience function to convert Python AST to HIR with default settings
///
/// This is a shorthand for `AstBridge::new().python_to_hir(module)`.
/// Use this when you don't need custom configuration or source code tracking.
///
/// # Arguments
///
/// * `module` - The Python AST module to convert
///
/// # Examples
///
/// ```rust
/// use depyler_core::ast_bridge::python_to_hir;
/// use rustpython_parser::{parse, Mode};
///
/// let python_code = "def simple(): return 42";
/// let ast = parse(python_code, Mode::Module, "<test>").unwrap();
/// let (hir, _type_env) = python_to_hir(ast).unwrap();
///
/// assert_eq!(hir.functions.len(), 1);
/// assert_eq!(hir.functions[0].name, "simple");
/// ```
pub fn python_to_hir(
    module: ast::Mod,
) -> Result<(
    HirModule,
    crate::type_system::type_environment::TypeEnvironment,
)> {
    AstBridge::new().python_to_hir(module)
}

/// DEPYLER-0359: Propagate can_fail property through function call chains
///
/// This function performs a fixed-point iteration to propagate the `can_fail` property
/// from callees to callers. If function A calls function B, and B can fail, then A
/// can also fail (unless it catches the error).
///
/// This is essential for correct Result type propagation in recursive functions.
///
/// Complexity: O(n * m) where n = number of functions, m = max call depth
fn propagate_can_fail_through_calls(functions: &mut [HirFunction]) {
    // Build a map of function names to can_fail status for quick lookup
    let mut can_fail_map: std::collections::HashMap<String, bool> = functions
        .iter()
        .map(|f| (f.name.clone(), f.properties.can_fail))
        .collect();

    // Fixed-point iteration: keep propagating until no changes occur
    let mut changed = true;
    let mut iterations = 0;
    const MAX_ITERATIONS: usize = 100; // Prevent infinite loops

    while changed && iterations < MAX_ITERATIONS {
        changed = false;
        iterations += 1;

        for func in functions.iter_mut() {
            // Skip if already marked as can_fail
            if func.properties.can_fail {
                continue;
            }

            // Check if this function calls any function that can fail
            if calls_failing_function(&func.body, &can_fail_map) {
                func.properties.can_fail = true;
                can_fail_map.insert(func.name.clone(), true);
                changed = true;
            }
        }
    }
}

/// Check if a statement sequence contains calls to functions that can fail
fn calls_failing_function(
    stmts: &[HirStmt],
    can_fail_map: &std::collections::HashMap<String, bool>,
) -> bool {
    for stmt in stmts {
        if stmt_calls_failing_function(stmt, can_fail_map) {
            return true;
        }
    }
    false
}

/// Check if a statement calls a function that can fail
fn stmt_calls_failing_function(
    stmt: &HirStmt,
    can_fail_map: &std::collections::HashMap<String, bool>,
) -> bool {
    match stmt {
        HirStmt::Return(Some(expr)) | HirStmt::Expr(expr) | HirStmt::Assign { value: expr, .. } => {
            expr_calls_failing_function(expr, can_fail_map)
        }
        HirStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            expr_calls_failing_function(condition, can_fail_map)
                || calls_failing_function(then_body, can_fail_map)
                || else_body
                    .as_ref()
                    .map(|body| calls_failing_function(body, can_fail_map))
                    .unwrap_or(false)
        }
        HirStmt::While { condition, body } => {
            expr_calls_failing_function(condition, can_fail_map)
                || calls_failing_function(body, can_fail_map)
        }
        HirStmt::For { iter, body, .. } => {
            expr_calls_failing_function(iter, can_fail_map)
                || calls_failing_function(body, can_fail_map)
        }
        HirStmt::Try {
            body,
            handlers,
            finalbody,
            ..
        } => {
            calls_failing_function(body, can_fail_map)
                || handlers
                    .iter()
                    .any(|h| calls_failing_function(&h.body, can_fail_map))
                || finalbody
                    .as_ref()
                    .map(|fb| calls_failing_function(fb, can_fail_map))
                    .unwrap_or(false)
        }
        _ => false,
    }
}

/// Check if an expression contains calls to functions that can fail
fn expr_calls_failing_function(
    expr: &HirExpr,
    can_fail_map: &std::collections::HashMap<String, bool>,
) -> bool {
    match expr {
        HirExpr::Call { func, args, .. } => {
            // Check if the called function is known to fail
            if can_fail_map.get(func).copied().unwrap_or(false) {
                return true;
            }
            // Also check arguments recursively
            args.iter()
                .any(|arg| expr_calls_failing_function(arg, can_fail_map))
        }
        HirExpr::Binary { left, right, .. } => {
            expr_calls_failing_function(left, can_fail_map)
                || expr_calls_failing_function(right, can_fail_map)
        }
        HirExpr::Unary { operand, .. } => expr_calls_failing_function(operand, can_fail_map),
        HirExpr::List(elements) | HirExpr::Tuple(elements) | HirExpr::Set(elements) => elements
            .iter()
            .any(|e| expr_calls_failing_function(e, can_fail_map)),
        HirExpr::MethodCall { object, args, .. } => {
            expr_calls_failing_function(object, can_fail_map)
                || args
                    .iter()
                    .any(|arg| expr_calls_failing_function(arg, can_fail_map))
        }
        HirExpr::Index { base, index } => {
            expr_calls_failing_function(base, can_fail_map)
                || expr_calls_failing_function(index, can_fail_map)
        }
        HirExpr::Slice { base, .. } => expr_calls_failing_function(base, can_fail_map),
        _ => false,
    }
}

fn convert_parameters(args: &ast::Arguments) -> Result<Vec<HirParam>> {
    use crate::ast_bridge::converters::ExprConverter;
    let mut params = Vec::new();

    // Calculate number of args without defaults
    let num_args = args.args.len();
    let defaults_vec: Vec<_> = args.defaults().collect();
    let num_defaults = defaults_vec.len();
    let first_default_idx = num_args.saturating_sub(num_defaults);

    for (i, arg) in args.args.iter().enumerate() {
        let name = arg.def.arg.to_string();

        // DEPYLER-0457: Extract base type from annotation
        let base_ty = if let Some(annotation) = &arg.def.annotation {
            TypeExtractor::extract_type(annotation)?
        } else {
            // No annotation - will be refined based on default value below
            Type::Unknown
        };

        // Check if this parameter has a default value
        let default = if i >= first_default_idx {
            let default_idx = i - first_default_idx;
            if let Some(default_expr) = defaults_vec.get(default_idx) {
                Some(ExprConverter::convert((*default_expr).clone())?)
            } else {
                None
            }
        } else {
            None
        };

        // DEPYLER-0457: Infer types for unannotated parameters
        // DEPYLER-0744: Use Option<Unknown> for unannotated params with default=None
        // This allows the return type inference to unify properly when the param is returned
        let ty = if let Some(HirExpr::Literal(Literal::None)) = &default {
            // Parameter has default=None, wrap type in Optional
            match base_ty {
                Type::Unknown => {
                    // DEPYLER-0834: Apply name-based heuristics for Optional params too
                    // When param has default=None but no type annotation, infer type from name
                    // This prevents Option<serde_json::Value> which causes E0308/E0599 errors
                    let param_lower = name.to_lowercase();

                    let inferred_type = if param_lower.contains("file")
                        || param_lower.contains("path")
                        || param_lower.contains("output")
                        || param_lower.contains("input")
                        || param_lower.contains("dir")
                        || param_lower.contains("folder")
                    {
                        // File/path parameters → Option<String>
                        Type::String
                    } else if param_lower.contains("name")
                        || param_lower.contains("title")
                        || param_lower.contains("text")
                        || param_lower.contains("message")
                        || param_lower.contains("content")
                        || param_lower.contains("label")
                        || param_lower.contains("description")
                        || param_lower.contains("prefix")
                        || param_lower.contains("suffix")
                        || param_lower.contains("format")
                        || param_lower == "value"
                        || param_lower.contains("column")
                        || param_lower.contains("key") && !param_lower.contains("keys")
                    {
                        // String-like parameters → Option<String>
                        Type::String
                    } else if param_lower.contains("count")
                        || param_lower.contains("num")
                        || param_lower.contains("index")
                        || param_lower.contains("size")
                        || param_lower.contains("limit")
                        || param_lower.contains("max")
                        || param_lower.contains("min")
                        || param_lower.contains("port")
                        || param_lower.contains("timeout")
                        || param_lower.contains("depth")
                        || param_lower == "n"
                        || param_lower == "i"
                        || param_lower == "j"
                    {
                        // Numeric parameters → Option<i64>
                        Type::Int
                    } else if param_lower.contains("flag")
                        || param_lower.contains("enabled")
                        || param_lower.contains("verbose")
                        || param_lower.contains("debug")
                        || param_lower.contains("quiet")
                        || param_lower.contains("force")
                        || param_lower.starts_with("is_")
                        || param_lower.starts_with("has_")
                        || param_lower.starts_with("use_")
                        || param_lower.starts_with("allow_")
                    {
                        // Boolean parameters → Option<bool>
                        Type::Bool
                    } else {
                        // DEPYLER-0744: Fallback to Unknown for return type unification
                        Type::Unknown
                    };

                    Type::Optional(Box::new(inferred_type))
                }
                Type::Optional(_) => {
                    // Already Optional<T>, don't double-wrap
                    base_ty
                }
                _ => {
                    // Wrap annotated type in Optional
                    Type::Optional(Box::new(base_ty))
                }
            }
        } else if default.is_none() && matches!(base_ty, Type::Unknown) {
            // DEPYLER-0457: Heuristic inference for unannotated required parameters
            // Only infer String for parameters with names suggesting string types
            let param_lower = name.to_lowercase();
            let is_likely_string = param_lower.contains("file")
                || param_lower.contains("path")
                || param_lower.contains("name")
                || param_lower.contains("column")
                || param_lower == "value" // Common in filter functions
                || param_lower.contains("key"); // But not "keys" plural

            if is_likely_string && !param_lower.contains("config") && !param_lower.contains("data")
            {
                // Conservative inference: only for clearly string-like parameters
                Type::String
            } else {
                // Keep as Unknown (→ serde_json::Value) for safety
                base_ty
            }
        } else {
            base_ty
        };

        params.push(HirParam {
            name,
            ty,
            default,
            is_vararg: false,
        });
    }

    // DEPYLER-0477: Extract varargs parameter (*args)
    if let Some(vararg) = &args.vararg {
        let name = vararg.arg.to_string();

        // Start with List<String> as a reasonable default
        // DEPYLER-0477 Phase 2.2: Infer element type from usage (tracked)
        let ty = Type::List(Box::new(Type::String));

        params.push(HirParam {
            name,
            ty,
            default: None, // Varargs never have defaults
            is_vararg: true,
        });
    }

    // DEPYLER-0477 Phase 2.2: Extract kwargs (**kwargs) (tracked)
    // if let Some(kwarg) = &args.kwarg {
    //     // Will transpile to HashMap<String, serde_json::Value>
    // }

    Ok(params)
}

pub(crate) fn convert_body(body: Vec<ast::Stmt>) -> Result<Vec<HirStmt>> {
    body.into_iter().map(convert_stmt).collect()
}

fn convert_stmt(stmt: ast::Stmt) -> Result<HirStmt> {
    StmtConverter::convert(stmt)
}

pub(crate) fn extract_assign_target(expr: &ast::Expr) -> Result<AssignTarget> {
    use crate::ast_bridge::converters::ExprConverter;
    match expr {
        ast::Expr::Name(n) => Ok(AssignTarget::Symbol(n.id.to_string())),
        ast::Expr::Subscript(s) => {
            let base = Box::new(ExprConverter::convert(s.value.as_ref().clone())?);
            let index = Box::new(ExprConverter::convert(s.slice.as_ref().clone())?);
            Ok(AssignTarget::Index { base, index })
        }
        ast::Expr::Attribute(a) => {
            let value = Box::new(ExprConverter::convert(a.value.as_ref().clone())?);
            Ok(AssignTarget::Attribute {
                value,
                attr: a.attr.to_string(),
            })
        }
        ast::Expr::Tuple(t) => {
            let targets = t
                .elts
                .iter()
                .map(extract_assign_target)
                .collect::<Result<Vec<_>>>()?;
            Ok(AssignTarget::Tuple(targets))
        }
        _ => bail!("Unsupported assignment target"),
    }
}

pub(crate) fn convert_expr(expr: ast::Expr) -> Result<HirExpr> {
    ExprConverter::convert(expr)
}

pub(crate) fn convert_binop(op: &ast::Operator) -> Result<BinOp> {
    Ok(match op {
        ast::Operator::Add => BinOp::Add,
        ast::Operator::Sub => BinOp::Sub,
        ast::Operator::Mult => BinOp::Mul,
        ast::Operator::Div => BinOp::Div,
        ast::Operator::FloorDiv => BinOp::FloorDiv,
        ast::Operator::Mod => BinOp::Mod,
        ast::Operator::Pow => BinOp::Pow,
        ast::Operator::BitAnd => BinOp::BitAnd,
        ast::Operator::BitOr => BinOp::BitOr,
        ast::Operator::BitXor => BinOp::BitXor,
        ast::Operator::LShift => BinOp::LShift,
        ast::Operator::RShift => BinOp::RShift,
        _ => bail!("Unsupported binary operator"),
    })
}

pub(crate) fn convert_aug_op(op: &ast::Operator) -> Result<BinOp> {
    // Augmented assignment operators map to the same binary operators
    convert_binop(op)
}

pub(crate) fn convert_unaryop(op: &ast::UnaryOp) -> Result<UnaryOp> {
    Ok(match op {
        ast::UnaryOp::Not => UnaryOp::Not,
        ast::UnaryOp::UAdd => UnaryOp::Pos,
        ast::UnaryOp::USub => UnaryOp::Neg,
        ast::UnaryOp::Invert => UnaryOp::BitNot,
    })
}

pub(crate) fn convert_cmpop(op: &ast::CmpOp) -> Result<BinOp> {
    Ok(match op {
        ast::CmpOp::Eq => BinOp::Eq,
        ast::CmpOp::NotEq => BinOp::NotEq,
        ast::CmpOp::Lt => BinOp::Lt,
        ast::CmpOp::LtE => BinOp::LtEq,
        ast::CmpOp::Gt => BinOp::Gt,
        ast::CmpOp::GtE => BinOp::GtEq,
        ast::CmpOp::In => BinOp::In,
        ast::CmpOp::NotIn => BinOp::NotIn,
        // DEPYLER-0188: Python 'is' checks identity (same object), but in transpiled code
        // we use value equality since Rust doesn't have Python's object identity concept.
        // This is correct for: x is None, x is True/False, small integers, interned strings
        ast::CmpOp::Is => BinOp::Eq,
        ast::CmpOp::IsNot => BinOp::NotEq,
    })
}

/// DEPYLER-CONVERGE-MULTI: Detect `if TYPE_CHECKING:` guard blocks.
/// These are Python-only constructs used for import-time type hints
/// that must be stripped during transpilation.
fn is_type_checking_guard(if_stmt: &ast::StmtIf) -> bool {
    match if_stmt.test.as_ref() {
        ast::Expr::Name(name) => name.id.as_str() == "TYPE_CHECKING",
        _ => false,
    }
}

/// DEPYLER-1136: Capture module-level alias for `import X as Y` patterns
fn convert_import(import: ast::StmtImport) -> Result<Vec<Import>> {
    import
        .names
        .into_iter()
        .map(|alias| {
            let module = alias.name.to_string();
            // DEPYLER-1136: Capture the "as Y" alias for module-level imports
            let module_alias = alias.asname.map(|a| a.to_string());
            // For "import module" or "import module as alias", we import the whole module
            let items = vec![];
            Ok(Import {
                module,
                alias: module_alias,
                items,
            })
        })
        .collect()
}

fn convert_import_from(import: ast::StmtImportFrom) -> Result<Vec<Import>> {
    let module = import.module.map(|m| m.to_string()).unwrap_or_default();

    let items = import
        .names
        .into_iter()
        .map(|alias| {
            let name = alias.name.to_string();
            if let Some(asname) = alias.asname {
                ImportItem::Aliased {
                    name,
                    alias: asname.to_string(),
                }
            } else {
                ImportItem::Named(name)
            }
        })
        .collect();

    // DEPYLER-1136: `from X import Y` has no module-level alias
    Ok(vec![Import {
        module,
        alias: None,
        items,
    }])
}

fn extract_docstring_and_body(body: Vec<ast::Stmt>) -> Result<(Option<String>, Vec<HirStmt>)> {
    if body.is_empty() {
        return Ok((None, vec![]));
    }

    // Check if the first statement is a string literal (docstring)
    let docstring = if let ast::Stmt::Expr(expr) = &body[0] {
        if let ast::Expr::Constant(constant) = expr.value.as_ref() {
            if let ast::Constant::Str(s) = &constant.value {
                Some(s.clone())
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    // Convert the body, skipping the docstring if it exists
    let start_index = if docstring.is_some() { 1 } else { 0 };
    let filtered_body: Vec<HirStmt> = body
        .into_iter()
        .skip(start_index)
        .filter_map(|stmt| convert_stmt(stmt).ok())
        .collect();

    Ok((docstring, filtered_body))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustpython_ast::Suite;
    use rustpython_parser::Parse;

    fn parse_python_to_hir(source: &str) -> HirModule {
        let statements = Suite::parse(source, "<test>").unwrap();
        let ast = rustpython_ast::Mod::Module(rustpython_ast::ModModule {
            body: statements,
            type_ignores: vec![],
            range: Default::default(),
        });
        let (hir, _type_env) = AstBridge::new()
            .with_source(source.to_string())
            .python_to_hir(ast)
            .unwrap();
        hir
    }

    #[test]
    fn test_simple_function_conversion() {
        let source = "def add(a: int, b: int) -> int:\n    return a + b";
        let hir = parse_python_to_hir(source);

        assert_eq!(hir.functions.len(), 1);
        let func = &hir.functions[0];
        assert_eq!(func.name, "add");
        assert_eq!(func.params.len(), 2);
        assert_eq!(func.params[0].name, "a");
        assert_eq!(func.params[0].ty, Type::Int);
        assert_eq!(func.ret_type, Type::Int);
    }

    #[test]
    fn test_type_annotation_conversion() {
        let source = "def process(items: List[str]) -> Optional[int]:\n    return None";
        let hir = parse_python_to_hir(source);

        let func = &hir.functions[0];
        assert_eq!(func.params[0].ty, Type::List(Box::new(Type::String)));
        assert_eq!(func.ret_type, Type::Optional(Box::new(Type::Int)));
    }

    #[test]
    fn test_import_conversion() {
        let source = "from typing import List, Dict\nimport sys";
        let hir = parse_python_to_hir(source);

        assert_eq!(hir.imports.len(), 2);
        assert_eq!(hir.imports[0].module, "typing");
        assert_eq!(hir.imports[1].module, "sys");
    }

    #[test]
    fn test_control_flow_conversion() {
        let source = r#"
def check(x: int) -> str:
    if x > 0:
        return "positive"
    else:
        return "negative"
"#;
        let hir = parse_python_to_hir(source);

        let func = &hir.functions[0];
        assert_eq!(func.body.len(), 1);
        if let HirStmt::If {
            condition,
            then_body,
            else_body,
        } = &func.body[0]
        {
            assert!(matches!(condition, HirExpr::Binary { op: BinOp::Gt, .. }));
            assert_eq!(then_body.len(), 1);
            assert!(else_body.is_some());
        } else {
            panic!("Expected if statement");
        }
    }

    #[test]
    fn test_binary_operations() {
        let source = "def calc() -> int:\n    return 1 + 2 * 3";
        let hir = parse_python_to_hir(source);

        let func = &hir.functions[0];
        if let HirStmt::Return(Some(HirExpr::Binary { op, .. })) = &func.body[0] {
            assert_eq!(*op, BinOp::Add);
        } else {
            panic!("Expected binary operation in return");
        }
    }

    #[test]
    fn test_function_properties_analysis() {
        let source = r#"
def pure_func(x: int) -> int:
    return x + 1

def impure_func(x: int):
    print(x)
"#;
        let hir = parse_python_to_hir(source);

        assert_eq!(hir.functions.len(), 2);
        assert!(hir.functions[0].properties.is_pure);
        assert!(!hir.functions[1].properties.is_pure);
    }

    #[test]
    fn test_for_loop_conversion() {
        let source = r#"
def iterate(items: list) -> int:
    total = 0
    for item in items:
        total = total + item
    return total
"#;
        let hir = parse_python_to_hir(source);

        let func = &hir.functions[0];
        assert_eq!(func.body.len(), 3); // assign, for, return
        if let HirStmt::For { target, iter, body } = &func.body[1] {
            assert!(matches!(target, AssignTarget::Symbol(ref s) if s == "item"));
            assert!(matches!(iter, HirExpr::Var(_)));
            assert_eq!(body.len(), 1);
        } else {
            panic!("Expected for loop");
        }
    }

    #[test]
    fn test_expression_types() {
        let source = r#"
def expressions():
    x = [1, 2, 3]
    z = (1, 2, 3)
"#;
        let hir = parse_python_to_hir(source);

        let func = &hir.functions[0];
        assert_eq!(func.body.len(), 2);

        // Check list assignment
        if let HirStmt::Assign {
            value: HirExpr::List(_),
            ..
        } = &func.body[0]
        {
            // OK
        } else {
            panic!("Expected list assignment");
        }

        // Check tuple assignment
        if let HirStmt::Assign {
            value: HirExpr::Tuple(_),
            ..
        } = &func.body[1]
        {
            // OK
        } else {
            panic!("Expected tuple assignment");
        }
    }

    #[test]
    fn test_comparison_operators() {
        let source = r#"
def compare(a: int, b: int) -> bool:
    return a > b
"#;
        let hir = parse_python_to_hir(source);

        let func = &hir.functions[0];
        if let HirStmt::Return(Some(HirExpr::Binary { op: BinOp::Gt, .. })) = &func.body[0] {
            // OK - simple comparison works
        } else {
            panic!("Expected > comparison");
        }
    }

    #[test]
    fn test_unary_operations() {
        let source = r#"
def unary_ops(x: int) -> int:
    return -x + +x
"#;
        let hir = parse_python_to_hir(source);

        let func = &hir.functions[0];
        if let HirStmt::Return(Some(HirExpr::Binary {
            op: BinOp::Add,
            left,
            right,
        })) = &func.body[0]
        {
            assert!(matches!(
                left.as_ref(),
                HirExpr::Unary {
                    op: UnaryOp::Neg,
                    ..
                }
            ));
            assert!(matches!(
                right.as_ref(),
                HirExpr::Unary {
                    op: UnaryOp::Pos,
                    ..
                }
            ));
        } else {
            panic!("Expected unary operations");
        }
    }

    #[test]
    fn test_function_calls() {
        let source = r#"
def call_functions() -> int:
    return len([1, 2, 3])
"#;
        let hir = parse_python_to_hir(source);

        let func = &hir.functions[0];
        if let HirStmt::Return(Some(HirExpr::Call {
            func: fname, args, ..
        })) = &func.body[0]
        {
            assert_eq!(fname, "len");
            assert_eq!(args.len(), 1);
            assert!(matches!(args[0], HirExpr::List(_)));
        } else {
            panic!("Expected function call");
        }
    }

    #[test]
    fn test_annotation_extraction() {
        let source = r#"
# @depyler: type_strategy = "aggressive"
# @depyler: optimization_level = "aggressive"
# @depyler: thread_safety = "required"
def process_data(items: List[int]) -> int:
    total = 0
    for x in items:
        total = total + x * 2
    return total
"#;
        let hir = parse_python_to_hir(source);

        let func = &hir.functions[0];
        assert_eq!(
            func.annotations.type_strategy,
            depyler_annotations::TypeStrategy::Aggressive
        );
        assert_eq!(
            func.annotations.optimization_level,
            depyler_annotations::OptimizationLevel::Aggressive
        );
        assert_eq!(
            func.annotations.thread_safety,
            depyler_annotations::ThreadSafety::Required
        );
    }

    #[test]
    fn test_annotation_with_performance_hints() {
        let source = r#"
# @depyler: performance_critical = "true"
# @depyler: vectorize = "true"
# @depyler: bounds_checking = "disabled"
def compute(data: List[float]) -> float:
    total = 0.0
    for x in data:
        total += x
    return total
"#;
        let hir = parse_python_to_hir(source);

        let func = &hir.functions[0];
        assert!(func
            .annotations
            .performance_hints
            .contains(&depyler_annotations::PerformanceHint::PerformanceCritical));
        assert!(func
            .annotations
            .performance_hints
            .contains(&depyler_annotations::PerformanceHint::Vectorize));
        assert_eq!(
            func.annotations.bounds_checking,
            depyler_annotations::BoundsChecking::Disabled
        );
    }

    #[test]
    fn test_docstring_extraction() {
        let source = r#"
def example_function(x: int) -> int:
    """This is a docstring that should become a comment"""
    return x * 2

def function_without_docstring(y: int) -> int:
    print("Not a docstring") 
    return y + 1
"#;
        let hir = parse_python_to_hir(source);

        assert_eq!(hir.functions.len(), 2);

        // First function should have a docstring
        let func_with_docstring = &hir.functions[0];
        assert_eq!(func_with_docstring.name, "example_function");
        assert_eq!(
            func_with_docstring.docstring,
            Some("This is a docstring that should become a comment".to_string())
        );
        assert_eq!(func_with_docstring.body.len(), 1); // Only the return statement

        // Second function should not have a docstring
        let func_without_docstring = &hir.functions[1];
        assert_eq!(func_without_docstring.name, "function_without_docstring");
        assert_eq!(func_without_docstring.docstring, None);
        assert_eq!(func_without_docstring.body.len(), 2); // print statement + return
    }

    // ============================================================
    // DEPYLER-COVERAGE-95: Comprehensive tests for operator conversions
    // ============================================================

    #[test]
    fn test_convert_binop_add() {
        let result = convert_binop(&ast::Operator::Add).unwrap();
        assert_eq!(result, BinOp::Add);
    }

    #[test]
    fn test_convert_binop_sub() {
        let result = convert_binop(&ast::Operator::Sub).unwrap();
        assert_eq!(result, BinOp::Sub);
    }

    #[test]
    fn test_convert_binop_mult() {
        let result = convert_binop(&ast::Operator::Mult).unwrap();
        assert_eq!(result, BinOp::Mul);
    }

    #[test]
    fn test_convert_binop_div() {
        let result = convert_binop(&ast::Operator::Div).unwrap();
        assert_eq!(result, BinOp::Div);
    }

    #[test]
    fn test_convert_binop_floor_div() {
        let result = convert_binop(&ast::Operator::FloorDiv).unwrap();
        assert_eq!(result, BinOp::FloorDiv);
    }

    #[test]
    fn test_convert_binop_mod() {
        let result = convert_binop(&ast::Operator::Mod).unwrap();
        assert_eq!(result, BinOp::Mod);
    }

    #[test]
    fn test_convert_binop_pow() {
        let result = convert_binop(&ast::Operator::Pow).unwrap();
        assert_eq!(result, BinOp::Pow);
    }

    #[test]
    fn test_convert_binop_bitand() {
        let result = convert_binop(&ast::Operator::BitAnd).unwrap();
        assert_eq!(result, BinOp::BitAnd);
    }

    #[test]
    fn test_convert_binop_bitor() {
        let result = convert_binop(&ast::Operator::BitOr).unwrap();
        assert_eq!(result, BinOp::BitOr);
    }

    #[test]
    fn test_convert_binop_bitxor() {
        let result = convert_binop(&ast::Operator::BitXor).unwrap();
        assert_eq!(result, BinOp::BitXor);
    }

    #[test]
    fn test_convert_binop_lshift() {
        let result = convert_binop(&ast::Operator::LShift).unwrap();
        assert_eq!(result, BinOp::LShift);
    }

    #[test]
    fn test_convert_binop_rshift() {
        let result = convert_binop(&ast::Operator::RShift).unwrap();
        assert_eq!(result, BinOp::RShift);
    }

    #[test]
    fn test_convert_binop_matmul_unsupported() {
        let result = convert_binop(&ast::Operator::MatMult);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_aug_op_delegates_to_binop() {
        // Augmented operators use same conversion as binop
        let result = convert_aug_op(&ast::Operator::Add).unwrap();
        assert_eq!(result, BinOp::Add);
    }

    #[test]
    fn test_convert_unaryop_not() {
        let result = convert_unaryop(&ast::UnaryOp::Not).unwrap();
        assert_eq!(result, UnaryOp::Not);
    }

    #[test]
    fn test_convert_unaryop_pos() {
        let result = convert_unaryop(&ast::UnaryOp::UAdd).unwrap();
        assert_eq!(result, UnaryOp::Pos);
    }

    #[test]
    fn test_convert_unaryop_neg() {
        let result = convert_unaryop(&ast::UnaryOp::USub).unwrap();
        assert_eq!(result, UnaryOp::Neg);
    }

    #[test]
    fn test_convert_unaryop_bitnot() {
        let result = convert_unaryop(&ast::UnaryOp::Invert).unwrap();
        assert_eq!(result, UnaryOp::BitNot);
    }

    #[test]
    fn test_convert_cmpop_eq() {
        let result = convert_cmpop(&ast::CmpOp::Eq).unwrap();
        assert_eq!(result, BinOp::Eq);
    }

    #[test]
    fn test_convert_cmpop_noteq() {
        let result = convert_cmpop(&ast::CmpOp::NotEq).unwrap();
        assert_eq!(result, BinOp::NotEq);
    }

    #[test]
    fn test_convert_cmpop_lt() {
        let result = convert_cmpop(&ast::CmpOp::Lt).unwrap();
        assert_eq!(result, BinOp::Lt);
    }

    #[test]
    fn test_convert_cmpop_lte() {
        let result = convert_cmpop(&ast::CmpOp::LtE).unwrap();
        assert_eq!(result, BinOp::LtEq);
    }

    #[test]
    fn test_convert_cmpop_gt() {
        let result = convert_cmpop(&ast::CmpOp::Gt).unwrap();
        assert_eq!(result, BinOp::Gt);
    }

    #[test]
    fn test_convert_cmpop_gte() {
        let result = convert_cmpop(&ast::CmpOp::GtE).unwrap();
        assert_eq!(result, BinOp::GtEq);
    }

    #[test]
    fn test_convert_cmpop_in() {
        let result = convert_cmpop(&ast::CmpOp::In).unwrap();
        assert_eq!(result, BinOp::In);
    }

    #[test]
    fn test_convert_cmpop_notin() {
        let result = convert_cmpop(&ast::CmpOp::NotIn).unwrap();
        assert_eq!(result, BinOp::NotIn);
    }

    #[test]
    fn test_convert_cmpop_is() {
        // DEPYLER-0188: 'is' maps to Eq for transpilation
        let result = convert_cmpop(&ast::CmpOp::Is).unwrap();
        assert_eq!(result, BinOp::Eq);
    }

    #[test]
    fn test_convert_cmpop_isnot() {
        // DEPYLER-0188: 'is not' maps to NotEq for transpilation
        let result = convert_cmpop(&ast::CmpOp::IsNot).unwrap();
        assert_eq!(result, BinOp::NotEq);
    }

    #[test]
    fn test_ast_bridge_default() {
        let bridge = AstBridge::default();
        assert!(bridge.source_code.is_none());
    }

    #[test]
    fn test_ast_bridge_with_source() {
        let source = "def foo(): pass";
        let bridge = AstBridge::new().with_source(source.to_string());
        assert_eq!(bridge.source_code, Some(source.to_string()));
    }

    #[test]
    fn test_class_with_methods() {
        let source = r#"
class Calculator:
    def add(self, a: int, b: int) -> int:
        return a + b

    def multiply(self, a: int, b: int) -> int:
        return a * b
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.classes.len(), 1);
        assert_eq!(hir.classes[0].name, "Calculator");
        assert_eq!(hir.classes[0].methods.len(), 2);
    }

    #[test]
    fn test_type_alias_simple() {
        let source = "UserId = int";
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.type_aliases.len(), 1);
        assert_eq!(hir.type_aliases[0].name, "UserId");
        assert_eq!(hir.type_aliases[0].target_type, Type::Int);
        assert!(!hir.type_aliases[0].is_newtype);
    }

    #[test]
    fn test_type_alias_generic() {
        let source = "StringList = List[str]";
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.type_aliases.len(), 1);
        assert_eq!(hir.type_aliases[0].name, "StringList");
        assert_eq!(
            hir.type_aliases[0].target_type,
            Type::List(Box::new(Type::String))
        );
    }

    #[test]
    fn test_module_constant() {
        let source = "MAX_SIZE = 100";
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.constants.len(), 1);
        assert_eq!(hir.constants[0].name, "MAX_SIZE");
    }

    #[test]
    fn test_annotated_constant() {
        let source = "MAX_SIZE: int = 100";
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.constants.len(), 1);
        assert_eq!(hir.constants[0].name, "MAX_SIZE");
        assert_eq!(hir.constants[0].type_annotation, Some(Type::Int));
    }

    #[test]
    fn test_async_function() {
        let source = r#"
async def fetch_data(url: str) -> str:
    return "data"
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
        assert_eq!(hir.functions[0].name, "fetch_data");
        assert!(hir.functions[0].properties.is_async);
    }

    #[test]
    fn test_function_with_varargs() {
        let source = r#"
def variadic(*args) -> int:
    return len(args)
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
        let func = &hir.functions[0];
        assert_eq!(func.params.len(), 1);
        assert!(func.params[0].is_vararg);
        assert_eq!(func.params[0].name, "args");
    }

    #[test]
    fn test_function_with_default_params() {
        let source = r#"
def greet(name: str = "World") -> str:
    return "Hello " + name
"#;
        let hir = parse_python_to_hir(source);
        let func = &hir.functions[0];
        assert_eq!(func.params.len(), 1);
        assert!(func.params[0].default.is_some());
    }

    #[test]
    fn test_for_loop_with_assignment() {
        let source = r#"
def sum_list(items: List[int]) -> int:
    total = 0
    for x in items:
        total = total + x
    return total
"#;
        let hir = parse_python_to_hir(source);
        let func = &hir.functions[0];
        // Should have: assignment, for loop, return
        assert!(func.body.len() >= 2);
        assert!(matches!(func.body[1], HirStmt::For { .. }));
    }

    #[test]
    fn test_while_loop_conversion() {
        let source = r#"
def countdown(n: int) -> int:
    while n > 0:
        n = n - 1
    return n
"#;
        let hir = parse_python_to_hir(source);
        let func = &hir.functions[0];
        assert!(matches!(func.body[0], HirStmt::While { .. }));
    }

    #[test]
    fn test_multiple_imports() {
        let source = r#"
import os
import sys
from typing import List, Dict, Optional
from collections import defaultdict
"#;
        let hir = parse_python_to_hir(source);
        assert!(hir.imports.len() >= 4);
    }

    #[test]
    fn test_aliased_import() {
        let source = "from collections import defaultdict as dd";
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.imports.len(), 1);
        assert_eq!(hir.imports[0].module, "collections");
        if let ImportItem::Aliased { name, alias } = &hir.imports[0].items[0] {
            assert_eq!(name, "defaultdict");
            assert_eq!(alias, "dd");
        } else {
            panic!("Expected aliased import");
        }
    }

    #[test]
    fn test_dict_expression() {
        let source = r#"
def make_dict() -> Dict[str, int]:
    return {"a": 1, "b": 2}
"#;
        let hir = parse_python_to_hir(source);
        let func = &hir.functions[0];
        if let HirStmt::Return(Some(HirExpr::Dict { .. })) = &func.body[0] {
            // success
        } else {
            panic!("Expected dict expression");
        }
    }

    #[test]
    fn test_list_comprehension() {
        let source = r#"
def squares(n: int) -> List[int]:
    return [x * x for x in range(n)]
"#;
        let hir = parse_python_to_hir(source);
        let func = &hir.functions[0];
        if let HirStmt::Return(Some(HirExpr::ListComp { .. })) = &func.body[0] {
            // success
        } else {
            panic!("Expected list comprehension");
        }
    }

    #[test]
    fn test_lambda_expression() {
        let source = r#"
def make_adder(x: int):
    return lambda y: x + y
"#;
        let hir = parse_python_to_hir(source);
        let func = &hir.functions[0];
        if let HirStmt::Return(Some(HirExpr::Lambda { .. })) = &func.body[0] {
            // success
        } else {
            panic!("Expected lambda expression");
        }
    }

    #[test]
    fn test_tuple_unpacking() {
        let source = r#"
def swap(a: int, b: int) -> tuple:
    a, b = b, a
    return (a, b)
"#;
        let hir = parse_python_to_hir(source);
        let func = &hir.functions[0];
        // First statement should be tuple unpacking assignment
        assert!(func.body.len() >= 2);
    }

    #[test]
    fn test_try_except() {
        let source = r#"
def safe_divide(a: int, b: int) -> Optional[int]:
    try:
        return a / b
    except ZeroDivisionError:
        return None
"#;
        let hir = parse_python_to_hir(source);
        let func = &hir.functions[0];
        assert!(matches!(func.body[0], HirStmt::Try { .. }));
    }

    #[test]
    fn test_with_statement() {
        let source = r#"
def read_file(path: str) -> str:
    with open(path) as f:
        return f.read()
"#;
        let hir = parse_python_to_hir(source);
        let func = &hir.functions[0];
        assert!(matches!(func.body[0], HirStmt::With { .. }));
    }

    #[test]
    fn test_ternary_expression() {
        let source = r#"
def abs_value(x: int) -> int:
    return x if x >= 0 else -x
"#;
        let hir = parse_python_to_hir(source);
        let func = &hir.functions[0];
        // Ternary is represented as IfExpr in HIR
        if let HirStmt::Return(Some(HirExpr::IfExpr { .. })) = &func.body[0] {
            // success
        } else {
            panic!("Expected IfExpr (ternary expression)");
        }
    }

    #[test]
    fn test_chained_comparison() {
        let source = r#"
def in_range(x: int) -> bool:
    return 0 <= x <= 10
"#;
        let hir = parse_python_to_hir(source);
        let func = &hir.functions[0];
        // Chained comparisons should be parsed - check it's a return with expression
        if let HirStmt::Return(Some(_)) = &func.body[0] {
            // success - parsed and has expression
        } else {
            panic!("Expected return with comparison expression");
        }
    }

    #[test]
    fn test_f_string() {
        let source = r#"
def greet(name: str) -> str:
    return f"Hello, {name}!"
"#;
        let hir = parse_python_to_hir(source);
        let func = &hir.functions[0];
        if let HirStmt::Return(Some(HirExpr::FString { .. })) = &func.body[0] {
            // success
        } else {
            panic!("Expected f-string");
        }
    }

    #[test]
    fn test_attribute_access() {
        let source = r#"
def get_length(s: str) -> int:
    return s.upper().lower()
"#;
        let hir = parse_python_to_hir(source);
        let func = &hir.functions[0];
        if let HirStmt::Return(Some(HirExpr::MethodCall { .. })) = &func.body[0] {
            // success
        } else {
            panic!("Expected method call chain");
        }
    }

    #[test]
    fn test_subscript_expression() {
        let source = r#"
def get_first(items: List[int]) -> int:
    return items[0]
"#;
        let hir = parse_python_to_hir(source);
        let func = &hir.functions[0];
        if let HirStmt::Return(Some(HirExpr::Index { .. })) = &func.body[0] {
            // success
        } else {
            panic!("Expected index expression");
        }
    }

    #[test]
    fn test_boolean_operations() {
        let source = r#"
def check(a: bool, b: bool, c: bool) -> bool:
    return a and b or not c
"#;
        let hir = parse_python_to_hir(source);
        let func = &hir.functions[0];
        // Should have logical operations
        if let HirStmt::Return(Some(HirExpr::Binary { op: BinOp::Or, .. })) = &func.body[0] {
            // success
        } else {
            panic!("Expected boolean expression");
        }
    }

    #[test]
    fn test_augmented_assignment() {
        let source = r#"
def increment(x: int) -> int:
    x += 1
    x *= 2
    return x
"#;
        let hir = parse_python_to_hir(source);
        let func = &hir.functions[0];
        assert!(func.body.len() >= 3);
        // Augmented assignments become regular Assign with computed value
        assert!(matches!(func.body[0], HirStmt::Assign { .. }));
        assert!(matches!(func.body[1], HirStmt::Assign { .. }));
    }

    #[test]
    fn test_pass_statement() {
        let source = r#"
def noop() -> None:
    pass
"#;
        let hir = parse_python_to_hir(source);
        let func = &hir.functions[0];
        assert!(matches!(func.body[0], HirStmt::Pass));
    }

    #[test]
    fn test_break_continue() {
        let source = r#"
def loop_control(items: List[int]) -> int:
    for x in items:
        if x == 0:
            continue
        if x < 0:
            break
    return 0
"#;
        let hir = parse_python_to_hir(source);
        let func = &hir.functions[0];
        // Should parse without error and have for loop
        assert!(matches!(func.body[0], HirStmt::For { .. }));
    }

    #[test]
    fn test_nested_function() {
        let source = r#"
def outer(x: int) -> int:
    def inner(y: int) -> int:
        return y * 2
    return inner(x)
"#;
        let hir = parse_python_to_hir(source);
        // Should have at least the outer function
        assert!(!hir.functions.is_empty());
        assert_eq!(hir.functions[0].name, "outer");
    }

    // ========================================================
    // DEPYLER-COVERAGE-95: Additional ast_bridge tests
    // ========================================================

    #[test]
    fn test_create_default_value_for_int() {
        let bridge = AstBridge::new();
        let result = bridge.create_default_value_for_type(&crate::hir::Type::Int);
        assert!(matches!(
            result,
            Some(crate::hir::HirExpr::Literal(crate::hir::Literal::Int(0)))
        ));
    }

    #[test]
    fn test_create_default_value_for_float() {
        let bridge = AstBridge::new();
        let result = bridge.create_default_value_for_type(&crate::hir::Type::Float);
        assert!(matches!(
            result,
            Some(crate::hir::HirExpr::Literal(crate::hir::Literal::Float(f))) if f == 0.0
        ));
    }

    #[test]
    fn test_create_default_value_for_bool() {
        let bridge = AstBridge::new();
        let result = bridge.create_default_value_for_type(&crate::hir::Type::Bool);
        assert!(matches!(
            result,
            Some(crate::hir::HirExpr::Literal(crate::hir::Literal::Bool(
                false
            )))
        ));
    }

    #[test]
    fn test_create_default_value_for_string() {
        let bridge = AstBridge::new();
        let result = bridge.create_default_value_for_type(&crate::hir::Type::String);
        assert!(matches!(
            result,
            Some(crate::hir::HirExpr::Literal(crate::hir::Literal::String(_)))
        ));
    }

    #[test]
    fn test_create_default_value_for_unknown() {
        let bridge = AstBridge::new();
        let result = bridge.create_default_value_for_type(&crate::hir::Type::Unknown);
        assert!(result.is_some());
    }

    #[test]
    fn test_create_default_value_for_list() {
        let bridge = AstBridge::new();
        let result =
            bridge.create_default_value_for_type(&crate::hir::Type::List(Box::new(Type::Int)));
        assert!(result.is_some());
    }

    #[test]
    fn test_class_with_base() {
        let source = r#"
class Child(Parent):
    def method(self) -> None:
        pass
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.classes.len(), 1);
        assert_eq!(hir.classes[0].name, "Child");
    }

    #[test]
    fn test_class_with_init_and_fields() {
        let source = r#"
class Point:
    def __init__(self, x: int, y: int) -> None:
        self.x = x
        self.y = y
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.classes.len(), 1);
        assert_eq!(hir.classes[0].name, "Point");
        assert!(!hir.classes[0].fields.is_empty());
    }

    #[test]
    fn test_class_with_staticmethod() {
        let source = r#"
class Utils:
    @staticmethod
    def helper(x: int) -> int:
        return x * 2
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.classes.len(), 1);
        assert!(!hir.classes[0].methods.is_empty());
    }

    #[test]
    fn test_class_with_classmethod() {
        let source = r#"
class Factory:
    @classmethod
    def create(cls) -> "Factory":
        return cls()
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.classes.len(), 1);
        // classmethod should be converted
        assert!(!hir.classes[0].methods.is_empty());
    }

    #[test]
    fn test_protocol_conversion() {
        let source = r#"
from typing import Protocol

class Comparable(Protocol):
    def compare(self, other: "Comparable") -> int:
        ...
"#;
        let hir = parse_python_to_hir(source);
        // Protocol should be converted to protocol type
        assert!(!hir.protocols.is_empty() || !hir.classes.is_empty());
    }

    #[test]
    fn test_function_with_kwargs() {
        let source = r#"
def greet(name: str, **kwargs) -> str:
    return "Hello " + name
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
        assert_eq!(hir.functions[0].name, "greet");
    }

    #[test]
    fn test_function_with_args_and_kwargs() {
        let source = r#"
def flexible(*args, **kwargs) -> None:
    pass
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_set_expression() {
        let source = r#"
def get_set() -> set:
    return {1, 2, 3}
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_set_comprehension() {
        let source = r#"
def squares() -> set:
    return {x * x for x in range(10)}
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_dict_comprehension() {
        let source = r#"
def make_dict() -> dict:
    return {k: v for k, v in items}
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_generator_expression() {
        let source = r#"
def gen() -> None:
    result = sum(x * x for x in range(10))
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_walrus_operator() {
        let source = r#"
def check(data: list) -> bool:
    if (n := len(data)) > 0:
        return True
    return False
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_match_statement() {
        let source = r#"
def process(cmd: str) -> int:
    match cmd:
        case "start":
            return 1
        case "stop":
            return 0
        case _:
            return -1
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_slice_with_step() {
        let source = r#"
def reverse(data: list) -> list:
    return data[::-1]
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_multi_target_assignment() {
        let source = r#"
def swap() -> None:
    a, b = b, a
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_starred_expression() {
        let source = r#"
def unpack(data: list) -> None:
    first, *rest = data
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_async_with_statement() {
        let source = r#"
async def read_file() -> str:
    async with open("file.txt") as f:
        return await f.read()
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
        assert!(hir.functions[0].properties.is_async);
    }

    #[test]
    fn test_async_for_loop() {
        let source = r#"
async def process() -> None:
    async for item in stream:
        print(item)
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
        assert!(hir.functions[0].properties.is_async);
    }

    #[test]
    fn test_assert_statement() {
        let source = r#"
def validate(x: int) -> None:
    assert x > 0, "x must be positive"
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_raise_statement() {
        let source = r#"
def fail() -> None:
    raise ValueError("error")
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_global_statement() {
        let source = r#"
counter = 0
def increment() -> None:
    global counter
    counter += 1
"#;
        let hir = parse_python_to_hir(source);
        assert!(!hir.functions.is_empty());
    }

    #[test]
    fn test_nonlocal_statement() {
        let source = r#"
def outer() -> int:
    count = 0
    def inner() -> None:
        nonlocal count
        count += 1
    inner()
    return count
"#;
        let hir = parse_python_to_hir(source);
        assert!(!hir.functions.is_empty());
    }

    #[test]
    fn test_yield_statement() {
        let source = r#"
def generate() -> int:
    yield 1
    yield 2
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_yield_from() {
        let source = r#"
def delegate() -> int:
    yield from other_gen()
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_delete_statement() {
        let source = r#"
def cleanup(data: dict) -> None:
    del data["key"]
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_complex_type_annotation() {
        let source = r#"
def process(data: list[dict[str, int]]) -> dict[str, list[int]]:
    return {}
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_union_type_annotation() {
        let source = r#"
def maybe(x: int | str) -> int | None:
    return None
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_optional_type_annotation() {
        let source = r#"
from typing import Optional
def maybe(x: Optional[int]) -> Optional[str]:
    return None
"#;
        let hir = parse_python_to_hir(source);
        assert!(!hir.functions.is_empty());
    }

    #[test]
    fn test_callable_annotation() {
        let source = r#"
from typing import Callable
def apply(f: Callable[[int], int], x: int) -> int:
    return f(x)
"#;
        let hir = parse_python_to_hir(source);
        assert!(!hir.functions.is_empty());
    }

    #[test]
    fn test_class_with_property() {
        let source = r#"
class Circle:
    def __init__(self, radius: float) -> None:
        self._radius = radius

    @property
    def radius(self) -> float:
        return self._radius
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.classes.len(), 1);
    }

    #[test]
    fn test_class_with_property_setter() {
        let source = r#"
class Circle:
    def __init__(self, radius: float) -> None:
        self._radius = radius

    @property
    def radius(self) -> float:
        return self._radius

    @radius.setter
    def radius(self, value: float) -> None:
        self._radius = value
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.classes.len(), 1);
    }

    #[test]
    fn test_multiple_decorators() {
        let source = r#"
@decorator1
@decorator2
def func() -> None:
    pass
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_decorator_with_args() {
        let source = r#"
@my_decorator(arg=True)
def func() -> None:
    pass
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_empty_function() {
        let source = r#"
def empty() -> None:
    pass
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_ellipsis_body() {
        let source = r#"
def stub() -> int:
    ...
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_bytes_literal() {
        let source = r#"
def get_bytes() -> bytes:
    return b"hello"
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_formatted_string_complex() {
        let source = r#"
def format(x: int, y: float) -> str:
    return f"{x} + {y:.2f} = {x + y}"
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_multiline_string() {
        let source = r#"
def docstring() -> str:
    """
    This is a
    multiline string
    """
    return "done"
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_try_except_finally() {
        let source = r#"
def safe() -> None:
    try:
        risky()
    except ValueError:
        handle()
    finally:
        cleanup()
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_try_except_else() {
        let source = r#"
def safe() -> None:
    try:
        risky()
    except:
        handle()
    else:
        success()
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_multiple_except_handlers() {
        let source = r#"
def safe() -> None:
    try:
        risky()
    except ValueError as e:
        handle_value(e)
    except TypeError as e:
        handle_type(e)
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_context_manager_as() {
        let source = r#"
def read_file() -> str:
    with open("file.txt") as f:
        return f.read()
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_multiple_context_managers() {
        let source = r#"
def copy_file() -> None:
    with open("in.txt") as f1, open("out.txt", "w") as f2:
        f2.write(f1.read())
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_class_with_class_var() {
        let source = r#"
class Counter:
    count: int = 0

    def increment(self) -> None:
        Counter.count += 1
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.classes.len(), 1);
    }

    #[test]
    fn test_dataclass_style() {
        let source = r#"
class Point:
    x: float
    y: float
"#;
        let hir = parse_python_to_hir(source);
        // Should be converted even without decorator
        assert!(!hir.classes.is_empty() || !hir.protocols.is_empty());
    }

    #[test]
    fn test_await_expression() {
        let source = r#"
async def fetch() -> str:
    result = await get_data()
    return result
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
        assert!(hir.functions[0].properties.is_async);
    }

    #[test]
    fn test_lambda_in_call() {
        let source = r#"
def sort_items(items: list) -> list:
    return sorted(items, key=lambda x: x.value)
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_conditional_expression_nested() {
        let source = r#"
def classify(x: int) -> str:
    return "positive" if x > 0 else "zero" if x == 0 else "negative"
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_not_in_operator() {
        let source = r#"
def check(x: int, data: list) -> bool:
    return x not in data
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_is_not_operator() {
        let source = r#"
def check(x: int) -> bool:
    return x is not None
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_power_operator() {
        let source = r#"
def square(x: int) -> int:
    return x ** 2
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_floor_division() {
        let source = r#"
def divide(a: int, b: int) -> int:
    return a // b
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_bitwise_operations() {
        let source = r#"
def bits(a: int, b: int) -> int:
    return (a & b) | (a ^ b) | (a << 2) | (b >> 1)
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_bitwise_not() {
        let source = r#"
def invert(x: int) -> int:
    return ~x
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_imports_from_package() {
        let source = r#"
from os.path import join, dirname
def get_path() -> str:
    return join(dirname(__file__), "data")
"#;
        let hir = parse_python_to_hir(source);
        assert!(!hir.imports.is_empty() || !hir.functions.is_empty());
    }

    #[test]
    fn test_relative_import() {
        let source = r#"
from . import utils
from ..helpers import helper
"#;
        let _hir = parse_python_to_hir(source);
        // Relative imports should be handled - test verifies no panic
    }

    #[test]
    fn test_star_import() {
        let source = r#"
from module import *
"#;
        let hir = parse_python_to_hir(source);
        assert!(!hir.imports.is_empty());
    }

    #[test]
    fn test_module_level_expr() {
        let source = r#"
print("module loaded")

def func() -> None:
    pass
"#;
        let hir = parse_python_to_hir(source);
        // Should handle module-level expression
        assert!(!hir.functions.is_empty());
    }

    #[test]
    fn test_class_with_multiple_inheritance() {
        let source = r#"
class MyClass(Base1, Base2):
    pass
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.classes.len(), 1);
    }

    #[test]
    fn test_generic_class() {
        let source = r#"
from typing import Generic, TypeVar
T = TypeVar('T')
class Container(Generic[T]):
    def __init__(self, value: T) -> None:
        self.value = value
"#;
        let hir = parse_python_to_hir(source);
        // Should handle generics
        assert!(!hir.classes.is_empty() || !hir.type_aliases.is_empty());
    }

    #[test]
    fn test_type_alias_with_typevar() {
        let source = r#"
from typing import TypeVar, List
T = TypeVar('T')
MyList = List[T]
"#;
        let _hir = parse_python_to_hir(source);
        // TypeVar and alias should be handled - test verifies no panic
    }

    #[test]
    fn test_new_style_union() {
        let source = r#"
def process(x: int | str | None) -> int | str:
    return x if x else 0
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_convert_parameters_keyword_only() {
        let source = r#"
def func(*, name: str, value: int) -> None:
    pass
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_convert_parameters_positional_only() {
        let source = r#"
def func(x: int, /, y: int) -> int:
    return x + y
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_propagate_can_fail() {
        let source = r#"
def may_fail() -> int:
    raise ValueError("error")

def calls_failing() -> int:
    return may_fail()
"#;
        let hir = parse_python_to_hir(source);
        // Both functions should be present
        assert!(hir.functions.len() >= 2);
    }

    #[test]
    fn test_elif_chain() {
        let source = r#"
def classify(x: int) -> str:
    if x < 0:
        return "negative"
    elif x == 0:
        return "zero"
    elif x < 10:
        return "small"
    else:
        return "large"
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_for_else() {
        let source = r#"
def search(data: list, target: int) -> int:
    for i, x in enumerate(data):
        if x == target:
            return i
    else:
        return -1
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_while_else() {
        let source = r#"
def countdown(n: int) -> None:
    while n > 0:
        n -= 1
    else:
        print("done")
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
    }

    #[test]
    fn test_extract_docstring_multiline() {
        let source = r#"
def documented() -> None:
    """
    This is a docstring.
    It has multiple lines.

    And even blank lines.
    """
    pass
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.functions.len(), 1);
        assert!(hir.functions[0].docstring.is_some());
    }

    #[test]
    fn test_class_docstring() {
        let source = r#"
class MyClass:
    """This is a class docstring."""

    def method(self) -> None:
        pass
"#;
        let hir = parse_python_to_hir(source);
        assert_eq!(hir.classes.len(), 1);
    }

    #[test]
    fn test_is_type_name_builtin() {
        let bridge = AstBridge::new();
        assert!(bridge.is_type_name("int"));
        assert!(bridge.is_type_name("str"));
        assert!(bridge.is_type_name("bool"));
        assert!(bridge.is_type_name("float"));
        assert!(bridge.is_type_name("list"));
        assert!(bridge.is_type_name("dict"));
        assert!(!bridge.is_type_name("myvar"));
    }

    #[test]
    fn test_is_type_variable() {
        let bridge = AstBridge::new();
        // Standard type variable names
        assert!(bridge.is_type_variable("T"));
        assert!(bridge.is_type_variable("K"));
        assert!(bridge.is_type_variable("V"));
        assert!(!bridge.is_type_variable("x"));
        assert!(!bridge.is_type_variable("count"));
    }
}
