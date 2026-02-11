# Type inference test: Complex multi-function chains
# Strategy: Deep inference chains with no annotations except test_module


def op_add(a, b):
    """Basic add - inferred from int usage."""
    return a + b


def op_mul(a, b):
    """Basic multiply - inferred from int usage."""
    return a * b


def op_mod(a, b):
    """Basic modulo - inferred from int usage."""
    if b == 0:
        return 0
    return a % b


def build_sequence(start, step, count):
    """Build a sequence and return sum. All types inferred."""
    total = 0
    current = start
    i = 0
    while i < count:
        total = op_add(total, current)
        current = op_add(current, step)
        i = i + 1
    return total


def fibonacci_via_ops(n):
    """Fibonacci using op_add for addition."""
    if n <= 0:
        return 0
    if n == 1:
        return 1
    prev = 0
    curr = 1
    i = 2
    while i <= n:
        temp = curr
        curr = op_add(prev, curr)
        prev = temp
        i = i + 1
    return curr


def power_via_ops(num, exp):
    """Power using op_mul for multiplication."""
    if exp <= 0:
        return 1
    result = 1
    i = 0
    while i < exp:
        result = op_mul(result, num)
        i = i + 1
    return result


def factorial_via_ops(n):
    """Factorial using op_mul."""
    result = 1
    i = 2
    while i <= n:
        result = op_mul(result, i)
        i = i + 1
    return result


def sum_of_powers(n, exp):
    """Sum of i^exp for i=1..n using power_via_ops."""
    total = 0
    i = 1
    while i <= n:
        p = power_via_ops(i, exp)
        total = op_add(total, p)
        i = i + 1
    return total


def gcd_via_ops(a, b):
    """GCD using op_mod."""
    va = a
    vb = b
    if va < 0:
        va = 0 - va
    if vb < 0:
        vb = 0 - vb
    while vb != 0:
        temp = vb
        vb = op_mod(va, vb)
        va = temp
    return va


def catalan_number(n):
    """Compute n-th Catalan number using factorials.
    C(n) = (2n)! / ((n+1)! * n!)"""
    if n <= 0:
        return 1
    numerator = factorial_via_ops(op_mul(2, n))
    denom1 = factorial_via_ops(op_add(n, 1))
    denom2 = factorial_via_ops(n)
    denominator = op_mul(denom1, denom2)
    if denominator == 0:
        return 0
    return numerator // denominator


def composition_chain(x):
    """Chain: fib(x) -> power by 2 -> mod 1000."""
    f = fibonacci_via_ops(x)
    p = power_via_ops(f, 2)
    return op_mod(p, 1000)


def test_module() -> int:
    """Test complex multi-function chains."""
    total: int = 0

    # build_sequence: arithmetic series sum
    # start=1, step=2, count=5: 1+3+5+7+9=25
    if build_sequence(1, 2, 5) == 25:
        total = total + 1

    # fibonacci_via_ops
    if fibonacci_via_ops(10) == 55:
        total = total + 1
    if fibonacci_via_ops(0) == 0:
        total = total + 1

    # power_via_ops
    if power_via_ops(2, 10) == 1024:
        total = total + 1
    if power_via_ops(3, 0) == 1:
        total = total + 1

    # factorial_via_ops
    if factorial_via_ops(5) == 120:
        total = total + 1
    if factorial_via_ops(0) == 1:
        total = total + 1

    # sum_of_powers: 1^2 + 2^2 + 3^2 = 14
    if sum_of_powers(3, 2) == 14:
        total = total + 1

    # gcd_via_ops
    if gcd_via_ops(12, 8) == 4:
        total = total + 1
    if gcd_via_ops(17, 13) == 1:
        total = total + 1

    # catalan_number: C(0)=1, C(1)=1, C(2)=2, C(3)=5, C(4)=14
    if catalan_number(0) == 1:
        total = total + 1
    if catalan_number(3) == 5:
        total = total + 1

    # composition_chain: fib(8)=21, 21^2=441, 441%1000=441
    if composition_chain(8) == 441:
        total = total + 1

    return total
