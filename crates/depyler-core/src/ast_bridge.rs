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

pub struct AstBridge {
    source_code: Option<String>,
    annotation_extractor: AnnotationExtractor,
    annotation_parser: AnnotationParser,
}

impl Default for AstBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl AstBridge {
    pub fn new() -> Self {
        Self {
            source_code: None,
            annotation_extractor: AnnotationExtractor::new(),
            annotation_parser: AnnotationParser::new(),
        }
    }

    pub fn with_source(mut self, source: String) -> Self {
        self.source_code = Some(source);
        self
    }

    pub fn python_to_hir(&self, module: ast::Mod) -> Result<HirModule> {
        match module {
            ast::Mod::Module(m) => self.convert_module(m),
            _ => bail!("Only module-level code is supported"),
        }
    }

    fn convert_module(&self, module: ast::ModModule) -> Result<HirModule> {
        let mut functions = Vec::new();
        let mut imports = Vec::new();
        let mut type_aliases = Vec::new();
        let mut protocols = Vec::new();
        let mut classes = Vec::new();

        for stmt in module.body {
            match stmt {
                ast::Stmt::FunctionDef(f) => {
                    functions.push(self.convert_function(f)?);
                }
                ast::Stmt::Import(i) => {
                    imports.extend(convert_import(i)?);
                }
                ast::Stmt::ImportFrom(i) => {
                    imports.extend(convert_import_from(i)?);
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
                    // Try to parse as type alias
                    if let Some(type_alias) = self.try_convert_type_alias(&assign)? {
                        type_aliases.push(type_alias);
                    }
                    // Otherwise skip regular assignments at module level
                }
                ast::Stmt::AnnAssign(ann_assign) => {
                    // Try to parse annotated assignment as type alias
                    if let Some(type_alias) = self.try_convert_annotated_type_alias(&ann_assign)? {
                        type_aliases.push(type_alias);
                    }
                    // Otherwise skip regular annotated assignments at module level
                }
                _ => {
                    // Skip other statements for now
                }
            }
        }

        Ok(HirModule {
            functions,
            imports,
            type_aliases,
            protocols,
            classes,
        })
    }

