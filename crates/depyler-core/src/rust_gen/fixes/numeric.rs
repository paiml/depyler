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
        collect_typed_var_from_line(line, &mut int_vars, &mut float_vars);
    }
    // Pass 2: propagate types through simple assignments
    for line in code.lines() {
        propagate_var_type_from_line(line, &mut int_vars, &mut float_vars);
    }
    // Pass 3: fix comparisons
    let lines: Vec<&str> = code.lines().collect();
    let mut result = Vec::with_capacity(lines.len());
    for line in &lines {
        result.push(fix_int_float_comparison_in_line(line, &int_vars, &float_vars));
    }
    result.join("\n")
}

/// Collect typed variable declarations from a single line into int/float sets.
fn collect_typed_var_from_line(
    line: &str,
    int_vars: &mut std::collections::HashSet<String>,
    float_vars: &mut std::collections::HashSet<String>,
) {
    let t = line.trim();
    if !t.starts_with("let ") {
        return;
    }
    let rest = &t[4..];
    // `let var: TYPE = ...`
    if let Some(colon) = rest.find(": ") {
        let var = rest[..colon].trim().trim_start_matches("mut ");
        let after_colon = &rest[colon + 2..];
        let type_name = after_colon.split([' ', '=', ';']).next().unwrap_or("");
        classify_type_name(type_name, var, int_vars, float_vars);
    }
    // `let var = expr as i32;`
    if let Some(eq) = rest.find(" = ") {
        let var = rest[..eq].trim().trim_start_matches("mut ");
        let rhs = &rest[eq + 3..];
        let rhs_trimmed = rhs.trim_end_matches(';');
        if rhs_trimmed.ends_with("as i32")
            || rhs_trimmed.ends_with("as i64")
            || rhs_trimmed.ends_with("as usize")
        {
            int_vars.insert(var.to_string());
        }
    }
}

