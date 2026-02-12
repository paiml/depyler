//! Numeric-related fix functions for transpiled Rust code.
//!
//! These functions handle float/int comparison fixes, type annotation
//! corrections for numeric literals, mixed numeric min/max operations,
//! spurious `.to_string()` removal on numeric fields, and other
//! numeric-specific post-processing transformations.

use super::depyler_value::find_matching_close;
use super::enums::find_top_level_comma;

pub(super) fn fix_float_int_comparison(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    for line in code.lines() {
        let fixed = fix_float_int_in_line(line);
        result.push_str(&fixed);
        result.push('\n');
    }
    // Remove trailing newline if original didn't have one
    if !code.ends_with('\n') {
        result.pop();
    }
    result
}

pub(super) fn fix_float_int_in_line(line: &str) -> String {
    let mut result = line.to_string();
    // Pattern: `field_name <= 0;` or `field_name <= 0 {` where field_name is a known float field
    let float_fields = [
        "beta",
        "learning_rate",
        "lr",
        "momentum",
        "weight_decay",
        "epsilon",
        "gamma",
        "alpha",
        "lambda",
        "temperature",
        "top_p",
        "top_k_float",
        "label_smoothing",
        "cliprange",
        "cliprange_value",
        "vf_coef",
        "max_grad_norm",
        "lam",
        "dropout",
        "warmup_ratio",
        "threshold",
        "score",
        "loss",
        "reward",
        "penalty",
        "decay",
        "rate",
        "ratio",
        "step_size",
        "min_lr",
        "max_lr",
        "diversity_penalty",
        "max_norm",
        "min_norm",
        "scale",
        "softmax_scale",
        "norm",
        "noise_std",
        "sample_rate",
        "confidence",
        "similarity",
        "distance",
        "tolerance",
        "probability",
        "weight",
        "bias",
        "margin",
        "entropy",
        "perplexity",
        "grad_norm",
        "clip_value",
        "frequency",
        "damping",
        "attenuation",
        "overlap",
        "gain",
        "spacing",
        "offset_val",
        "cutoff",
    ];
    for op in &["<= 0", ">= 0", "< 0", "> 0", "== 0", "!= 0"] {
        let float_op = op.replace(" 0", " 0.0");
        for field in &float_fields {
            let pattern = format!(".{} {}", field, op);
            if result.contains(&pattern) {
                let replacement = format!(".{} {}", field, float_op);
                result = result.replace(&pattern, &replacement);
            }
        }
    }
    // Also handle `<= 1`, `>= 1` for probability/ratio fields
    for op in &["<= 1", ">= 1", "< 1", "> 1", "== 1", "!= 1"] {
        let float_op = op.replace(" 1", " 1.0");
        for field in &[
            "dropout",
            "top_p",
            "label_smoothing",
            "warmup_ratio",
            "cliprange",
            "cliprange_value",
            "gamma",
            "lam",
            "ratio",
            "momentum",
            "probability",
            "confidence",
            "similarity",
            "alpha",
            "beta",
            "weight",
            "overlap",
            "tolerance",
        ] {
            let pattern = format!(".{} {}", field, op);
            if result.contains(&pattern) {
                let replacement = format!(".{} {}", field, float_op);
                result = result.replace(&pattern, &replacement);
            }
        }
    }
    result
}

pub(super) fn fix_cse_py_mul_type_annotation(code: &str) -> String {
    let mut result = code.to_string();
    for py_op in &[".py_mul(", ".py_div("] {
        let mut search_from = 0;
        while search_from < result.len() {
            let Some(op_pos) = result[search_from..].find(py_op) else {
                break;
            };
            let abs_pos = search_from + op_pos;
            let paren_start = abs_pos + py_op.len();
            if let Some(rel_close) = find_matching_close(&result[paren_start..]) {
                let block = &result[paren_start..paren_start + rel_close];
                let fixed_block = remove_into_after_unwrap_or_default(block);
                if fixed_block != block {
                    let before = &result[..paren_start];
                    let after = &result[paren_start + rel_close..];
                    result = format!("{}{}{}", before, fixed_block, after);
                }
            }
            search_from = abs_pos + py_op.len();
        }
    }
    result
}

