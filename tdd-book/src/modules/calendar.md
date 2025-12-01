# calendar - Calendar Operations and Date Calculations

Python's calendar module provides functions for working with calendars, including determining weekdays, leap years, and generating calendar matrices. Depyler transpiles these to Rust's `chrono` crate with full calendar functionality and type safety.

## Python → Rust Mapping

| Python Module | Rust Equivalent | Notes |
|--------------|-----------------|-------|
| `import calendar` | `use chrono::*` | Calendar operations |
| `calendar.weekday(y,m,d)` | `NaiveDate::weekday()` | Day of week (0=Mon) |
| `calendar.isleap(year)` | Custom leap logic | Leap year check |
| `calendar.leapdays(y1,y2)` | Custom range logic | Count leap days |
| `calendar.monthrange(y,m)` | `NaiveDate` methods | Month info |
| `calendar.monthcalendar(y,m)` | Custom calendar matrix | Calendar grid |

## Day of Week Calculations

### weekday() - Get Day of Week

Get the day of week for a specific date (0=Monday, 6=Sunday):

```python
import calendar

def get_weekday() -> int:
    # Get weekday for January 1, 2000 (Saturday = 5)
    day = calendar.weekday(2000, 1, 1)

    return day
```

**Generated Rust:**

```rust
use chrono::NaiveDate;

fn get_weekday() -> i32 {
    // Get weekday for January 1, 2000 (Saturday = 5)
    let date = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
    let day = date.weekday().num_days_from_monday() as i32;

    day
}
```

**Weekday Mapping:**
- 0 = Monday
- 1 = Tuesday
- 2 = Wednesday
- 3 = Thursday
- 4 = Friday
- 5 = Saturday
- 6 = Sunday

## Leap Year Detection

### isleap() - Check Leap Year

Determine if a year is a leap year:

```python
import calendar

def check_leap_year() -> bool:
    # Check if 2000 is a leap year (it is)
    is_leap = calendar.isleap(2000)

    # Check if 2001 is a leap year (it's not)
    not_leap = calendar.isleap(2001)

    return is_leap and not not_leap
```

**Generated Rust:**

```rust
fn check_leap_year() -> bool {
    // Check if 2000 is a leap year (it is)
    let is_leap = is_leap_year(2000);

    // Check if 2001 is a leap year (it's not)
    let not_leap = is_leap_year(2001);

    is_leap && !not_leap
}

// Helper function for leap year calculation
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
```

**Leap Year Rules:**
1. Years divisible by 4 are leap years
2. **EXCEPT** century years (divisible by 100) are not leap years
3. **EXCEPT** century years divisible by 400 are leap years

**Examples:**
- 2000: Leap year (divisible by 400)
- 1900: Not a leap year (divisible by 100 but not 400)
- 2024: Leap year (divisible by 4)
- 2023: Not a leap year

## Counting Leap Days

### leapdays() - Count Leap Days in Range

Count the number of leap years in a range [y1, y2):

```python
import calendar

def count_leap_days() -> int:
    # Count leap days between 2000 and 2020
    count = calendar.leapdays(2000, 2020)

    return count
```

**Generated Rust:**

```rust
fn count_leap_days() -> i32 {
    // Count leap days between 2000 and 2020
    let count = leapdays(2000, 2020);

    count
}

// Helper function to count leap days in range [y1, y2)
fn leapdays(y1: i32, y2: i32) -> i32 {
    let mut count = 0;
    for year in y1..y2 {
        if is_leap_year(year) {
            count += 1;
        }
    }
    count
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
```

**Range Behavior:**
- Range is [y1, y2) - includes y1, excludes y2
- Empty range (y1 == y2) returns 0
- Negative range returns 0

## Month Information

### monthrange() - Get Month Range

Get the weekday of the first day and number of days in a month:

```python
import calendar

def get_month_info() -> int:
    # Get info for January 2000
    # Returns (weekday of first day, number of days)
    first_weekday, num_days = calendar.monthrange(2000, 1)

    # January has 31 days
    return num_days
```

**Generated Rust:**

```rust
use chrono::NaiveDate;

fn get_month_info() -> i32 {
    // Get info for January 2000
    // Returns (weekday of first day, number of days)
    let (first_weekday, num_days) = monthrange(2000, 1);

    // January has 31 days
    num_days
}

fn monthrange(year: i32, month: u32) -> (i32, i32) {
    let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let first_weekday = first_day.weekday().num_days_from_monday() as i32;

    // Calculate days in month
    let next_month = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap()
    };

    let num_days = (next_month - first_day).num_days() as i32;

    (first_weekday, num_days)
}
```

