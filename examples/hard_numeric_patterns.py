# Hard numeric computation patterns for transpiler stress testing
# NO imports, NO I/O, ALL pure functions, ALL type-annotated


# ============================================================
# Category 1: Fixed-point arithmetic (scale factor = 1000)
# ============================================================

def fixed_multiply(a: int, b: int, scale: int) -> int:
    """Multiply two fixed-point numbers with proper scaling."""
    if scale == 0:
        return 0
    sign: int = 1
    if a < 0:
        sign = -sign
        a = -a
    if b < 0:
        sign = -sign
        b = -b
    high_a: int = a // scale
    low_a: int = a % scale
    high_b: int = b // scale
    low_b: int = b % scale
    result: int = high_a * high_b * scale + high_a * low_b + low_a * high_b + (low_a * low_b) // scale
    return sign * result


def fixed_divide(a: int, b: int, scale: int) -> int:
    """Divide two fixed-point numbers."""
    if b == 0 or scale == 0:
        return 0
    sign: int = 1
    if a < 0:
        sign = -sign
        a = -a
    if b < 0:
        sign = -sign
        b = -b
    result: int = (a * scale) // b
    return sign * result


def fixed_sqrt(x: int, scale: int) -> int:
    """Integer square root of fixed-point number using Newton's method."""
    if x <= 0 or scale <= 0:
        return 0
    guess: int = x
    prev: int = 0
    iterations: int = 0
    while guess != prev and iterations < 100:
        prev = guess
        if guess == 0:
            return 0
        guess = (guess + fixed_divide(x, guess, scale)) // 2
        iterations += 1
    return guess


def fixed_exp_approx(x: int, scale: int, terms: int) -> int:
    """Approximate e^x using Taylor series in fixed-point."""
    if scale == 0:
        return 0
    result: int = scale
    term: int = scale
    i: int = 1
    while i <= terms:
        term = fixed_multiply(term, x, scale) // i
        if term == 0:
            break
        result = result + term
        i += 1
    return result


def test_fixed_point() -> int:
    """Test fixed-point arithmetic operations."""
    total: int = 0
    scale: int = 1000
    r1: int = fixed_multiply(1500, 2000, scale)
    if r1 == 3000:
        total = total + 1
    r2: int = fixed_multiply(-1500, 2000, scale)
    if r2 == -3000:
        total = total + 1
    r3: int = fixed_divide(3000, 1500, scale)
    if r3 == 2000:
        total = total + 1
    r4: int = fixed_divide(0, 1500, scale)
    if r4 == 0:
        total = total + 1
    r5: int = fixed_divide(1000, 0, scale)
    if r5 == 0:
        total = total + 1
    r6: int = fixed_sqrt(4000, scale)
    if r6 >= 1990 and r6 <= 2010:
        total = total + 1
    r7: int = fixed_sqrt(0, scale)
    if r7 == 0:
        total = total + 1
    r8: int = fixed_exp_approx(0, scale, 10)
    if r8 == scale:
        total = total + 1
    return total


# ============================================================
# Category 2: Bit manipulation
# ============================================================

def popcount(n: int) -> int:
    """Count set bits in non-negative integer."""
    if n < 0:
        n = -n
    count: int = 0
    while n > 0:
        count = count + (n & 1)
        n = n >> 1
    return count


def parity(n: int) -> int:
    """Return 0 if even number of set bits, 1 if odd."""
    if n < 0:
        n = -n
    p: int = 0
    while n > 0:
        p = p ^ (n & 1)
        n = n >> 1
    return p


def leading_zeros_32(n: int) -> int:
    """Count leading zeros in a 32-bit representation."""
    if n <= 0:
        if n == 0:
            return 32
        return 0
    count: int = 0
    mask: int = 1 << 31
    while mask > 0 and (n & mask) == 0:
        count = count + 1
        mask = mask >> 1
    return count


def trailing_zeros(n: int) -> int:
    """Count trailing zeros."""
    if n == 0:
        return 32
    if n < 0:
        n = -n
    count: int = 0
    while (n & 1) == 0:
        count = count + 1
        n = n >> 1
    return count


def reverse_bits_32(n: int) -> int:
    """Reverse the bits of a 32-bit integer."""
    if n < 0:
        n = n & 0xFFFFFFFF
    result: int = 0
    i: int = 0
    while i < 32:
        result = (result << 1) | (n & 1)
        n = n >> 1
        i = i + 1
    return result


def next_power_of_two(n: int) -> int:
    """Find the next power of two >= n."""
    if n <= 0:
        return 1
    if n & (n - 1) == 0:
        return n
    result: int = 1
    while result < n:
        result = result << 1
    return result


def isolate_lowest_set_bit(n: int) -> int:
    """Isolate the lowest set bit."""
    if n == 0:
        return 0
    return n & (-n)


def clear_lowest_set_bit(n: int) -> int:
    """Clear the lowest set bit."""
    return n & (n - 1)


def bit_interleave(x: int, y: int) -> int:
    """Interleave bits of x and y (Morton code) for bottom 8 bits each."""
    x = x & 0xFF
    y = y & 0xFF
    result: int = 0
    i: int = 0
    while i < 8:
        result = result | ((x & (1 << i)) << i) | ((y & (1 << i)) << (i + 1))
        i = i + 1
    return result


