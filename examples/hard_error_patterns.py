"""Hard error/edge case patterns for depyler transpiler stress testing.
Tests defensive programming, boundary values, and error propagation."""

from typing import List, Dict, Tuple


# --- Division by zero guards ---

def safe_divide(a: int, b: int) -> int:
    """Division with zero guard returning sentinel -1."""
    if b == 0:
        return -1
    return a // b


def safe_divide_with_default(a: int, b: int, default: int) -> int:
    """Division with caller-supplied default on zero divisor."""
    if b == 0:
        return default
    return a // b


def chained_division(a: int, b: int, c: int) -> int:
    """Chained division: a / b / c with guards at each step."""
    if b == 0:
        return -1
    intermediate: int = a // b
    if c == 0:
        return -1
    return intermediate // c


# --- Integer overflow simulation and wrapping ---

def wrapping_add(a: int, b: int, max_val: int) -> int:
    """Simulate wrapping addition within [0, max_val]."""
    result: int = a + b
    if result > max_val:
        result = result - max_val - 1
    if result < 0:
        result = result + max_val + 1
    return result


def saturating_add(a: int, b: int, max_val: int) -> int:
    """Saturating addition: clamp at max_val."""
    result: int = a + b
    if result > max_val:
        return max_val
    if result < 0:
        return 0
    return result


def saturating_sub(a: int, b: int) -> int:
    """Saturating subtraction: floor at zero."""
    result: int = a - b
    if result < 0:
        return 0
    return result


def saturating_mul(a: int, b: int, max_val: int) -> int:
    """Saturating multiplication clamped to max_val."""
    result: int = a * b
    if result > max_val:
        return max_val
    if result < 0:
        return 0
    return result


# --- Boundary value computations ---

def clamp(value: int, lo: int, hi: int) -> int:
    """Clamp value to [lo, hi] range."""
    if value < lo:
        return lo
    if value > hi:
        return hi
    return value


def abs_diff(a: int, b: int) -> int:
    """Absolute difference without overflow risk."""
    if a > b:
        return a - b
    return b - a


def midpoint_safe(a: int, b: int) -> int:
    """Overflow-safe midpoint calculation."""
    if a > b:
        return b + (a - b) // 2
    return a + (b - a) // 2


def boundary_classify(value: int, max_val: int) -> int:
    """Classify value: 0=zero, 1=min boundary, 2=max boundary, 3=interior."""
    if value == 0:
        return 0
    if value == 1:
        return 1
    if value == max_val:
        return 2
    return 3


# --- Recursive depth limiting ---

def limited_factorial(n: int, max_depth: int) -> int:
    """Factorial with depth limit, returns -1 on exceeded depth."""
    if max_depth <= 0:
        return -1
    if n <= 1:
        return 1
    sub: int = limited_factorial(n - 1, max_depth - 1)
    if sub == -1:
        return -1
    return n * sub


def limited_fibonacci(n: int, max_depth: int) -> int:
    """Fibonacci with depth limit."""
    if max_depth <= 0:
        return -1
    if n <= 0:
        return 0
    if n == 1:
        return 1
    a: int = limited_fibonacci(n - 1, max_depth - 1)
    if a == -1:
        return -1
    b: int = limited_fibonacci(n - 2, max_depth - 1)
    if b == -1:
        return -1
    return a + b


def bounded_power(base: int, exp: int, limit: int) -> int:
    """Iterative power with result limit. Returns -1 if exceeded."""
    result: int = 1
    i: int = 0
    while i < exp:
        result = result * base
        if result > limit:
            return -1
        i = i + 1
    return result


# --- Empty collection guards ---

def safe_first(lst: List[int]) -> int:
    """Return first element or -1 for empty list."""
    if len(lst) == 0:
        return -1
    return lst[0]


def safe_last(lst: List[int]) -> int:
    """Return last element or -1 for empty list."""
    if len(lst) == 0:
        return -1
    return lst[len(lst) - 1]


def safe_sum(lst: List[int]) -> int:
    """Sum with empty list guard."""
    if len(lst) == 0:
        return 0
    total: int = 0
    for x in lst:
        total = total + x
    return total


