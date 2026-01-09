#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
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
pub struct URL {
    pub original: String,
    pub scheme: String,
    pub host: String,
    pub port: Option<i32>,
    pub path: String,
    pub query_params: std::collections::HashMap<String, String>,
    pub fragment: String,
}
impl URL {
    pub fn new(_url: String) -> Self {
        Self {
            original: String::new(),
            scheme: String::new(),
            host: String::new(),
            port: Default::default(),
            path: String::new(),
            query_params: std::collections::HashMap::new(),
            fragment: String::new(),
        }
    }
    pub fn _parse(&mut self, url: String) {
        let mut remaining = url;
        if remaining.contains("://") {
            let scheme_end = remaining.find("://").map(|i| i as i64).unwrap_or(-1);
            self.scheme = {
                let s = &remaining;
                let len = s.chars().count() as isize;
                let stop_idx = (scheme_end) as isize;
                let stop = if stop_idx < 0 {
                    (len + stop_idx).max(0) as usize
                } else {
                    stop_idx as usize
                };
                s.chars().take(stop).collect::<String>()
            };
            let remaining = {
                let s = &remaining;
                let len = s.chars().count() as isize;
                let start_idx = (scheme_end + 3) as isize;
                let start = if start_idx < 0 {
                    (len + start_idx).max(0) as usize
                } else {
                    start_idx as usize
                };
                s.chars().skip(start).collect::<String>()
            };
        };
        if remaining.contains("#") {
            let fragment_start = remaining.find("#").map(|i| i as i64).unwrap_or(-1);
            self.fragment = {
                let s = &remaining;
                let len = s.chars().count() as isize;
                let start_idx = (fragment_start + 1) as isize;
                let start = if start_idx < 0 {
                    (len + start_idx).max(0) as usize
                } else {
                    start_idx as usize
                };
                s.chars().skip(start).collect::<String>()
            };
            let remaining = {
                let s = &remaining;
                let len = s.chars().count() as isize;
                let stop_idx = (fragment_start) as isize;
                let stop = if stop_idx < 0 {
                    (len + stop_idx).max(0) as usize
                } else {
                    stop_idx as usize
                };
                s.chars().take(stop).collect::<String>()
            };
        };
        if remaining.contains("?") {
            let query_start = remaining.find("?").map(|i| i as i64).unwrap_or(-1);
            let query_string = {
                let s = &remaining;
                let len = s.chars().count() as isize;
                let start_idx = (query_start + 1) as isize;
                let start = if start_idx < 0 {
                    (len + start_idx).max(0) as usize
                } else {
                    start_idx as usize
                };
                s.chars().skip(start).collect::<String>()
            };
            let remaining = {
                let s = &remaining;
                let len = s.chars().count() as isize;
                let stop_idx = (query_start) as isize;
                let stop = if stop_idx < 0 {
                    (len + stop_idx).max(0) as usize
                } else {
                    stop_idx as usize
                };
                s.chars().take(stop).collect::<String>()
            };
            self._parse_query(query_string);
        };
        if remaining.contains("/") {
            let path_start = remaining.find("/").map(|i| i as i64).unwrap_or(-1);
            self.path = {
                let s = &remaining;
                let len = s.chars().count() as isize;
                let start_idx = (path_start) as isize;
                let start = if start_idx < 0 {
                    (len + start_idx).max(0) as usize
                } else {
                    start_idx as usize
                };
                s.chars().skip(start).collect::<String>()
            };
            let remaining = {
                let s = &remaining;
                let len = s.chars().count() as isize;
                let stop_idx = (path_start) as isize;
                let stop = if stop_idx < 0 {
                    (len + stop_idx).max(0) as usize
                } else {
                    stop_idx as usize
                };
                s.chars().take(stop).collect::<String>()
            };
        } else {
            self.path = "/".to_string();
        };
        if remaining.contains(":") {
            let colon_pos = remaining.find(":").map(|i| i as i64).unwrap_or(-1);
            self.host = {
                let s = &remaining;
                let len = s.chars().count() as isize;
                let stop_idx = (colon_pos) as isize;
                let stop = if stop_idx < 0 {
                    (len + stop_idx).max(0) as usize
                } else {
                    stop_idx as usize
                };
                s.chars().take(stop).collect::<String>()
            };
            let port_str = {
                let s = &remaining;
                let len = s.chars().count() as isize;
                let start_idx = (colon_pos + 1) as isize;
                let start = if start_idx < 0 {
                    (len + start_idx).max(0) as usize
                } else {
                    start_idx as usize
                };
                s.chars().skip(start).collect::<String>()
            };
            {
                let _result = (|| -> Result<(), Box<dyn std::error::Error>> {
                    {
                        self.port = port_str.parse::<i32>().unwrap_or(0);
                    }
                    Ok(())
                })();
                if let Err(_) = _result {
                    {
                        self.port = None;
                    }
                }
            }
        } else {
            self.host = remaining;
        };
    }
    pub fn _parse_query(&self, query_string: String) {
        if !query_string {
            return ();
        };
        let pairs = query_string
            .split("&")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        for pair in pairs {
            if pair.contains("=") {
                let eq_pos = pair.find("=").map(|i| i as i64).unwrap_or(-1);
                let key = {
                    let s = &pair;
                    let len = s.chars().count() as isize;
                    let stop_idx = (eq_pos) as isize;
                    let stop = if stop_idx < 0 {
                        (len + stop_idx).max(0) as usize
                    } else {
                        stop_idx as usize
                    };
                    s.chars().take(stop).collect::<String>()
                };
                let value = {
                    let s = &pair;
                    let len = s.chars().count() as isize;
                    let start_idx = (eq_pos + 1) as isize;
                    let start = if start_idx < 0 {
                        (len + start_idx).max(0) as usize
                    } else {
                        start_idx as usize
                    };
                    s.chars().skip(start).collect::<String>()
                };
                self.query_params.clone().insert(key, value);
            } else {
                self.query_params.clone().insert(pair, "".to_string());
            };
        }
    }
    pub fn get_query_param(&self, key: String) -> Option<String> {
        if self.query_params.clone().contains_key(&key) {
            return Some(
                self.query_params
                    .clone()
                    .get(&key)
                    .cloned()
                    .unwrap_or_default(),
            );
        };
        return None;
    }
    pub fn is_secure(&self) -> bool {
        return self.scheme.clone().to_lowercase() == "https".to_string();
    }
    pub fn get_base_url(&self) -> String {
        let mut result = "".to_string();
        if self.scheme.clone() {
            let result = result + self.scheme.clone() + "://".to_string();
        };
        let mut result = result + self.host.clone();
        if self.port.clone().is_some() {
            let result = result + ":".to_string() + self.port.clone().to_string();
        };
        return result;
    }
}
#[doc = "Extract domain from URL string"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn extract_domain(url: &str) -> String {
    let parsed = URL::new(url);
    parsed.host
}
#[doc = "Basic email validation"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn is_valid_email(email: &str) -> bool {
    let _cse_temp_0 = !email.contains("@");
    let _cse_temp_1 = email.matches("@").count() as i32 != 1;
    let _cse_temp_2 = (_cse_temp_0) || (_cse_temp_1);
    if _cse_temp_2 {
        return false;
    }
    let at_pos = email.find("@").map(|i| i as i32).unwrap_or(-1);
    let local_part = {
        let base = email;
        let stop_idx: i32 = at_pos;
        let len = base.chars().count() as i32;
        let actual_stop = if stop_idx < 0 {
            (len + stop_idx).max(0) as usize
        } else {
            stop_idx.min(len) as usize
        };
        base.chars().take(actual_stop).collect::<String>()
    };
    let domain_part = {
        let base = email;
        let start_idx: i32 = at_pos + 1;
        let len = base.chars().count() as i32;
        let actual_start = if start_idx < 0 {
            (len + start_idx).max(0) as usize
        } else {
            start_idx.min(len) as usize
        };
        base.chars().skip(actual_start).collect::<String>()
    };
    let _cse_temp_3 = local_part.len() as i32;
    let _cse_temp_4 = _cse_temp_3 == 0;
    let _cse_temp_5 = domain_part.len() as i32;
    let _cse_temp_6 = _cse_temp_5 == 0;
    let _cse_temp_7 = (_cse_temp_4) || (_cse_temp_6);
    if _cse_temp_7 {
        return false;
    }
    let _cse_temp_8 = !domain_part.contains(".");
    if _cse_temp_8 {
        return false;
    }
    let _cse_temp_9 = email.contains("..");
    if _cse_temp_9 {
        return false;
    }
    let _cse_temp_10 = (email.starts_with(".")) || (email.ends_with("."));
    if _cse_temp_10 {
        return false;
    }
    true
}
#[doc = "Normalize URL by removing trailing slash, converting to lowercase"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn normalize_url(url: &str) -> String {
    let mut normalized: String = Default::default();
    normalized = url.trim().to_string().to_lowercase();
    let _cse_temp_0 = normalized.len() as i32;
    let _cse_temp_1 = _cse_temp_0 > 1;
    let _cse_temp_2 = (normalized.ends_with("/")) && (_cse_temp_1);
    if _cse_temp_2 {
        let _cse_temp_3 = normalized.matches("/").count() as i32 > 2;
        if _cse_temp_3 {
            normalized = {
                let base = &normalized;
                let stop_idx = -1 as isize;
                let stop = if stop_idx < 0 {
                    (base.len() as isize + stop_idx).max(0) as usize
                } else {
                    stop_idx as usize
                };
                base[..stop.min(base.len())].to_vec()
            };
        }
    }
    normalized.to_string()
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_is_valid_email_examples() {
        let _ = is_valid_email(Default::default());
    }
    #[test]
    fn quickcheck_normalize_url() {
        fn prop(url: String) -> TestResult {
            let once = normalize_url((&*url).into());
            let twice = normalize_url(once.clone());
            if once != twice {
                return TestResult::failed();
            }
            TestResult::passed()
        }
        quickcheck(prop as fn(String) -> TestResult);
    }
}
