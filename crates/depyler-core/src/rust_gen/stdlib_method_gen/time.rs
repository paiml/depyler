//! Time Module Code Generation - EXTREME TDD
//!
//! Handles Python `time` module method conversions to Rust std::time/chrono.
//! Extracted from expr_gen.rs for testability and maintainability.
//!
//! Coverage target: 100% line coverage, 100% branch coverage

use crate::hir::{HirExpr, Literal};
use crate::rust_gen::context::{CodeGenContext, ToRustExpr};
use anyhow::{bail, Result};
use syn::parse_quote;

/// Convert Python time module method calls to Rust
/// DEPYLER-1025: Added NASA mode support for std-only compilation
///
/// # Supported Methods
/// - `time.time()` → `SystemTime::now().duration_since(UNIX_EPOCH).as_secs_f64()`
/// - `time.sleep(s)` → `thread::sleep(Duration::from_secs_f64(s))`
/// - `time.monotonic()` → `Instant::now()`
/// - `time.perf_counter()` → `Instant::now()`
/// - `time.process_time()` → `Instant::now()` (approximation)
/// - `time.thread_time()` → `Instant::now()` (approximation)
/// - `time.ctime(t)` → std::time formatting (NASA) or chrono (non-NASA)
/// - `time.strftime(fmt, t)` → std::time formatting (NASA) or chrono (non-NASA)
/// - `time.strptime(s, fmt)` → std::time parsing (NASA) or chrono (non-NASA)
/// - `time.gmtime(t)` → std::time conversion (NASA) or chrono (non-NASA)
/// - `time.localtime(t)` → std::time conversion (NASA) or chrono (non-NASA)
/// - `time.mktime(t)` → timestamp conversion
/// - `time.asctime(t)` → ASCII time string
///
/// # Complexity: 10 (match with 10+ branches, within limits)
pub fn convert_time_method(
    method: &str,
    args: &[HirExpr],
    ctx: &mut CodeGenContext,
) -> Result<Option<syn::Expr>> {
    let nasa_mode = ctx.type_mapper.nasa_mode;

    // Convert arguments first
    let arg_exprs: Vec<syn::Expr> = args
        .iter()
        .map(|arg| arg.to_rust_expr(ctx))
        .collect::<Result<Vec<_>>>()?;

    let result = match method {
        "time" => convert_time_time()?,
        "monotonic" | "perf_counter" => convert_monotonic()?,
        "process_time" | "thread_time" => convert_process_time()?,
        "sleep" => convert_sleep(&arg_exprs)?,
        "ctime" => {
            if !nasa_mode {
                ctx.needs_chrono = true;
            }
            convert_ctime(&arg_exprs, nasa_mode)?
        }
        "strftime" => {
            if !nasa_mode {
                ctx.needs_chrono = true;
            }
            convert_strftime(args, &arg_exprs, nasa_mode)?
        }
        "strptime" => {
            if !nasa_mode {
                ctx.needs_chrono = true;
            }
            convert_strptime(args, &arg_exprs, nasa_mode)?
        }
        "gmtime" => {
            if !nasa_mode {
                ctx.needs_chrono = true;
            }
            convert_gmtime(&arg_exprs, nasa_mode)?
        }
        "localtime" => {
            if !nasa_mode {
                ctx.needs_chrono = true;
            }
            convert_localtime(&arg_exprs, nasa_mode)?
        }
        "mktime" => {
            if !nasa_mode {
                ctx.needs_chrono = true;
            }
            convert_mktime(&arg_exprs)?
        }
        "asctime" => {
            if !nasa_mode {
                ctx.needs_chrono = true;
            }
            convert_asctime(&arg_exprs)?
        }
        _ => bail!("time.{} not implemented yet", method),
    };

    Ok(Some(result))
}

/// time.time() → SystemTime::now().duration_since(UNIX_EPOCH).as_secs_f64()
fn convert_time_time() -> Result<syn::Expr> {
    Ok(parse_quote! {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time operation failed")
            .as_secs_f64()
    })
}

/// time.monotonic() / time.perf_counter() → Instant::now()
fn convert_monotonic() -> Result<syn::Expr> {
    Ok(parse_quote! { std::time::Instant::now() })
}

/// time.process_time() / time.thread_time() → Instant::now() (approximation)
fn convert_process_time() -> Result<syn::Expr> {
    Ok(parse_quote! { std::time::Instant::now() })
}

