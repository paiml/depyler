//! Truthiness-related fix functions for Python-to-Rust transpilation.
//!
//! These functions handle converting Python truthiness semantics (where non-empty
//! collections and strings are truthy) into proper Rust boolean expressions using
//! `.is_empty()` and explicit comparisons.

use super::depyler_value::extract_string_typed_vars;

pub(super) fn fix_python_truthiness(code: &str) -> String {
    // Extract identifiers known to be bool from function signatures
    let bool_vars = extract_bool_typed_vars(code);
    let lines: Vec<&str> = code.lines().collect();
    let mut result = Vec::with_capacity(lines.len());

    for line in &lines {
        let fixed = fix_truthiness_line(line, &bool_vars);
        result.push(fixed);
    }

    result.join("\n") + "\n"
}

/// Extract variable names that are typed as `bool` from function signatures,
/// local variable declarations, and loop variables over Vec<bool>.
pub(super) fn extract_bool_typed_vars(code: &str) -> Vec<String> {
    let mut vars = Vec::new();
    // Track parameter names that are Vec<bool> for loop variable inference
    let mut vec_bool_params: Vec<String> = Vec::new();

    for line in code.lines() {
        let trimmed = line.trim();
        // Extract from function signatures: `fn foo(x: bool, values: &Vec<bool>)`
        if trimmed.starts_with("fn ") || trimmed.starts_with("pub fn ") {
            extract_bool_vars_from_fn_sig(trimmed, &mut vars, &mut vec_bool_params);
            continue;
        }
        // DEPYLER-99MODE-S9: Extract from local variable declarations
        // Patterns: `let mut result: bool = ...` or `let result: bool = ...`
        if trimmed.starts_with("let ") {
            extract_bool_var_from_let(trimmed, &mut vars);
        }
        // DEPYLER-99MODE-S9: Extract loop variables over Vec<bool> params
        // Pattern: `for VAR in PARAM.iter()` where PARAM is Vec<bool>
        if trimmed.starts_with("for ") {
            extract_bool_var_from_for_loop(trimmed, &vec_bool_params, &mut vars);
        }
    }
    vars
}

/// Extract bool-typed variables and Vec<bool> params from a function signature line.
fn extract_bool_vars_from_fn_sig(
    trimmed: &str,
    vars: &mut Vec<String>,
    vec_bool_params: &mut Vec<String>,
) {
    let Some(start) = trimmed.find('(') else {
        return;
    };
    let Some(end) = trimmed.find(')') else {
        return;
    };
    let params = &trimmed[start + 1..end];
    for param in params.split(',') {
        let p = param.trim();
        extract_bool_param(p, vars);
        extract_vec_bool_param(p, vec_bool_params);
    }
}

/// Extract a single bool-typed parameter name from a parameter string like `x: bool`.
fn extract_bool_param(p: &str, vars: &mut Vec<String>) {
    if !p.ends_with(": bool") {
        return;
    }
    if let Some(name) = p.strip_suffix(": bool") {
        let name = name.trim();
        if !name.is_empty() {
            vars.push(name.to_string());
        }
    }
}

/// Extract a Vec<bool> parameter name for loop variable inference.
fn extract_vec_bool_param(p: &str, vec_bool_params: &mut Vec<String>) {
    // DEPYLER-99MODE-S9: Track Vec<bool> params for loop var inference
    if !p.contains("Vec<bool>") && !p.contains("Vec < bool >") {
        return;
    }
    if let Some(colon_pos) = p.find(':') {
        let name = p[..colon_pos].trim();
        if !name.is_empty() {
            vec_bool_params.push(name.to_string());
        }
    }
}

/// Extract a bool variable from a `let` declaration line.
fn extract_bool_var_from_let(trimmed: &str, vars: &mut Vec<String>) {
    let rest = trimmed.strip_prefix("let ").unwrap_or("");
    let rest = rest.strip_prefix("mut ").unwrap_or(rest);
    if let Some(colon_pos) = rest.find(": bool") {
        let name = rest[..colon_pos].trim();
        if !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            vars.push(name.to_string());
        }
    }
}

/// Extract a loop variable from a `for` loop if the iterator is a Vec<bool> param.
fn extract_bool_var_from_for_loop(
    trimmed: &str,
    vec_bool_params: &[String],
    vars: &mut Vec<String>,
) {
    let Some(in_pos) = trimmed.find(" in ") else {
        return;
    };
    let loop_var = trimmed[4..in_pos].trim();
    let iter_part = &trimmed[in_pos + 4..];
    for param in vec_bool_params {
        let matches_param = iter_part.starts_with(&format!("{param}."))
            || iter_part.starts_with(&format!("{param} "));
        let is_valid_ident =
            !loop_var.is_empty() && loop_var.chars().all(|c| c.is_alphanumeric() || c == '_');
        if matches_param && is_valid_ident {
            vars.push(loop_var.to_string());
        }
    }
}

