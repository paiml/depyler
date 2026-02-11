"""
Example demonstrating patterns that trigger migration suggestions.
Rewritten with full type annotations and transpiler-compatible patterns.
"""


def accumulator_pattern(items: list[int]) -> list[int]:
    """Pattern: accumulator - should suggest iterator methods."""
    result: list[int] = []
    for item in items:
        if item > 0:
            doubled: int = item * 2
            result.append(doubled)
    return result


def validate_positive(value: int) -> bool:
    """Validate that value is positive."""
    return value > 0


def process_data_double(value: int) -> int:
    """Process data by doubling."""
    return value * 2


def error_with_fallback(value: int) -> int:
    """Pattern: error fallback - returns -1 on invalid input."""
    if not validate_positive(value):
        return -1
    processed: int = process_data_double(value)
    if processed == 0:
        return -1
    return processed


def mutating_parameter(data: list[int]) -> list[int]:
    """Pattern: mutating parameters - should suggest ownership patterns."""
    data.append(42)
    result: list[int] = []
    for i in range(len(data)):
        result.append(data[i])
    # Simple sort via bubble sort
    n: int = len(result)
    for i in range(n):
        for j in range(0, n - i - 1):
            if result[j] > result[j + 1]:
                tmp: int = result[j]
                result[j] = result[j + 1]
                result[j + 1] = tmp
    return result


def type_check_str(value: str) -> str:
    """Process string value by uppercasing."""
    result: str = value.upper()
    return result


def type_check_int(value: int) -> int:
    """Process int value by doubling."""
    return value * 2


def inefficient_string_building(items: list[str]) -> str:
    """Pattern: string concatenation - should suggest efficient methods."""
    result: str = ""
    for item in items:
        result = result + item + ","
    return result


def enumerate_pattern(items: list[str]) -> list[str]:
    """Pattern: range(len()) - should suggest enumerate."""
    result: list[str] = []
    for i in range(len(items)):
        entry: str = str(i) + ":" + items[i]
        result.append(entry)
    return result


def filter_map_pattern(data: list[int]) -> list[int]:
    """Pattern: filter + map in loop - should suggest filter_map."""
    output: list[int] = []
    for x in data:
        if x > 0:
            sq: int = x * x
            output.append(sq)
    return output


def while_true_pattern() -> int:
    """Pattern: while True - should suggest loop."""
    counter: int = 0
    running: bool = True
    while running:
        counter += 1
        if counter > 10:
            running = False
    return counter


def default_value_fn() -> int:
    """Return a default integer value."""
    return 0


def process_value(x: int) -> int:
    """Process a value by returning it unchanged."""
    return x


def none_checking_pattern(optional_value: int) -> int:
    """Pattern: check for sentinel value -1 instead of None."""
    if optional_value != -1:
        return process_value(optional_value)
    else:
        return default_value_fn()


def test_module() -> int:
    """Run all migration pattern tests and return count of passed tests."""
    passed: int = 0

    # Test accumulator_pattern
    acc_result: list[int] = accumulator_pattern([1, -2, 3, -4, 5])
    if len(acc_result) == 3:
        passed += 1
    if acc_result[0] == 2:
        passed += 1
    if acc_result[1] == 6:
        passed += 1

    # Test validate_positive
    if validate_positive(5):
        passed += 1
    if not validate_positive(-3):
        passed += 1

    # Test process_data_double
    if process_data_double(7) == 14:
        passed += 1

    # Test error_with_fallback
    if error_with_fallback(5) == 10:
        passed += 1
    if error_with_fallback(-3) == -1:
        passed += 1

    # Test mutating_parameter
    mut_data: list[int] = [3, 1, 2]
    sorted_data: list[int] = mutating_parameter(mut_data)
    if sorted_data[0] == 1:
        passed += 1

    # Test type_check_str
    if type_check_str("hello") == "HELLO":
        passed += 1

    # Test type_check_int
    if type_check_int(5) == 10:
        passed += 1

    # Test inefficient_string_building
    built: str = inefficient_string_building(["a", "b", "c"])
    if len(built) > 0:
        passed += 1

    # Test enumerate_pattern
    enum_result: list[str] = enumerate_pattern(["x", "y", "z"])
    if len(enum_result) == 3:
        passed += 1

    # Test filter_map_pattern
    fm_result: list[int] = filter_map_pattern([-1, 2, -3, 4])
    if len(fm_result) == 2:
        passed += 1
    if fm_result[0] == 4:
        passed += 1

    # Test while_true_pattern
    if while_true_pattern() == 11:
        passed += 1

    # Test none_checking_pattern
    if none_checking_pattern(42) == 42:
        passed += 1
    if none_checking_pattern(-1) == 0:
        passed += 1

    # Test default_value_fn
    if default_value_fn() == 0:
        passed += 1

    # Test process_value
    if process_value(99) == 99:
        passed += 1

    return passed


if __name__ == "__main__":
    result: int = test_module()
    print("PASSED: " + str(result))