def test_bit_manipulation() -> int:
    """Test bit manipulation operations."""
    total: int = 0
    if popcount(0) == 0:
        total = total + 1
    if popcount(7) == 3:
        total = total + 1
    if popcount(255) == 8:
        total = total + 1
    if parity(7) == 1:
        total = total + 1
    if parity(3) == 0:
        total = total + 1
    if leading_zeros_32(1) == 31:
        total = total + 1
    if leading_zeros_32(0) == 32:
        total = total + 1
    if trailing_zeros(8) == 3:
        total = total + 1
    if trailing_zeros(0) == 32:
        total = total + 1
    if next_power_of_two(5) == 8:
        total = total + 1
    if next_power_of_two(8) == 8:
        total = total + 1
    if isolate_lowest_set_bit(12) == 4:
        total = total + 1
    if clear_lowest_set_bit(12) == 8:
        total = total + 1
    r: int = reverse_bits_32(1)
    if r == (1 << 31):
        total = total + 1
    if bit_interleave(0, 0) == 0:
        total = total + 1
    return total


# ============================================================
# Category 3: Integer overflow detection
# ============================================================

def safe_add(a: int, b: int, max_val: int) -> int:
    """Add with overflow detection, returns max_val on overflow."""
    if b > 0 and a > max_val - b:
        return max_val
    if b < 0 and a < -max_val - b:
        return -max_val
    return a + b


def safe_multiply(a: int, b: int, max_val: int) -> int:
    """Multiply with overflow detection."""
    if a == 0 or b == 0:
        return 0
    if max_val <= 0:
        return 0
    sign: int = 1
    abs_a: int = a
    abs_b: int = b
    if a < 0:
        sign = -sign
        abs_a = -a
    if b < 0:
        sign = -sign
        abs_b = -b
    if abs_a > max_val // abs_b:
        return sign * max_val
    return a * b


def safe_power(base: int, exp: int, max_val: int) -> int:
    """Compute base^exp with overflow detection."""
    if exp < 0:
        return 0
    if exp == 0:
        return 1
    if max_val <= 0:
        return 0
    result: int = 1
    b: int = base
    e: int = exp
    while e > 0:
        if e & 1 == 1:
            result = safe_multiply(result, b, max_val)
            if result >= max_val or result <= -max_val:
                return result
        b = safe_multiply(b, b, max_val)
        e = e >> 1
    return result


def test_overflow_detection() -> int:
    """Test overflow detection patterns."""
    total: int = 0
    mx: int = 1000000
    if safe_add(999999, 2, mx) == mx:
        total = total + 1
    if safe_add(100, 200, mx) == 300:
        total = total + 1
    if safe_add(-999999, -2, mx) == -mx:
        total = total + 1
    if safe_multiply(0, 100, mx) == 0:
        total = total + 1
    if safe_multiply(1001, 1001, mx) == mx:
        total = total + 1
    if safe_multiply(100, 100, mx) == 10000:
        total = total + 1
    if safe_power(2, 0, mx) == 1:
        total = total + 1
    if safe_power(2, 10, mx) == 1024:
        total = total + 1
    if safe_power(2, 30, mx) == mx:
        total = total + 1
    return total


# ============================================================
# Category 4: Division / modulo edge cases
# ============================================================

def safe_div(a: int, b: int) -> int:
    """Division that handles zero and negative correctly."""
    if b == 0:
        if a > 0:
            return 2147483647
        if a < 0:
            return -2147483647
        return 0
    if a == -2147483648 and b == -1:
        return 2147483647
    return a // b


def safe_mod(a: int, b: int) -> int:
    """Modulo that handles negative numbers consistently."""
    if b == 0:
        return 0
    r: int = a % b
    if r < 0 and b > 0:
        r = r + b
    elif r > 0 and b < 0:
        r = r + b
    return r


def euclidean_div(a: int, b: int) -> int:
    """Euclidean division (result always non-negative remainder)."""
    if b == 0:
        return 0
    q: int = a // b
    r: int = a - q * b
    if r < 0:
        if b > 0:
            q = q - 1
        else:
            q = q + 1
    return q


def ceiling_div(a: int, b: int) -> int:
    """Ceiling division."""
    if b == 0:
        return 0
    if (a >= 0 and b > 0) or (a <= 0 and b < 0):
        return (a + b - 1) // b if b > 0 else (a + b + 1) // b
    return a // b


def test_division_edge_cases() -> int:
    """Test division edge cases."""
    total: int = 0
    if safe_div(10, 3) == 3:
        total = total + 1
    if safe_div(10, 0) == 2147483647:
        total = total + 1
    if safe_div(-10, 0) == -2147483647:
        total = total + 1
    if safe_mod(10, 3) == 1:
        total = total + 1
    if safe_mod(0, 5) == 0:
        total = total + 1
    if safe_mod(10, 0) == 0:
        total = total + 1
    if euclidean_div(7, 3) == 2:
        total = total + 1
    if euclidean_div(0, 5) == 0:
        total = total + 1
    if ceiling_div(7, 3) == 3:
        total = total + 1
    if ceiling_div(6, 3) == 2:
        total = total + 1
    if ceiling_div(0, 1) == 0:
        total = total + 1
    return total


# ============================================================
# Category 5: Multi-precision arithmetic (digit lists, base 10)
# ============================================================

def bignum_add(a: list[int], b: list[int]) -> list[int]:
    """Add two big numbers represented as digit lists (LSB first)."""
    max_len: int = len(a) if len(a) > len(b) else len(b)
    result: list[int] = []
    carry: int = 0
    i: int = 0
    while i < max_len or carry > 0:
        digit_a: int = a[i] if i < len(a) else 0
        digit_b: int = b[i] if i < len(b) else 0
        s: int = digit_a + digit_b + carry
        result.append(s % 10)
        carry = s // 10
        i = i + 1
    return result


