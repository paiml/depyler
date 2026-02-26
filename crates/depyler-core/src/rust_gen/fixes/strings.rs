//! String-related post-generation fix functions for transpiled Rust code.

pub(super) fn fix_str_params_in_new_calls(code: &str) -> String {
    let lines: Vec<&str> = code.lines().collect();
    let str_params = collect_str_param_names(&lines);
    if str_params.is_empty() {
        return code.to_string();
    }
    apply_to_string_in_new_calls(code, &str_params)
}

pub(super) fn collect_str_param_names(lines: &[&str]) -> Vec<String> {
    let mut params = Vec::new();
    for line in lines {
        let trimmed = line.trim();
        // Match function parameter lines like `name: &str,` or `name: &'a str,`
        if (trimmed.contains(": &str") || trimmed.contains(": &'")) && trimmed.ends_with(',') {
            let colon = match trimmed.find(':') {
                Some(c) => c,
                None => continue,
            };
            let name = trimmed[..colon].trim().trim_start_matches("mut ");
            if name.chars().all(|c| c.is_alphanumeric() || c == '_') && !name.is_empty() {
                params.push(name.to_string());
            }
        }
    }
    params.sort();
    params.dedup();
    params
}

/// Check if a line starts or ends an ops impl block, updating tracking state.
fn update_ops_impl_state(line: &str, trimmed: &str, in_ops_impl: &mut bool) {
    if trimmed.starts_with("impl std::ops::") || trimmed.starts_with("impl DepylerTimeDelta") {
        *in_ops_impl = true;
    }
    if *in_ops_impl && trimmed == "}" && !line.starts_with(' ') && !line.starts_with('\t') {
        *in_ops_impl = false;
    }
}

/// Replace bare str param references with .to_string() variants in a new-call line.
fn apply_param_to_string(line: &str, params: &[String]) -> String {
    let mut fixed = line.to_string();
    for param in params {
        let trailing_comma = format!("{},", param);
        let trailing_paren = format!("{})", param);
        if fixed.contains(&trailing_comma) {
            let repl = format!("{}.to_string(),", param);
            fixed = fixed.replace(&trailing_comma, &repl);
        }
        if fixed.contains(&trailing_paren) {
            let repl = format!("{}.to_string())", param);
            fixed = fixed.replace(&trailing_paren, &repl);
        }
    }
    fixed
}

/// Count the net paren balance change for a line.
fn paren_balance_delta(line: &str) -> i32 {
    line.chars().fold(0, |acc, ch| match ch {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc,
    })
}

pub(super) fn apply_to_string_in_new_calls(code: &str, params: &[String]) -> String {
    let mut result = String::with_capacity(code.len());
    let mut in_new_call = false;
    let mut paren_depth: i32 = 0;
    let mut in_ops_impl = false;
    for line in code.lines() {
        let trimmed = line.trim();
        update_ops_impl_state(line, trimmed, &mut in_ops_impl);
        let mut fixed_line = line.to_string();
        if line.contains("::new(") && !in_ops_impl {
            in_new_call = true;
            paren_depth = 0;
        }
        if in_new_call {
            paren_depth += paren_balance_delta(line);
            fixed_line = apply_param_to_string(line, params);
            if paren_depth <= 0 {
                in_new_call = false;
            }
        }
        result.push_str(&fixed_line);
        result.push('\n');
    }
    if !code.ends_with('\n') {
        result.pop();
    }
    result
}

pub(super) fn fix_string_array_contains(code: &str) -> String {
    let mut result = code.to_string();
    // Find patterns like: ["x".to_string(), "y".to_string()].contains(
    let marker = ".to_string()].contains(";
    while let Some(contains_pos) = result.find(marker) {
        // Find the opening bracket for this array
        let bracket_search = &result[..contains_pos];
        let Some(open_bracket) = bracket_search.rfind('[') else {
            break;
        };
        let array_content = &result[open_bracket + 1..contains_pos + ".to_string()".len()];
        // Check if all elements are "string".to_string() patterns
        if !array_content.contains(".to_string()") {
            break;
        }
        // Strip .to_string() from each element
        let stripped = array_content.replace(".to_string()", "");
        // Find the closing paren of .contains(arg)
        let after_contains = contains_pos + marker.len();
        let Some(close_paren) = result[after_contains..].find(')') else {
            break;
        };
        let arg = result[after_contains..after_contains + close_paren].trim();
        // Build replacement: ["x", "y"].contains(&arg)
        let old_end = after_contains + close_paren + 1;
        let new_expr = format!("[{}].contains(&{})", stripped, arg);
        result = format!("{}{}{}", &result[..open_bracket], new_expr, &result[old_end..]);
    }
    result
}

