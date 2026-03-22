    fn try_convert_type_alias(&mut self, assign: &ast::StmtAssign) -> Result<Option<TypeAlias>> {
        // Look for patterns like: UserId = int or UserId = NewType('UserId', int)
        if assign.targets.len() != 1 {
            return Ok(None); // Skip multiple assignment targets
        }

        let target = match &assign.targets[0] {
            ast::Expr::Name(name) => name.id.as_str(),
            _ => return Ok(None), // Skip complex assignment targets
        };

        // Check if this looks like a type alias (simple assignment of a type)
        let (target_type, is_newtype) = match assign.value.as_ref() {
            // Simple alias: UserId = int
            ast::Expr::Name(n) => {
                let type_name = n.id.as_str();
                if self.is_type_name(type_name) {
                    (TypeExtractor::extract_simple_type(type_name)?, false)
                } else {
                    return Ok(None); // Not a type name
                }
            }
            // Generic alias: UserId = Optional[int]
            // DEPYLER-0503: Only treat subscripts with type base (Optional, List, etc.) as type aliases
            // Regular value subscripts like items[0] should return None (not a type alias)
            ast::Expr::Subscript(s) => {
                // Check if the base is a type name
                if let ast::Expr::Name(base_name) = s.value.as_ref() {
                    if self.is_type_name(base_name.id.as_str()) {
                        (TypeExtractor::extract_type(&assign.value)?, false)
                    } else {
                        return Ok(None); // Base is variable, not a type - not a type alias
                    }
                } else {
                    return Ok(None); // Complex base expression - not a type alias
                }
            }
            // NewType pattern: UserId = NewType('UserId', int)
            ast::Expr::Call(call) => {
                if let ast::Expr::Name(func_name) = call.func.as_ref() {
                    if func_name.id.as_str() == "NewType" && call.args.len() == 2 {
                        // Second argument should be the base type
                        let base_type = TypeExtractor::extract_type(&call.args[1])?;
                        (base_type, true)
                    } else {
                        return Ok(None); // Not a NewType call
                    }
                } else {
                    return Ok(None); // Complex function call
                }
            }
            _ => return Ok(None), // Not a type alias pattern
        };

        Ok(Some(TypeAlias { name: target.to_string(), target_type, is_newtype }))
    }

    fn is_type_name(&self, name: &str) -> bool {
        matches!(
            name,
            "int"
                | "float"
                | "str"
                | "bool"
                | "None"
                | "list"
                | "dict"
                | "tuple"
                | "set"
                | "frozenset"
                | "List"
                | "Dict"
                | "Tuple"
                | "Set"
                | "FrozenSet"
                | "Optional"
                | "Union"
                | "Callable"
                | "Any"
                | "TypeVar"
        )
    }
