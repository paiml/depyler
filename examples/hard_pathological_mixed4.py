# Pathological mixed: Mixed arithmetic and string operations
# Tests: complex interplay between numeric computation and string formatting


def describe_comparison(a: int, b: int) -> str:
    """Return string describing relationship between a and b."""
    diff: int = a - b
    if diff > 0:
        return str(a) + " is greater than " + str(b) + " by " + str(diff)
    if diff < 0:
        abs_diff: int = 0 - diff
        return str(a) + " is less than " + str(b) + " by " + str(abs_diff)
    return str(a) + " equals " + str(b)


def fizzbuzz_str(n: int) -> str:
    """Classic fizzbuzz returning string result."""
    if n % 15 == 0:
        return "fizzbuzz"
    if n % 3 == 0:
        return "fizz"
    if n % 5 == 0:
        return "buzz"
    return str(n)


def build_report(values: list[int]) -> str:
    """Build a text report of statistics."""
    if len(values) == 0:
        return "empty"
    total: int = 0
    min_val: int = values[0]
    max_val: int = values[0]
    i: int = 0
    while i < len(values):
        total = total + values[i]
        if values[i] < min_val:
            min_val = values[i]
        if values[i] > max_val:
            max_val = values[i]
        i = i + 1
    avg: int = total // len(values)
    report: str = "count=" + str(len(values))
    report = report + " sum=" + str(total)
    report = report + " avg=" + str(avg)
    report = report + " min=" + str(min_val)
    report = report + " max=" + str(max_val)
    return report


def encode_run_length(text: str) -> str:
    """Run-length encode a string: 'aaabbc' -> 'a3b2c1'."""
    if len(text) == 0:
        return ""
    result: str = ""
    current: str = text[0]
    count: int = 1
    i: int = 1
    while i < len(text):
        c: str = text[i]
        if c == current:
            count = count + 1
        else:
            result = result + current + str(count)
            current = c
            count = 1
        i = i + 1
    result = result + current + str(count)
    return result


def roman_to_int(s: str) -> int:
    """Convert Roman numeral string to integer (supports I,V,X,L,C,D,M)."""
    result: int = 0
    i: int = 0
    while i < len(s):
        c: str = s[i]
        val: int = 0
        if c == "I":
            val = 1
        elif c == "V":
            val = 5
        elif c == "X":
            val = 10
        elif c == "L":
            val = 50
        elif c == "C":
            val = 100
        elif c == "D":
            val = 500
        elif c == "M":
            val = 1000
        # Check for subtractive
        if i + 1 < len(s):
            next_c: str = s[i + 1]
            next_val: int = 0
            if next_c == "I":
                next_val = 1
            elif next_c == "V":
                next_val = 5
            elif next_c == "X":
                next_val = 10
            elif next_c == "L":
                next_val = 50
            elif next_c == "C":
                next_val = 100
            elif next_c == "D":
                next_val = 500
            elif next_c == "M":
                next_val = 1000
            if next_val > val:
                result = result + next_val - val
                i = i + 2
                continue
        result = result + val
        i = i + 1
    return result


def test_module() -> int:
    passed: int = 0
    # Test 1: describe comparison
    desc: str = describe_comparison(10, 7)
    if desc == "10 is greater than 7 by 3":
        passed = passed + 1
    # Test 2: fizzbuzz
    if fizzbuzz_str(15) == "fizzbuzz":
        passed = passed + 1
    # Test 3: fizzbuzz number
    if fizzbuzz_str(7) == "7":
        passed = passed + 1
    # Test 4: build report
    report: str = build_report([10, 20, 30])
    if report == "count=3 sum=60 avg=20 min=10 max=30":
        passed = passed + 1
    # Test 5: run length encode
    if encode_run_length("aaabbc") == "a3b2c1":
        passed = passed + 1
    # Test 6: roman XIV = 14
    if roman_to_int("XIV") == 14:
        passed = passed + 1
    # Test 7: roman MCMXC = 1990
    if roman_to_int("MCMXC") == 1990:
        passed = passed + 1
    return passed