/// Fix a single line's Python truthiness negation patterns.
///
/// Converts `if !identifier {` to `if identifier.is_empty() {` for non-boolean
/// identifiers. Skips known boolean prefixes (is_, has_, should_, etc.).
pub(super) fn fix_truthiness_line(line: &str, bool_vars: &[String]) -> String {
    let trimmed = line.trim();
    // Match pattern: `if !IDENT {` where IDENT is a simple variable name
    if let Some(rest) = trimmed.strip_prefix("if !") {
        if let Some(ident) = rest.strip_suffix(" {") {
            if is_likely_non_boolean_ident(ident, bool_vars) {
                let indent = &line[..line.len() - line.trim_start().len()];
                return format!("{}if {}.is_empty() {{", indent, ident);
            }
        }
    }
    line.to_string()
}

/// Check if an identifier is likely NOT a boolean.
pub(super) fn is_likely_non_boolean_ident(ident: &str, bool_vars: &[String]) -> bool {
    // Must be a simple identifier (no dots, parens, brackets, spaces)
    if ident.contains('.') || ident.contains('(') || ident.contains('[') || ident.contains(' ') {
        return false;
    }
    // Skip variables known to be bool from type annotations
    if bool_vars.iter().any(|v| v == ident) {
        return false;
    }
    // Skip known boolean prefixes
    let bool_prefixes = [
        "is_",
        "has_",
        "should_",
        "can_",
        "will_",
        "was_",
        "did_",
        "does_",
        "are_",
        "do_",
        "were_",
        "ok",
        "err",
        "found",
        "done",
        "valid",
        "enabled",
        "disabled",
        "active",
        "ready",
        "empty",
        "full",
        "true",
        "false",
        "success",
        "failed",
        "_cse_temp",
    ];
    for prefix in &bool_prefixes {
        if ident.starts_with(prefix) || ident == *prefix {
            return false;
        }
    }
    // Must be a valid Rust identifier
    ident.chars().all(|c| c.is_alphanumeric() || c == '_')
}

pub(super) fn fix_negation_on_non_bool(code: &str) -> String {
    if !code.contains("!") {
        return code.to_string();
    }
    let string_typed_vars = extract_string_typed_vars(code);
    if string_typed_vars.is_empty() {
        return code.to_string();
    }
    // DEPYLER-99MODE-S9: Exclude variables that are also bool-typed in other scopes.
    // A variable like `v` may be `String` in a From impl and `bool` in a loop.
    // When ambiguous, do NOT rewrite - leaving `!v` for a bool is correct.
    let bool_vars = extract_bool_typed_vars(code);
    let string_typed_vars: Vec<String> =
        string_typed_vars.into_iter().filter(|v| !bool_vars.contains(v)).collect();
    if string_typed_vars.is_empty() {
        return code.to_string();
    }
    // Sort by length descending to match longer names first (avoid substring matches)
    let mut sorted_vars = string_typed_vars.clone();
    sorted_vars.sort_by_key(|b: &String| std::cmp::Reverse(b.len()));
    let lines: Vec<&str> = code.lines().collect();
    let mut result: Vec<String> = Vec::with_capacity(lines.len());
    for line in &lines {
        let fixed = replace_negation_with_is_empty(line, &sorted_vars);
        result.push(fixed);
    }
    result.join("\n")
}

/// Replace `!var` with `var.is_empty()` for string-typed variables in a single line.
fn replace_negation_with_is_empty(line: &str, sorted_vars: &[String]) -> String {
    let mut fixed = line.to_string();
    for var in sorted_vars {
        let neg_pattern = format!("!{}", var);
        // Check that the char AFTER the var name is not alphanumeric (word boundary)
        let Some(pos) = fixed.find(&neg_pattern) else {
            continue;
        };
        let after_pos = pos + neg_pattern.len();
        let next_char = fixed[after_pos..].chars().next();
        // DEPYLER-99MODE-S9: Don't replace `!var.method()` - that's boolean negation
        // of the method result, NOT truthiness of the variable.
        let is_word_boundary =
            next_char.map(|c| !c.is_alphanumeric() && c != '_' && c != '.').unwrap_or(true);
        if is_word_boundary {
            let empty_check = format!("{}.is_empty()", var);
            fixed = format!("{}{}{}", &fixed[..pos], empty_check, &fixed[after_pos..]);
        }
    }
    fixed
}