def bignum_multiply_scalar(a: list[int], scalar: int) -> list[int]:
    """Multiply a big number by a single digit."""
    if scalar == 0:
        return [0]
    result: list[int] = []
    carry: int = 0
    i: int = 0
    while i < len(a):
        prod: int = a[i] * scalar + carry
        result.append(prod % 10)
        carry = prod // 10
        i = i + 1
    while carry > 0:
        result.append(carry % 10)
        carry = carry // 10
    return result


def bignum_to_int(digits: list[int]) -> int:
    """Convert digit list (LSB first) to integer."""
    result: int = 0
    power: int = 1
    i: int = 0
    while i < len(digits):
        result = result + digits[i] * power
        power = power * 10
        i = i + 1
    return result


def bignum_from_int(n: int) -> list[int]:
    """Convert integer to digit list (LSB first)."""
    if n == 0:
        return [0]
    result: list[int] = []
    val: int = n
    if val < 0:
        val = -val
    while val > 0:
        result.append(val % 10)
        val = val // 10
    return result


def bignum_compare(a: list[int], b: list[int]) -> int:
    """Compare two bignums. Returns -1, 0, or 1."""
    la: int = len(a)
    lb: int = len(b)
    while la > 1 and a[la - 1] == 0:
        la = la - 1
    while lb > 1 and b[lb - 1] == 0:
        lb = lb - 1
    if la != lb:
        if la < lb:
            return -1
        return 1
    i: int = la - 1
    while i >= 0:
        if a[i] < b[i]:
            return -1
        if a[i] > b[i]:
            return 1
        i = i - 1
    return 0


def test_bignum() -> int:
    """Test multi-precision arithmetic."""
    total: int = 0
    a: list[int] = [9, 9, 9]
    b: list[int] = [1]
    s: list[int] = bignum_add(a, b)
    if bignum_to_int(s) == 1000:
        total = total + 1
    c: list[int] = bignum_multiply_scalar([2, 1], 3)
    if bignum_to_int(c) == 36:
        total = total + 1
    d: list[int] = bignum_from_int(12345)
    if bignum_to_int(d) == 12345:
        total = total + 1
    if bignum_from_int(0) == [0]:
        total = total + 1
    if bignum_compare([1, 2], [1, 2]) == 0:
        total = total + 1
    if bignum_compare([1, 2], [1, 3]) == -1:
        total = total + 1
    if bignum_compare([1, 3], [1, 2]) == 1:
        total = total + 1
    if bignum_compare([1], [1, 2]) == -1:
        total = total + 1
    return total


# ============================================================
# Category 6: Number base conversion
# ============================================================

def to_base(n: int, base: int) -> list[int]:
    """Convert n to given base, returns digit list (LSB first)."""
    if base < 2:
        return [0]
    if n == 0:
        return [0]
    val: int = n
    if val < 0:
        val = -val
    result: list[int] = []
    while val > 0:
        result.append(val % base)
        val = val // base
    return result


def from_base(digits: list[int], base: int) -> int:
    """Convert digit list (LSB first) from given base to decimal."""
    if base < 2:
        return 0
    result: int = 0
    power: int = 1
    i: int = 0
    while i < len(digits):
        result = result + digits[i] * power
        power = power * base
        i = i + 1
    return result


def to_binary_str_value(n: int) -> int:
    """Return the decimal number formed by the binary digits of n.
    E.g. 5 -> 101 (reading binary digits as decimal)."""
    if n == 0:
        return 0
    val: int = n
    if val < 0:
        val = -val
    result: int = 0
    power: int = 1
    while val > 0:
        bit: int = val & 1
        result = result + bit * power
        power = power * 10
        val = val >> 1
    return result


def count_digits_in_base(n: int, base: int) -> int:
    """Count how many digits n has in given base."""
    if base < 2:
        return 0
    if n == 0:
        return 1
    val: int = n
    if val < 0:
        val = -val
    count: int = 0
    while val > 0:
        val = val // base
        count = count + 1
    return count


def test_base_conversion() -> int:
    """Test base conversion operations."""
    total: int = 0
    b: list[int] = to_base(10, 2)
    if from_base(b, 2) == 10:
        total = total + 1
    h: list[int] = to_base(255, 16)
    if from_base(h, 16) == 255:
        total = total + 1
    o: list[int] = to_base(8, 8)
    if from_base(o, 8) == 8:
        total = total + 1
    if to_base(0, 2) == [0]:
        total = total + 1
    if to_binary_str_value(5) == 101:
        total = total + 1
    if to_binary_str_value(0) == 0:
        total = total + 1
    if count_digits_in_base(255, 16) == 2:
        total = total + 1
    if count_digits_in_base(0, 10) == 1:
        total = total + 1
    if count_digits_in_base(999, 10) == 3:
        total = total + 1
    return total


# ============================================================
# Category 7: Numeric parsing from digit lists
# ============================================================

def parse_digits(digits: list[int]) -> int:
    """Parse MSB-first digit list to integer."""
    result: int = 0
    i: int = 0
    while i < len(digits):
        d: int = digits[i]
        if d < 0 or d > 9:
            return -1
        result = result * 10 + d
        i = i + 1
    return result


def split_into_digits(n: int) -> list[int]:
    """Split integer into MSB-first digit list."""
    if n == 0:
        return [0]
    val: int = n
    if val < 0:
        val = -val
    digits: list[int] = []
    while val > 0:
        digits.append(val % 10)
        val = val // 10
    result: list[int] = []
    i: int = len(digits) - 1
    while i >= 0:
        result.append(digits[i])
        i = i - 1
    return result


