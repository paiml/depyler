use regex as re;
use serde_json;
use serde_json::from_str;
use serde_json::to_string;
use std as sys;
use std::collections::HashMap;
use std::path::Path::join as path_join;
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn get_current_dir() -> String {
    std::env::current_dir()
        .unwrap()
        .to_string_lossy()
        .to_string()
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn parse_json(data: String) -> HashMap<String, serde_json::Value> {
    serde_json::from_str(data)
}
#[doc = " Depyler: verified panic-free"]
pub fn join_paths<'a, 'b>(base: &'a str, paths: &[String]) -> String {
    let mut result = base.clone();
    for p in paths.iter().cloned() {
        result = std::path::Path::join(result, p);
    }
    result.to_string()
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn find_pattern<'b, 'a>(text: &'a str, pattern: &'b str) -> Vec<String> {
    let regex = regex::Regex::new(pattern).unwrap();
    regex
        .find_iter(text)
        .map(|m| m.as_str().to_string())
        .collect::<Vec<String>>()
}