def safe_max(lst: List[int]) -> int:
    """Max with empty list guard, returns sentinel -999999."""
    if len(lst) == 0:
        return -999999
    best: int = lst[0]
    for x in lst:
        if x > best:
            best = x
    return best


def safe_min(lst: List[int]) -> int:
    """Min with empty list guard, returns sentinel 999999."""
    if len(lst) == 0:
        return 999999
    best: int = lst[0]
    for x in lst:
        if x < best:
            best = x
    return best


def safe_average_int(lst: List[int]) -> int:
    """Integer average with empty guard."""
    if len(lst) == 0:
        return 0
    total: int = 0
    for x in lst:
        total = total + x
    return total // len(lst)


# --- Index bounds checking ---

def safe_index(lst: List[int], idx: int) -> int:
    """Bounds-checked index access, returns -1 on out of bounds."""
    if idx < 0:
        return -1
    if idx >= len(lst):
        return -1
    return lst[idx]


def safe_set(lst: List[int], idx: int, val: int) -> int:
    """Returns 1 if set succeeded, 0 if out of bounds."""
    if idx < 0:
        return 0
    if idx >= len(lst):
        return 0
    lst[idx] = val
    return 1


def find_or_default(lst: List[int], target: int, default: int) -> int:
    """Linear search returning index or default."""
    i: int = 0
    while i < len(lst):
        if lst[i] == target:
            return i
        i = i + 1
    return default


# --- Sentinel value propagation ---

def propagate_sentinel(a: int, b: int) -> int:
    """If either input is sentinel -1, propagate it."""
    if a == -1:
        return -1
    if b == -1:
        return -1
    return a + b


def chain_sentinel_ops(a: int, b: int, c: int) -> int:
    """Chain operations, propagating -1 sentinel at each step."""
    step1: int = propagate_sentinel(a, b)
    if step1 == -1:
        return -1
    step2: int = propagate_sentinel(step1, c)
    return step2


def sentinel_map(lst: List[int]) -> int:
    """Sum list but propagate sentinel -1 from any element."""
    total: int = 0
    for x in lst:
        if x == -1:
            return -1
        total = total + x
    return total


# --- Multiple return path functions ---

def classify_triangle(a: int, b: int, c: int) -> int:
    """0=invalid, 1=equilateral, 2=isosceles, 3=scalene."""
    if a <= 0 or b <= 0 or c <= 0:
        return 0
    if a + b <= c or a + c <= b or b + c <= a:
        return 0
    if a == b and b == c:
        return 1
    if a == b or b == c or a == c:
        return 2
    return 3


def multi_range_classify(x: int) -> int:
    """Classify into ranges: 0=negative, 1=[0,10), 2=[10,100), 3=[100,1000), 4=large."""
    if x < 0:
        return 0
    if x < 10:
        return 1
    if x < 100:
        return 2
    if x < 1000:
        return 3
    return 4


def grade_score(score: int) -> int:
    """Map score to grade: 5=A, 4=B, 3=C, 2=D, 1=F, 0=invalid."""
    if score < 0 or score > 100:
        return 0
    if score >= 90:
        return 5
    if score >= 80:
        return 4
    if score >= 70:
        return 3
    if score >= 60:
        return 2
    return 1


# --- Error accumulation ---

def count_errors(lst: List[int]) -> int:
    """Count how many elements are negative (error indicators)."""
    errors: int = 0
    for x in lst:
        if x < 0:
            errors = errors + 1
    return errors


def count_valid(lst: List[int], lo: int, hi: int) -> int:
    """Count elements within valid range [lo, hi]."""
    valid: int = 0
    for x in lst:
        if x >= lo and x <= hi:
            valid = valid + 1
    return valid


def sum_valid_only(lst: List[int]) -> int:
    """Sum only non-negative elements, skip errors."""
    total: int = 0
    for x in lst:
        if x >= 0:
            total = total + x
    return total


def first_error_index(lst: List[int]) -> int:
    """Return index of first negative element, or -1 if none."""
    i: int = 0
    while i < len(lst):
        if lst[i] < 0:
            return i
        i = i + 1
    return -1