pub(super) fn fix_field_access_truthiness(code: &str) -> String {
    let lines: Vec<&str> = code.lines().collect();
    let mut result: Vec<String> = Vec::with_capacity(lines.len());
    for line in &lines {
        let fixed = fix_field_negation_in_line(line);
        result.push(fixed);
    }
    result.join("\n")
}

pub(super) fn fix_field_negation_in_line(line: &str) -> String {
    let mut result = line.to_string();
    loop {
        let current = result.clone();
        if let Some(replacement) = find_and_replace_field_negation(&current) {
            result = replacement;
        } else {
            break;
        }
    }
    result
}

pub(super) fn find_and_replace_field_negation(line: &str) -> Option<String> {
    let bytes = line.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    while i < len {
        if bytes[i] == b'!' {
            if let Some(replacement) = try_replace_field_negation_at(line, i) {
                return Some(replacement);
            }
        }
        i += 1;
    }
    None
}

/// Try to replace a `!ident.field` negation starting at position `bang_pos`.
/// Returns `Some(replaced_line)` if a replacement was made, `None` otherwise.
fn try_replace_field_negation_at(line: &str, bang_pos: usize) -> Option<String> {
    let bytes = line.as_bytes();
    let len = bytes.len();

    if !is_valid_negation_prefix(bytes, bang_pos) {
        return None;
    }

    // Parse first identifier after `!`
    let (_ident1, dot_pos) = parse_ident_before_dot(bytes, line, bang_pos + 1, len)?;

    // Parse second identifier (field) after `.`
    let field_start = dot_pos + 1;
    let (field, field_end): (&str, usize) = parse_field_name(bytes, line, field_start, len)?;

    // Check what follows: if `(` it's a method call - skip
    if field_end < len && bytes[field_end] == b'(' {
        return None;
    }
    // Also skip if it's another `.` (chained access like `!self.field.method()`)
    if field_end < len && bytes[field_end] == b'.' {
        return None;
    }
    // DEPYLER-99MODE-S9: Skip numeric tuple indices (e.g., `!bf_result.1`)
    // Tuple field access like `.0`, `.1` returns the element type (often bool),
    // not a collection. Applying `.is_empty()` to a bool is E0599.
    if field.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    // Skip known bool-returning or bool-typed fields
    if is_likely_bool_field(field) {
        return None;
    }

    // Replace `!ident.field` with `ident.field.is_empty()`
    let ident1 = &line[bang_pos + 1..dot_pos];
    let replacement =
        format!("{}{}.{}.is_empty(){}", &line[..bang_pos], ident1, field, &line[field_end..]);
    Some(replacement)
}

/// Check if the byte before `bang_pos` is a valid prefix for negation.
fn is_valid_negation_prefix(bytes: &[u8], bang_pos: usize) -> bool {
    bang_pos == 0 || matches!(bytes[bang_pos - 1], b' ' | b'(' | b'=' | b'{' | b'|' | b'&' | b'\t')
}

/// Parse an identifier starting at `start`, returning `(ident_str, dot_position)`.
/// Returns `None` if no valid identifier followed by `.` is found.
fn parse_ident_before_dot<'a>(
    bytes: &[u8],
    line: &'a str,
    start: usize,
    len: usize,
) -> Option<(&'a str, usize)> {
    let mut j = start;
    while j < len && (bytes[j].is_ascii_alphanumeric() || bytes[j] == b'_') {
        j += 1;
    }
    if j == start || j >= len || bytes[j] != b'.' {
        return None;
    }
    Some((&line[start..j], j))
}

/// Parse a field name starting at `start`, returning `(field_str, end_position)`.
/// Returns `None` if no valid field name is found.
fn parse_field_name<'a>(
    bytes: &[u8],
    line: &'a str,
    start: usize,
    len: usize,
) -> Option<(&'a str, usize)> {
    let mut j = start;
    while j < len && (bytes[j].is_ascii_alphanumeric() || bytes[j] == b'_') {
        j += 1;
    }
    if j == start {
        return None;
    }
    Some((&line[start..j], j))
}

