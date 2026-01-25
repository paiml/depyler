//! Datetime Module Code Generation - EXTREME TDD
//!
//! Handles Python `datetime` module conversions to Rust chrono/std.
//! Single-shot compilation: No manual lifetime annotations needed.
//!
//! Coverage target: 100% line coverage, 100% branch coverage
//!
//! ## Supported Classes
//! - `datetime.datetime` → chrono::DateTime or DepylerDateTime (NASA mode)
//! - `datetime.date` → chrono::NaiveDate or DepylerDate (NASA mode)
//! - `datetime.time` → chrono::NaiveTime
//! - `datetime.timedelta` → chrono::Duration or DepylerTimeDelta (NASA mode)
//!
//! ## Design Principles
//! 1. NASA Mode: Use std library only (DepylerDateTime struct)
//! 2. Non-NASA Mode: Use chrono crate for full datetime support
//! 3. Strategic Cloning: Clone when ownership required

use crate::hir::HirExpr;
use crate::rust_gen::context::{CodeGenContext, ToRustExpr};
use anyhow::{bail, Result};
use syn::parse_quote;

/// Convert Python datetime module method calls to Rust
///
/// # Supported Methods - datetime.datetime
/// - `datetime.datetime.now()` → current datetime
/// - `datetime.datetime.utcnow()` → current UTC datetime
/// - `datetime.datetime.today()` → current date as datetime
/// - `datetime.datetime.fromisoformat(s)` → parse ISO 8601 string
/// - `datetime.datetime.fromtimestamp(ts)` → from Unix timestamp
/// - `datetime.datetime.combine(date, time)` → combine date and time
/// - `datetime.datetime.strptime(s, fmt)` → parse with format string
///
/// # Supported Methods - datetime.date
/// - `datetime.date.today()` → current date
/// - `datetime.date.fromisoformat(s)` → parse ISO 8601 date
/// - `datetime.date.fromtimestamp(ts)` → from Unix timestamp
///
/// # Supported Methods - datetime.timedelta
/// - `datetime.timedelta(...)` → duration
///
/// # Complexity: 9 (within limits)
pub fn convert_datetime_method(
    class_name: &str,
    method: &str,
    args: &[HirExpr],
    ctx: &mut CodeGenContext,
) -> Result<Option<syn::Expr>> {
    let nasa_mode = ctx.type_mapper.nasa_mode;

    // Convert arguments
    let arg_exprs: Vec<syn::Expr> = args
        .iter()
        .map(|arg| arg.to_rust_expr(ctx))
        .collect::<Result<Vec<_>>>()?;

    let result = match (class_name, method) {
        // datetime.datetime class methods
        ("datetime", "now") => convert_datetime_now(nasa_mode, ctx)?,
        ("datetime", "utcnow") => convert_datetime_utcnow(nasa_mode, ctx)?,
        ("datetime", "today") => convert_datetime_today(nasa_mode, ctx)?,
        ("datetime", "fromisoformat") => {
            convert_datetime_fromisoformat(&arg_exprs, nasa_mode, ctx)?
        }
        ("datetime", "fromtimestamp") => {
            convert_datetime_fromtimestamp(&arg_exprs, nasa_mode, ctx)?
        }
        ("datetime", "combine") => convert_datetime_combine(&arg_exprs, nasa_mode, ctx)?,
        ("datetime", "strptime") => convert_datetime_strptime(args, &arg_exprs, nasa_mode, ctx)?,

        // datetime.date class methods
        ("date", "today") => convert_date_today(nasa_mode, ctx)?,
        ("date", "fromisoformat") => convert_date_fromisoformat(&arg_exprs, nasa_mode, ctx)?,
        ("date", "fromtimestamp") => convert_date_fromtimestamp(&arg_exprs, nasa_mode, ctx)?,

        // datetime.time class methods
        ("time", "fromisoformat") => convert_time_fromisoformat(&arg_exprs, nasa_mode, ctx)?,

        // timedelta constructor
        ("timedelta", "new" | "__init__") => {
            convert_timedelta_new(args, &arg_exprs, nasa_mode, ctx)?
        }

        _ => bail!("datetime.{}.{} not implemented yet", class_name, method),
    };

    Ok(Some(result))
}

