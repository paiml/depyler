# Type inference test: Variable type from function return
# Strategy: Variables get their types entirely from function return values


def make_value(seed):
    """Create a value from seed."""
    return seed * 7 + 3


def transform_value(v):
    """Transform a value."""
    return v * 2 + 1


def combine_values(a, b):
    """Combine two values."""
    return a + b


def scale_value(v, factor):
    """Scale a value by factor."""
    return v * factor


def reduce_value(v, amount):
    """Reduce a value."""
    return v - amount


def clamp_positive(v):
    """Ensure value is non-negative."""
    if v < 0:
        return 0
    return v


def process_pipeline(seed):
    """Multi-step pipeline where each variable type comes from prev function."""
    v1 = make_value(seed)
    v2 = transform_value(v1)
    v3 = scale_value(v2, 3)
    v4 = reduce_value(v3, 100)
    v5 = clamp_positive(v4)
    return v5


def dual_pipeline(s1, s2):
    """Two parallel pipelines merged."""
    left = process_pipeline(s1)
    right = process_pipeline(s2)
    return combine_values(left, right)


def accumulate_pipeline(count):
    """Accumulate pipeline results."""
    total = 0
    i = 0
    while i < count:
        v = make_value(i)
        t = transform_value(v)
        total = total + t
        i = i + 1
    return total


def recursive_transform(v, depth):
    """Apply transform_value recursively."""
    if depth <= 0:
        return v
    result = transform_value(v)
    return recursive_transform(result, depth - 1)


def test_module() -> int:
    """Test variable type inference from function returns."""
    total: int = 0

    # make_value: seed*7 + 3
    if make_value(1) == 10:
        total = total + 1
    if make_value(0) == 3:
        total = total + 1

    # transform_value: v*2 + 1
    if transform_value(10) == 21:
        total = total + 1

    # combine_values
    if combine_values(10, 20) == 30:
        total = total + 1

    # scale_value
    if scale_value(5, 3) == 15:
        total = total + 1

    # reduce_value
    if reduce_value(100, 30) == 70:
        total = total + 1

    # clamp_positive
    if clamp_positive(5) == 5:
        total = total + 1
    if clamp_positive(0 - 3) == 0:
        total = total + 1

    # process_pipeline
    # make_value(1)=10, transform=21, scale=63, reduce=-37, clamp=0
    if process_pipeline(1) == 0:
        total = total + 1
    # make_value(5)=38, transform=77, scale=231, reduce=131, clamp=131
    if process_pipeline(5) == 131:
        total = total + 1

    # dual_pipeline
    dp: int = dual_pipeline(5, 10)
    if dp > 0:
        total = total + 1

    # accumulate_pipeline
    ap: int = accumulate_pipeline(3)
    # i=0: make(0)=3, trans=7; i=1: make(1)=10, trans=21; i=2: make(2)=17, trans=35
    # total = 7+21+35 = 63
    if ap == 63:
        total = total + 1

    # recursive_transform: apply v*2+1 three times starting from 1
    # 1 -> 3 -> 7 -> 15
    if recursive_transform(1, 3) == 15:
        total = total + 1

    return total
