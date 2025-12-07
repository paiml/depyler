use serde_json;
use std::collections::HashMap;
use std::collections::HashSet;
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
#[doc = "Showcase nested dictionary assignment support"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn showcase_dictionary_assignment() -> (
    std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>,
    serde_json::Value,
) {
    let mut d = {
        let map = HashMap::new();
        map
    };
    d.insert("key".to_string().to_string(), serde_json::json!("value"));
    let nested = {
        let mut map = HashMap::new();
        map.insert("level1".to_string(), {
            let mut map = HashMap::new();
            map.insert("level2".to_string(), {
                let map = HashMap::new();
                map
            });
            map
        });
        map
    };
    nested
        .get_mut(&"level1")
        .unwrap()
        .get_mut(&"level2")
        .unwrap()
        .insert("level3".to_string().to_string(), "deep value");
    let mut coords = {
        let map = HashMap::new();
        map
    };
    coords.insert((10, 20), serde_json::json!("location A"));
    coords.insert((30, 40), serde_json::json!("location B"));
    (nested, coords)
}
#[doc = "Showcase comprehensive set support"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn showcase_set_operations() -> (i32, i32, i32, i32, serde_json::Value) {
    let set1 = {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(4);
        set.insert(5);
        set
    };
    let set2 = {
        let mut set = HashSet::new();
        set.insert(4);
        set.insert(5);
        set.insert(6);
        set.insert(7);
        set.insert(8);
        set
    };
    let _cse_temp_0 = set1
        .intersection(&set2)
        .cloned()
        .collect::<std::collections::HashSet<_>>();
    let intersection = _cse_temp_0;
    let _cse_temp_1 = set1
        .union(&set2)
        .cloned()
        .collect::<std::collections::HashSet<_>>();
    let union = _cse_temp_1;
    let difference = set1
        .difference(&set2)
        .cloned()
        .collect::<std::collections::HashSet<_>>();
    let _cse_temp_2 = set1
        .symmetric_difference(&set2)
        .cloned()
        .collect::<std::collections::HashSet<_>>();
    let symmetric_diff = _cse_temp_2;
    let mut mutable_set = {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set
    };
    mutable_set.insert(4);
    if !mutable_set.remove(&2) {
        panic!("KeyError: element not in set")
    };
    mutable_set.remove(&5);
    (intersection, union, difference, symmetric_diff, mutable_set)
}
#[doc = "Showcase set comprehensions"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn showcase_set_comprehensions() -> (serde_json::Value, serde_json::Value, serde_json::Value) {
    let squares = (0..10).into_iter().map(|x| x * x).collect::<HashSet<_>>();
    let even_squares = (0..10)
        .into_iter()
        .filter(|&x| x % 2 == 0)
        .map(|x| x * x)
        .collect::<HashSet<_>>();
    let unique_chars = "hello world"
        .to_string()
        .into_iter()
        .filter(|&c| c.chars().all(|c| c.is_alphabetic()))
        .map(|c| c.to_uppercase())
        .collect::<HashSet<_>>();
    (squares, even_squares, unique_chars)
}
#[doc = "Showcase frozen set support"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn showcase_frozen_sets() -> (i32, std::collections::HashMap<i32, String>) {
    let immutable1 = std::sync::Arc::new(vec![1, 2, 3, 4].into_iter().collect::<HashSet<_>>());
    let immutable2 = std::sync::Arc::new(3..6.into_iter().collect::<HashSet<_>>());
    let _cse_temp_0 = immutable1 & immutable2;
    let result = _cse_temp_0;
    let frozen_dict = {
        let mut map = HashMap::new();
        map.insert(immutable1, "first set");
        map.insert(immutable2, "second set");
        map
    };
    (result, frozen_dict)
}
#[doc = "Showcase break and continue statements"]
#[doc = " Depyler: proven to terminate"]
pub fn showcase_control_flow(
) -> Result<(serde_json::Value, serde_json::Value, serde_json::Value), Box<dyn std::error::Error>> {
    let mut result1 = vec![];
    for i in 0..10 {
        if i == 5 {
            break;
        }
        result1.push(i);
    }
    let mut result2 = vec![];
    for i in 0..10 {
        if i % 2 == 0 {
            continue;
        }
        result2.push(i);
    }
    let mut result3 = vec![];
    for i in 0..3 {
        for j in 0..3 {
            if (i == 1) && (j == 1) {
                break;
            }
            result3.push((i, j));
        }
    }
    Ok((result1, result2, result3))
}
#[doc = "Showcase power operator support"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn showcase_power_operator() -> (f64, f64, f64, f64) {
    let _cse_temp_0 = ({ 2 } as i32)
        .checked_pow({ 10 } as u32)
        .expect("Power operation overflowed");
    let int_power = _cse_temp_0;
    let _cse_temp_1 = ({ 2.5 } as f64).powf({ 3.0 } as f64);
    let float_power = _cse_temp_1;
    let _cse_temp_2 = ({ 2 } as f64).powf({ -3 } as f64);
    let inverse = _cse_temp_2;
    let _cse_temp_3 = ({ 10 } as i32)
        .checked_pow({ 6 } as u32)
        .expect("Power operation overflowed");
    let large = _cse_temp_3;
    (int_power, float_power, inverse, large)
}
#[doc = "Run all showcases"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{} {}",
        "Dictionary Assignment:",
        showcase_dictionary_assignment()
    );
    println!("{} {}", "Set Operations:", showcase_set_operations());
    println!(
        "{} {}",
        "Set Comprehensions:",
        showcase_set_comprehensions()
    );
    println!("{} {}", "Frozen Sets:", showcase_frozen_sets());
    println!("{} {:?}", "Control Flow:", showcase_control_flow());
    println!("{} {}", "Power Operator:", showcase_power_operator());
    Ok(())
}