/// Convert datetime instance methods (called on datetime objects)
pub fn convert_datetime_instance_method(
    method: &str,
    args: &[HirExpr],
    ctx: &mut CodeGenContext,
) -> Result<Option<syn::Expr>> {
    let nasa_mode = ctx.type_mapper.nasa_mode;

    let arg_exprs: Vec<syn::Expr> = args
        .iter()
        .map(|arg| arg.to_rust_expr(ctx))
        .collect::<Result<Vec<_>>>()?;

    let result = match method {
        "date" => convert_instance_date(nasa_mode, ctx)?,
        "time" => convert_instance_time(nasa_mode, ctx)?,
        "timestamp" => convert_instance_timestamp(nasa_mode, ctx)?,
        "isoformat" => convert_instance_isoformat(nasa_mode, ctx)?,
        "strftime" => convert_instance_strftime(&arg_exprs, nasa_mode, ctx)?,
        "replace" => convert_instance_replace(args, &arg_exprs, nasa_mode, ctx)?,
        "weekday" => convert_instance_weekday(nasa_mode, ctx)?,
        "isoweekday" => convert_instance_isoweekday(nasa_mode, ctx)?,
        "year" | "month" | "day" | "hour" | "minute" | "second" | "microsecond" => {
            convert_instance_component(method, nasa_mode, ctx)?
        }
        _ => bail!("datetime instance method '{}' not implemented", method),
    };

    Ok(Some(result))
}

// =============================================================================
// datetime.datetime class methods
// =============================================================================

fn convert_datetime_now(nasa_mode: bool, ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    if nasa_mode {
        ctx.needs_depyler_datetime = true;
        Ok(parse_quote! { DepylerDateTime::now() })
    } else {
        ctx.needs_chrono = true;
        Ok(parse_quote! { chrono::Local::now() })
    }
}

fn convert_datetime_utcnow(nasa_mode: bool, ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    if nasa_mode {
        ctx.needs_depyler_datetime = true;
        Ok(parse_quote! { DepylerDateTime::utcnow() })
    } else {
        ctx.needs_chrono = true;
        Ok(parse_quote! { chrono::Utc::now() })
    }
}

fn convert_datetime_today(nasa_mode: bool, ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    if nasa_mode {
        ctx.needs_depyler_datetime = true;
        Ok(parse_quote! { DepylerDateTime::today() })
    } else {
        ctx.needs_chrono = true;
        Ok(parse_quote! { chrono::Local::now().date_naive() })
    }
}

