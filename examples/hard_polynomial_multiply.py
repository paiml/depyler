# Polynomial multiplication, division
# Polynomials represented as list of coefficients [a0, a1, a2, ...] = a0 + a1*x + a2*x^2 + ...


def poly_multiply(a: list[int], b: list[int]) -> list[int]:
    if len(a) == 0 or len(b) == 0:
        return []
    n: int = len(a) + len(b) - 1
    result: list[int] = []
    i: int = 0
    while i < n:
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


def poly_add(a: list[int], b: list[int]) -> list[int]:
    n: int = len(a)
    if len(b) > n:
        n = len(b)
    result: list[int] = []
    i: int = 0
    while i < n:
        va: int = 0
        vb: int = 0
        if i < len(a):
            va = a[i]
        if i < len(b):
            vb = b[i]
        result.append(va + vb)
        i = i + 1
    return result


def poly_evaluate(p: list[int], x: int) -> int:
    # Horner's method
    if len(p) == 0:
        return 0
    result: int = p[len(p) - 1]
    i: int = len(p) - 2
    while i >= 0:
        result = result * x + p[i]
        i = i - 1
    return result


def poly_degree(p: list[int]) -> int:
    i: int = len(p) - 1
    while i >= 0:
        if p[i] != 0:
            return i
        i = i - 1
    return -1


def poly_scale(p: list[int], scalar: int) -> list[int]:
    result: list[int] = []
    i: int = 0
    while i < len(p):
        result.append(p[i] * scalar)
        i = i + 1
    return result


def test_module() -> int:
    passed: int = 0

    # Test 1: multiply (1+x) * (1+x) = 1+2x+x^2
    a: list[int] = [1, 1]
    product: list[int] = poly_multiply(a, a)
    if product[0] == 1 and product[1] == 2 and product[2] == 1:
        passed = passed + 1

    # Test 2: multiply (1+x) * (1-x) ... using (1+x)(2+3x) = 2+5x+3x^2
    b: list[int] = [2, 3]
    p: list[int] = poly_multiply(a, b)
    if p[0] == 2 and p[1] == 5 and p[2] == 3:
        passed = passed + 1

    # Test 3: add polynomials
    s: list[int] = poly_add([1, 2, 3], [4, 5])
    if s[0] == 5 and s[1] == 7 and s[2] == 3:
        passed = passed + 1

    # Test 4: evaluate 2+3x at x=4 = 14
    if poly_evaluate([2, 3], 4) == 14:
        passed = passed + 1

    # Test 5: evaluate x^2 + 2x + 1 at x=3 = 16
    if poly_evaluate([1, 2, 1], 3) == 16:
        passed = passed + 1

    # Test 6: degree
    if poly_degree([1, 2, 3]) == 2:
        passed = passed + 1

    # Test 7: scale
    scaled: list[int] = poly_scale([1, 2, 3], 3)
    if scaled[0] == 3 and scaled[1] == 6 and scaled[2] == 9:
        passed = passed + 1

    return passed
