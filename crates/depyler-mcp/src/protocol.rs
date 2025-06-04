use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct McpMessage {
    pub id: String,
    pub method: String,
    pub params: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct McpResponse {
    pub id: String,
    pub result: Option<serde_json::Value>,
    pub error: Option<McpError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct McpError {
    pub code: i32,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

pub const MCP_VERSION: &str = "1.0.0";

pub mod methods {
    pub const TRANSPILE: &str = "depyler/transpile";
    pub const ANALYZE: &str = "depyler/analyze";
    pub const SUGGEST: &str = "depyler/suggest";
}

pub mod error_codes {
    pub const PARSE_ERROR: i32 = -32001;
    pub const UNSUPPORTED_CONSTRUCT: i32 = -32002;
    pub const TYPE_ERROR: i32 = -32003;
    pub const TIMEOUT: i32 = -32004;
}
