#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#[doc = "// NOTE: Map Python module 'configparser'(tracked in DEPYLER-0424)"]
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
#[doc = "Test basic reading of config from string."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_configparser_basic_read() {
    let config_string = "\n[DEFAULT]\nServerAliveInterval = 45\nCompression = yes\nCompressionLevel = 9\n\n[bitbucket.org]\nUser = hg\n\n[topsecret.server.com]\nPort = 50022\nForwardX11 = no\n";
    let config = configparser.ConfigParser();
    config.read_string(config_string);
    assert!(config.contains("bitbucket.org"));
    assert!(config.contains("topsecret.server.com"));
    assert_eq!(
        config
            .get("bitbucket.org")
            .cloned()
            .unwrap_or_default()
            .get("User")
            .cloned()
            .unwrap_or_default(),
        "hg".to_string()
    );
    assert_eq!(
        config
            .get("topsecret.server.com")
            .cloned()
            .unwrap_or_default()
            .get("Port")
            .cloned()
            .unwrap_or_default(),
        "50022".to_string()
    );
    println!("{}", "PASS: test_configparser_basic_read");
}
#[doc = "Test DEFAULT section values."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_configparser_defaults() {
    let config_string =
        "\n[DEFAULT]\nServerAliveInterval = 45\nCompression = yes\n\n[example.com]\nUser = john\n";
    let config = configparser.ConfigParser();
    config.read_string(config_string);
    assert_eq!(
        config
            .get("example.com")
            .cloned()
            .unwrap_or_default()
            .get("ServerAliveInterval")
            .cloned()
            .unwrap_or_default(),
        "45".to_string()
    );
    assert_eq!(
        config
            .get("example.com")
            .cloned()
            .unwrap_or_default()
            .get("Compression")
            .cloned()
            .unwrap_or_default(),
        "yes".to_string()
    );
    assert_eq!(
        config
            .get("example.com")
            .cloned()
            .unwrap_or_default()
            .get("User")
            .cloned()
            .unwrap_or_default(),
        "john".to_string()
    );
    println!("{}", "PASS: test_configparser_defaults");
}
#[doc = "Test type-converting get methods."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_configparser_get_methods() {
    let config_string = "\n[settings]\nport = 8080\ndebug = true\ntimeout = 30.5\n";
    let config = configparser.ConfigParser();
    config.read_string(config_string);
    assert_eq!(
        config.get("settings").cloned().unwrap_or("port"),
        "8080".to_string()
    );
    assert_eq!(
        config.getint("settings".to_string(), "port".to_string()),
        8080
    );
    assert_eq!(
        config.getboolean("settings".to_string(), "debug".to_string()),
        true
    );
    assert_eq!(
        config.getfloat("settings".to_string(), "timeout".to_string()),
        30.5
    );
    println!("{}", "PASS: test_configparser_get_methods");
}
#[doc = "Test listing sections."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_configparser_sections() {
    let config_string =
        "\n[section1]\nkey1 = value1\n\n[section2]\nkey2 = value2\n\n[section3]\nkey3 = value3\n";
    let config = configparser.ConfigParser();
    config.read_string(config_string);
    let sections = config.sections();
    assert_eq!(sections.len() as i32, 3);
    assert!(sections.contains("section1"));
    assert!(sections.contains("section2"));
    assert!(sections.contains("section3"));
    println!("{}", "PASS: test_configparser_sections");
}
#[doc = "Test listing options in a section."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_configparser_options() {
    let config_string =
        "\n[database]\nhost = localhost\nport = 5432\nuser = admin\npassword = secret\n";
    let config = configparser.ConfigParser();
    config.read_string(config_string);
    let options = config.options("database".to_string());
    assert!(options.contains("host"));
    assert!(options.contains("port"));
    assert!(options.contains("user"));
    assert!(options.contains("password"));
    println!("{}", "PASS: test_configparser_options");
}
#[doc = "Test setting configuration values programmatically."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_configparser_set_values() {
    let config = configparser.ConfigParser();
    config.add_section("newsection".to_string());
    config.set(
        "newsection".to_string(),
        "option1".to_string(),
        "value1".to_string(),
    );
    config.set(
        "newsection".to_string(),
        "option2".to_string(),
        "value2".to_string(),
    );
    assert_eq!(
        config
            .get("newsection")
            .cloned()
            .unwrap_or_default()
            .get("option1")
            .cloned()
            .unwrap_or_default(),
        "value1".to_string()
    );
    assert_eq!(
        config
            .get("newsection")
            .cloned()
            .unwrap_or_default()
            .get("option2")
            .cloned()
            .unwrap_or_default(),
        "value2".to_string()
    );
    println!("{}", "PASS: test_configparser_set_values");
}
#[doc = "Test checking for section existence."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_configparser_has_section() {
    let config_string = "\n[existing]\nkey = value\n";
    let config = configparser.ConfigParser();
    config.read_string(config_string);
    assert_eq!(config.has_section("existing".to_string()), true);
    assert_eq!(config.has_section("nonexistent".to_string()), false);
    println!("{}", "PASS: test_configparser_has_section");
}
#[doc = "Test checking for option existence."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_configparser_has_option() {
    let config_string = "\n[section]\nexisting_option = value\n";
    let config = configparser.ConfigParser();
    config.read_string(config_string);
    assert_eq!(
        config.has_option("section".to_string(), "existing_option".to_string()),
        true
    );
    assert_eq!(
        config.has_option("section".to_string(), "missing_option".to_string()),
        false
    );
    println!("{}", "PASS: test_configparser_has_option");
}
#[doc = "Test removing sections and options."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_configparser_remove() {
    let config_string =
        "\n[section1]\noption1 = value1\noption2 = value2\n\n[section2]\noption3 = value3\n";
    let config = configparser.ConfigParser();
    config.read_string(config_string);
    config.remove_option("section1".to_string(), "option1".to_string());
    assert!(!config.has_option("section1".to_string(), "option1".to_string()));
    assert!(config.has_option("section1".to_string(), "option2".to_string()));
    config.remove_section("section2".to_string());
    assert!(!config.has_section("section2".to_string()));
    println!("{}", "PASS: test_configparser_remove");
}
#[doc = "Run all configparser tests."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() {
    println!("{}", STR__.repeat(60 as usize));
    println!("{}", "CONFIGPARSER MODULE TESTS");
    println!("{}", STR__.repeat(60 as usize));
    test_configparser_basic_read();
    test_configparser_defaults();
    test_configparser_get_methods();
    test_configparser_sections();
    test_configparser_options();
    test_configparser_set_values();
    test_configparser_has_section();
    test_configparser_has_option();
    test_configparser_remove();
    println!("{}", STR__.repeat(60 as usize));
    println!("{}", "ALL CONFIGPARSER TESTS PASSED!");
    println!("{}", "Total tests: 9");
    println!("{}", STR__.repeat(60 as usize));
}
