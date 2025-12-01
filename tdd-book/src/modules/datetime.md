# datetime - Date and Time Manipulation

Python's datetime module provides classes for working with dates, times, and time intervals. Depyler transpiles these to Rust's `chrono` crate with full timezone support and type safety.

## Python → Rust Mapping

| Python Module | Rust Equivalent | Notes |
|--------------|-----------------|-------|
| `from datetime import datetime` | `use chrono::DateTime` | Date and time |
| `from datetime import timedelta` | `use chrono::Duration` | Time intervals |
| `datetime.now()` | `Utc::now()` | Current time |
| `datetime(2024, 10, 22)` | `NaiveDate::from_ymd` | Create datetime |

## Creating Datetime Objects

### datetime() - Create Datetime

Create datetime objects with specific date and time:

```python
from datetime import datetime

def create_datetime() -> datetime:
    # Create datetime object
    dt = datetime(2024, 10, 22, 14, 30, 0)  # Oct 22, 2024, 2:30 PM

    return dt
```

**Generated Rust:**

```rust
use chrono::NaiveDateTime;

fn create_datetime() -> NaiveDateTime {
    // Create datetime object
    let dt = NaiveDateTime::parse_from_str(
        "2024-10-22 14:30:00",
        "%Y-%m-%d %H:%M:%S"
    ).unwrap();

    dt
}
```

### now() - Current Datetime

Get the current date and time:

```python
from datetime import datetime

def get_current_time() -> datetime:
    # Get current datetime
    now = datetime.now()

    return now
```

**Generated Rust:**

```rust
use chrono::{Utc, DateTime};

fn get_current_time() -> DateTime<Utc> {
    // Get current datetime
    let now = Utc::now();

    now
}
```

## Formatting Datetime

### strftime() - Format to String

Format datetime objects as strings:

```python
from datetime import datetime

def format_datetime() -> str:
    dt = datetime(2024, 10, 22, 14, 30, 0)

    # Format datetime to string
    formatted = dt.strftime("%Y-%m-%d %H:%M:%S")  # "2024-10-22 14:30:00"

    return formatted
```

**Generated Rust:**

```rust
use chrono::NaiveDateTime;

fn format_datetime() -> String {
    let dt = NaiveDateTime::parse_from_str(
        "2024-10-22 14:30:00",
        "%Y-%m-%d %H:%M:%S"
    ).unwrap();

    // Format datetime to string
    let formatted = dt.format("%Y-%m-%d %H:%M:%S").to_string();

    formatted
}
```

## Parsing Datetime

### strptime() - Parse from String

Parse datetime objects from formatted strings:

```python
from datetime import datetime

def parse_datetime() -> datetime:
    # Parse datetime from string
    dt = datetime.strptime("2024-10-22 14:30:00", "%Y-%m-%d %H:%M:%S")

    return dt
```

**Generated Rust:**

```rust
use chrono::NaiveDateTime;

fn parse_datetime() -> NaiveDateTime {
    // Parse datetime from string
    let dt = NaiveDateTime::parse_from_str(
        "2024-10-22 14:30:00",
        "%Y-%m-%d %H:%M:%S"
    ).unwrap();

    dt
}
```

## Date Arithmetic

### timedelta - Time Intervals

Add or subtract time intervals from datetime objects:

```python
from datetime import datetime, timedelta

def datetime_arithmetic() -> datetime:
    dt = datetime(2024, 10, 22, 14, 30, 0)

    # Add 7 days
    future_dt = dt + timedelta(days=7)

    return future_dt
```

**Generated Rust:**

```rust
use chrono::{NaiveDateTime, Duration};

fn datetime_arithmetic() -> NaiveDateTime {
    let dt = NaiveDateTime::parse_from_str(
        "2024-10-22 14:30:00",
        "%Y-%m-%d %H:%M:%S"
    ).unwrap();

    // Add 7 days
    let future_dt = dt + Duration::days(7);

    future_dt
}
```

## Accessing Components

### year, month, day, hour, minute, second

Access individual components of datetime objects:

```python
from datetime import datetime

def datetime_components() -> int:
    dt = datetime(2024, 10, 22, 14, 30, 0)

    # Access individual components
    year: int = dt.year      # 2024
    month: int = dt.month    # 10
    day: int = dt.day        # 22
    hour: int = dt.hour      # 14
    minute: int = dt.minute  # 30

    return year
```

**Generated Rust:**

