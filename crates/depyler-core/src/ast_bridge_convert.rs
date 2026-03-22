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

        Ok(Some(Protocol { name, type_params, methods, is_runtime_checkable }))
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

        Ok(ProtocolMethod { name, params: params.into(), ret_type, is_optional, has_default })
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
