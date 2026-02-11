"""Calendar computations: day of week, leap year, date difference, day number.

Tests: is_leap_year, days_in_month, day_of_year, day_of_week, date_diff.
"""


def is_leap_year(year: int) -> int:
    """Returns 1 if leap year, 0 otherwise."""
    if year % 400 == 0:
        return 1
    if year % 100 == 0:
        return 0
    if year % 4 == 0:
        return 1
    return 0


def days_in_month(month: int, year: int) -> int:
    """Days in given month (1-12) of given year."""
    if month == 2:
        if is_leap_year(year) == 1:
            return 29
        return 28
    if month == 4:
        return 30
    if month == 6:
        return 30
    if month == 9:
        return 30
    if month == 11:
        return 30
    return 31


def day_of_year(day: int, month: int, year: int) -> int:
    """Day number within the year (1-366)."""
    total: int = 0
    m: int = 1
    while m < month:
        total = total + days_in_month(m, year)
        m = m + 1
    total = total + day
    return total


def days_from_epoch(day: int, month: int, year: int) -> int:
    """Count days from year 1 (simplified). For difference calculations."""
    total: int = 0
    y: int = 1
    while y < year:
        if is_leap_year(y) == 1:
            total = total + 366
        else:
            total = total + 365
        y = y + 1
    total = total + day_of_year(day, month, year)
    return total


def date_diff_days(d1: int, m1: int, y1: int, d2: int, m2: int, y2: int) -> int:
    """Absolute difference in days between two dates."""
    days1: int = days_from_epoch(d1, m1, y1)
    days2: int = days_from_epoch(d2, m2, y2)
    diff: int = days1 - days2
    if diff < 0:
        diff = 0 - diff
    return diff


def day_of_week_zeller(day: int, month: int, year: int) -> int:
    """Day of week using Zeller's congruence. 0=Saturday, 1=Sunday, ..., 6=Friday."""
    m: int = month
    y: int = year
    if m < 3:
        m = m + 12
        y = y - 1
    k: int = y % 100
    j: int = y // 100
    h: int = (day + (13 * (m + 1)) // 5 + k + k // 4 + j // 4 + 5 * j) % 7
    return h


def days_in_year(year: int) -> int:
    """Number of days in a year."""
    if is_leap_year(year) == 1:
        return 366
    return 365


def is_valid_date(day: int, month: int, year: int) -> int:
    """Check if date is valid. Returns 1 or 0."""
    if year < 1:
        return 0
    if month < 1:
        return 0
    if month > 12:
        return 0
    if day < 1:
        return 0
    max_day: int = days_in_month(month, year)
    if day > max_day:
        return 0
    return 1


def test_module() -> int:
    """Test calendar computations."""
    passed: int = 0

    if is_leap_year(2000) == 1:
        passed = passed + 1

    if is_leap_year(1900) == 0:
        passed = passed + 1

    if is_leap_year(2024) == 1:
        passed = passed + 1

    if days_in_month(2, 2024) == 29:
        passed = passed + 1

    if day_of_year(1, 3, 2024) == 61:
        passed = passed + 1

    if is_valid_date(31, 2, 2024) == 0:
        passed = passed + 1

    if is_valid_date(29, 2, 2024) == 1:
        passed = passed + 1

    diff: int = date_diff_days(1, 1, 2024, 1, 1, 2023)
    if diff == 366:
        passed = passed + 1

    return passed