# --- Fallback chains ---

def fallback_divide(a: int, b: int, c: int, d: int) -> int:
    """Try a/b, then a/c, then a/d, then return 0."""
    if b != 0:
        return a // b
    if c != 0:
        return a // c
    if d != 0:
        return a // d
    return 0


def fallback_lookup(lst: List[int], i1: int, i2: int, i3: int, default: int) -> int:
    """Try index i1, then i2, then i3, then default."""
    if i1 >= 0 and i1 < len(lst):
        return lst[i1]
    if i2 >= 0 and i2 < len(lst):
        return lst[i2]
    if i3 >= 0 and i3 < len(lst):
        return lst[i3]
    return default


def coalesce(a: int, b: int, c: int, sentinel: int) -> int:
    """Return first non-sentinel value, or sentinel if all are sentinel."""
    if a != sentinel:
        return a
    if b != sentinel:
        return b
    if c != sentinel:
        return c
    return sentinel


# --- Validation pipelines ---

def validate_age(age: int) -> int:
    """0=valid, 1=negative, 2=too_large, 3=zero."""
    if age < 0:
        return 1
    if age == 0:
        return 3
    if age > 150:
        return 2
    return 0


def validate_range(val: int, lo: int, hi: int) -> int:
    """0=valid, 1=below, 2=above."""
    if val < lo:
        return 1
    if val > hi:
        return 2
    return 0


def validate_pair(a: int, b: int) -> int:
    """Validate pair: 0=ok, 1=a_negative, 2=b_negative, 3=both_negative, 4=a_equals_b."""
    if a < 0 and b < 0:
        return 3
    if a < 0:
        return 1
    if b < 0:
        return 2
    if a == b:
        return 4
    return 0


def validation_pipeline(val: int) -> int:
    """Run multiple validations, return first failure code or 0."""
    if val < 0:
        return 1
    if val > 10000:
        return 2
    if val % 2 != 0:
        return 3
    if val == 0:
        return 4
    return 0


# --- Defensive copying and transforms ---

def sum_of_clamped(lst: List[int], lo: int, hi: int) -> int:
    """Sum elements after clamping each to [lo, hi]."""
    total: int = 0
    for x in lst:
        clamped: int = clamp(x, lo, hi)
        total = total + clamped
    return total


def count_in_band(lst: List[int], center: int, width: int) -> int:
    """Count elements within [center - width, center + width]."""
    count: int = 0
    lo: int = center - width
    hi: int = center + width
    for x in lst:
        if x >= lo and x <= hi:
            count = count + 1
    return count


# --- Retry simulation with counters ---

def retry_until_positive(start: int, step: int, max_tries: int) -> int:
    """Increment from start by step until positive. Return value or -1."""
    current: int = start
    tries: int = 0
    while tries < max_tries:
        if current > 0:
            return current
        current = current + step
        tries = tries + 1
    return -1


def countdown_to_target(start: int, target: int, max_steps: int) -> int:
    """Decrement from start toward target. Return steps taken or -1."""
    current: int = start
    steps: int = 0
    while steps < max_steps:
        if current <= target:
            return steps
        current = current - 1
        steps = steps + 1
    return -1


def converge_to_zero(value: int, max_iters: int) -> int:
    """Halve value toward zero. Return iterations or -1."""
    current: int = value
    if current < 0:
        current = -current
    iters: int = 0
    while iters < max_iters:
        if current == 0:
            return iters
        current = current // 2
        iters = iters + 1
    if current == 0:
        return iters
    return -1


# --- State validation ---

def state_machine_step(state: int, input_val: int) -> int:
    """Simple state machine: returns next state or -1 for invalid."""
    if state == 0:
        if input_val > 0:
            return 1
        return 0
    if state == 1:
        if input_val > 10:
            return 2
        if input_val < 0:
            return 0
        return 1
    if state == 2:
        if input_val == 0:
            return 0
        return 2
    return -1