    fn convert_function(&self, func: ast::StmtFunctionDef) -> Result<HirFunction> {
        let name = func.name.to_string();
        let params = convert_parameters(&func.args)?;
        let ret_type = TypeExtractor::extract_return_type(&func.returns)?;

        // Extract annotations from source code if available
        let annotations = self.extract_function_annotations(&func);

        // Extract docstring and filter it from the body
        let (docstring, filtered_body) = extract_docstring_and_body(func.body)?;
        let properties = FunctionAnalyzer::analyze(&filtered_body);

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

    fn try_convert_type_alias(&self, assign: &ast::StmtAssign) -> Result<Option<TypeAlias>> {
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
            ast::Expr::Subscript(_) => (TypeExtractor::extract_type(&assign.value)?, false),
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

    fn try_convert_protocol(&self, class: &ast::StmtClassDef) -> Result<Option<Protocol>> {
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

    fn try_convert_class(&self, class: &ast::StmtClassDef) -> Result<Option<HirClass>> {
        // Extract docstring if present
        let docstring = self.extract_class_docstring(&class.body);

        // Check if it's a dataclass
        let is_dataclass = class.decorator_list.iter().any(|d| {
            matches!(d, ast::Expr::Name(n) if n.id.as_str() == "dataclass")
                || matches!(d, ast::Expr::Attribute(a) if a.attr.as_str() == "dataclass")
        });

        // Extract base classes (for now, just store the names)
        let base_classes = class
            .bases
            .iter()
            .filter_map(|base| {
                if let ast::Expr::Name(n) = base {
                    Some(n.id.to_string())
                } else {
                    None
                }
            })
            .collect();

        // Convert methods and fields
        let mut methods = Vec::new();
        let mut fields = Vec::new();
        let mut init_method = None;

        for stmt in &class.body {
            match stmt {
                ast::Stmt::FunctionDef(method) => {
                    if method.name.as_str() == "__init__" {
                        // Store __init__ for field inference
                        init_method = Some(method);
                    }
                    if let Some(hir_method) = self.convert_method(method)? {
                        methods.push(hir_method);
                    }
                }
                ast::Stmt::AnnAssign(ann_assign) => {
                    // Handle annotated fields
                    if let ast::Expr::Name(target) = ann_assign.target.as_ref() {
                        let field_name = target.id.to_string();
                        let field_type = TypeExtractor::extract_type(&ann_assign.annotation)?;

                        let default_value = if let Some(_value) = &ann_assign.value {
                            // For now, skip default value conversion
                            // TODO: Implement expression conversion for class fields
                            None
                        } else {
                            None
                        };

                        fields.push(HirField {
                            name: field_name,
                            field_type,
                            default_value,
                            is_class_var: false, // TODO: Detect class variables
                        });
                    }
                }
                _ => {
                    // Skip other statements for now
                }
            }
        }

        // Infer fields from __init__ if no explicit fields are defined
        if fields.is_empty() && !is_dataclass {
            if let Some(init) = init_method {
                fields = self.infer_fields_from_init(init)?;
            }
        }

        Ok(Some(HirClass {
            name: class.name.to_string(),
            base_classes,
            methods,
            fields,
            is_dataclass,
            docstring,
        }))
    }

    fn convert_method(&self, method: &ast::StmtFunctionDef) -> Result<Option<HirMethod>> {
        use smallvec::smallvec;

        let name = method.name.to_string();

        // Skip dunder methods except __init__, __iter__, __next__, __enter__, __exit__
        if name.starts_with("__")
            && name.ends_with("__")
            && !matches!(
                name.as_str(),
                "__init__" | "__iter__" | "__next__" | "__enter__" | "__exit__"
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
            params.push((param_name, param_type));
        }

        // Convert return type
        let ret_type = if let Some(ret) = &method.returns {
            TypeExtractor::extract_type(ret)?
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
            docstring,
        }))
    }

    fn extract_class_docstring(&self, body: &[ast::Stmt]) -> Option<String> {
        if let Some(ast::Stmt::Expr(expr)) = body.first() {
            if let ast::Expr::Constant(c) = expr.value.as_ref() {
                if let ast::Constant::Str(s) = &c.value {
                    return Some(s.to_string());
                }
            }
        }
        None
    }

    fn extract_class_type_params(&self, class: &ast::StmtClassDef) -> Vec<String> {
        // Look for Generic[T, U] in base classes
        for base in &class.bases {
            if let ast::Expr::Subscript(subscript) = base {
                if let ast::Expr::Name(n) = subscript.value.as_ref() {
                    if n.id.as_str() == "Generic" {
                        return self.extract_generic_params(&subscript.slice);
                    }
                }
            }
        }
        Vec::new()
    }

    fn extract_generic_params(&self, slice: &ast::Expr) -> Vec<String> {
        match slice {
            ast::Expr::Name(n) => vec![n.id.to_string()],
            ast::Expr::Tuple(tuple) => tuple
                .elts
                .iter()
                .filter_map(|elt| {
                    if let ast::Expr::Name(n) = elt {
                        Some(n.id.to_string())
                    } else {
                        None
                    }
                })
                .collect(),
            _ => Vec::new(),
        }
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

        // Look for self.field assignments in __init__
        for stmt in &init.body {
            if let ast::Stmt::Assign(assign) = stmt {
                // Check if it's a self.field assignment
                if assign.targets.len() == 1 {
                    if let ast::Expr::Attribute(attr) = &assign.targets[0] {
                        if let ast::Expr::Name(name) = attr.value.as_ref() {
                            if name.id.as_str() == "self" {
                                let field_name = attr.attr.to_string();

                                // Try to infer type from the assigned value
                                let field_type =
                                    if let ast::Expr::Name(value_name) = assign.value.as_ref() {
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
        }

        Ok(fields)
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
}

// Keep the old function for backwards compatibility
pub fn python_to_hir(module: ast::Mod) -> Result<HirModule> {
    AstBridge::new().python_to_hir(module)
}

fn convert_parameters(args: &ast::Arguments) -> Result<Vec<(Symbol, Type)>> {
    let mut params = Vec::new();

    for arg in args.args.iter() {
        let name = arg.def.arg.to_string();
        let ty = if let Some(annotation) = &arg.def.annotation {
            TypeExtractor::extract_type(annotation)?
        } else {
            Type::Unknown
        };
        params.push((name, ty));
    }

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
        _ => bail!("Unsupported assignment target"),
    }
}

pub(crate) fn extract_simple_target(expr: &ast::Expr) -> Result<Symbol> {
    match expr {
        ast::Expr::Name(n) => Ok(n.id.to_string()),
        _ => bail!("Only simple name targets supported for loops"),
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
        _ => bail!("Unsupported comparison operator"),
    })
}

fn convert_import(import: ast::StmtImport) -> Result<Vec<Import>> {
    import
        .names
        .into_iter()
        .map(|alias| {
            let module = alias.name.to_string();
            let items = if let Some(asname) = alias.asname {
                vec![ImportItem::Aliased {
                    name: module.clone(),
                    alias: asname.to_string(),
                }]
            } else {
                vec![ImportItem::Named(module.clone())]
            };
            Ok(Import { module, items })
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

    Ok(vec![Import { module, items }])
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
    let filtered_body = body
        .into_iter()
        .skip(start_index)
        .map(convert_stmt)
        .collect::<Result<Vec<_>>>()?;

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
        AstBridge::new()
            .with_source(source.to_string())
            .python_to_hir(ast)
            .unwrap()
    }

    #[test]
    fn test_simple_function_conversion() {
        let source = "def add(a: int, b: int) -> int:\n    return a + b";
        let hir = parse_python_to_hir(source);

        assert_eq!(hir.functions.len(), 1);
        let func = &hir.functions[0];
        assert_eq!(func.name, "add");
        assert_eq!(func.params.len(), 2);
        assert_eq!(func.params[0].0, "a");
        assert_eq!(func.params[0].1, Type::Int);
        assert_eq!(func.ret_type, Type::Int);
    }

    #[test]
    fn test_type_annotation_conversion() {
        let source = "def process(items: List[str]) -> Optional[int]:\n    return None";
        let hir = parse_python_to_hir(source);

        let func = &hir.functions[0];
        assert_eq!(func.params[0].1, Type::List(Box::new(Type::String)));
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
            assert_eq!(target, "item");
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
        if let HirStmt::Return(Some(HirExpr::Call { func: fname, args })) = &func.body[0] {
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
}
