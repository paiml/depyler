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
            // DEPYLER-0273: Handle PEP 604 union syntax (int | None)
            ast::Expr::BinOp(b) if matches!(b.op, ast::Operator::BitOr) => {
                Self::extract_union_from_binop(b)
            }
            // DEPYLER-0501: Handle Callable[[Any], Any] - parameter list is ExprList
            ast::Expr::List(list) => {
                // For now, treat list of types as tuple-like (used in Callable)
                // Map to Unknown for simplicity
                if list.elts.is_empty() {
                    Ok(Type::Unknown)
                } else if list.elts.len() == 1 {
                    // Single element list - extract that type
                    Self::extract_type(&list.elts[0])
                } else {
                    // Multiple elements - for Callable[[T1, T2], R], map to Unknown
                    // Full Callable support would need function pointer types
                    Ok(Type::Unknown)
                }
            }
            // DEPYLER-0512: Handle module-qualified types (module.Class)
            ast::Expr::Attribute(attr) => Self::extract_module_qualified_type(attr),
            _ => bail!("Unsupported type annotation: {:?}", expr),
        }
    }

    pub fn extract_simple_type(name: &str) -> Result<Type> {
        // DEPYLER-0501: Handle Any type (maps to Unknown)
        if name == "Any" {
            return Ok(Type::Unknown);
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

        // Special case: typing.Any → Unknown
        if type_name == "Any" {
            return Ok(Type::Unknown);
        }

        // Check if it's a known builtin type (e.g., typing.List → list[T])
        if let Some(ty) = Self::try_extract_builtin_type(type_name) {
            return Ok(ty);
        }

        // Otherwise, treat as custom type (module prefix discarded)
        // Examples: argparse.Namespace → Namespace, pathlib.Path → Path
        Ok(Type::Custom(type_name.to_string()))
    }
}
