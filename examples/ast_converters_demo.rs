use std::f64 as math;
pub const int_literal: i32 = 42;
pub const float_literal: f64 = 3.14;
pub const string_literal: &str = "hello world";
pub const bool_literal: bool = true;
pub const none_literal: serde_json::Value = None;
pub const addition: serde_json::Value = 10 + 20;
pub const subtraction: serde_json::Value = 50 - 15;
pub const multiplication: serde_json::Value = 7 * 8;
pub const division: serde_json::Value = 100 / 4;
pub const modulo: serde_json::Value = 17 % 5;
pub const power: serde_json::Value = ({ 2 } as i32)
    .checked_pow({ 8 } as u32)
    .expect("Power operation overflowed");
pub const greater_than: serde_json::Value = 10 > 5;
pub const less_than: serde_json::Value = 3 < 7;
pub const equal_to: serde_json::Value = 42 == 42;
pub const not_equal: serde_json::Value = "a".to_string() != "b".to_string();
pub const greater_equal: serde_json::Value = 100 >= 100;
pub const less_equal: serde_json::Value = 50 <= 60;
pub const and_op: serde_json::Value = (true) && (false);
pub const or_op: serde_json::Value = (true) || (false);
pub const not_op: serde_json::Value = !true;
pub const negation: i32 = -42;
pub const positive: i32 = 42;
pub const bitwise_not: serde_json::Value = !255;
pub static list_example: once_cell::sync::Lazy<serde_json::Value> =
    once_cell::sync::Lazy::new(|| serde_json::to_value(vec![1, 2, 3, 4, 5]).unwrap());
pub static tuple_example: once_cell::sync::Lazy<serde_json::Value> =
    once_cell::sync::Lazy::new(|| (1, "hello".to_string().to_string(), 3.14, true));
pub static dict_example: once_cell::sync::Lazy<serde_json::Value> =
    once_cell::sync::Lazy::new(|| {
        serde_json::to_value({
            let mut map = std::collections::HashMap::new();
            map.insert("name".to_string(), serde_json::json!("John".to_string()));
            map.insert("age".to_string(), serde_json::json!(30));
            map.insert("city".to_string(), serde_json::json!("NYC".to_string()));
            map
        })
        .unwrap()
    });
pub static set_example: once_cell::sync::Lazy<serde_json::Value> =
    once_cell::sync::Lazy::new(|| {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(4);
        set.insert(5);
        set
    });
pub const list_index: serde_json::Value = list_example.get(0usize).cloned().unwrap_or_default();
pub const dict_access: serde_json::Value = dict_example.get("name").cloned().unwrap_or_default();
pub const slice_example: serde_json::Value = {
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
pub const slice_with_step: serde_json::Value = {
    let base = list_example;
    let step = 2;
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
pub const slice_reverse: serde_json::Value = {
    let base = list_example;
    let step = -1;
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
pub const list_comp: serde_json::Value = (0..10).into_iter().map(|x| x * 2).collect::<Vec<_>>();
pub const list_comp_filtered: serde_json::Value = (0..20)
    .into_iter()
    .filter(|&x| x % 2 == 0)
    .map(|x| x)
    .collect::<Vec<_>>();
pub const set_comp: serde_json::Value = (0..5)
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
    .collect::<HashSet<_>>();
pub const dict_comp: serde_json::Value = (0..5)
    .into_iter()
    .map(|x| {
        (x, {
            if 2 >= 0 && (2 as i64) <= (u32::MAX as i64) {
                ({ x } as i32)
                    .checked_pow({ 2 } as u32)
                    .expect("Power operation overflowed")
            } else {
                ({ x } as f64).powf({ 2 } as f64) as i32
            }
        })
    })
    .collect::<std::collections::HashMap<_, _>>();
pub const simple_call: serde_json::Value = println!("{}", "Hello".to_string());
pub const method_call: serde_json::Value = "hello".to_string().to_uppercase();
pub const chained_calls: serde_json::Value =
    "  hello  ".to_string().trim().to_string().to_uppercase();
pub const pi_value: serde_json::Value = std::f64::consts::PI;
pub const module_function: serde_json::Value = (16 as f64).sqrt();
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
use once_cell::sync::Lazy;
use serde_json;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Read;
use std::io::Write;
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
#[derive(Debug, Clone)]
pub struct DemoClass {
    pub value: serde_json::Value,
    pub data: Vec<serde_json::Value>,
}
impl DemoClass {
    pub fn new(value: serde_json::Value) -> Self {
        Self {
            value,
            data: Vec::new(),
        }
    }
    pub fn method(&self) -> i32 {
        return self.value * 2;
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
    for i in 0..10 {
        if i == 5 {
            continue;
        }
        if i == 8 {
            break;
        }
        println!("{}", i);
    }
    for i in 0..3 {
        println!("{}", i);
    }
    for i in 0..3 {
        for j in 0..3 {
            println!("{}", format!("({:?}, {:?})", i, j));
        }
    }
    let _cse_temp_5 = x > 100;
    if _cse_temp_5 {
        Ok(x)
    } else {
        let _cse_temp_6 = x > 50;
        if _cse_temp_6 {
            Ok(x * 2)
        } else {
            Ok(None)
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
    let transformed = (0..5)
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
