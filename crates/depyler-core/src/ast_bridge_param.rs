fn resolve_optional_param_type(base_ty: Type, name: &str) -> Type {
    match base_ty {
        Type::Unknown => {
            let inferred_type = infer_optional_type_from_name(name);
            Type::Optional(Box::new(inferred_type))
        }
        Type::Optional(_) => {
            // Already Optional<T>, don't double-wrap
            base_ty
        }
        _ => {
            // Wrap annotated type in Optional
            Type::Optional(Box::new(base_ty))
        }
    }
}

fn infer_required_param_type(base_ty: Type, name: &str) -> Type {
    let param_lower = name.to_lowercase();
    let is_likely_string = param_lower.contains("file")
        || param_lower.contains("path")
        || param_lower.contains("name")
        || param_lower.contains("column")
        || param_lower == "value"
        || param_lower.contains("key");

    if is_likely_string && !param_lower.contains("config") && !param_lower.contains("data") {
        Type::String
    } else {
        base_ty
    }
}

fn convert_parameters(args: &ast::Arguments) -> Result<Vec<HirParam>> {
    use crate::ast_bridge::converters::ExprConverter;
    let mut params = Vec::new();

    // Calculate number of args without defaults
    let num_args = args.args.len();
    let defaults_vec: Vec<_> = args.defaults().collect();
    let num_defaults = defaults_vec.len();
    let first_default_idx = num_args.saturating_sub(num_defaults);

    for (i, arg) in args.args.iter().enumerate() {
        let name = arg.def.arg.to_string();

        // DEPYLER-0457: Extract base type from annotation
        let base_ty = if let Some(annotation) = &arg.def.annotation {
            TypeExtractor::extract_type(annotation)?
        } else {
            Type::Unknown
        };

        // Check if this parameter has a default value
        let default = if i >= first_default_idx {
            let default_idx = i - first_default_idx;
            if let Some(default_expr) = defaults_vec.get(default_idx) {
                Some(ExprConverter::convert((*default_expr).clone())?)
            } else {
                None
            }
        } else {
            None
        };

        // DEPYLER-0457/0744: Resolve final type based on default value and annotations
        let ty = if let Some(HirExpr::Literal(Literal::None)) = &default {
            resolve_optional_param_type(base_ty, &name)
        } else if default.is_none() && matches!(base_ty, Type::Unknown) {
            infer_required_param_type(base_ty, &name)
        } else {
            base_ty
        };

        params.push(HirParam { name, ty, default, is_vararg: false });
    }

    // DEPYLER-0477: Extract varargs parameter (*args)
    if let Some(vararg) = &args.vararg {
        let name = vararg.arg.to_string();

        // Start with List<String> as a reasonable default
        // DEPYLER-0477 Phase 2.2: Infer element type from usage (tracked)
        let ty = Type::List(Box::new(Type::String));

        params.push(HirParam {
            name,
            ty,
            default: None, // Varargs never have defaults
            is_vararg: true,
        });
    }

    // DEPYLER-0477 Phase 2.2: Extract kwargs (**kwargs) (tracked)
    // if let Some(kwarg) = &args.kwarg {
    //     // Will transpile to HashMap<String, serde_json::Value>
    // }

    Ok(params)
}