fn convert_datetime_fromisoformat(
    arg_exprs: &[syn::Expr],
    nasa_mode: bool,
    ctx: &mut CodeGenContext,
) -> Result<syn::Expr> {
    if arg_exprs.is_empty() {
        bail!("datetime.fromisoformat() requires 1 argument");
    }
    let s = &arg_exprs[0];

    if nasa_mode {
        ctx.needs_depyler_datetime = true;
        Ok(parse_quote! { DepylerDateTime::fromisoformat(#s)? })
    } else {
        ctx.needs_chrono = true;
        Ok(parse_quote! {
            chrono::NaiveDateTime::parse_from_str(#s, "%Y-%m-%dT%H:%M:%S")
                .or_else(|_| chrono::NaiveDateTime::parse_from_str(#s, "%Y-%m-%d %H:%M:%S"))?
        })
    }
}

fn convert_datetime_fromtimestamp(
    arg_exprs: &[syn::Expr],
    nasa_mode: bool,
    ctx: &mut CodeGenContext,
) -> Result<syn::Expr> {
    if arg_exprs.is_empty() {
        bail!("datetime.fromtimestamp() requires 1 argument");
    }
    let ts = &arg_exprs[0];

    if nasa_mode {
        ctx.needs_depyler_datetime = true;
        Ok(parse_quote! { DepylerDateTime::fromtimestamp(#ts as i64) })
    } else {
        ctx.needs_chrono = true;
        Ok(parse_quote! {
            chrono::DateTime::from_timestamp(#ts as i64, 0)
                .map(|dt| dt.naive_local())
                .unwrap_or_else(|| chrono::NaiveDateTime::default())
        })
    }
}

fn convert_datetime_combine(
    arg_exprs: &[syn::Expr],
    nasa_mode: bool,
    ctx: &mut CodeGenContext,
) -> Result<syn::Expr> {
    if arg_exprs.len() < 2 {
        bail!("datetime.combine() requires 2 arguments (date, time)");
    }
    let date_expr = &arg_exprs[0];
    let time_expr = &arg_exprs[1];

    if nasa_mode {
        ctx.needs_depyler_datetime = true;
        Ok(parse_quote! { DepylerDateTime::combine(#date_expr, #time_expr) })
    } else {
        ctx.needs_chrono = true;
        Ok(parse_quote! { chrono::NaiveDateTime::new(#date_expr, #time_expr) })
    }
}

fn convert_datetime_strptime(
    _args: &[HirExpr],
    arg_exprs: &[syn::Expr],
    nasa_mode: bool,
    ctx: &mut CodeGenContext,
) -> Result<syn::Expr> {
    if arg_exprs.len() < 2 {
        bail!("datetime.strptime() requires 2 arguments (string, format)");
    }
    let s = &arg_exprs[0];
    let fmt = &arg_exprs[1];

    if nasa_mode {
        ctx.needs_depyler_datetime = true;
        Ok(parse_quote! { DepylerDateTime::strptime(#s, #fmt)? })
    } else {
        ctx.needs_chrono = true;
        // Convert Python format to chrono format at runtime
        Ok(parse_quote! {
            chrono::NaiveDateTime::parse_from_str(
                #s,
                &depyler_runtime::convert_python_datetime_format(#fmt)
            )?
        })
    }
}

// =============================================================================
// datetime.date class methods
// =============================================================================

fn convert_date_today(nasa_mode: bool, ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    if nasa_mode {
        ctx.needs_depyler_date = true;
        Ok(parse_quote! { DepylerDate::today() })
    } else {
        ctx.needs_chrono = true;
        Ok(parse_quote! { chrono::Local::now().date_naive() })
    }
}

fn convert_date_fromisoformat(
    arg_exprs: &[syn::Expr],
    nasa_mode: bool,
    ctx: &mut CodeGenContext,
) -> Result<syn::Expr> {
    if arg_exprs.is_empty() {
        bail!("date.fromisoformat() requires 1 argument");
    }
    let s = &arg_exprs[0];

    if nasa_mode {
        ctx.needs_depyler_date = true;
        Ok(parse_quote! { DepylerDate::fromisoformat(#s)? })
    } else {
        ctx.needs_chrono = true;
        Ok(parse_quote! { chrono::NaiveDate::parse_from_str(#s, "%Y-%m-%d")? })
    }
}

fn convert_date_fromtimestamp(
    arg_exprs: &[syn::Expr],
    nasa_mode: bool,
    ctx: &mut CodeGenContext,
) -> Result<syn::Expr> {
    if arg_exprs.is_empty() {
        bail!("date.fromtimestamp() requires 1 argument");
    }
    let ts = &arg_exprs[0];

    if nasa_mode {
        ctx.needs_depyler_date = true;
        Ok(parse_quote! { DepylerDate::fromtimestamp(#ts as i64) })
    } else {
        ctx.needs_chrono = true;
        Ok(parse_quote! {
            chrono::DateTime::from_timestamp(#ts as i64, 0)
                .map(|dt| dt.date_naive())
                .unwrap_or_else(|| chrono::NaiveDate::default())
        })
    }
}

fn convert_time_fromisoformat(
    arg_exprs: &[syn::Expr],
    nasa_mode: bool,
    ctx: &mut CodeGenContext,
) -> Result<syn::Expr> {
    if arg_exprs.is_empty() {
        bail!("time.fromisoformat() requires 1 argument");
    }
    let s = &arg_exprs[0];

    if nasa_mode {
        // NASA mode: simple struct
        Ok(parse_quote! {
            {
                let parts: Vec<&str> = #s.split(':').collect();
                (
                    parts.get(0).and_then(|s| s.parse::<u32>().ok()).unwrap_or(0),
                    parts.get(1).and_then(|s| s.parse::<u32>().ok()).unwrap_or(0),
                    parts.get(2).and_then(|s| s.parse::<u32>().ok()).unwrap_or(0),
                )
            }
        })
    } else {
        ctx.needs_chrono = true;
        Ok(parse_quote! { chrono::NaiveTime::parse_from_str(#s, "%H:%M:%S")? })
    }
}

// =============================================================================
// datetime.timedelta
// =============================================================================

fn convert_timedelta_new(
    _args: &[HirExpr],
    arg_exprs: &[syn::Expr],
    nasa_mode: bool,
    ctx: &mut CodeGenContext,
) -> Result<syn::Expr> {
    // timedelta(days=0, seconds=0, microseconds=0, milliseconds=0, minutes=0, hours=0, weeks=0)
    // For simplicity, support positional: timedelta(days, seconds, microseconds)
    // or keyword arguments

    if nasa_mode {
        ctx.needs_depyler_timedelta = true;
        if arg_exprs.is_empty() {
            Ok(parse_quote! { DepylerTimeDelta::new(0, 0, 0) })
        } else if arg_exprs.len() == 1 {
            let days = &arg_exprs[0];
            Ok(parse_quote! { DepylerTimeDelta::from_days(#days as i64) })
        } else if arg_exprs.len() >= 2 {
            let days = &arg_exprs[0];
            let seconds = &arg_exprs[1];
            Ok(parse_quote! { DepylerTimeDelta::new(#days as i64, #seconds as i64, 0) })
        } else {
            Ok(parse_quote! { DepylerTimeDelta::new(0, 0, 0) })
        }
    } else {
        ctx.needs_chrono = true;
        if arg_exprs.is_empty() {
            Ok(parse_quote! { chrono::Duration::zero() })
        } else if arg_exprs.len() == 1 {
            let days = &arg_exprs[0];
            Ok(parse_quote! { chrono::Duration::days(#days as i64) })
        } else if arg_exprs.len() >= 2 {
            let days = &arg_exprs[0];
            let seconds = &arg_exprs[1];
            Ok(parse_quote! {
                chrono::Duration::days(#days as i64) + chrono::Duration::seconds(#seconds as i64)
            })
        } else {
            Ok(parse_quote! { chrono::Duration::zero() })
        }
    }
}

// =============================================================================
// Instance methods (called on datetime/date objects)
// =============================================================================

fn convert_instance_date(nasa_mode: bool, ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    if nasa_mode {
        ctx.needs_depyler_date = true;
        // Placeholder - actual object reference handled by caller
        Ok(parse_quote! { self.date() })
    } else {
        ctx.needs_chrono = true;
        Ok(parse_quote! { self.date() })
    }
}

fn convert_instance_time(nasa_mode: bool, ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    if nasa_mode {
        Ok(parse_quote! { self.time() })
    } else {
        ctx.needs_chrono = true;
        Ok(parse_quote! { self.time() })
    }
}

fn convert_instance_timestamp(nasa_mode: bool, ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    if nasa_mode {
        Ok(parse_quote! { self.timestamp() })
    } else {
        ctx.needs_chrono = true;
        Ok(parse_quote! { self.and_utc().timestamp() as f64 })
    }
}

fn convert_instance_isoformat(nasa_mode: bool, ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    if nasa_mode {
        Ok(parse_quote! { self.isoformat() })
    } else {
        ctx.needs_chrono = true;
        Ok(parse_quote! { self.format("%Y-%m-%dT%H:%M:%S").to_string() })
    }
}

fn convert_instance_strftime(
    arg_exprs: &[syn::Expr],
    nasa_mode: bool,
    ctx: &mut CodeGenContext,
) -> Result<syn::Expr> {
    if arg_exprs.is_empty() {
        bail!("strftime() requires 1 argument (format string)");
    }
    let fmt = &arg_exprs[0];

    if nasa_mode {
        Ok(parse_quote! { self.strftime(#fmt) })
    } else {
        ctx.needs_chrono = true;
        Ok(parse_quote! {
            self.format(&depyler_runtime::convert_python_datetime_format(#fmt)).to_string()
        })
    }
}

fn convert_instance_replace(
    _args: &[HirExpr],
    _arg_exprs: &[syn::Expr],
    nasa_mode: bool,
    ctx: &mut CodeGenContext,
) -> Result<syn::Expr> {
    // Simplified: replace() returns a new datetime with modified components
    // Full implementation would handle kwargs (year=, month=, etc.)
    if nasa_mode {
        Ok(parse_quote! { self.replace() })
    } else {
        ctx.needs_chrono = true;
        // Without kwargs, just clone
        Ok(parse_quote! { self.clone() })
    }
}

fn convert_instance_weekday(nasa_mode: bool, ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    if nasa_mode {
        Ok(parse_quote! { self.weekday() })
    } else {
        ctx.needs_chrono = true;
        // chrono weekday is 0=Mon, Python is 0=Mon (same!)
        Ok(parse_quote! { self.weekday().num_days_from_monday() as i32 })
    }
}

fn convert_instance_isoweekday(nasa_mode: bool, ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    if nasa_mode {
        Ok(parse_quote! { self.isoweekday() })
    } else {
        ctx.needs_chrono = true;
        // ISO weekday: 1=Mon, 7=Sun
        Ok(parse_quote! { (self.weekday().num_days_from_monday() + 1) as i32 })
    }
}

fn convert_instance_component(
    component: &str,
    nasa_mode: bool,
    ctx: &mut CodeGenContext,
) -> Result<syn::Expr> {
    if !nasa_mode {
        ctx.needs_chrono = true;
    }

    match component {
        "year" => Ok(parse_quote! { self.year() as i32 }),
        "month" => Ok(parse_quote! { self.month() as u32 }),
        "day" => Ok(parse_quote! { self.day() as u32 }),
        "hour" => Ok(parse_quote! { self.hour() as u32 }),
        "minute" => Ok(parse_quote! { self.minute() as u32 }),
        "second" => Ok(parse_quote! { self.second() as u32 }),
        "microsecond" => {
            if nasa_mode {
                Ok(parse_quote! { self.microsecond() as u32 })
            } else {
                Ok(parse_quote! { (self.nanosecond() / 1000) as u32 })
            }
        }
        _ => bail!("Unknown datetime component: {}", component),
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datetime_now_nasa_mode() {
        let mut ctx = CodeGenContext::default();
        let result = convert_datetime_now(true, &mut ctx).unwrap();
        assert!(ctx.needs_depyler_datetime);
        let code = quote::quote!(#result).to_string();
        assert!(code.contains("DepylerDateTime"));
    }

    #[test]
    fn test_datetime_now_chrono_mode() {
        let mut ctx = CodeGenContext::default();
        let result = convert_datetime_now(false, &mut ctx).unwrap();
        assert!(ctx.needs_chrono);
        let code = quote::quote!(#result).to_string();
        assert!(code.contains("chrono"));
    }

    #[test]
    fn test_datetime_utcnow_nasa_mode() {
        let mut ctx = CodeGenContext::default();
        let _result = convert_datetime_utcnow(true, &mut ctx).unwrap();
        assert!(ctx.needs_depyler_datetime);
    }

    #[test]
    fn test_datetime_utcnow_chrono_mode() {
        let mut ctx = CodeGenContext::default();
        let result = convert_datetime_utcnow(false, &mut ctx).unwrap();
        assert!(ctx.needs_chrono);
        let code = quote::quote!(#result).to_string();
        assert!(code.contains("Utc"));
    }

    #[test]
    fn test_date_today_nasa_mode() {
        let mut ctx = CodeGenContext::default();
        let _result = convert_date_today(true, &mut ctx).unwrap();
        assert!(ctx.needs_depyler_date);
    }

    #[test]
    fn test_date_today_chrono_mode() {
        let mut ctx = CodeGenContext::default();
        let _result = convert_date_today(false, &mut ctx).unwrap();
        assert!(ctx.needs_chrono);
    }

    #[test]
    fn test_timedelta_new_empty() {
        let mut ctx = CodeGenContext::default();
        let result = convert_timedelta_new(&[], &[], false, &mut ctx).unwrap();
        assert!(ctx.needs_chrono);
        let code = quote::quote!(#result).to_string();
        assert!(code.contains("zero"));
    }

    #[test]
    fn test_timedelta_new_days() {
        let mut ctx = CodeGenContext::default();
        let days: syn::Expr = parse_quote! { 5 };
        let result = convert_timedelta_new(&[], &[days], false, &mut ctx).unwrap();
        let code = quote::quote!(#result).to_string();
        assert!(code.contains("days"));
    }

    #[test]
    fn test_fromisoformat_requires_argument() {
        let mut ctx = CodeGenContext::default();
        let result = convert_datetime_fromisoformat(&[], false, &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_fromtimestamp_requires_argument() {
        let mut ctx = CodeGenContext::default();
        let result = convert_datetime_fromtimestamp(&[], false, &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_combine_requires_two_arguments() {
        let mut ctx = CodeGenContext::default();
        let result = convert_datetime_combine(&[], false, &mut ctx);
        assert!(result.is_err());

        let date: syn::Expr = parse_quote! { date };
        let result = convert_datetime_combine(&[date], false, &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_strptime_requires_two_arguments() {
        let mut ctx = CodeGenContext::default();
        let result = convert_datetime_strptime(&[], &[], false, &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_instance_weekday() {
        let mut ctx = CodeGenContext::default();
        let result = convert_instance_weekday(false, &mut ctx).unwrap();
        assert!(ctx.needs_chrono);
        let code = quote::quote!(#result).to_string();
        assert!(code.contains("weekday"));
    }

    #[test]
    fn test_instance_isoweekday() {
        let mut ctx = CodeGenContext::default();
        let result = convert_instance_isoweekday(false, &mut ctx).unwrap();
        let code = quote::quote!(#result).to_string();
        assert!(code.contains("weekday"));
    }

    #[test]
    fn test_instance_components() {
        let mut ctx = CodeGenContext::default();

        for component in &["year", "month", "day", "hour", "minute", "second"] {
            let result = convert_instance_component(component, false, &mut ctx).unwrap();
            let code = quote::quote!(#result).to_string();
            assert!(
                code.contains(component),
                "Component {} not found",
                component
            );
        }
    }

    #[test]
    fn test_instance_microsecond_chrono() {
        let mut ctx = CodeGenContext::default();
        let result = convert_instance_component("microsecond", false, &mut ctx).unwrap();
        let code = quote::quote!(#result).to_string();
        assert!(code.contains("nanosecond"));
    }

    #[test]
    fn test_instance_microsecond_nasa() {
        let mut ctx = CodeGenContext::default();
        let result = convert_instance_component("microsecond", true, &mut ctx).unwrap();
        let code = quote::quote!(#result).to_string();
        assert!(code.contains("microsecond"));
    }

    #[test]
    fn test_convert_datetime_method_dispatch() {
        let mut ctx = CodeGenContext::default();
        let result = convert_datetime_method("datetime", "now", &[], &mut ctx).unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_convert_datetime_method_unsupported() {
        let mut ctx = CodeGenContext::default();
        let result = convert_datetime_method("datetime", "unsupported_method", &[], &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_strftime_requires_format() {
        let mut ctx = CodeGenContext::default();
        let result = convert_instance_strftime(&[], false, &mut ctx);
        assert!(result.is_err());
    }
}
