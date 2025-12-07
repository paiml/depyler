use crate::hir::Type;
use anyhow::{bail, Result};
use rustpython_ast::{self as ast};

#[cfg(test)]
#[path = "type_extraction_tests.rs"]
mod tests;

/// Utility for extracting type information from Python AST type annotations
///
/// # Examples
///
/// ```rust
/// use depyler_core::ast_bridge::TypeExtractor;
/// use depyler_core::hir::Type;
///
/// // Extract simple types
/// let int_type = TypeExtractor::extract_simple_type("int").unwrap();
/// assert_eq!(int_type, Type::Int);
///
/// // Extract type variables
/// let type_var = TypeExtractor::extract_simple_type("T").unwrap();
/// assert_eq!(type_var, Type::TypeVar("T".to_string()));
///
/// // Extract custom types
/// let custom = TypeExtractor::extract_simple_type("MyClass").unwrap();
/// assert_eq!(custom, Type::Custom("MyClass".to_string()));
/// ```
pub struct TypeExtractor;

impl TypeExtractor {
    pub fn extract_return_type(returns: &Option<Box<ast::Expr>>) -> Result<Type> {
        if let Some(ret) = returns {
            Self::extract_type(ret)
        } else {
            Ok(Type::Unknown)
        }
    }

    pub fn extract_type(expr: &ast::Expr) -> Result<Type> {
        match expr {
            ast::Expr::Name(n) => Self::extract_simple_type(n.id.as_str()),
            ast::Expr::Subscript(s) => Self::extract_generic_type(s),
            // Handle None constant (used in -> None return annotations)
            ast::Expr::Constant(c) if matches!(c.value, ast::Constant::None) => Ok(Type::None),
            // DEPYLER-0188: Handle Ellipsis in type annotations (used in tuple[int, ...], Generator[T, None, None])
            ast::Expr::Constant(c) if matches!(c.value, ast::Constant::Ellipsis) => {
                // Ellipsis in type context means "variable length" - map to Unknown for now
                Ok(Type::Unknown)
            }
            // DEPYLER-0188: Handle string literal forward references (-> "ClassName")
            // PEP 484: Forward references use string literals containing type names
            // DEPYLER-0740: Parse forward references to extract generic types
            ast::Expr::Constant(ast::ExprConstant {
                value: ast::Constant::Str(s),
                ..
            }) => Self::parse_forward_reference(s.as_str()),
            // DEPYLER-0273: Handle PEP 604 union syntax (int | None)
            ast::Expr::BinOp(b) if matches!(b.op, ast::Operator::BitOr) => {
                Self::extract_union_from_binop(b)
            }
            // DEPYLER-0501/DEPYLER-197: Handle Callable[[T1, T2], R] - parameter list is ExprList
            // Python Callable uses a list for parameters: Callable[[int, str], bool]
            // Extract as Tuple so type mapper can build Box<dyn Fn(i32, String) -> bool>
            ast::Expr::List(list) => {
                if list.elts.is_empty() {
                    // Empty list [] -> no parameters
                    Ok(Type::Unknown)
                } else if list.elts.len() == 1 {
                    // Single element list [T] - extract that type directly
                    Self::extract_type(&list.elts[0])
                } else {
                    // Multiple elements [T1, T2, ...] -> extract as Tuple for Callable params
                    // This allows Callable[[int, str], bool] to correctly map parameters
                    let types = list
                        .elts
                        .iter()
                        .map(Self::extract_type)
                        .collect::<Result<Vec<_>>>()?;
                    Ok(Type::Tuple(types))
                }
            }
            // DEPYLER-0512: Handle module-qualified types (module.Class)
            ast::Expr::Attribute(attr) => Self::extract_module_qualified_type(attr),
            _ => bail!("Unsupported type annotation: {:?}", expr),
        }
    }

