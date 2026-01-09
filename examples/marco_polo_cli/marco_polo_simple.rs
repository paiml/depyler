#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
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
#[doc = "Generate a number in range(simplified without random)."]
#[doc = " Depyler: proven to terminate"]
pub fn generate_number(min_val: i32, max_val: i32) -> Result<i32, Box<dyn std::error::Error>> {
    Ok({
        let a = min_val + max_val;
        let b = 2;
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
    })
}
#[doc = "Provide a hint based on the guess."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn get_hint(guess: i32, target: i32) -> String {
    let _cse_temp_0 = guess < target;
    if _cse_temp_0 {
        return "Marco!(Too low)".to_string();
    } else {
        let _cse_temp_1 = guess > target;
        if _cse_temp_1 {
            return "Marco!(Too high)".to_string();
        } else {
            return "Polo!".to_string();
        }
    }
}
#[doc = "Calculate final score."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn calculate_score(attempts: i32, rounds: i32) -> i32 {
    let _cse_temp_0 = rounds == 0;
    if _cse_temp_0 {
        return 0;
    }
    let _cse_temp_1 = 100 * rounds;
    let base_score = _cse_temp_1;
    let _cse_temp_2 = attempts * 5;
    let penalty = _cse_temp_2;
    let score = base_score - penalty;
    let _cse_temp_3 = score < 0;
    if _cse_temp_3 {
        return 0;
    }
    score
}
#[doc = "Get difficulty name from level."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn get_difficulty_name(level: i32) -> String {
    let _cse_temp_0 = level == 1;
    if _cse_temp_0 {
        return "Easy".to_string();
    } else {
        let _cse_temp_1 = level == 2;
        if _cse_temp_1 {
            return "Medium".to_string();
        } else {
            let _cse_temp_2 = level == 3;
            if _cse_temp_2 {
                return "Hard".to_string();
            } else {
                return "Unknown".to_string();
            }
        }
    }
}
#[doc = "Calculate average with safety check."]
#[doc = " Depyler: proven to terminate"]
pub fn calculate_average(total: f64, count: f64) -> Result<f64, Box<dyn std::error::Error>> {
    let _cse_temp_0 = count == 0f64;
    if _cse_temp_0 {
        return Ok(0.0);
    }
    Ok(((total) as f64) / ((count) as f64))
}
#[doc = "Format game statistics as string."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn format_statistics(
    score: i32,
    attempts: f64,
    rounds: f64,
) -> Result<String, Box<dyn std::error::Error>> {
    let avg = calculate_average(attempts, rounds)?;
    let mut result = "Game Statistics:\n".to_string();
    let _cse_temp_0 = (score).to_string();
    let _cse_temp_1 = format!("{}{}", format!("{}{}", result, "Score: "), _cse_temp_0);
    result = format!("{}{}", _cse_temp_1, "\n");
    let _cse_temp_2 = (attempts).to_string();
    let _cse_temp_3 = format!("{}{}", format!("{}{}", result, "Attempts: "), _cse_temp_2);
    result = format!("{}{}", _cse_temp_3, "\n");
    let _cse_temp_4 = (avg).to_string();
    let _cse_temp_5 = format!("{}{}", format!("{}{}", result, "Average: "), _cse_temp_4);
    result = format!("{}{}", _cse_temp_5, "\n");
    Ok(result.to_string())
}
#[doc = "Check if guess is in valid range."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn validate_guess(guess: i32, min_val: i32, max_val: i32) -> bool {
    let _cse_temp_0 = guess < min_val;
    if _cse_temp_0 {
        return false;
    }
    let _cse_temp_1 = guess > max_val;
    if _cse_temp_1 {
        return false;
    }
    true
}
#[doc = "Simulate a round with fixed guesses."]
#[doc = " Depyler: verified panic-free"]
pub fn play_simple_round(target: i32, max_attempts: i32) -> i32 {
    let mut attempts: i32 = Default::default();
    attempts = 0;
    let mut guess = 50;
    while attempts < max_attempts {
        attempts = attempts + 1;
        if guess == target {
            return attempts;
        } else {
            if guess < target {
                guess = guess + 10;
            } else {
                guess = guess - 5;
            }
        }
    }
    attempts
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_generate_number_examples() {
        assert_eq!(generate_number(0, 0), 0);
        assert_eq!(generate_number(1, 2), 3);
        assert_eq!(generate_number(-1, 1), 0);
    }
    #[test]
    fn test_calculate_score_examples() {
        assert_eq!(calculate_score(0, 0), 0);
        assert_eq!(calculate_score(1, 2), 3);
        assert_eq!(calculate_score(-1, 1), 0);
    }
    #[test]
    fn test_play_simple_round_examples() {
        assert_eq!(play_simple_round(0, 0), 0);
        assert_eq!(play_simple_round(1, 2), 3);
        assert_eq!(play_simple_round(-1, 1), 0);
    }
}