/// Remove `.into()` that follows `.unwrap_or_default()` with optional whitespace between.
pub(super) fn remove_into_after_unwrap_or_default(block: &str) -> String {
    let target = ".unwrap_or_default()";
    let mut result = block.to_string();
    let mut search_from = 0;
    while let Some(pos) = result[search_from..].find(target) {
        let abs_end = search_from + pos + target.len();
        let remaining = &result[abs_end..];
        // Skip whitespace (including newlines)
        let ws_len = remaining.len() - remaining.trim_start().len();
        let after_ws = &remaining[ws_len..];
        if after_ws.starts_with(".into()") {
            // Remove the whitespace + `.into()`
            let remove_start = abs_end;
            let remove_end = abs_end + ws_len + ".into()".len();
            result = format!("{}{}", &result[..remove_start], &result[remove_end..]);
        }
        search_from = abs_end;
        if search_from >= result.len() {
            break;
        }
    }
    result
}

pub(super) fn fix_cse_int_float_comparison(code: &str) -> String {
    let mut int_vars: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut float_vars: std::collections::HashSet<String> = std::collections::HashSet::new();
    // Pass 1: collect typed variables
    for line in code.lines() {
        let t = line.trim();
        if !t.starts_with("let ") {
            continue;
        }
        let rest = &t[4..];
        // `let var: TYPE = ...`
        if let Some(colon) = rest.find(": ") {
            let var = rest[..colon].trim().trim_start_matches("mut ");
            let after_colon = &rest[colon + 2..];
            let type_name = after_colon.split([' ', '=', ';']).next().unwrap_or("");
            match type_name {
                "i32" | "i64" | "isize" | "usize" | "u32" | "u64" => {
                    int_vars.insert(var.to_string());
                }
                "f64" | "f32" => {
                    float_vars.insert(var.to_string());
                }
                _ => {}
            }
        }
        // `let var = expr as i32;`
        if let Some(eq) = rest.find(" = ") {
            let var = rest[..eq].trim().trim_start_matches("mut ");
            let rhs = &rest[eq + 3..];
            if rhs.trim_end_matches(';').ends_with("as i32")
                || rhs.trim_end_matches(';').ends_with("as i64")
                || rhs.trim_end_matches(';').ends_with("as usize")
            {
                int_vars.insert(var.to_string());
            }
        }
    }
    // Pass 2: propagate types through simple assignments
    // `let var = other_var;` or `let var = _cse_temp_N;`
    for line in code.lines() {
        let t = line.trim();
        if !t.starts_with("let ") || t.contains(": ") {
            continue;
        }
        if let Some(eq) = t.find(" = ") {
            let var = t[4..eq].trim().trim_start_matches("mut ");
            let rhs = t[eq + 3..].trim().trim_end_matches(';').trim();
            // Simple assignment from known var
            if int_vars.contains(rhs) {
                int_vars.insert(var.to_string());
            } else if float_vars.contains(rhs) {
                float_vars.insert(var.to_string());
            }
        }
    }
    // Pass 3: fix comparisons
    let lines: Vec<&str> = code.lines().collect();
    let mut result = Vec::with_capacity(lines.len());
    for line in &lines {
        let mut fixed = line.to_string();
        let trimmed = fixed.trim_start();
        if trimmed.starts_with("let ") {
            for op in &[" == ", " != ", " < ", " > ", " <= ", " >= "] {
                if let Some(op_pos) = fixed.find(op) {
                    // Extract LHS variable name
                    let before_op = fixed[..op_pos].trim();
                    let lhs_var = before_op.rsplit([' ', '(']).next().unwrap_or("").trim();
                    // Only fix if LHS is known integer (not float)
                    if int_vars.contains(lhs_var) && !float_vars.contains(lhs_var) {
                        let after_op = op_pos + op.len();
                        let rest = &fixed[after_op..];
                        let lit_end = rest
                            .find(|c: char| !c.is_ascii_digit() && c != '.' && c != 'f')
                            .unwrap_or(rest.len());
                        let literal = &rest[..lit_end];
                        if literal.ends_with("f64") && lit_end > 3 {
                            let int_lit = literal.trim_end_matches("f64").trim_end_matches('.');
                            if !int_lit.is_empty() {
                                let before = &fixed[..after_op];
                                let after = &fixed[after_op + lit_end..];
                                fixed = format!("{}{}{}", before, int_lit, after);
                            }
                        }
                    }
                }
            }
        }
        result.push(fixed);
    }
    result.join("\n")
}