    pub fn extract_simple_type(name: &str) -> Result<Type> {
        // DEPYLER-0725: Handle Any type - keep as Custom("Any") so type_mapper produces serde_json::Value
        // Previously mapped to Unknown which caused generic_inference to emit T: Clone
        if name == "Any" {
            return Ok(Type::Custom("Any".to_string()));
        }

        // Try builtin types first (complexity 5)
        if let Some(ty) = Self::try_extract_builtin_type(name) {
            return Ok(ty);
        }

        // Check for type variables (single uppercase letter)
        if name.len() == 1 && name.chars().next().is_some_and(|c| c.is_uppercase()) {
            return Ok(Type::TypeVar(name.to_string()));
        }

        // Default to custom type
        Ok(Type::Custom(name.to_string()))
    }

    /// DEPYLER-0740: Parse forward reference strings like "Container[U]"
    /// to extract generic type structure instead of treating as opaque string.
    fn parse_forward_reference(s: &str) -> Result<Type> {
        let s = s.trim();

        // Check for generic syntax: Base[T, U, ...]
        if let Some(bracket_pos) = s.find('[') {
            if s.ends_with(']') {
                let base = s[..bracket_pos].trim();
                let params_str = &s[bracket_pos + 1..s.len() - 1];

                // Parse the type parameters (handles simple cases like "T, U")
                let params: Vec<Type> = params_str
                    .split(',')
                    .map(|p| Self::extract_simple_type(p.trim()))
                    .collect::<Result<Vec<_>>>()?;

                if params.is_empty() {
                    return Self::extract_simple_type(base);
                }

                return Ok(Type::Generic {
                    base: base.to_string(),
                    params,
                });
            }
        }

        // No generic syntax, fall back to simple type extraction
        Self::extract_simple_type(s)
    }

    fn try_extract_builtin_type(name: &str) -> Option<Type> {
        // Try primitive types first (complexity 5)
        if let Some(ty) = Self::try_extract_primitive_type(name) {
            return Some(ty);
        }

        // Try collection types (complexity 4)
        Self::try_extract_collection_type(name)
    }

    fn try_extract_primitive_type(name: &str) -> Option<Type> {
        Some(match name {
            "int" => Type::Int,
            "float" => Type::Float,
            "str" => Type::String,
            "bool" => Type::Bool,
            "None" => Type::None,
            _ => return None,
        })
    }

    fn try_extract_collection_type(name: &str) -> Option<Type> {
        Some(match name {
            "list" => Type::List(Box::new(Type::Unknown)),
            "dict" => Type::Dict(Box::new(Type::Unknown), Box::new(Type::Unknown)),
            "set" => Type::Set(Box::new(Type::Unknown)),
            // DEPYLER-0662: Handle bare `tuple` annotation without type params
            // Python `-> tuple` should be inferred from return statements
            "tuple" => Type::Tuple(vec![]),
            _ => return None,
        })
    }

    fn extract_generic_type(s: &ast::ExprSubscript) -> Result<Type> {
        if let ast::Expr::Name(n) = s.value.as_ref() {
            Self::extract_named_generic_type(n.id.as_str(), s)
        } else {
            bail!("Complex type annotations not yet supported")
        }
    }

    fn extract_named_generic_type(name: &str, s: &ast::ExprSubscript) -> Result<Type> {
        match name {
            // Uppercase (from typing module)
            "List" => Self::extract_list_type(s),
            "Dict" => Self::extract_dict_type(s),
            "Set" => Self::extract_set_type(s),
            "Tuple" => Self::extract_tuple_type(s),
            "Optional" => Self::extract_optional_type(s),
            "Union" => Self::extract_union_type(s),
            "Generic" => Self::extract_parameterized_generic(s),
            "Final" => Self::extract_final_type(s),
            // Lowercase (PEP 585 - Python 3.9+ built-in generics)
            "list" => Self::extract_list_type(s),
            "dict" => Self::extract_dict_type(s),
            "set" => Self::extract_set_type(s),
            "tuple" => Self::extract_tuple_type(s),
            base_name => Self::extract_custom_generic(base_name, s),
        }
    }

