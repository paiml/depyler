"""Pathological type narrowing and complex expression patterns.

Stresses type inference, conditional type handling, mixed arithmetic,
ternary chains, bitwise operations, chained comparisons, string
multiplication, floor/true division, power operator, and boolean
short-circuit evaluation.
"""


# ---------- 1. Conditional return types ----------

def clamp_positive(x: int) -> int:
    """Return x if positive, else return 0 as default."""
    if x > 0:
        return x
    else:
        return 0


def test_clamp_positive() -> int:
    """Test conditional return with positive and negative inputs."""
    a: int = clamp_positive(42)
    b: int = clamp_positive(-7)
    c: int = clamp_positive(0)
    return a + b + c


# ---------- 2. Type narrowing via value checks ----------

def safe_divide(a: int, b: int) -> int:
    """Divide a by b, returning 0 when b is zero."""
    if b != 0:
        return a // b
    else:
        return 0


def test_safe_divide() -> int:
    """Test division with zero and non-zero divisors."""
    r1: int = safe_divide(100, 3)
    r2: int = safe_divide(50, 0)
    r3: int = safe_divide(-20, 4)
    return r1 + r2 + r3


# ---------- 3. Complex ternary chains ----------

def classify_score(score: int) -> int:
    """Map score to grade bucket via chained ternary expressions."""
    return 4 if score >= 90 else 3 if score >= 80 else 2 if score >= 70 else 1 if score >= 60 else 0


def test_classify_score() -> int:
    """Test all five ternary branches."""
    a: int = classify_score(95)
    b: int = classify_score(85)
    c: int = classify_score(75)
    d: int = classify_score(65)
    e: int = classify_score(50)
    return a + b + c + d + e


# ---------- 4. Mixed int/float arithmetic ----------

def weighted_average(a: int, b: int, w: float) -> float:
    """Weighted average mixing int and float types."""
    left: float = a * w
    right: float = b * (1.0 - w)
    return left + right


def test_weighted_average() -> int:
    """Test mixed int/float arithmetic returns expected value."""
    result: float = weighted_average(10, 20, 0.5)
    return int(result)


# ---------- 5. String-to-int parsing patterns ----------

def parse_digit_sum(s: str) -> int:
    """Sum the digit values in a string of digits."""
    total: int = 0
    for ch in s:
        if ch.isdigit():
            total += int(ch)
    return total


def test_parse_digit_sum() -> int:
    """Test digit parsing from mixed string."""
    r1: int = parse_digit_sum("12345")
    r2: int = parse_digit_sum("a1b2c3")
    return r1 + r2


# ---------- 6. Chained comparisons ----------

def in_range_exclusive(x: int, lo: int, hi: int) -> bool:
    """Check if lo < x < hi using chained comparison."""
    return lo < x < hi


def count_in_band(values: list[int], lo: int, hi: int) -> int:
    """Count values strictly between lo and hi."""
    count: int = 0
    for v in values:
        if lo < v < hi:
            count += 1
    return count


def test_chained_comparisons() -> int:
    """Test chained comparison logic."""
    nums: list[int] = [1, 5, 10, 15, 20, 25, 30]
    c: int = count_in_band(nums, 5, 25)
    b1: int = 1 if in_range_exclusive(10, 0, 20) else 0
    b2: int = 1 if in_range_exclusive(0, 0, 20) else 0
    return c + b1 + b2


# ---------- 7. Multiple assignment from function ----------

def divmod_pair(a: int, b: int) -> list[int]:
    """Return quotient and remainder as a two-element list."""
    q: int = a // b
    r: int = a % b
    return [q, r]


def test_divmod_pair() -> int:
    """Test multiple return via list unpacking."""
    pair: list[int] = divmod_pair(17, 5)
    q: int = pair[0]
    r: int = pair[1]
    return q * 10 + r


# ---------- 8. Nested conditional expressions ----------

def filter_in_range(lst: list[int], lo: int, hi: int) -> list[int]:
    """Filter list to values in [lo, hi) using compound condition."""
    result: list[int] = []
    for x in lst:
        if x >= lo and x < hi:
            result.append(x)
    return result


def test_filter_in_range() -> int:
    """Test list filtering with compound conditions."""
    data: list[int] = [3, 7, 15, 22, 50, 75, 99, 100]
    filtered: list[int] = filter_in_range(data, 10, 80)
    total: int = 0
    for v in filtered:
        total += v
    return total


# ---------- 9. Dict comprehension with conditionals ----------

def index_above_threshold(values: list[int], threshold: int) -> dict[int, int]:
    """Build dict mapping index to value for values above threshold."""
    result: dict[int, int] = {}
    for i in range(len(values)):
        if values[i] > threshold:
            result[i] = values[i]
    return result


def test_index_above_threshold() -> int:
    """Test dict building with conditional filtering."""
    data: list[int] = [5, 12, 3, 18, 7, 25, 1]
    d: dict[int, int] = index_above_threshold(data, 10)
    total: int = 0
    for k in d:
        total += d[k]
    return total