def run_state_machine(inputs: List[int]) -> int:
    """Run state machine over input list, return final state."""
    state: int = 0
    for inp in inputs:
        state = state_machine_step(state, inp)
        if state == -1:
            return -1
    return state


def validate_sequence(lst: List[int]) -> int:
    """0=valid ascending, 1=has_duplicate, 2=has_descent, 3=empty."""
    if len(lst) == 0:
        return 3
    i: int = 1
    while i < len(lst):
        if lst[i] == lst[i - 1]:
            return 1
        if lst[i] < lst[i - 1]:
            return 2
        i = i + 1
    return 0


# --- Guard clause heavy functions ---

def guarded_compute(a: int, b: int, c: int) -> int:
    """Multiple guard clauses before computation."""
    if a == 0:
        return 0
    if b == 0:
        return a
    if c == 0:
        return a + b
    if a < 0:
        return -1
    if b < 0:
        return -2
    if c < 0:
        return -3
    return (a * b) + c


def deeply_guarded(x: int, y: int, z: int, w: int) -> int:
    """Four-parameter guarded computation."""
    if x <= 0:
        return 0
    if y <= 0:
        return x
    if z <= 0:
        return x + y
    if w <= 0:
        return x + y + z
    return x * y + z * w


# --- Null-object pattern simulation ---

def null_safe_length(lst: List[int]) -> int:
    """Treat empty list as null-equivalent, return 0."""
    if len(lst) == 0:
        return 0
    return len(lst)


def null_safe_product(lst: List[int]) -> int:
    """Product with empty = 1 (identity) and zero short-circuit."""
    if len(lst) == 0:
        return 1
    result: int = 1
    for x in lst:
        if x == 0:
            return 0
        result = result * x
    return result


# --- Default value providers ---

def get_or_default(lst: List[int], idx: int, default: int) -> int:
    """Get element at index or return default."""
    if idx < 0 or idx >= len(lst):
        return default
    return lst[idx]


def first_positive_or_default(lst: List[int], default: int) -> int:
    """Return first positive element or default."""
    for x in lst:
        if x > 0:
            return x
    return default


def max_or_default(lst: List[int], default: int) -> int:
    """Return max of list or default if empty."""
    if len(lst) == 0:
        return default
    best: int = lst[0]
    for x in lst:
        if x > best:
            best = x
    return best


# --- Timeout simulation ---

def simulate_timeout(work: int, timeout: int) -> int:
    """Simulate work with timeout. Returns completed units or -1."""
    done: int = 0
    elapsed: int = 0
    while done < work:
        if elapsed >= timeout:
            return -1
        done = done + 1
        elapsed = elapsed + 1
    return done


def work_with_backoff(total: int, max_rounds: int) -> int:
    """Simulate work with exponential backoff. Returns rounds used."""
    remaining: int = total
    rounds: int = 0
    chunk: int = 1
    while remaining > 0 and rounds < max_rounds:
        if chunk > remaining:
            chunk = remaining
        remaining = remaining - chunk
        chunk = chunk * 2
        rounds = rounds + 1
    if remaining > 0:
        return -1
    return rounds


# --- Untyped variants for transpiler stress ---

def untyped_safe_divide(a, b):
    """Untyped division guard."""
    if b == 0:
        return -1
    return a // b


def untyped_clamp(value, lo, hi):
    """Untyped clamping."""
    if value < lo:
        return lo
    if value > hi:
        return hi
    return value


def untyped_fallback(a, b, c, sentinel):
    """Untyped coalesce."""
    if a != sentinel:
        return a
    if b != sentinel:
        return b
    if c != sentinel:
        return c
    return sentinel


def untyped_retry(start, step, max_tries):
    """Untyped retry loop."""
    current = start
    tries = 0
    while tries < max_tries:
        if current > 0:
            return current
        current = current + step
        tries = tries + 1
    return -1


def untyped_guard_compute(a, b, c):
    """Untyped guard clause function."""
    if a == 0:
        return 0
    if b == 0:
        return a
    if c == 0:
        return a + b
    return (a * b) + c


# --- Combined stress patterns ---

