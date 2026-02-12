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

pub(super) fn apply_to_string_in_new_calls(code: &str, params: &[String]) -> String {
    let mut result = String::with_capacity(code.len());
    let mut in_new_call = false;
    let mut paren_depth: i32 = 0;
    // DEPYLER-99MODE-S9: Skip Self::new() inside impl std::ops:: blocks
    let mut in_ops_impl = false;
    for line in code.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("impl std::ops::") || trimmed.starts_with("impl DepylerTimeDelta") {
            in_ops_impl = true;
        }
        // End of impl block (approximate: closing brace at indent 0)
        if in_ops_impl && trimmed == "}" && !line.starts_with(' ') && !line.starts_with('\t') {
            in_ops_impl = false;
        }
        let mut fixed_line = line.to_string();
        if line.contains("::new(") && !in_ops_impl {
            in_new_call = true;
            paren_depth = 0;
        }
        if in_new_call {
            for ch in line.chars() {
                match ch {
                    '(' => paren_depth += 1,
                    ')' => paren_depth -= 1,
                    _ => {}
                }
            }
            for param in params {
                let trailing_comma = format!("{},", param);
                let trailing_paren = format!("{})", param);
                if fixed_line.contains(&trailing_comma) {
                    let repl = format!("{}.to_string(),", param);
                    fixed_line = fixed_line.replace(&trailing_comma, &repl);
                }
                if fixed_line.contains(&trailing_paren) {
                    let repl = format!("{}.to_string())", param);
                    fixed_line = fixed_line.replace(&trailing_paren, &repl);
                }
            }
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
        result = format!(
            "{}{}{}",
            &result[..open_bracket],
            new_expr,
            &result[old_end..]
        );
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

pub(super) fn fix_format_expect(code: &str) -> String {
    if !code.contains("format!") || !code.contains(".expect(") {
        return code.to_string();
    }
    let lines: Vec<&str> = code.lines().collect();
    let mut result = Vec::with_capacity(lines.len());
    let mut i = 0;

    while i < lines.len() {
        // Check for multi-line: format!(...)\n        .expect("...")
        // Only match when the line's balanced parens indicate the final `)` closes `format!(`
        if i + 1 < lines.len() {
            let cur = lines[i];
            let next = lines[i + 1];
            let next_trimmed = next.trim();

            if next_trimmed.starts_with(".expect(") {
                // Check if current line has format!( where the final ) closes it
                if let Some(fmt_pos) = cur.find("format!(") {
                    let after_format = &cur[fmt_pos + 7..]; // after "format!"
                    // Count parens from "format!(" onwards - if balanced, the ) closes format!
                    let paren_balance: i32 = after_format
                        .chars()
                        .fold(0, |acc, c| match c {
                            '(' => acc + 1,
                            ')' => acc - 1,
                            _ => acc,
                        });
                    // Only match when parens are balanced (the last ) closes the format! open paren)
                    if paren_balance == 0 && cur.trim_end().ends_with(')') {
                        let cur_trimmed = cur.trim_end();
                        let expect_rest = &next_trimmed[8..]; // skip .expect(
                        if let Some(close) = find_matching_paren(expect_rest) {
                            let after_expect = expect_rest[close + 1..].trim();
                            result.push(format!("{}{}", cur_trimmed, after_expect));
                            i += 2;
                            continue;
                        }
                    }
                }
            }
        }

        // Single-line: format!(...).expect("...")
        // Only match when the ) before .expect( closes the format!( paren
        let line = lines[i];
        if line.contains("format!(") && line.contains(").expect(") {
            if let Some(fmt_pos) = line.find("format!(") {
                // Find the matching close paren for format!(
                let after_fmt_open = &line[fmt_pos + 8..]; // after "format!("
                if let Some(close_idx) = find_matching_paren(after_fmt_open) {
                    let abs_close = fmt_pos + 8 + close_idx;
                    // Check if .expect( immediately follows
                    let after_close = &line[abs_close + 1..];
                    if after_close.starts_with(".expect(") {
                        let in_expect = &after_close[8..]; // skip .expect(
                        if let Some(exp_close) = find_matching_paren(in_expect) {
                            let rest = &in_expect[exp_close + 1..];
                            result.push(format!("{}{}", &line[..abs_close + 1], rest));
                            i += 1;
                            continue;
                        }
                    }
                }
            }
        }

        result.push(line.to_string());
        i += 1;
    }

    let joined = result.join("\n");
    if code.ends_with('\n') {
        joined + "\n"
    } else {
        joined
    }
}

/// Find the matching closing paren in a string starting after an open paren.
pub(super) fn find_matching_paren(s: &str) -> Option<usize> {
    let mut depth = 1;
    let mut in_string = false;
    let mut escape_next = false;
    for (i, c) in s.char_indices() {
        if escape_next {
            escape_next = false;
            continue;
        }
        if c == '\\' {
            escape_next = true;
            continue;
        }
        if c == '"' {
            in_string = !in_string;
            continue;
        }
        if in_string {
            continue;
        }
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

pub(super) fn fix_str_param_return_as_string(code: &str) -> String {
    let lines: Vec<&str> = code.lines().collect();
    let mut result = Vec::with_capacity(lines.len());
    let mut current_fn_returns_string = false;
    let mut current_fn_str_params: Vec<String> = Vec::new();

    for line in &lines {
        let trimmed = line.trim();

        // Track function signatures: fn foo(s: &str, ...) -> String
        if (trimmed.starts_with("fn ") || trimmed.starts_with("pub fn "))
            && trimmed.contains("-> String")
        {
            current_fn_returns_string = true;
            current_fn_str_params.clear();
            // Extract &str param names
            if let Some(start) = trimmed.find('(') {
                if let Some(end) = trimmed.rfind(')') {
                    let params = &trimmed[start + 1..end];
                    for param in params.split(',') {
                        let p = param.trim();
                        // Match &str, &'a str, & str, &'lifetime str
                        if p.contains("str") && p.contains('&') {
                            if let Some(colon) = p.find(':') {
                                let name = p[..colon].trim().trim_start_matches("mut ");
                                if !name.is_empty()
                                    && name
                                        .chars()
                                        .all(|c| c.is_alphanumeric() || c == '_')
                                {
                                    current_fn_str_params.push(name.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }

        // Reset on new function definition
        if trimmed.starts_with("fn ") && !trimmed.contains("-> String") {
            current_fn_returns_string = false;
            current_fn_str_params.clear();
        }

        // Check if this is a bare return of a &str param
        if current_fn_returns_string && !current_fn_str_params.is_empty() {
            let mut fixed = false;
            for param in &current_fn_str_params {
                // Match: "    s" or "    return s;" where s is a &str param
                if trimmed == *param || trimmed == format!("{};", param) {
                    let indent = &line[..line.len() - trimmed.len()];
                    result.push(format!(
                        "{}{}.to_string()",
                        indent,
                        trimmed.trim_end_matches(';')
                    ));
                    fixed = true;
                    break;
                }
                let ret_pattern = format!("return {};", param);
                let ret_bare = format!("return {}", param);
                if trimmed == ret_pattern || trimmed == ret_bare {
                    let indent = &line[..line.len() - trimmed.len()];
                    result.push(format!("{}return {}.to_string();", indent, param));
                    fixed = true;
                    break;
                }
            }
            if fixed {
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
            let next_has_into_bytes = i + 1 < lines.len()
                && lines[i + 1].trim().starts_with(".into_bytes()");
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

pub(super) fn fix_format_debug_in_int_vec(code: &str) -> String {
    // Pattern: format!("{:?}", N) inside vec![] blocks (may span multiple lines)
    // Replace with just N when the expression is a simple integer literal
    let mut result = String::with_capacity(code.len());
    let pattern_start = "format!(\"{:?}\", ";
    let lines: Vec<&str> = code.lines().collect();
    let mut in_vec_block = false;
    let mut vec_depth: i32 = 0;
    for line in &lines {
        let trimmed = line.trim();
        // Track whether we're inside a vec![...] block
        if trimmed.contains("vec![") {
            in_vec_block = true;
            vec_depth += trimmed.matches('[').count() as i32;
            vec_depth -= trimmed.matches(']').count() as i32;
        } else if in_vec_block {
            vec_depth += trimmed.matches('[').count() as i32;
            vec_depth -= trimmed.matches(']').count() as i32;
            if vec_depth <= 0 {
                in_vec_block = false;
                vec_depth = 0;
            }
        }
        if (in_vec_block || trimmed.contains("vec![")) && line.contains(pattern_start) {
            let mut new_line = line.to_string();
            while let Some(start) = new_line.find(pattern_start) {
                let after = &new_line[start + pattern_start.len()..];
                if let Some(end) = after.find(')') {
                    let expr = after[..end].trim();
                    // Only replace if expr is a simple integer literal (possibly negative)
                    let is_int = if let Some(stripped) = expr.strip_prefix('-') {
                        stripped.chars().all(|c| c.is_ascii_digit()) && !stripped.is_empty()
                    } else {
                        expr.chars().all(|c| c.is_ascii_digit()) && !expr.is_empty()
                    };
                    if is_int {
                        let full_match_end = start + pattern_start.len() + end + 1;
                        new_line = format!(
                            "{}{}{}",
                            &new_line[..start],
                            expr,
                            &new_line[full_match_end..]
                        );
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            result.push_str(&new_line);
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }
    result
}

pub(super) fn fix_str_clone_to_string(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    let lines: Vec<&str> = code.lines().collect();
    let mut str_params: Vec<String> = Vec::new();
    for line in &lines {
        let trimmed = line.trim();
        // Track &str parameters
        if trimmed.starts_with("pub fn ") || trimmed.starts_with("fn ") {
            str_params.clear();
            if let Some(paren_start) = trimmed.find('(') {
                if let Some(paren_end) = trimmed.rfind(')') {
                    let params_str = &trimmed[paren_start + 1..paren_end];
                    for param in params_str.split(',') {
                        let param = param.trim();
                        if param.contains("&str") || param.contains("&'") {
                            if let Some(colon) = param.find(':') {
                                let name = param[..colon].trim().to_string();
                                str_params.push(name);
                            }
                        }
                    }
                }
            }
        }
        // Fix: Vec<String> = vec![str_param.clone()] -> vec![str_param.to_string()]
        if trimmed.contains("Vec<String>") && trimmed.contains(".clone()") {
            let mut new_line = line.to_string();
            for param in &str_params {
                let pat = format!("{}.clone()", param);
                let rep = format!("{}.to_string()", param);
                new_line = new_line.replace(&pat, &rep);
            }
            result.push_str(&new_line);
        } else if !str_params.is_empty()
            && trimmed.contains(".clone()")
            && (trimmed.contains(".push(") || trimmed.starts_with("vec!["))
        {
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
            result.push_str(&line.replace(
                ".as_ref().expect(\"value is None\")",
                "",
            ));
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }
    result
}

pub(super) fn fix_char_as_str(code: &str) -> String {
    code.replace(
        ".as_str().unwrap_or_default().to_string()",
        ".to_string()",
    )
}

pub(super) fn fix_format_vec_display(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    // Collect functions that return Vec types
    let mut vec_returning_fns: Vec<String> = Vec::new();
    let mut vec_vars: Vec<String> = Vec::new();
    let lines: Vec<&str> = code.lines().collect();
    for line in &lines {
        let trimmed = line.trim();
        // Track fn returning Vec
        if (trimmed.starts_with("pub fn ") || trimmed.starts_with("fn "))
            && trimmed.contains("-> Vec<")
        {
            if let Some(name_start) = trimmed.find("fn ") {
                let after_fn = &trimmed[name_start + 3..];
                if let Some(paren) = after_fn.find('(') {
                    let fn_name = after_fn[..paren].trim();
                    if !fn_name.is_empty() {
                        vec_returning_fns.push(fn_name.to_string());
                    }
                }
            }
        }
        // Track Vec variables
        if trimmed.contains(": Vec<") || trimmed.contains(":Vec<") {
            if let Some(colon) = trimmed.find(':') {
                let before = trimmed[..colon].trim();
                let name = before
                    .strip_prefix("let ")
                    .unwrap_or(before)
                    .trim()
                    .trim_start_matches("mut ")
                    .trim();
                if !name.is_empty()
                    && name.chars().all(|c| c.is_alphanumeric() || c == '_')
                {
                    vec_vars.push(name.to_string());
                }
            }
        }
    }
    for line in &lines {
        let mut new_line = line.to_string();
        if new_line.contains("format!(\"") && new_line.contains("{}") {
            // Check if any Vec-returning fn call is in the format args
            for fn_name in &vec_returning_fns {
                let call_pat = format!("{fn_name}(");
                if new_line.contains(&call_pat) && new_line.contains("{}") {
                    // Replace {} with {:?} for the position corresponding to the Vec call
                    // Simple approach: if format has {} and a vec-fn-call, replace first matching {}
                    new_line = new_line.replacen("{}", "{:?}", new_line.matches("{}").count());
                    break;
                }
            }
            // Check for Vec variables
            if new_line.contains("{}") {
                for var in &vec_vars {
                    if new_line.contains(var.as_str()) {
                        new_line = new_line.replacen("{}", "{:?}", new_line.matches("{}").count());
                        break;
                    }
                }
            }
        }
        result.push_str(&new_line);
        result.push('\n');
    }
    result
}
