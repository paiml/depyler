# Pathological mixed: Complex mathematical formulas with string results
# Tests: computing math, formatting results, multi-step string+numeric pipeline


def compute_polynomial(x: int, coeffs: list[int]) -> int:
    """Evaluate polynomial: coeffs[0]*x^(n-1) + coeffs[1]*x^(n-2) + ... + coeffs[n-1].
    Uses Horner's method."""
    if len(coeffs) == 0:
        return 0
    result: int = coeffs[0]
    i: int = 1
    while i < len(coeffs):
        result = result * x + coeffs[i]
        i = i + 1
    return result


def integer_sqrt(n: int) -> int:
    """Integer square root using Newton's method."""
    if n < 0:
        return 0 - 1
    if n == 0:
        return 0
    guess: int = n
    while guess * guess > n:
        guess = (guess + n // guess) // 2
    return guess


def format_polynomial(coeffs: list[int]) -> str:
    """Format polynomial as string: '3x^2 + 2x + 1'."""
    if len(coeffs) == 0:
        return "0"
    degree: int = len(coeffs) - 1
    result: str = ""
    i: int = 0
    while i < len(coeffs):
        coeff: int = coeffs[i]
        curr_deg: int = degree - i
        if coeff == 0:
            i = i + 1
            continue
        # Add separator
        if len(result) > 0:
            if coeff > 0:
                result = result + " + "
            else:
                result = result + " - "
                coeff = 0 - coeff
        elif coeff < 0:
            result = result + "-"
            coeff = 0 - coeff
        # Add term
        if curr_deg == 0:
            result = result + str(coeff)
        elif curr_deg == 1:
            if coeff == 1:
                result = result + "x"
            else:
                result = result + str(coeff) + "x"
        else:
            if coeff == 1:
                result = result + "x^" + str(curr_deg)
            else:
                result = result + str(coeff) + "x^" + str(curr_deg)
        i = i + 1
    if len(result) == 0:
        return "0"
    return result


def describe_roots_quadratic(a: int, b: int, c: int) -> str:
    """Describe nature of roots of ax^2 + bx + c using discriminant."""
    disc: int = b * b - 4 * a * c
    if disc > 0:
        sqrt_disc: int = integer_sqrt(disc)
        if sqrt_disc * sqrt_disc == disc:
            return "two rational roots, discriminant=" + str(disc)
        return "two irrational roots, discriminant=" + str(disc)
    if disc == 0:
        return "one repeated root, discriminant=0"
    return "two complex roots, discriminant=" + str(disc)


def matrix_trace_str(flat: list[int], size: int) -> str:
    """Compute trace of square matrix and format as string."""
    trace: int = 0
    i: int = 0
    while i < size:
        idx: int = i * size + i
        trace = trace + flat[idx]
        i = i + 1
    return "trace=" + str(trace) + " size=" + str(size)


def number_properties(n: int) -> str:
    """Return string describing properties of number."""
    props: str = str(n) + ":"
    if n % 2 == 0:
        props = props + " even"
    else:
        props = props + " odd"
    # Check perfect square
    sq: int = integer_sqrt(n)
    if sq * sq == n:
        props = props + " perfect_square"
    # Check if prime (simple)
    if n > 1:
        is_prime: bool = True
        d: int = 2
        while d * d <= n:
            if n % d == 0:
                is_prime = False
                break
            d = d + 1
        if is_prime == True:
            props = props + " prime"
    return props


def test_module() -> int:
    passed: int = 0
    # Test 1: polynomial eval (3x^2 + 2x + 1 at x=2 -> 17)
    if compute_polynomial(2, [3, 2, 1]) == 17:
        passed = passed + 1
    # Test 2: integer sqrt
    if integer_sqrt(16) == 4:
        passed = passed + 1
    # Test 3: format polynomial
    if format_polynomial([3, 2, 1]) == "3x^2 + 2x + 1":
        passed = passed + 1
    # Test 4: quadratic roots (x^2 - 5x + 6, disc=1)
    desc: str = describe_roots_quadratic(1, 0 - 5, 6)
    if desc == "two rational roots, discriminant=1":
        passed = passed + 1
    # Test 5: matrix trace (2x2: [[1,2],[3,4]] -> trace=5)
    if matrix_trace_str([1, 2, 3, 4], 2) == "trace=5 size=2":
        passed = passed + 1
    # Test 6: number properties
    props: str = number_properties(4)
    if props == "4: even perfect_square":
        passed = passed + 1
    # Test 7: prime properties
    props2: str = number_properties(7)
    if props2 == "7: odd prime":
        passed = passed + 1
    return passed