    fn extract_custom_generic(base_name: &str, s: &ast::ExprSubscript) -> Result<Type> {
        let params = Self::extract_type_params(s)?;
        if params.is_empty() {
            Ok(Type::Custom(base_name.to_string()))
        } else {
            Ok(Type::Generic {
                base: base_name.to_string(),
                params,
            })
        }
    }

    fn extract_list_type(s: &ast::ExprSubscript) -> Result<Type> {
        let inner = Self::extract_type(s.slice.as_ref())?;
        Ok(Type::List(Box::new(inner)))
    }

    fn extract_dict_type(s: &ast::ExprSubscript) -> Result<Type> {
        if let ast::Expr::Tuple(t) = s.slice.as_ref() {
            if t.elts.len() == 2 {
                let key = Self::extract_type(&t.elts[0])?;
                let value = Self::extract_type(&t.elts[1])?;
                Ok(Type::Dict(Box::new(key), Box::new(value)))
            } else {
                bail!("Dict type requires exactly 2 type parameters")
            }
        } else {
            bail!("Invalid Dict type annotation")
        }
    }

    fn extract_set_type(s: &ast::ExprSubscript) -> Result<Type> {
        let inner = Self::extract_type(s.slice.as_ref())?;
        Ok(Type::Set(Box::new(inner)))
    }

    fn extract_tuple_type(s: &ast::ExprSubscript) -> Result<Type> {
        match s.slice.as_ref() {
            ast::Expr::Tuple(t) => {
                let types = t
                    .elts
                    .iter()
                    .map(Self::extract_type)
                    .collect::<Result<Vec<_>>>()?;
                Ok(Type::Tuple(types))
            }
            // Single type in tuple[T] case - make it a 1-tuple
            expr => {
                let ty = Self::extract_type(expr)?;
                Ok(Type::Tuple(vec![ty]))
            }
        }
    }

    fn extract_optional_type(s: &ast::ExprSubscript) -> Result<Type> {
        let inner = Self::extract_type(s.slice.as_ref())?;
        Ok(Type::Optional(Box::new(inner)))
    }

    fn extract_final_type(s: &ast::ExprSubscript) -> Result<Type> {
        let inner = Self::extract_type(s.slice.as_ref())?;
        Ok(Type::Final(Box::new(inner)))
    }

    fn extract_union_type(s: &ast::ExprSubscript) -> Result<Type> {
        match s.slice.as_ref() {
            ast::Expr::Tuple(t) => {
                let types = t
                    .elts
                    .iter()
                    .map(Self::extract_type)
                    .collect::<Result<Vec<_>>>()?;
                Ok(Type::Union(types))
            }
            // Single type in Union[T] case
            expr => {
                let ty = Self::extract_type(expr)?;
                Ok(Type::Union(vec![ty]))
            }
        }
    }

    /// DEPYLER-0273: Extract union type from PEP 604 binary operator syntax (T | U)
    ///
    /// Handles Python 3.10+ union type syntax like `int | None`, `int | str | None`.
    /// Special case: `T | None` is converted to `Optional[T]` (Rust's `Option<T>`)
    ///
    /// Complexity: 8 (recursive collection of union types)
    fn extract_union_from_binop(b: &ast::ExprBinOp) -> Result<Type> {
        // Collect all types in the union chain (left | middle | right)
        let mut types = Vec::new();
        Self::collect_union_types(&b.left, &mut types)?;
        Self::collect_union_types(&b.right, &mut types)?;

        // Special case: T | None → Optional[T] (Rust's Option<T>)
        if types.len() == 2 {
            let has_none = types.iter().any(|t| matches!(t, Type::None));
            if has_none {
                // Find the non-None type
                if let Some(inner_type) = types.iter().find(|t| !matches!(t, Type::None)) {
                    return Ok(Type::Optional(Box::new(inner_type.clone())));
                }
            }
        }

        // Multiple types (int | str | None, etc.) → Union type
        Ok(Type::Union(types))
    }

