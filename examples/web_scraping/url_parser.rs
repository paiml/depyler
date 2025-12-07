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
                if let Err(_e) = _result {
                    {
                        self.port = ();
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
                self.query_params.insert(key, value);
            } else {
                self.query_params.insert(pair, "".to_string());
            };
        }
    }
    pub fn get_query_param(&self, key: String) -> Option<String> {
        if self.query_params.contains_key(&key) {
            return self.query_params.get(&key).cloned().unwrap_or_default();
        };
        return ();
    }
    pub fn is_secure(&self) -> bool {
        return self.scheme.to_lowercase() == "https".to_string();
    }
    pub fn get_base_url(&self) -> String {
        let mut result = "".to_string();
        if self.scheme {
            let result = result + self.scheme + "://".to_string();
        };
        let mut result = result + self.host;
        if self.port.is_some() {
            let result = result + ":".to_string() + self.port.to_string();
        };
        return result;
    }
}
#[doc = "Extract domain from URL string"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn extract_domain(url: String) -> String {
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
    let mut normalized = url.trim().to_string().to_lowercase();
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
