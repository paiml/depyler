#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
use std::f64 as math;
pub const int_literal: i32 = 42;
pub const float_literal: f64 = 3.14;
pub const string_literal: &str = "hello world";
pub const bool_literal: bool = true;
pub const none_literal: Option<()> = None;
pub const addition: i32 = 10 + 20;
pub const subtraction: i32 = 50 - 15;
pub const multiplication: i32 = 7 * 8;
pub const division: i32 = 100 / 4;
pub const modulo: i32 = 17 % 5;
pub const power: i32 = ({ 2 } as i32)
    .checked_pow({ 8 } as u32)
    .expect("Power operation overflowed");
pub const greater_than: bool = 10 > 5;
pub const less_than: bool = 3 < 7;
pub const equal_to: bool = 42 == 42;
pub const not_equal: bool = "a".to_string() != "b".to_string();
pub const greater_equal: bool = 100 >= 100;
pub const less_equal: bool = 50 <= 60;
pub const and_op: bool = (true) && (false);
pub const or_op: bool = (true) || (false);
pub const not_op: String = !true;
pub const negation: i32 = -42;
pub const positive: i32 = 42;
pub const bitwise_not: String = !255;
pub static list_example: std::sync::LazyLock<Vec<String>> =
    std::sync::LazyLock::new(|| vec![1, 2, 3, 4, 5]);
pub static tuple_example: std::sync::LazyLock<String> =
    std::sync::LazyLock::new(|| (1, "hello".to_string().to_string(), 3.14, true));
pub static dict_example: std::sync::LazyLock<std::collections::HashMap<String, String>> =
    std::sync::LazyLock::new(|| {
        let mut map = HashMap::new();
        map.insert(
            "name".to_string().to_string(),
            DepylerValue::Str("John".to_string().to_string()),
        );
        map.insert("age".to_string().to_string(), DepylerValue::Int(30 as i64));
        map.insert(
            "city".to_string().to_string(),
            DepylerValue::Str("NYC".to_string().to_string()),
        );
        map
    });
pub static set_example: std::sync::LazyLock<std::collections::HashSet<String>> =
    std::sync::LazyLock::new(|| {
        let mut set = std::collections::HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(4);
        set.insert(5);
        set
    });
pub const list_index: i32 = list_example
    .get(0usize)
    .cloned()
    .expect("IndexError: list index out of range");
pub const dict_access: i32 = dict_example.get("name").cloned().unwrap_or_default();
pub const slice_example: String = {
    let base = &list_example;
    let start_idx = 1 as isize;
    let stop_idx = 4 as isize;
    let start = if start_idx < 0 {
        (base.len() as isize + start_idx).max(0) as usize
    } else {
        start_idx as usize
    };
    let stop = if stop_idx < 0 {
        (base.len() as isize + stop_idx).max(0) as usize
    } else {
        stop_idx as usize
    };
    if start < base.len() {
        base[start..stop.min(base.len())].to_vec()
    } else {
        Vec::new()
    }
};
pub const slice_with_step: String = {
    let base = list_example;
    let step: i32 = 2;
    if step == 1 {
        base.clone()
    } else if step > 0 {
        base.iter()
            .step_by(step as usize)
            .cloned()
            .collect::<Vec<_>>()
    } else if step == -1 {
        base.iter().rev().cloned().collect::<Vec<_>>()
    } else {
        let abs_step = (-step) as usize;
        base.iter()
            .rev()
            .step_by(abs_step)
            .cloned()
            .collect::<Vec<_>>()
    }
};
pub const slice_reverse: String = {
    let base = list_example;
    let step: i32 = -1;
    if step == 1 {
        base.clone()
    } else if step > 0 {
        base.iter()
            .step_by(step as usize)
            .cloned()
            .collect::<Vec<_>>()
    } else if step == -1 {
        base.iter().rev().cloned().collect::<Vec<_>>()
    } else {
        let abs_step = (-step) as usize;
        base.iter()
            .rev()
            .step_by(abs_step)
            .cloned()
            .collect::<Vec<_>>()
    }
};
pub const list_comp: Vec<i32> = (0..(10)).into_iter().map(|x| x * 2).collect::<Vec<_>>();
pub const list_comp_filtered: Vec<DepylerValue> = (0..(20))
    .into_iter()
    .filter(|x| {
        let x = x.clone();
        x % 2 == 0
    })
    .map(|x| x)
    .collect::<Vec<_>>();
pub const set_comp: std::collections::HashSet<i32> = (0..(5))
    .into_iter()
    .map(|x| {
        if 2 >= 0 && (2 as i64) <= (u32::MAX as i64) {
            ({ x } as i32)
                .checked_pow({ 2 } as u32)
                .expect("Power operation overflowed")
        } else {
            ({ x } as f64).powf({ 2 } as f64) as i32
        }
    })
    .collect::<std::collections::HashSet<_>>();
pub const dict_comp: std::collections::HashMap<String, i32> = (0..(5))
    .into_iter()
    .map(|x| {
        let _v = {
            if 2 >= 0 && (2 as i64) <= (u32::MAX as i64) {
                ({ x } as i32)
                    .checked_pow({ 2 } as u32)
                    .expect("Power operation overflowed")
            } else {
                ({ x } as f64).powf({ 2 } as f64) as i32
            }
        };
        (x, _v)
    })
    .collect::<std::collections::HashMap<_, _>>();
