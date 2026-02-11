"""Long if/elif/else chains with 10+ branches."""


def day_of_week(n: int) -> str:
    """Map number 0-6 to day name."""
    if n == 0:
        return "monday"
    elif n == 1:
        return "tuesday"
    elif n == 2:
        return "wednesday"
    elif n == 3:
        return "thursday"
    elif n == 4:
        return "friday"
    elif n == 5:
        return "saturday"
    elif n == 6:
        return "sunday"
    else:
        return "unknown"


def month_days(month: int, is_leap: int) -> int:
    """Return days in month (1-12). is_leap: 1 for leap year."""
    if month == 1:
        return 31
    elif month == 2:
        if is_leap == 1:
            return 29
        else:
            return 28
    elif month == 3:
        return 31
    elif month == 4:
        return 30
    elif month == 5:
        return 31
    elif month == 6:
        return 30
    elif month == 7:
        return 31
    elif month == 8:
        return 31
    elif month == 9:
        return 30
    elif month == 10:
        return 31
    elif month == 11:
        return 30
    elif month == 12:
        return 31
    else:
        return 0


def grade_score(score: int) -> str:
    """Convert numeric score to letter grade."""
    if score >= 97:
        return "A+"
    elif score >= 93:
        return "A"
    elif score >= 90:
        return "A-"
    elif score >= 87:
        return "B+"
    elif score >= 83:
        return "B"
    elif score >= 80:
        return "B-"
    elif score >= 77:
        return "C+"
    elif score >= 73:
        return "C"
    elif score >= 70:
        return "C-"
    elif score >= 67:
        return "D+"
    elif score >= 63:
        return "D"
    elif score >= 60:
        return "D-"
    else:
        return "F"


def roman_digit(n: int) -> str:
    """Convert single digit (0-9) to partial Roman numeral using chain."""
    if n == 9:
        return "IX"
    elif n == 8:
        return "VIII"
    elif n == 7:
        return "VII"
    elif n == 6:
        return "VI"
    elif n == 5:
        return "V"
    elif n == 4:
        return "IV"
    elif n == 3:
        return "III"
    elif n == 2:
        return "II"
    elif n == 1:
        return "I"
    else:
        return ""


def fizzbuzz_classify(n: int) -> int:
    """FizzBuzz classification: 3=fizz, 5=buzz, 15=fizzbuzz, else=n."""
    if n % 15 == 0:
        return 15
    elif n % 3 == 0:
        return 3
    elif n % 5 == 0:
        return 5
    else:
        return n


def multi_range_classify(val: int) -> int:
    """Classify value into one of 12 ranges."""
    if val < 0 - 100:
        return -6
    elif val < 0 - 50:
        return -5
    elif val < 0 - 20:
        return -4
    elif val < 0 - 10:
        return -3
    elif val < 0 - 5:
        return -2
    elif val < 0:
        return -1
    elif val == 0:
        return 0
    elif val <= 5:
        return 1
    elif val <= 10:
        return 2
    elif val <= 20:
        return 3
    elif val <= 50:
        return 4
    elif val <= 100:
        return 5
    else:
        return 6


def test_module() -> int:
    """Test all conditional chain functions."""
    passed: int = 0
    if day_of_week(0) == "monday":
        passed = passed + 1
    if day_of_week(6) == "sunday":
        passed = passed + 1
    if day_of_week(99) == "unknown":
        passed = passed + 1
    if month_days(2, 1) == 29:
        passed = passed + 1
    if month_days(2, 0) == 28:
        passed = passed + 1
    if month_days(7, 0) == 31:
        passed = passed + 1
    if grade_score(95) == "A":
        passed = passed + 1
    if grade_score(50) == "F":
        passed = passed + 1
    if roman_digit(9) == "IX":
        passed = passed + 1
    if roman_digit(4) == "IV":
        passed = passed + 1
    if fizzbuzz_classify(15) == 15:
        passed = passed + 1
    if fizzbuzz_classify(7) == 7:
        passed = passed + 1
    if multi_range_classify(0) == 0:
        passed = passed + 1
    if multi_range_classify(0 - 200) == -6:
        passed = passed + 1
    if multi_range_classify(200) == 6:
        passed = passed + 1
    return passed


if __name__ == "__main__":
    print(test_module())