    /// Helper to recursively collect all types from a union chain
    fn collect_union_types(expr: &ast::Expr, types: &mut Vec<Type>) -> Result<()> {
        match expr {
            // Nested union: (a | b) | c
            ast::Expr::BinOp(b) if matches!(b.op, ast::Operator::BitOr) => {
                Self::collect_union_types(&b.left, types)?;
                Self::collect_union_types(&b.right, types)?;
                Ok(())
            }
            // Base case: single type
            _ => {
                types.push(Self::extract_type(expr)?);
                Ok(())
            }
        }
    }

    fn extract_type_params(s: &ast::ExprSubscript) -> Result<Vec<Type>> {
        match s.slice.as_ref() {
            ast::Expr::Tuple(t) => t.elts.iter().map(Self::extract_type).collect(),
            // Single type parameter
            expr => Ok(vec![Self::extract_type(expr)?]),
        }
    }

    fn extract_parameterized_generic(s: &ast::ExprSubscript) -> Result<Type> {
        match s.slice.as_ref() {
            ast::Expr::Name(n) => Self::try_extract_type_var_name(n.id.as_str()),
            ast::Expr::Tuple(t) => Self::try_extract_single_type_var_tuple(t),
            _ => bail!("Invalid Generic type annotation"),
        }
    }

    fn try_extract_type_var_name(name: &str) -> Result<Type> {
        if Self::is_type_var_name(name) {
            Ok(Type::TypeVar(name.to_string()))
        } else {
            bail!("Invalid Generic type annotation")
        }
    }

    fn try_extract_single_type_var_tuple(t: &ast::ExprTuple) -> Result<Type> {
        if t.elts.len() == 1 {
            if let ast::Expr::Name(n) = &t.elts[0] {
                if Self::is_type_var_name(n.id.as_str()) {
                    return Ok(Type::TypeVar(n.id.to_string()));
                }
            }
        }
        bail!("Complex Generic parameters not supported")
    }

    fn is_type_var_name(name: &str) -> bool {
        name.len() == 1 && name.chars().next().is_some_and(|c| c.is_uppercase())
    }

    /// DEPYLER-0512: Extract module-qualified type (module.Class or module.submodule.Class)
    ///
    /// Handles Python type annotations like:
    /// - `argparse.Namespace` → Custom("Namespace")
    /// - `typing.Any` → Unknown
    /// - `pathlib.Path` → Custom("Path")
    /// - `collections.abc.Iterable` → Custom("Iterable")
    ///
    /// Strategy: Extract the final attribute name and map known types, otherwise treat as custom.
    /// Module prefix is discarded since Rust types don't include module qualification.
    fn extract_module_qualified_type(attr: &ast::ExprAttribute) -> Result<Type> {
        // Extract the final attribute name (e.g., "Namespace" from "argparse.Namespace")
        let type_name = attr.attr.as_str();

        // Extract module name for special handling
        let module_name = Self::extract_module_name(&attr.value);

        // DEPYLER-0725: typing.Any → Custom("Any") → serde_json::Value via type_mapper
        if type_name == "Any" {
            return Ok(Type::Custom("Any".to_string()));
        }

        // Check if it's a known builtin type (e.g., typing.List → list[T])
        if let Some(ty) = Self::try_extract_builtin_type(type_name) {
            return Ok(ty);
        }

        // DEPYLER-0609: Map known Python stdlib types to Rust equivalents
        // This enables compilation even for unsupported modules
        if let Some(module) = &module_name {
            if let Some(rust_type) = Self::map_stdlib_type(module, type_name) {
                return Ok(rust_type);
            }
        }

        // Otherwise, treat as custom type (module prefix discarded)
        // Examples: argparse.Namespace → Namespace, pathlib.Path → Path
        Ok(Type::Custom(type_name.to_string()))
    }

