//! Advanced expression conversion for ExprConverter
//!
//! Handles list/set/dict comprehensions, module constructors, lambda, fstring,
//! attribute access, and dynamic calls.

use crate::direct_rules::{make_ident, parse_target_pattern};
use crate::hir::*;
use anyhow::Result;
use syn::parse_quote;

use super::ExprConverter;

impl<'a> ExprConverter<'a> {
    pub(super) fn convert_list_comp(
        &self,
        element: &HirExpr,
        target: &str,
        iter: &HirExpr,
        condition: &Option<Box<HirExpr>>,
    ) -> Result<syn::Expr> {
        let target_pat = parse_target_pattern(target);
        let iter_expr = self.convert(iter)?;

        // DEPYLER-1100: Infer element type and create converter with loop variable typed
        let element_type = self.infer_iterable_element_type(iter);
        let inner_converter = if let Some(elem_type) = element_type {
            self.with_additional_param(target.to_string(), elem_type)
        } else {
            Self {
                type_mapper: self.type_mapper,
                is_classmethod: self.is_classmethod,
                vararg_functions: self.vararg_functions,
                param_types: self.param_types.clone(),
                class_field_types: self.class_field_types.clone(),
            }
        };

        let element_expr = inner_converter.convert(element)?;

        if let Some(cond) = condition {
            // With condition: iter().filter().map().collect()
            // DEPYLER-0833: Use |x| pattern (not |&x|) to avoid E0507 on non-Copy types
            // DEPYLER-1000: Clone loop variable inside filter to fix E0308 reference mismatch
            let cond_expr = inner_converter.convert(cond)?;
            Ok(parse_quote! {
                #iter_expr
                    .into_iter()
                    .filter(|#target_pat| { let #target_pat = #target_pat.clone(); #cond_expr })
                    .map(|#target_pat| #element_expr)
                    .collect::<Vec<_>>()
            })
        } else {
            // Without condition: iter().map().collect()
            Ok(parse_quote! {
                #iter_expr
                    .into_iter()
                    .map(|#target_pat| #element_expr)
                    .collect::<Vec<_>>()
            })
        }
    }

    pub(super) fn convert_set_comp(
        &self,
        element: &HirExpr,
        target: &str,
        iter: &HirExpr,
        condition: &Option<Box<HirExpr>>,
    ) -> Result<syn::Expr> {
        let target_pat = parse_target_pattern(target);
        let iter_expr = self.convert(iter)?;

        // DEPYLER-1100: Infer element type and create converter with loop variable typed
        let element_type = self.infer_iterable_element_type(iter);
        let inner_converter = if let Some(elem_type) = element_type {
            self.with_additional_param(target.to_string(), elem_type)
        } else {
            Self {
                type_mapper: self.type_mapper,
                is_classmethod: self.is_classmethod,
                vararg_functions: self.vararg_functions,
                param_types: self.param_types.clone(),
                class_field_types: self.class_field_types.clone(),
            }
        };

        let element_expr = inner_converter.convert(element)?;

        // DEPYLER-0831: Use fully-qualified path for E0412 resolution
        if let Some(cond) = condition {
            // With condition: iter().filter().map().collect()
            // DEPYLER-0833: Use |x| pattern (not |&x|) to avoid E0507 on non-Copy types
            // DEPYLER-1000: Clone loop variable inside filter to fix E0308 reference mismatch
            let cond_expr = inner_converter.convert(cond)?;
            Ok(parse_quote! {
                #iter_expr
                    .into_iter()
                    .filter(|#target_pat| { let #target_pat = #target_pat.clone(); #cond_expr })
                    .map(|#target_pat| #element_expr)
                    .collect::<std::collections::HashSet<_>>()
            })
        } else {
            // Without condition: iter().map().collect()
            Ok(parse_quote! {
                #iter_expr
                    .into_iter()
                    .map(|#target_pat| #element_expr)
                    .collect::<std::collections::HashSet<_>>()
            })
        }
    }

    /// DEPYLER-0610: Convert Python stdlib module constructor calls to Rust
    /// threading.Semaphore(n) → std::sync::Mutex::new(n)
    /// queue.Queue() → std::collections::VecDeque::new()
    pub(super) fn convert_module_constructor(
        &self,
        module: &str,
        constructor: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        let arg_exprs: Vec<syn::Expr> =
            args.iter().map(|arg| self.convert(arg)).collect::<Result<Vec<_>>>()?;

        let nasa_mode = self.type_mapper.nasa_mode;
        let result = match module {
            "threading" => convert_threading_constructor(constructor, &arg_exprs),
            "queue" => convert_queue_constructor(constructor),
            "datetime" => convert_datetime_constructor(constructor, &arg_exprs, nasa_mode),
            "collections" => convert_collections_constructor(constructor, &arg_exprs),
            "asyncio" => convert_asyncio_constructor(constructor, &arg_exprs, nasa_mode),
            "json" => convert_json_constructor(constructor, &arg_exprs, nasa_mode),
            "os" => convert_os_constructor(constructor, &arg_exprs),
            "re" => None,
            "fnmatch" => convert_fnmatch_constructor(constructor, &arg_exprs),
            _ => None,
        };

        Ok(result)
    }

    pub(super) fn convert_dict_comp(
        &self,
        key: &HirExpr,
        value: &HirExpr,
        target: &str,
        iter: &HirExpr,
        condition: &Option<Box<HirExpr>>,
    ) -> Result<syn::Expr> {
        let target_pat = parse_target_pattern(target);
        let iter_expr = self.convert(iter)?;
        let key_expr = self.convert(key)?;
        let value_expr = self.convert(value)?;

        if let Some(cond) = condition {
            // With condition: iter().filter().map().collect()
            // DEPYLER-0833: Use |x| pattern (not |&x|) to avoid E0507 on non-Copy types
            let cond_expr = self.convert(cond)?;
            Ok(parse_quote! {
                #iter_expr
                    .into_iter()
                    .filter(|#target_pat| #cond_expr)
                    .map(|#target_pat| (#key_expr, #value_expr))
                    .collect::<std::collections::HashMap<_, _>>()
            })
        } else {
            // Without condition: iter().map().collect()
            Ok(parse_quote! {
                #iter_expr
                    .into_iter()
                    .map(|#target_pat| (#key_expr, #value_expr))
                    .collect::<std::collections::HashMap<_, _>>()
            })
        }
    }

    pub(super) fn convert_lambda(&self, params: &[String], body: &HirExpr) -> Result<syn::Expr> {
        // Convert parameters to pattern identifiers
        // NOTE (DEPYLER-1061): Lambda parameters are intentionally NOT typed with DepylerValue.
        // Adding DepylerValue type annotations breaks call sites that pass raw literals.
        // E0282 "type annotations needed" errors occur for lambdas stored in variables
        // that use iterator methods like .iter(). This is a known limitation requiring
        // bidirectional type inference (from usage context to lambda definition).
        let param_pats: Vec<syn::Pat> = params
            .iter()
            .map(|p| {
                let ident = make_ident(p);
                parse_quote! { #ident }
            })
            .collect();

        // Convert body expression
        let body_expr = self.convert(body)?;

        // Generate closure
        // DEPYLER-0837: Use `move` closures to match Python's closure semantics
        // Python closures capture variables by reference but extend their lifetime
        // Rust requires `move` when returning closures that capture local variables
        if params.is_empty() {
            // No parameters
            Ok(parse_quote! { move || #body_expr })
        } else if params.len() == 1 {
            // Single parameter
            let param = &param_pats[0];
            Ok(parse_quote! { move |#param| #body_expr })
        } else {
            // Multiple parameters
            Ok(parse_quote! { move |#(#param_pats),*| #body_expr })
        }
    }

    pub(super) fn convert_await(&self, value: &HirExpr) -> Result<syn::Expr> {
        let value_expr = self.convert(value)?;
        Ok(parse_quote! { #value_expr.await })
    }

    /// DEPYLER-0513: Convert F-string to format!() macro
    ///
    /// Handles Python f-strings like `f"Hello {name}"` → `format!("Hello {}", name)`
    ///
    /// Strategy: Build format template and collect args, then generate format!() call.
    /// Simplified version for direct_rules - basic formatting only.
    pub(super) fn convert_fstring(&self, parts: &[crate::hir::FStringPart]) -> Result<syn::Expr> {
        use crate::hir::FStringPart;

        // Handle empty f-strings
        if parts.is_empty() {
            return Ok(parse_quote! { "".to_string() });
        }

        // Check if it's just a plain string (no expressions)
        let has_expressions = parts.iter().any(|p| matches!(p, FStringPart::Expr(_)));

        if !has_expressions {
            // Just literal parts - concatenate them
            let mut result = String::new();
            for part in parts {
                if let FStringPart::Literal(s) = part {
                    result.push_str(s);
                }
            }
            return Ok(parse_quote! { #result.to_string() });
        }

        // Build format string template and collect arguments
        let mut template = String::new();
        let mut args = Vec::new();

        for part in parts {
            match part {
                FStringPart::Literal(s) => {
                    template.push_str(s);
                }
                FStringPart::Expr(expr) => {
                    // Add {} placeholder to template
                    template.push_str("{}");
                    // Convert expression to Rust and add to args
                    let arg_expr = self.convert(expr)?;
                    args.push(arg_expr);
                }
            }
        }

        // Generate format!() macro call
        Ok(parse_quote! { format!(#template, #(#args),*) })
    }

    pub(super) fn convert_attribute(&self, value: &HirExpr, attr: &str) -> Result<syn::Expr> {
        // Handle classmethod cls.ATTR → Self::ATTR
        if let HirExpr::Var(var_name) = value {
            if var_name == "cls" && self.is_classmethod {
                let attr_ident = make_ident(attr);
                return Ok(parse_quote! { Self::#attr_ident });
            }

            // DEPYLER-1069: Handle datetime class constants (min, max, resolution)
            // date.min → DepylerDate::new(1, 1, 1)
            // datetime.min → DepylerDateTime::new(1, 1, 1, 0, 0, 0, 0)
            // time.min → (0, 0, 0, 0)
            let nasa_mode = self.type_mapper.nasa_mode;
            if (var_name == "date" || var_name == "datetime" || var_name == "time")
                && (attr == "min" || attr == "max")
            {
                if var_name == "date" {
                    if nasa_mode {
                        return Ok(if attr == "min" {
                            parse_quote! { DepylerDate::new(1, 1, 1) }
                        } else {
                            parse_quote! { DepylerDate::new(9999, 12, 31) }
                        });
                    } else {
                        return Ok(if attr == "min" {
                            parse_quote! { chrono::NaiveDate::MIN }
                        } else {
                            parse_quote! { chrono::NaiveDate::MAX }
                        });
                    }
                } else if var_name == "datetime" {
                    if nasa_mode {
                        return Ok(if attr == "min" {
                            parse_quote! { DepylerDateTime::new(1, 1, 1, 0, 0, 0, 0) }
                        } else {
                            parse_quote! { DepylerDateTime::new(9999, 12, 31, 23, 59, 59, 999999) }
                        });
                    } else {
                        return Ok(if attr == "min" {
                            parse_quote! { chrono::NaiveDateTime::MIN }
                        } else {
                            parse_quote! { chrono::NaiveDateTime::MAX }
                        });
                    }
                } else if var_name == "time" {
                    if nasa_mode {
                        return Ok(if attr == "min" {
                            parse_quote! { (0u32, 0u32, 0u32, 0u32) }
                        } else {
                            parse_quote! { (23u32, 59u32, 59u32, 999999u32) }
                        });
                    } else {
                        return Ok(if attr == "min" {
                            parse_quote! { chrono::NaiveTime::MIN }
                        } else {
                            parse_quote! { chrono::NaiveTime::from_hms_micro_opt(23, 59, 59, 999999).expect("invalid time") }
                        });
                    }
                }
            }

            // DEPYLER-1097: Handle sys module attribute access
            // sys.argv → std::env::args().collect::<Vec<String>>()
            // sys.version → Rust version string stub
            // sys.platform → std::env::consts::OS
            if var_name == "sys" {
                match attr {
                    "argv" => {
                        return Ok(parse_quote! { std::env::args().collect::<Vec<String>>() });
                    }
                    "version" | "version_info" => {
                        return Ok(parse_quote! { env!("CARGO_PKG_VERSION").to_string() });
                    }
                    "platform" => {
                        return Ok(parse_quote! { std::env::consts::OS.to_string() });
                    }
                    "path" => {
                        return Ok(parse_quote! { Vec::<String>::new() });
                    }
                    "stdin" => {
                        return Ok(parse_quote! { std::io::stdin() });
                    }
                    "stdout" => {
                        return Ok(parse_quote! { std::io::stdout() });
                    }
                    "stderr" => {
                        return Ok(parse_quote! { std::io::stderr() });
                    }
                    "maxsize" => {
                        return Ok(parse_quote! { i64::MAX });
                    }
                    _ => {} // Fall through for other sys attributes
                }
            }

            // DEPYLER-0616: Detect enum/type constant access patterns
            // TypeName.CONSTANT → TypeName::CONSTANT
            // Heuristic: If name starts with uppercase and attr is ALL_CAPS, it's likely an enum constant
            let first_char = var_name.chars().next().unwrap_or('a');
            let is_type_name = first_char.is_uppercase();
            // DEPYLER-CONVERGE-MULTI: Allow digits in constant names (e.g. FP8_E4M3)
            let is_constant =
                attr.chars().all(|c| c.is_uppercase() || c == '_' || c.is_ascii_digit());

            if is_type_name && is_constant {
                let type_ident = make_ident(var_name);
                let attr_ident = make_ident(attr);
                return Ok(parse_quote! { #type_ident::#attr_ident });
            }
        }

        let value_expr = self.convert(value)?;
        // DEPYLER-0596: Use make_ident to handle keywords like "match"
        let attr_ident = make_ident(attr);

        // DEPYLER-0737: Check if this attribute is a @property method
        // In Python, @property allows method access without (), but in Rust we need ()
        let is_prop_method = crate::direct_rules::is_property_method(attr);

        if is_prop_method {
            // Property access needs method call syntax: obj.prop()
            Ok(parse_quote! { #value_expr.#attr_ident() })
        } else {
            // Regular field access: obj.field
            // DEPYLER-0740: For self.field accesses, add .clone() to avoid E0507 moves
            // Python semantics don't consume values on field access, so cloning is safe
            if let HirExpr::Var(var_name) = value {
                if var_name == "self" {
                    return Ok(parse_quote! { #value_expr.#attr_ident.clone() });
                }
            }
            Ok(parse_quote! { #value_expr.#attr_ident })
        }
    }

    /// DEPYLER-0188: Convert dynamic/subscript function call
    /// Pattern: `handlers[name](args)` → `(handlers[&name])(args)` or `handlers.get(&name).unwrap()(args)`
    ///
    /// In Rust, calling a value from a HashMap requires:
    /// 1. Index access with reference: `handlers[&name]`
    /// 2. Parentheses to call the result: `(handlers[&name])(args)`
    pub(super) fn convert_dynamic_call(
        &self,
        callee: &HirExpr,
        args: &[HirExpr],
    ) -> Result<syn::Expr> {
        // Convert the callee expression (e.g., handlers[name])
        let callee_expr = self.convert(callee)?;

        // Convert arguments
        let arg_exprs: Vec<syn::Expr> =
            args.iter().map(|arg| self.convert(arg)).collect::<Result<Vec<_>>>()?;

        // Generate: (callee)(args)
        // Wrap callee in parentheses to ensure correct parsing
        if arg_exprs.is_empty() {
            Ok(parse_quote! { (#callee_expr)() })
        } else {
            Ok(parse_quote! { (#callee_expr)(#(#arg_exprs),*) })
        }
    }
}

fn convert_threading_constructor(constructor: &str, arg_exprs: &[syn::Expr]) -> Option<syn::Expr> {
    match constructor {
        "Semaphore" | "BoundedSemaphore" => {
            if let Some(arg) = arg_exprs.first() {
                Some(parse_quote! { std::sync::Mutex::new(#arg) })
            } else {
                Some(parse_quote! { std::sync::Mutex::new(0) })
            }
        }
        "Lock" | "RLock" => Some(parse_quote! { std::sync::Mutex::new(()) }),
        "Event" => Some(parse_quote! { std::sync::Condvar::new() }),
        "Thread" => Some(parse_quote! { std::thread::spawn(|| {}) }),
        _ => None,
    }
}

fn convert_queue_constructor(constructor: &str) -> Option<syn::Expr> {
    match constructor {
        "Queue" | "LifoQueue" | "PriorityQueue" => {
            Some(parse_quote! { std::collections::VecDeque::new() })
        }
        _ => None,
    }
}

fn convert_datetime_constructor(
    constructor: &str,
    arg_exprs: &[syn::Expr],
    nasa_mode: bool,
) -> Option<syn::Expr> {
    match constructor {
        "datetime" => convert_datetime_datetime(nasa_mode),
        "date" => convert_datetime_date(nasa_mode),
        "time" => convert_datetime_time(nasa_mode),
        "timedelta" => convert_datetime_timedelta(arg_exprs, nasa_mode),
        "now" => convert_datetime_now(nasa_mode),
        _ => None,
    }
}

fn convert_datetime_datetime(nasa_mode: bool) -> Option<syn::Expr> {
    if nasa_mode {
        Some(parse_quote! { std::time::SystemTime::now() })
    } else {
        Some(parse_quote! { chrono::Utc::now() })
    }
}

fn convert_datetime_date(nasa_mode: bool) -> Option<syn::Expr> {
    if nasa_mode {
        Some(parse_quote! { std::time::SystemTime::now() })
    } else {
        Some(parse_quote! { chrono::Utc::now().date_naive() })
    }
}

fn convert_datetime_time(nasa_mode: bool) -> Option<syn::Expr> {
    if nasa_mode {
        Some(parse_quote! { std::time::SystemTime::now() })
    } else {
        Some(parse_quote! { chrono::Utc::now().time() })
    }
}

fn convert_datetime_timedelta(arg_exprs: &[syn::Expr], nasa_mode: bool) -> Option<syn::Expr> {
    if nasa_mode {
        if let Some(arg) = arg_exprs.first() {
            Some(parse_quote! { std::time::Duration::from_secs((#arg as u64) * 86400) })
        } else {
            Some(parse_quote! { std::time::Duration::from_secs(0) })
        }
    } else if let Some(arg) = arg_exprs.first() {
        Some(parse_quote! { chrono::Duration::days(#arg) })
    } else {
        Some(parse_quote! { chrono::Duration::zero() })
    }
}

fn convert_datetime_now(nasa_mode: bool) -> Option<syn::Expr> {
    if nasa_mode {
        Some(parse_quote! { std::time::SystemTime::now() })
    } else {
        Some(parse_quote! { chrono::Utc::now() })
    }
}

fn convert_collections_constructor(
    constructor: &str,
    arg_exprs: &[syn::Expr],
) -> Option<syn::Expr> {
    match constructor {
        "deque" => {
            if arg_exprs.is_empty() {
                Some(parse_quote! { std::collections::VecDeque::new() })
            } else {
                let arg = &arg_exprs[0];
                Some(parse_quote! { std::collections::VecDeque::from(#arg) })
            }
        }
        "Counter" => {
            if arg_exprs.is_empty() {
                Some(parse_quote! { std::collections::HashMap::new() })
            } else {
                let arg = &arg_exprs[0];
                Some(parse_quote! {
                    #arg.into_iter().fold(std::collections::HashMap::new(), |mut acc, item| {
                        *acc.entry(item).or_insert(0) += 1;
                        acc
                    })
                })
            }
        }
        "OrderedDict" => {
            if arg_exprs.is_empty() {
                Some(parse_quote! { std::collections::HashMap::new() })
            } else {
                let arg = &arg_exprs[0];
                Some(parse_quote! { #arg.into_iter().collect::<std::collections::HashMap<_, _>>() })
            }
        }
        "defaultdict" => Some(parse_quote! { std::collections::HashMap::new() }),
        _ => None,
    }
}

fn convert_asyncio_constructor(
    constructor: &str,
    arg_exprs: &[syn::Expr],
    nasa_mode: bool,
) -> Option<syn::Expr> {
    match constructor {
        "Event" => convert_asyncio_event(nasa_mode),
        "Lock" => convert_asyncio_lock(nasa_mode),
        "Semaphore" => convert_asyncio_semaphore(arg_exprs, nasa_mode),
        "Queue" => convert_asyncio_queue(nasa_mode),
        "sleep" => convert_asyncio_sleep(arg_exprs, nasa_mode),
        "run" => convert_asyncio_run(arg_exprs, nasa_mode),
        _ => None,
    }
}

fn convert_asyncio_event(nasa_mode: bool) -> Option<syn::Expr> {
    if nasa_mode {
        Some(parse_quote! { std::sync::Condvar::new() })
    } else {
        Some(parse_quote! { tokio::sync::Notify::new() })
    }
}

fn convert_asyncio_lock(nasa_mode: bool) -> Option<syn::Expr> {
    if nasa_mode {
        Some(parse_quote! { std::sync::Mutex::new(()) })
    } else {
        Some(parse_quote! { tokio::sync::Mutex::new(()) })
    }
}

fn convert_asyncio_semaphore(arg_exprs: &[syn::Expr], nasa_mode: bool) -> Option<syn::Expr> {
    if nasa_mode {
        Some(parse_quote! { () })
    } else if let Some(arg) = arg_exprs.first() {
        Some(parse_quote! { tokio::sync::Semaphore::new(#arg as usize) })
    } else {
        Some(parse_quote! { tokio::sync::Semaphore::new(1) })
    }
}

fn convert_asyncio_queue(nasa_mode: bool) -> Option<syn::Expr> {
    if nasa_mode {
        Some(parse_quote! { std::sync::mpsc::channel().1 })
    } else {
        Some(parse_quote! { tokio::sync::mpsc::channel(100).1 })
    }
}

fn convert_asyncio_sleep(arg_exprs: &[syn::Expr], nasa_mode: bool) -> Option<syn::Expr> {
    if nasa_mode {
        if let Some(arg) = arg_exprs.first() {
            Some(parse_quote! {
                std::thread::sleep(std::time::Duration::from_secs_f64(#arg as f64))
            })
        } else {
            Some(parse_quote! {
                std::thread::sleep(std::time::Duration::from_secs(0))
            })
        }
    } else if let Some(arg) = arg_exprs.first() {
        Some(parse_quote! {
            tokio::time::sleep(std::time::Duration::from_secs_f64(#arg as f64))
        })
    } else {
        Some(parse_quote! {
            tokio::time::sleep(std::time::Duration::from_secs(0))
        })
    }
}

fn convert_asyncio_run(arg_exprs: &[syn::Expr], nasa_mode: bool) -> Option<syn::Expr> {
    if nasa_mode {
        arg_exprs.first().map(|arg| parse_quote! { #arg })
    } else {
        arg_exprs.first().map(|arg| {
            parse_quote! {
                tokio::runtime::Runtime::new().expect("tokio runtime failed").block_on(#arg)
            }
        })
    }
}

fn convert_json_constructor(
    constructor: &str,
    arg_exprs: &[syn::Expr],
    nasa_mode: bool,
) -> Option<syn::Expr> {
    match constructor {
        "loads" | "load" => {
            if nasa_mode {
                arg_exprs.first().map(
                    |_| parse_quote! { std::collections::HashMap::<String, DepylerValue>::new() },
                )
            } else {
                arg_exprs.first().map(|arg| parse_quote! { serde_json::from_str::<serde_json::Value>(&#arg).expect("JSON parse failed") })
            }
        }
        "dumps" | "dump" => {
            if nasa_mode {
                arg_exprs.first().map(|arg| parse_quote! { format!("{:?}", #arg) })
            } else {
                arg_exprs
                    .first()
                    .map(|arg| parse_quote! { serde_json::to_string(&#arg).expect("JSON serialize failed") })
            }
        }
        _ => None,
    }
}

fn convert_os_constructor(constructor: &str, arg_exprs: &[syn::Expr]) -> Option<syn::Expr> {
    match constructor {
        "getcwd" => Some(parse_quote! { std::env::current_dir()?.to_string_lossy().to_string() }),
        "getenv" => arg_exprs.first().map(|arg| parse_quote! { std::env::var(#arg).ok() }),
        "listdir" => {
            if let Some(arg) = arg_exprs.first() {
                Some(
                    parse_quote! { std::fs::read_dir(#arg)?.map(|e| e.expect("dir entry error").file_name().to_string_lossy().to_string()).collect::<Vec<_>>() },
                )
            } else {
                Some(
                    parse_quote! { std::fs::read_dir(".")?.map(|e| e.expect("dir entry error").file_name().to_string_lossy().to_string()).collect::<Vec<_>>() },
                )
            }
        }
        _ => None,
    }
}

fn convert_fnmatch_constructor(constructor: &str, arg_exprs: &[syn::Expr]) -> Option<syn::Expr> {
    match constructor {
        "fnmatch" => {
            if arg_exprs.len() >= 2 {
                let name = &arg_exprs[0];
                let pattern = &arg_exprs[1];
                Some(parse_quote! { #name.contains(&#pattern) })
            } else {
                Some(parse_quote! { false })
            }
        }
        _ => None,
    }
}
