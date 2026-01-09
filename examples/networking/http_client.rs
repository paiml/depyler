#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
const STR_EMPTY: &'static str = "";
use std::collections::HashMap;
#[doc = r" Sum type for heterogeneous dictionary values(Python fidelity)"]
#[derive(Debug, Clone, PartialEq)]
pub enum DepylerValue {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    None,
    List(Vec<DepylerValue>),
    Dict(std::collections::HashMap<String, DepylerValue>),
}
impl std::fmt::Display for DepylerValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DepylerValue::Int(i) => write!(f, "{}", i),
            DepylerValue::Float(fl) => write!(f, "{}", fl),
            DepylerValue::Str(s) => write!(f, "{}", s),
            DepylerValue::Bool(b) => write!(f, "{}", b),
            DepylerValue::None => write!(f, "None"),
            DepylerValue::List(l) => write!(f, "{:?}", l),
            DepylerValue::Dict(d) => write!(f, "{:?}", d),
        }
    }
}
impl DepylerValue {
    #[doc = r" Get length of string, list, or dict"]
    pub fn len(&self) -> usize {
        match self {
            DepylerValue::Str(s) => s.len(),
            DepylerValue::List(l) => l.len(),
            DepylerValue::Dict(d) => d.len(),
            _ => 0,
        }
    }
    #[doc = r" Check if empty"]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    #[doc = r" Get chars iterator for string values"]
    pub fn chars(&self) -> std::str::Chars<'_> {
        match self {
            DepylerValue::Str(s) => s.chars(),
            _ => "".chars(),
        }
    }
    #[doc = r" Insert into dict(mutates self if Dict variant)"]
    pub fn insert(&mut self, key: String, value: DepylerValue) {
        if let DepylerValue::Dict(d) = self {
            d.insert(key, value);
        }
    }
    #[doc = r" Get value from dict by key"]
    pub fn get(&self, key: &str) -> Option<&DepylerValue> {
        if let DepylerValue::Dict(d) = self {
            d.get(key)
        } else {
            Option::None
        }
    }
    #[doc = r" Check if dict contains key"]
    pub fn contains_key(&self, key: &str) -> bool {
        if let DepylerValue::Dict(d) = self {
            d.contains_key(key)
        } else {
            false
        }
    }
    #[doc = r" Convert to String"]
    pub fn to_string(&self) -> String {
        match self {
            DepylerValue::Str(s) => s.clone(),
            DepylerValue::Int(i) => i.to_string(),
            DepylerValue::Float(fl) => fl.to_string(),
            DepylerValue::Bool(b) => b.to_string(),
            DepylerValue::None => "None".to_string(),
            DepylerValue::List(l) => format!("{:?}", l),
            DepylerValue::Dict(d) => format!("{:?}", d),
        }
    }
    #[doc = r" Convert to i64"]
    pub fn to_i64(&self) -> i64 {
        match self {
            DepylerValue::Int(i) => *i,
            DepylerValue::Float(fl) => *fl as i64,
            DepylerValue::Bool(b) => {
                if *b {
                    1
                } else {
                    0
                }
            }
            DepylerValue::Str(s) => s.parse().unwrap_or(0),
            _ => 0,
        }
    }
    #[doc = r" Convert to f64"]
    pub fn to_f64(&self) -> f64 {
        match self {
            DepylerValue::Float(fl) => *fl,
            DepylerValue::Int(i) => *i as f64,
            DepylerValue::Bool(b) => {
                if *b {
                    1.0
                } else {
                    0.0
                }
            }
            DepylerValue::Str(s) => s.parse().unwrap_or(0.0),
            _ => 0.0,
        }
    }
    #[doc = r" Convert to bool"]
    pub fn to_bool(&self) -> bool {
        match self {
            DepylerValue::Bool(b) => *b,
            DepylerValue::Int(i) => *i != 0,
            DepylerValue::Float(fl) => *fl != 0.0,
            DepylerValue::Str(s) => !s.is_empty(),
            DepylerValue::List(l) => !l.is_empty(),
            DepylerValue::Dict(d) => !d.is_empty(),
            DepylerValue::None => false,
        }
    }
}
impl std::ops::Index<usize> for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, idx: usize) -> &Self::Output {
        match self {
            DepylerValue::List(l) => &l[idx],
            _ => panic!("Cannot index non-list DepylerValue"),
        }
    }
}
impl std::ops::Index<&str> for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, key: &str) -> &Self::Output {
        match self {
            DepylerValue::Dict(d) => d.get(key).unwrap_or(&DepylerValue::None),
            _ => panic!("Cannot index non-dict DepylerValue with string key"),
        }
    }
}
#[derive(Debug, Clone)]
pub struct HTTPRequest {
    pub method: DepylerValue,
    pub url: String,
    pub headers: DepylerValue,
    pub body: String,
}
impl HTTPRequest {
    pub fn new(
        method: String,
        url: String,
        headers: Option<std::collections::HashMap<String, String>>,
    ) -> Self {
        Self {
            method,
            url,
            headers,
            body: String::new(),
        }
    }
    pub fn add_header(&self, name: String, value: String) {
        self.headers.clone().insert(name, value);
    }
    pub fn set_body(&mut self, body: String) {
        self.body = body;
    }
    pub fn to_string(&self) -> String {
        let mut lines = vec![];
        lines.push(format!(
            "{} {} HTTP/1.1",
            self.method.clone(),
            self.url.clone()
        ));
        for (name, value) in self.headers.clone().items() {
            lines.push(format!("{}: {}", name, value));
        }
        lines.push("".to_string());
        if self.body.clone() {
            lines.push(self.body.clone());
        };
        return lines.join("\n".to_string());
    }
}
#[derive(Debug, Clone)]
pub struct HTTPResponse {
    pub status_code: i32,
    pub reason: String,
    pub headers: std::collections::HashMap<String, String>,
    pub body: String,
}
impl HTTPResponse {
    pub fn new(status_code: i32, reason: String) -> Self {
        Self {
            status_code,
            reason,
            headers: std::collections::HashMap::new(),
            body: String::new(),
        }
    }
    pub fn add_header(&self, name: String, value: String) {
        self.headers.clone().insert(name, value);
    }
    pub fn set_body(&mut self, body: String) {
        self.body = body;
    }
    pub fn is_success(&self) -> bool {
        return 200 <= self.status_code.clone() && self.status_code.clone() < 300;
    }
    pub fn is_client_error(&self) -> bool {
        return 400 <= self.status_code.clone() && self.status_code.clone() < 500;
    }
    pub fn is_server_error(&self) -> bool {
        return 500 <= self.status_code.clone() && self.status_code.clone() < 600;
    }
}
#[derive(Debug, Clone)]
pub struct HTTPClient {
    pub default_headers: std::collections::HashMap<String, String>,
}
impl HTTPClient {
    pub fn new() -> Self {
        Self {
            default_headers: std::collections::HashMap::new(),
        }
    }
    pub fn create_request(&self, method: String, url: String) -> HTTPRequest {
        let request = HTTPRequest::new(method, url, self.default_headers.clone().clone());
        return request;
    }
    pub fn parse_response(&self, response_text: String) -> HTTPResponse {
        let lines = response_text
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        if !lines {
            return HTTPResponse::new(500, "Invalid Response".to_string());
        };
        let status_line = lines[0 as usize];
        let status_parts = status_line
            .splitn((2 + 1) as usize, " ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        if (status_parts.len() as i32) < 3 {
            return HTTPResponse::new(500, "Invalid Status Line".to_string());
        };
        {
            let _result = (|| -> Result<(), Box<dyn std::error::Error>> {
                {
                    let status_code = status_parts[1 as usize].parse::<i32>().unwrap_or(0);
                }
                Ok(())
            })();
            if let Err(_) = _result {
                {
                    let status_code = 500;
                }
            }
        }
        let reason = if (status_parts.len() as i32) > 2 {
            status_parts[2 as usize]
        } else {
            "Unknown".to_string()
        };
        let response = HTTPResponse::new(status_code, reason);
        let mut i = 1;
        while i < (lines.len() as i32) && lines[i as usize].trim().to_string() {
            let header_line = lines[i as usize].trim().to_string();
            if header_line.contains_key(&":".to_string()) {
                let colon_pos = header_line.find(":").map(|i| i as i64).unwrap_or(-1);
                let name = {
                    let s = &header_line;
                    let len = s.chars().count() as isize;
                    let stop_idx = (colon_pos) as isize;
                    let stop = if stop_idx < 0 {
                        (len + stop_idx).max(0) as usize
                    } else {
                        stop_idx as usize
                    };
                    s.chars().take(stop).collect::<String>()
                }
                .trim()
                .to_string();
                let value = {
                    let s = &header_line;
                    let len = s.chars().count() as isize;
                    let start_idx = (colon_pos + 1) as isize;
                    let start = if start_idx < 0 {
                        (len + start_idx).max(0) as usize
                    } else {
                        start_idx as usize
                    };
                    s.chars().skip(start).collect::<String>()
                }
                .trim()
                .to_string();
                response.add_header(name, value);
            };
            let i = i + 1;
        }
        let body_start = i + 1;
        if body_start < (lines.len() as i32) {
            let body_lines = {
                let s = &lines;
                let len = s.chars().count() as isize;
                let start_idx = (body_start) as isize;
                let start = if start_idx < 0 {
                    (len + start_idx).max(0) as usize
                } else {
                    start_idx as usize
                };
                s.chars().skip(start).collect::<String>()
            };
            response.set_body(body_lines.join("\n".to_string()));
        };
        return response;
    }
}
#[doc = "Build URL query string from parameters"]
#[doc = " Depyler: verified panic-free"]
pub fn build_query_string(params: &std::collections::HashMap<String, String>) -> String {
    if params.is_empty() {
        return STR_EMPTY;
    }
    let mut pairs: Vec<String> = vec![];
    for (key, value) in params
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect::<Vec<_>>()
    {
        let encoded_key = key.replace(" ", "%20");
        let encoded_value = value.replace(" ", "%20");
        pairs.push(format!("{}={}", encoded_key, encoded_value));
    }
    format!("{}{}", "?", pairs.join("&"))
}
#[doc = "Parse URL into scheme, host, path, and query"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn parse_url_components(url: &str) -> (String, String, String, String) {
    let mut remaining: String = Default::default();
    let mut query: String = Default::default();
    let mut scheme: String = Default::default();
    let mut path: String = Default::default();
    let mut host: String = Default::default();
    scheme = STR_EMPTY;
    host = STR_EMPTY;
    path = "/".to_string();
    query = STR_EMPTY;
    remaining = url.to_string();
    let _cse_temp_0 = remaining.contains("://");
    if _cse_temp_0 {
        let scheme_end = remaining.find("://").map(|i| i as i32).unwrap_or(-1);
        scheme = {
            let base = remaining;
            let stop_idx: i32 = scheme_end;
            let len = base.chars().count() as i32;
            let actual_stop = if stop_idx < 0 {
                (len + stop_idx).max(0) as usize
            } else {
                stop_idx.min(len) as usize
            };
            base.chars().take(actual_stop).collect::<String>()
        };
        remaining = {
            let base = &remaining;
            let start_idx = scheme_end + 3 as isize;
            let start = if start_idx < 0 {
                (base.len() as isize + start_idx).max(0) as usize
            } else {
                start_idx as usize
            };
            if start < base.len() {
                base[start..].to_vec()
            } else {
                Vec::new()
            }
        };
    }
    let _cse_temp_1 = remaining.contains("?");
    if _cse_temp_1 {
        let query_start = remaining.find("?").map(|i| i as i32).unwrap_or(-1);
        query = {
            let base = &remaining;
            let start_idx = query_start + 1 as isize;
            let start = if start_idx < 0 {
                (base.len() as isize + start_idx).max(0) as usize
            } else {
                start_idx as usize
            };
            if start < base.len() {
                base[start..].to_vec()
            } else {
                Vec::new()
            }
        };
        remaining = {
            let base = &remaining;
            let stop_idx = query_start as isize;
            let stop = if stop_idx < 0 {
                (base.len() as isize + stop_idx).max(0) as usize
            } else {
                stop_idx as usize
            };
            base[..stop.min(base.len())].to_vec()
        };
    }
    let _cse_temp_2 = remaining.contains("/");
    if _cse_temp_2 {
        let path_start = remaining.find("/").map(|i| i as i32).unwrap_or(-1);
        path = {
            let base = &remaining;
            let start_idx = path_start as isize;
            let start = if start_idx < 0 {
                (base.len() as isize + start_idx).max(0) as usize
            } else {
                start_idx as usize
            };
            if start < base.len() {
                base[start..].to_vec()
            } else {
                Vec::new()
            }
        };
        host = {
            let base = &remaining;
            let stop_idx = path_start as isize;
            let stop = if stop_idx < 0 {
                (base.len() as isize + stop_idx).max(0) as usize
            } else {
                stop_idx as usize
            };
            base[..stop.min(base.len())].to_vec()
        };
    } else {
        host = remaining;
        path = "/".to_string();
    }
    (scheme, host, path, query)
}
#[doc = "Format response as JSON-like string(simplified)"]
#[doc = " Depyler: verified panic-free"]
pub fn format_json_response(response: &HTTPResponse) -> String {
    let mut result = "{\n".to_string();
    let _cse_temp_0 = format!(
        "{}{}",
        result,
        format!("  \"status\": {},\n", response.status_code)
    );
    result = _cse_temp_0;
    result = _cse_temp_0;
    result = format!("{}{}", result, "  \"headers\": {\n");
    let mut header_items: Vec<String> = vec![];
    for (name, value) in response
        .headers
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect::<Vec<_>>()
    {
        header_items.push(format!("    \"{:?}\": \"{:?}\"", name, value));
    }
    let _cse_temp_1 = format!("{}{}", result, header_items.join(",\n"));
    result = _cse_temp_1;
    result = format!("{}{}", result, "\n  },\n");
    result = _cse_temp_0;
    result = format!("{}{}", result, "}");
    result.to_string()
}
