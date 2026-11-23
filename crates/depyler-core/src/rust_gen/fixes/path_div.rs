//! GH-279: lower `.py_div(` to `.join(` only when the receiver is demonstrably a path.
//!
//! Python `a / b` on numbers must stay `py_div`; pathlib `Path / str` joins. The
//! receiver is a path when it is `Path::new(..)` / `PathBuf::from(..)`, or a local
//! identifier bound (or declared as a parameter) as a Path/PathBuf in the same fn.

const PATH_CTORS: [&str; 2] = ["Path::new", "PathBuf::from"];
const PATH_TYPES: [&str; 4] = ["PathBuf", "Path", "&Path", "&std::path::Path"];

/// Rewrite `<path>.py_div(x)` to `<path>.join(x)`; all other `py_div` is untouched.
pub(super) fn lower_path_div(code: &str) -> String {
    if !code.contains(".py_div(") {
        return code.to_string();
    }
    let mut out = String::with_capacity(code.len());
    for seg in split_fns(code) {
        out.push_str(&rewrite_segment(seg));
    }
    out
}

/// Split into segments, each starting at a column-0 `fn `/`pub fn ` line.
fn split_fns(code: &str) -> Vec<&str> {
    let mut starts: Vec<usize> = vec![0];
    let mut pos = 0;
    for line in code.split_inclusive('\n') {
        if pos > 0 && (line.starts_with("fn ") || line.starts_with("pub fn ")) {
            starts.push(pos);
        }
        pos += line.len();
    }
    starts.push(code.len());
    starts.windows(2).map(|w| &code[w[0]..w[1]]).collect()
}

fn rewrite_segment(seg: &str) -> String {
    if !seg.contains(".py_div(") {
        return seg.to_string();
    }
    let names = path_names(seg);
    let mut out = String::with_capacity(seg.len());
    let mut rest = seg;
    while let Some(i) = rest.find(".py_div(") {
        let (head, tail) = rest.split_at(i);
        out.push_str(head);
        let is_path =
            receiver_is_path(&format!("{}{}", &seg[..seg.len() - rest.len()], head), &names);
        out.push_str(if is_path { ".join(" } else { ".py_div(" });
        rest = &tail[".py_div(".len()..];
    }
    out.push_str(rest);
    out
}

/// Names declared as Path/PathBuf in this fn (params and `let` bindings).
fn path_names(seg: &str) -> Vec<String> {
    let mut names: Vec<String> = seg.lines().filter_map(let_path_name).collect();
    if let Some(sig) = seg.lines().next() {
        names.extend(sig.split(['(', ',', ')']).filter_map(param_path_name));
    }
    names
}

fn is_ctor(expr: &str) -> bool {
    let e = expr.trim_start_matches("std::path::");
    PATH_CTORS.iter().any(|c| e.starts_with(c))
}

/// `let [mut] name[: ty] = rhs` where rhs is a path constructor or ty is a path type.
fn let_path_name(line: &str) -> Option<String> {
    let r = line.trim_start().strip_prefix("let ")?;
    let r = r.strip_prefix("mut ").unwrap_or(r);
    let (lhs, rhs) = r.split_once('=')?;
    let (name, ty) = lhs.split_once(':').unwrap_or((lhs, ""));
    let typed = PATH_TYPES.contains(&ty.trim());
    (typed || is_ctor(rhs.trim_start())).then(|| name.trim().to_string())
}

/// `name: PathType` fragment of a fn signature.
fn param_path_name(frag: &str) -> Option<String> {
    let (n, ty) = frag.split_once(':')?;
    let ty = ty.trim().trim_start_matches("std::path::");
    PATH_TYPES.contains(&ty).then(|| n.trim().trim_start_matches("mut ").to_string())
}

/// Start index of the receiver expression that ends `before` (a call or parenthesised group).
fn expr_start(before: &str) -> usize {
    let head = match before.strip_suffix(')') {
        Some(inner) => matching_open(inner).unwrap_or(0),
        None => before.len(),
    };
    before[..head]
        .rfind(|c: char| !(c.is_alphanumeric() || c == '_' || c == ':'))
        .map_or(0, |i| i + 1)
}

/// `before` is all text preceding `.py_div(` in the segment.
fn receiver_is_path(before: &str, names: &[String]) -> bool {
    let expr = &before[expr_start(before)..];
    let recv = match expr.strip_prefix('(') {
        Some(rest) => rest.strip_suffix(')').unwrap_or(rest),
        None => expr,
    };
    let recv = recv.trim_start_matches('&').trim_start_matches("mut ").trim();
    is_ctor(recv) || names.iter().any(|n| n == recv)
}

/// Index of the `(` matching the `)` that ended `s` (so `s` excludes that `)`).
fn matching_open(s: &str) -> Option<usize> {
    let mut depth = 1usize;
    for (i, c) in s.char_indices().rev() {
        depth = match c {
            ')' => depth + 1,
            '(' => depth - 1,
            _ => depth,
        };
        if depth == 0 {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joins_pathbuf_local() {
        let c = "pub fn f(b: &str) {\n    let q = std::path::PathBuf::from(b);\n    let r = (q).py_div(\"x\");\n}\n";
        assert!(lower_path_div(c).contains("(q).join(\"x\")"));
    }

    #[test]
    fn joins_typed_param() {
        let c = "pub fn f(p: &Path) {\n    let r = p.py_div(\"x\");\n}\n";
        assert!(lower_path_div(c).contains("p.join(\"x\")"));
    }

    #[test]
    fn joins_ctor_receiver() {
        let c = "fn f() { let r = (Path::new(\"a\")).py_div(\"x\"); }\n";
        assert!(lower_path_div(c).contains("Path::new(\"a\")).join("));
    }

    #[test]
    fn leaves_numeric_div() {
        let c = "pub fn f(a: f64, b: f64) -> f64 {\n    (a).py_div(b)\n}\n";
        assert_eq!(lower_path_div(c), c);
    }

    #[test]
    fn leaves_other_call_result() {
        let c = "fn f() { let r = (len(x)).py_div(2); let s = g(1).py_div(2); }\n";
        assert_eq!(lower_path_div(c), c);
    }

    #[test]
    fn path_name_in_other_fn_does_not_leak() {
        let c = "fn a() {\n    let q = PathBuf::from(\"a\");\n}\nfn b(q: f64) {\n    (q).py_div(2.0)\n}\n";
        assert_eq!(lower_path_div(c), c);
    }

    #[test]
    fn no_py_div_is_identity() {
        assert_eq!(lower_path_div("fn a() {}\n"), "fn a() {}\n");
    }
}
