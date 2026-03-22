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

    fn is_type_variable(&self, name: &str) -> bool {
        name.len() == 1 && name.chars().next().is_some_and(|c| c.is_uppercase())
    }

    fn extract_generic_params_recursive(&mut self, expr: &ast::Expr) -> Vec<String> {
        let mut params = Vec::new();
        self.collect_type_vars_from_expr(expr, &mut params);
        params
    }

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

    fn extract_type_vars_from_annotation(&self, annotation: &ast::Expr) -> Vec<String> {
        let mut params = Vec::new();
        self.collect_type_vars_from_expr(annotation, &mut params);
        params
    }
