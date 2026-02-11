# Pathological mixed: Parsing numeric strings
# Tests: string to int conversion, validation, multi-format parsing


def is_digit_char(c: str) -> bool:
    """Check if character is a digit.
    Workaround: avoid returning chained comparison directly (generates .as_str()
    which is unstable). Use explicit if/else instead."""
    if c == "0" or c == "1" or c == "2" or c == "3" or c == "4":
        return True
    if c == "5" or c == "6" or c == "7" or c == "8" or c == "9":
        return True
    return False


def is_valid_integer_str(s: str) -> bool:
    """Check if string represents a valid integer."""
    if len(s) == 0:
        return False
    start: int = 0
    c0: str = s[0]
    if c0 == "-" or c0 == "+":
        start = 1
    if start >= len(s):
        return False
    i: int = start
    while i < len(s):
        c: str = s[i]
        if is_digit_char(c) == False:
            return False
        i = i + 1
    return True


def parse_list_of_ints(text: str) -> list[int]:
    """Parse comma-separated integers from string."""
    result: list[int] = []
    current: str = ""
    i: int = 0
    while i < len(text):
        c: str = text[i]
        if c == ",":
            if len(current) > 0:
                if is_valid_integer_str(current) == True:
                    result.append(int(current))
                current = ""
        elif c != " ":
            current = current + c
        i = i + 1
    if len(current) > 0:
        if is_valid_integer_str(current) == True:
            result.append(int(current))
    return result


def extract_numbers_from_text(text: str) -> list[int]:
    """Extract all integer numbers embedded in text."""
    result: list[int] = []
    current: str = ""
    i: int = 0
    while i < len(text):
        c: str = text[i]
        if is_digit_char(c) == True:
            current = current + c
        else:
            if len(current) > 0:
                result.append(int(current))
                current = ""
        i = i + 1
    if len(current) > 0:
        result.append(int(current))
    return result


def count_numeric_tokens(text: str) -> int:
    """Count how many space-separated tokens are valid integers."""
    count: int = 0
    current: str = ""
    i: int = 0
    while i < len(text):
        c: str = text[i]
        if c == " ":
            if len(current) > 0:
                if is_valid_integer_str(current) == True:
                    count = count + 1
                current = ""
        else:
            current = current + c
        i = i + 1
    if len(current) > 0:
        if is_valid_integer_str(current) == True:
            count = count + 1
    return count


def sum_embedded_numbers(text: str) -> int:
    """Sum all numbers found in text."""
    nums: list[int] = extract_numbers_from_text(text)
    total: int = 0
    i: int = 0
    while i < len(nums):
        total = total + nums[i]
        i = i + 1
    return total


def test_module() -> int:
    passed: int = 0
    # Test 1: valid integer string
    if is_valid_integer_str("123") == True:
        passed = passed + 1
    # Test 2: invalid
    if is_valid_integer_str("12.3") == False:
        passed = passed + 1
    # Test 3: parse list
    nums: list[int] = parse_list_of_ints("1, 2, 3, 4")
    if len(nums) == 4 and nums[0] == 1 and nums[3] == 4:
        passed = passed + 1
    # Test 4: extract numbers
    extracted: list[int] = extract_numbers_from_text("there are 3 cats and 12 dogs")
    if len(extracted) == 2:
        passed = passed + 1
    # Test 5: count numeric tokens
    if count_numeric_tokens("hello 42 world 7 foo") == 2:
        passed = passed + 1
    # Test 6: sum embedded
    if sum_embedded_numbers("a1b2c3") == 6:
        passed = passed + 1
    # Test 7: negative check
    if is_valid_integer_str("-42") == True:
        passed = passed + 1
    return passed