def is_valid_digit_list(digits: list[int], base: int) -> int:
    """Check if all digits are valid for given base. 1=valid, 0=invalid."""
    if base < 2:
        return 0
    i: int = 0
    while i < len(digits):
        if digits[i] < 0 or digits[i] >= base:
            return 0
        i = i + 1
    return 1


def digit_sum_recursive(n: int) -> int:
    """Recursively sum digits until single digit."""
    if n < 0:
        n = -n
    while n >= 10:
        s: int = 0
        while n > 0:
            s = s + n % 10
            n = n // 10
        n = s
    return n


def test_digit_parsing() -> int:
    """Test digit parsing operations."""
    total: int = 0
    if parse_digits([1, 2, 3]) == 123:
        total = total + 1
    if parse_digits([0]) == 0:
        total = total + 1
    if parse_digits([1, 11, 3]) == -1:
        total = total + 1
    d: list[int] = split_into_digits(4567)
    if d == [4, 5, 6, 7]:
        total = total + 1
    if split_into_digits(0) == [0]:
        total = total + 1
    if is_valid_digit_list([0, 1], 2) == 1:
        total = total + 1
    if is_valid_digit_list([0, 2], 2) == 0:
        total = total + 1
    if digit_sum_recursive(9999) == 9:
        total = total + 1
    if digit_sum_recursive(0) == 0:
        total = total + 1
    return total


# ============================================================
# Category 8: Continued fraction computation
# ============================================================

def continued_fraction_sqrt(n: int, terms: int) -> list[int]:
    """Compute continued fraction expansion of sqrt(n) up to 'terms' terms."""
    if n <= 0:
        return [0]
    a0: int = 0
    while (a0 + 1) * (a0 + 1) <= n:
        a0 = a0 + 1
    if a0 * a0 == n:
        return [a0]
    result: list[int] = [a0]
    m: int = 0
    d: int = 1
    a: int = a0
    count: int = 0
    while count < terms:
        m = d * a - m
        if m == 0 and d == 0:
            break
        d = (n - m * m) // d
        if d == 0:
            break
        a = (a0 + m) // d
        result.append(a)
        count = count + 1
    return result


def evaluate_continued_fraction(cf: list[int]) -> list[int]:
    """Evaluate continued fraction, returns [numerator, denominator]."""
    if len(cf) == 0:
        return [0, 1]
    i: int = len(cf) - 1
    num: int = cf[i]
    den: int = 1
    i = i - 1
    while i >= 0:
        old_num: int = num
        num = cf[i] * num + den
        den = old_num
        i = i - 1
    return [num, den]


def convergent_of_sqrt(n: int, depth: int) -> list[int]:
    """Get rational approximation of sqrt(n) as [num, den]."""
    cf: list[int] = continued_fraction_sqrt(n, depth)
    return evaluate_continued_fraction(cf)


def test_continued_fractions() -> int:
    """Test continued fraction computations."""
    total: int = 0
    cf4: list[int] = continued_fraction_sqrt(4, 5)
    if cf4 == [2]:
        total = total + 1
    cf2: list[int] = continued_fraction_sqrt(2, 4)
    if len(cf2) >= 2 and cf2[0] == 1:
        total = total + 1
    ev: list[int] = evaluate_continued_fraction([3, 7])
    if ev[0] == 22 and ev[1] == 7:
        total = total + 1
    ev2: list[int] = evaluate_continued_fraction([1])
    if ev2[0] == 1 and ev2[1] == 1:
        total = total + 1
    approx: list[int] = convergent_of_sqrt(2, 5)
    ratio: int = approx[0] * approx[0]
    target: int = 2 * approx[1] * approx[1]
    diff: int = ratio - target
    if diff < 0:
        diff = -diff
    if diff <= approx[1]:
        total = total + 1
    if continued_fraction_sqrt(0, 5) == [0]:
        total = total + 1
    return total


# ============================================================
# Category 9: Newton's method (integer)
# ============================================================

