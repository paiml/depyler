# datetime

## datetime.date - Date objects without time.

## Date arithmetic with timedelta.

## datetime.time - Time objects without date.

## datetime.datetime - Combined date and time.

## datetime.timedelta - Represents duration.

## Edge cases and quirks in datetime module.

## String formatting and parsing.

### Basic: Create a date with year, month, day.

```python
def test_date_creation_basic(self):
    """Basic: Create a date with year, month, day."""
    d = datetime.date(2024, 10, 3)
    assert d.year == 2024
    assert d.month == 10
    assert d.day == 3
```

**Verification**: ✅ Tested in CI

### Basic: Get today's date.

```python
def test_date_today(self):
    """Basic: Get today's date."""
    today = datetime.date.today()
    assert isinstance(today, datetime.date)
    assert today.year >= 2024
```

**Verification**: ✅ Tested in CI

### Feature: Create date from Unix timestamp.

```python
def test_date_from_timestamp(self):
    """Feature: Create date from Unix timestamp."""
    d = datetime.date.fromtimestamp(1704067200)
    assert d.year == 2024
    assert d.month == 1
    assert d.day == 1
```

**Verification**: ✅ Tested in CI

### Error: Invalid date values raise ValueError.

```python
def test_date_invalid_values_raise(self):
    """Error: Invalid date values raise ValueError."""
    with pytest.raises(ValueError):
        datetime.date(2024, 13, 1)
    with pytest.raises(ValueError):
        datetime.date(2024, 2, 30)
    with pytest.raises(ValueError):
        datetime.date(2024, 0, 1)
```

**Verification**: ✅ Tested in CI

### Edge: Minimum and maximum date values.

```python
def test_date_min_max(self):
    """Edge: Minimum and maximum date values."""
    min_date = datetime.date.min
    assert min_date.year == 1
    assert min_date.month == 1
    assert min_date.day == 1
    max_date = datetime.date.max
    assert max_date.year == 9999
    assert max_date.month == 12
    assert max_date.day == 31
```

**Verification**: ✅ Tested in CI

### Basic: Add days to a date using timedelta.

```python
def test_add_days_to_date(self):
    """Basic: Add days to a date using timedelta."""
    d = datetime.date(2024, 1, 1)
    new_date = d + datetime.timedelta(days=10)
    assert new_date == datetime.date(2024, 1, 11)
```

**Verification**: ✅ Tested in CI

### Basic: Subtract dates to get timedelta.

```python
def test_subtract_dates(self):
    """Basic: Subtract dates to get timedelta."""
    d1 = datetime.date(2024, 1, 10)
    d2 = datetime.date(2024, 1, 1)
    delta = d1 - d2
    assert delta.days == 9
    assert isinstance(delta, datetime.timedelta)
```

**Verification**: ✅ Tested in CI

### Property: Date comparison operators work.

```python
def test_date_comparison(self):
    """Property: Date comparison operators work."""
    d1 = datetime.date(2024, 1, 1)
    d2 = datetime.date(2024, 1, 2)
    d3 = datetime.date(2024, 1, 1)
    assert d1 < d2
    assert d2 > d1
    assert d1 == d3
    assert d1 != d2
```

**Verification**: ✅ Tested in CI

### Edge: February 29th exists in leap years.

```python
def test_leap_year_handling(self):
    """Edge: February 29th exists in leap years."""
    leap_date = datetime.date(2024, 2, 29)
    assert leap_date.day == 29
    with pytest.raises(ValueError):
        datetime.date(2023, 2, 29)
```

**Verification**: ✅ Tested in CI

### Edge: Adding days rolls over months correctly.

```python
def test_month_rollover(self):
    """Edge: Adding days rolls over months correctly."""
    d = datetime.date(2024, 1, 31)
    new_date = d + datetime.timedelta(days=1)
    assert new_date.month == 2
    assert new_date.day == 1
```

**Verification**: ✅ Tested in CI

### Basic: Create a time with hour, minute, second.

```python
def test_time_creation_basic(self):
    """Basic: Create a time with hour, minute, second."""
    t = datetime.time(14, 30, 45)
    assert t.hour == 14
    assert t.minute == 30
    assert t.second == 45
```

**Verification**: ✅ Tested in CI

### Feature: Time supports microseconds.

```python
def test_time_with_microseconds(self):
    """Feature: Time supports microseconds."""
    t = datetime.time(12, 30, 45, 123456)
    assert t.microsecond == 123456
```

**Verification**: ✅ Tested in CI

### Error: Invalid time values raise ValueError.

