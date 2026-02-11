def is_leap_year(year: int) -> int:
    if year % 400 == 0:
        return 1
    if year % 100 == 0:
        return 0
    if year % 4 == 0:
        return 1
    return 0


def days_in_month(month: int, year: int) -> int:
    if month == 2:
        if is_leap_year(year) == 1:
            return 29
        return 28
    if month == 4 or month == 6 or month == 9 or month == 11:
        return 30
    return 31


def day_of_year(day: int, month: int, year: int) -> int:
    total: int = 0
    m: int = 1
    while m < month:
        total = total + days_in_month(m, year)
        m = m + 1
    total = total + day
    return total


def days_in_year(year: int) -> int:
    if is_leap_year(year) == 1:
        return 366
    return 365


def days_between_dates(y1: int, m1: int, d1: int, y2: int, m2: int, d2: int) -> int:
    total: int = 0
    cy: int = y1
    cm: int = m1
    cd: int = d1
    while cy < y2 or (cy == y2 and (cm < m2 or (cm == m2 and cd < d2))):
        cd = cd + 1
        if cd > days_in_month(cm, cy):
            cd = 1
            cm = cm + 1
            if cm > 12:
                cm = 1
                cy = cy + 1
        total = total + 1
    return total


def test_module() -> int:
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
    if day_of_year(31, 12, 2023) == 365:
        passed = passed + 1
    if days_in_year(2024) == 366:
        passed = passed + 1
    if days_between_dates(2024, 1, 1, 2024, 1, 10) == 9:
        passed = passed + 1
    return passed
