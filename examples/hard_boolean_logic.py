"""Hard boolean logic patterns for transpiler stress-testing.

Tests: boolean operations (and, or, not), truthiness evaluation,
conditional expressions, short-circuit evaluation, boolean algebra,
predicate composition, and truth table generation.
"""


def bool_and(a: bool, b: bool) -> bool:
    """Logical AND of two booleans."""
    if a:
        if b:
            return True
    return False


def bool_or(a: bool, b: bool) -> bool:
    """Logical OR of two booleans."""
    if a:
        return True
    if b:
        return True
    return False


def bool_not(a: bool) -> bool:
    """Logical NOT of a boolean."""
    if a:
        return False
    return True


def bool_xor(a: bool, b: bool) -> bool:
    """Logical XOR of two booleans."""
    if a and not b:
        return True
    if not a and b:
        return True
    return False


def bool_nand(a: bool, b: bool) -> bool:
    """Logical NAND of two booleans."""
    return not (a and b)


def bool_nor(a: bool, b: bool) -> bool:
    """Logical NOR of two booleans."""
    return not (a or b)


def bool_implies(a: bool, b: bool) -> bool:
    """Logical implication: a implies b (equivalent to not a or b)."""
    if a and not b:
        return False
    return True


def bool_iff(a: bool, b: bool) -> bool:
    """Logical biconditional: a if and only if b."""
    return a == b


def majority_vote(a: bool, b: bool, c: bool) -> bool:
    """Return True if at least 2 of 3 inputs are True."""
    count: int = 0
    if a:
        count += 1
    if b:
        count += 1
    if c:
        count += 1
    return count >= 2


def all_true(values: list[bool]) -> bool:
    """Check if all values in the list are True."""
    for v in values:
        if not v:
            return False
    return True


def any_true(values: list[bool]) -> bool:
    """Check if any value in the list is True."""
    for v in values:
        if v:
            return True
    return False


def none_true(values: list[bool]) -> bool:
    """Check if no values in the list are True."""
    for v in values:
        if v:
            return False
    return True


def count_true(values: list[bool]) -> int:
    """Count the number of True values in a list."""
    count: int = 0
    for v in values:
        if v:
            count += 1
    return count


def conditional_sign(x: int) -> str:
    """Return sign description using conditional expressions."""
    if x > 0:
        return "positive"
    elif x < 0:
        return "negative"
    else:
        return "zero"


def clamp(value: int, low: int, high: int) -> int:
    """Clamp a value between low and high bounds."""
    if value < low:
        return low
    if value > high:
        return high
    return value


def is_in_range(value: int, low: int, high: int) -> bool:
    """Check if value is within [low, high] inclusive."""
    return value >= low and value <= high


def is_leap_year(year: int) -> bool:
    """Determine if a year is a leap year using boolean logic."""
    if year % 400 == 0:
        return True
    if year % 100 == 0:
        return False
    if year % 4 == 0:
        return True
    return False


def fizzbuzz_classify(n: int) -> str:
    """Classify a number for FizzBuzz using boolean conditions."""
    div_by_3: bool = n % 3 == 0
    div_by_5: bool = n % 5 == 0
    if div_by_3 and div_by_5:
        return "fizzbuzz"
    if div_by_3:
        return "fizz"
    if div_by_5:
        return "buzz"
    return str(n)


def chain_comparisons(a: int, b: int, c: int) -> bool:
    """Check if a < b < c using chained comparison logic."""
    return a < b and b < c


def multi_condition_filter(nums: list[int]) -> list[int]:
    """Filter numbers that are positive, even, and less than 100."""
    result: list[int] = []
    for x in nums:
        is_positive: bool = x > 0
        is_even: bool = x % 2 == 0
        is_under_100: bool = x < 100
        if is_positive and is_even and is_under_100:
            result.append(x)
    return result


def short_circuit_safe_divide(a: int, b: int) -> int:
    """Demonstrate short-circuit evaluation with safe division."""
    if b != 0 and a // b > 0:
        return a // b
    return 0


def predicate_count(nums: list[int], check_positive: bool, check_even: bool) -> int:
    """Count numbers matching configurable predicates."""
    count: int = 0
    for x in nums:
        passes: bool = True
        if check_positive and x <= 0:
            passes = False
        if check_even and x % 2 != 0:
            passes = False
        if passes:
            count += 1
    return count


def truth_table_and() -> list[list[bool]]:
    """Generate the truth table for AND operation."""
    table: list[list[bool]] = []
    inputs: list[bool] = [False, True]
    for a in inputs:
        for b in inputs:
            table.append([a, b, a and b])
    return table


def truth_table_or() -> list[list[bool]]:
    """Generate the truth table for OR operation."""
    table: list[list[bool]] = []
    inputs: list[bool] = [False, True]
    for a in inputs:
        for b in inputs:
            table.append([a, b, a or b])
    return table


def evaluate_expression(a: bool, b: bool, c: bool) -> bool:
    """Evaluate a complex boolean expression: (a AND b) OR (NOT a AND c)."""
    return (a and b) or (not a and c)


