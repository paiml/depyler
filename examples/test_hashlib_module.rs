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
#[doc = "Test basic SHA256 hashing."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_sha256_basic() {
    let data = b"Hello, World!";
    let mut hash_obj = {
                        let mut hasher = Box::new(std::collections::hash_map::DefaultHasher::new()) as Box<dyn DynDigest>;
        hasher.update(data);
        hasher
    };
    let result = {
                hex::encode(hash_obj.finalize_reset())
    };
    let expected = "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f";
    assert_eq!(result, expected);
    println!("{}", "PASS: test_sha256_basic");
}
#[doc = "Test SHA256 of empty data."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_sha256_empty() {
    let data = b"";
    let mut hash_obj = {
                        let mut hasher = Box::new(std::collections::hash_map::DefaultHasher::new()) as Box<dyn DynDigest>;
        hasher.update(data);
        hasher
    };
    let result = {
                hex::encode(hash_obj.finalize_reset())
    };
    let expected = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
    assert_eq!(result, expected);
    println!("{}", "PASS: test_sha256_empty");
}
#[doc = "Test SHA256 with multiple updates."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_sha256_update() {
    let mut hash_obj = {
                        Box::new(std::collections::hash_map::DefaultHasher::new()) as Box<dyn DynDigest>
    };
    hash_obj.update(&b"Hello, ");
    hash_obj.update(&b"World!");
    let result = {
                hex::encode(hash_obj.finalize_reset())
    };
    let expected = "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f";
    assert_eq!(result, expected);
    println!("{}", "PASS: test_sha256_update");
}
#[doc = "Test basic SHA1 hashing."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_sha1_basic() {
    let data = b"test data";
    let mut hash_obj = {
                use sha1::Digest;
        let mut hasher = Box::new(sha1::Sha1::new()) as Box<dyn DynDigest>;
        hasher.update(data);
        hasher
    };
    let result = {
                hex::encode(hash_obj.finalize_reset())
    };
    let expected = "f48dd853820860816c75d54d0f584dc863327a7c";
    assert_eq!(result, expected);
    println!("{}", "PASS: test_sha1_basic");
}
#[doc = "Test basic MD5 hashing."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_md5_basic() {
    let data = b"test";
    let mut hash_obj = {
                use md5::Digest;
        let mut hasher = Box::new(md5::Md5::new()) as Box<dyn DynDigest>;
        hasher.update(data);
        hasher
    };
    let result = {
                hex::encode(hash_obj.finalize_reset())
    };
    let expected = "098f6bcd4621d373cade4e832627b4f6";
    assert_eq!(result, expected);
    println!("{}", "PASS: test_md5_basic");
}
#[doc = "Test hashing binary data."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_sha256_binary_data() {
    let data = 0..(256);
    let mut hash_obj = {
                        let mut hasher = Box::new(std::collections::hash_map::DefaultHasher::new()) as Box<dyn DynDigest>;
        hasher.update(data);
        hasher
    };
    let result = {
                hex::encode(hash_obj.finalize_reset())
    };
    assert_eq!(result.len() as i32, 64);
    assert!(result
        .iter()
        .cloned()
        .map(|c| "0123456789abcdef".to_string().contains(&*c))
        .all(|x| x));
    println!("{}", "PASS: test_sha256_binary_data");
}
#[doc = "Test hashing larger data."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_sha256_large_data() {
    let _cse_temp_0 = b"A".repeat(10000 as usize);
    let data = _cse_temp_0;
    let mut hash_obj = {
                        let mut hasher = Box::new(std::collections::hash_map::DefaultHasher::new()) as Box<dyn DynDigest>;
        hasher.update(data);
        hasher
    };
    let result = {
                hex::encode(hash_obj.finalize_reset())
    };
    assert_eq!(result.len() as i32, 64);
    assert!(result
        .iter()
        .cloned()
        .map(|c| "0123456789abcdef".to_string().contains(&*c))
        .all(|x| x));
    println!("{}", "PASS: test_sha256_large_data");
}
#[doc = "Test that different data produces different hashes."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_hash_different_data() {
    let hash1 = {
                hex::encode(
            {
                                                let mut hasher = Box::new(std::collections::hash_map::DefaultHasher::new()) as Box<dyn DynDigest>;
                hasher.update(b"data1");
                hasher
            }
            .finalize_reset(),
        )
    };
    let hash2 = {
                hex::encode(
            {
                                                let mut hasher = Box::new(std::collections::hash_map::DefaultHasher::new()) as Box<dyn DynDigest>;
                hasher.update(b"data2");
                hasher
            }
            .finalize_reset(),
        )
    };
    assert_ne!(hash1, hash2);
    println!("{}", "PASS: test_hash_different_data");
}
#[doc = "Test that same data produces same hash."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_hash_deterministic() {
    let data = b"deterministic test";
    let hash1 = {
                hex::encode(
            {
                                                let mut hasher = Box::new(std::collections::hash_map::DefaultHasher::new()) as Box<dyn DynDigest>;
                hasher.update(data);
                hasher
            }
            .finalize_reset(),
        )
    };
    let hash2 = {
                hex::encode(
            {
                                                let mut hasher = Box::new(std::collections::hash_map::DefaultHasher::new()) as Box<dyn DynDigest>;
                hasher.update(data);
                hasher
            }
            .finalize_reset(),
        )
    };
    assert_eq!(hash1, hash2);
    println!("{}", "PASS: test_hash_deterministic");
}
#[doc = "Test hashing text(encoded to bytes)."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_sha256_text() {
    let text = "Hello, 世界!";
    let data = text.as_bytes().to_vec();
    let mut hash_obj = {
                        let mut hasher = Box::new(std::collections::hash_map::DefaultHasher::new()) as Box<dyn DynDigest>;
        hasher.update(data);
        hasher
    };
    let result = {
                hex::encode(hash_obj.finalize_reset())
    };
    assert_eq!(result.len() as i32, 64);
    assert!(result
        .iter()
        .cloned()
        .map(|c| "0123456789abcdef".to_string().contains(&*c))
        .all(|x| x));
    println!("{}", "PASS: test_sha256_text");
}
#[doc = "Run all hashlib tests."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() {
    println!("{}", STR__.repeat(60 as usize));
    println!("{}", "HASHLIB MODULE TESTS");
    println!("{}", STR__.repeat(60 as usize));
    test_sha256_basic();
    test_sha256_empty();
    test_sha256_update();
    test_sha1_basic();
    test_md5_basic();
    test_sha256_binary_data();
    test_sha256_large_data();
    test_hash_different_data();
    test_hash_deterministic();
    test_sha256_text();
    println!("{}", STR__.repeat(60 as usize));
    println!("{}", "ALL HASHLIB TESTS PASSED!");
    println!("{}", "Total tests: 10");
    println!("{}", STR__.repeat(60 as usize));
}
