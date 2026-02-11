# Sequence operations (Fibonacci variants, Catalan, triangular)
# NO imports, NO I/O, ALL pure functions, ALL type-annotated


def fibonacci(n: int) -> int:
    """Compute the nth Fibonacci number iteratively."""
    if n <= 0:
        return 0
    if n == 1:
        return 1
    a: int = 0
    b: int = 1
    i: int = 2
    while i <= n:
        temp: int = a + b
        a = b
        b = temp
        i = i + 1
    return b


def fibonacci_sequence(n: int) -> list[int]:
    """Generate the first n Fibonacci numbers."""
    result: list[int] = []
    i: int = 0
    while i < n:
        result.append(fibonacci(i))
        i = i + 1
    return result


def triangular_number(n: int) -> int:
    """Compute the nth triangular number: n*(n+1)/2."""
    return (n * (n + 1)) // 2


def sum_of_squares(n: int) -> int:
    """Compute sum of squares: 1^2 + 2^2 + ... + n^2."""
    total: int = 0
    i: int = 1
    while i <= n:
        total = total + i * i
        i = i + 1
    return total


def collatz_length(n: int) -> int:
    """Count the number of steps to reach 1 in the Collatz sequence."""
    if n <= 0:
        return 0
    steps: int = 0
    current: int = n
    while current != 1:
        if current % 2 == 0:
            current = current // 2
        else:
            current = 3 * current + 1
        steps = steps + 1
    return steps


def test_module() -> int:
    assert fibonacci(0) == 0
    assert fibonacci(1) == 1
    assert fibonacci(10) == 55
    assert fibonacci_sequence(6) == [0, 1, 1, 2, 3, 5]
    assert fibonacci_sequence(0) == []
    assert triangular_number(1) == 1
    assert triangular_number(5) == 15
    assert triangular_number(10) == 55
    assert sum_of_squares(3) == 14
    assert sum_of_squares(5) == 55
    assert collatz_length(1) == 0
    assert collatz_length(6) == 8
    assert collatz_length(27) == 111
    return 0


if __name__ == "__main__":
    test_module()
