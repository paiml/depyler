    fn convert_module(&mut self, module: ast::ModModule) -> Result<HirModule> {
        let mut functions = Vec::new();
        let mut imports = Vec::new();
        let mut type_aliases = Vec::new();
        let mut protocols = Vec::new();
        let mut classes = Vec::new();
        let mut constants = Vec::new();
        // DEPYLER-1216: Capture top-level statements for script-style Python
        let mut top_level_stmts = Vec::new();

        for stmt in module.body {
            match stmt {
                ast::Stmt::FunctionDef(f) => {
                    functions.push(self.convert_function(f, false)?);
                }
                ast::Stmt::Import(i) => {
                    imports.extend(convert_import(i)?);
                }
                ast::Stmt::ImportFrom(i) => {
                    imports.extend(convert_import_from(i)?);
                }
                ast::Stmt::AsyncFunctionDef(f) => {
                    functions.push(self.convert_async_function(f)?);
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
                    // Try to parse as type alias first
                    if let Some(type_alias) = self.try_convert_type_alias(&assign)? {
                        type_aliases.push(type_alias);
                    } else {
                        // Otherwise, treat as module-level constant
                        if let Some(constant) = self.try_convert_constant(&assign)? {
                            constants.push(constant);
                        }
                    }
                }
                ast::Stmt::AnnAssign(ann_assign) => {
                    // Try to parse annotated assignment as type alias first
                    if let Some(type_alias) = self.try_convert_annotated_type_alias(&ann_assign)? {
                        type_aliases.push(type_alias);
                    } else {
                        // Otherwise, treat as annotated module-level constant
                        if let Some(constant) = self.try_convert_annotated_constant(&ann_assign)? {
                            constants.push(constant);
                        }
                    }
                }
                // DEPYLER-1155: Handle `if __name__ == "__main__":` pattern
                // Convert to a `fn main()` function that contains the block body
                // BUT only if there's no `def main():` already defined
                ast::Stmt::If(if_stmt) => {
                    // DEPYLER-CONVERGE-MULTI: Skip `if TYPE_CHECKING:` blocks.
                    // These contain import-time-only type hints that have no
                    // runtime meaning and produce E0425 in generated Rust.
                    if is_type_checking_guard(&if_stmt) {
                        continue;
                    }
                    let has_main_function = functions.iter().any(|f| f.name == "main");
                    if !has_main_function {
                        if let Some(main_fn) = self.try_convert_if_main(&if_stmt)? {
                            functions.push(main_fn);
                        } else {
                            // DEPYLER-1216: Not an `if __name__ == "__main__":` pattern,
                            // so capture as a top-level statement for script-style Python
                            if let Ok(hir_stmt) = convert_stmt(ast::Stmt::If(if_stmt)) {
                                top_level_stmts.push(hir_stmt);
                            }
                        }
                    }
                }
                // DEPYLER-1216: Capture executable top-level statements for script-style Python
                // These will be wrapped into a synthetic main() if no explicit main exists
                ast::Stmt::Expr(expr) => {
                    if let Ok(hir_stmt) = convert_stmt(ast::Stmt::Expr(expr)) {
                        top_level_stmts.push(hir_stmt);
                    }
                }
                ast::Stmt::For(for_stmt) => {
                    if let Ok(hir_stmt) = convert_stmt(ast::Stmt::For(for_stmt)) {
                        top_level_stmts.push(hir_stmt);
                    }
                }
                ast::Stmt::While(while_stmt) => {
                    if let Ok(hir_stmt) = convert_stmt(ast::Stmt::While(while_stmt)) {
                        top_level_stmts.push(hir_stmt);
                    }
                }
                ast::Stmt::Try(try_stmt) => {
                    if let Ok(hir_stmt) = convert_stmt(ast::Stmt::Try(try_stmt)) {
                        top_level_stmts.push(hir_stmt);
                    }
                }
                ast::Stmt::With(with_stmt) => {
                    if let Ok(hir_stmt) = convert_stmt(ast::Stmt::With(with_stmt)) {
                        top_level_stmts.push(hir_stmt);
                    }
                }
                ast::Stmt::Return(ret_stmt) => {
                    // Top-level return (unusual but valid in scripts executed via exec())
                    if let Ok(hir_stmt) = convert_stmt(ast::Stmt::Return(ret_stmt)) {
                        top_level_stmts.push(hir_stmt);
                    }
                }
                _ => {
                    // Skip other statements (Pass, Break, Continue, etc.)
                }
            }
        }

        // DEPYLER-0359: Propagate can_fail through function calls
        // If a function calls another function that can fail, mark it as can_fail too
        propagate_can_fail_through_calls(&mut functions);

        Ok(HirModule {
            functions,
            imports,
            type_aliases,
            protocols,
            classes,
            constants,
            top_level_stmts,
        })
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

            Ok(Some(TypeAlias { name: target.to_string(), target_type, is_newtype }))
        } else {
            Ok(None) // No value assigned
        }
    }

    fn try_convert_constant(&mut self, assign: &ast::StmtAssign) -> Result<Option<HirConstant>> {
        // Only handle single assignment targets
        if assign.targets.len() != 1 {
            return Ok(None);
        }

        let name = match &assign.targets[0] {
            ast::Expr::Name(n) => n.id.to_string(),
            _ => return Ok(None), // Skip complex assignment targets
        };

        // Convert the value expression
        let value = convert_expr(*assign.value.clone())?;

        Ok(Some(HirConstant { name, value, type_annotation: None }))
    }

    fn try_convert_annotated_constant(
        &mut self,
        ann_assign: &ast::StmtAnnAssign,
    ) -> Result<Option<HirConstant>> {
        let name = match ann_assign.target.as_ref() {
            ast::Expr::Name(n) => n.id.to_string(),
            _ => return Ok(None), // Skip complex assignment targets
        };

        // Extract type annotation
        let type_annotation = Some(TypeExtractor::extract_type(&ann_assign.annotation)?);

        // DEPYLER-0500 Phase 2: Bind type annotation to TypeEnvironment
        if let Some(ref ty) = type_annotation {
            self.type_env.bind_var(&name, ty.clone());
        }

        // Get the value (annotated assignments at module level should have values)
        if let Some(value_expr) = &ann_assign.value {
            let value = convert_expr(*value_expr.clone())?;

            Ok(Some(HirConstant { name, value, type_annotation }))
        } else {
            Ok(None) // No value, skip it
        }
    }

fn is_type_checking_guard(if_stmt: &ast::StmtIf) -> bool {
    match if_stmt.test.as_ref() {
        ast::Expr::Name(name) => name.id.as_str() == "TYPE_CHECKING",
        _ => false,
    }
}

fn convert_import(import: ast::StmtImport) -> Result<Vec<Import>> {
    import
        .names
        .into_iter()
        .map(|alias| {
            let module = alias.name.to_string();
            // DEPYLER-1136: Capture the "as Y" alias for module-level imports
            let module_alias = alias.asname.map(|a| a.to_string());
            // For "import module" or "import module as alias", we import the whole module
            let items = vec![];
            Ok(Import { module, alias: module_alias, items })
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
                ImportItem::Aliased { name, alias: asname.to_string() }
            } else {
                ImportItem::Named(name)
            }
        })
        .collect();

    // DEPYLER-1136: `from X import Y` has no module-level alias
    Ok(vec![Import { module, alias: None, items }])
}