/// DEPYLER-CONVERGE-MULTI-ITER9: Generalize DepylerValue insert wrapping.
///
/// Extends beyond just `map.insert()` to handle `kwargs.insert()`,
/// `config.insert()`, `params.insert()`, and other common variable names.
pub(super) fn is_likely_bool_field(field: &str) -> bool {
    const BOOL_PREFIXES: &[&str] = &[
        "is_",
        "has_",
        "should_",
        "can_",
        "enable",
        "disable",
        "use_",
        "load_in_",
        "allow_",
        "do_",
        "with_",
        "no_",
        "skip_",
        "force_",
        "apply_",
        "generate_",
        "include_",
        "exclude_",
    ];
    const BOOL_SUFFIXES: &[&str] = &["_enabled", "_flag", "_only"];
    const BOOL_EXACT: &[&str] =
        &["verbose", "debug", "quiet", "overwrite", "resume", "fp16", "bf16"];

    BOOL_PREFIXES.iter().any(|p| field.starts_with(p))
        || BOOL_SUFFIXES.iter().any(|s| field.ends_with(s))
        || BOOL_EXACT.iter().any(|e| field == *e)
}

pub(super) fn fix_not_string_truthiness(code: &str) -> String {
    let mut result = code.to_string();
    // Pattern: `(!expr.trim().to_string())` -> `expr.trim().is_empty()`
    result = fix_not_trim_to_string(&result);
    // Pattern: `(!expr.to_string())` -> `expr.is_empty()`
    result = fix_not_to_string(&result);
    result
}

pub(super) fn fix_not_trim_to_string(code: &str) -> String {
    let mut result = code.to_string();
    let pattern = ".trim().to_string())";
    while let Some(end_pos) = result.find(pattern) {
        // Walk backwards to find the `(!` that starts this expression
        let before = &result[..end_pos];
        if let Some(start) = before.rfind("(!") {
            let expr = &result[start + 2..end_pos];
            // Only fix if expr looks like a variable/field access
            if expr.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '.') {
                let old = format!("(!{}{})", expr, ".trim().to_string()");
                let new = format!("{}.trim().is_empty()", expr);
                result = result.replacen(&old, &new, 1);
                continue;
            }
        }
        break;
    }
    result
}

pub(super) fn fix_not_to_string(code: &str) -> String {
    let mut result = code.to_string();
    let marker = ".to_string())";
    let mut search_from = 0;
    while search_from < result.len() {
        let haystack = &result[search_from..];
        let Some(rel_pos) = haystack.find(marker) else {
            break;
        };
        let end_pos = search_from + rel_pos;
        let before = &result[..end_pos];
        if let Some(start) = before.rfind("(!") {
            let expr = &result[start + 2..end_pos];
            if expr.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '.') {
                let old = format!("(!{}.to_string())", expr);
                if result[start..].starts_with(&old) {
                    let new = format!("{}.is_empty()", expr);
                    result = format!("{}{}{}", &result[..start], new, &result[start + old.len()..]);
                    search_from = start + new.len();
                    continue;
                }
            }
        }
        search_from = end_pos + marker.len();
    }
    result
}

pub(super) fn fix_bitwise_and_truthiness(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    for line in code.lines() {
        let fixed = fix_bitwise_and_line(line);
        result.push_str(&fixed);
        result.push('\n');
    }
    if code.ends_with('\n') {
        result
    } else {
        result.truncate(result.len().saturating_sub(1));
        result
    }
}

/// Process a single line for bitwise AND truthiness conversion.
/// Converts `if EXPR & MASK {` to `if (EXPR & MASK) != 0 {` when appropriate.
fn fix_bitwise_and_line(line: &str) -> String {
    let trimmed = line.trim();
    if !is_bitwise_and_candidate(trimmed) {
        return line.to_string();
    }
    // Extract: "if EXPR {"
    let Some(rest) = trimmed.strip_prefix("if ") else {
        return line.to_string();
    };
    let Some(expr) = rest.strip_suffix('{') else {
        return line.to_string();
    };
    let expr = expr.trim();
    // Only fix if expr contains & and looks like bitwise
    if !expr.contains(" & ") {
        return line.to_string();
    }
    let indent = line.len() - line.trim_start().len();
    let pad: String = " ".repeat(indent);
    format!("{}if ({}) != 0 {{", pad, expr)
}

/// Check if a trimmed line is a candidate for bitwise AND truthiness fix.
fn is_bitwise_and_candidate(trimmed: &str) -> bool {
    trimmed.starts_with("if ")
        && trimmed.contains(" & ")
        && trimmed.ends_with('{')
        && !trimmed.contains("!=")
        && !trimmed.contains("==")
        && !trimmed.contains("&&")
        && !trimmed.contains("||")
        // DEPYLER-99MODE-S9: Skip lines with borrow patterns (& var)
        // to avoid confusing Rust borrow `& var` with bitwise AND `x & mask`
        && !trimmed.contains("(& ")
        && !trimmed.contains(", & ")
}
