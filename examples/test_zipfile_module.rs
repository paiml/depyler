#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#[doc = "// NOTE: Map Python module 'zipfile'(tracked in DEPYLER-0424)"]
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
#[doc = "Test creating a ZIP file and reading it back."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_zipfile_create_and_read() {
    let mut buffer = std::io::Cursor();
    let mut _context = zipfile.ZipFile(buffer, "w".to_string());
    let zf = _context.__enter__();
    zf.writestr("test.txt".to_string(), "Hello, ZIP!".to_string());
    buffer.seek(0);
    let mut _context = zipfile.ZipFile(buffer, "r".to_string());
    let zf = _context.__enter__();
    let content = {
        let mut _read_buf = vec![0u8; "test.txt".to_string()];
        let _n = zf.read(&mut _read_buf).unwrap_or(0);
        _read_buf.truncate(_n);
        _read_buf
    };
    assert_eq!(content, b"Hello, ZIP!");
    println!("{}", "PASS: test_zipfile_create_and_read");
}
#[doc = "Test ZIP with multiple files."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_zipfile_multiple_files() {
    let mut buffer = std::io::Cursor();
    let mut _context = zipfile.ZipFile(buffer, "w".to_string());
    let zf = _context.__enter__();
    zf.writestr("file1.txt".to_string(), "Content 1".to_string());
    zf.writestr("file2.txt".to_string(), "Content 2".to_string());
    zf.writestr("file3.txt".to_string(), "Content 3".to_string());
    buffer.seek(0);
    let mut _context = zipfile.ZipFile(buffer, "r".to_string());
    let zf = _context.__enter__();
    assert_eq!(zf.namelist().len() as i32, 3);
    assert!(zf.namelist().contains("file1.txt"));
    assert!(zf.namelist().contains("file2.txt"));
    assert!(zf.namelist().contains("file3.txt"));
    assert_eq!(
        {
            let mut _read_buf = vec![0u8; "file2.txt".to_string()];
            let _n = zf.read(&mut _read_buf).unwrap_or(0);
            _read_buf.truncate(_n);
            _read_buf
        },
        b"Content 2"
    );
    println!("{}", "PASS: test_zipfile_multiple_files");
}
#[doc = "Test listing files in ZIP archive."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_zipfile_namelist() {
    let mut buffer = std::io::Cursor();
    let mut _context = zipfile.ZipFile(buffer, "w".to_string());
    let zf = _context.__enter__();
    zf.writestr("alpha.txt".to_string(), "A".to_string());
    zf.writestr("beta.txt".to_string(), "B".to_string());
    zf.writestr("gamma.txt".to_string(), "C".to_string());
    buffer.seek(0);
    let mut _context = zipfile.ZipFile(buffer, "r".to_string());
    let zf = _context.__enter__();
    let names = zf.namelist();
    assert_eq!(names.len() as i32, 3);
    assert!(names.contains("alpha.txt"));
    assert!(names.contains("beta.txt"));
    assert!(names.contains("gamma.txt"));
    println!("{}", "PASS: test_zipfile_namelist");
}
#[doc = "Test getting file info from ZIP."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_zipfile_getinfo() {
    let mut buffer = std::io::Cursor();
    let mut _context = zipfile.ZipFile(buffer, "w".to_string());
    let zf = _context.__enter__();
    zf.writestr("data.txt".to_string(), "Test data content".to_string());
    buffer.seek(0);
    let mut _context = zipfile.ZipFile(buffer, "r".to_string());
    let zf = _context.__enter__();
    let info = zf.getinfo("data.txt".to_string());
    assert_eq!(info.filename, "data.txt".to_string());
    assert_eq!(info.file_size, "Test data content".to_string().len() as i32);
    println!("{}", "PASS: test_zipfile_getinfo");
}
#[doc = "Test ZIP with compression."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_zipfile_compression() {
    let mut buffer = std::io::Cursor();
    let _cse_temp_0 = "This is test data that should compress well! ".repeat(10 as usize);
    let data = _cse_temp_0;
    let mut _context = zipfile.ZipFile(buffer, "w".to_string(), zipfile.ZIP_DEFLATED);
    let zf = _context.__enter__();
    zf.writestr("compressed.txt".to_string(), data);
    buffer.seek(0);
    let mut _context = zipfile.ZipFile(buffer, "r".to_string());
    let zf = _context.__enter__();
    let content = String::from_utf8_lossy(&{
        let mut _read_buf = vec![0u8; "compressed.txt".to_string()];
        let _n = zf.read(&mut _read_buf).unwrap_or(0);
        _read_buf.truncate(_n);
        _read_buf
    })
    .to_string();
    assert_eq!(content, data);
    println!("{}", "PASS: test_zipfile_compression");
}
#[doc = "Test ZIP with binary data."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_zipfile_binary_data() {
    let mut buffer = std::io::Cursor();
    let binary_data = 0..(256);
    let mut _context = zipfile.ZipFile(buffer, "w".to_string());
    let zf = _context.__enter__();
    zf.writestr("binary.dat".to_string(), binary_data);
    buffer.seek(0);
    let mut _context = zipfile.ZipFile(buffer, "r".to_string());
    let zf = _context.__enter__();
    let content = {
        let mut _read_buf = vec![0u8; "binary.dat".to_string()];
        let _n = zf.read(&mut _read_buf).unwrap_or(0);
        _read_buf.truncate(_n);
        _read_buf
    };
    assert_eq!(content, binary_data);
    println!("{}", "PASS: test_zipfile_binary_data");
}
#[doc = "Test creating empty ZIP archive."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_zipfile_empty() {
    let mut buffer = std::io::Cursor();
    let mut _context = zipfile.ZipFile(buffer, "w".to_string());
    let zf = _context.__enter__();
    buffer.seek(0);
    let mut _context = zipfile.ZipFile(buffer, "r".to_string());
    let zf = _context.__enter__();
    assert_eq!(zf.namelist().len() as i32, 0);
    println!("{}", "PASS: test_zipfile_empty");
}
#[doc = "Test reading from existing ZIP."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_zipfile_read_mode() {
    let mut buffer = std::io::Cursor();
    let mut _context = zipfile.ZipFile(buffer, "w".to_string());
    let zf = _context.__enter__();
    zf.writestr("readonly.txt".to_string(), "Read-only content".to_string());
    buffer.seek(0);
    let mut _context = zipfile.ZipFile(buffer, "r".to_string());
    let zf = _context.__enter__();
    let content = {
        let mut _read_buf = vec![0u8; "readonly.txt".to_string()];
        let _n = zf.read(&mut _read_buf).unwrap_or(0);
        _read_buf.truncate(_n);
        _read_buf
    };
    assert_eq!(content, b"Read-only content");
    println!("{}", "PASS: test_zipfile_read_mode");
}
#[doc = "Run all zipfile tests."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() {
    println!("{}", STR__.repeat(60 as usize));
    println!("{}", "ZIPFILE MODULE TESTS");
    println!("{}", STR__.repeat(60 as usize));
    test_zipfile_create_and_read();
    test_zipfile_multiple_files();
    test_zipfile_namelist();
    test_zipfile_getinfo();
    test_zipfile_compression();
    test_zipfile_binary_data();
    test_zipfile_empty();
    test_zipfile_read_mode();
    println!("{}", STR__.repeat(60 as usize));
    println!("{}", "ALL ZIPFILE TESTS PASSED!");
    println!("{}", "Total tests: 8");
    println!("{}", STR__.repeat(60 as usize));
}
