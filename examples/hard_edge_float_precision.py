"""Float comparison with epsilon, rounding, and precision edge cases."""


def float_approx_equal(a: float, b: float, eps: float) -> int:
    """Return 1 if a and b are within eps of each other."""
    diff: float = a - b
    if diff < 0.0:
        diff = 0.0 - diff
    if diff <= eps:
        return 1
    return 0


def float_floor_int(x: float) -> int:
    """Floor of float to int (positive values only)."""
    truncated: int = int(x)
    if x >= 0.0:
        return truncated
    f: float = float(truncated)
    if f > x:
        return truncated - 1
    return truncated


def float_ceil_int(x: float) -> int:
    """Ceiling of float to int."""
    truncated: int = int(x)
    f: float = float(truncated)
    if x > f:
        return truncated + 1
    return truncated


def float_round_int(x: float) -> int:
    """Round float to nearest int."""
    if x >= 0.0:
        return int(x + 0.5)
    return int(x - 0.5)


def float_abs(x: float) -> float:
    """Absolute value of float."""
    if x < 0.0:
        return 0.0 - x
    return x


def float_max(a: float, b: float) -> float:
    """Maximum of two floats."""
    if a >= b:
        return a
    return b


def float_min(a: float, b: float) -> float:
    """Minimum of two floats."""
    if a <= b:
        return a
    return b


def float_clamp(val: float, lo: float, hi: float) -> float:
    """Clamp float to range [lo, hi]."""
    if val < lo:
        return lo
    if val > hi:
        return hi
    return val


def lerp(a: float, b: float, t: float) -> float:
    """Linear interpolation between a and b by factor t."""
    return a + (b - a) * t


def inverse_lerp(a: float, b: float, val: float) -> float:
    """Inverse linear interpolation: find t such that lerp(a,b,t) = val."""
    diff: float = b - a
    if float_abs(diff) < 0.000001:
        return 0.0
    return (val - a) / diff


def sum_float_list(arr: list[float]) -> float:
    """Sum a list of floats using Kahan summation for better precision."""
    total: float = 0.0
    compensation: float = 0.0
    i: int = 0
    while i < len(arr):
        y: float = arr[i] - compensation
        t: float = total + y
        compensation = (t - total) - y
        total = t
        i = i + 1
    return total


def mean_float_list(arr: list[float]) -> float:
    """Compute mean of float list."""
    n: int = len(arr)
    if n == 0:
        return 0.0
    total: float = sum_float_list(arr)
    return total / float(n)


def test_module() -> int:
    """Test all float precision functions."""
    passed: int = 0
    if float_approx_equal(1.0, 1.0000001, 0.001) == 1:
        passed = passed + 1
    if float_approx_equal(1.0, 2.0, 0.001) == 0:
        passed = passed + 1
    if float_floor_int(3.7) == 3:
        passed = passed + 1
    if float_ceil_int(3.2) == 4:
        passed = passed + 1
    if float_ceil_int(3.0) == 3:
        passed = passed + 1
    if float_round_int(3.5) == 4:
        passed = passed + 1
    if float_round_int(3.4) == 3:
        passed = passed + 1
    r1: float = float_abs(0.0 - 5.5)
    if float_approx_equal(r1, 5.5, 0.01) == 1:
        passed = passed + 1
    r2: float = float_clamp(10.0, 0.0, 5.0)
    if float_approx_equal(r2, 5.0, 0.01) == 1:
        passed = passed + 1
    r3: float = lerp(0.0, 10.0, 0.5)
    if float_approx_equal(r3, 5.0, 0.01) == 1:
        passed = passed + 1
    r4: float = inverse_lerp(0.0, 10.0, 5.0)
    if float_approx_equal(r4, 0.5, 0.01) == 1:
        passed = passed + 1
    vals: list[float] = [0.1, 0.2, 0.3]
    r5: float = sum_float_list(vals)
    if float_approx_equal(r5, 0.6, 0.01) == 1:
        passed = passed + 1
    r6: float = mean_float_list(vals)
    if float_approx_equal(r6, 0.2, 0.01) == 1:
        passed = passed + 1
    return passed


if __name__ == "__main__":
    print(test_module())