**Days per Month:**
- January: 31 days
- February: 28 days (29 in leap years)
- March: 31 days
- April: 30 days
- May: 31 days
- June: 30 days
- July: 31 days
- August: 31 days
- September: 30 days
- October: 31 days
- November: 30 days
- December: 31 days

## Calendar Matrix

### monthcalendar() - Generate Calendar Matrix

Generate a matrix representing a month's calendar:

```python
import calendar

def get_month_calendar() -> int:
    # Get calendar matrix for October 2025
    # Returns list of weeks, each week is list of 7 days
    cal = calendar.monthcalendar(2025, 10)

    # Count non-zero days using traditional loops
    count = 0
    for week in cal:
        for day in week:
            if day != 0:
                count += 1

    return count
```

**Generated Rust:**

```rust
use chrono::NaiveDate;

fn get_month_calendar() -> i32 {
    // Get calendar matrix for October 2025
    // Returns list of weeks, each week is list of 7 days
    let cal = monthcalendar(2025, 10);

    // Count non-zero days using traditional loops
    let mut count = 0;
    for week in &cal {
        for &day in week {
            if day != 0 {
                count += 1;
            }
        }
    }

    count
}

fn monthcalendar(year: i32, month: u32) -> Vec<Vec<i32>> {
    let mut weeks: Vec<Vec<i32>> = Vec::new();
    let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let (first_weekday, num_days) = monthrange(year, month);

    let mut week: Vec<i32> = vec![0; first_weekday as usize];

    for day in 1..=num_days {
        week.push(day);
        if week.len() == 7 {
            weeks.push(week);
            week = Vec::new();
        }
    }

    if !week.is_empty() {
        while week.len() < 7 {
            week.push(0);
        }
        weeks.push(week);
    }

    weeks
}

fn monthrange(year: i32, month: u32) -> (i32, i32) {
    let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let first_weekday = first_day.weekday().num_days_from_monday() as i32;

    let next_month = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap()
    };

    let num_days = (next_month - first_day).num_days() as i32;

    (first_weekday, num_days)
}
```

**Matrix Structure:**
- Returns `Vec<Vec<i32>>` (list of weeks)
- Each week has exactly 7 days
- 0 represents days from adjacent months
- First week may have leading zeros
- Last week may have trailing zeros
- Typical matrix has 4-6 weeks

## Complete Function Coverage

All common calendar functions are supported:

| Python Function | Rust Equivalent | Category |
|----------------|-----------------|----------|
| `calendar.weekday(y,m,d)` | `NaiveDate::weekday()` | Day of Week |
| `calendar.isleap(year)` | `is_leap_year()` | Leap Year |
| `calendar.leapdays(y1,y2)` | `leapdays()` | Range Count |
| `calendar.monthrange(y,m)` | `monthrange()` | Month Info |
| `calendar.monthcalendar(y,m)` | `monthcalendar()` | Calendar Matrix |

## Calendar Constants

Python's calendar module provides useful constants:

```python
import calendar

# Day names (full)
calendar.day_name[0]  # 'Monday'
calendar.day_name[6]  # 'Sunday'

# Day abbreviations
calendar.day_abbr[0]  # 'Mon'
calendar.day_abbr[6]  # 'Sun'

# Month names (full, 13 entries with 0 being empty)
calendar.month_name[0]   # ''
calendar.month_name[1]   # 'January'
calendar.month_name[12]  # 'December'

# Month abbreviations
calendar.month_abbr[1]   # 'Jan'
calendar.month_abbr[12]  # 'Dec'
```

## Common Use Cases

### 1. Calculate Day of Week for Any Date

```python
import calendar

def day_name_for_date(year: int, month: int, day: int) -> str:
    weekday = calendar.weekday(year, month, day)
    days = ['Monday', 'Tuesday', 'Wednesday', 'Thursday',
            'Friday', 'Saturday', 'Sunday']
    return days[weekday]
```

### 2. Check If Year Has 366 Days

```python
import calendar

def days_in_year(year: int) -> int:
    return 366 if calendar.isleap(year) else 365
```

### 3. Generate Month Calendar Display

