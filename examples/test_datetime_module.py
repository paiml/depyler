"""
Comprehensive test of date and time calculations transpilation to Rust.

Rewrites datetime module patterns using pure integer arithmetic.
Dates are represented as (year, month, day) stored in list[int].
Times are represented as (hour, minute, second) stored in list[int].
No imports, no classes, no datetime objects.
"""


def make_date(year: int, month: int, day: int) -> list[int]:
    """Create a date as [year, month, day]."""
    d: list[int] = []
    d.append(year)
    d.append(month)
    d.append(day)
    return d


def make_time(hour: int, minute: int, second: int) -> list[int]:
    """Create a time as [hour, minute, second]."""
    t: list[int] = []
    t.append(hour)
    t.append(minute)
    t.append(second)
    return t


def date_year(d: list[int]) -> int:
    """Get year from date."""
    return d[0]


def date_month(d: list[int]) -> int:
    """Get month from date."""
    return d[1]


def date_day(d: list[int]) -> int:
    """Get day from date."""
    return d[2]


def time_hour(t: list[int]) -> int:
    """Get hour from time."""
    return t[0]


def time_minute(t: list[int]) -> int:
    """Get minute from time."""
    return t[1]


def time_second(t: list[int]) -> int:
    """Get second from time."""
    return t[2]


def is_leap_year(year: int) -> int:
    """Check if a year is a leap year. Returns 1 or 0."""
    if year % 400 == 0:
        return 1
    if year % 100 == 0:
        return 0
    if year % 4 == 0:
        return 1
    return 0


def days_in_month(year: int, month: int) -> int:
    """Get number of days in a specific month."""
    days: list[int] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    if month < 1 or month > 12:
        return 0
    if month == 2 and is_leap_year(year) == 1:
        return 29
    idx: int = month - 1
    return days[idx]


def date_to_days(d: list[int]) -> int:
    """Convert date to absolute day count from year 0 (approximate)."""
    y: int = date_year(d)
    m: int = date_month(d)
    day_val: int = date_day(d)
    total: int = y * 365 + y // 4 - y // 100 + y // 400
    i: int = 1
    while i < m:
        total = total + days_in_month(y, i)
        i = i + 1
    total = total + day_val
    return total


def days_between(d1: list[int], d2: list[int]) -> int:
    """Calculate days between two dates."""
    abs1: int = date_to_days(d1)
    abs2: int = date_to_days(d2)
    diff: int = abs2 - abs1
    if diff < 0:
        diff = 0 - diff
    return diff


def add_days_to_date(d: list[int], num_days: int) -> list[int]:
    """Add days to a date (simplified: just adjusts day count)."""
    y: int = date_year(d)
    m: int = date_month(d)
    day_val: int = date_day(d) + num_days
    dim: int = days_in_month(y, m)
    while day_val > dim:
        day_val = day_val - dim
        m = m + 1
        if m > 12:
            m = 1
            y = y + 1
        dim = days_in_month(y, m)
    result: list[int] = make_date(y, m, day_val)
    return result


def subtract_days_from_date(d: list[int], num_days: int) -> list[int]:
    """Subtract days from a date (simplified)."""
    y: int = date_year(d)
    m: int = date_month(d)
    day_val: int = date_day(d) - num_days
    while day_val < 1:
        m = m - 1
        if m < 1:
            m = 12
            y = y - 1
        dim: int = days_in_month(y, m)
        day_val = day_val + dim
    result: list[int] = make_date(y, m, day_val)
    return result


def calculate_age(birth_year: int, birth_month: int, birth_day: int, curr_year: int, curr_month: int, curr_day: int) -> int:
    """Calculate age in years given birth date and current date."""
    birth: list[int] = make_date(birth_year, birth_month, birth_day)
    current: list[int] = make_date(curr_year, curr_month, curr_day)
    diff: int = days_between(birth, current)
    age_years: int = diff // 365
    return age_years


def days_until_event(event_year: int, event_month: int, event_day: int, curr_year: int, curr_month: int, curr_day: int) -> int:
    """Calculate days until a future event."""
    event: list[int] = make_date(event_year, event_month, event_day)
    current: list[int] = make_date(curr_year, curr_month, curr_day)
    e_days: int = date_to_days(event)
    c_days: int = date_to_days(current)
    if e_days < c_days:
        return 0
    return e_days - c_days


def working_days_between(start_year: int, start_month: int, start_day: int, end_year: int, end_month: int, end_day: int) -> int:
    """Calculate approximate working days between two dates."""
    s: list[int] = make_date(start_year, start_month, start_day)
    e: list[int] = make_date(end_year, end_month, end_day)
    total_days: int = days_between(s, e)
    working: int = (total_days * 5) // 7
    return working


