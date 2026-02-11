"""Evaluate arithmetic expressions represented as token arrays."""


def eval_add_sub(values: list[int], operations: list[int]) -> int:
    """Evaluate a sequence of additions and subtractions.
    operations: 1=add, 2=subtract."""
    if len(values) == 0:
        return 0
    result: int = values[0]
    i: int = 0
    while i < len(operations):
        val_idx: int = i + 1
        if operations[i] == 1:
            result = result + values[val_idx]
        elif operations[i] == 2:
            result = result - values[val_idx]
        i = i + 1
    return result


def eval_mul_div_first(values: list[int], operations: list[int]) -> int:
    """Evaluate with proper precedence: mul/div before add/sub.
    operations: 1=add, 2=sub, 3=mul, 4=div."""
    if len(values) == 0:
        return 0
    # First pass: handle mul/div
    reduced_vals: list[int] = [values[0]]
    reduced_ops: list[int] = []
    i: int = 0
    while i < len(operations):
        val_idx: int = i + 1
        if operations[i] == 3:
            last_idx: int = len(reduced_vals) - 1
            product: int = reduced_vals[last_idx] * values[val_idx]
            reduced_vals[last_idx] = product
        elif operations[i] == 4:
            last_idx2: int = len(reduced_vals) - 1
            if values[val_idx] != 0:
                quotient: int = reduced_vals[last_idx2] // values[val_idx]
                reduced_vals[last_idx2] = quotient
        else:
            reduced_vals.append(values[val_idx])
            reduced_ops.append(operations[i])
        i = i + 1
    # Second pass: handle add/sub
    result: int = eval_add_sub(reduced_vals, reduced_ops)
    return result


def polynomial_eval(coeffs: list[int], x: int) -> int:
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


def repeated_squaring(base: int, exp: int, modulus: int) -> int:
    """Compute base^exp mod modulus using repeated squaring."""
    if modulus <= 0:
        return 0
    if exp == 0:
        return 1 % modulus
    result: int = 1
    b: int = base % modulus
    e: int = exp
    while e > 0:
        if e % 2 == 1:
            result = (result * b) % modulus
        b = (b * b) % modulus
        e = e // 2
    return result


def test_module() -> int:
    """Test arithmetic evaluation functions."""
    ok: int = 0

    vals1: list[int] = [10, 3, 2]
    ops1: list[int] = [1, 2]
    if eval_add_sub(vals1, ops1) == 11:
        ok = ok + 1

    # 2 + 3 * 4 = 14
    vals2: list[int] = [2, 3, 4]
    ops2: list[int] = [1, 3]
    if eval_mul_div_first(vals2, ops2) == 14:
        ok = ok + 1

    # 3x^2 + 2x + 1 at x=2 => 17
    coeffs: list[int] = [3, 2, 1]
    if polynomial_eval(coeffs, 2) == 17:
        ok = ok + 1

    if polynomial_eval(coeffs, 0) == 1:
        ok = ok + 1

    if repeated_squaring(2, 10, 1000) == 24:
        ok = ok + 1

    if repeated_squaring(3, 0, 7) == 1:
        ok = ok + 1

    return ok