pub(super) fn fix_regex_match_string_arg(code: &str) -> String {
    let target = "DepylerRegexMatch::new(";
    if !code.contains(target) {
        return code.to_string();
    }
    let mut result = String::with_capacity(code.len());
    for line in code.lines() {
        if line.contains(target) && line.contains(".to_string()") {
            let fixed = fix_regex_match_line(line, target);
            result.push_str(&fixed);
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }
    if code.ends_with('\n') {
        result
    } else {
        result.truncate(result.len().saturating_sub(1));
        result
    }
}

/// Fix a single line with DepylerRegexMatch::new(x.to_string(), ...).
pub(super) fn fix_regex_match_line(line: &str, target: &str) -> String {
    let idx = match line.find(target) {
        Some(i) => i,
        None => return line.to_string(),
    };
    let after_new = &line[idx + target.len()..];
    // Check if first arg has .to_string() and isn't already &
    if after_new.trim_start().starts_with('&') {
        return line.to_string();
    }
    if let Some(ts_pos) = after_new.find(".to_string()") {
        // Ensure .to_string() is in the first arg (before first comma at depth 0)
        let before_ts = &after_new[..ts_pos];
        let has_comma = before_ts.chars().any(|c| c == ',');
        if !has_comma {
            // Insert & before the first argument
            let insert_pos = idx + target.len();
            return format!("{}&{}", &line[..insert_pos], &line[insert_pos..]);
        }
    }
    line.to_string()
}

/// Try to fix a multi-line format!(...)\n.expect("...") pattern.
/// Returns Some(fixed_line, lines_consumed) on success.
fn try_fix_multiline_format_expect(cur: &str, next: &str) -> Option<(String, usize)> {
    let next_trimmed = next.trim();
    let expect_rest = next_trimmed.strip_prefix(".expect(")?;
    let fmt_pos = cur.find("format!(")?;
    let after_format = &cur[fmt_pos + 7..]; // after "format!"
    let paren_balance: i32 = after_format.chars().fold(0, |acc, c| match c {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc,
    });
    if paren_balance != 0 || !cur.trim_end().ends_with(')') {
        return None;
    }
    let close = find_matching_paren(expect_rest)?;
    let after_expect = expect_rest[close + 1..].trim();
    Some((format!("{}{}", cur.trim_end(), after_expect), 2))
}

/// Try to fix a single-line format!(...).expect("...") pattern.
/// Returns Some(fixed_line) on success.
fn try_fix_single_line_format_expect(line: &str) -> Option<String> {
    if !line.contains("format!(") || !line.contains(").expect(") {
        return None;
    }
    let fmt_pos = line.find("format!(")?;
    let after_fmt_open = &line[fmt_pos + 8..]; // after "format!("
    let close_idx = find_matching_paren(after_fmt_open)?;
    let abs_close = fmt_pos + 8 + close_idx;
    let after_close = &line[abs_close + 1..];
    let in_expect = after_close.strip_prefix(".expect(")?;
    let exp_close = find_matching_paren(in_expect)?;
    let rest = &in_expect[exp_close + 1..];
    Some(format!("{}{}", &line[..abs_close + 1], rest))
}

pub(super) fn fix_format_expect(code: &str) -> String {
    if !code.contains("format!") || !code.contains(".expect(") {
        return code.to_string();
    }
    let lines: Vec<&str> = code.lines().collect();
    let mut result = Vec::with_capacity(lines.len());
    let mut i = 0;

    while i < lines.len() {
        if i + 1 < lines.len() {
            if let Some((fixed, consumed)) = try_fix_multiline_format_expect(lines[i], lines[i + 1])
            {
                result.push(fixed);
                i += consumed;
                continue;
            }
        }

        if let Some(fixed) = try_fix_single_line_format_expect(lines[i]) {
            result.push(fixed);
            i += 1;
            continue;
        }

        result.push(lines[i].to_string());
        i += 1;
    }

    let joined = result.join("\n");
    if code.ends_with('\n') {
        joined + "\n"
    } else {
        joined
    }
}

/// Advance the paren-matching state machine by one character.
/// Returns `Some(true)` if the matching close paren is found,
/// `Some(false)` to continue, or updates state in place.
fn advance_paren_state(
    c: char,
    depth: &mut i32,
    in_string: &mut bool,
    escape_next: &mut bool,
) -> Option<bool> {
    if *escape_next {
        *escape_next = false;
        return Some(false);
    }
    if c == '\\' {
        *escape_next = true;
        return Some(false);
    }
    if c == '"' {
        *in_string = !*in_string;
        return Some(false);
    }
    if *in_string {
        return Some(false);
    }
    match c {
        '(' => *depth += 1,
        ')' => {
            *depth -= 1;
            if *depth == 0 {
                return Some(true);
            }
        }
        _ => {}
    }
    Some(false)
}

/// Find the matching closing paren in a string starting after an open paren.
pub(super) fn find_matching_paren(s: &str) -> Option<usize> {
    let mut depth = 1;
    let mut in_string = false;
    let mut escape_next = false;
    for (i, c) in s.char_indices() {
        if let Some(found) = advance_paren_state(c, &mut depth, &mut in_string, &mut escape_next) {
            if found {
                return Some(i);
            }
        }
    }
    None
}

/// Try to extract a &str parameter name from a single param fragment like `name: &str`.
fn try_extract_str_param_name(param_fragment: &str) -> Option<String> {
    let p = param_fragment.trim();
    if !p.contains("str") || !p.contains('&') {
        return None;
    }
    let colon = p.find(':')?;
    let name = p[..colon].trim().trim_start_matches("mut ");
    if name.is_empty() || !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }
    Some(name.to_string())
}

