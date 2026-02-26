//! Slicing handlers for ExpressionConverter
//!
//! Extracted from mod.rs to reduce file size. Contains `convert_slice`
//! and `convert_string_slice` for Vec/List and String slicing operations.

use crate::hir::*;
use crate::rust_gen::context::ToRustExpr;
use crate::rust_gen::expr_gen::ExpressionConverter;
use anyhow::Result;
use syn::parse_quote;

impl<'a, 'b> ExpressionConverter<'a, 'b> {
    pub(crate) fn convert_slice(
        &mut self,
        base: &HirExpr,
        start: &Option<Box<HirExpr>>,
        stop: &Option<Box<HirExpr>>,
        step: &Option<Box<HirExpr>>,
    ) -> Result<syn::Expr> {
        let base_expr = base.to_rust_expr(self.ctx)?;

        // DEPYLER-0302 Phase 3: Check if we're slicing a string
        let is_string = self.is_string_base(base);

        // Convert slice parameters
        let start_expr = if let Some(s) = start { Some(s.to_rust_expr(self.ctx)?) } else { None };

        let stop_expr = if let Some(s) = stop { Some(s.to_rust_expr(self.ctx)?) } else { None };

        let step_expr = if let Some(s) = step { Some(s.to_rust_expr(self.ctx)?) } else { None };

        // DEPYLER-0302 Phase 3: Generate string-specific slice code
        if is_string {
            // DEPYLER-0573: If base is dict value access returning Value, convert to owned String
            // Value.as_str() returns &str with limited lifetime, so convert to String
            let final_base_expr = if self.is_dict_value_access(base) {
                parse_quote! { #base_expr.as_str().map(|s| s.to_string()).unwrap_or_default() }
            } else {
                base_expr
            };
            return self.convert_string_slice(final_base_expr, start_expr, stop_expr, step_expr);
        }

        // Generate slice code based on the parameters (for Vec/List)
        match (start_expr, stop_expr, step_expr) {
            // Full slice with step: base[::step]
            (None, None, Some(step)) => {
                Ok(parse_quote! {
                    {
                        // DEPYLER-1148: Deref LazyLock and borrow inner value
                        let base = &*#base_expr;
                        // DEPYLER-0812: Use i32 for step to support negative values
                        let step: i32 = #step;
                        if step == 1 {
                            base.clone()
                        } else if step > 0 {
                            base.iter().step_by(step as usize).cloned().collect::<Vec<_>>()
                        } else if step == -1 {
                            base.iter().rev().cloned().collect::<Vec<_>>()
                        } else {
                            // Negative step with abs value
                            let abs_step = (-step) as usize;
                            base.iter().rev().step_by(abs_step).cloned().collect::<Vec<_>>()
                        }
                    }
                })
            }

            // Start and stop: base[start:stop]
            (Some(start), Some(stop), None) => Ok(parse_quote! {
                {
                    // DEPYLER-1148: Deref LazyLock and borrow inner value
                    let base = &*#base_expr;
                    // DEPYLER-0459: Cast to isize first to handle negative indices
                    // DEPYLER-1083: Parenthesize to avoid cast precedence issues (i + size as isize parses as i + (size as isize))
                    let start_idx = (#start) as isize;
                    let stop_idx = (#stop) as isize;
                    let start = if start_idx < 0 {
                        (base.len() as isize + start_idx).max(0) as usize
                    } else {
                        start_idx as usize
                    };
                    let stop = if stop_idx < 0 {
                        (base.len() as isize + stop_idx).max(0) as usize
                    } else {
                        stop_idx as usize
                    };
                    if start < base.len() {
                        base[start..stop.min(base.len())].to_vec()
                    } else {
                        Vec::new()
                    }
                }
            }),

            // Start only: base[start:]
            (Some(start), None, None) => Ok(parse_quote! {
                {
                    // DEPYLER-1148: Deref LazyLock and borrow inner value
                    let base = &*#base_expr;
                    // DEPYLER-0459: Cast to isize first to handle negative indices
                    // DEPYLER-1083: Parenthesize to avoid cast precedence issues
                    let start_idx = (#start) as isize;
                    let start = if start_idx < 0 {
                        (base.len() as isize + start_idx).max(0) as usize
                    } else {
                        start_idx as usize
                    };
                    if start < base.len() {
                        base[start..].to_vec()
                    } else {
                        Vec::new()
                    }
                }
            }),

            // Stop only: base[:stop]
            (None, Some(stop), None) => Ok(parse_quote! {
                {
                    // DEPYLER-1148: Deref LazyLock and borrow inner value
                    let base = &*#base_expr;
                    // DEPYLER-0459: Cast to isize first to handle negative indices
                    // DEPYLER-1083: Parenthesize to avoid cast precedence issues
                    let stop_idx = (#stop) as isize;
                    let stop = if stop_idx < 0 {
                        (base.len() as isize + stop_idx).max(0) as usize
                    } else {
                        stop_idx as usize
                    };
                    base[..stop.min(base.len())].to_vec()
                }
            }),

            // Full slice: base[:]
            (None, None, None) => Ok(parse_quote! { #base_expr.clone() }),

            // Start, stop, and step: base[start:stop:step]
            (Some(start), Some(stop), Some(step)) => {
                Ok(parse_quote! {
                    {
                        // DEPYLER-1148: Deref LazyLock and borrow inner value
                        let base = &*#base_expr;
                        // DEPYLER-0459: Cast to isize first to handle negative indices
                        // DEPYLER-1083: Parenthesize to avoid cast precedence issues
                        let start_idx = (#start) as isize;
                        let stop_idx = (#stop) as isize;
                        let start = if start_idx < 0 {
                            (base.len() as isize + start_idx).max(0) as usize
                        } else {
                            start_idx as usize
                        };
                        let stop = if stop_idx < 0 {
                            (base.len() as isize + stop_idx).max(0) as usize
                        } else {
                            stop_idx as usize
                        };
                        // DEPYLER-0812: Use i32 for step to support negative values
                        let step: i32 = #step;

                        if step == 1 {
                            if start < base.len() {
                                base[start..stop.min(base.len())].to_vec()
                            } else {
                                Vec::new()
                            }
                        } else if step > 0 {
                            base[start..stop.min(base.len())]
                                .iter()
                                .step_by(step as usize)
                                .cloned()
                                .collect::<Vec<_>>()
                        } else {
                            // Negative step - slice in reverse
                            let abs_step = (-step) as usize;
                            if start < base.len() {
                                base[start..stop.min(base.len())]
                                    .iter()
                                    .rev()
                                    .step_by(abs_step)
                                    .cloned()
                                    .collect::<Vec<_>>()
                            } else {
                                Vec::new()
                            }
                        }
                    }
                })
            }

            // Start and step: base[start::step]
            (Some(start), None, Some(step)) => Ok(parse_quote! {
                {
                    // DEPYLER-1148: Deref LazyLock and borrow inner value
                    let base = &*#base_expr;
                    // DEPYLER-0459: Cast to isize first to handle negative indices
                    // DEPYLER-1083: Parenthesize to avoid cast precedence issues
                    let start_idx = (#start) as isize;
                    let start = if start_idx < 0 {
                        (base.len() as isize + start_idx).max(0) as usize
                    } else {
                        start_idx as usize
                    };
                    // DEPYLER-0812: Use i32 for step to support negative values
                    let step: i32 = #step;

                    if start < base.len() {
                        if step == 1 {
                            base[start..].to_vec()
                        } else if step > 0 {
                            base[start..]
                                .iter()
                                .step_by(step as usize)
                                .cloned()
                                .collect::<Vec<_>>()
                        } else if step == -1 {
                            base[start..]
                                .iter()
                                .rev()
                                .cloned()
                                .collect::<Vec<_>>()
                        } else {
                            let abs_step = (-step) as usize;
                            base[start..]
                                .iter()
                                .rev()
                                .step_by(abs_step)
                                .cloned()
                                .collect::<Vec<_>>()
                        }
                    } else {
                        Vec::new()
                    }
                }
            }),

            // Stop and step: base[:stop:step]
            (None, Some(stop), Some(step)) => Ok(parse_quote! {
                {
                    // DEPYLER-1148: Deref LazyLock and borrow inner value
                    let base = &*#base_expr;
                    let stop = (#stop).max(0) as usize;
                    // DEPYLER-0812: Use i32 for step to support negative values
                    let step: i32 = #step;

                    if step == 1 {
                        base[..stop.min(base.len())].to_vec()
                    } else if step > 0 {
                        base[..stop.min(base.len())]
                            .iter()
                            .step_by(step as usize)
                            .cloned()
                            .collect::<Vec<_>>()
                    } else if step == -1 {
                        base[..stop.min(base.len())]
                            .iter()
                            .rev()
                            .cloned()
                            .collect::<Vec<_>>()
                    } else {
                        let abs_step = (-step) as usize;
                        base[..stop.min(base.len())]
                            .iter()
                            .rev()
                            .step_by(abs_step)
                            .cloned()
                            .collect::<Vec<_>>()
                    }
                }
            }),
        }
    }

    /// DEPYLER-0302 Phase 3: String-specific slice code generation
    /// Handles string slicing with proper char boundaries and negative indices
    pub(super) fn convert_string_slice(
        &mut self,
        base_expr: syn::Expr,
        start_expr: Option<syn::Expr>,
        stop_expr: Option<syn::Expr>,
        step_expr: Option<syn::Expr>,
    ) -> Result<syn::Expr> {
        match (start_expr, stop_expr, step_expr) {
            // Full slice with step: s[::step]
            // DEPYLER-1315: Clone base to prevent E0382 "use of moved value"
            (None, None, Some(step)) => {
                Ok(parse_quote! {
                    {
                        let base = (#base_expr).clone();
                        let step: i32 = #step;
                        if step == 1 {
                            base.to_string()
                        } else if step > 0 {
                            base.chars().step_by(step as usize).collect::<String>()
                        } else if step == -1 {
                            base.chars().rev().collect::<String>()
                        } else {
                            // Negative step with abs value
                            let abs_step = step.abs() as usize;
                            base.chars().rev().step_by(abs_step).collect::<String>()
                        }
                    }
                })
            }

            // Start and stop: s[start:stop]
            // DEPYLER-1315: Clone base to prevent E0382 "use of moved value"
            (Some(start), Some(stop), None) => Ok(parse_quote! {
                {
                    let base = (#base_expr).clone();
                    let start_idx: i32 = #start;
                    let stop_idx: i32 = #stop;
                    let len = base.chars().count() as i32;

                    // Handle negative indices
                    let actual_start = if start_idx < 0 {
                        (len + start_idx).max(0) as usize
                    } else {
                        start_idx.min(len) as usize
                    };

                    let actual_stop = if stop_idx < 0 {
                        (len + stop_idx).max(0) as usize
                    } else {
                        stop_idx.min(len) as usize
                    };

                    if actual_start < actual_stop {
                        base.chars().skip(actual_start).take(actual_stop - actual_start).collect::<String>()
                    } else {
                        String::new()
                    }
                }
            }),

            // Start only: s[start:]
            // DEPYLER-1315: Clone base to prevent E0382 "use of moved value"
            (Some(start), None, None) => Ok(parse_quote! {
                {
                    let base = (#base_expr).clone();
                    let start_idx: i32 = #start;
                    let len = base.chars().count() as i32;

                    // Handle negative index for s[-n:]
                    let actual_start = if start_idx < 0 {
                        (len + start_idx).max(0) as usize
                    } else {
                        start_idx.min(len) as usize
                    };

                    base.chars().skip(actual_start).collect::<String>()
                }
            }),

            // Stop only: s[:stop]
            // DEPYLER-1315: Clone base to prevent E0382 "use of moved value"
            (None, Some(stop), None) => Ok(parse_quote! {
                {
                    let base = (#base_expr).clone();
                    let stop_idx: i32 = #stop;
                    let len = base.chars().count() as i32;

                    // Handle negative index for s[:-n]
                    let actual_stop = if stop_idx < 0 {
                        (len + stop_idx).max(0) as usize
                    } else {
                        stop_idx.min(len) as usize
                    };

                    base.chars().take(actual_stop).collect::<String>()
                }
            }),

            // Full slice: s[:]
            (None, None, None) => Ok(parse_quote! { #base_expr.to_string() }),

            // Start, stop, and step: s[start:stop:step]
            // DEPYLER-1315: Clone base to prevent E0382 "use of moved value"
            (Some(start), Some(stop), Some(step)) => {
                Ok(parse_quote! {
                    {
                        let base = (#base_expr).clone();
                        let start_idx: i32 = #start;
                        let stop_idx: i32 = #stop;
                        let step: i32 = #step;
                        let len = base.chars().count() as i32;

                        // Handle negative indices
                        let actual_start = if start_idx < 0 {
                            (len + start_idx).max(0) as usize
                        } else {
                            start_idx.min(len) as usize
                        };

                        let actual_stop = if stop_idx < 0 {
                            (len + stop_idx).max(0) as usize
                        } else {
                            stop_idx.min(len) as usize
                        };

                        if step == 1 {
                            if actual_start < actual_stop {
                                base.chars().skip(actual_start).take(actual_stop - actual_start).collect::<String>()
                            } else {
                                String::new()
                            }
                        } else if step > 0 {
                            base.chars()
                                .skip(actual_start)
                                .take(actual_stop.saturating_sub(actual_start))
                                .step_by(step as usize)
                                .collect::<String>()
                        } else {
                            // Negative step - collect range then reverse
                            let abs_step = step.abs() as usize;
                            if actual_start < actual_stop {
                                base.chars()
                                    .skip(actual_start)
                                    .take(actual_stop - actual_start)
                                    .rev()
                                    .step_by(abs_step)
                                    .collect::<String>()
                            } else {
                                String::new()
                            }
                        }
                    }
                })
            }

            // Start and step: s[start::step]
            // DEPYLER-1315: Clone base to prevent E0382 "use of moved value"
            (Some(start), None, Some(step)) => Ok(parse_quote! {
                {
                    let base = (#base_expr).clone();
                    let start_idx: i32 = #start;
                    let step: i32 = #step;
                    let len = base.chars().count() as i32;

                    let actual_start = if start_idx < 0 {
                        (len + start_idx).max(0) as usize
                    } else {
                        start_idx.min(len) as usize
                    };

                    if step == 1 {
                        base.chars().skip(actual_start).collect::<String>()
                    } else if step > 0 {
                        base.chars().skip(actual_start).step_by(step as usize).collect::<String>()
                    } else if step == -1 {
                        base.chars().skip(actual_start).rev().collect::<String>()
                    } else {
                        let abs_step = step.abs() as usize;
                        base.chars().skip(actual_start).rev().step_by(abs_step).collect::<String>()
                    }
                }
            }),

            // Stop and step: s[:stop:step]
            // DEPYLER-1315: Clone base to prevent E0382 "use of moved value"
            (None, Some(stop), Some(step)) => Ok(parse_quote! {
                {
                    let base = (#base_expr).clone();
                    let stop_idx: i32 = #stop;
                    let step: i32 = #step;
                    let len = base.chars().count() as i32;

                    let actual_stop = if stop_idx < 0 {
                        (len + stop_idx).max(0) as usize
                    } else {
                        stop_idx.min(len) as usize
                    };

                    if step == 1 {
                        base.chars().take(actual_stop).collect::<String>()
                    } else if step > 0 {
                        base.chars().take(actual_stop).step_by(step as usize).collect::<String>()
                    } else if step == -1 {
                        base.chars().take(actual_stop).rev().collect::<String>()
                    } else {
                        let abs_step = step.abs() as usize;
                        base.chars().take(actual_stop).rev().step_by(abs_step).collect::<String>()
                    }
                }
            }),
        }
    }
}