```python
def test_time_invalid_values_raise(self):
    """Error: Invalid time values raise ValueError."""
    with pytest.raises(ValueError):
        datetime.time(24, 0, 0)
    with pytest.raises(ValueError):
        datetime.time(12, 60, 0)
    with pytest.raises(ValueError):
        datetime.time(12, 30, 60)
```

**Verification**: ✅ Tested in CI

### Edge: Minimum and maximum time values.

```python
def test_time_min_max(self):
    """Edge: Minimum and maximum time values."""
    min_time = datetime.time.min
    assert min_time.hour == 0
    assert min_time.minute == 0
    assert min_time.second == 0
    max_time = datetime.time.max
    assert max_time.hour == 23
    assert max_time.minute == 59
    assert max_time.second == 59
```

**Verification**: ✅ Tested in CI

### Property: Time comparison operators work.

```python
def test_time_comparison(self):
    """Property: Time comparison operators work."""
    t1 = datetime.time(10, 0, 0)
    t2 = datetime.time(15, 0, 0)
    t3 = datetime.time(10, 0, 0)
    assert t1 < t2
    assert t2 > t1
    assert t1 == t3
```

**Verification**: ✅ Tested in CI

### Basic: Create datetime with year, month, day, hour, minute, second.

```python
def test_datetime_creation_basic(self):
    """Basic: Create datetime with year, month, day, hour, minute, second."""
    dt = datetime.datetime(2024, 10, 3, 14, 30, 45)
    assert dt.year == 2024
    assert dt.month == 10
    assert dt.day == 3
    assert dt.hour == 14
    assert dt.minute == 30
    assert dt.second == 45
```

**Verification**: ✅ Tested in CI

### Basic: Get current datetime.

```python
def test_datetime_now(self):
    """Basic: Get current datetime."""
    now = datetime.datetime.now()
    assert isinstance(now, datetime.datetime)
    assert now.year >= 2024
```

**Verification**: ✅ Tested in CI

### Feature: Combine date and time objects.

```python
def test_datetime_combine(self):
    """Feature: Combine date and time objects."""
    d = datetime.date(2024, 10, 3)
    t = datetime.time(14, 30, 45)
    dt = datetime.datetime.combine(d, t)
    assert dt.year == 2024
    assert dt.month == 10
    assert dt.day == 3
    assert dt.hour == 14
    assert dt.minute == 30
```

**Verification**: ✅ Tested in CI

### Feature: Extract date and time from datetime.

```python
def test_datetime_to_date_and_time(self):
    """Feature: Extract date and time from datetime."""
    dt = datetime.datetime(2024, 10, 3, 14, 30, 45)
    d = dt.date()
    assert d == datetime.date(2024, 10, 3)
    t = dt.time()
    assert t == datetime.time(14, 30, 45)
```

**Verification**: ✅ Tested in CI

### Feature: Format datetime as string.

```python
def test_datetime_strftime(self):
    """Feature: Format datetime as string."""
    dt = datetime.datetime(2024, 10, 3, 14, 30, 45)
    formatted = dt.strftime('%Y-%m-%d %H:%M:%S')
    assert formatted == '2024-10-03 14:30:45'
```

**Verification**: ✅ Tested in CI

### Feature: Parse string to datetime.

```python
def test_datetime_strptime(self):
    """Feature: Parse string to datetime."""
    dt = datetime.datetime.strptime('2024-10-03 14:30:45', '%Y-%m-%d %H:%M:%S')
    assert dt.year == 2024
    assert dt.month == 10
    assert dt.day == 3
    assert dt.hour == 14
```

**Verification**: ✅ Tested in CI

### Basic: Create timedelta with days, seconds, microseconds.

```python
def test_timedelta_creation(self):
    """Basic: Create timedelta with days, seconds, microseconds."""
    td = datetime.timedelta(days=5, hours=3, minutes=30)
    assert td.days == 5
    assert td.seconds == 3 * 3600 + 30 * 60
```

**Verification**: ✅ Tested in CI

### Property: Timedelta supports arithmetic operations.

```python
def test_timedelta_arithmetic(self):
    """Property: Timedelta supports arithmetic operations."""
    td1 = datetime.timedelta(days=5)
    td2 = datetime.timedelta(days=3)
    assert td1 + td2 == datetime.timedelta(days=8)
    assert td1 - td2 == datetime.timedelta(days=2)
    assert td1 * 2 == datetime.timedelta(days=10)
```

**Verification**: ✅ Tested in CI

### Feature: Get total duration in seconds.