def safe_weighted_average(values: List[int], weights: List[int]) -> int:
    """Weighted average with mismatched-length and zero-weight guards."""
    if len(values) == 0:
        return 0
    if len(weights) == 0:
        return 0
    length: int = len(values)
    if len(weights) < length:
        length = len(weights)
    total_weight: int = 0
    weighted_sum: int = 0
    i: int = 0
    while i < length:
        w: int = weights[i]
        if w < 0:
            w = 0
        total_weight = total_weight + w
        weighted_sum = weighted_sum + values[i] * w
        i = i + 1
    if total_weight == 0:
        return 0
    return weighted_sum // total_weight


def robust_binary_search(lst: List[int], target: int) -> int:
    """Binary search with full bounds protection."""
    if len(lst) == 0:
        return -1
    lo: int = 0
    hi: int = len(lst) - 1
    while lo <= hi:
        mid: int = lo + (hi - lo) // 2
        if mid < 0 or mid >= len(lst):
            return -1
        val: int = lst[mid]
        if val == target:
            return mid
        if val < target:
            lo = mid + 1
        else:
            hi = mid - 1
    return -1


def cascading_validation(a: int, b: int, c: int, d: int) -> int:
    """Cascading validation: each param validated against previous.
    Returns 0 on success, error code 1-7 on failure."""
    if a < 0:
        return 1
    if b < a:
        return 2
    if c < b:
        return 3
    if d < c:
        return 4
    if d - a > 1000:
        return 5
    if (b - a) + (d - c) > 500:
        return 6
    if a + b + c + d == 0:
        return 7
    return 0


def error_recovery_chain(values: List[int]) -> int:
    """Process list with error recovery at each step.
    Negative = error, skip and try next. Returns sum of valid * count_valid."""
    valid_count: int = 0
    valid_sum: int = 0
    consecutive_errors: int = 0
    for v in values:
        if v < 0:
            consecutive_errors = consecutive_errors + 1
            if consecutive_errors >= 3:
                return -1
        else:
            consecutive_errors = 0
            valid_count = valid_count + 1
            valid_sum = valid_sum + v
    if valid_count == 0:
        return 0
    return valid_sum * valid_count


# --- The test runner ---