/// Extract &str parameter names from a function signature line.
fn extract_str_params_from_sig(trimmed: &str) -> Vec<String> {
    let start = match trimmed.find('(') {
        Some(s) => s,
        None => return Vec::new(),
    };
    let end = match trimmed.rfind(')') {
        Some(e) => e,
        None => return Vec::new(),
    };
    let param_str = &trimmed[start + 1..end];
    param_str.split(',').filter_map(try_extract_str_param_name).collect()
}

/// Check if a line is a bare expression return of a &str param and fix it.
fn try_fix_bare_return(line: &str, trimmed: &str, params: &[String]) -> Option<String> {
    let indent = &line[..line.len() - trimmed.len()];
    for param in params {
        if trimmed == param.as_str() || trimmed == format!("{};", param) {
            return Some(format!("{}{}.to_string()", indent, trimmed.trim_end_matches(';')));
        }
        let ret_pattern = format!("return {};", param);
        let ret_bare = format!("return {}", param);
        if trimmed == ret_pattern || trimmed == ret_bare {
            return Some(format!("{}return {}.to_string();", indent, param));
        }
    }
    None
}

/// Returns true if the line starts a function that returns String.
fn is_fn_returning_string(trimmed: &str) -> bool {
    (trimmed.starts_with("fn ") || trimmed.starts_with("pub fn ")) && trimmed.contains("-> String")
}

pub(super) fn fix_str_param_return_as_string(code: &str) -> String {
    let lines: Vec<&str> = code.lines().collect();
    let mut result = Vec::with_capacity(lines.len());
    let mut current_fn_returns_string = false;
    let mut current_fn_str_params: Vec<String> = Vec::new();

    for line in &lines {
        let trimmed = line.trim();

        if is_fn_returning_string(trimmed) {
            current_fn_returns_string = true;
            current_fn_str_params = extract_str_params_from_sig(trimmed);
        }

        if trimmed.starts_with("fn ") && !trimmed.contains("-> String") {
            current_fn_returns_string = false;
            current_fn_str_params.clear();
        }

        if current_fn_returns_string && !current_fn_str_params.is_empty() {
            if let Some(fixed_line) = try_fix_bare_return(line, trimmed, &current_fn_str_params) {
                result.push(fixed_line);
                continue;
            }
        }

        result.push(line.to_string());
    }

    let joined = result.join("\n");
    if code.ends_with('\n') {
        joined + "\n"
    } else {
        joined
    }
}

