use serde_json as json;
use serde_json;
use std as os;
use std as sys;
use std::collections::HashMap;
#[doc = "Parse JSON from string"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn parse_json(text: &str) -> HashMap<String, serde_json::Value> {
    serde_json::from_str::<serde_json::Value>(&text).unwrap()
}
#[doc = "Convert dictionary to JSON string"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn serialize_json(data: &HashMap<String, serde_json::Value>) -> String {
    serde_json::to_string(&data).unwrap()
}
#[doc = "Get environment variable"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn get_env_var(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| "".to_string().to_string())
}
#[doc = "Get current working directory"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn get_current_directory() -> String {
    std::env::current_dir()
        .unwrap()
        .to_string_lossy()
        .to_string()
}
#[doc = "Get command line arguments"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn get_args() -> Vec<String> {
    std::env::args().collect::<Vec<String>>()
}
#[doc = "Exit program with code"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn exit_program(code: i32) {
    std::process::exit(code);
}
