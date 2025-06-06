use crate::hir::Type;
use anyhow::{bail, Result};
use rustpython_ast::{self as ast};

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
            ast::Expr::Name(n) => Self::extract_simple_type(&n.id),
            ast::Expr::Subscript(s) => Self::extract_generic_type(s),
            _ => bail!("Unsupported type annotation"),
        }
    }

    fn extract_simple_type(name: &str) -> Result<Type> {
        Ok(match name {
            "int" => Type::Int,
            "float" => Type::Float,
            "str" => Type::String,
            "bool" => Type::Bool,
            "None" => Type::None,
            name => Type::Custom(name.to_string()),
        })
    }

    fn extract_generic_type(s: &ast::ExprSubscript) -> Result<Type> {
        if let ast::Expr::Name(n) = s.value.as_ref() {
            match n.id.as_str() {
                "List" => Self::extract_list_type(s),
                "Dict" => Self::extract_dict_type(s),
                "Optional" => Self::extract_optional_type(s),
                _ => Ok(Type::Custom(n.id.to_string())),
            }
        } else {
            bail!("Complex type annotations not yet supported")
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

    fn extract_optional_type(s: &ast::ExprSubscript) -> Result<Type> {
        let inner = Self::extract_type(s.slice.as_ref())?;
        Ok(Type::Optional(Box::new(inner)))
    }
}
