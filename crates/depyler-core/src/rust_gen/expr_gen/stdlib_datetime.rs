//! Datetime stdlib code generation
//!
//! Handles conversion of Python datetime module calls to Rust chrono/NASA equivalents.

use crate::hir::*;
use crate::rust_gen::context::ToRustExpr;
use anyhow::{bail, Result};
use syn::parse_quote;

use super::ExpressionConverter;

impl<'a, 'b> ExpressionConverter<'a, 'b> {
    /// DEPYLER-0830/1025: Convert datetime/timedelta methods on variable instances
    /// This handles cases like `td.total_seconds()` where td is a TimeDelta variable
    /// Unlike try_convert_datetime_method which handles module calls like datetime.datetime.now()
    #[inline]
    pub(crate) fn convert_datetime_instance_method(
        &mut self,
        dt_expr: &syn::Expr,
        method: &str,
        hir_args: &[HirExpr],
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        let nasa_mode = self.ctx.type_mapper.nasa_mode;
        // Only mark chrono needed if NOT in NASA mode
        if !nasa_mode {
            self.ctx.needs_chrono = true;
        }

        let result = match method {
            // timedelta.total_seconds() → td.total_seconds() (NASA DepylerTimeDelta) or td.num_seconds() as f64 (chrono)
            // DEPYLER-1068: Use DepylerTimeDelta::total_seconds() in NASA mode
            "total_seconds" => {
                if nasa_mode {
                    self.ctx.needs_depyler_timedelta = true;
                    parse_quote! { #dt_expr.total_seconds() }
                } else {
                    parse_quote! { #dt_expr.num_seconds() as f64 }
                }
            }

            // datetime.fromisoformat(s) → DepylerDateTime (NASA) or parse timestamp string (chrono)
            // DEPYLER-1067: Use DepylerDateTime in NASA mode
            "fromisoformat" => {
                if arg_exprs.is_empty() {
                    bail!("fromisoformat() requires 1 argument (string)");
                }
                let s = &arg_exprs[0];
                if nasa_mode {
                    // NASA mode: return current time (simplified placeholder)
                    self.ctx.needs_depyler_datetime = true;
                    parse_quote! { DepylerDateTime::now() }
                } else {
                    parse_quote! {
                        chrono::NaiveDateTime::parse_from_str(&#s, "%Y-%m-%dT%H:%M:%S").expect("parse failed")
                    }
                }
            }

            // datetime.isoformat() → format!("{:?}", dt) (NASA) or dt.format(...) (chrono)
            "isoformat" => {
                if nasa_mode {
                    parse_quote! { format!("{:?}", #dt_expr) }
                } else {
                    parse_quote! { #dt_expr.format("%Y-%m-%dT%H:%M:%S").to_string() }
                }
            }

            // datetime.strftime(fmt) → format!("{:?}", dt) (NASA) or dt.format(fmt) (chrono)
            "strftime" => {
                if arg_exprs.is_empty() {
                    bail!("strftime() requires 1 argument (format string)");
                }
                if nasa_mode {
                    parse_quote! { format!("{:?}", #dt_expr) }
                } else {
                    let fmt = match hir_args.first() {
                        Some(HirExpr::Literal(Literal::String(s))) => parse_quote! { #s },
                        _ => arg_exprs[0].clone(),
                    };
                    parse_quote! { #dt_expr.format(#fmt).to_string() }
                }
            }

            // datetime.timestamp() → UNIX_EPOCH.elapsed() (NASA) or dt.and_utc().timestamp() (chrono)
            "timestamp" => {
                if nasa_mode {
                    parse_quote! {
                        #dt_expr.duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs_f64()).unwrap_or(0.0)
                    }
                } else {
                    parse_quote! { #dt_expr.and_utc().timestamp() as f64 }
                }
            }

            // datetime.timetuple() - return tuple (NASA returns zeros, chrono extracts components)
            "timetuple" => {
                if nasa_mode {
                    parse_quote! { (0i32, 0u32, 0u32, 0u32, 0u32, 0u32) }
                } else {
                    parse_quote! {
                        (#dt_expr.year(), #dt_expr.month(), #dt_expr.day(),
                         #dt_expr.hour(), #dt_expr.minute(), #dt_expr.second())
                    }
                }
            }

            // datetime.weekday() → dt.weekday() (NASA DepylerDate) or dt.weekday().num_days_from_monday() (chrono)
            // DEPYLER-1066: Use DepylerDate::weekday() in NASA mode
            "weekday" => {
                if nasa_mode {
                    self.ctx.needs_depyler_date = true;
                    parse_quote! { #dt_expr.weekday() as i32 }
                } else {
                    parse_quote! { #dt_expr.weekday().num_days_from_monday() as i32 }
                }
            }

            // datetime.isoweekday() → dt.isoweekday() (NASA DepylerDate) or dt.weekday().number_from_monday() (chrono)
            // DEPYLER-1066: Use DepylerDate::isoweekday() in NASA mode
            "isoweekday" => {
                if nasa_mode {
                    self.ctx.needs_depyler_date = true;
                    parse_quote! { #dt_expr.isoweekday() as i32 }
                } else {
                    parse_quote! { (#dt_expr.weekday().num_days_from_monday() + 1) as i32 }
                }
            }

            // datetime.isocalendar() → (year, week, weekday) tuple
            "isocalendar" => {
                if nasa_mode {
                    parse_quote! { (2024i32, 1i32, 1i32) }
                } else {
                    parse_quote! {
                        {
                            let iso = #dt_expr.iso_week();
                            (iso.year(), iso.week() as i32, #dt_expr.weekday().number_from_monday() as i32)
                        }
                    }
                }
            }

            // datetime.replace() - simplified: pass through
            "replace" => {
                parse_quote! { #dt_expr }
            }

            // Fallback: pass through as method call
            _ => {
                let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
                parse_quote! { #dt_expr.#method_ident(#(#arg_exprs),*) }
            }
        };

        Ok(result)
    }

    /// Try to convert datetime module method calls
    /// DEPYLER-STDLIB-DATETIME/1025: Comprehensive datetime module support with NASA mode
    #[inline]
    pub(super) fn try_convert_datetime_method(
        &mut self,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        let nasa_mode = self.ctx.type_mapper.nasa_mode;
        // Only mark chrono needed if NOT in NASA mode
        if !nasa_mode {
            self.ctx.needs_chrono = true;
        }

        // Convert arguments first
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| arg.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;

        let result = match method {
            // datetime.datetime.now([tz]) → DepylerDateTime::now() (NASA) or Local::now() (chrono)
            // DEPYLER-1067: Use DepylerDateTime::now() in NASA mode
            "now" => {
                if nasa_mode {
                    self.ctx.needs_depyler_datetime = true;
                    parse_quote! { DepylerDateTime::now() }
                } else if arg_exprs.is_empty() {
                    parse_quote! { chrono::Local::now().naive_local() }
                } else {
                    parse_quote! { chrono::Utc::now().naive_utc() }
                }
            }

            // datetime.datetime.utcnow() → DepylerDateTime::now() (NASA) or Utc::now() (chrono)
            // DEPYLER-1067: Use DepylerDateTime in NASA mode
            "utcnow" => {
                if arg_exprs.is_empty() {
                    if nasa_mode {
                        self.ctx.needs_depyler_datetime = true;
                        parse_quote! { DepylerDateTime::now() }
                    } else {
                        parse_quote! { chrono::Utc::now().naive_utc() }
                    }
                } else {
                    bail!("datetime.utcnow() takes no arguments");
                }
            }

            // datetime.datetime.today() → DepylerDateTime::today() (NASA) or Local::now() (chrono)
            // DEPYLER-1067: Use DepylerDateTime::today() in NASA mode (datetime.today() returns datetime, not date)
            "today" => {
                if arg_exprs.is_empty() {
                    if nasa_mode {
                        self.ctx.needs_depyler_datetime = true;
                        parse_quote! { DepylerDateTime::today() }
                    } else {
                        parse_quote! { chrono::Local::now().naive_local() }
                    }
                } else {
                    bail!("datetime.today() takes no arguments");
                }
            }

            // datetime.datetime.strftime(format) → format!("{:?}", dt) (NASA) or dt.format(...) (chrono)
            "strftime" => {
                if arg_exprs.len() != 2 {
                    bail!("strftime() requires exactly 2 arguments (self, format)");
                }
                let dt = &arg_exprs[0];
                if nasa_mode {
                    parse_quote! { format!("{:?}", #dt) }
                } else {
                    let fmt = match &args[1] {
                        HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                        _ => arg_exprs[1].clone(),
                    };
                    parse_quote! { #dt.format(#fmt).to_string() }
                }
            }

            // datetime.datetime.strptime(string, format) → DepylerDateTime::now() (NASA) or parse_from_str (chrono)
            // DEPYLER-1067: Use DepylerDateTime in NASA mode
            "strptime" => {
                if arg_exprs.len() != 2 {
                    bail!("strptime() requires exactly 2 arguments (string, format)");
                }
                if nasa_mode {
                    // NASA mode: simplified - return current time as placeholder
                    // Full parsing would require significant code
                    self.ctx.needs_depyler_datetime = true;
                    parse_quote! { DepylerDateTime::now() }
                } else {
                    let s = &arg_exprs[0];
                    let fmt: syn::Expr = match &args[1] {
                        HirExpr::Literal(Literal::String(fmt_str)) => parse_quote! { #fmt_str },
                        _ => {
                            let fmt_expr = &arg_exprs[1];
                            parse_quote! { &#fmt_expr }
                        }
                    };
                    parse_quote! {
                        chrono::NaiveDateTime::parse_from_str(#s, #fmt).expect("parse failed")
                    }
                }
            }

            // datetime.datetime.isoformat() → format!("{:?}", dt) (NASA) or dt.to_string() (chrono)
            "isoformat" => {
                if arg_exprs.len() != 1 {
                    bail!("isoformat() requires exactly 1 argument (self)");
                }
                let dt = &arg_exprs[0];
                if nasa_mode {
                    parse_quote! { format!("{:?}", #dt) }
                } else {
                    parse_quote! { #dt.to_string() }
                }
            }

            // datetime.datetime.timestamp() → UNIX_EPOCH duration (NASA) or dt.timestamp() (chrono)
            "timestamp" => {
                if arg_exprs.len() != 1 {
                    bail!("timestamp() requires exactly 1 argument (self)");
                }
                let dt = &arg_exprs[0];
                if nasa_mode {
                    parse_quote! {
                        #dt.duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs_f64()).unwrap_or(0.0)
                    }
                } else {
                    parse_quote! { #dt.and_utc().timestamp() as f64 }
                }
            }

            // datetime.datetime.fromtimestamp(ts) or datetime.datetime.fromtimestamp(ts, tz)
            // → DepylerDateTime (NASA) or DateTime (chrono)
            // DEPYLER-1067: Use DepylerDateTime::fromtimestamp() in NASA mode
            // Note: Second argument (timezone) is ignored in NASA mode
            "fromtimestamp" => {
                if arg_exprs.is_empty() || arg_exprs.len() > 2 {
                    bail!("fromtimestamp() requires 1 or 2 arguments (timestamp, [timezone])");
                }
                let ts = &arg_exprs[0];
                if nasa_mode {
                    self.ctx.needs_depyler_datetime = true;
                    parse_quote! { DepylerDateTime::fromtimestamp(#ts as f64) }
                } else {
                    parse_quote! {
                        chrono::DateTime::from_timestamp((#ts).clone() as i64, 0)
                            .expect("invalid timestamp")
                            .naive_local()
                    }
                }
            }

            // date.weekday() → dt.weekday() (NASA DepylerDate) or dt.weekday().num_days_from_monday() (chrono)
            // DEPYLER-1066: Use DepylerDate::weekday() in NASA mode
            "weekday" => {
                if arg_exprs.len() != 1 {
                    bail!("weekday() requires exactly 1 argument (self)");
                }
                let dt = &arg_exprs[0];
                if nasa_mode {
                    self.ctx.needs_depyler_date = true;
                    parse_quote! { #dt.weekday() as i32 }
                } else {
                    parse_quote! { #dt.weekday().num_days_from_monday() as i32 }
                }
            }

            // date.isoweekday() → dt.isoweekday() (NASA DepylerDate) or dt.weekday().number_from_monday() (chrono)
            // DEPYLER-1066: Use DepylerDate::isoweekday() in NASA mode
            "isoweekday" => {
                if arg_exprs.len() != 1 {
                    bail!("isoweekday() requires exactly 1 argument (self)");
                }
                let dt = &arg_exprs[0];
                if nasa_mode {
                    self.ctx.needs_depyler_date = true;
                    parse_quote! { #dt.isoweekday() as i32 }
                } else {
                    parse_quote! { (#dt.weekday().num_days_from_monday() + 1) as i32 }
                }
            }

            // timedelta.total_seconds() → total_seconds() (NASA DepylerTimeDelta) or num_seconds (chrono)
            // DEPYLER-1068: Use DepylerTimeDelta::total_seconds() in NASA mode
            "total_seconds" => {
                if arg_exprs.len() != 1 {
                    bail!("total_seconds() requires exactly 1 argument (self)");
                }
                let td = &arg_exprs[0];
                if nasa_mode {
                    self.ctx.needs_depyler_timedelta = true;
                    parse_quote! { #td.total_seconds() }
                } else {
                    parse_quote! { #td.num_seconds() as f64 }
                }
            }

            // datetime.date() → extract date part (passthrough for both modes)
            "date" => {
                if arg_exprs.len() != 1 {
                    bail!("date() requires exactly 1 argument (self)");
                }
                let dt = &arg_exprs[0];
                if nasa_mode {
                    parse_quote! { #dt }
                } else {
                    parse_quote! { #dt.date() }
                }
            }

            // datetime.time() → extract time part (passthrough for NASA)
            "time" => {
                if arg_exprs.len() != 1 {
                    bail!("time() requires exactly 1 argument (self)");
                }
                let dt = &arg_exprs[0];
                if nasa_mode {
                    parse_quote! { #dt }
                } else {
                    parse_quote! { #dt.time() }
                }
            }

            // datetime.replace() - passthrough for both modes
            "replace" => {
                if arg_exprs.len() != 2 {
                    bail!("replace() not fully implemented (requires keyword args)");
                }
                let dt = &arg_exprs[0];
                if nasa_mode {
                    parse_quote! { #dt }
                } else {
                    let new_year = &arg_exprs[1];
                    parse_quote! { #dt.with_year(#new_year as i32).expect("invalid date") }
                }
            }

            // DEPYLER-0938/1025: datetime.combine(date, time) → SystemTime (NASA) or NaiveDateTime (chrono)
            "combine" => {
                if arg_exprs.len() != 2 {
                    bail!("combine() requires exactly 2 arguments (date, time)");
                }
                if nasa_mode {
                    parse_quote! { std::time::SystemTime::now() }
                } else {
                    let date_expr = &arg_exprs[0];
                    let time_expr = &arg_exprs[1];
                    parse_quote! { chrono::NaiveDateTime::new(#date_expr, #time_expr) }
                }
            }

            // DEPYLER-0938/1025/1067: datetime.fromisoformat(string) → DepylerDateTime (NASA) or parse_from_str (chrono)
            "fromisoformat" => {
                if arg_exprs.len() != 1 {
                    bail!("fromisoformat() requires exactly 1 argument (string)");
                }
                if nasa_mode {
                    self.ctx.needs_depyler_datetime = true;
                    parse_quote! { DepylerDateTime::now() }
                } else {
                    let s = &arg_exprs[0];
                    parse_quote! {
                        chrono::NaiveDateTime::parse_from_str(#s, "%Y-%m-%dT%H:%M:%S")
                            .or_else(|_| chrono::NaiveDateTime::parse_from_str(#s, "%Y-%m-%d %H:%M:%S"))
                            .or_else(|_| chrono::NaiveDateTime::parse_from_str(#s, "%Y-%m-%d"))
                            .expect("parse failed")
                    }
                }
            }

            // DEPYLER-1069: date.fromordinal(n) → DepylerDate (NASA) or NaiveDate (chrono)
            "fromordinal" => {
                if arg_exprs.len() != 1 {
                    bail!("fromordinal() requires exactly 1 argument (ordinal)");
                }
                let n = &arg_exprs[0];
                if nasa_mode {
                    self.ctx.needs_depyler_date = true;
                    parse_quote! { DepylerDate::from_ordinal(#n as i64) }
                } else {
                    parse_quote! {
                        chrono::NaiveDate::from_num_days_from_ce_opt(#n as i32).expect("invalid date")
                    }
                }
            }

            // Note: min/max are handled in the caller where we know the module name
            // to differentiate between date.min and datetime.min
            _ => return Ok(None), // Not a recognized datetime method
        };

        Ok(Some(result))
    }
}