def run_all_tests() -> int:
    """Execute all test cases and return total passed count."""
    passed: int = 0

    # safe_divide
    if safe_divide(10, 3) == 3:
        passed = passed + 1
    if safe_divide(10, 0) == -1:
        passed = passed + 1
    if safe_divide(0, 5) == 0:
        passed = passed + 1

    # safe_divide_with_default
    if safe_divide_with_default(10, 0, 42) == 42:
        passed = passed + 1
    if safe_divide_with_default(10, 2, 42) == 5:
        passed = passed + 1

    # chained_division
    if chained_division(100, 5, 2) == 10:
        passed = passed + 1
    if chained_division(100, 0, 2) == -1:
        passed = passed + 1
    if chained_division(100, 5, 0) == -1:
        passed = passed + 1

    # wrapping_add
    if wrapping_add(250, 10, 255) == 4:
        passed = passed + 1

    # saturating_add
    if saturating_add(250, 10, 255) == 255:
        passed = passed + 1
    if saturating_add(5, 3, 255) == 8:
        passed = passed + 1

    # saturating_sub
    if saturating_sub(10, 3) == 7:
        passed = passed + 1
    if saturating_sub(3, 10) == 0:
        passed = passed + 1

    # saturating_mul
    if saturating_mul(10, 10, 50) == 50:
        passed = passed + 1

    # clamp
    if clamp(5, 0, 10) == 5:
        passed = passed + 1
    if clamp(-5, 0, 10) == 0:
        passed = passed + 1
    if clamp(15, 0, 10) == 10:
        passed = passed + 1

    # abs_diff
    if abs_diff(10, 3) == 7:
        passed = passed + 1
    if abs_diff(3, 10) == 7:
        passed = passed + 1

    # midpoint_safe
    if midpoint_safe(0, 10) == 5:
        passed = passed + 1
    if midpoint_safe(10, 0) == 5:
        passed = passed + 1

    # boundary_classify
    if boundary_classify(0, 100) == 0:
        passed = passed + 1
    if boundary_classify(1, 100) == 1:
        passed = passed + 1
    if boundary_classify(100, 100) == 2:
        passed = passed + 1
    if boundary_classify(50, 100) == 3:
        passed = passed + 1

    # limited_factorial
    if limited_factorial(5, 10) == 120:
        passed = passed + 1
    if limited_factorial(5, 2) == -1:
        passed = passed + 1

    # limited_fibonacci
    if limited_fibonacci(6, 20) == 8:
        passed = passed + 1

    # bounded_power
    if bounded_power(2, 8, 1000) == 256:
        passed = passed + 1
    if bounded_power(2, 20, 1000) == -1:
        passed = passed + 1

    # safe_first / safe_last
    if safe_first([10, 20, 30]) == 10:
        passed = passed + 1
    if safe_first([]) == -1:
        passed = passed + 1
    if safe_last([10, 20, 30]) == 30:
        passed = passed + 1
    if safe_last([]) == -1:
        passed = passed + 1

    # safe_sum
    if safe_sum([1, 2, 3, 4]) == 10:
        passed = passed + 1
    if safe_sum([]) == 0:
        passed = passed + 1

    # safe_max / safe_min
    if safe_max([3, 1, 4, 1, 5]) == 5:
        passed = passed + 1
    if safe_max([]) == -999999:
        passed = passed + 1
    if safe_min([3, 1, 4, 1, 5]) == 1:
        passed = passed + 1
    if safe_min([]) == 999999:
        passed = passed + 1

    # safe_average_int
    if safe_average_int([10, 20, 30]) == 20:
        passed = passed + 1
    if safe_average_int([]) == 0:
        passed = passed + 1

    # safe_index
    if safe_index([10, 20, 30], 1) == 20:
        passed = passed + 1
    if safe_index([10, 20, 30], -1) == -1:
        passed = passed + 1
    if safe_index([10, 20, 30], 5) == -1:
        passed = passed + 1

    # find_or_default
    if find_or_default([10, 20, 30], 20, -1) == 1:
        passed = passed + 1
    if find_or_default([10, 20, 30], 99, -1) == -1:
        passed = passed + 1

    # propagate_sentinel
    if propagate_sentinel(5, 3) == 8:
        passed = passed + 1
    if propagate_sentinel(-1, 3) == -1:
        passed = passed + 1
    if propagate_sentinel(5, -1) == -1:
        passed = passed + 1

    # chain_sentinel_ops
    if chain_sentinel_ops(1, 2, 3) == 6:
        passed = passed + 1
    if chain_sentinel_ops(-1, 2, 3) == -1:
        passed = passed + 1

    # sentinel_map
    if sentinel_map([1, 2, 3]) == 6:
        passed = passed + 1
    if sentinel_map([1, -1, 3]) == -1:
        passed = passed + 1

    # classify_triangle
    if classify_triangle(3, 3, 3) == 1:
        passed = passed + 1
    if classify_triangle(3, 3, 4) == 2:
        passed = passed + 1
    if classify_triangle(3, 4, 5) == 3:
        passed = passed + 1
    if classify_triangle(1, 1, 10) == 0:
        passed = passed + 1
    if classify_triangle(-1, 3, 3) == 0:
        passed = passed + 1

    # multi_range_classify
    if multi_range_classify(-5) == 0:
        passed = passed + 1
    if multi_range_classify(5) == 1:
        passed = passed + 1
    if multi_range_classify(50) == 2:
        passed = passed + 1
    if multi_range_classify(500) == 3:
        passed = passed + 1
    if multi_range_classify(5000) == 4:
        passed = passed + 1

    # grade_score
    if grade_score(95) == 5:
        passed = passed + 1
    if grade_score(85) == 4:
        passed = passed + 1
    if grade_score(55) == 1:
        passed = passed + 1
    if grade_score(-1) == 0:
        passed = passed + 1

    # count_errors
    if count_errors([1, -1, 2, -2, 3]) == 2:
        passed = passed + 1

    # count_valid
    if count_valid([1, 5, 10, 15, 20], 5, 15) == 3:
        passed = passed + 1

    # sum_valid_only
    if sum_valid_only([1, -1, 2, -2, 3]) == 6:
        passed = passed + 1

    # first_error_index
    if first_error_index([1, 2, -3, 4]) == 2:
        passed = passed + 1
    if first_error_index([1, 2, 3]) == -1:
        passed = passed + 1

    # fallback_divide
    if fallback_divide(10, 0, 0, 2) == 5:
        passed = passed + 1
    if fallback_divide(10, 0, 0, 0) == 0:
        passed = passed + 1

    # fallback_lookup
    if fallback_lookup([10, 20, 30], -1, 5, 1, 99) == 20:
        passed = passed + 1

    # coalesce
    if coalesce(-1, -1, 42, -1) == 42:
        passed = passed + 1
    if coalesce(-1, -1, -1, -1) == -1:
        passed = passed + 1

    # validate_age
    if validate_age(25) == 0:
        passed = passed + 1
    if validate_age(-5) == 1:
        passed = passed + 1
    if validate_age(200) == 2:
        passed = passed + 1
    if validate_age(0) == 3:
        passed = passed + 1

    # validate_range
    if validate_range(5, 0, 10) == 0:
        passed = passed + 1
    if validate_range(-1, 0, 10) == 1:
        passed = passed + 1
    if validate_range(11, 0, 10) == 2:
        passed = passed + 1

    # validate_pair
    if validate_pair(1, 2) == 0:
        passed = passed + 1
    if validate_pair(-1, 2) == 1:
        passed = passed + 1
    if validate_pair(1, -1) == 2:
        passed = passed + 1
    if validate_pair(-1, -1) == 3:
        passed = passed + 1
    if validate_pair(5, 5) == 4:
        passed = passed + 1

    # validation_pipeline
    if validation_pipeline(10) == 0:
        passed = passed + 1
    if validation_pipeline(-1) == 1:
        passed = passed + 1
    if validation_pipeline(20000) == 2:
        passed = passed + 1
    if validation_pipeline(7) == 3:
        passed = passed + 1
    if validation_pipeline(0) == 4:
        passed = passed + 1

    # sum_of_clamped
    if sum_of_clamped([-5, 3, 15, 7], 0, 10) == 20:
        passed = passed + 1

    # count_in_band
    if count_in_band([1, 5, 10, 15, 20], 10, 5) == 3:
        passed = passed + 1

    # retry_until_positive
    if retry_until_positive(-5, 2, 10) == 1:
        passed = passed + 1
    if retry_until_positive(-100, 1, 5) == -1:
        passed = passed + 1

    # countdown_to_target
    if countdown_to_target(10, 5, 100) == 5:
        passed = passed + 1

    # converge_to_zero
    if converge_to_zero(16, 100) == 5:
        passed = passed + 1
    if converge_to_zero(0, 100) == 0:
        passed = passed + 1

    # state_machine_step
    if state_machine_step(0, 5) == 1:
        passed = passed + 1
    if state_machine_step(0, -1) == 0:
        passed = passed + 1
    if state_machine_step(1, 15) == 2:
        passed = passed + 1
    if state_machine_step(5, 0) == -1:
        passed = passed + 1

    # run_state_machine
    if run_state_machine([5, 15, 0]) == 0:
        passed = passed + 1
    if run_state_machine([]) == 0:
        passed = passed + 1

    # validate_sequence
    if validate_sequence([1, 2, 3, 4]) == 0:
        passed = passed + 1
    if validate_sequence([1, 2, 2, 3]) == 1:
        passed = passed + 1
    if validate_sequence([1, 3, 2, 4]) == 2:
        passed = passed + 1
    if validate_sequence([]) == 3:
        passed = passed + 1

    # guarded_compute
    if guarded_compute(0, 5, 5) == 0:
        passed = passed + 1
    if guarded_compute(3, 0, 5) == 3:
        passed = passed + 1
    if guarded_compute(3, 4, 0) == 7:
        passed = passed + 1
    if guarded_compute(3, 4, 5) == 17:
        passed = passed + 1
    if guarded_compute(-1, 4, 5) == -1:
        passed = passed + 1

    # deeply_guarded
    if deeply_guarded(0, 1, 1, 1) == 0:
        passed = passed + 1
    if deeply_guarded(5, 0, 1, 1) == 5:
        passed = passed + 1
    if deeply_guarded(5, 3, 0, 1) == 8:
        passed = passed + 1
    if deeply_guarded(5, 3, 2, 0) == 10:
        passed = passed + 1
    if deeply_guarded(2, 3, 4, 5) == 26:
        passed = passed + 1

    # null_safe_length
    if null_safe_length([]) == 0:
        passed = passed + 1
    if null_safe_length([1, 2, 3]) == 3:
        passed = passed + 1

    # null_safe_product
    if null_safe_product([]) == 1:
        passed = passed + 1
    if null_safe_product([2, 3, 4]) == 24:
        passed = passed + 1
    if null_safe_product([2, 0, 4]) == 0:
        passed = passed + 1

    # get_or_default
    if get_or_default([10, 20, 30], 1, -1) == 20:
        passed = passed + 1
    if get_or_default([10, 20, 30], 5, -1) == -1:
        passed = passed + 1

    # first_positive_or_default
    if first_positive_or_default([-1, -2, 3, 4], 0) == 3:
        passed = passed + 1
    if first_positive_or_default([-1, -2], 0) == 0:
        passed = passed + 1

    # max_or_default
    if max_or_default([3, 1, 4], 0) == 4:
        passed = passed + 1
    if max_or_default([], 0) == 0:
        passed = passed + 1

    # simulate_timeout
    if simulate_timeout(5, 10) == 5:
        passed = passed + 1
    if simulate_timeout(10, 5) == -1:
        passed = passed + 1

    # work_with_backoff
    if work_with_backoff(7, 10) == 3:
        passed = passed + 1
    if work_with_backoff(1000, 3) == -1:
        passed = passed + 1

    # untyped_safe_divide
    if untyped_safe_divide(10, 0) == -1:
        passed = passed + 1
    if untyped_safe_divide(10, 3) == 3:
        passed = passed + 1

    # untyped_clamp
    if untyped_clamp(5, 0, 10) == 5:
        passed = passed + 1
    if untyped_clamp(-5, 0, 10) == 0:
        passed = passed + 1

    # untyped_fallback
    if untyped_fallback(-1, -1, 42, -1) == 42:
        passed = passed + 1

    # untyped_retry
    if untyped_retry(-5, 2, 10) == 1:
        passed = passed + 1

    # untyped_guard_compute
    if untyped_guard_compute(3, 4, 5) == 17:
        passed = passed + 1

    # safe_weighted_average
    if safe_weighted_average([10, 20, 30], [1, 2, 3]) == 23:
        passed = passed + 1
    if safe_weighted_average([], [1, 2]) == 0:
        passed = passed + 1
    if safe_weighted_average([10, 20], []) == 0:
        passed = passed + 1

    # robust_binary_search
    if robust_binary_search([1, 3, 5, 7, 9], 5) == 2:
        passed = passed + 1
    if robust_binary_search([1, 3, 5, 7, 9], 4) == -1:
        passed = passed + 1
    if robust_binary_search([], 5) == -1:
        passed = passed + 1

    # cascading_validation
    if cascading_validation(1, 2, 3, 4) == 0:
        passed = passed + 1
    if cascading_validation(-1, 2, 3, 4) == 1:
        passed = passed + 1
    if cascading_validation(5, 3, 4, 5) == 2:
        passed = passed + 1

    # error_recovery_chain
    if error_recovery_chain([1, 2, 3]) == 18:
        passed = passed + 1
    if error_recovery_chain([-1, -1, -1, 5]) == -1:
        passed = passed + 1
    if error_recovery_chain([1, -1, 2, -1, 3]) == 18:
        passed = passed + 1

    return passed


if __name__ == "__main__":
    result: int = run_all_tests()
    print(result)