# ---------- 10. Set operations ----------

def set_algebra(a: list[int], b: list[int], c: list[int]) -> int:
    """Compute |(a union b) intersect c| using set operations."""
    sa: set[int] = set(a)
    sb: set[int] = set(b)
    sc: set[int] = set(c)
    union_ab: set[int] = sa | sb
    result: set[int] = union_ab & sc
    return len(result)


def test_set_algebra() -> int:
    """Test set union, intersection chain."""
    a: list[int] = [1, 2, 3, 4, 5]
    b: list[int] = [4, 5, 6, 7, 8]
    c: list[int] = [2, 4, 6, 8, 10]
    return set_algebra(a, b, c)


# ---------- 11. String multiplication ----------

def repeat_string(s: str, n: int) -> str:
    """Repeat a string n times."""
    return s * n


def build_separator(char: str, width: int) -> str:
    """Build a separator line of given width."""
    return char * width


def test_string_multiplication() -> int:
    """Test string repetition patterns."""
    r1: str = repeat_string("ab", 3)
    r2: str = build_separator("-", 10)
    return len(r1) + len(r2)


# ---------- 12. Floor division vs true division ----------

def floor_vs_true(a: int, b: int) -> list[int]:
    """Compare floor division and integer division results."""
    floor_result: int = a // b
    remainder: int = a % b
    reconstructed: int = floor_result * b + remainder
    return [floor_result, remainder, reconstructed]


def test_floor_division() -> int:
    """Test floor division with positive and negative operands."""
    pos: list[int] = floor_vs_true(17, 5)
    neg: list[int] = floor_vs_true(-17, 5)
    return pos[0] + pos[1] + neg[0] + neg[1]


# ---------- 13. Power operator ----------

def power_chain(base: int, exp: int) -> int:
    """Compute base raised to exp using the power operator."""
    return base ** exp


def sum_of_squares(n: int) -> int:
    """Sum of squares from 1 to n using power operator."""
    total: int = 0
    for i in range(1, n + 1):
        total += i ** 2
    return total


def test_power_operator() -> int:
    """Test power operator with various combinations."""
    a: int = power_chain(2, 10)
    b: int = power_chain(3, 4)
    c: int = sum_of_squares(5)
    return a + b + c


# ---------- 14. Bitwise operations ----------

def extract_bits(x: int, mask: int) -> int:
    """Extract bits from x using AND mask."""
    return x & mask


def set_flags(x: int, flag: int) -> int:
    """Set flag bits in x using OR."""
    return x | flag


def toggle_bits(x: int, toggle: int) -> int:
    """Toggle bits in x using XOR."""
    return x ^ toggle


def shift_pack(high: int, low: int, bits: int) -> int:
    """Pack two values by shifting high left and ORing with low."""
    return (high << bits) | low


def shift_unpack_high(packed: int, bits: int) -> int:
    """Unpack the high portion by shifting right."""
    return packed >> bits


def test_bitwise_operations() -> int:
    """Test AND, OR, XOR, left shift, right shift."""
    a: int = extract_bits(0xFF, 0x0F)
    b: int = set_flags(0xF0, 0x0F)
    c: int = toggle_bits(0xFF, 0x0F)
    packed: int = shift_pack(5, 3, 4)
    high: int = shift_unpack_high(packed, 4)
    return a + b + c + packed + high


# ---------- 15. Complex boolean short-circuit ----------

def threshold_check(x: int) -> bool:
    """Return True if x is above threshold."""
    return x > 50


def parity_check(x: int) -> bool:
    """Return True if x is even."""
    return x % 2 == 0


def range_check(x: int) -> bool:
    """Return True if x is in [10, 100]."""
    return x >= 10 and x <= 100


def compound_logic(a: int, b: int, c: int) -> int:
    """Evaluate compound boolean with short-circuit semantics.

    Returns 1 if (threshold_check(a) and parity_check(b)) or range_check(c),
    otherwise returns 0.
    """
    if threshold_check(a) and parity_check(b):
        return 1
    if range_check(c):
        return 1
    return 0


def test_boolean_short_circuit() -> int:
    """Test complex boolean short-circuit evaluation paths."""
    r1: int = compound_logic(60, 4, 200)
    r2: int = compound_logic(30, 4, 50)
    r3: int = compound_logic(30, 3, 5)
    r4: int = compound_logic(80, 7, 200)
    return r1 + r2 + r3 + r4


# ---------- Runner ----------

def run_all_tests() -> int:
    """Run all test functions and return sum of results."""
    total: int = 0
    total += test_clamp_positive()
    total += test_safe_divide()
    total += test_classify_score()
    total += test_weighted_average()
    total += test_parse_digit_sum()
    total += test_chained_comparisons()
    total += test_divmod_pair()
    total += test_filter_in_range()
    total += test_index_above_threshold()
    total += test_set_algebra()
    total += test_string_multiplication()
    total += test_floor_division()
    total += test_power_operator()
    total += test_bitwise_operations()
    total += test_boolean_short_circuit()
    return total
