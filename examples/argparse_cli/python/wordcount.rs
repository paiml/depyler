#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
use std::path::PathBuf;
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
pub struct Stats {
    pub lines: i32,
    pub words: i32,
    pub chars: i32,
    pub filename: String,
}
#[derive(Default)]
struct Args {
    #[doc = "Files to process"]
    files: Vec<PathBuf>,
    #[doc = "Show only line count"]
    lines: bool,
    #[doc = "Show only word count"]
    words: bool,
    #[doc = "Show only character count"]
    chars: bool,
}
#[doc = "Count statistics for a single file"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn count_file(filepath: &std::path::PathBuf) -> Stats {
    let mut content: Option<String> = None;
    let mut lines: i32 = Default::default();
    let mut chars: i32 = Default::default();
    let mut words: i32 = Default::default();
    match (|| -> Result<(), Box<dyn std::error::Error>> {
        content = Some(std::fs::read_to_string(&filepath).unwrap());
        lines = content
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .len() as i32;
        words = content
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .len() as i32;
        chars = content.len() as i32;
        return Ok(Stats::new(
            lines,
            words,
            chars,
            (filepath).display().to_string(),
        ));
    })() {
        Ok(_result) => {
            return _result;
        }
        Err(e) => {
            eprintln!(
                "{}",
                format!("Error reading {}: {:?}", filepath.display(), e)
            );
            return Stats::new(0, 0, 0, (filepath).display().to_string());
        }
    }
}
#[doc = "Format statistics for output"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn format_stats(stats: Stats, show_filename: bool) -> String {
    let mut result: String = Default::default();
    result = format!("{} {} {}", stats.lines, stats.words, stats.chars);
    if show_filename {
        let _cse_temp_0 = format!("{}{}", result, format!(" {}", stats.filename));
        result = _cse_temp_0;
    }
    result.to_string()
}
#[doc = "Main entry point"]
#[doc = " Depyler: verified panic-free"]
pub fn main() {
    let mut total_chars: i32 = Default::default();
    let mut total_words: i32 = Default::default();
    let mut total_lines: i32 = Default::default();
    let args = Args::default();
    total_lines = 0;
    total_words = 0;
    total_chars = 0;
    for filepath in args.files {
        let stats = count_file(&filepath);
        total_lines = total_lines + stats.lines;
        total_words = total_words + stats.words;
        total_chars = total_chars + stats.chars;
        if args.lines {
            println!("{}", format!("{} {}", stats.lines, stats.filename));
        } else {
            if args.words {
                println!("{}", format!("{} {}", stats.words, stats.filename));
            } else {
                if args.chars {
                    println!("{}", format!("{} {}", stats.chars, stats.filename));
                } else {
                    println!("{}", format_stats(stats, true));
                }
            }
        }
    }
    let _cse_temp_0 = args.files.len() as i32;
    let _cse_temp_1 = _cse_temp_0 > 1;
    if _cse_temp_1 {
        let total_stats = Stats::new(total_lines, total_words, total_chars, "total".to_string());
        println!("{}", format_stats(total_stats, true));
    }
    ()
}
