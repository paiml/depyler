"""General linear recurrence relations."""


def fibonacci(n: int) -> int:
    """Compute nth Fibonacci number (0-indexed: fib(0)=0, fib(1)=1)."""
    if n <= 0:
        return 0
    if n == 1:
        return 1
    a: int = 0
    b: int = 1
    idx: int = 2
    while idx <= n:
        temp: int = a + b
        a = b
        b = temp
        idx = idx + 1
    return b


def tribonacci(n: int) -> int:
    """Compute nth Tribonacci number: T(0)=0, T(1)=0, T(2)=1."""
    if n <= 1:
        return 0
    if n == 2:
        return 1
    a: int = 0
    b: int = 0
    c: int = 1
    idx: int = 3
    while idx <= n:
        temp: int = a + b + c
        a = b
        b = c
        c = temp
        idx = idx + 1
    return c


def linear_recurrence(coeffs: list[int], initial: list[int], n: int) -> int:
    """Compute nth term of linear recurrence with given coefficients.
    coeffs[0] multiplies the most recent term, coeffs[1] the second most recent, etc.
    initial contains the first len(coeffs) terms."""
    order: int = len(coeffs)
    if n < order:
        return initial[n]
    values: list[int] = []
    vi: int = 0
    while vi < order:
        values.append(initial[vi])
        vi = vi + 1
    idx: int = order
    while idx <= n:
        new_val: int = 0
        ci: int = 0
        val_len: int = len(values)
        while ci < order:
            back_pos: int = val_len - 1 - ci
            new_val = new_val + coeffs[ci] * values[back_pos]
            ci = ci + 1
        values.append(new_val)
        idx = idx + 1
    return values[n]


def padovan(n: int) -> int:
    """Compute nth Padovan number: P(0)=P(1)=P(2)=1, P(n)=P(n-2)+P(n-3)."""
    if n <= 2:
        return 1
    a: int = 1
    b: int = 1
    c: int = 1
    idx: int = 3
    while idx <= n:
        temp: int = a + b
        a = b
        b = c
        c = temp
        idx = idx + 1
    return c


def test_module() -> int:
    passed: int = 0

    if fibonacci(10) == 55:
        passed = passed + 1
    if fibonacci(0) == 0:
        passed = passed + 1
    if tribonacci(7) == 13:
        passed = passed + 1

    fib_val: int = linear_recurrence([1, 1], [0, 1], 10)
    if fib_val == 55:
        passed = passed + 1

    if padovan(5) == 3:
        passed = passed + 1
    if padovan(8) == 7:
        passed = passed + 1

    if tribonacci(2) == 1:
        passed = passed + 1

    return passed
