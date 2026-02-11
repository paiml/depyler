# Pathological control flow: Complex boolean expression evaluation
# Tests: chained and/or, nested conditions, truth table evaluation


def eval_complex_bool(a: bool, b: bool, c: bool, d: bool) -> int:
    """Evaluate complex boolean expression and return category."""
    if a == True and b == True and c == True and d == True:
        return 1
    if a == True and b == True and c == False and d == False:
        return 2
    if a == False and b == False and c == True and d == True:
        return 3
    if (a == True or b == True) and (c == True or d == True):
        return 4
    if a == True or b == True:
        return 5
    if c == True or d == True:
        return 6
    return 7


def count_truth_combos(vals: list[int]) -> int:
    """Count how many adjacent pairs are both positive (both 'true')."""
    count: int = 0
    i: int = 0
    while i + 1 < len(vals):
        if vals[i] > 0 and vals[i + 1] > 0:
            count = count + 1
        i = i + 1
    return count


def evaluate_logic_circuit(inputs: list[int]) -> int:
    """Simulate a 4-input logic circuit with AND/OR/NOT gates.
    inputs: [a, b, c, d] where 0=false, nonzero=true
    Circuit: output = ((a AND b) OR (NOT c AND d)) AND (a OR d)
    Returns 1 for true, 0 for false."""
    if len(inputs) < 4:
        return 0 - 1
    a_val: bool = inputs[0] != 0
    b_val: bool = inputs[1] != 0
    c_val: bool = inputs[2] != 0
    d_val: bool = inputs[3] != 0
    # Gate 1: a AND b
    gate1: bool = a_val == True and b_val == True
    # Gate 2: NOT c
    not_c: bool = c_val == False
    # Gate 3: NOT c AND d
    gate3: bool = not_c == True and d_val == True
    # Gate 4: gate1 OR gate3
    gate4: bool = gate1 == True or gate3 == True
    # Gate 5: a OR d
    gate5: bool = a_val == True or d_val == True
    # Final: gate4 AND gate5
    result: bool = gate4 == True and gate5 == True
    if result == True:
        return 1
    return 0


def multi_condition_filter(nums: list[int]) -> list[int]:
    """Filter numbers through multiple boolean conditions."""
    result: list[int] = []
    i: int = 0
    while i < len(nums):
        val: int = nums[i]
        is_positive: bool = val > 0
        is_even: bool = val % 2 == 0
        is_small: bool = val < 100
        is_not_divisible_by_3: bool = val % 3 != 0
        # Accept if: positive AND (even OR small) AND not div by 3
        if is_positive == True:
            if is_even == True or is_small == True:
                if is_not_divisible_by_3 == True:
                    result.append(val)
        i = i + 1
    return result


def boolean_majority(a: bool, b: bool, c: bool, d: bool, e: bool) -> bool:
    """Return true if majority (3+) of 5 bools are true."""
    count: int = 0
    if a == True:
        count = count + 1
    if b == True:
        count = count + 1
    if c == True:
        count = count + 1
    if d == True:
        count = count + 1
    if e == True:
        count = count + 1
    return count >= 3


def test_module() -> int:
    passed: int = 0
    # Test 1: all true
    if eval_complex_bool(True, True, True, True) == 1:
        passed = passed + 1
    # Test 2: mixed
    if eval_complex_bool(True, True, False, False) == 2:
        passed = passed + 1
    # Test 3: all false
    if eval_complex_bool(False, False, False, False) == 7:
        passed = passed + 1
    # Test 4: truth combos
    if count_truth_combos([1, 2, 0, 3, 4]) == 2:
        passed = passed + 1
    # Test 5: logic circuit (a=1, b=1, c=0, d=1) -> (1 AND 1) OR (NOT 0 AND 1) AND (1 OR 1) = 1
    if evaluate_logic_circuit([1, 1, 0, 1]) == 1:
        passed = passed + 1
    # Test 6: filter
    filtered: list[int] = multi_condition_filter([1, 2, 4, 6, 7, 8, 9, 10, 50])
    if len(filtered) == 5:
        passed = passed + 1
    # Test 7: majority
    if boolean_majority(True, True, True, False, False) == True:
        passed = passed + 1
    # Test 8: minority
    if boolean_majority(True, False, False, False, False) == False:
        passed = passed + 1
    return passed