```python
import calendar

def display_month(year: int, month: int) -> None:
    cal = calendar.monthcalendar(year, month)

    # Print header
    print("Mo Tu We Th Fr Sa Su")

    # Print weeks
    for week in cal:
        for day in week:
            if day == 0:
                print("  ", end=" ")
            else:
                print(f"{day:2d}", end=" ")
        print()
```

### 4. Count Business Days (Excluding Weekends)

```python
import calendar

def count_business_days(year: int, month: int) -> int:
    cal = calendar.monthcalendar(year, month)
    business_days = 0

    for week in cal:
        # Days 0-4 are Monday-Friday
        for day_index in range(5):
            if week[day_index] != 0:
                business_days += 1

    return business_days
```

## Performance Characteristics

| Operation | Python | Rust | Notes |
|-----------|--------|------|-------|
| `weekday()` | O(1) | O(1) | Date calculation |
| `isleap()` | O(1) | O(1) | Arithmetic only |
| `leapdays()` | O(n) | O(n) | Linear in range |
| `monthrange()` | O(1) | O(1) | Date arithmetic |
| `monthcalendar()` | O(1) | O(1) | Fixed 4-6 weeks |

## Safety and Guarantees

**Calendar operation safety:**
- Invalid dates raise `ValueError` (Python) or panic (Rust)
- Leap year calculation follows ISO 8601
- Weekday calculation is consistent with ISO 8601 (Monday = 0)
- Month range handles February leap years correctly
- Calendar matrix always has 7 days per week

**Important Notes:**
- `weekday()` uses Monday=0 (differs from some languages)
- `leapdays()` uses [y1, y2) range (excludes y2)
- `monthrange()` returns tuple: (first_weekday, num_days)
- `monthcalendar()` uses 0 for days from adjacent months
- February always handled correctly based on leap year rules

## Leap Year Algorithm

The leap year calculation follows the Gregorian calendar rules:

```python
import calendar

def is_leap_year_explained(year: int) -> bool:
    # Rule 1: Divisible by 4 → leap year
    if year % 4 != 0:
        return False

    # Rule 2: Divisible by 100 → NOT a leap year
    if year % 100 != 0:
        return True

    # Rule 3: Divisible by 400 → leap year
    return year % 400 == 0
```

**Examples:**
- 2024: True (divisible by 4, not by 100)
- 2000: True (divisible by 400)
- 1900: False (divisible by 100, not by 400)
- 2100: False (divisible by 100, not by 400)

## Edge Cases

### February Days
```python
import calendar

def february_days(year: int) -> int:
    _, num_days = calendar.monthrange(year, 2)
    return num_days  # 28 or 29
```

### Y2K Compliance
```python
import calendar

def test_y2k_compliance() -> bool:
    # Year 2000 was correctly handled as leap year
    return calendar.isleap(2000) and \
           calendar.monthrange(2000, 2)[1] == 29
```

### Century Boundaries
```python
import calendar

def century_leap_years() -> list[bool]:
    return [
        calendar.isleap(1600),  # True (divisible by 400)
        calendar.isleap(1700),  # False
        calendar.isleap(1800),  # False
        calendar.isleap(1900),  # False
        calendar.isleap(2000),  # True (divisible by 400)
    ]
```

## Testing

All examples in this chapter are verified by the test suite in `tdd-book/tests/test_calendar.py`. Run:

```bash
cd tdd-book
uv run pytest tests/test_calendar.py -v
```

## Performance Tips

**Optimization strategies:**
- Cache leap year calculations for repeated checks
- Pre-compute month ranges for entire years
- Use `monthcalendar()` matrix directly instead of rebuilding
- Combine multiple calendar operations in single function

**Example: Cached Leap Year Check**
```python
import calendar

_leap_cache = {}

def cached_isleap(year: int) -> bool:
    if year not in _leap_cache:
        _leap_cache[year] = calendar.isleap(year)
    return _leap_cache[year]
```

## Historical Context

The Gregorian calendar (used by `calendar` module):
- Introduced by Pope Gregory XIII in October 1582
- Improved accuracy over Julian calendar
- Leap year rules minimize drift (1 day per ~3,236 years)
- Adopted by different countries at different times
- Standard for civil calendar worldwide

**Accuracy:**
- Solar year: 365.2425 days (Gregorian calendar)
- Actual tropical year: 365.2422 days
- Error: ~1 day every 3,236 years