pub static simple_call: std::sync::LazyLock<String> =
    std::sync::LazyLock::new(|| println!("{}", "Hello".to_string()).unwrap());
pub const method_call: String = "hello".to_string().to_uppercase();
pub const chained_calls: String = "  hello  ".to_string().trim().to_string().to_uppercase();
pub const pi_value: String = std::f64::consts::PI;
pub const module_function: String = (16 as f64).sqrt();
pub fn square(x: i32) -> i32 {
    {
        if 2 >= 0 && (2 as i64) <= (u32::MAX as i64) {
            ({ x } as i32)
                .checked_pow({ 2 } as u32)
                .expect("Power operation overflowed")
        } else {
            ({ x } as f64).powf({ 2 } as f64) as i32
        }
    }
}
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}
pub fn conditional_lambda(x: i32) -> i32 {
    if x > 0 {
        x
    } else {
        -x
    }
}
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Read;
use std::io::Write;
use std::sync::LazyLock;
#[derive(Debug, Clone)]
pub struct ZeroDivisionError {
    message: String,
}
impl std::fmt::Display for ZeroDivisionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "division by zero: {}", self.message)
    }
}
impl std::error::Error for ZeroDivisionError {}
impl ZeroDivisionError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ValueError {
    message: String,
}
impl std::fmt::Display for ValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "value error: {}", self.message)
    }
}
impl std::error::Error for ValueError {}
impl ValueError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
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
pub struct DemoClass {
    pub value: DepylerValue,
    pub data: Vec<DepylerValue>,
}
impl DemoClass {
    pub fn new(value: DepylerValue) -> Self {
        Self {
            value,
            data: Vec::new(),
        }
    }
    pub fn method(&self) -> i32 {
        return self.value.clone() * 2;
    }
    pub fn chain_example(&self) -> i32 {
        return self.method() + 10;
    }
}
#[doc = "Show various statement types."]
pub fn demonstrate_statements() -> Result<i32, Box<dyn std::error::Error>> {
    let mut x = 10;
    let mut y = 20;
    x = x + 5;
    let _cse_temp_0 = y * 2;
    y = _cse_temp_0;
    let _cse_temp_1 = {
        let a = x;
        let b = 3;
        let q = a / b;
        let r = a % b;
        let r_negative = r < 0;
        let b_negative = b < 0;
        let r_nonzero = r != 0;
        let signs_differ = r_negative != b_negative;
        let needs_adjustment = r_nonzero && signs_differ;
        if needs_adjustment {
            q - 1
        } else {
            q
        }
    };
    x = _cse_temp_1;
    let _cse_temp_2 = x > 0;
    if _cse_temp_2 {
        println!("{}", "Positive");
    } else {
        let _cse_temp_3 = x < 0;
        if _cse_temp_3 {
            println!("{}", "Negative");
        } else {
            println!("{}", "Zero");
        }
    }
    if _cse_temp_2 {
        let _cse_temp_4 = x > 10;
        if _cse_temp_4 {
            println!("{}", "Greater than 10");
        } else {
            println!("{}", "Between 1 and 10");
        }
    }
    let mut counter = 0;
    while counter < 5 {
        println!("{}", counter);
        counter = counter + 1;
    }
    for i in 0..(10) {
        if i == 5 {
            continue;
        }
        if i == 8 {
            break;
        }
        println!("{}", i);
    }
    for i in 0..(3) {
        println!("{}", i);
    }
    for i in 0..(3) {
        for j in 0..(3) {
            println!("{}", format!("({}, {})", i, j));
        }
    }
    let _cse_temp_5 = x > 100;
    if _cse_temp_5 {
        return Ok(x);
    } else {
        let _cse_temp_6 = x > 50;
        if _cse_temp_6 {
            return Ok(x * 2);
        } else {
            return Ok(None);
        }
    }
}
#[doc = "Show advanced statement types."]
#[doc = " Depyler: proven to terminate"]
pub fn demonstrate_advanced() -> Result<String, Box<dyn std::error::Error>> {
    let mut f = std::fs::File::create("file.txt".to_string())?;
    f.write_all("Hello, World!".to_string().as_bytes()).unwrap();
    if false {
        return Err(Box::new(ValueError::new(
            "Something went wrong".to_string(),
        )));
    }
    let result = "  Hello World  "
        .to_string()
        .trim()
        .to_string()
        .to_lowercase()
        .replace(" ", "_");
    Ok(result.to_string())
}
#[doc = "Show various comprehension types."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn demonstrate_comprehensions() -> Vec<(i32, i32, i32)> {
    let transformed = (0..(5))
        .into_iter()
        .map(|x| {
            (
                x,
                {
                    if 2 >= 0 && (2 as i64) <= (u32::MAX as i64) {
                        ({ x } as i32)
                            .checked_pow({ 2 } as u32)
                            .expect("Power operation overflowed")
                    } else {
                        ({ x } as f64).powf({ 2 } as f64) as i32
                    }
                },
                {
                    if 3 >= 0 && (3 as i64) <= (u32::MAX as i64) {
                        ({ x } as i32)
                            .checked_pow({ 3 } as u32)
                            .expect("Power operation overflowed")
                    } else {
                        ({ x } as f64).powf({ 3 } as f64) as i32
                    }
                },
            )
        })
        .collect::<Vec<_>>();
    transformed
}
