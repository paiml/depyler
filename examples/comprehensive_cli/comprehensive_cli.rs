#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
use std::collections::HashMap;
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
#[derive(Default)]
struct Args {
    #[doc = "Input file to process"]
    input_file: String,
    #[doc = "Verbose mode(short only)"]
    v: Option<String>,
    #[doc = "Debug mode(long only)"]
    debug: bool,
    #[doc = "Output file(short + long)"]
    output: Option<String>,
    #[doc = "Quiet mode"]
    quiet: bool,
    #[doc = "Disable colors"]
    color: bool,
    #[doc = "Verbosity level"]
    V: u8,
    #[doc = "Include paths"]
    include: Vec<String>,
    #[doc = "Extra files(1 or more)"]
    extras: Vec<String>,
    #[doc = "Optional args(0 or more)"]
    optional: Option<Vec<String>>,
    #[doc = "Config file(0 or 1)"]
    config: Option<String>,
    #[doc = "X Y coordinates"]
    coords: Vec<f64>,
    #[doc = "Item count"]
    count: Option<i32>,
    #[doc = "Processing rate"]
    rate: Option<f64>,
    #[doc = "Message text"]
    message: Option<String>,
    #[doc = "Path argument"]
    path: Option<PathBuf>,
    #[doc = "Timeout seconds"]
    timeout: i32,
    #[doc = "Threshold"]
    threshold: f64,
    #[doc = "Project name"]
    name: String,
    #[doc = "API key(required)"]
    api_key: String,
    #[doc = "Token(optional)"]
    token: Option<String>,
    #[doc = "Custom dest"]
    custom_input: Option<String>,
    #[doc = "With metavar"]
    file: Option<String>,
    #[doc = "Output format"]
    format: Option<String>,
    #[doc = "Processing mode"]
    mode: Option<String>,
    #[doc = "Fast mode"]
    speed: bool,
}
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() {
    let args = Args::default();
    let has_v = args.v.is_some();
    let has_output = args.output.is_some();
    let has_config = args.config.is_some();
    let has_count = args.count.is_some();
    let has_rate = args.rate.is_some();
    let has_message = args.message.is_some();
    let has_path = args.path.is_some();
    let has_token = args.token.is_some();
    let has_custom_input = args.custom_input.is_some();
    let has_file = args.file.is_some();
    let has_format = args.format.is_some();
    let has_mode = args.mode.is_some();
    let result = {
        let mut map = HashMap::new();
        map.insert(
            "input_file".to_string(),
            DepylerValue::Str(format!("{:?}", args.input_file)),
        );
        map.insert(
            "debug".to_string(),
            DepylerValue::Str(format!("{:?}", args.debug)),
        );
        map.insert(
            "output".to_string(),
            DepylerValue::Str(format!("{:?}", args.output)),
        );
        map.insert(
            "quiet".to_string(),
            DepylerValue::Str(format!("{:?}", args.quiet)),
        );
        map.insert(
            "color".to_string(),
            DepylerValue::Str(format!("{:?}", args.color)),
        );
        map.insert(
            "verbosity".to_string(),
            DepylerValue::Str(format!("{:?}", args.V)),
        );
        map.insert(
            "includes".to_string(),
            DepylerValue::Str(format!("{:?}", args.include)),
        );
        map.insert(
            "extras".to_string(),
            DepylerValue::Str(format!("{:?}", args.extras)),
        );
        map.insert(
            "config".to_string(),
            DepylerValue::Str(format!("{:?}", args.config)),
        );
        map.insert(
            "count".to_string(),
            DepylerValue::Str(format!("{:?}", args.count)),
        );
        map.insert(
            "rate".to_string(),
            DepylerValue::Str(format!("{:?}", args.rate)),
        );
        map.insert(
            "timeout".to_string(),
            DepylerValue::Str(format!("{:?}", args.timeout)),
        );
        map.insert(
            "threshold".to_string(),
            DepylerValue::Str(format!("{:?}", args.threshold)),
        );
        map.insert(
            "name".to_string(),
            DepylerValue::Str(format!("{:?}", args.name)),
        );
        map.insert(
            "api_key".to_string(),
            DepylerValue::Str(format!("{:?}", args.api_key)),
        );
        map.insert(
            "format".to_string(),
            DepylerValue::Str(format!("{:?}", args.format)),
        );
        map.insert(
            "mode".to_string(),
            DepylerValue::Str(format!("{:?}", args.mode)),
        );
        map.insert(
            "speed".to_string(),
            DepylerValue::Str(format!("{:?}", args.speed)),
        );
        map
    };
    println!("{}", format!("{:?}", result));
}
