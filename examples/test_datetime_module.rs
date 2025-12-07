use chrono::DateTime;
use chrono::Duration;
use chrono::NaiveDate;
use chrono::NaiveTime;
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
#[doc = "Test getting current date and time"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_current_datetime() -> String {
    let now: chrono::NaiveDateTime = chrono::Local::now().naive_local();
    let year: i32 = now.year() as i32;
    let month: i32 = now.month() as i32;
    let day: i32 = now.day() as i32;
    let hour: i32 = now.hour() as i32;
    let minute: i32 = now.minute() as i32;
    let second: i32 = now.second() as i32;
    let result: String = format!(
        "{:?}-{:?}-{:?} {:?}:{:?}:{:?}",
        year, month, day, hour, minute, second
    );
    result.to_string()
}
#[doc = "Test creating specific dates"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_date_creation() -> chrono::NaiveDate {
    let birthday: chrono::NaiveDate =
        chrono::NaiveDate::from_ymd_opt(1990 as i32, 5 as u32, 15 as u32).unwrap();
    birthday
}
#[doc = "Test creating specific times"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_time_creation() -> chrono::NaiveTime {
    let meeting_time: chrono::NaiveTime =
        chrono::NaiveTime::from_hms_opt(14 as u32, 30 as u32, 0 as u32).unwrap();
    meeting_time
}
#[doc = "Test creating specific datetime objects"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_datetime_creation() -> chrono::NaiveDateTime {
    let event: chrono::NaiveDateTime =
        chrono::NaiveDate::from_ymd_opt(2025 as i32, 12 as u32, 31 as u32)
            .unwrap()
            .and_hms_opt(23 as u32, 59 as u32, 59 as u32)
            .unwrap();
    event
}
#[doc = "Test date arithmetic operations"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_date_arithmetic() -> i32 {
    let start_date: chrono::NaiveDate =
        chrono::NaiveDate::from_ymd_opt(2025 as i32, 1 as u32, 1 as u32).unwrap();
    let end_date: chrono::NaiveDate =
        chrono::NaiveDate::from_ymd_opt(2025 as i32, 12 as u32, 31 as u32).unwrap();
    let difference: chrono::Duration = end_date - start_date;
    let num_days: i32 = difference.days;
    num_days
}
#[doc = "Test creating and using timedelta objects"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_timedelta_creation() -> chrono::Duration {
    let one_week: chrono::Duration = chrono::Duration::zero();
    let duration: chrono::Duration = chrono::Duration::zero();
    one_week + duration
}
#[doc = "Test adding timedelta to dates"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_date_addition() -> chrono::NaiveDate {
    let today: chrono::NaiveDate =
        chrono::NaiveDate::from_ymd_opt(2025 as i32, 11 as u32, 5 as u32).unwrap();
    let one_week: chrono::Duration = chrono::Duration::zero();
    let next_week: chrono::NaiveDate = today + one_week;
    next_week
}
#[doc = "Test subtracting timedelta from dates"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_date_subtraction() -> chrono::NaiveDate {
    let today: chrono::NaiveDate =
        chrono::NaiveDate::from_ymd_opt(2025 as i32, 11 as u32, 5 as u32).unwrap();
    let one_month: chrono::Duration = chrono::Duration::zero();
    let last_month: chrono::NaiveDate = today - one_month;
    last_month
}
#[doc = "Calculate age in years given birth date"]
#[doc = " Depyler: proven to terminate"]
pub fn calculate_age<'b, 'a>(
    birth_date: &'a chrono::NaiveDate,
    current_date: &'b chrono::NaiveDate,
) -> Result<i32, Box<dyn std::error::Error>> {
    let diff: chrono::Duration = current_date - birth_date;
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
    event_date: &'a chrono::NaiveDate,
    current_date: &'b chrono::NaiveDate,
) -> i32 {
    let _cse_temp_0 = event_date < current_date;
    if _cse_temp_0 {
        return 0;
    }
    let diff: chrono::Duration = event_date - current_date;
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
        base.get(actual_idx).cloned().unwrap_or_default()
    })
}
#[doc = "Test accessing timedelta components"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_timedelta_components() -> String {
    let duration: chrono::Duration = chrono::Duration::zero();
    let days: i32 = duration.days;
    let seconds: i32 = duration.seconds;
    let _cse_temp_0 = (duration.total_seconds()) as i32;
    let total_seconds: i32 = _cse_temp_0;
    let result: String = format!(
        "Days: {:?}, Seconds: {:?}, Total: {}",
        days, seconds, total_seconds
    );
    result.to_string()
}
#[doc = "Test comparing datetime objects"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_datetime_comparison() -> bool {
    let dt1: chrono::NaiveDateTime =
        chrono::NaiveDate::from_ymd_opt(2025 as i32, 1 as u32, 1 as u32)
            .unwrap()
            .and_hms_opt(0 as u32, 0 as u32, 0 as u32)
            .unwrap();
    let dt2: chrono::NaiveDateTime =
        chrono::NaiveDate::from_ymd_opt(2025 as i32, 12 as u32, 31 as u32)
            .unwrap()
            .and_hms_opt(23 as u32, 59 as u32, 59 as u32)
            .unwrap();
    let _cse_temp_0 = dt1 < dt2;
    let is_before: bool = _cse_temp_0;
    is_before
}
#[doc = "Test comparing date objects"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_date_comparison() -> bool {
    let d1: chrono::NaiveDate =
        chrono::NaiveDate::from_ymd_opt(2025 as i32, 1 as u32, 1 as u32).unwrap();
    let d2: chrono::NaiveDate =
        chrono::NaiveDate::from_ymd_opt(2025 as i32, 12 as u32, 31 as u32).unwrap();
    let _cse_temp_0 = d1 < d2;
    let is_before: bool = _cse_temp_0;
    is_before
}
#[doc = "Calculate working days between two dates(excluding weekends)"]
#[doc = " Depyler: proven to terminate"]
pub fn working_days_between<'b, 'a>(
    start: &'a chrono::NaiveDate,
    end: &'b chrono::NaiveDate,
) -> Result<i32, Box<dyn std::error::Error>> {
    let _cse_temp_0 = start >= end;
    if _cse_temp_0 {
        return Ok(0);
    }
    let diff: chrono::Duration = end - start;
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
    start_date: &chrono::NaiveDate,
    num_days: i32,
) -> Result<chrono::NaiveDate, Box<dyn std::error::Error>> {
    let _cse_temp_0 = start_date + chrono::Duration::zero();
    let result: chrono::NaiveDate = _cse_temp_0;
    Ok(result)
}
#[doc = "Test datetime string formatting"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_datetime_formatting() -> String {
    let dt: chrono::NaiveDateTime =
        chrono::NaiveDate::from_ymd_opt(2025 as i32, 11 as u32, 5 as u32)
            .unwrap()
            .and_hms_opt(14 as u32, 30 as u32, 0 as u32)
            .unwrap();
    let year: i32 = dt.year() as i32;
    let month: i32 = dt.month() as i32;
    let day: i32 = dt.day() as i32;
    let hour: i32 = dt.hour() as i32;
    let minute: i32 = dt.minute() as i32;
    let formatted: String = format!("{}-{}-{:?} {:?}:{:?}", year, month, day, hour, minute);
    formatted.to_string()
}
#[doc = "Generate list of dates in range"]
#[doc = " Depyler: verified panic-free"]
pub fn test_date_range<'a, 'b>(
    start: &'a chrono::NaiveDate,
    end: &'b chrono::NaiveDate,
) -> Vec<chrono::NaiveDate> {
    let mut dates: Vec<chrono::NaiveDate> = vec![];
    let mut current: chrono::NaiveDate = start.clone();
    let one_day: chrono::Duration = chrono::Duration::zero();
    while current <= end {
        dates.push(current);
        current = current + one_day;
    }
    dates
}
#[doc = "Test time calculations using timedelta"]
#[doc = " Depyler: proven to terminate"]
pub fn test_time_arithmetic() -> Result<i32, Box<dyn std::error::Error>> {
    let meeting_start: chrono::NaiveTime =
        chrono::NaiveTime::from_hms_opt(9 as u32, 0 as u32, 0 as u32).unwrap();
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
pub fn quarter_of_year(d: &chrono::NaiveDate) -> i32 {
    let month: i32 = d.month() as i32;
    let _cse_temp_0 = month <= 3;
    if _cse_temp_0 {
        1
    } else {
        let _cse_temp_1 = month <= 6;
        if _cse_temp_1 {
            2
        } else {
            let _cse_temp_2 = month <= 9;
            if _cse_temp_2 {
                3
            } else {
                4
            }
        }
    }
}
#[doc = "Run all datetime module tests"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_all_datetime_features() -> Result<(), Box<dyn std::error::Error>> {
    let _current_str: String = test_current_datetime();
    let _my_date: chrono::NaiveDate = test_date_creation();
    let _my_time: chrono::NaiveTime = test_time_creation();
    let _my_datetime: chrono::NaiveDateTime = test_datetime_creation();
    let _days_diff: i32 = test_date_arithmetic();
    let _delta: chrono::Duration = test_timedelta_creation();
    let _future_date: chrono::NaiveDate = test_date_addition();
    let _past_date: chrono::NaiveDate = test_date_subtraction();
    let birth: chrono::NaiveDate =
        chrono::NaiveDate::from_ymd_opt(1990 as i32, 5 as u32, 15 as u32).unwrap();
    let today: chrono::NaiveDate =
        chrono::NaiveDate::from_ymd_opt(2025 as i32, 11 as u32, 5 as u32).unwrap();
    let _age: i32 = calculate_age(birth, today)?;
    let event: chrono::NaiveDate =
        chrono::NaiveDate::from_ymd_opt(2025 as i32, 12 as u32, 31 as u32).unwrap();
    let _days_left: i32 = days_until_event(event, today);
    let _is_leap_2024: bool = is_leap_year(2024)?;
    let _is_leap_2025: bool = is_leap_year(2025)?;
    let _days_feb_2024: i32 = days_in_month(2024, 2)?;
    let _days_feb_2025: i32 = days_in_month(2025, 2)?;
    let _delta_str: String = test_timedelta_components();
    let _cmp_dt: bool = test_datetime_comparison();
    let _cmp_date: bool = test_date_comparison();
    let start: chrono::NaiveDate =
        chrono::NaiveDate::from_ymd_opt(2025 as i32, 1 as u32, 1 as u32).unwrap();
    let end: chrono::NaiveDate =
        chrono::NaiveDate::from_ymd_opt(2025 as i32, 12 as u32, 31 as u32).unwrap();
    let _work_days: i32 = working_days_between(start, end)?;
    let _future: chrono::NaiveDate = add_business_days(today, 10)?;
    let _formatted: String = test_datetime_formatting();
    let range_start: chrono::NaiveDate =
        chrono::NaiveDate::from_ymd_opt(2025 as i32, 11 as u32, 1 as u32).unwrap();
    let range_end: chrono::NaiveDate =
        chrono::NaiveDate::from_ymd_opt(2025 as i32, 11 as u32, 7 as u32).unwrap();
    let _date_list: Vec<chrono::NaiveDate> = test_date_range(range_start, range_end);
    let _meeting_end: i32 = test_time_arithmetic()?;
    let _q: i32 = quarter_of_year(today);
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
