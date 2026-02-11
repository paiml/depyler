# Type inference test: Multi-step type propagation
# Strategy: Types propagate through multiple levels of function calls


def step_add(a, b):
    """Simple addition - types from call site."""
    return a + b


def step_mul(a, b):
    """Simple multiplication - types from call site."""
    return a * b


def step_sub(a, b):
    """Simple subtraction - types from call site."""
    return a - b


def step_div(a, b):
    """Integer division - types from call site."""
    if b == 0:
        return 0
    return a // b


def evaluate_polynomial_horner(coeffs: list[int], x):
    """Evaluate polynomial using Horner's method.
    coeffs[0] is highest degree coefficient.
    x type inferred from multiplication."""
    if len(coeffs) == 0:
        return 0
    result = coeffs[0]
    i = 1
    while i < len(coeffs):
        result = step_add(step_mul(result, x), coeffs[i])
        i = i + 1
    return result


def evaluate_polynomial_direct(coeffs: list[int], x):
    """Evaluate polynomial directly.
    coeffs[i] is coefficient of x^i."""
    result = 0
    power = 1
    i = 0
    while i < len(coeffs):
        result = step_add(result, step_mul(coeffs[i], power))
        power = step_mul(power, x)
        i = i + 1
    return result


def compute_mean(vals: list[int]):
    """Mean of integer values (integer division)."""
    if len(vals) == 0:
        return 0
    total = 0
    i = 0
    while i < len(vals):
        total = step_add(total, vals[i])
        i = i + 1
    return step_div(total, len(vals))


def compute_variance_approx(vals: list[int]):
    """Approximate variance using integer arithmetic."""
    if len(vals) == 0:
        return 0
    avg = compute_mean(vals)
    total_sq_diff = 0
    i = 0
    while i < len(vals):
        diff = step_sub(vals[i], avg)
        sq = step_mul(diff, diff)
        total_sq_diff = step_add(total_sq_diff, sq)
        i = i + 1
    return step_div(total_sq_diff, len(vals))


def distance_squared(x1, y1, x2, y2):
    """Squared Euclidean distance, types propagated through steps."""
    dx = step_sub(x2, x1)
    dy = step_sub(y2, y1)
    return step_add(step_mul(dx, dx), step_mul(dy, dy))


def manhattan_distance(x1, y1, x2, y2):
    """Manhattan distance with abs via step functions."""
    dx = step_sub(x2, x1)
    dy = step_sub(y2, y1)
    if dx < 0:
        dx = step_sub(0, dx)
    if dy < 0:
        dy = step_sub(0, dy)
    return step_add(dx, dy)


def test_module() -> int:
    """Test multi-step type propagation."""
    total: int = 0

    # Basic step functions
    if step_add(3, 4) == 7:
        total = total + 1
    if step_mul(3, 4) == 12:
        total = total + 1
    if step_sub(10, 3) == 7:
        total = total + 1
    if step_div(10, 3) == 3:
        total = total + 1
    if step_div(10, 0) == 0:
        total = total + 1

    # evaluate_polynomial_horner: 2x^2 + 3x + 1 at x=2
    # coeffs = [2, 3, 1] (highest first)
    if evaluate_polynomial_horner([2, 3, 1], 2) == 15:
        total = total + 1

    # evaluate_polynomial_direct: 1 + 3x + 2x^2 at x=2
    # coeffs = [1, 3, 2] (lowest first)
    if evaluate_polynomial_direct([1, 3, 2], 2) == 15:
        total = total + 1

    # compute_mean
    if compute_mean([10, 20, 30]) == 20:
        total = total + 1
    if compute_mean([]) == 0:
        total = total + 1

    # compute_variance_approx
    # mean of [2,4,6] = 4, diffs = [-2,0,2], sqs = [4,0,4], sum=8, var=8/3=2
    if compute_variance_approx([2, 4, 6]) == 2:
        total = total + 1

    # distance_squared: (1,2) to (4,6) = 9+16=25
    if distance_squared(1, 2, 4, 6) == 25:
        total = total + 1

    # manhattan_distance: (1,2) to (4,6) = 3+4=7
    if manhattan_distance(1, 2, 4, 6) == 7:
        total = total + 1

    return total
