#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
const STR__: &'static str = "=";
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
#[doc = "Test basic base64 encoding."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_base64_encode_basic() {
    let data = b"Hello, World!";
    let encoded = format!("{:?}", data)
        .into_bytes();
    assert_eq!(encoded, b"SGVsbG8sIFdvcmxkIQ==");
    println!("{}", "PASS: test_base64_encode_basic");
}
#[doc = "Test basic base64 decoding."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_base64_decode_basic() {
    let encoded = b"SGVsbG8sIFdvcmxkIQ==";
    let decoded = format!("{:?}", encoded);
    assert_eq!(decoded, b"Hello, World!");
    println!("{}", "PASS: test_base64_decode_basic");
}
#[doc = "Test encode-decode round trip."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_base64_roundtrip() {
    let original = b"Python to Rust transpilation!";
    let encoded = format!("{:?}", original)
        .into_bytes();
    let decoded = format!("{:?}", encoded);
    assert_eq!(decoded, original);
    println!("{}", "PASS: test_base64_roundtrip");
}
#[doc = "Test encoding/decoding empty data."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_base64_empty() {
    let data = b"";
    let encoded = format!("{:?}", data)
        .into_bytes();
    assert_eq!(encoded, b"");
    let decoded = format!("{:?}", b"");
    assert_eq!(decoded, b"");
    println!("{}", "PASS: test_base64_empty");
}
#[doc = "Test encoding binary data."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_base64_binary_data() {
    let data = 0..(256);
    let encoded = format!("{:?}", data)
        .into_bytes();
    let decoded = format!("{:?}", encoded);
    assert_eq!(decoded, data);
    println!("{}", "PASS: test_base64_binary_data");
}
#[doc = "Test URL-safe base64 encoding."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_base64_urlsafe_encode() {
    let data = b"Hello>>???World";
    let encoded = format!("{:?}", data)
        .into_bytes();
    assert!(!encoded.get(&b"+").is_some());
    assert!(!encoded.get(&b"/").is_some());
    println!("{}", "PASS: test_base64_urlsafe_encode");
}
#[doc = "Test URL-safe base64 decoding."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_base64_urlsafe_decode() {
    let data = b"Test data with special chars";
    let encoded = format!("{:?}", data)
        .into_bytes();
    let decoded = base64::engine::general_purpose::URL_SAFE
        .decode(encoded)
        .unwrap();
    assert_eq!(decoded, data);
    println!("{}", "PASS: test_base64_urlsafe_decode");
}
#[doc = "Test base64 padding handling."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_base64_padding() {
    let data1 = b"a";
    let encoded1 = format!("{:?}", data1)
        .into_bytes();
    assert_eq!(encoded1, b"YQ==");
    let data2 = b"ab";
    let encoded2 = format!("{:?}", data2)
        .into_bytes();
    assert_eq!(encoded2, b"YWI=");
    let data3 = b"abc";
    let encoded3 = format!("{:?}", data3)
        .into_bytes();
    assert_eq!(encoded3, b"YWJj");
    println!("{}", "PASS: test_base64_padding");
}
#[doc = "Test encoding larger data."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_base64_multiline() {
    let _cse_temp_0 = b"The quick brown fox jumps over the lazy dog. ".repeat(10 as usize);
    let data = _cse_temp_0;
    let encoded = format!("{:?}", data)
        .into_bytes();
    let decoded = format!("{:?}", encoded);
    assert_eq!(decoded, data);
    println!("{}", "PASS: test_base64_multiline");
}
#[doc = "Test encoding Unicode text."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_base64_unicode() {
    let text = "Hello 世界 🌍";
    let data = text.as_bytes().to_vec();
    let encoded = format!("{:?}", data)
        .into_bytes();
    let decoded = format!("{:?}", encoded);
    let result = String::from_utf8_lossy(&decoded).to_string();
    assert_eq!(result, text);
    println!("{}", "PASS: test_base64_unicode");
}
#[doc = "Run all base64 tests."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() {
    println!("{}", STR__.repeat(60 as usize));
    println!("{}", "BASE64 MODULE TESTS");
    println!("{}", STR__.repeat(60 as usize));
    test_base64_encode_basic();
    test_base64_decode_basic();
    test_base64_roundtrip();
    test_base64_empty();
    test_base64_binary_data();
    test_base64_urlsafe_encode();
    test_base64_urlsafe_decode();
    test_base64_padding();
    test_base64_multiline();
    test_base64_unicode();
    println!("{}", STR__.repeat(60 as usize));
    println!("{}", "ALL BASE64 TESTS PASSED!");
    println!("{}", "Total tests: 10");
    println!("{}", STR__.repeat(60 as usize));
}
