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
            if let Some(annotation_text) =
                self.annotation_extractor.extract_function_annotations(source, &func.name)
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
            if let Some(annotation_text) =
                self.annotation_extractor.extract_function_annotations(source, &func.name)
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
