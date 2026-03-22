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
                                        let inferred = self
                                            .infer_type_from_expr(assign.value.as_ref())
                                            .unwrap_or(Type::Unknown);
                                        // DEPYLER-99MODE: Fields initialized to None are Optional
                                        // Python pattern: self.field = None → Option<T>
                                        if inferred == Type::None {
                                            Type::Optional(Box::new(Type::Unknown))
                                        } else {
                                            inferred
                                        }
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