def isqrt(n: int) -> int:
    """Integer square root using Newton's method."""
    if n < 0:
        return -1
    if n == 0:
        return 0
    x: int = n
    y: int = (x + 1) // 2
    while y < x:
        x = y
        y = (x + n // x) // 2
    return x


def icbrt(n: int) -> int:
    """Integer cube root using Newton's method."""
    if n == 0:
        return 0
    neg: int = 0
    val: int = n
    if n < 0:
        neg = 1
        val = -n
    x: int = val
    while True:
        if x == 0:
            break
        x2: int = (2 * x + val // (x * x)) // 3
        if x2 >= x:
            break
        x = x2
    while x * x * x > val:
        x = x - 1
    if neg == 1:
        return -x
    return x


def integer_nth_root(n: int, k: int) -> int:
    """Compute floor(n^(1/k)) using Newton's method."""
    if k <= 0:
        return 0
    if n <= 0:
        return 0
    if k == 1:
        return n
    x: int = 1
    while True:
        xk: int = 1
        i: int = 0
        while i < k:
            xk = xk * (x + 1)
            i = i + 1
        if xk > n:
            break
        x = x + 1
    return x


def test_newton_methods() -> int:
    """Test Newton's method implementations."""
    total: int = 0
    if isqrt(0) == 0:
        total = total + 1
    if isqrt(1) == 1:
        total = total + 1
    if isqrt(4) == 2:
        total = total + 1
    if isqrt(8) == 2:
        total = total + 1
    if isqrt(9) == 3:
        total = total + 1
    if isqrt(100) == 10:
        total = total + 1
    if isqrt(-1) == -1:
        total = total + 1
    if icbrt(0) == 0:
        total = total + 1
    if icbrt(8) == 2:
        total = total + 1
    if icbrt(27) == 3:
        total = total + 1
    if icbrt(-27) == -3:
        total = total + 1
    if integer_nth_root(16, 2) == 4:
        total = total + 1
    if integer_nth_root(16, 4) == 2:
        total = total + 1
    return total


# ============================================================
# Category 10: Matrix operations on 2D lists
# ============================================================

def matrix_multiply(a: list[list[int]], b: list[list[int]]) -> list[list[int]]:
    """Multiply two matrices represented as 2D lists."""
    rows_a: int = len(a)
    if rows_a == 0:
        return []
    cols_a: int = len(a[0])
    rows_b: int = len(b)
    if rows_b == 0 or cols_a != rows_b:
        return []
    cols_b: int = len(b[0])
    result: list[list[int]] = []
    i: int = 0
    while i < rows_a:
        row: list[int] = []
        j: int = 0
        while j < cols_b:
            s: int = 0
            k: int = 0
            while k < cols_a:
                s = s + a[i][k] * b[k][j]
                k = k + 1
            row.append(s)
            j = j + 1
        result.append(row)
        i = i + 1
    return result


def matrix_transpose(m: list[list[int]]) -> list[list[int]]:
    """Transpose a matrix."""
    rows: int = len(m)
    if rows == 0:
        return []
    cols: int = len(m[0])
    result: list[list[int]] = []
    j: int = 0
    while j < cols:
        row: list[int] = []
        i: int = 0
        while i < rows:
            row.append(m[i][j])
            i = i + 1
        result.append(row)
        j = j + 1
    return result


def matrix_determinant_2x2(m: list[list[int]]) -> int:
    """Compute determinant of a 2x2 matrix."""
    if len(m) != 2 or len(m[0]) != 2 or len(m[1]) != 2:
        return 0
    return m[0][0] * m[1][1] - m[0][1] * m[1][0]


def matrix_determinant_3x3(m: list[list[int]]) -> int:
    """Compute determinant of a 3x3 matrix using cofactor expansion."""
    if len(m) != 3:
        return 0
    i: int = 0
    while i < 3:
        if len(m[i]) != 3:
            return 0
        i = i + 1
    a: int = m[0][0]
    b: int = m[0][1]
    c: int = m[0][2]
    d: int = m[1][0]
    e: int = m[1][1]
    f: int = m[1][2]
    g: int = m[2][0]
    h: int = m[2][1]
    k: int = m[2][2]
    return a * (e * k - f * h) - b * (d * k - f * g) + c * (d * h - e * g)


def matrix_trace(m: list[list[int]]) -> int:
    """Compute trace (sum of diagonal) of a square matrix."""
    n: int = len(m)
    s: int = 0
    i: int = 0
    while i < n:
        if i < len(m[i]):
            s = s + m[i][i]
        i = i + 1
    return s


def matrix_identity(n: int) -> list[list[int]]:
    """Create n x n identity matrix."""
    result: list[list[int]] = []
    i: int = 0
    while i < n:
        row: list[int] = []
        j: int = 0
        while j < n:
            if i == j:
                row.append(1)
            else:
                row.append(0)
            j = j + 1
        result.append(row)
        i = i + 1
    return result


def test_matrix_ops() -> int:
    """Test matrix operations."""
    total: int = 0
    a: list[list[int]] = [[1, 2], [3, 4]]
    b: list[list[int]] = [[5, 6], [7, 8]]
    c: list[list[int]] = matrix_multiply(a, b)
    if c == [[19, 22], [43, 50]]:
        total = total + 1
    t: list[list[int]] = matrix_transpose(a)
    if t == [[1, 3], [2, 4]]:
        total = total + 1
    det: int = matrix_determinant_2x2(a)
    if det == -2:
        total = total + 1
    m3: list[list[int]] = [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
    det3: int = matrix_determinant_3x3(m3)
    if det3 == 1:
        total = total + 1
    tr: int = matrix_trace(a)
    if tr == 5:
        total = total + 1
    ident: list[list[int]] = matrix_identity(3)
    if matrix_trace(ident) == 3:
        total = total + 1
    if matrix_determinant_3x3(ident) == 1:
        total = total + 1
    empty: list[list[int]] = matrix_multiply([], b)
    if empty == []:
        total = total + 1
    return total


# ============================================================
# Category 11: Polynomial evaluation
# ============================================================

def horner_eval(coeffs: list[int], x: int) -> int:
    """Evaluate polynomial using Horner's method.
    coeffs[0] is highest degree coefficient."""
    if len(coeffs) == 0:
        return 0
    result: int = coeffs[0]
    i: int = 1
    while i < len(coeffs):
        result = result * x + coeffs[i]
        i = i + 1
    return result


def poly_add(a: list[int], b: list[int]) -> list[int]:
    """Add two polynomials (index = degree)."""
    max_len: int = len(a) if len(a) > len(b) else len(b)
    result: list[int] = []
    i: int = 0
    while i < max_len:
        va: int = a[i] if i < len(a) else 0
        vb: int = b[i] if i < len(b) else 0
        result.append(va + vb)
        i = i + 1
    return result


def poly_multiply(a: list[int], b: list[int]) -> list[int]:
    """Multiply two polynomials (index = degree)."""
    if len(a) == 0 or len(b) == 0:
        return []
    result_len: int = len(a) + len(b) - 1
    result: list[int] = []
    i: int = 0
    while i < result_len:
        result.append(0)
        i = i + 1
    i = 0
    while i < len(a):
        j: int = 0
        while j < len(b):
            result[i + j] = result[i + j] + a[i] * b[j]
            j = j + 1
        i = i + 1
    return result


def poly_derivative(coeffs: list[int]) -> list[int]:
    """Derivative of polynomial (index = degree)."""
    if len(coeffs) <= 1:
        return [0]
    result: list[int] = []
    i: int = 1
    while i < len(coeffs):
        result.append(i * coeffs[i])
        i = i + 1
    return result


def test_polynomial() -> int:
    """Test polynomial operations."""
    total: int = 0
    val: int = horner_eval([1, -3, 2], 3)
    if val == 2:
        total = total + 1
    val2: int = horner_eval([1, 0, 0], 5)
    if val2 == 25:
        total = total + 1
    if horner_eval([], 5) == 0:
        total = total + 1
    s: list[int] = poly_add([1, 2, 3], [4, 5])
    if s == [5, 7, 3]:
        total = total + 1
    p: list[int] = poly_multiply([1, 1], [1, 1])
    if p == [1, 2, 1]:
        total = total + 1
    d: list[int] = poly_derivative([5, 3, 6])
    if d == [3, 12]:
        total = total + 1
    if poly_derivative([7]) == [0]:
        total = total + 1
    return total


# ============================================================
# Category 12: GCD chains and extended GCD
# ============================================================

def gcd(a: int, b: int) -> int:
    """Greatest common divisor."""
    if a < 0:
        a = -a
    if b < 0:
        b = -b
    while b != 0:
        t: int = b
        b = a % b
        a = t
    return a


def extended_gcd(a: int, b: int) -> list[int]:
    """Extended GCD returning [gcd, x, y] where a*x + b*y = gcd."""
    if a == 0:
        return [b, 0, 1]
    old_r: int = a
    r: int = b
    old_s: int = 1
    s: int = 0
    old_t: int = 0
    t: int = 1
    while r != 0:
        q: int = old_r // r
        tmp_r: int = r
        r = old_r - q * r
        old_r = tmp_r
        tmp_s: int = s
        s = old_s - q * s
        old_s = tmp_s
        tmp_t: int = t
        t = old_t - q * t
        old_t = tmp_t
    if old_r < 0:
        old_r = -old_r
        old_s = -old_s
        old_t = -old_t
    return [old_r, old_s, old_t]


def lcm(a: int, b: int) -> int:
    """Least common multiple."""
    if a == 0 or b == 0:
        return 0
    g: int = gcd(a, b)
    if g == 0:
        return 0
    va: int = a
    vb: int = b
    if va < 0:
        va = -va
    if vb < 0:
        vb = -vb
    return (va // g) * vb


def gcd_of_list(nums: list[int]) -> int:
    """GCD of a list of numbers."""
    if len(nums) == 0:
        return 0
    result: int = nums[0]
    i: int = 1
    while i < len(nums):
        result = gcd(result, nums[i])
        i = i + 1
    return result


def mod_inverse(a: int, m: int) -> int:
    """Modular multiplicative inverse using extended GCD.
    Returns -1 if no inverse exists."""
    if m <= 0:
        return -1
    res: list[int] = extended_gcd(a % m, m)
    if res[0] != 1:
        return -1
    inv: int = res[1] % m
    if inv < 0:
        inv = inv + m
    return inv


def test_gcd_extended() -> int:
    """Test GCD and extended GCD operations."""
    total: int = 0
    if gcd(12, 8) == 4:
        total = total + 1
    if gcd(0, 5) == 5:
        total = total + 1
    if gcd(7, 0) == 7:
        total = total + 1
    if gcd(-12, 8) == 4:
        total = total + 1
    eg: list[int] = extended_gcd(12, 8)
    if eg[0] == 4:
        total = total + 1
    verify: int = 12 * eg[1] + 8 * eg[2]
    if verify == 4:
        total = total + 1
    if lcm(4, 6) == 12:
        total = total + 1
    if lcm(0, 5) == 0:
        total = total + 1
    if gcd_of_list([12, 8, 4]) == 4:
        total = total + 1
    inv: int = mod_inverse(3, 7)
    if (3 * inv) % 7 == 1:
        total = total + 1
    if mod_inverse(2, 4) == -1:
        total = total + 1
    return total


# ============================================================
# Category 13: Chinese Remainder Theorem
# ============================================================

def crt_two(r1: int, m1: int, r2: int, m2: int) -> list[int]:
    """Chinese Remainder Theorem for two congruences.
    Returns [solution, combined_modulus] or [-1, 0] if no solution."""
    eg: list[int] = extended_gcd(m1, m2)
    g: int = eg[0]
    if g == 0:
        return [-1, 0]
    if (r2 - r1) % g != 0:
        return [-1, 0]
    combined: int = lcm(m1, m2)
    if combined == 0:
        return [-1, 0]
    diff: int = (r2 - r1) // g
    x: int = r1 + m1 * (diff * eg[1] % (m2 // g))
    x = x % combined
    if x < 0:
        x = x + combined
    return [x, combined]


def crt_list(remainders: list[int], moduli: list[int]) -> list[int]:
    """CRT for a list of congruences."""
    if len(remainders) == 0 or len(remainders) != len(moduli):
        return [-1, 0]
    cur_r: int = remainders[0]
    cur_m: int = moduli[0]
    i: int = 1
    while i < len(remainders):
        result: list[int] = crt_two(cur_r, cur_m, remainders[i], moduli[i])
        if result[1] == 0:
            return [-1, 0]
        cur_r = result[0]
        cur_m = result[1]
        i = i + 1
    return [cur_r, cur_m]


def test_crt() -> int:
    """Test Chinese Remainder Theorem."""
    total: int = 0
    r: list[int] = crt_two(2, 3, 3, 5)
    if r[0] == 8 and r[1] == 15:
        total = total + 1
    r2: list[int] = crt_two(0, 2, 0, 3)
    if r2[0] == 0 and r2[1] == 6:
        total = total + 1
    r3: list[int] = crt_list([2, 3, 2], [3, 5, 7])
    if r3[1] == 105:
        total = total + 1
    check: int = r3[0] % 3
    if check == 2:
        total = total + 1
    check2: int = r3[0] % 5
    if check2 == 3:
        total = total + 1
    check3: int = r3[0] % 7
    if check3 == 2:
        total = total + 1
    bad: list[int] = crt_two(0, 2, 1, 2)
    if bad[1] == 0:
        total = total + 1
    return total


# ============================================================
# Category 14: Integer logarithm
# ============================================================

def floor_log2(n: int) -> int:
    """Floor of log base 2."""
    if n <= 0:
        return -1
    result: int = 0
    val: int = n
    while val > 1:
        val = val >> 1
        result = result + 1
    return result


def floor_log10(n: int) -> int:
    """Floor of log base 10."""
    if n <= 0:
        return -1
    result: int = 0
    val: int = n
    while val >= 10:
        val = val // 10
        result = result + 1
    return result


def floor_log_base(n: int, base: int) -> int:
    """Floor of log base 'base'."""
    if n <= 0 or base <= 1:
        return -1
    result: int = 0
    val: int = n
    while val >= base:
        val = val // base
        result = result + 1
    return result


def is_perfect_power(n: int) -> int:
    """Check if n is a perfect power (n = a^b for b >= 2). 1=yes, 0=no."""
    if n <= 1:
        return 0
    b: int = 2
    while b <= 40:
        a: int = integer_nth_root(n, b)
        power: int = 1
        i: int = 0
        while i < b:
            power = power * a
            i = i + 1
        if power == n:
            return 1
        b = b + 1
    return 0


def test_integer_log() -> int:
    """Test integer logarithm operations."""
    total: int = 0
    if floor_log2(1) == 0:
        total = total + 1
    if floor_log2(2) == 1:
        total = total + 1
    if floor_log2(7) == 2:
        total = total + 1
    if floor_log2(8) == 3:
        total = total + 1
    if floor_log2(0) == -1:
        total = total + 1
    if floor_log10(1) == 0:
        total = total + 1
    if floor_log10(99) == 1:
        total = total + 1
    if floor_log10(100) == 2:
        total = total + 1
    if floor_log_base(8, 2) == 3:
        total = total + 1
    if floor_log_base(27, 3) == 3:
        total = total + 1
    if is_perfect_power(8) == 1:
        total = total + 1
    if is_perfect_power(10) == 0:
        total = total + 1
    if is_perfect_power(1) == 0:
        total = total + 1
    return total


# ============================================================
# Category 15: Power mod (modular exponentiation)
# ============================================================

def power_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation: (base^exp) % mod."""
    if mod <= 0:
        return 0
    if mod == 1:
        return 0
    if exp < 0:
        return 0
    result: int = 1
    b: int = base % mod
    if b < 0:
        b = b + mod
    e: int = exp
    while e > 0:
        if e & 1 == 1:
            result = (result * b) % mod
        e = e >> 1
        b = (b * b) % mod
    return result


def is_probable_prime_fermat(n: int) -> int:
    """Fermat primality test. 1=probably prime, 0=composite."""
    if n < 2:
        return 0
    if n == 2 or n == 3:
        return 1
    if n % 2 == 0:
        return 0
    bases: list[int] = [2, 3, 5, 7, 11, 13]
    i: int = 0
    while i < len(bases):
        a: int = bases[i]
        if a >= n:
            i = i + 1
            continue
        if power_mod(a, n - 1, n) != 1:
            return 0
        i = i + 1
    return 1


def discrete_log_brute(base: int, target: int, mod: int) -> int:
    """Discrete log by brute force: find x such that base^x = target (mod mod).
    Returns -1 if not found within mod steps."""
    if mod <= 0:
        return -1
    val: int = 1
    x: int = 0
    while x < mod:
        if val % mod == target % mod:
            return x
        val = (val * base) % mod
        x = x + 1
    return -1


def euler_totient(n: int) -> int:
    """Euler's totient function."""
    if n <= 0:
        return 0
    if n == 1:
        return 1
    result: int = n
    p: int = 2
    temp: int = n
    while p * p <= temp:
        if temp % p == 0:
            while temp % p == 0:
                temp = temp // p
            result = result - result // p
        p = p + 1
    if temp > 1:
        result = result - result // temp
    return result


def sum_of_powers_mod(n: int, k: int, mod: int) -> int:
    """Compute (1^k + 2^k + ... + n^k) % mod."""
    if n <= 0 or mod <= 0:
        return 0
    result: int = 0
    i: int = 1
    while i <= n:
        result = (result + power_mod(i, k, mod)) % mod
        i = i + 1
    return result


def test_power_mod() -> int:
    """Test modular exponentiation operations."""
    total: int = 0
    if power_mod(2, 10, 1000) == 24:
        total = total + 1
    if power_mod(2, 10, 1024) == 0:
        total = total + 1
    if power_mod(3, 0, 7) == 1:
        total = total + 1
    if power_mod(5, 3, 13) == 8:
        total = total + 1
    if power_mod(2, 10, 1) == 0:
        total = total + 1
    if is_probable_prime_fermat(2) == 1:
        total = total + 1
    if is_probable_prime_fermat(17) == 1:
        total = total + 1
    if is_probable_prime_fermat(4) == 0:
        total = total + 1
    if is_probable_prime_fermat(1) == 0:
        total = total + 1
    dl: int = discrete_log_brute(2, 8, 13)
    if power_mod(2, dl, 13) == 8:
        total = total + 1
    if euler_totient(1) == 1:
        total = total + 1
    if euler_totient(12) == 4:
        total = total + 1
    sp: int = sum_of_powers_mod(5, 2, 1000)
    if sp == 55:
        total = total + 1
    return total


# ============================================================
# Bonus: Additional pathological patterns
# ============================================================

def karatsuba_multiply(x: int, y: int) -> int:
    """Simplified Karatsuba multiplication for pedagogical purposes."""
    if x < 10 or y < 10:
        return x * y
    n: int = count_digits_in_base(x, 10)
    m: int = count_digits_in_base(y, 10)
    half: int = (n if n < m else m) // 2
    if half == 0:
        return x * y
    power: int = 1
    i: int = 0
    while i < half:
        power = power * 10
        i = i + 1
    high_x: int = x // power
    low_x: int = x % power
    high_y: int = y // power
    low_y: int = y % power
    z0: int = low_x * low_y
    z2: int = high_x * high_y
    z1: int = (low_x + high_x) * (low_y + high_y) - z2 - z0
    return z2 * power * power + z1 * power + z0


def sieve_count_primes(limit: int) -> int:
    """Count primes up to limit using a sieve."""
    if limit < 2:
        return 0
    is_prime: list[int] = []
    i: int = 0
    while i <= limit:
        is_prime.append(1)
        i = i + 1
    is_prime[0] = 0
    is_prime[1] = 0
    p: int = 2
    while p * p <= limit:
        if is_prime[p] == 1:
            j: int = p * p
            while j <= limit:
                is_prime[j] = 0
                j = j + p
        p = p + 1
    count: int = 0
    k: int = 0
    while k <= limit:
        count = count + is_prime[k]
        k = k + 1
    return count


def collatz_length(n: int) -> int:
    """Length of Collatz sequence starting from n."""
    if n <= 0:
        return 0
    steps: int = 0
    val: int = n
    while val != 1:
        if val % 2 == 0:
            val = val // 2
        else:
            val = 3 * val + 1
        steps = steps + 1
        if steps > 10000:
            return -1
    return steps


def ackermann_bounded(m: int, n: int, limit: int) -> int:
    """Bounded Ackermann function to prevent stack overflow."""
    if limit <= 0:
        return -1
    if m == 0:
        return n + 1
    if n == 0:
        return ackermann_bounded(m - 1, 1, limit - 1)
    inner: int = ackermann_bounded(m, n - 1, limit - 1)
    if inner == -1:
        return -1
    return ackermann_bounded(m - 1, inner, limit - 1)


def fibonacci_mod(n: int, mod: int) -> int:
    """Compute n-th Fibonacci number modulo mod."""
    if n <= 0 or mod <= 0:
        return 0
    if n == 1 or n == 2:
        return 1 % mod
    a: int = 1
    b: int = 1
    i: int = 3
    while i <= n:
        c: int = (a + b) % mod
        a = b
        b = c
        i = i + 1
    return b


def test_bonus_patterns() -> int:
    """Test bonus pathological patterns."""
    total: int = 0
    if karatsuba_multiply(12, 34) == 408:
        total = total + 1
    if karatsuba_multiply(0, 100) == 0:
        total = total + 1
    if karatsuba_multiply(999, 999) == 998001:
        total = total + 1
    if sieve_count_primes(10) == 4:
        total = total + 1
    if sieve_count_primes(1) == 0:
        total = total + 1
    if sieve_count_primes(100) == 25:
        total = total + 1
    if collatz_length(1) == 0:
        total = total + 1
    if collatz_length(6) == 8:
        total = total + 1
    if collatz_length(0) == 0:
        total = total + 1
    if ackermann_bounded(0, 0, 100) == 1:
        total = total + 1
    if ackermann_bounded(1, 1, 100) == 3:
        total = total + 1
    if ackermann_bounded(2, 2, 200) == 7:
        total = total + 1
    if fibonacci_mod(10, 1000) == 55:
        total = total + 1
    if fibonacci_mod(1, 100) == 1:
        total = total + 1
    return total


# ============================================================
# Master test runner
# ============================================================

def run_all_tests() -> int:
    """Run all tests and return sum of passing tests."""
    total: int = 0
    total = total + test_fixed_point()
    total = total + test_bit_manipulation()
    total = total + test_overflow_detection()
    total = total + test_division_edge_cases()
    total = total + test_bignum()
    total = total + test_base_conversion()
    total = total + test_digit_parsing()
    total = total + test_continued_fractions()
    total = total + test_newton_methods()
    total = total + test_matrix_ops()
    total = total + test_polynomial()
    total = total + test_gcd_extended()
    total = total + test_crt()
    total = total + test_power_mod()
    total = total + test_bonus_patterns()
    return total


if __name__ == "__main__":
    result: int = run_all_tests()
    assert result > 0