pub(super) fn fix_from_utf8_lossy_string_arg(code: &str) -> String {
    if !code.contains("from_utf8_lossy") {
        return code.to_string();
    }
    // Collect variable names assigned from format!() that produce String (not Vec<u8>)
    let mut format_string_vars: Vec<String> = Vec::new();
    let lines: Vec<&str> = code.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if let Some(eq_idx) = trimmed.find(" = format!(") {
            // Check the NEXT line for .into_bytes() which would make it Vec<u8>
            let next_has_into_bytes =
                i + 1 < lines.len() && lines[i + 1].trim().starts_with(".into_bytes()");
            // Also check same line for .into_bytes() chain
            let same_line_into_bytes = trimmed[eq_idx..].contains(".into_bytes()");

            if !next_has_into_bytes && !same_line_into_bytes {
                let before_eq = &trimmed[..eq_idx];
                if let Some(name) = before_eq.split_whitespace().last() {
                    let clean_name = name.trim_end_matches(':');
                    format_string_vars.push(clean_name.to_string());
                }
            }
        }
    }
    if format_string_vars.is_empty() {
        return code.to_string();
    }
    let mut result = code.to_string();
    for var in &format_string_vars {
        let bad = format!("from_utf8_lossy(&{})", var);
        let good = format!("from_utf8_lossy({}.as_bytes())", var);
        result = result.replace(&bad, &good);
    }
    result
}

/// Check if a string is a simple integer literal (possibly negative).
fn is_simple_int_literal(expr: &str) -> bool {
    let digits = match expr.strip_prefix('-') {
        Some(stripped) => stripped,
        None => expr,
    };
    !digits.is_empty() && digits.chars().all(|c| c.is_ascii_digit())
}

/// Update vec_block tracking state and return whether we are inside a vec block.
fn update_vec_block_state(trimmed: &str, in_vec_block: &mut bool, vec_depth: &mut i32) {
    if trimmed.contains("vec![") {
        *in_vec_block = true;
        *vec_depth += trimmed.matches('[').count() as i32;
        *vec_depth -= trimmed.matches(']').count() as i32;
    } else if *in_vec_block {
        *vec_depth += trimmed.matches('[').count() as i32;
        *vec_depth -= trimmed.matches(']').count() as i32;
        if *vec_depth <= 0 {
            *in_vec_block = false;
            *vec_depth = 0;
        }
    }
}

/// Replace format!("{:?}", N) with N for integer literals in a line.
fn strip_debug_format_ints(line: &str) -> String {
    let pattern_start = "format!(\"{:?}\", ";
    let mut new_line = line.to_string();
    while let Some(start) = new_line.find(pattern_start) {
        let after = &new_line[start + pattern_start.len()..];
        let end = match after.find(')') {
            Some(e) => e,
            None => break,
        };
        let expr = after[..end].trim();
        if !is_simple_int_literal(expr) {
            break;
        }
        let full_match_end = start + pattern_start.len() + end + 1;
        new_line = format!("{}{}{}", &new_line[..start], expr, &new_line[full_match_end..]);
    }
    new_line
}

