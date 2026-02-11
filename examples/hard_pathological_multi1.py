# Pathological multi-function: 8+ functions in complex dependency chain
# Tests: deep call chains, functions calling multiple other functions


def clamp_val(val: int, lo: int, hi: int) -> int:
    if val < lo:
        return lo
    if val > hi:
        return hi
    return val


def abs_val(n: int) -> int:
    if n < 0:
        return 0 - n
    return n


def sign_val(n: int) -> int:
    if n > 0:
        return 1
    if n < 0:
        return 0 - 1
    return 0


def normalize_range(val: int, old_min: int, old_max: int, new_min: int, new_max: int) -> int:
    """Normalize val from [old_min, old_max] to [new_min, new_max] using integer math."""
    clamped: int = clamp_val(val, old_min, old_max)
    old_range: int = old_max - old_min
    if old_range == 0:
        return new_min
    new_range: int = new_max - new_min
    offset: int = clamped - old_min
    scaled: int = (offset * new_range) // old_range + new_min
    return clamp_val(scaled, new_min, new_max)


def distance_1d(a: int, b: int) -> int:
    return abs_val(a - b)


def manhattan_distance(x1: int, y1: int, x2: int, y2: int) -> int:
    return distance_1d(x1, x2) + distance_1d(y1, y2)


def closest_point_index(target_x: int, target_y: int,
                        points_x: list[int], points_y: list[int]) -> int:
    """Find index of closest point by manhattan distance."""
    best_idx: int = 0
    best_dist: int = manhattan_distance(target_x, target_y, points_x[0], points_y[0])
    i: int = 1
    while i < len(points_x):
        d: int = manhattan_distance(target_x, target_y, points_x[i], points_y[i])
        if d < best_dist:
            best_dist = d
            best_idx = i
        i = i + 1
    return best_idx


def transform_pipeline(vals: list[int], lo: int, hi: int,
                       new_lo: int, new_hi: int) -> list[int]:
    """Clamp, normalize, then apply sign-based adjustment to each value."""
    result: list[int] = []
    i: int = 0
    while i < len(vals):
        clamped: int = clamp_val(vals[i], lo, hi)
        normalized: int = normalize_range(clamped, lo, hi, new_lo, new_hi)
        s: int = sign_val(vals[i])
        adjusted: int = normalized + s
        result.append(adjusted)
        i = i + 1
    return result


def weighted_score(scores: list[int], weights: list[int],
                   lo: int, hi: int) -> int:
    """Compute weighted average, clamped to range."""
    total_weight: int = 0
    total_score: int = 0
    i: int = 0
    limit: int = len(scores)
    if len(weights) < limit:
        limit = len(weights)
    while i < limit:
        total_score = total_score + scores[i] * weights[i]
        total_weight = total_weight + weights[i]
        i = i + 1
    if total_weight == 0:
        return lo
    avg: int = total_score // total_weight
    return clamp_val(avg, lo, hi)


def test_module() -> int:
    passed: int = 0
    # Test 1: clamp
    if clamp_val(15, 0, 10) == 10:
        passed = passed + 1
    # Test 2: abs
    if abs_val(0 - 7) == 7:
        passed = passed + 1
    # Test 3: normalize
    if normalize_range(50, 0, 100, 0, 10) == 5:
        passed = passed + 1
    # Test 4: manhattan distance
    if manhattan_distance(0, 0, 3, 4) == 7:
        passed = passed + 1
    # Test 5: closest point
    px: list[int] = [0, 10, 5]
    py: list[int] = [0, 10, 3]
    if closest_point_index(4, 3, px, py) == 2:
        passed = passed + 1
    # Test 6: transform pipeline
    transformed: list[int] = transform_pipeline([0, 50, 100], 0, 100, 0, 10)
    if transformed[1] == 6:
        passed = passed + 1
    # Test 7: weighted score
    if weighted_score([80, 90, 70], [1, 2, 1], 0, 100) == 82:
        passed = passed + 1
    return passed
