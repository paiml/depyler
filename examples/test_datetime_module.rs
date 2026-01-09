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
#[derive(Debug, Clone)]
pub struct IndexError {
    message: String,
}
impl std::fmt::Display for IndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "index out of range: {}", self.message)
    }
}
impl std::error::Error for IndexError {}
impl IndexError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
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
#[doc = "Test getting current date and time"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_current_datetime() -> String {
    let now: std::time::SystemTime = std::time::SystemTime::now();
    let year: i32 = now.year() as i32;
    let month: i32 = now.month() as i32;
    let day: i32 = now.day() as i32;
    let hour: i32 = now.hour() as i32;
    let minute: i32 = now.minute() as i32;
    let second: i32 = now.second() as i32;
    let result: String = format!("{}-{}-{} {}:{}:{}", year, month, day, hour, minute, second);
    result.to_string()
}
#[doc = "Test creating specific dates"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_date_creation() -> (u32, u32, u32) {
    let birthday: (u32, u32, u32) = (1990 as u32, 5 as u32, 15 as u32);
    birthday
}
#[doc = "Test creating specific times"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_time_creation() -> (u32, u32, u32) {
    let meeting_time: (u32, u32, u32) = (14 as u32, 30 as u32, 0 as u32);
    meeting_time
}
#[doc = "Test creating specific datetime objects"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_datetime_creation() -> std::time::SystemTime {
    let event: std::time::SystemTime = std::time::SystemTime::now();
    event
}
#[doc = "Test date arithmetic operations"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_date_arithmetic() -> i32 {
    let start_date: (u32, u32, u32) = (2025 as u32, 1 as u32, 1 as u32);
    let end_date: (u32, u32, u32) = (2025 as u32, 12 as u32, 31 as u32);
    let difference: std::time::Duration = end_date - start_date;
    let num_days: i32 = difference.days;
    num_days
}
#[doc = "Test creating and using timedelta objects"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_timedelta_creation() -> std::time::Duration {
    let one_week: std::time::Duration = std::time::Duration::from_secs(0);
    let duration: std::time::Duration = std::time::Duration::from_secs(0);
    one_week + duration
}
#[doc = "Test adding timedelta to dates"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_date_addition() -> (u32, u32, u32) {
    let today: (u32, u32, u32) = (2025 as u32, 11 as u32, 5 as u32);
    let one_week: std::time::Duration = std::time::Duration::from_secs(0);
    let next_week: (u32, u32, u32) = today + one_week;
    next_week
}
#[doc = "Test subtracting timedelta from dates"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_date_subtraction() -> (u32, u32, u32) {
    let today: (u32, u32, u32) = (2025 as u32, 11 as u32, 5 as u32);
    let one_month: std::time::Duration = std::time::Duration::from_secs(0);
    let last_month: (u32, u32, u32) = today - one_month;
    last_month
}
#[doc = "Calculate age in years given birth date"]
#[doc = " Depyler: proven to terminate"]
pub fn calculate_age<'a, 'b>(
    birth_date: &'a (u32, u32, u32),
    current_date: &'b (u32, u32, u32),
) -> Result<i32, Box<dyn std::error::Error>> {
    let diff: std::time::Duration = *current_date - *birth_date;
    let _cse_temp_0 = {
        let a = diff.days;
        let b = 365;
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
    let age_in_years: i32 = _cse_temp_0;
    Ok(age_in_years)
}
#[doc = "Calculate days until a future event"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn days_until_event<'a, 'b>(
    event_date: &'a (u32, u32, u32),
    current_date: &'b (u32, u32, u32),
) -> i32 {
    let _cse_temp_0 = event_date < current_date;
    if _cse_temp_0 {
        return 0;
    }
    let diff: std::time::Duration = *event_date - *current_date;
    diff.days
}
#[doc = "Check if a year is a leap year"]
#[doc = " Depyler: proven to terminate"]
pub fn is_leap_year(year: i32) -> Result<bool, Box<dyn std::error::Error>> {
    let _cse_temp_0 = year % 400;
    let _cse_temp_1 = _cse_temp_0 == 0;
    if _cse_temp_1 {
        return Ok(true);
    }
    let _cse_temp_2 = year % 100;
    let _cse_temp_3 = _cse_temp_2 == 0;
    if _cse_temp_3 {
        return Ok(false);
    }
    let _cse_temp_4 = year % 4;
    let _cse_temp_5 = _cse_temp_4 == 0;
    if _cse_temp_5 {
        return Ok(true);
    }
    Ok(false)
}
#[doc = "Get number of days in a specific month"]
#[doc = " Depyler: proven to terminate"]
pub fn days_in_month(year: i32, month: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let days: Vec<i32> = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let _cse_temp_0 = month < 1;
    let _cse_temp_1 = month > 12;
    let _cse_temp_2 = (_cse_temp_0) || (_cse_temp_1);
    if _cse_temp_2 {
        return Ok(0);
    }
    let _cse_temp_3 = month == 2;
    let _cse_temp_4 = (_cse_temp_3) && (is_leap_year(year)?);
    if _cse_temp_4 {
        return Ok(29);
    }
    Ok({
        let base = &days;
        let idx: i32 = month - 1;
        let actual_idx = if idx < 0 {
            base.len().saturating_sub(idx.abs() as usize)
        } else {
            idx as usize
        };
        base.get(actual_idx)
            .cloned()
            .expect("IndexError: list index out of range")
    })
}
#[doc = "Test accessing timedelta components"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_timedelta_components() -> String {
    let duration: std::time::Duration = std::time::Duration::from_secs(0);
    let days: i32 = duration.days;
    let seconds: i32 = duration.seconds;
    let _cse_temp_0 = (duration.as_secs_f64()) as i32;
    let total_seconds: i32 = _cse_temp_0;
    let result: String = format!(
        "Days: {}, Seconds: {}, Total: {}",
        days, seconds, total_seconds
    );
    result.to_string()
}
#[doc = "Test comparing datetime objects"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_datetime_comparison() -> bool {
    let dt1: std::time::SystemTime = std::time::SystemTime::now();
    let dt2: std::time::SystemTime = std::time::SystemTime::now();
    let _cse_temp_0 = dt1 < dt2;
    let is_before: bool = _cse_temp_0;
    is_before
}
#[doc = "Test comparing date objects"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_date_comparison() -> bool {
    let d1: (u32, u32, u32) = (2025 as u32, 1 as u32, 1 as u32);
    let d2: (u32, u32, u32) = (2025 as u32, 12 as u32, 31 as u32);
    let _cse_temp_0 = d1 < d2;
    let is_before: bool = _cse_temp_0;
    is_before
}
#[doc = "Calculate working days between two dates(excluding weekends)"]
#[doc = " Depyler: proven to terminate"]
pub fn working_days_between<'b, 'a>(
    start: &'a (u32, u32, u32),
    end: &'b (u32, u32, u32),
) -> Result<i32, Box<dyn std::error::Error>> {
    let _cse_temp_0 = start >= end;
    if _cse_temp_0 {
        return Ok(0);
    }
    let diff: std::time::Duration = *end - *start;
    let total_days: i32 = diff.days;
    let _cse_temp_1 = total_days * 5;
    let _cse_temp_2 = {
        let a = _cse_temp_1;
        let b = 7;
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
    let working_days: i32 = _cse_temp_2;
    Ok(working_days)
}
#[doc = "Add business days to a date(simplified)"]
#[doc = " Depyler: proven to terminate"]
pub fn add_business_days(
    start_date: &(u32, u32, u32),
    num_days: i32,
) -> Result<(u32, u32, u32), Box<dyn std::error::Error>> {
    let _cse_temp_0 = num_days * 7;
    let _cse_temp_1 = {
        let a = _cse_temp_0;
        let b = 5;
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
    let calendar_days: i32 = _cse_temp_1;
    let _cse_temp_2 = start_date + std::time::Duration::from_secs(0);
    let result: (u32, u32, u32) = _cse_temp_2;
    Ok(result)
}
#[doc = "Test datetime string formatting"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_datetime_formatting() -> String {
    let dt: std::time::SystemTime = std::time::SystemTime::now();
    let year: i32 = dt.year() as i32;
    let month: i32 = dt.month() as i32;
    let day: i32 = dt.day() as i32;
    let hour: i32 = dt.hour() as i32;
    let minute: i32 = dt.minute() as i32;
    let formatted: String = format!("{}-{}-{} {}:{}", year, month, day, hour, minute);
    formatted.to_string()
}
#[doc = "Generate list of dates in range"]
#[doc = " Depyler: verified panic-free"]
pub fn test_date_range<'a, 'b>(
    start: &'a (u32, u32, u32),
    end: &'b (u32, u32, u32),
) -> Vec<(u32, u32, u32)> {
    let mut dates: Vec<(u32, u32, u32)> = vec![];
    let mut current: (u32, u32, u32) = start.clone();
    let one_day: std::time::Duration = std::time::Duration::from_secs(0);
    while current <= end {
        dates.push(current);
        current = current + one_day;
    }
    dates
}
#[doc = "Test time calculations using timedelta"]
#[doc = " Depyler: proven to terminate"]
pub fn test_time_arithmetic() -> Result<i32, Box<dyn std::error::Error>> {
    let meeting_start: (u32, u32, u32) = (9 as u32, 0 as u32, 0 as u32);
    let _cse_temp_0 = meeting_start.hour() as i32 * 60;
    let _cse_temp_1 = _cse_temp_0 + meeting_start.minute() as i32;
    let start_minutes: i32 = _cse_temp_1;
    let duration_minutes: i32 = 150;
    let end_minutes: i32 = start_minutes + duration_minutes;
    let _cse_temp_2 = {
        let a = end_minutes;
        let b = 60;
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
    let end_hour: i32 = _cse_temp_2;
    Ok(end_hour)
}
#[doc = "Get the quarter(1-4) for a given date"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn quarter_of_year(d: &(u32, u32, u32)) -> i32 {
    let month: i32 = d.month() as i32;
    let _cse_temp_0 = month <= 3;
    if _cse_temp_0 {
        return 1;
    } else {
        let _cse_temp_1 = month <= 6;
        if _cse_temp_1 {
            return 2;
        } else {
            let _cse_temp_2 = month <= 9;
            if _cse_temp_2 {
                return 3;
            } else {
                return 4;
            }
        }
    }
}
#[doc = "Run all datetime module tests"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_all_datetime_features() -> Result<(), Box<dyn std::error::Error>> {
    let current_str: String = test_current_datetime();
    let my_date: (u32, u32, u32) = test_date_creation();
    let my_time: (u32, u32, u32) = test_time_creation();
    let my_datetime: std::time::SystemTime = test_datetime_creation();
    let days_diff: i32 = test_date_arithmetic();
    let delta: std::time::Duration = test_timedelta_creation();
    let future_date: (u32, u32, u32) = test_date_addition();
    let past_date: (u32, u32, u32) = test_date_subtraction();
    let birth: (u32, u32, u32) = (1990 as u32, 5 as u32, 15 as u32);
    let today: (u32, u32, u32) = (2025 as u32, 11 as u32, 5 as u32);
    let age: i32 = calculate_age(&birth, &today)?;
    let event: (u32, u32, u32) = (2025 as u32, 12 as u32, 31 as u32);
    let days_left: i32 = days_until_event(&event, &today);
    let is_leap_2024: bool = is_leap_year(2024)?;
    let is_leap_2025: bool = is_leap_year(2025)?;
    let days_feb_2024: i32 = days_in_month(2024, 2)?;
    let days_feb_2025: i32 = days_in_month(2025, 2)?;
    let delta_str: String = test_timedelta_components();
    let cmp_dt: bool = test_datetime_comparison();
    let cmp_date: bool = test_date_comparison();
    let start: (u32, u32, u32) = (2025 as u32, 1 as u32, 1 as u32);
    let end: (u32, u32, u32) = (2025 as u32, 12 as u32, 31 as u32);
    let work_days: i32 = working_days_between(&start, &end)?;
    let future: (u32, u32, u32) = add_business_days(&today, 10)?;
    let formatted: String = test_datetime_formatting();
    let range_start: (u32, u32, u32) = (2025 as u32, 11 as u32, 1 as u32);
    let range_end: (u32, u32, u32) = (2025 as u32, 11 as u32, 7 as u32);
    let date_list: Vec<(u32, u32, u32)> = test_date_range(&range_start, &range_end);
    let meeting_end: i32 = test_time_arithmetic()?;
    let q: i32 = quarter_of_year(&today);
    println!("{}", "All datetime module tests completed successfully");
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    #[test]
    fn test_test_date_arithmetic_examples() {
        let _ = test_date_arithmetic();
    }
    #[test]
    fn test_is_leap_year_examples() {
        let _ = is_leap_year(Default::default());
    }
    #[test]
    fn test_days_in_month_examples() {
        assert_eq!(days_in_month(0, 0), 0);
        assert_eq!(days_in_month(1, 2), 3);
        assert_eq!(days_in_month(-1, 1), 0);
    }
    #[test]
    fn test_test_time_arithmetic_examples() {
        let _ = test_time_arithmetic();
    }
}
