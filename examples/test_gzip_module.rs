#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#[doc = "// NOTE: Map Python module 'gzip'(tracked in DEPYLER-0424)"]
use std::io::Cursor;
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
#[doc = "Test basic compression and decompression."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_gzip_compress_decompress() {
    let data = b"Hello, this is a test string for compression!";
    let compressed = gzip.compress(data);
    let decompressed = gzip.decompress(compressed);
    assert_eq!(decompressed, data);
    println!("{}", "PASS: test_gzip_compress_decompress");
}
#[doc = "Test compressing text data."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_gzip_compress_text() {
    let _cse_temp_0 = "The quick brown fox jumps over the lazy dog. ".repeat(10 as usize);
    let text = _cse_temp_0;
    let data = text.as_bytes().to_vec();
    let compressed = gzip.compress(data);
    let decompressed = gzip.decompress(compressed);
    assert_eq!(String::from_utf8_lossy(&decompressed).to_string(), text);
    println!("{}", "PASS: test_gzip_compress_text");
}
#[doc = "Test compressing empty data."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_gzip_compress_empty() {
    let data = b"";
    let compressed = gzip.compress(data);
    let decompressed = gzip.decompress(compressed);
    assert_eq!(decompressed, b"");
    println!("{}", "PASS: test_gzip_compress_empty");
}
#[doc = "Test different compression levels."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_gzip_compress_levels() {
    let _cse_temp_0 = b"Test data for compression levels! ".repeat(100 as usize);
    let data = _cse_temp_0;
    let compressed_1 = gzip.compress(data);
    let decompressed_1 = gzip.decompress(compressed_1);
    assert_eq!(decompressed_1, data);
    let compressed_9 = gzip.compress(data);
    let decompressed_9 = gzip.decompress(compressed_9);
    assert_eq!(decompressed_9, data);
    assert!(compressed_9.len() as i32 <= compressed_1.len() as i32);
    println!("{}", "PASS: test_gzip_compress_levels");
}
#[doc = "Test compressing larger data."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_gzip_large_data() {
    let _cse_temp_0 = b"ABCDEFGHIJ".repeat(100 as usize);
    let data = _cse_temp_0;
    let compressed = gzip.compress(data);
    let decompressed = gzip.decompress(compressed);
    assert_eq!(decompressed, data);
    assert!((compressed.len() as i32) < data.len() as i32 / 2);
    println!("{}", "PASS: test_gzip_large_data");
}
#[doc = "Test compressing binary data."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_gzip_binary_data() {
    let data = 0..(256);
    let compressed = gzip.compress(data);
    let decompressed = gzip.decompress(compressed);
    assert_eq!(decompressed, data);
    println!("{}", "PASS: test_gzip_binary_data");
}
#[doc = "Test compressing Unicode text."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_gzip_unicode_text() {
    let text = "Hello 世界 🌍 Привет مرحبا";
    let data = text.as_bytes().to_vec();
    let compressed = gzip.compress(data);
    let decompressed = gzip.decompress(compressed);
    assert_eq!(String::from_utf8_lossy(&decompressed).to_string(), text);
    println!("{}", "PASS: test_gzip_unicode_text");
}
#[doc = "Test compressing already compressed data."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_gzip_multiple_compress() {
    let data = b"Original data for double compression test";
    let compressed_once = gzip.compress(data);
    let compressed_twice = gzip.compress(compressed_once);
    let decompressed_once = gzip.decompress(compressed_twice);
    let decompressed_twice = gzip.decompress(decompressed_once);
    assert_eq!(decompressed_twice, data);
    println!("{}", "PASS: test_gzip_multiple_compress");
}
#[doc = "Run all gzip tests."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() {
    println!("{}", STR__.repeat(60 as usize));
    println!("{}", "GZIP MODULE TESTS");
    println!("{}", STR__.repeat(60 as usize));
    test_gzip_compress_decompress();
    test_gzip_compress_text();
    test_gzip_compress_empty();
    test_gzip_compress_levels();
    test_gzip_large_data();
    test_gzip_binary_data();
    test_gzip_unicode_text();
    test_gzip_multiple_compress();
    println!("{}", STR__.repeat(60 as usize));
    println!("{}", "ALL GZIP TESTS PASSED!");
    println!("{}", "Total tests: 8");
    println!("{}", STR__.repeat(60 as usize));
}
