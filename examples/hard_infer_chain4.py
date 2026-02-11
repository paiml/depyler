# Type inference test: Conditional type inference
# Strategy: Types inferred through if/else branches


def classify_sign(x):
    """Classify as positive (1), negative (-1), or zero (0)."""
    if x > 0:
        return 1
    if x < 0:
        return 0 - 1
    return 0


def classify_magnitude(x):
    """Classify magnitude: 0=zero, 1=small, 2=medium, 3=large."""
    val = x
    if val < 0:
        val = 0 - val
    if val == 0:
        return 0
    if val < 10:
        return 1
    if val < 100:
        return 2
    return 3


def branch_compute(x, mode):
    """Different computations based on mode, all returning int."""
    if mode == 0:
        return x + x
    if mode == 1:
        return x * x
    if mode == 2:
        return x * x * x
    return x


def conditional_accumulate(n, use_squares):
    """Accumulate either values or their squares based on flag."""
    total = 0
    i = 1
    while i <= n:
        if use_squares > 0:
            total = total + i * i
        else:
            total = total + i
        i = i + 1
    return total


def threshold_count(vals: list[int], thresh):
    """Count values above threshold."""
    count = 0
    i = 0
    while i < len(vals):
        if vals[i] > thresh:
            count = count + 1
        i = i + 1
    return count


def partition_sum(vals: list[int], pivot):
    """Sum elements below pivot and above pivot, return difference."""
    below = 0
    above = 0
    i = 0
    while i < len(vals):
        if vals[i] < pivot:
            below = below + vals[i]
        elif vals[i] > pivot:
            above = above + vals[i]
        i = i + 1
    return above - below


def stepped_function(x):
    """Piecewise linear function."""
    if x < 0:
        return 0
    if x < 10:
        return x
    if x < 20:
        return 10 + (x - 10) * 2
    if x < 30:
        return 30 + (x - 20) * 3
    return 60


def multi_branch_min(a, b, c, d):
    """Find minimum of four values via comparisons."""
    result = a
    if b < result:
        result = b
    if c < result:
        result = c
    if d < result:
        result = d
    return result


def zigzag(n):
    """Generate zigzag sum: +1, -2, +3, -4, ..."""
    total = 0
    i = 1
    while i <= n:
        if i % 2 == 1:
            total = total + i
        else:
            total = total - i
        i = i + 1
    return total


def test_module() -> int:
    """Test conditional type inference."""
    total: int = 0

    # classify_sign tests
    if classify_sign(5) == 1:
        total = total + 1
    if classify_sign(0 - 3) == 0 - 1:
        total = total + 1
    if classify_sign(0) == 0:
        total = total + 1

    # classify_magnitude tests
    if classify_magnitude(0) == 0:
        total = total + 1
    if classify_magnitude(5) == 1:
        total = total + 1
    if classify_magnitude(50) == 2:
        total = total + 1
    if classify_magnitude(500) == 3:
        total = total + 1

    # branch_compute tests
    if branch_compute(5, 0) == 10:
        total = total + 1
    if branch_compute(5, 1) == 25:
        total = total + 1
    if branch_compute(3, 2) == 27:
        total = total + 1

    # conditional_accumulate
    if conditional_accumulate(3, 0) == 6:
        total = total + 1
    if conditional_accumulate(3, 1) == 14:
        total = total + 1

    # threshold_count
    data: list[int] = [1, 5, 3, 8, 2, 7]
    if threshold_count(data, 4) == 3:
        total = total + 1

    # partition_sum
    if partition_sum([1, 5, 3, 8, 2, 7], 4) == 14:
        total = total + 1

    # stepped_function
    if stepped_function(5) == 5:
        total = total + 1
    if stepped_function(15) == 20:
        total = total + 1

    # multi_branch_min
    if multi_branch_min(5, 2, 8, 3) == 2:
        total = total + 1

    # zigzag
    if zigzag(4) == 0 - 2:
        total = total + 1

    return total