pub(super) fn fix_mixed_numeric_min_max(code: &str) -> String {
    let mut int_vars: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut float_vars: std::collections::HashSet<String> = std::collections::HashSet::new();
    for line in code.lines() {
        let t = line.trim();
        for int_type in &["i32", "i64", "isize", "usize"] {
            let pat = format!(": {} ", int_type);
            if t.starts_with("let ") && t.contains(&pat) {
                if let Some(name) = t.strip_prefix("let ") {
                    let var = name.split(':').next().unwrap_or("").trim();
                    let var = var.trim_start_matches("mut ");
                    if !var.is_empty() {
                        int_vars.insert(var.to_string());
                    }
                }
            }
        }
        for float_type in &["f64", "f32"] {
            let pat = format!(": {} ", float_type);
            if t.starts_with("let ") && t.contains(&pat) {
                if let Some(name) = t.strip_prefix("let ") {
                    let var = name.split(':').next().unwrap_or("").trim();
                    let var = var.trim_start_matches("mut ");
                    if !var.is_empty() {
                        float_vars.insert(var.to_string());
                    }
                }
            }
        }
    }
    if int_vars.is_empty() || float_vars.is_empty() {
        return code.to_string();
    }
    let mut result = code.to_string();
    for func in &["depyler_min", "depyler_max"] {
        let pattern = format!("{}(", func);
        let mut search_from = 0;
        while let Some(pos) = result[search_from..].find(&pattern) {
            let abs_pos = search_from + pos;
            let args_start = abs_pos + pattern.len();
            if let Some(close) = find_matching_close(&result[args_start..]) {
                let args_str = result[args_start..args_start + close].to_string();
                // Split on the top-level comma
                if let Some(comma) = find_top_level_comma(&args_str) {
                    let arg1 = args_str[..comma].trim().to_string();
                    let arg2 = args_str[comma + 1..].trim().to_string();
                    let a1_base = arg1
                        .trim_start_matches('(')
                        .trim_end_matches(')')
                        .replace(".clone()", "");
                    let a2_base = arg2
                        .trim_start_matches('(')
                        .trim_end_matches(')')
                        .replace(".clone()", "");
                    let a1_is_int = int_vars.contains(a1_base.trim());
                    let a2_is_int = int_vars.contains(a2_base.trim());
                    let a1_is_float = float_vars.contains(a1_base.trim());
                    let a2_is_float = float_vars.contains(a2_base.trim());
                    if a1_is_int && a2_is_float {
                        let new_arg1 = format!("{} as f64", arg1);
                        let old_call = format!("{}({}, {})", func, arg1, arg2);
                        let new_call = format!("{}({}, {})", func, new_arg1, arg2);
                        result = result.replacen(&old_call, &new_call, 1);
                    } else if a1_is_float && a2_is_int {
                        let new_arg2 = format!("{} as f64", arg2);
                        let old_call = format!("{}({}, {})", func, arg1, arg2);
                        let new_call = format!("{}({}, {})", func, arg1, new_arg2);
                        result = result.replacen(&old_call, &new_call, 1);
                    }
                }
            }
            search_from = abs_pos + pattern.len();
        }
    }
    result
}

pub(super) fn fix_spurious_i64_conversion(code: &str) -> String {
    // NO-OP: Blanket .to_i64()/.as_i64() replacement breaks DepylerValue
    // method definitions in the preamble. Needs targeted fix scoped to
    // call sites only, not method definitions.
    code.to_string()
}

pub(super) fn has_result_return_multiline(lines: &[&str], start: usize) -> bool {
    for i in 1..=5 {
        let idx = start + i;
        if idx >= lines.len() {
            break;
        }
        let l = lines[idx].trim();
        if l.contains("-> Result<") {
            return true;
        }
        if l.ends_with('{') || l.starts_with("pub fn ") || l.starts_with("fn ") {
            break;
        }
    }
    false
}

