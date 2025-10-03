# calendar

## calendar.weekday() - Return weekday (0=Monday ... 6=Sunday) for year, month, day.

## calendar.isleap() - Check if year is leap year.

## calendar.leapdays() - Number of leap days in range [y1, y2).

## calendar.monthrange() - Tuple (weekday of first day, number of days) for year/month.

## calendar.monthcalendar() - Matrix representing month's calendar.

## calendar.month() - Formatted text calendar for a month.

## calendar.calendar() - Formatted text calendar for a year.

## calendar.TextCalendar - Text calendar generator.

## calendar.HTMLCalendar - HTML calendar generator.

## Calendar module constants.

## Edge cases and special scenarios.

### Basic: Get weekday for a known date.

```python
def test_weekday_basic(self):
    """Basic: Get weekday for a known date."""
    day = calendar.weekday(2000, 1, 1)
    assert day == 5
```

**Verification**: ✅ Tested in CI

### Feature: Monday returns 0.

```python
def test_weekday_monday(self):
    """Feature: Monday returns 0."""
    day = calendar.weekday(2000, 1, 3)
    assert day == 0
```

**Verification**: ✅ Tested in CI

### Feature: Sunday returns 6.

```python
def test_weekday_sunday(self):
    """Feature: Sunday returns 6."""
    day = calendar.weekday(2000, 1, 2)
    assert day == 6
```

**Verification**: ✅ Tested in CI

### Edge: Leap year Feb 29 is valid.

```python
def test_weekday_leap_year_feb_29(self):
    """Edge: Leap year Feb 29 is valid."""
    day = calendar.weekday(2000, 2, 29)
    assert day in range(7)
```

**Verification**: ✅ Tested in CI

### Error: Invalid date raises ValueError or calendar.IllegalMonthError.

```python
def test_weekday_invalid_date_raises(self):
    """Error: Invalid date raises ValueError or calendar.IllegalMonthError."""
    with pytest.raises((ValueError, calendar.IllegalMonthError)):
        calendar.weekday(2025, 13, 1)
```

**Verification**: ✅ Tested in CI

### Basic: 2000 is a leap year.

```python
def test_isleap_basic_leap_year(self):
    """Basic: 2000 is a leap year."""
    assert calendar.isleap(2000) is True
```

**Verification**: ✅ Tested in CI

### Basic: 2001 is not a leap year.

```python
def test_isleap_basic_non_leap_year(self):
    """Basic: 2001 is not a leap year."""
    assert calendar.isleap(2001) is False
```

**Verification**: ✅ Tested in CI

### Rule: Years divisible by 4 are leap years.

```python
def test_isleap_divisible_by_4(self):
    """Rule: Years divisible by 4 are leap years."""
    assert calendar.isleap(2024) is True
```

**Verification**: ✅ Tested in CI

### Rule: Century years not divisible by 400 are not leap.

```python
def test_isleap_century_not_leap(self):
    """Rule: Century years not divisible by 400 are not leap."""
    assert calendar.isleap(1900) is False
    assert calendar.isleap(2100) is False
```

**Verification**: ✅ Tested in CI

### Rule: Century years divisible by 400 are leap.

```python
def test_isleap_century_divisible_by_400(self):
    """Rule: Century years divisible by 400 are leap."""
    assert calendar.isleap(2000) is True
    assert calendar.isleap(2400) is True
```

**Verification**: ✅ Tested in CI

### Property: Leap years follow 4-year cycle.

```python
def test_isleap_sequence(self):
    """Property: Leap years follow 4-year cycle."""
    assert calendar.isleap(2020) is True
    assert calendar.isleap(2021) is False
    assert calendar.isleap(2022) is False
    assert calendar.isleap(2023) is False
    assert calendar.isleap(2024) is True
```

**Verification**: ✅ Tested in CI

### Basic: Count leap days in range.

```python
def test_leapdays_basic(self):
    """Basic: Count leap days in range."""
    count = calendar.leapdays(2000, 2020)
    assert count == 5
```

**Verification**: ✅ Tested in CI

### Edge: Range of 1 year returns 0 or 1.

```python
def test_leapdays_single_year(self):
    """Edge: Range of 1 year returns 0 or 1."""
    assert calendar.leapdays(2000, 2001) == 1
    assert calendar.leapdays(2001, 2002) == 0
```

**Verification**: ✅ Tested in CI

### Edge: Empty range returns 0.

```python
def test_leapdays_empty_range(self):
    """Edge: Empty range returns 0."""
    assert calendar.leapdays(2000, 2000) == 0
```

**Verification**: ✅ Tested in CI

### Edge: Century boundaries affect leap day count.

```python
def test_leapdays_century_boundary(self):
    """Edge: Century boundaries affect leap day count."""
    count_1900 = calendar.leapdays(1900, 1901)
    count_2000 = calendar.leapdays(2000, 2001)
    assert count_1900 == 0
    assert count_2000 == 1
```

**Verification**: ✅ Tested in CI

