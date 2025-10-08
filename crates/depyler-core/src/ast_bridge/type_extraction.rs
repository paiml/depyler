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
            ast::Expr::Constant(c) if matches!(c.value, ast::Constant::None) => {
                Ok(Type::None)
            }
            _ => bail!("Unsupported type annotation"),
        }
    }

    pub fn extract_simple_type(name: &str) -> Result<Type> {
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
            "List" => Self::extract_list_type(s),
            "Dict" => Self::extract_dict_type(s),
            "Set" => Self::extract_set_type(s),
            "Optional" => Self::extract_optional_type(s),
            "Union" => Self::extract_union_type(s),
            "Generic" => Self::extract_parameterized_generic(s),
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
}