    /// Extract the module name from an expression (e.g., "threading" from "threading.Semaphore")
    fn extract_module_name(expr: &ast::Expr) -> Option<String> {
        match expr {
            ast::Expr::Name(name) => Some(name.id.to_string()),
            ast::Expr::Attribute(attr) => {
                // Handle nested attributes like collections.abc
                let parent = Self::extract_module_name(&attr.value)?;
                Some(format!("{}.{}", parent, attr.attr))
            }
            _ => None,
        }
    }

    /// Map known Python stdlib module types to Rust equivalents
    /// DEPYLER-0609: Pragmatic compilation - map to types that compile
    fn map_stdlib_type(module: &str, type_name: &str) -> Option<Type> {
        match module {
            // Threading module - map to serde_json::Value as universal placeholder
            // Real sync primitives would need proper Rust equivalents
            "threading" => match type_name {
                "Lock" | "RLock" => Some(Type::Custom("std::sync::Mutex<()>".to_string())),
                "Semaphore" | "BoundedSemaphore" => {
                    // No stdlib Semaphore, use Mutex as placeholder
                    Some(Type::Custom("std::sync::Mutex<i32>".to_string()))
                }
                "Event" => Some(Type::Custom("std::sync::Condvar".to_string())),
                "Thread" => Some(Type::Custom("std::thread::JoinHandle<()>".to_string())),
                _ => Some(Type::Custom("serde_json::Value".to_string())),
            },
            // Datetime module - map to chrono types
            "datetime" => match type_name {
                "datetime" | "date" | "time" => {
                    Some(Type::Custom("chrono::DateTime<chrono::Utc>".to_string()))
                }
                "timedelta" => Some(Type::Custom("chrono::Duration".to_string())),
                _ => Some(Type::Custom("serde_json::Value".to_string())),
            },
            // Queue module
            "queue" => match type_name {
                "Queue" | "LifoQueue" | "PriorityQueue" => {
                    Some(Type::Custom("std::collections::VecDeque<serde_json::Value>".to_string()))
                }
                _ => Some(Type::Custom("serde_json::Value".to_string())),
            },
            // Multiprocessing
            "multiprocessing" => Some(Type::Custom("serde_json::Value".to_string())),
            // Asyncio types
            "asyncio" => match type_name {
                "Task" => Some(Type::Custom("tokio::task::JoinHandle<()>".to_string())),
                "Event" => Some(Type::Custom("tokio::sync::Notify".to_string())),
                "Queue" => Some(Type::Custom("tokio::sync::mpsc::Receiver<serde_json::Value>".to_string())),
                "Lock" => Some(Type::Custom("tokio::sync::Mutex<()>".to_string())),
                "Semaphore" => Some(Type::Custom("tokio::sync::Semaphore".to_string())),
                _ => Some(Type::Custom("serde_json::Value".to_string())),
            },
            // DEPYLER-0679: subprocess module types
            "subprocess" => match type_name {
                "CompletedProcess" => Some(Type::Custom("CompletedProcess".to_string())),
                "Popen" => Some(Type::Custom("std::process::Child".to_string())),
                _ => Some(Type::Custom("serde_json::Value".to_string())),
            },
            // Catch-all for other stdlib modules
            "collections" | "collections.abc" | "typing" | "types" | "functools" | "itertools"
            | "pathlib" | "os" | "sys" | "io" | "re" | "json" | "pickle"
            | "socket" | "ssl" | "http" | "urllib" => {
                // For most stdlib types, use serde_json::Value as placeholder
                Some(Type::Custom("serde_json::Value".to_string()))
            }
            _ => None, // Unknown module - let the default handling apply
        }
    }
}
