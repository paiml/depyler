use serde_json;
#[doc = " Depyler: verified panic-free"]
pub fn build_path<'b, 'a>(base: &'a str, parts: &'b Vec<serde_json::Value>) -> String {
    let mut result = base.to_string();
    for p in parts.iter().cloned() {
        result = format!("{}{}", format!("{}{}", result, "/"), p);
    }
    result.to_string()
}
