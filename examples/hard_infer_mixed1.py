# Type inference test: Mixed typed/untyped functions
# Strategy: Some functions fully typed, some untyped, testing cross-function inference


def clamp_typed(val: int, low: int, high: int) -> int:
    """Fully typed: clamp value to range [low, high]."""
    if val < low:
        return low
    if val > high:
        return high
    return val


def clamp_untyped(val, low, high):
    """Untyped: same logic, types inferred from call site."""
    if val < low:
        return low
    if val > high:
        return high
    return val


def map_range_typed(val: int, in_low: int, in_high: int, out_low: int, out_high: int) -> int:
    """Fully typed: map value from one range to another (integer)."""
    if in_high == in_low:
        return out_low
    return out_low + (val - in_low) * (out_high - out_low) // (in_high - in_low)


def map_range_untyped(val, in_low, in_high, out_low, out_high):
    """Untyped: same mapping."""
    if in_high == in_low:
        return out_low
    return out_low + (val - in_low) * (out_high - out_low) // (in_high - in_low)


def lerp_int_typed(a: int, b: int, t_num: int, t_den: int) -> int:
    """Fully typed: integer linear interpolation."""
    if t_den == 0:
        return a
    return a + (b - a) * t_num // t_den


def lerp_int_untyped(a, b, t_num, t_den):
    """Untyped: same interpolation."""
    if t_den == 0:
        return a
    return a + (b - a) * t_num // t_den


def sign_typed(x: int) -> int:
    """Fully typed: sign function."""
    if x > 0:
        return 1
    if x < 0:
        return 0 - 1
    return 0


def sign_untyped(x):
    """Untyped: sign function."""
    if x > 0:
        return 1
    if x < 0:
        return 0 - 1
    return 0


def distance_1d(a, b):
    """Untyped: absolute distance between two values."""
    diff = a - b
    if diff < 0:
        diff = 0 - diff
    return diff


def test_module() -> int:
    """Test mixed typed/untyped inference."""
    total: int = 0

    # Compare typed vs untyped clamp
    if clamp_typed(5, 1, 10) == 5:
        total = total + 1
    if clamp_untyped(5, 1, 10) == 5:
        total = total + 1
    if clamp_typed(0, 1, 10) == 1:
        total = total + 1
    if clamp_untyped(15, 1, 10) == 10:
        total = total + 1

    # Compare typed vs untyped map_range
    if map_range_typed(5, 0, 10, 0, 100) == 50:
        total = total + 1
    if map_range_untyped(5, 0, 10, 0, 100) == 50:
        total = total + 1

    # Compare typed vs untyped lerp
    if lerp_int_typed(0, 100, 1, 2) == 50:
        total = total + 1
    if lerp_int_untyped(0, 100, 1, 2) == 50:
        total = total + 1

    # Compare typed vs untyped sign
    if sign_typed(5) == 1:
        total = total + 1
    if sign_untyped(0 - 3) == 0 - 1:
        total = total + 1
    if sign_typed(0) == 0:
        total = total + 1

    # distance_1d tests
    if distance_1d(10, 3) == 7:
        total = total + 1
    if distance_1d(3, 10) == 7:
        total = total + 1
    if distance_1d(5, 5) == 0:
        total = total + 1

    return total
