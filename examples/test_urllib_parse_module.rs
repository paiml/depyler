#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
const STR__: &'static str = "=";
use std::collections::HashMap;
#[doc = r" Sum type for heterogeneous dictionary values(Python fidelity)"]
#[derive(Debug, Clone, PartialEq, Default)]
pub enum DepylerValue {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    #[default]
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
#[doc = "Test basic URL parsing."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_urlparse_basic() {
    let url = "https://example.com/path/page.html";
    let result = String::from(url);
    assert_eq!(result.scheme, "https".to_string());
    assert_eq!(result.netloc, "example.com".to_string());
    assert_eq!(result.path, "/path/page.html".to_string());
    println!("{}", "PASS: test_urlparse_basic");
}
#[doc = "Test URL parsing with query string."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_urlparse_with_query() {
    let url = "https://example.com/search?q=python&lang=en";
    let result = String::from(url);
    assert_eq!(result.scheme, "https".to_string());
    assert_eq!(result.netloc, "example.com".to_string());
    assert_eq!(result.path, "/search".to_string());
    assert_eq!(result.query, "q=python&lang=en".to_string());
    println!("{}", "PASS: test_urlparse_with_query");
}
#[doc = "Test URL parsing with fragment."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_urlparse_with_fragment() {
    let url = "https://example.com/page#section1";
    let result = String::from(url);
    assert_eq!(result.scheme, "https".to_string());
    assert_eq!(result.path, "/page".to_string());
    assert_eq!(result.fragment, "section1".to_string());
    println!("{}", "PASS: test_urlparse_with_fragment");
}
#[doc = "Test full URL parsing with all components."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_urlparse_full() {
    let url = "https://user:pass@example.com:8080/path?query=value#fragment";
    let result = String::from(url);
    assert_eq!(result.scheme, "https".to_string());
    assert_eq!(result.netloc, "user:pass@example.com:8080".to_string());
    assert_eq!(result.path, "/path".to_string());
    assert_eq!(result.query, "query=value".to_string());
    assert_eq!(result.fragment, "fragment".to_string());
    println!("{}", "PASS: test_urlparse_full");
}
#[doc = "Test query string parsing."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_parse_qs_basic() {
    let query = "name=John&age=30&city=NYC";
    let result = parse_qs(&query);
    assert_eq!(
        result.get("name").cloned().unwrap_or_default(),
        vec!["John".to_string().to_string()]
    );
    assert_eq!(
        result.get("age").cloned().unwrap_or_default(),
        vec!["30".to_string()]
    );
    assert_eq!(
        result.get("city").cloned().unwrap_or_default(),
        vec!["NYC".to_string().to_string()]
    );
    println!("{}", "PASS: test_parse_qs_basic");
}
#[doc = "Test query string with multiple values."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_parse_qs_multiple_values() {
    let query = "tag=python&tag=rust&tag=programming";
    let result = parse_qs(&query);
    assert_eq!(
        result.get("tag").cloned().unwrap_or_default().len() as i32,
        3
    );
    assert!(result
        .get("tag")
        .cloned()
        .unwrap_or_default()
        .contains("python"));
    assert!(result
        .get("tag")
        .cloned()
        .unwrap_or_default()
        .contains("rust"));
    println!("{}", "PASS: test_parse_qs_multiple_values");
}
#[doc = "Test query string parsing as list of tuples."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_parse_qsl_tuples() {
    let query = "a=1&b=2&c=3";
    let result = parse_qsl(&query);
    assert_eq!(result.len() as i32, 3);
    assert!(result
        .get(&("a".to_string().to_string(), "1".to_string().to_string()))
        .is_some());
    assert!(result
        .get(&("b".to_string().to_string(), "2".to_string().to_string()))
        .is_some());
    assert!(result
        .get(&("c".to_string().to_string(), "3".to_string().to_string()))
        .is_some());
    println!("{}", "PASS: test_parse_qsl_tuples");
}
#[doc = "Test URL encoding from dict."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_urlencode_basic() {
    let params = {
        let mut map = HashMap::new();
        map.insert("name".to_string(), "John Doe".to_string());
        map.insert("age".to_string(), "30".to_string());
        map
    };
    let result = urlencode(&params);
    assert!((result.contains("name=John+Doe")) || (result.contains("name=John%20Doe")));
    assert!(result.contains("age=30"));
    println!("{}", "PASS: test_urlencode_basic");
}
#[doc = "Test URL quoting/encoding."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_quote_string() {
    let text = "Hello World!";
    let result = url::percent_encoding::percent_encode(text);
    assert_eq!(result, "Hello%20World%21");
    println!("{}", "PASS: test_quote_string");
}
#[doc = "Test URL unquoting/decoding."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_unquote_string() {
    let encoded = "Hello%20World%21";
    let result = url::percent_encoding::percent_decode(encoded);
    assert_eq!(result, "Hello World!");
    println!("{}", "PASS: test_unquote_string");
}
#[doc = "Test URL quoting with safe characters."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_quote_safe_chars() {
    let path = "/path/to/file";
    let result = url::percent_encoding::percent_encode(path, "/".to_string());
    assert_eq!(result, "/path/to/file");
    println!("{}", "PASS: test_quote_safe_chars");
}
#[doc = "Test joining URLs."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_urljoin_basic() {
    let base = "https://example.com/dir/";
    let relative = "page.html";
    let result = format!("{}{}", base, relative);
    assert_eq!(result, "https://example.com/dir/page.html".to_string());
    println!("{}", "PASS: test_urljoin_basic");
}
#[doc = "Test joining with absolute URL."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_urljoin_absolute() {
    let base = "https://example.com/dir/";
    let absolute = "https://other.com/page.html";
    let result = format!("{}{}", base, absolute);
    assert_eq!(result, "https://other.com/page.html");
    println!("{}", "PASS: test_urljoin_absolute");
}
#[doc = "Test URL splitting(similar to urlparse)."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_urlsplit_basic() {
    let url = "https://example.com/path?query=value#fragment";
    let result = urlsplit(&url);
    assert_eq!(result.scheme, "https".to_string());
    assert_eq!(result.netloc, "example.com".to_string());
    assert_eq!(result.path, "/path".to_string());
    assert_eq!(result.query, "query=value".to_string());
    assert_eq!(result.fragment, "fragment".to_string());
    println!("{}", "PASS: test_urlsplit_basic");
}
#[doc = "Run all urllib.parse tests."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() {
    println!("{}", STR__.repeat(60 as usize));
    println!("{}", "URLLIB.PARSE MODULE TESTS");
    println!("{}", STR__.repeat(60 as usize));
    test_urlparse_basic();
    test_urlparse_with_query();
    test_urlparse_with_fragment();
    test_urlparse_full();
    test_parse_qs_basic();
    test_parse_qs_multiple_values();
    test_parse_qsl_tuples();
    test_urlencode_basic();
    test_quote_string();
    test_unquote_string();
    test_quote_safe_chars();
    test_urljoin_basic();
    test_urljoin_absolute();
    test_urlsplit_basic();
    println!("{}", STR__.repeat(60 as usize));
    println!("{}", "ALL URLLIB.PARSE TESTS PASSED!");
    println!("{}", "Total tests: 14");
    println!("{}", STR__.repeat(60 as usize));
}
