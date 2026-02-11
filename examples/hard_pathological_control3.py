# Pathological control flow: Functions with 10+ early returns
# Tests: extensive validation and classification with many return paths


def validate_and_classify(val: int, mode: int, threshold: int) -> int:
    """Validate and classify with 12 possible return paths."""
    if val < 0:
        return 0 - 1
    if mode < 0:
        return 0 - 2
    if threshold < 0:
        return 0 - 3
    if val == 0:
        return 0
    if mode == 0:
        if val < threshold:
            return 1
        if val == threshold:
            return 2
        if val < threshold * 2:
            return 3
        return 4
    if mode == 1:
        if val % 2 == 0:
            return 5
        return 6
    if mode == 2:
        if val > threshold:
            return 7
        return 8
    return 9


def categorize_triangle(a: int, b: int, c: int) -> int:
    """Categorize triangle by sides. Returns category code."""
    if a <= 0:
        return 0 - 1
    if b <= 0:
        return 0 - 1
    if c <= 0:
        return 0 - 1
    if a + b <= c:
        return 0
    if a + c <= b:
        return 0
    if b + c <= a:
        return 0
    if a == b and b == c:
        return 1  # equilateral
    if a == b:
        return 2  # isosceles
    if b == c:
        return 2
    if a == c:
        return 2
    return 3  # scalene


def grade_exam(score: int, bonus: int, penalty: int, curved: bool) -> str:
    """Grade with many early returns based on conditions."""
    if score < 0:
        return "invalid"
    if score > 100:
        return "invalid"
    adjusted: int = score + bonus - penalty
    if curved == True:
        adjusted = adjusted + 5
    if adjusted >= 97:
        return "A+"
    if adjusted >= 93:
        return "A"
    if adjusted >= 90:
        return "A-"
    if adjusted >= 87:
        return "B+"
    if adjusted >= 83:
        return "B"
    if adjusted >= 80:
        return "B-"
    if adjusted >= 70:
        return "C"
    if adjusted >= 60:
        return "D"
    return "F"


def test_module() -> int:
    passed: int = 0
    # Test 1: validate basic
    if validate_and_classify(5, 0, 10) == 1:
        passed = passed + 1
    # Test 2: negative val
    if validate_and_classify(0 - 1, 0, 10) == 0 - 1:
        passed = passed + 1
    # Test 3: mode 1 even
    if validate_and_classify(4, 1, 10) == 5:
        passed = passed + 1
    # Test 4: equilateral
    if categorize_triangle(5, 5, 5) == 1:
        passed = passed + 1
    # Test 5: scalene
    if categorize_triangle(3, 4, 5) == 3:
        passed = passed + 1
    # Test 6: invalid triangle
    if categorize_triangle(1, 2, 10) == 0:
        passed = passed + 1
    # Test 7: grade A+
    if grade_exam(95, 3, 0, False) == "A+":
        passed = passed + 1
    # Test 8: grade with curve
    if grade_exam(85, 0, 0, True) == "A-":
        passed = passed + 1
    return passed
