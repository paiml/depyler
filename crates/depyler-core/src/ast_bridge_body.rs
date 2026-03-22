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
    let filtered_body: Vec<HirStmt> =
        body.into_iter().skip(start_index).filter_map(|stmt| convert_stmt(stmt).ok()).collect();

    Ok((docstring, filtered_body))
}