/// Given a line and a prefix pattern like "Ok(fname(", find the closing
/// `)` of the `fname(...)` call and return its position in the line.
pub(super) fn find_call_close_paren(line: &str, prefix: &str, fname: &str) -> Option<usize> {
    let pat_pos = line.find(prefix)?;
    // Find the ( of the function call itself
    let fn_name_start = pat_pos + prefix.len() - fname.len() - 1;
    let after_name = &line[fn_name_start..];
    let fn_paren = after_name.find('(')?;
    let call_start = fn_name_start + fn_paren;
    let mut depth = 0;
    for (i, c) in line[call_start..].char_indices() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(call_start + i);
                }
            }
            _ => {}
        }
    }
    None
}

/// Fix single-element tuples created by trailing commas in arithmetic.
///
/// Pattern: `(left_max) - (\n  expr,\n)` creates `(i32,)` tuple.
/// The trailing comma before the closing `)` must be removed.
pub(super) fn fix_trailing_comma_in_arith_parens(code: &str) -> String {
    let lines: Vec<&str> = code.lines().collect();
    let mut output: Vec<String> = lines.iter().map(|l| l.to_string()).collect();
    let mut lines_to_fix: Vec<usize> = Vec::new();

    for start in 0..lines.len() {
        let trimmed = lines[start].trim();
        let is_arith = trimmed.ends_with("- (")
            || trimmed.ends_with("+ (")
            || trimmed.ends_with("* (")
            || trimmed.ends_with("/ (");
        if !is_arith {
            continue;
        }
        // Compute paren depth at end of start line
        let mut depth = 0i32;
        for ch in lines[start].chars() {
            match ch {
                '(' => depth += 1,
                ')' => depth -= 1,
                _ => {}
            }
        }
        let arith_inner = depth;
        let target = depth - 1;
        let mut j = start + 1;
        let mut last_trailing_comma: Option<usize> = None;
        let mut comma_count = 0u32;

        while j < lines.len() {
            let mut found_close = false;
            for ch in lines[j].chars() {
                match ch {
                    '(' => depth += 1,
                    ')' => {
                        depth -= 1;
                        if depth == target {
                            found_close = true;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            if found_close {
                if comma_count == 1 {
                    if let Some(fix) = last_trailing_comma {
                        lines_to_fix.push(fix);
                    }
                }
                break;
            }
            if lines[j].trim().ends_with(',') && depth == arith_inner {
                last_trailing_comma = Some(j);
                comma_count += 1;
            }
            j += 1;
        }
    }
    for &idx in &lines_to_fix {
        if let Some(pos) = output[idx].rfind(',') {
            output[idx].remove(pos);
        }
    }
    let mut result = output.join("\n");
    if code.ends_with('\n') {
        result.push('\n');
    }
    result
}

pub(super) fn fix_spurious_to_string_in_numeric_call(code: &str) -> String {
    if !code.contains(".to_string()") {
        return code.to_string();
    }
    let mut result = String::with_capacity(code.len());
    for line in code.lines() {
        let trimmed = line.trim();
        // Match: DepylerDate::new(self.year, self.month, self.day.to_string())
        // Also match partial: self.day.to_string()
        if trimmed.contains("::new(") && trimmed.contains(".to_string()") {
            // Check if this is a call where numeric fields have .to_string()
            let fixed = fix_numeric_field_to_string(line);
            result.push_str(&fixed);
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }
    if !code.ends_with('\n') {
        result.truncate(result.len().saturating_sub(1));
    }
    result
}

/// Remove .to_string() from self.field in ::new() calls where the field is numeric.
pub(super) fn fix_numeric_field_to_string(line: &str) -> String {
    // Common numeric field names in struct constructors
    let numeric_fields = [
        "self.day", "self.month", "self.year", "self.hour", "self.minute",
        "self.second", "self.width", "self.height", "self.x", "self.y",
        "self.index", "self.count", "self.size", "self.length", "self.age",
        "self.port", "self.timeout", "self.max_size", "self.min_size",
        "self.capacity", "self.priority", "self.weight", "self.score",
    ];
    let mut result = line.to_string();
    for field in &numeric_fields {
        let pattern = format!("{}.to_string()", field);
        if result.contains(&pattern) {
            result = result.replace(&pattern, field);
        }
    }
    result
}

pub(super) fn fix_usize_to_string_in_constructor(code: &str) -> String {
    if !code.contains(".to_string()") {
        return code.to_string();
    }
    let mut result = String::with_capacity(code.len());
    for line in code.lines() {
        let trimmed = line.trim();
        // Match pattern: `VAR.to_string())` at end of line where VAR is a known usize-like name
        // Common usize vars: end, start, len, size, idx, index, pos, offset, count
        let usize_vars = [
            "end", "start", "len", "size", "idx", "index", "pos", "offset", "count", "capacity",
        ];
        let mut new_line = line.to_string();
        for var in &usize_vars {
            // Replace `VAR.to_string()` with just `VAR` when inside a constructor/function call
            let pattern = format!("{}.to_string()", var);
            if trimmed.contains(&pattern) {
                // Only replace when this appears as a function argument (preceded by comma or open paren)
                let safe_pattern_comma = format!(", {}", pattern);
                let safe_pattern_paren = format!("({}", pattern);
                if new_line.contains(&safe_pattern_comma) {
                    new_line = new_line.replace(&safe_pattern_comma, &format!(", {}", var));
                } else if new_line.contains(&safe_pattern_paren) {
                    new_line = new_line.replace(&safe_pattern_paren, &format!("({}", var));
                }
            }
        }
        result.push_str(&new_line);
        result.push('\n');
    }
    result
}

pub(super) fn fix_negative_literal_type_annotation(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    for line in code.lines() {
        let trimmed = line.trim();
        // Match: `let VAR = -DIGITS;` without type annotation
        let prefix = if trimmed.starts_with("let mut ") {
            Some("let mut ")
        } else if trimmed.starts_with("let ") {
            Some("let ")
        } else {
            None
        };
        if let Some(pfx) = prefix {
            let after = &trimmed[pfx.len()..];
            // Check for `VAR = -DIGITS;` pattern (no colon = no type annotation)
            if let Some(eq_pos) = after.find(" = ") {
                let var_part = &after[..eq_pos];
                let rhs = after[eq_pos + 3..].trim();
                // Only if no existing type annotation (no `:` in var part)
                if !var_part.contains(':') {
                    // Check if RHS is a negative integer literal like `-1;` or `-1i32;`
                    if rhs.starts_with('-') {
                        let digits_part = rhs[1..].trim_end_matches(';').trim();
                        if !digits_part.is_empty()
                            && digits_part.chars().all(|c| c.is_ascii_digit())
                        {
                            let indent = &line[..line.len() - trimmed.len()];
                            let new_line = format!(
                                "{}{}{}: i32 = {}",
                                indent, pfx, var_part, rhs
                            );
                            result.push_str(&new_line);
                            result.push('\n');
                            continue;
                        }
                    }
                }
            }
        }
        result.push_str(line);
        result.push('\n');
    }
    result
}

pub(super) fn fix_floor_div_type_annotation(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    for line in code.lines() {
        let trimmed = line.trim();
        // Match: `let q = a / b;` or `let r = a % b;` without type annotation
        if (trimmed.starts_with("let q = ") || trimmed.starts_with("let r = "))
            && !trimmed.contains(':')
            && (trimmed.contains(" / ") || trimmed.contains(" % "))
            && trimmed.ends_with(';')
        {
            let indent = &line[..line.len() - trimmed.len()];
            let new_line = trimmed.replacen("let q = ", "let q: i32 = ", 1);
            let new_line = new_line.replacen("let r = ", "let r: i32 = ", 1);
            result.push_str(&format!("{}{}", indent, new_line));
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }
    result
}

pub(super) fn fix_i32_as_i64_cast(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    for line in code.lines() {
        let trimmed = line.trim();
        // Pattern: (Ni32) as i64 — should just be the integer
        if trimmed.contains("i32) as i64") {
            result.push_str(&line.replace("i32) as i64", "i32)"));
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }
    result
}