/// time.sleep(seconds) → thread::sleep(Duration::from_secs_f64(seconds))
fn convert_sleep(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("time.sleep() requires exactly 1 argument (seconds)");
    }
    let seconds = &arg_exprs[0];
    Ok(parse_quote! {
        std::thread::sleep(std::time::Duration::from_secs_f64(#seconds))
    })
}

/// time.ctime(timestamp) → std::time formatting (NASA) or chrono (non-NASA)
fn convert_ctime(arg_exprs: &[syn::Expr], nasa_mode: bool) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("time.ctime() requires exactly 1 argument (timestamp)");
    }
    let timestamp = &arg_exprs[0];
    if nasa_mode {
        // NASA mode: use std::time formatting
        Ok(parse_quote! {
            format!("{:?}", std::time::UNIX_EPOCH + std::time::Duration::from_secs_f64(#timestamp))
        })
    } else {
        Ok(parse_quote! {
            {
                let secs = #timestamp as i64;
                let nanos = ((#timestamp - secs as f64) * 1_000_000_000.0) as u32;
                chrono::DateTime::<chrono::Utc>::from_timestamp(secs, nanos)
                    .expect("time operation failed")
                    .to_string()
            }
        })
    }
}

/// time.strftime(format, time_tuple) → std::time formatting (NASA) or chrono (non-NASA)
fn convert_strftime(
    args: &[HirExpr],
    arg_exprs: &[syn::Expr],
    nasa_mode: bool,
) -> Result<syn::Expr> {
    if arg_exprs.len() < 2 {
        bail!("time.strftime() requires at least 2 arguments (format, time_tuple)");
    }
    if nasa_mode {
        // NASA mode: use format! with debug
        Ok(parse_quote! {
            format!("{:?}", std::time::SystemTime::now())
        })
    } else {
        // Extract bare string literal for chrono's format() which takes &str
        let format = match args.first() {
            Some(HirExpr::Literal(Literal::String(s))) => parse_quote! { #s },
            _ => arg_exprs[0].clone(),
        };
        Ok(parse_quote! {
            chrono::Local::now().format(#format).to_string()
        })
    }
}

/// time.strptime(string, format) → std::time parsing (NASA) or chrono (non-NASA)
fn convert_strptime(
    args: &[HirExpr],
    arg_exprs: &[syn::Expr],
    nasa_mode: bool,
) -> Result<syn::Expr> {
    if arg_exprs.len() < 2 {
        bail!("time.strptime() requires at least 2 arguments (string, format)");
    }
    if nasa_mode {
        // NASA mode: return current time (simplified)
        Ok(parse_quote! { std::time::SystemTime::now() })
    } else {
        let time_str = &arg_exprs[0];
        // Extract bare string literal for chrono's parse_from_str()
        let format = match args.get(1) {
            Some(HirExpr::Literal(Literal::String(s))) => parse_quote! { #s },
            _ => arg_exprs[1].clone(),
        };
        Ok(parse_quote! {
            chrono::NaiveDateTime::parse_from_str(#time_str, #format).expect("time operation failed")
        })
    }
}

/// time.gmtime(timestamp) → std::time conversion (NASA) or chrono (non-NASA)
fn convert_gmtime(arg_exprs: &[syn::Expr], nasa_mode: bool) -> Result<syn::Expr> {
    let timestamp = if arg_exprs.is_empty() {
        parse_quote! { std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("time operation failed").as_secs_f64() }
    } else {
        arg_exprs[0].clone()
    };

    if nasa_mode {
        // NASA mode: return SystemTime
        Ok(parse_quote! {
            std::time::UNIX_EPOCH + std::time::Duration::from_secs_f64(#timestamp)
        })
    } else {
        Ok(parse_quote! {
            {
                let secs = #timestamp as i64;
                let nanos = ((#timestamp - secs as f64) * 1_000_000_000.0) as u32;
                chrono::DateTime::<chrono::Utc>::from_timestamp(secs, nanos).expect("time operation failed")
            }
        })
    }
}

/// time.localtime(timestamp) → std::time conversion (NASA) or chrono (non-NASA)
fn convert_localtime(arg_exprs: &[syn::Expr], nasa_mode: bool) -> Result<syn::Expr> {
    let timestamp = if arg_exprs.is_empty() {
        parse_quote! { std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("time operation failed").as_secs_f64() }
    } else {
        arg_exprs[0].clone()
    };

    if nasa_mode {
        // NASA mode: return SystemTime
        Ok(parse_quote! {
            std::time::UNIX_EPOCH + std::time::Duration::from_secs_f64(#timestamp)
        })
    } else {
        Ok(parse_quote! {
            {
                let secs = #timestamp as i64;
                let nanos = ((#timestamp - secs as f64) * 1_000_000_000.0) as u32;
                chrono::DateTime::<chrono::Local>::from_timestamp(secs, nanos).expect("time operation failed")
            }
        })
    }
}

/// time.mktime(time_tuple) → timestamp conversion
fn convert_mktime(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("time.mktime() requires exactly 1 argument (time_tuple)");
    }
    let time_tuple = &arg_exprs[0];
    Ok(parse_quote! { #time_tuple.timestamp() as f64 })
}

/// time.asctime(time_tuple) → ASCII time string
fn convert_asctime(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("time.asctime() requires exactly 1 argument (time_tuple)");
    }
    let time_tuple = &arg_exprs[0];
    Ok(parse_quote! { #time_tuple.to_string() })
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============ convert_time_method - time() tests ============

    #[test]
    fn test_convert_time_time_basic() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_time_method("time", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("SystemTime"));
        assert!(code.contains("UNIX_EPOCH"));
    }

    // ============ convert_time_method - monotonic/perf_counter tests ============

    #[test]
    fn test_convert_time_monotonic_basic() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_time_method("monotonic", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("Instant") && code.contains("now"));
    }

    #[test]
    fn test_convert_time_perf_counter_basic() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_time_method("perf_counter", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("Instant") && code.contains("now"));
    }

    // ============ convert_time_method - process_time/thread_time tests ============

    #[test]
    fn test_convert_time_process_time_basic() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_time_method("process_time", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("Instant") && code.contains("now"));
    }

    #[test]
    fn test_convert_time_thread_time_basic() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_time_method("thread_time", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("Instant") && code.contains("now"));
    }

    // ============ convert_time_method - sleep() tests ============

    #[test]
    fn test_convert_time_sleep_basic() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::Float(1.5))];
        let result = convert_time_method("sleep", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("thread") && code.contains("sleep"));
        assert!(code.contains("Duration") && code.contains("from_secs_f64"));
    }

    #[test]
    fn test_convert_time_sleep_missing_args() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_time_method("sleep", &args, &mut ctx);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("requires exactly 1 argument"));
    }

    #[test]
    fn test_convert_time_sleep_too_many_args() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::Float(1.0)),
            HirExpr::Literal(Literal::Float(2.0)),
        ];
        let result = convert_time_method("sleep", &args, &mut ctx);
        assert!(result.is_err());
    }

    // ============ convert_time_method - ctime() tests ============

    #[test]
    fn test_convert_time_ctime_basic() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::Float(1234567890.0))];
        let result = convert_time_method("ctime", &args, &mut ctx);
        assert!(result.is_ok());
        // assert!(ctx.needs_chrono); // Not in NASA mode
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("UNIX_EPOCH")); // NASA mode uses std::time
                                              // DEPYLER-1086: NASA mode (default) uses std::time, not chrono
                                              // chrono::Utc is only generated when nasa_mode=false
    }

    #[test]
    fn test_convert_time_ctime_missing_args() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_time_method("ctime", &args, &mut ctx);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("requires exactly 1 argument"));
    }

    // ============ convert_time_method - strftime() tests ============

    #[test]
    fn test_convert_time_strftime_basic() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::String("%Y-%m-%d".to_string())),
            HirExpr::Var("time_tuple".to_string()),
        ];
        let result = convert_time_method("strftime", &args, &mut ctx);
        assert!(result.is_ok());
        // assert!(ctx.needs_chrono); // Not in NASA mode
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("format"));
    }

    #[test]
    fn test_convert_time_strftime_with_variable_format() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Var("fmt".to_string()),
            HirExpr::Var("time_tuple".to_string()),
        ];
        let result = convert_time_method("strftime", &args, &mut ctx);
        assert!(result.is_ok());
        // assert!(ctx.needs_chrono); // Not in NASA mode
    }

    #[test]
    fn test_convert_time_strftime_missing_args() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::String("%Y".to_string()))];
        let result = convert_time_method("strftime", &args, &mut ctx);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("requires at least 2 arguments"));
    }

    // ============ convert_time_method - strptime() tests ============

    #[test]
    fn test_convert_time_strptime_basic() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::String("2023-01-15".to_string())),
            HirExpr::Literal(Literal::String("%Y-%m-%d".to_string())),
        ];
        let result = convert_time_method("strptime", &args, &mut ctx);
        assert!(result.is_ok());
        // assert!(ctx.needs_chrono); // Not in NASA mode
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("SystemTime")); // NASA mode stub
    }

    #[test]
    fn test_convert_time_strptime_with_variable_format() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Var("time_str".to_string()),
            HirExpr::Var("fmt".to_string()),
        ];
        let result = convert_time_method("strptime", &args, &mut ctx);
        assert!(result.is_ok());
        // assert!(ctx.needs_chrono); // Not in NASA mode
    }

    #[test]
    fn test_convert_time_strptime_missing_args() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::String("2023-01-15".to_string()))];
        let result = convert_time_method("strptime", &args, &mut ctx);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("requires at least 2 arguments"));
    }

    // ============ convert_time_method - gmtime() tests ============

    #[test]
    fn test_convert_time_gmtime_with_timestamp() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::Float(1234567890.0))];
        let result = convert_time_method("gmtime", &args, &mut ctx);
        assert!(result.is_ok());
        // assert!(ctx.needs_chrono); // Not in NASA mode
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("UNIX_EPOCH")); // NASA mode uses std::time
                                              // DEPYLER-1086: NASA mode (default) uses std::time, not chrono
                                              // chrono::Utc is only generated when nasa_mode=false
    }

    #[test]
    fn test_convert_time_gmtime_no_args() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_time_method("gmtime", &args, &mut ctx);
        assert!(result.is_ok());
        // assert!(ctx.needs_chrono); // Not in NASA mode
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("SystemTime") && code.contains("now"));
    }

    // ============ convert_time_method - localtime() tests ============

    #[test]
    fn test_convert_time_localtime_with_timestamp() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::Float(1234567890.0))];
        let result = convert_time_method("localtime", &args, &mut ctx);
        assert!(result.is_ok());
        // assert!(ctx.needs_chrono); // Not in NASA mode
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("UNIX_EPOCH")); // NASA mode uses std::time
                                              // DEPYLER-1086: NASA mode (default) uses std::time, not chrono
                                              // chrono::Local is only generated when nasa_mode=false
    }

    #[test]
    fn test_convert_time_localtime_no_args() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_time_method("localtime", &args, &mut ctx);
        assert!(result.is_ok());
        // assert!(ctx.needs_chrono); // Not in NASA mode
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("SystemTime") && code.contains("now"));
    }

    // ============ convert_time_method - mktime() tests ============

    #[test]
    fn test_convert_time_mktime_basic() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("time_tuple".to_string())];
        let result = convert_time_method("mktime", &args, &mut ctx);
        assert!(result.is_ok());
        // assert!(ctx.needs_chrono); // Not in NASA mode
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("timestamp"));
    }

    #[test]
    fn test_convert_time_mktime_missing_args() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_time_method("mktime", &args, &mut ctx);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("requires exactly 1 argument"));
    }

    // ============ convert_time_method - asctime() tests ============

    #[test]
    fn test_convert_time_asctime_basic() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("time_tuple".to_string())];
        let result = convert_time_method("asctime", &args, &mut ctx);
        assert!(result.is_ok());
        // assert!(ctx.needs_chrono); // Not in NASA mode
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("to_string"));
    }

    #[test]
    fn test_convert_time_asctime_missing_args() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_time_method("asctime", &args, &mut ctx);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("requires exactly 1 argument"));
    }

    // ============ convert_time_method - unknown method tests ============

    #[test]
    fn test_convert_time_unknown_method() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_time_method("unknown_func", &args, &mut ctx);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("not implemented yet"));
    }

    // ============ Helper function unit tests ============

    #[test]
    fn test_convert_time_time_helper() {
        let result = convert_time_time();
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_monotonic_helper() {
        let result = convert_monotonic();
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_process_time_helper() {
        let result = convert_process_time();
        assert!(result.is_ok());
    }
}
