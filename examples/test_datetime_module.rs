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
    let now: datetime = chrono::Local::now().naive_local();
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
    result
}
#[doc = "Test creating specific dates"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_date_creation() -> date {
    let birthday: date = chrono::NaiveDate::from_ymd_opt(1990 as i32, 5 as u32, 15 as u32).unwrap();
    birthday
}
#[doc = "Test creating specific times"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_time_creation() -> time {
    let meeting_time: time =
        chrono::NaiveTime::from_hms_opt(14 as u32, 30 as u32, 0 as u32).unwrap();
    meeting_time
}
#[doc = "Test creating specific datetime objects"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_datetime_creation() -> datetime {
    let event: datetime = chrono::NaiveDate::from_ymd_opt(2025 as i32, 12 as u32, 31 as u32)
        .unwrap()
        .and_hms_opt(23 as u32, 59 as u32, 59 as u32)
        .unwrap();
    event
}
#[doc = "Test date arithmetic operations"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_date_arithmetic() -> i32 {
    let start_date: date =
        chrono::NaiveDate::from_ymd_opt(2025 as i32, 1 as u32, 1 as u32).unwrap();
    let end_date: date =
        chrono::NaiveDate::from_ymd_opt(2025 as i32, 12 as u32, 31 as u32).unwrap();
    let difference: timedelta = end_date - start_date;
    let num_days: i32 = difference.num_days() as i32;
    num_days
}
#[doc = "Test creating and using timedelta objects"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_timedelta_creation() -> timedelta {
    let one_week: timedelta = chrono::Duration::zero();
    let duration: timedelta = chrono::Duration::zero();
    one_week + duration
}
#[doc = "Test adding timedelta to dates"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_date_addition() -> date {
    let today: date = chrono::NaiveDate::from_ymd_opt(2025 as i32, 11 as u32, 5 as u32).unwrap();
    let one_week: timedelta = chrono::Duration::zero();
    let next_week: date = today + one_week;
    next_week
}
#[doc = "Test subtracting timedelta from dates"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_date_subtraction() -> date {
    let today: date = chrono::NaiveDate::from_ymd_opt(2025 as i32, 11 as u32, 5 as u32).unwrap();
    let one_month: timedelta = chrono::Duration::zero();
    let last_month: date = today - one_month;
    last_month
}
#[doc = "Calculate age in years given birth date"]
#[doc = " Depyler: proven to terminate"]
pub fn calculate_age<'b, 'a>(
    birth_date: &'a date,
    current_date: &'b date,
) -> Result<i32, ZeroDivisionError> {
    let diff: timedelta = current_date - birth_date;
    let _cse_temp_0 = {
        let a = diff.num_days() as i32;
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
pub fn days_until_event<'a, 'b>(event_date: &'a date, current_date: &'b date) -> i32 {
    let _cse_temp_0 = event_date < current_date;
    if _cse_temp_0 {
        return 0;
    }
    let diff: timedelta = event_date - current_date;
    diff.num_days() as i32
}
#[doc = "Check if a year is a leap year"]
#[doc = " Depyler: proven to terminate"]
pub fn is_leap_year(year: i32) -> Result<bool, ZeroDivisionError> {
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
pub fn days_in_month(year: i32, month: i32) -> Result<i32, IndexError> {
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
    let duration: timedelta = chrono::Duration::zero();
    let days: i32 = duration.num_days() as i32;
    let seconds: i32 = (duration.num_seconds() % 86400) as i32;
    let _cse_temp_0 = (duration.total_seconds()) as i32;
    let total_seconds: i32 = _cse_temp_0;
    let result: String = format!(
        "Days: {:?}, Seconds: {:?}, Total: {:?}",
        days, seconds, total_seconds
    );
    result
}
#[doc = "Test comparing datetime objects"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_datetime_comparison() -> bool {
    let dt1: datetime = chrono::NaiveDate::from_ymd_opt(2025 as i32, 1 as u32, 1 as u32)
        .unwrap()
        .and_hms_opt(0 as u32, 0 as u32, 0 as u32)
        .unwrap();
    let dt2: datetime = chrono::NaiveDate::from_ymd_opt(2025 as i32, 12 as u32, 31 as u32)
        .unwrap()
        .and_hms_opt(23 as u32, 59 as u32, 59 as u32)
        .unwrap();
    let _cse_temp_0 = dt1 < dt2;
    let is_before: bool = _cse_temp_0;
    let _cse_temp_1 = dt1 > dt2;
    let is_after: bool = _cse_temp_1;
    let _cse_temp_2 = dt1 == dt2;
    let is_equal: bool = _cse_temp_2;
    is_before
}
#[doc = "Test comparing date objects"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_date_comparison() -> bool {
    let d1: date = chrono::NaiveDate::from_ymd_opt(2025 as i32, 1 as u32, 1 as u32).unwrap();
    let d2: date = chrono::NaiveDate::from_ymd_opt(2025 as i32, 12 as u32, 31 as u32).unwrap();
    let _cse_temp_0 = d1 < d2;
    let is_before: bool = _cse_temp_0;
    is_before
}
#[doc = "Calculate working days between two dates(excluding weekends)"]
#[doc = " Depyler: proven to terminate"]
pub fn working_days_between<'a, 'b>(
    start: &'a date,
    end: &'b date,
) -> Result<i32, ZeroDivisionError> {
    let _cse_temp_0 = start >= end;
    if _cse_temp_0 {
        return Ok(0);
    }
    let count: i32 = 0;
    let current: date = start;
    let diff: timedelta = end - start;
    let total_days: i32 = diff.num_days() as i32;
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
pub fn add_business_days(start_date: &date, num_days: i32) -> Result<date, ZeroDivisionError> {
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
    let _cse_temp_2 = start_date + chrono::Duration::zero();
    let result: date = _cse_temp_2;
    Ok(result)
}
#[doc = "Test datetime string formatting"]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_datetime_formatting() -> String {
    let dt: datetime = chrono::NaiveDate::from_ymd_opt(2025 as i32, 11 as u32, 5 as u32)
        .unwrap()
        .and_hms_opt(14 as u32, 30 as u32, 0 as u32)
        .unwrap();
    let year: i32 = dt.year() as i32;
    let month: i32 = dt.month() as i32;
    let day: i32 = dt.day() as i32;
    let hour: i32 = dt.hour() as i32;
    let minute: i32 = dt.minute() as i32;
    let formatted: String = format!("{:?}-{:?}-{:?} {:?}:{:?}", year, month, day, hour, minute);
    formatted
}
#[doc = "Generate list of dates in range"]
#[doc = " Depyler: verified panic-free"]
pub fn test_date_range<'b, 'a>(start: &'a date, end: &'b date) -> Vec<date> {
    let mut dates: Vec<date> = vec![];
    let mut current: date = start;
    let one_day: timedelta = chrono::Duration::zero();
    while current <= end {
        dates.push(current);
        current = current + one_day;
    }
    dates
}
#[doc = "Test time calculations using timedelta"]
#[doc = " Depyler: proven to terminate"]
pub fn test_time_arithmetic() -> Result<i32, ZeroDivisionError> {
    let meeting_start: time =
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
pub fn quarter_of_year(d: &date) -> i32 {
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
    let current_str: String = test_current_datetime()?;
    let my_date: date = test_date_creation()?;
    let my_time: time = test_time_creation()?;
    let my_datetime: datetime = test_datetime_creation()?;
    let days_diff: i32 = test_date_arithmetic()?;
    let delta: timedelta = test_timedelta_creation()?;
    let future_date: date = test_date_addition()?;
    let past_date: date = test_date_subtraction()?;
    let birth: date = chrono::NaiveDate::from_ymd_opt(1990 as i32, 5 as u32, 15 as u32).unwrap();
    let today: date = chrono::NaiveDate::from_ymd_opt(2025 as i32, 11 as u32, 5 as u32).unwrap();
    let age: i32 = calculate_age(birth, today)?;
    let event: date = chrono::NaiveDate::from_ymd_opt(2025 as i32, 12 as u32, 31 as u32).unwrap();
    let days_left: i32 = days_until_event(event, today)?;
    let is_leap_2024: bool = is_leap_year(2024)?;
    let is_leap_2025: bool = is_leap_year(2025)?;
    let days_feb_2024: i32 = days_in_month(2024, 2)?;
    let days_feb_2025: i32 = days_in_month(2025, 2)?;
    let delta_str: String = test_timedelta_components()?;
    let cmp_dt: bool = test_datetime_comparison()?;
    let cmp_date: bool = test_date_comparison()?;
    let start: date = chrono::NaiveDate::from_ymd_opt(2025 as i32, 1 as u32, 1 as u32).unwrap();
    let end: date = chrono::NaiveDate::from_ymd_opt(2025 as i32, 12 as u32, 31 as u32).unwrap();
    let work_days: i32 = working_days_between(start, end)?;
    let future: date = add_business_days(today, 10)?;
    let formatted: String = test_datetime_formatting()?;
    let range_start: date =
        chrono::NaiveDate::from_ymd_opt(2025 as i32, 11 as u32, 1 as u32).unwrap();
    let range_end: date =
        chrono::NaiveDate::from_ymd_opt(2025 as i32, 11 as u32, 7 as u32).unwrap();
    let date_list: Vec<date> = test_date_range(range_start, range_end)?;
    let meeting_end: i32 = test_time_arithmetic()?;
    let q: i32 = quarter_of_year(today)?;
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