```python
def test_timedelta_total_seconds(self):
    """Feature: Get total duration in seconds."""
    td = datetime.timedelta(days=1, hours=2, minutes=3, seconds=4)
    total = td.total_seconds()
    expected = 86400 + 7200 + 180 + 4
    assert total == expected
```

**Verification**: ✅ Tested in CI

### Edge: Timedelta can be negative.

```python
def test_timedelta_negative(self):
    """Edge: Timedelta can be negative."""
    td = datetime.timedelta(days=-5)
    assert td.days == -5
    d1 = datetime.date(2024, 1, 1)
    d2 = datetime.date(2024, 1, 10)
    delta = d1 - d2
    assert delta.days == -9
```

**Verification**: ✅ Tested in CI

### Property: Timedelta comparison operators work.

```python
def test_timedelta_comparison(self):
    """Property: Timedelta comparison operators work."""
    td1 = datetime.timedelta(days=5)
    td2 = datetime.timedelta(days=10)
    td3 = datetime.timedelta(days=5)
    assert td1 < td2
    assert td2 > td1
    assert td1 == td3
```

**Verification**: ✅ Tested in CI

### Edge: Year 2000 is a leap year (divisible by 400).

```python
def test_year_2000_is_leap_year(self):
    """Edge: Year 2000 is a leap year (divisible by 400)."""
    leap_date = datetime.date(2000, 2, 29)
    assert leap_date.day == 29
```

**Verification**: ✅ Tested in CI

### Edge: Year 1900 is NOT a leap year (divisible by 100, not 400).

```python
def test_year_1900_not_leap_year(self):
    """Edge: Year 1900 is NOT a leap year (divisible by 100, not 400)."""
    with pytest.raises(ValueError):
        datetime.date(1900, 2, 29)
```

**Verification**: ✅ Tested in CI

### Edge: Datetime supports microsecond precision.

```python
def test_datetime_microsecond_precision(self):
    """Edge: Datetime supports microsecond precision."""
    dt = datetime.datetime(2024, 1, 1, 12, 0, 0, 999999)
    assert dt.microsecond == 999999
    new_dt = dt + datetime.timedelta(microseconds=1)
    assert new_dt.second == 1
    assert new_dt.microsecond == 0
```

**Verification**: ✅ Tested in CI

### Feature: ISO 8601 format support.

```python
def test_date_iso_format(self):
    """Feature: ISO 8601 format support."""
    d = datetime.date(2024, 10, 3)
    assert d.isoformat() == '2024-10-03'
    dt = datetime.datetime(2024, 10, 3, 14, 30, 45)
    assert dt.isoformat() == '2024-10-03T14:30:45'
```

**Verification**: ✅ Tested in CI

### Edge: weekday() returns 0 for Monday.

```python
def test_weekday_monday_is_zero(self):
    """Edge: weekday() returns 0 for Monday."""
    d = datetime.date(2024, 10, 3)
    assert d.weekday() == 3
    monday = datetime.date(2024, 9, 30)
    assert monday.weekday() == 0
```

**Verification**: ✅ Tested in CI

### Edge: isoweekday() returns 1 for Monday (ISO standard).

```python
def test_isoweekday_monday_is_one(self):
    """Edge: isoweekday() returns 1 for Monday (ISO standard)."""
    monday = datetime.date(2024, 9, 30)
    assert monday.isoweekday() == 1
```

**Verification**: ✅ Tested in CI

### Property: Years 1-9999 are valid.

```python
@given(st.integers(min_value=1, max_value=9999))
def test_year_range_valid(self, year):
    """Property: Years 1-9999 are valid."""
    d = datetime.date(year, 1, 1)
    assert d.year == year
```

**Verification**: ✅ Tested in CI

### Feature: Common strftime format codes.

```python
def test_common_format_codes(self):
    """Feature: Common strftime format codes."""
    dt = datetime.datetime(2024, 10, 3, 14, 5, 7)
    assert dt.strftime('%Y') == '2024'
    assert dt.strftime('%m') == '10'
    assert dt.strftime('%d') == '03'
    assert dt.strftime('%H') == '14'
    assert dt.strftime('%M') == '05'
    assert dt.strftime('%S') == '07'
```

**Verification**: ✅ Tested in CI

### Error: Invalid format string raises ValueError.

```python
def test_parse_invalid_format_raises(self):
    """Error: Invalid format string raises ValueError."""
    with pytest.raises(ValueError):
        datetime.datetime.strptime('2024-10-03', '%Y/%m/%d')
```

**Verification**: ✅ Tested in CI
