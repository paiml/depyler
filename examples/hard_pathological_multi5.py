# Pathological multi-function: Functions with 5+ parameters all fully typed
# Tests: complex function signatures, many typed parameters


def weighted_average(v1: int, w1: int, v2: int, w2: int, v3: int, w3: int) -> int:
    """Weighted average of 3 values with 3 weights."""
    total_weight: int = w1 + w2 + w3
    if total_weight == 0:
        return 0
    total: int = v1 * w1 + v2 * w2 + v3 * w3
    return total // total_weight


def interpolate(x0: int, y0: int, x1: int, y1: int, x: int) -> int:
    """Linear interpolation (integer). Compute y at point x given two points."""
    if x1 == x0:
        return y0
    result: int = y0 + (y1 - y0) * (x - x0) // (x1 - x0)
    return result


def bounded_transform(val: int, lo: int, hi: int, scale: int, offset: int) -> int:
    """Apply val * scale + offset, clamped to [lo, hi]."""
    result: int = val * scale + offset
    if result < lo:
        return lo
    if result > hi:
        return hi
    return result


def categorize_point(x: int, y: int, min_x: int, max_x: int, min_y: int, max_y: int) -> int:
    """Categorize a 2D point relative to a bounding box.
    Returns: 0=inside, 1=left, 2=right, 3=above, 4=below, 5=corner."""
    is_left: bool = x < min_x
    is_right: bool = x > max_x
    is_below: bool = y < min_y
    is_above: bool = y > max_y
    if is_left == True and is_above == True:
        return 5
    if is_right == True and is_above == True:
        return 5
    if is_left == True and is_below == True:
        return 5
    if is_right == True and is_below == True:
        return 5
    if is_left == True:
        return 1
    if is_right == True:
        return 2
    if is_above == True:
        return 3
    if is_below == True:
        return 4
    return 0


def compute_score(correct: int, wrong: int, skipped: int,
                  correct_weight: int, wrong_penalty: int) -> int:
    """Compute exam score with weights and penalties."""
    raw: int = correct * correct_weight - wrong * wrong_penalty
    if raw < 0:
        return 0
    total_possible: int = (correct + wrong + skipped) * correct_weight
    if total_possible == 0:
        return 0
    return (raw * 100) // total_possible


def blend_colors(r1: int, g1: int, b1: int, r2: int, g2: int, b2: int, ratio: int) -> list[int]:
    """Blend two RGB colors. ratio is 0-100 (0=all color1, 100=all color2)."""
    inv_ratio: int = 100 - ratio
    r: int = (r1 * inv_ratio + r2 * ratio) // 100
    g: int = (g1 * inv_ratio + g2 * ratio) // 100
    b: int = (b1 * inv_ratio + b2 * ratio) // 100
    result: list[int] = [r, g, b]
    return result


def apply_all_transforms(vals: list[int], lo: int, hi: int,
                         scale: int, offset: int) -> list[int]:
    """Apply bounded_transform to each value."""
    result: list[int] = []
    i: int = 0
    while i < len(vals):
        result.append(bounded_transform(vals[i], lo, hi, scale, offset))
        i = i + 1
    return result


def test_module() -> int:
    passed: int = 0
    # Test 1: weighted average
    if weighted_average(100, 1, 80, 2, 60, 1) == 80:
        passed = passed + 1
    # Test 2: interpolate (0,0) to (10,100) at x=5 -> 50
    if interpolate(0, 0, 10, 100, 5) == 50:
        passed = passed + 1
    # Test 3: bounded transform
    if bounded_transform(5, 0, 100, 3, 10) == 25:
        passed = passed + 1
    # Test 4: categorize inside
    if categorize_point(5, 5, 0, 10, 0, 10) == 0:
        passed = passed + 1
    # Test 5: categorize corner
    if categorize_point(0 - 1, 11, 0, 10, 0, 10) == 5:
        passed = passed + 1
    # Test 6: compute score (8 correct, 2 wrong, 0 skipped, weight 10, penalty 5)
    # raw = 80-10 = 70, total_possible = 100, score = 70
    if compute_score(8, 2, 0, 10, 5) == 70:
        passed = passed + 1
    # Test 7: blend colors (50/50)
    blended: list[int] = blend_colors(255, 0, 0, 0, 255, 0, 50)
    if blended[0] == 127 and blended[1] == 127:
        passed = passed + 1
    return passed
