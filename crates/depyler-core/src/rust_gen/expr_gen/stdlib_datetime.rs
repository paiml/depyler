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
            "total_seconds" => self.convert_inst_total_seconds(dt_expr, nasa_mode),
            "fromisoformat" => self.convert_inst_fromisoformat(dt_expr, arg_exprs, nasa_mode)?,
            "isoformat" => self.convert_inst_isoformat(dt_expr, nasa_mode),
            "strftime" => self.convert_inst_strftime(dt_expr, hir_args, arg_exprs, nasa_mode)?,
            "timestamp" => self.convert_inst_timestamp(dt_expr, nasa_mode),
            "timetuple" => self.convert_inst_timetuple(dt_expr, nasa_mode),
            "weekday" => self.convert_inst_weekday(dt_expr, nasa_mode),
            "isoweekday" => self.convert_inst_isoweekday(dt_expr, nasa_mode),
            "isocalendar" => self.convert_inst_isocalendar(dt_expr, nasa_mode),
            "replace" => {
                parse_quote! { #dt_expr }
            }
            _ => {
                let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
                parse_quote! { #dt_expr.#method_ident(#(#arg_exprs),*) }
            }
        };

        Ok(result)
    }

    fn convert_inst_total_seconds(&mut self, dt_expr: &syn::Expr, nasa_mode: bool) -> syn::Expr {
        if nasa_mode {
            self.ctx.needs_depyler_timedelta = true;
            parse_quote! { #dt_expr.total_seconds() }
        } else {
            parse_quote! { #dt_expr.num_seconds() as f64 }
        }
    }

    fn convert_inst_fromisoformat(
        &mut self,
        dt_expr: &syn::Expr,
        arg_exprs: &[syn::Expr],
        nasa_mode: bool,
    ) -> Result<syn::Expr> {
        if arg_exprs.is_empty() {
            bail!("fromisoformat() requires 1 argument (string)");
        }
        let s = &arg_exprs[0];
        if nasa_mode {
            self.ctx.needs_depyler_datetime = true;
            Ok(parse_quote! { DepylerDateTime::now() })
        } else {
            let _ = dt_expr;
            Ok(parse_quote! {
                chrono::NaiveDateTime::parse_from_str(&#s, "%Y-%m-%dT%H:%M:%S").expect("parse failed")
            })
        }
    }

    fn convert_inst_isoformat(&self, dt_expr: &syn::Expr, nasa_mode: bool) -> syn::Expr {
        if nasa_mode {
            parse_quote! { format!("{:?}", #dt_expr) }
        } else {
            parse_quote! { #dt_expr.format("%Y-%m-%dT%H:%M:%S").to_string() }
        }
    }

    fn convert_inst_strftime(
        &self,
        dt_expr: &syn::Expr,
        hir_args: &[HirExpr],
        arg_exprs: &[syn::Expr],
        nasa_mode: bool,
    ) -> Result<syn::Expr> {
        if arg_exprs.is_empty() {
            bail!("strftime() requires 1 argument (format string)");
        }
        if nasa_mode {
            return Ok(parse_quote! { format!("{:?}", #dt_expr) });
        }
        let fmt = match hir_args.first() {
            Some(HirExpr::Literal(Literal::String(s))) => parse_quote! { #s },
            _ => arg_exprs[0].clone(),
        };
        Ok(parse_quote! { #dt_expr.format(#fmt).to_string() })
    }

    fn convert_inst_timestamp(&self, dt_expr: &syn::Expr, nasa_mode: bool) -> syn::Expr {
        if nasa_mode {
            parse_quote! {
                #dt_expr.duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs_f64()).unwrap_or(0.0)
            }
        } else {
            parse_quote! { #dt_expr.and_utc().timestamp() as f64 }
        }
    }

    fn convert_inst_timetuple(&self, dt_expr: &syn::Expr, nasa_mode: bool) -> syn::Expr {
        if nasa_mode {
            parse_quote! { (0i32, 0u32, 0u32, 0u32, 0u32, 0u32) }
        } else {
            parse_quote! {
                (#dt_expr.year(), #dt_expr.month(), #dt_expr.day(),
                 #dt_expr.hour(), #dt_expr.minute(), #dt_expr.second())
            }
        }
    }

    fn convert_inst_weekday(&mut self, dt_expr: &syn::Expr, nasa_mode: bool) -> syn::Expr {
        if nasa_mode {
            self.ctx.needs_depyler_date = true;
            parse_quote! { #dt_expr.weekday() as i32 }
        } else {
            parse_quote! { #dt_expr.weekday().num_days_from_monday() as i32 }
        }
    }

    fn convert_inst_isoweekday(&mut self, dt_expr: &syn::Expr, nasa_mode: bool) -> syn::Expr {
        if nasa_mode {
            self.ctx.needs_depyler_date = true;
            parse_quote! { #dt_expr.isoweekday() as i32 }
        } else {
            parse_quote! { (#dt_expr.weekday().num_days_from_monday() + 1) as i32 }
        }
    }

    fn convert_inst_isocalendar(&self, dt_expr: &syn::Expr, nasa_mode: bool) -> syn::Expr {
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
        let arg_exprs: Vec<syn::Expr> =
            args.iter().map(|arg| arg.to_rust_expr(self.ctx)).collect::<Result<Vec<_>>>()?;

        let result = match method {
            "now" => self.convert_dt_now(&arg_exprs, nasa_mode),
            "utcnow" => self.convert_dt_utcnow(&arg_exprs, nasa_mode)?,
            "today" => self.convert_dt_today(&arg_exprs, nasa_mode)?,
            "strftime" => self.convert_dt_strftime(args, &arg_exprs, nasa_mode)?,
            "strptime" => self.convert_dt_strptime(args, &arg_exprs, nasa_mode)?,
            "isoformat" => self.convert_dt_isoformat(&arg_exprs, nasa_mode)?,
            "timestamp" => self.convert_dt_timestamp(&arg_exprs, nasa_mode)?,
            "fromtimestamp" => self.convert_dt_fromtimestamp(&arg_exprs, nasa_mode)?,
            "weekday" => self.convert_dt_weekday(&arg_exprs, nasa_mode)?,
            "isoweekday" => self.convert_dt_isoweekday(&arg_exprs, nasa_mode)?,
            "total_seconds" => self.convert_dt_total_seconds(&arg_exprs, nasa_mode)?,
            "date" => self.convert_dt_date(&arg_exprs, nasa_mode)?,
            "time" => self.convert_dt_time(&arg_exprs, nasa_mode)?,
            "replace" => self.convert_dt_replace(&arg_exprs, nasa_mode)?,
            "combine" => self.convert_dt_combine(&arg_exprs, nasa_mode)?,
            "fromisoformat" => self.convert_dt_fromisoformat(&arg_exprs, nasa_mode)?,
            "fromordinal" => self.convert_dt_fromordinal(&arg_exprs, nasa_mode)?,
            _ => return Ok(None),
        };

        Ok(Some(result))
    }

    fn convert_dt_now(&mut self, arg_exprs: &[syn::Expr], nasa_mode: bool) -> syn::Expr {
        if nasa_mode {
            self.ctx.needs_depyler_datetime = true;
            parse_quote! { DepylerDateTime::now() }
        } else if arg_exprs.is_empty() {
            parse_quote! { chrono::Local::now().naive_local() }
        } else {
            parse_quote! { chrono::Utc::now().naive_utc() }
        }
    }

    fn convert_dt_utcnow(&mut self, arg_exprs: &[syn::Expr], nasa_mode: bool) -> Result<syn::Expr> {
        if !arg_exprs.is_empty() {
            bail!("datetime.utcnow() takes no arguments");
        }
        if nasa_mode {
            self.ctx.needs_depyler_datetime = true;
            Ok(parse_quote! { DepylerDateTime::now() })
        } else {
            Ok(parse_quote! { chrono::Utc::now().naive_utc() })
        }
    }

    fn convert_dt_today(&mut self, arg_exprs: &[syn::Expr], nasa_mode: bool) -> Result<syn::Expr> {
        if !arg_exprs.is_empty() {
            bail!("datetime.today() takes no arguments");
        }
        if nasa_mode {
            self.ctx.needs_depyler_datetime = true;
            Ok(parse_quote! { DepylerDateTime::today() })
        } else {
            Ok(parse_quote! { chrono::Local::now().naive_local() })
        }
    }

    fn convert_dt_strftime(
        &self,
        args: &[HirExpr],
        arg_exprs: &[syn::Expr],
        nasa_mode: bool,
    ) -> Result<syn::Expr> {
        if arg_exprs.len() != 2 {
            bail!("strftime() requires exactly 2 arguments (self, format)");
        }
        let dt = &arg_exprs[0];
        if nasa_mode {
            return Ok(parse_quote! { format!("{:?}", #dt) });
        }
        let fmt = match &args[1] {
            HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
            _ => arg_exprs[1].clone(),
        };
        Ok(parse_quote! { #dt.format(#fmt).to_string() })
    }

    fn convert_dt_strptime(
        &mut self,
        args: &[HirExpr],
        arg_exprs: &[syn::Expr],
        nasa_mode: bool,
    ) -> Result<syn::Expr> {
        if arg_exprs.len() != 2 {
            bail!("strptime() requires exactly 2 arguments (string, format)");
        }
        if nasa_mode {
            self.ctx.needs_depyler_datetime = true;
            return Ok(parse_quote! { DepylerDateTime::now() });
        }
        let s = &arg_exprs[0];
        let fmt: syn::Expr = match &args[1] {
            HirExpr::Literal(Literal::String(fmt_str)) => parse_quote! { #fmt_str },
            _ => {
                let fmt_expr = &arg_exprs[1];
                parse_quote! { &#fmt_expr }
            }
        };
        Ok(parse_quote! {
            chrono::NaiveDateTime::parse_from_str(#s, #fmt).expect("parse failed")
        })
    }

    fn convert_dt_isoformat(&self, arg_exprs: &[syn::Expr], nasa_mode: bool) -> Result<syn::Expr> {
        if arg_exprs.len() != 1 {
            bail!("isoformat() requires exactly 1 argument (self)");
        }
        let dt = &arg_exprs[0];
        if nasa_mode {
            Ok(parse_quote! { format!("{:?}", #dt) })
        } else {
            Ok(parse_quote! { #dt.to_string() })
        }
    }

    fn convert_dt_timestamp(&self, arg_exprs: &[syn::Expr], nasa_mode: bool) -> Result<syn::Expr> {
        if arg_exprs.len() != 1 {
            bail!("timestamp() requires exactly 1 argument (self)");
        }
        let dt = &arg_exprs[0];
        if nasa_mode {
            Ok(parse_quote! {
                #dt.duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs_f64()).unwrap_or(0.0)
            })
        } else {
            Ok(parse_quote! { #dt.and_utc().timestamp() as f64 })
        }
    }

    fn convert_dt_fromtimestamp(
        &mut self,
        arg_exprs: &[syn::Expr],
        nasa_mode: bool,
    ) -> Result<syn::Expr> {
        if arg_exprs.is_empty() || arg_exprs.len() > 2 {
            bail!("fromtimestamp() requires 1 or 2 arguments (timestamp, [timezone])");
        }
        let ts = &arg_exprs[0];
        if nasa_mode {
            self.ctx.needs_depyler_datetime = true;
            Ok(parse_quote! { DepylerDateTime::fromtimestamp(#ts as f64) })
        } else {
            Ok(parse_quote! {
                chrono::DateTime::from_timestamp((#ts).clone() as i64, 0)
                    .expect("invalid timestamp")
                    .naive_local()
            })
        }
    }

    fn convert_dt_weekday(
        &mut self,
        arg_exprs: &[syn::Expr],
        nasa_mode: bool,
    ) -> Result<syn::Expr> {
        if arg_exprs.len() != 1 {
            bail!("weekday() requires exactly 1 argument (self)");
        }
        let dt = &arg_exprs[0];
        if nasa_mode {
            self.ctx.needs_depyler_date = true;
            Ok(parse_quote! { #dt.weekday() as i32 })
        } else {
            Ok(parse_quote! { #dt.weekday().num_days_from_monday() as i32 })
        }
    }

    fn convert_dt_isoweekday(
        &mut self,
        arg_exprs: &[syn::Expr],
        nasa_mode: bool,
    ) -> Result<syn::Expr> {
        if arg_exprs.len() != 1 {
            bail!("isoweekday() requires exactly 1 argument (self)");
        }
        let dt = &arg_exprs[0];
        if nasa_mode {
            self.ctx.needs_depyler_date = true;
            Ok(parse_quote! { #dt.isoweekday() as i32 })
        } else {
            Ok(parse_quote! { (#dt.weekday().num_days_from_monday() + 1) as i32 })
        }
    }

    fn convert_dt_total_seconds(
        &mut self,
        arg_exprs: &[syn::Expr],
        nasa_mode: bool,
    ) -> Result<syn::Expr> {
        if arg_exprs.len() != 1 {
            bail!("total_seconds() requires exactly 1 argument (self)");
        }
        let td = &arg_exprs[0];
        if nasa_mode {
            self.ctx.needs_depyler_timedelta = true;
            Ok(parse_quote! { #td.total_seconds() })
        } else {
            Ok(parse_quote! { #td.num_seconds() as f64 })
        }
    }

    fn convert_dt_date(&self, arg_exprs: &[syn::Expr], nasa_mode: bool) -> Result<syn::Expr> {
        if arg_exprs.len() != 1 {
            bail!("date() requires exactly 1 argument (self)");
        }
        let dt = &arg_exprs[0];
        if nasa_mode {
            Ok(parse_quote! { #dt })
        } else {
            Ok(parse_quote! { #dt.date() })
        }
    }

    fn convert_dt_time(&self, arg_exprs: &[syn::Expr], nasa_mode: bool) -> Result<syn::Expr> {
        if arg_exprs.len() != 1 {
            bail!("time() requires exactly 1 argument (self)");
        }
        let dt = &arg_exprs[0];
        if nasa_mode {
            Ok(parse_quote! { #dt })
        } else {
            Ok(parse_quote! { #dt.time() })
        }
    }

    fn convert_dt_replace(&self, arg_exprs: &[syn::Expr], nasa_mode: bool) -> Result<syn::Expr> {
        if arg_exprs.len() != 2 {
            bail!("replace() not fully implemented (requires keyword args)");
        }
        let dt = &arg_exprs[0];
        if nasa_mode {
            Ok(parse_quote! { #dt })
        } else {
            let new_year = &arg_exprs[1];
            Ok(parse_quote! { #dt.with_year(#new_year as i32).expect("invalid date") })
        }
    }

    fn convert_dt_combine(&self, arg_exprs: &[syn::Expr], nasa_mode: bool) -> Result<syn::Expr> {
        if arg_exprs.len() != 2 {
            bail!("combine() requires exactly 2 arguments (date, time)");
        }
        if nasa_mode {
            Ok(parse_quote! { std::time::SystemTime::now() })
        } else {
            let date_expr = &arg_exprs[0];
            let time_expr = &arg_exprs[1];
            Ok(parse_quote! { chrono::NaiveDateTime::new(#date_expr, #time_expr) })
        }
    }

    fn convert_dt_fromisoformat(
        &mut self,
        arg_exprs: &[syn::Expr],
        nasa_mode: bool,
    ) -> Result<syn::Expr> {
        if arg_exprs.len() != 1 {
            bail!("fromisoformat() requires exactly 1 argument (string)");
        }
        if nasa_mode {
            self.ctx.needs_depyler_datetime = true;
            Ok(parse_quote! { DepylerDateTime::now() })
        } else {
            let s = &arg_exprs[0];
            Ok(parse_quote! {
                chrono::NaiveDateTime::parse_from_str(#s, "%Y-%m-%dT%H:%M:%S")
                    .or_else(|_| chrono::NaiveDateTime::parse_from_str(#s, "%Y-%m-%d %H:%M:%S"))
                    .or_else(|_| chrono::NaiveDateTime::parse_from_str(#s, "%Y-%m-%d"))
                    .expect("parse failed")
            })
        }
    }

    fn convert_dt_fromordinal(
        &mut self,
        arg_exprs: &[syn::Expr],
        nasa_mode: bool,
    ) -> Result<syn::Expr> {
        if arg_exprs.len() != 1 {
            bail!("fromordinal() requires exactly 1 argument (ordinal)");
        }
        let n = &arg_exprs[0];
        if nasa_mode {
            self.ctx.needs_depyler_date = true;
            Ok(parse_quote! { DepylerDate::from_ordinal(#n as i64) })
        } else {
            Ok(parse_quote! {
                chrono::NaiveDate::from_num_days_from_ce_opt(#n as i32).expect("invalid date")
            })
        }
    }
}