def add_business_days(start_year: int, start_month: int, start_day: int, num_days: int) -> list[int]:
    """Add business days to a date (simplified: calendar_days = num_days * 7/5)."""
    calendar_days: int = (num_days * 7) // 5
    s: list[int] = make_date(start_year, start_month, start_day)
    result: list[int] = add_days_to_date(s, calendar_days)
    return result


def quarter_of_year(month: int) -> int:
    """Get the quarter (1-4) for a given month."""
    if month <= 3:
        return 1
    if month <= 6:
        return 2
    if month <= 9:
        return 3
    return 4


def time_to_minutes(hour: int, minute: int) -> int:
    """Convert time to total minutes."""
    total: int = hour * 60 + minute
    return total


def minutes_to_hour(total_minutes: int) -> int:
    """Convert total minutes back to hour."""
    return total_minutes // 60


def minutes_to_minute(total_minutes: int) -> int:
    """Get the minute part from total minutes."""
    return total_minutes % 60


def date_compare(d1: list[int], d2: list[int]) -> int:
    """Compare two dates: -1 if d1<d2, 0 if equal, 1 if d1>d2."""
    days1: int = date_to_days(d1)
    days2: int = date_to_days(d2)
    if days1 < days2:
        return 0 - 1
    if days1 > days2:
        return 1
    return 0


def format_date_str(y: int, m: int, d: int) -> str:
    """Format date as string Y-M-D."""
    ys: str = str(y)
    ms: str = str(m)
    ds: str = str(d)
    result: str = ys + "-" + ms + "-" + ds
    return result


def format_time_str(h: int, m: int, s: int) -> str:
    """Format time as string H:M:S."""
    hs: str = str(h)
    ms: str = str(m)
    ss: str = str(s)
    result: str = hs + ":" + ms + ":" + ss
    return result


def test_module() -> int:
    """Run all datetime-equivalent tests and count passes."""
    ok: int = 0

    d1: list[int] = make_date(2025, 11, 5)
    if date_year(d1) == 2025:
        ok = ok + 1
    if date_month(d1) == 11:
        ok = ok + 1
    if date_day(d1) == 5:
        ok = ok + 1

    t1: list[int] = make_time(14, 30, 0)
    if time_hour(t1) == 14:
        ok = ok + 1
    if time_minute(t1) == 30:
        ok = ok + 1

    lp1: int = is_leap_year(2024)
    if lp1 == 1:
        ok = ok + 1

    lp2: int = is_leap_year(2025)
    if lp2 == 0:
        ok = ok + 1

    lp3: int = is_leap_year(2000)
    if lp3 == 1:
        ok = ok + 1

    lp4: int = is_leap_year(1900)
    if lp4 == 0:
        ok = ok + 1

    dim1: int = days_in_month(2024, 2)
    if dim1 == 29:
        ok = ok + 1

    dim2: int = days_in_month(2025, 2)
    if dim2 == 28:
        ok = ok + 1

    dim3: int = days_in_month(2025, 1)
    if dim3 == 31:
        ok = ok + 1

    start: list[int] = make_date(2025, 1, 1)
    end: list[int] = make_date(2025, 12, 31)
    diff: int = days_between(start, end)
    if diff == 364:
        ok = ok + 1

    week_later: list[int] = add_days_to_date(d1, 7)
    if date_month(week_later) == 11:
        ok = ok + 1
    if date_day(week_later) == 12:
        ok = ok + 1

    month_ago: list[int] = subtract_days_from_date(d1, 30)
    if date_month(month_ago) == 10:
        ok = ok + 1

    age: int = calculate_age(1990, 5, 15, 2025, 11, 5)
    if age == 35:
        ok = ok + 1

    days_left: int = days_until_event(2025, 12, 31, 2025, 11, 5)
    if days_left == 56:
        ok = ok + 1

    q1: int = quarter_of_year(3)
    if q1 == 1:
        ok = ok + 1

    q2: int = quarter_of_year(6)
    if q2 == 2:
        ok = ok + 1

    q3: int = quarter_of_year(9)
    if q3 == 3:
        ok = ok + 1

    q4: int = quarter_of_year(12)
    if q4 == 4:
        ok = ok + 1

    start_min: int = time_to_minutes(9, 0)
    duration_min: int = 150
    end_min: int = start_min + duration_min
    end_hour: int = minutes_to_hour(end_min)
    if end_hour == 11:
        ok = ok + 1

    cmp1: int = date_compare(start, end)
    if cmp1 == 0 - 1:
        ok = ok + 1

    cmp2: int = date_compare(end, start)
    if cmp2 == 1:
        ok = ok + 1

    ds: str = format_date_str(2025, 11, 5)
    if ds == "2025-11-5":
        ok = ok + 1

    ts: str = format_time_str(14, 30, 0)
    if ts == "14:30:0":
        ok = ok + 1

    work_days: int = working_days_between(2025, 1, 1, 2025, 12, 31)
    if work_days > 200:
        ok = ok + 1

    return ok