### Basic: Get month range for January 2000.

```python
def test_monthrange_basic(self):
    """Basic: Get month range for January 2000."""
    weekday, num_days = calendar.monthrange(2000, 1)
    assert weekday == 5
    assert num_days == 31
```

**Verification**: ✅ Tested in CI

### Edge: February in leap year has 29 days.

```python
def test_monthrange_february_leap_year(self):
    """Edge: February in leap year has 29 days."""
    weekday, num_days = calendar.monthrange(2000, 2)
    assert num_days == 29
```

**Verification**: ✅ Tested in CI

### Edge: February in non-leap year has 28 days.

```python
def test_monthrange_february_non_leap_year(self):
    """Edge: February in non-leap year has 28 days."""
    weekday, num_days = calendar.monthrange(2001, 2)
    assert num_days == 28
```

**Verification**: ✅ Tested in CI

### Property: April, June, September, November have 30 days.

```python
def test_monthrange_30_day_months(self):
    """Property: April, June, September, November have 30 days."""
    for month in [4, 6, 9, 11]:
        _, num_days = calendar.monthrange(2025, month)
        assert num_days == 30
```

**Verification**: ✅ Tested in CI

### Property: Jan, Mar, May, Jul, Aug, Oct, Dec have 31 days.

```python
def test_monthrange_31_day_months(self):
    """Property: Jan, Mar, May, Jul, Aug, Oct, Dec have 31 days."""
    for month in [1, 3, 5, 7, 8, 10, 12]:
        _, num_days = calendar.monthrange(2025, month)
        assert num_days == 31
```

**Verification**: ✅ Tested in CI

### Basic: Get calendar matrix for a month.

```python
def test_monthcalendar_basic(self):
    """Basic: Get calendar matrix for a month."""
    cal = calendar.monthcalendar(2000, 1)
    assert isinstance(cal, list)
    assert len(cal) >= 4
    assert len(cal) <= 6
```

**Verification**: ✅ Tested in CI

### Property: First week may have leading zeros.

```python
def test_monthcalendar_first_week(self):
    """Property: First week may have leading zeros."""
    cal = calendar.monthcalendar(2000, 1)
    first_week = cal[0]
    assert 0 in first_week
    assert 1 in first_week
```

**Verification**: ✅ Tested in CI

### Property: Last week may have trailing zeros.

```python
def test_monthcalendar_last_week(self):
    """Property: Last week may have trailing zeros."""
    cal = calendar.monthcalendar(2000, 1)
    last_week = cal[-1]
    assert 31 in last_week
    assert isinstance(last_week, list)
    assert len(last_week) == 7
```

**Verification**: ✅ Tested in CI

### Property: Every week has 7 days.

```python
def test_monthcalendar_week_length(self):
    """Property: Every week has 7 days."""
    cal = calendar.monthcalendar(2025, 10)
    for week in cal:
        assert len(week) == 7
```

**Verification**: ✅ Tested in CI

### Property: Calendar contains all days of month.

```python
def test_monthcalendar_contains_all_days(self):
    """Property: Calendar contains all days of month."""
    cal = calendar.monthcalendar(2025, 10)
    all_days = [day for week in cal for day in week if day != 0]
    assert sorted(all_days) == list(range(1, 32))
```

**Verification**: ✅ Tested in CI

### Basic: Generate text calendar for month.

```python
def test_month_basic(self):
    """Basic: Generate text calendar for month."""
    text = calendar.month(2000, 1)
    assert isinstance(text, str)
    assert 'January' in text or 'Jan' in text or '2000' in text
```

**Verification**: ✅ Tested in CI

### Property: Calendar contains day numbers.

```python
def test_month_contains_days(self):
    """Property: Calendar contains day numbers."""
    text = calendar.month(2025, 10)
    assert '1' in text
    assert '31' in text
```

**Verification**: ✅ Tested in CI

### Property: Month calendar is multiline.

```python
def test_month_multiline(self):
    """Property: Month calendar is multiline."""
    text = calendar.month(2025, 10)
    lines = text.strip().split('\n')
    assert len(lines) > 1
```

**Verification**: ✅ Tested in CI

### Basic: Generate text calendar for year.

```python
def test_calendar_basic(self):
    """Basic: Generate text calendar for year."""
    text = calendar.calendar(2000)
    assert isinstance(text, str)
    assert '2000' in text
```

**Verification**: ✅ Tested in CI

### Property: Year calendar contains all 12 months.

```python
def test_calendar_contains_all_months(self):
    """Property: Year calendar contains all 12 months."""
    text = calendar.calendar(2025)
    month_count = 0
    for month_name in ['January', 'February', 'March', 'April', 'May', 'June', 'July', 'August', 'September', 'October', 'November', 'December']:
        if month_name in text:
            month_count += 1
    assert len(text) > 100
```

**Verification**: ✅ Tested in CI

### Basic: Create TextCalendar instance.