pub(super) fn fix_format_debug_in_int_vec(code: &str) -> String {
    let pattern_start = "format!(\"{:?}\", ";
    let mut result = String::with_capacity(code.len());
    let lines: Vec<&str> = code.lines().collect();
    let mut in_vec_block = false;
    let mut vec_depth: i32 = 0;
    for line in &lines {
        let trimmed = line.trim();
        update_vec_block_state(trimmed, &mut in_vec_block, &mut vec_depth);
        let is_in_vec = in_vec_block || trimmed.contains("vec![");
        if is_in_vec && line.contains(pattern_start) {
            result.push_str(&strip_debug_format_ints(line));
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }
    result
}

/// Extract &str parameter names from a function definition line.
fn extract_str_param_names(trimmed: &str) -> Vec<String> {
    let mut params = Vec::new();
    let paren_start = match trimmed.find('(') {
        Some(s) => s,
        None => return params,
    };
    let paren_end = match trimmed.rfind(')') {
        Some(e) => e,
        None => return params,
    };
    let params_str = &trimmed[paren_start + 1..paren_end];
    for param in params_str.split(',') {
        let param = param.trim();
        if param.contains("&str") || param.contains("&'") {
            if let Some(colon) = param.find(':') {
                let name = param[..colon].trim().to_string();
                params.push(name);
            }
        }
    }
    params
}

/// Check if a line needs clone-to-to_string replacement.
fn needs_clone_to_string_fix(trimmed: &str, str_params: &[String]) -> bool {
    if trimmed.contains("Vec<String>") && trimmed.contains(".clone()") {
        return true;
    }
    !str_params.is_empty()
        && trimmed.contains(".clone()")
        && (trimmed.contains(".push(") || trimmed.starts_with("vec!["))
}

pub(super) fn fix_str_clone_to_string(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    let lines: Vec<&str> = code.lines().collect();
    let mut str_params: Vec<String> = Vec::new();
    for line in &lines {
        let trimmed = line.trim();
        if trimmed.starts_with("pub fn ") || trimmed.starts_with("fn ") {
            str_params = extract_str_param_names(trimmed);
        }
        if needs_clone_to_string_fix(trimmed, &str_params) {
            let mut new_line = line.to_string();
            for param in &str_params {
                let pat = format!("{}.clone()", param);
                let rep = format!("{}.to_string()", param);
                new_line = new_line.replace(&pat, &rep);
            }
            result.push_str(&new_line);
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }
    result
}

pub(super) fn fix_string_as_ref_ambiguity(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    for line in code.lines() {
        // Pattern: `expr.as_ref().expect("value is None")` in File::create context
        // The .as_ref() on String is ambiguous. Just remove the whole .as_ref().expect(...)
        // chain and use the expr directly (String implements AsRef<Path>).
        if line.contains(".as_ref().expect(\"value is None\")") {
            result.push_str(&line.replace(".as_ref().expect(\"value is None\")", ""));
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }
    result
}

pub(super) fn fix_char_as_str(code: &str) -> String {
    code.replace(".as_str().unwrap_or_default().to_string()", ".to_string()")
}

/// Collect names of functions that return Vec types.
fn collect_vec_returning_fns(lines: &[&str]) -> Vec<String> {
    let mut fns = Vec::new();
    for line in lines {
        let trimmed = line.trim();
        let is_fn_def = trimmed.starts_with("pub fn ") || trimmed.starts_with("fn ");
        if !is_fn_def || !trimmed.contains("-> Vec<") {
            continue;
        }
        if let Some(name_start) = trimmed.find("fn ") {
            let after_fn = &trimmed[name_start + 3..];
            if let Some(paren) = after_fn.find('(') {
                let fn_name = after_fn[..paren].trim();
                if !fn_name.is_empty() {
                    fns.push(fn_name.to_string());
                }
            }
        }
    }
    fns
}

/// Collect names of variables declared with Vec type.
fn collect_vec_vars(lines: &[&str]) -> Vec<String> {
    let mut vars = Vec::new();
    for line in lines {
        let trimmed = line.trim();
        if !trimmed.contains(": Vec<") && !trimmed.contains(":Vec<") {
            continue;
        }
        if let Some(colon) = trimmed.find(':') {
            let before = trimmed[..colon].trim();
            let name = before
                .strip_prefix("let ")
                .unwrap_or(before)
                .trim()
                .trim_start_matches("mut ")
                .trim();
            if !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                vars.push(name.to_string());
            }
        }
    }
    vars
}

/// Replace all `{}` with `{:?}` if a format line references a Vec fn call or Vec variable.
fn replace_display_with_debug(line: &str, vec_fns: &[String], vec_vars: &[String]) -> String {
    let mut new_line = line.to_string();
    if !new_line.contains("format!(\"") || !new_line.contains("{}") {
        return new_line;
    }
    for fn_name in vec_fns {
        let call_pat = format!("{fn_name}(");
        if new_line.contains(&call_pat) && new_line.contains("{}") {
            new_line = new_line.replacen("{}", "{:?}", new_line.matches("{}").count());
            return new_line;
        }
    }
    for var in vec_vars {
        if new_line.contains("{}") && new_line.contains(var.as_str()) {
            new_line = new_line.replacen("{}", "{:?}", new_line.matches("{}").count());
            return new_line;
        }
    }
    new_line
}

pub(super) fn fix_format_vec_display(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    let lines: Vec<&str> = code.lines().collect();
    let vec_returning_fns = collect_vec_returning_fns(&lines);
    let vec_vars = collect_vec_vars(&lines);
    for line in &lines {
        let new_line = replace_display_with_debug(line, &vec_returning_fns, &vec_vars);
        result.push_str(&new_line);
        result.push('\n');
    }
    result
}