def demorgans_law_check(a: bool, b: bool) -> bool:
    """Verify De Morgan's law: NOT(a AND b) == (NOT a) OR (NOT b)."""
    lhs: bool = not (a and b)
    rhs: bool = (not a) or (not b)
    return lhs == rhs


def test_all() -> bool:
    """Comprehensive test exercising all boolean logic functions."""
    # Test basic gates
    assert bool_and(True, True) == True
    assert bool_and(True, False) == False
    assert bool_and(False, True) == False
    assert bool_and(False, False) == False

    assert bool_or(True, True) == True
    assert bool_or(True, False) == True
    assert bool_or(False, True) == True
    assert bool_or(False, False) == False

    assert bool_not(True) == False
    assert bool_not(False) == True

    assert bool_xor(True, True) == False
    assert bool_xor(True, False) == True
    assert bool_xor(False, True) == True
    assert bool_xor(False, False) == False

    assert bool_nand(True, True) == False
    assert bool_nand(True, False) == True
    assert bool_nand(False, False) == True

    assert bool_nor(False, False) == True
    assert bool_nor(True, False) == False
    assert bool_nor(False, True) == False

    # Test implication
    assert bool_implies(True, True) == True
    assert bool_implies(True, False) == False
    assert bool_implies(False, True) == True
    assert bool_implies(False, False) == True

    # Test biconditional
    assert bool_iff(True, True) == True
    assert bool_iff(False, False) == True
    assert bool_iff(True, False) == False

    # Test majority_vote
    assert majority_vote(True, True, False) == True
    assert majority_vote(True, False, False) == False
    assert majority_vote(True, True, True) == True
    assert majority_vote(False, False, False) == False

    # Test all_true, any_true, none_true
    assert all_true([True, True, True]) == True
    assert all_true([True, False, True]) == False
    assert all_true([]) == True

    assert any_true([False, False, True]) == True
    assert any_true([False, False, False]) == False
    assert any_true([]) == False

    assert none_true([False, False, False]) == True
    assert none_true([False, True, False]) == False
    assert none_true([]) == True

    # Test count_true
    assert count_true([True, False, True, True, False]) == 3
    assert count_true([]) == 0

    # Test conditional_sign
    assert conditional_sign(5) == "positive"
    assert conditional_sign(-3) == "negative"
    assert conditional_sign(0) == "zero"

    # Test clamp
    assert clamp(5, 0, 10) == 5
    assert clamp(-5, 0, 10) == 0
    assert clamp(15, 0, 10) == 10

    # Test is_in_range
    assert is_in_range(5, 1, 10) == True
    assert is_in_range(0, 1, 10) == False
    assert is_in_range(10, 1, 10) == True

    # Test is_leap_year
    assert is_leap_year(2000) == True
    assert is_leap_year(1900) == False
    assert is_leap_year(2024) == True
    assert is_leap_year(2023) == False

    # Test fizzbuzz_classify
    assert fizzbuzz_classify(15) == "fizzbuzz"
    assert fizzbuzz_classify(9) == "fizz"
    assert fizzbuzz_classify(10) == "buzz"
    assert fizzbuzz_classify(7) == "7"

    # Test chain_comparisons
    assert chain_comparisons(1, 2, 3) == True
    assert chain_comparisons(1, 3, 2) == False
    assert chain_comparisons(2, 2, 3) == False

    # Test multi_condition_filter
    filtered: list[int] = multi_condition_filter([-2, 0, 3, 4, 50, 101, 200, 8])
    assert filtered == [4, 50, 8]

    # Test short_circuit_safe_divide
    assert short_circuit_safe_divide(10, 3) == 3
    assert short_circuit_safe_divide(10, 0) == 0
    assert short_circuit_safe_divide(-5, 2) == 0

    # Test predicate_count
    nums: list[int] = [-3, -2, -1, 0, 1, 2, 3, 4, 5, 6]
    assert predicate_count(nums, True, False) == 6  # positive: 1,2,3,4,5,6
    assert predicate_count(nums, True, True) == 3   # positive even: 2,4,6
    assert predicate_count(nums, False, True) == 5   # even: -2,0,2,4,6

    # Test truth_table_and
    and_table: list[list[bool]] = truth_table_and()
    assert len(and_table) == 4
    assert and_table[0] == [False, False, False]
    assert and_table[3] == [True, True, True]

    # Test truth_table_or
    or_table: list[list[bool]] = truth_table_or()
    assert len(or_table) == 4
    assert or_table[0] == [False, False, False]
    assert or_table[1] == [False, True, True]

    # Test evaluate_expression
    assert evaluate_expression(True, True, False) == True   # (T AND T) OR (F AND F) = T
    assert evaluate_expression(False, True, True) == True    # (F AND T) OR (T AND T) = T
    assert evaluate_expression(False, True, False) == False  # (F AND T) OR (T AND F) = F

    # Test De Morgan's law holds for all combinations
    for a in [True, False]:
        for b in [True, False]:
            assert demorgans_law_check(a, b) == True

    return True


def main() -> None:
    """Run all tests and report results."""
    result: bool = test_all()
    if result:
        print("All boolean logic tests passed!")


if __name__ == "__main__":
    main()