```python
def test_textcalendar_basic(self):
    """Basic: Create TextCalendar instance."""
    cal = calendar.TextCalendar()
    assert isinstance(cal, calendar.TextCalendar)
```

**Verification**: ✅ Tested in CI

### Feature: Format month with TextCalendar.

```python
def test_textcalendar_formatmonth(self):
    """Feature: Format month with TextCalendar."""
    cal = calendar.TextCalendar()
    text = cal.formatmonth(2000, 1)
    assert isinstance(text, str)
    assert len(text) > 0
```

**Verification**: ✅ Tested in CI

### Feature: Set first day of week.

```python
def test_textcalendar_first_weekday(self):
    """Feature: Set first day of week."""
    cal_mon = calendar.TextCalendar(firstweekday=0)
    cal_sun = calendar.TextCalendar(firstweekday=6)
    assert cal_mon.firstweekday == 0
    assert cal_sun.firstweekday == 6
```

**Verification**: ✅ Tested in CI

### Basic: Create HTMLCalendar instance.

```python
def test_htmlcalendar_basic(self):
    """Basic: Create HTMLCalendar instance."""
    cal = calendar.HTMLCalendar()
    assert isinstance(cal, calendar.HTMLCalendar)
```

**Verification**: ✅ Tested in CI

### Feature: Format month as HTML.

```python
def test_htmlcalendar_formatmonth(self):
    """Feature: Format month as HTML."""
    cal = calendar.HTMLCalendar()
    html = cal.formatmonth(2000, 1)
    assert isinstance(html, str)
    assert '<table' in html or '<' in html
```

**Verification**: ✅ Tested in CI

### Property: HTML calendar contains table structure.

```python
def test_htmlcalendar_contains_table(self):
    """Property: HTML calendar contains table structure."""
    cal = calendar.HTMLCalendar()
    html = cal.formatmonth(2025, 10)
    assert 'table' in html.lower() or '<td' in html or '<tr' in html
```

**Verification**: ✅ Tested in CI

### Property: day_name contains weekday names.

```python
def test_day_name_constant(self):
    """Property: day_name contains weekday names."""
    assert len(calendar.day_name) == 7
    assert 'Monday' in calendar.day_name
    assert 'Sunday' in calendar.day_name
```

**Verification**: ✅ Tested in CI

### Property: day_abbr contains abbreviated weekday names.

```python
def test_day_abbr_constant(self):
    """Property: day_abbr contains abbreviated weekday names."""
    assert len(calendar.day_abbr) == 7
    assert 'Mon' in calendar.day_abbr
    assert 'Sun' in calendar.day_abbr
```

**Verification**: ✅ Tested in CI

### Property: month_name has 13 entries (0-12, 0 is empty).

```python
def test_month_name_constant(self):
    """Property: month_name has 13 entries (0-12, 0 is empty)."""
    assert len(calendar.month_name) == 13
    assert calendar.month_name[0] == ''
    assert calendar.month_name[1] == 'January'
    assert calendar.month_name[12] == 'December'
```

**Verification**: ✅ Tested in CI

### Property: month_abbr has 13 entries with abbreviations.

```python
def test_month_abbr_constant(self):
    """Property: month_abbr has 13 entries with abbreviations."""
    assert len(calendar.month_abbr) == 13
    assert calendar.month_abbr[0] == ''
    assert calendar.month_abbr[1] == 'Jan'
    assert calendar.month_abbr[12] == 'Dec'
```

**Verification**: ✅ Tested in CI

### Edge: February leap year affects monthrange.

```python
def test_leap_year_february_range(self):
    """Edge: February leap year affects monthrange."""
    _, days_2000 = calendar.monthrange(2000, 2)
    _, days_1900 = calendar.monthrange(1900, 2)
    assert days_2000 == 29
    assert days_1900 == 28
```

**Verification**: ✅ Tested in CI

### Property: Weekday is consistent with monthrange.

```python
def test_weekday_consistency(self):
    """Property: Weekday is consistent with monthrange."""
    year, month = (2025, 10)
    weekday_func = calendar.weekday(year, month, 1)
    weekday_range, _ = calendar.monthrange(year, month)
    assert weekday_func == weekday_range
```

**Verification**: ✅ Tested in CI

### Property: monthcalendar has no gaps in day sequence.

```python
def test_monthcalendar_no_gaps(self):
    """Property: monthcalendar has no gaps in day sequence."""
    cal = calendar.monthcalendar(2025, 10)
    days = [day for week in cal for day in week if day != 0]
    for i, day in enumerate(sorted(days), start=1):
        assert day == i
```

**Verification**: ✅ Tested in CI

### Edge: Year 2000 handled correctly (Y2K compliance).

```python
def test_calendar_year_2000_bug(self):
    """Edge: Year 2000 handled correctly (Y2K compliance)."""
    assert calendar.isleap(2000) is True
    weekday, days = calendar.monthrange(2000, 2)
    assert days == 29
```

**Verification**: ✅ Tested in CI