/// Classify a type name and insert the variable into the appropriate set.
fn classify_type_name(
    type_name: &str,
    var: &str,
    int_vars: &mut std::collections::HashSet<String>,
    float_vars: &mut std::collections::HashSet<String>,
) {
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

/// Propagate types through simple assignments like `let var = other_var;`.
fn propagate_var_type_from_line(
    line: &str,
    int_vars: &mut std::collections::HashSet<String>,
    float_vars: &mut std::collections::HashSet<String>,
) {
    let t = line.trim();
    if !t.starts_with("let ") || t.contains(": ") {
        return;
    }
    let Some(eq) = t.find(" = ") else { return };
    let var = t[4..eq].trim().trim_start_matches("mut ");
    let rhs = t[eq + 3..].trim().trim_end_matches(';').trim();
    if int_vars.contains(rhs) {
        int_vars.insert(var.to_string());
    } else if float_vars.contains(rhs) {
        float_vars.insert(var.to_string());
    }
}

/// Fix a single comparison line by replacing f64 literals with int literals
/// when the LHS variable is a known integer type.
fn fix_int_float_comparison_in_line(
    line: &str,
    int_vars: &std::collections::HashSet<String>,
    float_vars: &std::collections::HashSet<String>,
) -> String {
    let mut fixed = line.to_string();
    let trimmed = fixed.trim_start();
    if !trimmed.starts_with("let ") {
        return fixed;
    }
    for op in &[" == ", " != ", " < ", " > ", " <= ", " >= "] {
        fixed = try_fix_f64_literal_for_op(&fixed, op, int_vars, float_vars);
    }
    fixed
}

/// Attempt to replace an f64 literal on the RHS of a comparison operator
/// with an integer literal, if the LHS variable is a known int.
fn try_fix_f64_literal_for_op(
    line: &str,
    op: &str,
    int_vars: &std::collections::HashSet<String>,
    float_vars: &std::collections::HashSet<String>,
) -> String {
    let Some(op_pos) = line.find(op) else {
        return line.to_string();
    };
    let before_op = line[..op_pos].trim();
    let lhs_var = before_op.rsplit([' ', '(']).next().unwrap_or("").trim();
    if !int_vars.contains(lhs_var) || float_vars.contains(lhs_var) {
        return line.to_string();
    }
    let after_op = op_pos + op.len();
    let rest = &line[after_op..];
    let lit_end = rest
        .find(|c: char| !c.is_ascii_digit() && c != '.' && c != 'f')
        .unwrap_or(rest.len());
    let literal = &rest[..lit_end];
    if !literal.ends_with("f64") || lit_end <= 3 {
        return line.to_string();
    }
    let int_lit = literal.trim_end_matches("f64").trim_end_matches('.');
    if int_lit.is_empty() {
        return line.to_string();
    }
    format!("{}{}{}", &line[..after_op], int_lit, &line[after_op + lit_end..])
}

pub(super) fn fix_mixed_numeric_min_max(code: &str) -> String {
    let mut int_vars: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut float_vars: std::collections::HashSet<String> = std::collections::HashSet::new();
    for line in code.lines() {
        collect_min_max_typed_var(line, &mut int_vars, &mut float_vars);
    }
    if int_vars.is_empty() || float_vars.is_empty() {
        return code.to_string();
    }
    let mut result = code.to_string();
    for func in &["depyler_min", "depyler_max"] {
        result = fix_all_min_max_calls_for_func(&result, func, &int_vars, &float_vars);
    }
    result
}

/// Collect int/float variable declarations for min/max fix from a single line.
fn collect_min_max_typed_var(
    line: &str,
    int_vars: &mut std::collections::HashSet<String>,
    float_vars: &mut std::collections::HashSet<String>,
) {
    let t = line.trim();
    if !t.starts_with("let ") {
        return;
    }
    let name = &t[4..];
    let types_and_sets: &[(&[&str], bool)] = &[
        (&["i32", "i64", "isize", "usize"], true),
        (&["f64", "f32"], false),
    ];
    for &(type_names, is_int) in types_and_sets {
        for type_name in type_names {
            let pat = format!(": {} ", type_name);
            if !t.contains(&pat) {
                continue;
            }
            let var = name.split(':').next().unwrap_or("").trim();
            let var = var.trim_start_matches("mut ");
            if !var.is_empty() {
                if is_int {
                    int_vars.insert(var.to_string());
                } else {
                    float_vars.insert(var.to_string());
                }
            }
        }
    }
}

/// Fix all occurrences of a single min/max function in the code.
fn fix_all_min_max_calls_for_func(
    code: &str,
    func: &str,
    int_vars: &std::collections::HashSet<String>,
    float_vars: &std::collections::HashSet<String>,
) -> String {
    let pattern = format!("{}(", func);
    let mut result = code.to_string();
    let mut search_from = 0;
    while let Some(pos) = result[search_from..].find(&pattern) {
        let abs_pos = search_from + pos;
        let args_start = abs_pos + pattern.len();
        if let Some(close) = find_matching_close(&result[args_start..]) {
            let args_str = result[args_start..args_start + close].to_string();
            result = try_cast_min_max_args(&result, func, &args_str, int_vars, float_vars);
        }
        search_from = abs_pos + pattern.len();
    }
    result
}

/// Try to insert an `as f64` cast on the int argument of a mixed int/float min/max call.
fn try_cast_min_max_args(
    code: &str,
    func: &str,
    args_str: &str,
    int_vars: &std::collections::HashSet<String>,
    float_vars: &std::collections::HashSet<String>,
) -> String {
    let Some(comma) = find_top_level_comma(args_str) else {
        return code.to_string();
    };
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
    let a1_is_float = float_vars.contains(a1_base.trim());
    let a2_is_int = int_vars.contains(a2_base.trim());
    let a2_is_float = float_vars.contains(a2_base.trim());
    if a1_is_int && a2_is_float {
        let old_call = format!("{}({}, {})", func, arg1, arg2);
        let new_call = format!("{}({} as f64, {})", func, arg1, arg2);
        return code.replacen(&old_call, &new_call, 1);
    }
    if a1_is_float && a2_is_int {
        let old_call = format!("{}({}, {})", func, arg1, arg2);
        let new_call = format!("{}({}, {} as f64)", func, arg1, arg2);
        return code.replacen(&old_call, &new_call, 1);
    }
    code.to_string()
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
        if !is_arith_paren_line(lines[start]) {
            continue;
        }
        let initial_depth = compute_paren_depth(lines[start]);
        if let Some(fix_idx) = find_trailing_comma_in_block(&lines, start, initial_depth) {
            lines_to_fix.push(fix_idx);
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

/// Check if a line ends with an arithmetic operator followed by `(`.
fn is_arith_paren_line(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.ends_with("- (")
        || trimmed.ends_with("+ (")
        || trimmed.ends_with("* (")
        || trimmed.ends_with("/ (")
}

/// Compute the net paren depth at the end of a line.
fn compute_paren_depth(line: &str) -> i32 {
    let mut depth = 0i32;
    for ch in line.chars() {
        match ch {
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => {}
        }
    }
    depth
}

/// Scan lines after `start` to find a trailing comma that creates a single-element tuple.
/// Returns the line index of the trailing comma to fix, if any.
fn find_trailing_comma_in_block(lines: &[&str], start: usize, initial_depth: i32) -> Option<usize> {
    let arith_inner = initial_depth;
    let target = initial_depth - 1;
    let mut depth = initial_depth;
    let mut j = start + 1;
    let mut last_trailing_comma: Option<usize> = None;
    let mut comma_count = 0u32;

    while j < lines.len() {
        if scan_line_for_close(lines[j], &mut depth, target) {
            return if comma_count == 1 { last_trailing_comma } else { None };
        }
        if lines[j].trim().ends_with(',') && depth == arith_inner {
            last_trailing_comma = Some(j);
            comma_count += 1;
        }
        j += 1;
    }
    None
}

/// Scan a line's chars for paren depth changes. Returns true if the target depth is reached.
fn scan_line_for_close(line: &str, depth: &mut i32, target: i32) -> bool {
    for ch in line.chars() {
        match ch {
            '(' => *depth += 1,
            ')' => {
                *depth -= 1;
                if *depth == target {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
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
        if let Some(fixed) = try_annotate_negative_literal(line) {
            result.push_str(&fixed);
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }
    result
}

/// Try to add `: i32` type annotation to a `let VAR = -DIGITS;` line.
/// Returns `Some(fixed_line)` if the annotation was added, `None` otherwise.
fn try_annotate_negative_literal(line: &str) -> Option<String> {
    let trimmed = line.trim();
    let pfx = detect_let_prefix(trimmed)?;
    let after = &trimmed[pfx.len()..];
    let eq_pos = after.find(" = ")?;
    let var_part = &after[..eq_pos];
    if var_part.contains(':') {
        return None;
    }
    let rhs = after[eq_pos + 3..].trim();
    if !is_negative_integer_literal(rhs) {
        return None;
    }
    let indent = &line[..line.len() - trimmed.len()];
    Some(format!("{}{}{}: i32 = {}", indent, pfx, var_part, rhs))
}

/// Detect the `let ` or `let mut ` prefix of a line.
fn detect_let_prefix(trimmed: &str) -> Option<&'static str> {
    if trimmed.starts_with("let mut ") {
        Some("let mut ")
    } else if trimmed.starts_with("let ") {
        Some("let ")
    } else {
        None
    }
}

/// Check if a string is a negative integer literal like `-1;` or `-42;`.
fn is_negative_integer_literal(rhs: &str) -> bool {
    let Some(stripped) = rhs.strip_prefix('-') else {
        return false;
    };
    let digits_part = stripped.trim_end_matches(';').trim();
    !digits_part.is_empty() && digits_part.chars().all(|c| c.is_ascii_digit())
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
