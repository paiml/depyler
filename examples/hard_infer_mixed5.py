# Type inference test: Complex inference chains
# Strategy: Mix of typed/untyped with multi-level inference


def typed_square(x: int) -> int:
    """Fully typed anchor function."""
    return x * x


def typed_add(a: int, b: int) -> int:
    """Fully typed anchor function."""
    return a + b


def untyped_double(x):
    """Inferred from int operations."""
    return x + x


def untyped_abs(x):
    """Inferred from comparisons."""
    if x < 0:
        return 0 - x
    return x


def chain_level1(x):
    """Calls typed functions, return inferred."""
    sq = typed_square(x)
    return typed_add(sq, x)


def chain_level2(x):
    """Calls chain_level1, type propagates."""
    v1 = chain_level1(x)
    v2 = chain_level1(x + 1)
    return v1 + v2


def chain_level3(x):
    """Calls chain_level2, deeper propagation."""
    return chain_level2(x) + chain_level2(x + 1)


def multi_path(x, y):
    """Multiple inference paths through typed functions."""
    path_a = typed_square(x)
    path_b = typed_square(y)
    combined = typed_add(path_a, path_b)
    doubled = untyped_double(combined)
    return doubled


def conditional_chain(x, threshold):
    """Type inferred through conditional paths."""
    if x > threshold:
        result = typed_square(x)
    else:
        result = typed_add(x, threshold)
    return result


def loop_accumulate_chain(n):
    """Type inferred through loop with typed function calls."""
    total = 0
    i = 1
    while i <= n:
        val = typed_square(i)
        total = typed_add(total, val)
        i = i + 1
    return total


def nested_untyped(a, b, c):
    """Multiple untyped params flowing through typed ops."""
    r1 = typed_add(a, b)
    r2 = typed_add(r1, c)
    r3 = untyped_double(r2)
    return untyped_abs(r3)


def test_module() -> int:
    """Test complex inference chains."""
    total: int = 0

    # chain_level1: x^2 + x
    if chain_level1(3) == 12:
        total = total + 1
    if chain_level1(0) == 0:
        total = total + 1

    # chain_level2: (x^2+x) + ((x+1)^2+(x+1))
    if chain_level2(2) == 18:
        total = total + 1

    # chain_level3: chain_level2(x) + chain_level2(x+1)
    r3: int = chain_level3(1)
    if r3 > 0:
        total = total + 1

    # multi_path: 2 * (x^2 + y^2)
    if multi_path(3, 4) == 50:
        total = total + 1

    # conditional_chain
    if conditional_chain(10, 5) == 100:
        total = total + 1
    if conditional_chain(3, 5) == 8:
        total = total + 1

    # loop_accumulate_chain: sum of squares 1..n
    if loop_accumulate_chain(3) == 14:
        total = total + 1
    if loop_accumulate_chain(0) == 0:
        total = total + 1

    # nested_untyped
    if nested_untyped(1, 2, 3) == 12:
        total = total + 1

    # untyped_double
    if untyped_double(7) == 14:
        total = total + 1

    # untyped_abs
    if untyped_abs(0 - 5) == 5:
        total = total + 1

    return total
