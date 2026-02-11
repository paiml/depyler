"""Calendar operations.

Tests: leap year, days in month, day of year, days between dates.
"""


def is_leap_year_val(year: int) -> int:
    """Check if a year is a leap year. Returns 1 if yes, 0 if no."""
    if year % 400 == 0:
        return 1
    if year % 100 == 0:
        return 0
    if year % 4 == 0:
        return 1
    return 0


def days_in_month(year: int, month: int) -> int:
    """Return number of days in given month of given year."""
    if month == 2:
        if is_leap_year_val(year) == 1:
            return 29
        return 28
    if month == 4 or month == 6 or month == 9 or month == 11:
        return 30
    return 31


def day_of_year(year: int, month: int, day: int) -> int:
    """Calculate day of year (1-365/366) for a given date."""
    total: int = 0
    m: int = 1
    while m < month:
        total = total + days_in_month(year, m)
        m = m + 1
    return total + day


def days_in_year(year: int) -> int:
    """Return total days in a year."""
    if is_leap_year_val(year) == 1:
        return 366
    return 365


def days_between(y1: int, m1: int, d1: int, y2: int, m2: int, d2: int) -> int:
    """Calculate absolute days between two dates."""
    if y1 == y2:
        doy1: int = day_of_year(y1, m1, d1)
        doy2: int = day_of_year(y2, m2, d2)
        diff: int = doy2 - doy1
        if diff < 0:
            diff = -diff
        return diff
    total: int = 0
    ay1: int = y1
    am1: int = m1
    ad1: int = d1
    ay2: int = y2
    am2: int = m2
    ad2: int = d2
    if ay1 > ay2:
        tmp_y: int = ay1
        ay1 = ay2
        ay2 = tmp_y
        tmp_m: int = am1
        am1 = am2
        am2 = tmp_m
        tmp_d: int = ad1
        ad1 = ad2
        ad2 = tmp_d
    remaining1: int = days_in_year(ay1) - day_of_year(ay1, am1, ad1)
    total = remaining1 + day_of_year(ay2, am2, ad2)
    yr: int = ay1 + 1
    while yr < ay2:
        total = total + days_in_year(yr)
        yr = yr + 1
    return total


def test_module() -> None:
    assert is_leap_year_val(2000) == 1
    assert is_leap_year_val(1900) == 0
    assert is_leap_year_val(2024) == 1
    assert is_leap_year_val(2023) == 0
    assert days_in_month(2024, 2) == 29
    assert days_in_month(2023, 2) == 28
    assert days_in_month(2023, 1) == 31
    assert days_in_month(2023, 4) == 30
    assert day_of_year(2023, 1, 1) == 1
    assert day_of_year(2023, 3, 1) == 60
    assert day_of_year(2024, 3, 1) == 61
    assert days_in_year(2024) == 366
    assert days_in_year(2023) == 365
    assert days_between(2023, 1, 1, 2023, 12, 31) == 364
    assert days_between(2023, 1, 1, 2024, 1, 1) == 365