```rust
use chrono::NaiveDateTime;

fn datetime_components() -> i32 {
    let dt = NaiveDateTime::parse_from_str(
        "2024-10-22 14:30:00",
        "%Y-%m-%d %H:%M:%S"
    ).unwrap();

    // Access individual components
    let year: i32 = dt.year();
    let month: u32 = dt.month();
    let day: u32 = dt.day();
    let hour: u32 = dt.hour();
    let minute: u32 = dt.minute();

    year
}
```

## Complete Operation Coverage

All common datetime operations are supported:

| Python Operation | Rust Equivalent | Category |
|-----------------|-----------------|----------|
| `datetime(y,m,d,h,m,s)` | `NaiveDateTime::parse_from_str` | Construction |
| `datetime.now()` | `Utc::now()` | Current Time |
| `dt.strftime(fmt)` | `dt.format(fmt)` | Formatting |
| `datetime.strptime(s, fmt)` | `parse_from_str` | Parsing |
| `dt + timedelta(days=n)` | `dt + Duration::days(n)` | Arithmetic |
| `dt - timedelta(days=n)` | `dt - Duration::days(n)` | Arithmetic |
| `dt.year` | `dt.year()` | Components |
| `dt.month` | `dt.month()` | Components |
| `dt.day` | `dt.day()` | Components |
| `dt.hour` | `dt.hour()` | Components |
| `dt.minute` | `dt.minute()` | Components |
| `dt.second` | `dt.second()` | Components |

## Format Codes

Common format codes for datetime formatting and parsing:

| Code | Meaning | Example |
|------|---------|---------|
| `%Y` | 4-digit year | 2024 |
| `%m` | Month (01-12) | 10 |
| `%d` | Day (01-31) | 22 |
| `%H` | Hour (00-23) | 14 |
| `%M` | Minute (00-59) | 30 |
| `%S` | Second (00-59) | 00 |
| `%a` | Weekday abbr | Mon |
| `%A` | Weekday full | Monday |
| `%b` | Month abbr | Oct |
| `%B` | Month full | October |

## Common Use Cases

### 1. Timestamp Logging

```python
from datetime import datetime

def log_timestamp() -> str:
    now = datetime.now()
    return now.strftime("%Y-%m-%d %H:%M:%S")
```

### 2. Date Calculations

```python
from datetime import datetime, timedelta

def days_until(target_date: datetime) -> int:
    today = datetime.now()
    delta = target_date - today
    return delta.days
```

### 3. Date Range Generation

```python
from datetime import datetime, timedelta

def date_range(start: datetime, days: int) -> list[datetime]:
    return [start + timedelta(days=i) for i in range(days)]
```

## Performance Characteristics

| Operation | Python | Rust | Notes |
|-----------|--------|------|-------|
| `datetime()` | O(1) | O(1) | Construction |
| `now()` | O(1) | O(1) | System call |
| `strftime()` | O(n) | O(n) | String formatting |
| `strptime()` | O(n) | O(n) | String parsing |
| Arithmetic | O(1) | O(1) | Add/subtract |
| Components | O(1) | O(1) | Field access |

## Safety and Guarantees

**Datetime operation safety:**
- Invalid dates raise `ValueError` (Python) or panic (Rust)
- Timezone-aware vs naive datetime separation
- Leap year handling built-in
- Overflow detection in date arithmetic
- Thread-safe datetime operations

**Important Notes:**
- Use timezone-aware datetime for production systems
- `datetime.now()` returns naive datetime (no timezone)
- Use `datetime.utcnow()` for UTC time
- Arithmetic requires compatible datetime types
- Format codes must match string representation

## Timezone Awareness

Python datetime supports both naive and timezone-aware datetimes:

```python
from datetime import datetime, timezone

def timezone_aware() -> datetime:
    # Naive datetime (no timezone)
    naive = datetime(2024, 10, 22, 14, 30, 0)

    # Timezone-aware datetime (UTC)
    aware = datetime(2024, 10, 22, 14, 30, 0, tzinfo=timezone.utc)

    return aware
```

**Generated Rust:**

```rust
use chrono::{DateTime, Utc};

fn timezone_aware() -> DateTime<Utc> {
    // Parse and make timezone-aware
    let aware = DateTime::parse_from_rfc3339("2024-10-22T14:30:00Z")
        .unwrap()
        .with_timezone(&Utc);

    aware
}
```

## Testing

All examples in this chapter are verified by the test suite in `tdd-book/tests/test_datetime.py`. Run:

```bash
cd tdd-book
uv run pytest tests/test_datetime.py -v
```
