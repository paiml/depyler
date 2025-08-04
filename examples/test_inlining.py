"""
Example demonstrating function inlining optimizations in Depyler.
"""

def square(x: int) -> int:
    """Trivial function - should be inlined."""
    return x * x


def add_one(n: int) -> int:
    """Another trivial function."""
    return n + 1


def compute_distance_squared(x1: int, y1: int, x2: int, y2: int) -> int:
    """Should inline the square calls."""
    dx = x2 - x1
    dy = y2 - y1
    return square(dx) + square(dy)


def process_single_use(value: int) -> int:
    """Called only once - should be inlined."""
    temp = value * 2
    result = temp + 10
    return result


def main_computation(a: int, b: int) -> int:
    """Main function that uses other functions."""
    # This should inline process_single_use
    step1 = process_single_use(a)
    
    # These should inline add_one
    step2 = add_one(step1)
    step3 = add_one(b)
    
    # This should inline square calls
    distance = compute_distance_squared(0, 0, step2, step3)
    
    return distance


def recursive_factorial(n: int) -> int:
    """Recursive function - should NOT be inlined."""
    if n <= 1:
        return 1
    return n * recursive_factorial(n - 1)


def has_loop(items: list) -> int:
    """Contains loop - may not be inlined depending on config."""
    total = 0
    for item in items:
        total = total + item
    return total


def large_function(x: int, y: int, z: int) -> int:
    """Large function - should NOT be inlined."""
    a = x + y
    b = y + z
    c = z + x
    d = a * b
    e = b * c
    f = c * a
    g = d + e
    h = e + f
    i = f + d
    j = g * h
    k = h * i
    l = i * g
    m = j + k
    n = k + l
    o = l + j
    result = m + n + o
    return result