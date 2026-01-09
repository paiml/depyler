#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#[doc = "// NOTE: Map Python module 'enum'(tracked in DEPYLER-0424)"]
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
#[derive(Debug, Clone)]
pub struct Color {}
impl Color {
    pub const RED: i32 = 1;
    pub const GREEN: i32 = 2;
    pub const BLUE: i32 = 3;
    pub fn new() -> Self {
        Self {}
    }
}
#[derive(Debug, Clone)]
pub struct Status {}
impl Status {
    pub const PENDING: i32 = auto();
    pub const APPROVED: i32 = auto();
    pub const REJECTED: i32 = auto();
    pub fn new() -> Self {
        Self {}
    }
}
#[derive(Debug, Clone)]
pub struct Direction {}
impl Direction {
    pub const NORTH: i32 = 0;
    pub const EAST: i32 = 1;
    pub const SOUTH: i32 = 2;
    pub const WEST: i32 = 3;
    pub fn new() -> Self {
        Self {}
    }
}
#[doc = r" Stub for local import from module: #module_name"]
#[doc = r" DEPYLER-0615: Generated to allow standalone compilation"]
#[allow(dead_code, unused_variables)]
pub fn Enum<T: Default>(_args: impl std::any::Any) -> T {
    Default::default()
}
#[doc = r" Stub for local import from module: #module_name"]
#[doc = r" DEPYLER-0615: Generated to allow standalone compilation"]
#[allow(dead_code, unused_variables)]
pub fn IntEnum<T: Default>(_args: impl std::any::Any) -> T {
    Default::default()
}
#[doc = r" Stub for local import from module: #module_name"]
#[doc = r" DEPYLER-0615: Generated to allow standalone compilation"]
#[allow(dead_code, unused_variables)]
pub fn auto<T: Default>(_args: impl std::any::Any) -> T {
    Default::default()
}
#[doc = "Test basic enum value access"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_enum_basic_access() -> i32 {
    let color: i32 = Color::RED;
    color
}
#[doc = "Test enum comparison"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_enum_comparison() -> bool {
    let color1: i32 = Color::RED;
    let color2: i32 = Color::GREEN;
    let _cse_temp_0 = color1 != color2;
    let are_different: bool = _cse_temp_0;
    are_different
}
#[doc = "Test getting enum name(simplified)"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_enum_to_name() -> String {
    let mut name: String = Default::default();
    let color: i32 = Color::BLUE;
    name = "".to_string();
    let _cse_temp_0 = color == Color::RED;
    if _cse_temp_0 {
        name = "RED".to_string();
    } else {
        if _cse_temp_0 {
            name = "GREEN".to_string();
        } else {
            if _cse_temp_0 {
                name = "BLUE".to_string();
            }
        }
    }
    name.to_string()
}
#[doc = "Test getting enum value"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_enum_to_value() -> i32 {
    let color: i32 = Color::RED;
    let value: i32 = color;
    value
}
#[doc = "Test creating enum from value"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_enum_from_value(value: i32) -> i32 {
    let mut result: i32 = Default::default();
    let _cse_temp_0 = value == Color::RED;
    let _cse_temp_1 = (_cse_temp_0) || (_cse_temp_0);
    let _cse_temp_2 = (_cse_temp_1) || (_cse_temp_0);
    if _cse_temp_2 {
        result = value;
    } else {
        result = Color::RED;
    }
    result
}
#[doc = "Test status enumeration"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_status_enum() -> i32 {
    let mut status: i32 = Default::default();
    status = Status::PENDING;
    let _cse_temp_0 = status == Status::PENDING;
    if _cse_temp_0 {
        status = Status::APPROVED;
    }
    status
}
#[doc = "Test direction enumeration"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_direction_enum() -> i32 {
    let mut current: i32 = Default::default();
    current = Direction::NORTH;
    let _cse_temp_0 = current == Direction::NORTH;
    if _cse_temp_0 {
        current = Direction::EAST;
    } else {
        if _cse_temp_0 {
            current = Direction::SOUTH;
        } else {
            if _cse_temp_0 {
                current = Direction::WEST;
            } else {
                if _cse_temp_0 {
                    current = Direction::NORTH;
                }
            }
        }
    }
    current
}
#[doc = "Rotate direction 90 degrees"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn rotate_direction(direction: i32, clockwise: bool) -> i32 {
    if clockwise {
        let _cse_temp_0 = direction == Direction::NORTH;
        if _cse_temp_0 {
            return Direction::EAST;
        } else {
            if _cse_temp_0 {
                return Direction::SOUTH;
            } else {
                if _cse_temp_0 {
                    return Direction::WEST;
                } else {
                    return Direction::NORTH;
                }
            }
        }
    } else {
        let _cse_temp_1 = direction == Direction::NORTH;
        if _cse_temp_1 {
            return Direction::WEST;
        } else {
            if _cse_temp_1 {
                return Direction::SOUTH;
            } else {
                if _cse_temp_1 {
                    return Direction::EAST;
                } else {
                    return Direction::NORTH;
                }
            }
        }
    }
}
#[doc = "Get opposite direction"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn opposite_direction(direction: i32) -> i32 {
    let _cse_temp_0 = direction == Direction::NORTH;
    if _cse_temp_0 {
        return Direction::SOUTH;
    } else {
        if _cse_temp_0 {
            return Direction::NORTH;
        } else {
            if _cse_temp_0 {
                return Direction::WEST;
            } else {
                return Direction::EAST;
            }
        }
    }
}
#[doc = "Check if direction is horizontal"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn is_horizontal(direction: i32) -> bool {
    (direction == Direction::EAST) || (direction == Direction::WEST)
}
#[doc = "Check if direction is vertical"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn is_vertical(direction: i32) -> bool {
    (direction == Direction::NORTH) || (direction == Direction::SOUTH)
}
#[doc = "Test iterating over enum values"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_enum_iteration() -> Vec<i32> {
    let colors: Vec<i32> = vec![Color::RED, Color::GREEN, Color::BLUE];
    colors
}
#[doc = "Test counting enum members"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_enum_count() -> i32 {
    let colors: Vec<i32> = vec![Color::RED, Color::GREEN, Color::BLUE];
    let _cse_temp_0 = colors.len() as i32;
    let count: i32 = _cse_temp_0;
    count
}
#[doc = "Convert color enum to RGB values"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn color_to_rgb(color: i32) -> (i32, i32, i32) {
    let _cse_temp_0 = color == Color::RED;
    if _cse_temp_0 {
        return (255, 0, 0);
    } else {
        if _cse_temp_0 {
            return (0, 255, 0);
        } else {
            if _cse_temp_0 {
                return (0, 0, 255);
            } else {
                return (0, 0, 0);
            }
        }
    }
}
#[doc = "Convert status enum to message"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn status_to_message(status: i32) -> String {
    let _cse_temp_0 = status == Status::PENDING;
    if _cse_temp_0 {
        return "Waiting for approval".to_string();
    } else {
        if _cse_temp_0 {
            return "Request approved".to_string();
        } else {
            if _cse_temp_0 {
                return "Request rejected".to_string();
            } else {
                return "Unknown status".to_string();
            }
        }
    }
}
#[doc = "Process value based on status"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn process_by_status(status: i32, value: i32) -> i32 {
    let _cse_temp_0 = status == Status::APPROVED;
    if _cse_temp_0 {
        return value * 2;
    } else {
        if _cse_temp_0 {
            return 0;
        } else {
            return value;
        }
    }
}
#[doc = "Test enum as flags(bit operations)"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_enum_flags() -> bool {
    let READ: i32 = 1;
    let WRITE: i32 = 2;
    let EXECUTE: i32 = 4;
    let _cse_temp_0 = READ | WRITE;
    let permissions: i32 = _cse_temp_0;
    let _cse_temp_1 = permissions & READ;
    let _cse_temp_2 = _cse_temp_1 != 0;
    let has_read: bool = _cse_temp_2;
    let _cse_temp_3 = permissions & EXECUTE;
    let _cse_temp_4 = _cse_temp_3 != 0;
    let has_execute: bool = _cse_temp_4;
    (has_read) && (!has_execute)
}
#[doc = "Test enum value ranges"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_enum_range() -> Vec<i32> {
    let mut directions: Vec<i32> = vec![];
    for i in 0..(4) {
        directions.push(i);
    }
    directions
}
#[doc = "Validate if value is in enum range"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn validate_enum_value(value: i32, min_val: i32, max_val: i32) -> bool {
    let _cse_temp_0 = value >= min_val;
    let _cse_temp_1 = value <= max_val;
    let _cse_temp_2 = (_cse_temp_0) && (_cse_temp_1);
    let is_valid: bool = _cse_temp_2;
    is_valid
}
#[doc = "Run all enum module tests"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_all_enum_features() {
    let color: i32 = test_enum_basic_access();
    let is_different: bool = test_enum_comparison();
    let name: String = test_enum_to_name();
    let value: i32 = test_enum_to_value();
    let from_value: i32 = test_enum_from_value(2);
    let status: i32 = test_status_enum();
    let msg: String = status_to_message(status);
    let direction: i32 = test_direction_enum();
    let rotated: i32 = rotate_direction(Direction::NORTH, true);
    let opposite: i32 = opposite_direction(Direction::NORTH);
    let is_horiz: bool = is_horizontal(Direction::EAST);
    let is_vert: bool = is_vertical(Direction::NORTH);
    let colors: Vec<i32> = test_enum_iteration();
    let count: i32 = test_enum_count();
    let rgb: (i32, i32, i32) = color_to_rgb(Color::RED);
    let processed: i32 = process_by_status(Status::APPROVED, 10);
    let has_perms: bool = test_enum_flags();
    let dir_range: Vec<i32> = test_enum_range();
    let is_valid: bool = validate_enum_value(2, 0, 3);
    println!("{}", "All enum module tests completed successfully");
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_enum_basic_access_examples() {
        let _ = test_enum_basic_access();
    }
    #[test]
    fn test_test_enum_to_value_examples() {
        let _ = test_enum_to_value();
    }
    #[test]
    fn test_test_enum_from_value_examples() {
        assert_eq!(test_enum_from_value(0), 0);
        assert_eq!(test_enum_from_value(1), 1);
        assert_eq!(test_enum_from_value(-1), -1);
    }
    #[test]
    fn test_test_status_enum_examples() {
        let _ = test_status_enum();
    }
    #[test]
    fn test_test_direction_enum_examples() {
        let _ = test_direction_enum();
    }
    #[test]
    fn test_opposite_direction_examples() {
        assert_eq!(opposite_direction(0), 0);
        assert_eq!(opposite_direction(1), 1);
        assert_eq!(opposite_direction(-1), -1);
    }
    #[test]
    fn test_is_horizontal_examples() {
        let _ = is_horizontal(Default::default());
    }
    #[test]
    fn test_is_vertical_examples() {
        let _ = is_vertical(Default::default());
    }
    #[test]
    fn test_test_enum_count_examples() {
        let _ = test_enum_count();
    }
    #[test]
    fn test_process_by_status_examples() {
        assert_eq!(process_by_status(0, 0), 0);
        assert_eq!(process_by_status(1, 2), 3);
        assert_eq!(process_by_status(-1, 1), 0);
    }
}
