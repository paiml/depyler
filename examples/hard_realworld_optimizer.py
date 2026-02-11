"""Real-world simple optimization algorithms.

Mimics: scipy.optimize, gradient descent implementations, hyperparameter tuning.
Implements golden section search, bisection, gradient descent (integer approximation).
"""


def abs_val(x: int) -> int:
    """Absolute value."""
    if x < 0:
        return 0 - x
    return x


def golden_section_min(a: int, b: int, tolerance: int) -> int:
    """Golden section search to find minimum of f(x) = (x-50)^2 on [a,b].
    All values scaled by 1000 for integer precision.
    Returns x*1000 of approximate minimum."""
    # Golden ratio approximation: 618/1000
    gr: int = 618
    c: int = b - (gr * (b - a)) // 1000
    d: int = a + (gr * (b - a)) // 1000
    iterations: int = 0
    max_iter: int = 100
    while abs_val(b - a) > tolerance and iterations < max_iter:
        fc: int = (c - 50000) * (c - 50000)
        fd: int = (d - 50000) * (d - 50000)
        if fc < fd:
            b = d
        else:
            a = c
        c = b - (gr * (b - a)) // 1000
        d = a + (gr * (b - a)) // 1000
        iterations = iterations + 1
    return (a + b) // 2


def bisection_root(a: int, b: int, tolerance: int) -> int:
    """Bisection method to find root of f(x) = x^2 - 2500 (i.e., sqrt(2500)=50).
    Values scaled by 100. Returns x*100."""
    iterations: int = 0
    max_iter: int = 100
    while abs_val(b - a) > tolerance and iterations < max_iter:
        mid: int = (a + b) // 2
        fa: int = a * a - 250000
        fm: int = mid * mid - 250000
        if (fa > 0 and fm > 0) or (fa < 0 and fm < 0):
            a = mid
        else:
            b = mid
        iterations = iterations + 1
    return (a + b) // 2


def gradient_descent_1d(start_x: int, learning_rate: int, max_steps: int) -> int:
    """1D gradient descent on f(x) = (x-30)^2.
    All values scaled by 100. learning_rate is also scaled by 100.
    Returns x*100 at convergence."""
    x: int = start_x
    step: int = 0
    while step < max_steps:
        # gradient of (x-3000)^2 is 2*(x-3000)
        grad: int = 2 * (x - 3000)
        delta: int = (learning_rate * grad) // 100
        x = x - delta
        if abs_val(delta) < 1:
            step = max_steps
        else:
            step = step + 1
    return x


def linear_search_min(values: list[int]) -> list[int]:
    """Find minimum value and its index. Returns [min_val, min_idx]."""
    if len(values) == 0:
        return [0, -1]
    min_val: int = values[0]
    min_idx: int = 0
    idx: int = 1
    while idx < len(values):
        if values[idx] < min_val:
            min_val = values[idx]
            min_idx = idx
        idx = idx + 1
    return [min_val, min_idx]


def grid_search_2d(x_start: int, x_end: int, y_start: int, y_end: int, step: int) -> list[int]:
    """Grid search for minimum of f(x,y) = (x-20)^2 + (y-30)^2.
    Returns [best_x, best_y, best_val]."""
    best_x: int = x_start
    best_y: int = y_start
    dx: int = x_start - 20
    dy: int = y_start - 30
    best_val: int = dx * dx + dy * dy
    x: int = x_start
    while x <= x_end:
        y: int = y_start
        while y <= y_end:
            dx2: int = x - 20
            dy2: int = y - 30
            val: int = dx2 * dx2 + dy2 * dy2
            if val < best_val:
                best_val = val
                best_x = x
                best_y = y
            y = y + step
        x = x + step
    return [best_x, best_y, best_val]


def simulated_annealing_step(current_val: int, neighbor_val: int, temperature: int) -> bool:
    """Decide whether to accept neighbor in simulated annealing.
    Accept if better. If worse, accept with probability based on temp.
    Uses simplified integer heuristic."""
    if neighbor_val < current_val:
        return True
    if temperature <= 0:
        return False
    diff: int = neighbor_val - current_val
    # Simple heuristic: accept if diff < temperature
    return diff < temperature


def convergence_check(history: list[int], window: int, threshold: int) -> bool:
    """Check if optimization has converged by looking at recent history.
    Converged if max-min of last window values < threshold."""
    if len(history) < window:
        return False
    start: int = len(history) - window
    lo: int = history[start]
    hi: int = history[start]
    idx: int = start + 1
    while idx < len(history):
        if history[idx] < lo:
            lo = history[idx]
        if history[idx] > hi:
            hi = history[idx]
        idx = idx + 1
    return (hi - lo) < threshold


def evaluate_quadratic(x: int, a_coeff: int, b_coeff: int, c_coeff: int) -> int:
    """Evaluate ax^2 + bx + c."""
    return a_coeff * x * x + b_coeff * x + c_coeff


def test_module() -> int:
    """Test optimizer module."""
    passed: int = 0

    # Test 1: golden section finds minimum near 50
    result: int = golden_section_min(0, 100000, 100)
    if abs_val(result - 50000) < 500:
        passed = passed + 1

    # Test 2: bisection finds root near 50 (x*100 scale -> 5000)
    root: int = bisection_root(0, 10000, 10)
    if abs_val(root - 500) < 100:
        passed = passed + 1

    # Test 3: gradient descent converges near 30
    gd: int = gradient_descent_1d(0, 10, 200)
    if abs_val(gd - 3000) < 100:
        passed = passed + 1

    # Test 4: linear search min
    vals: list[int] = [5, 3, 8, 1, 9]
    lmin: list[int] = linear_search_min(vals)
    if lmin[0] == 1 and lmin[1] == 3:
        passed = passed + 1

    # Test 5: grid search 2d
    gs: list[int] = grid_search_2d(0, 40, 0, 60, 5)
    if gs[0] == 20 and gs[1] == 30:
        passed = passed + 1

    # Test 6: simulated annealing accept better
    if simulated_annealing_step(100, 50, 10):
        passed = passed + 1

    # Test 7: convergence check
    history: list[int] = [100, 50, 30, 28, 27, 27, 27]
    if convergence_check(history, 3, 2):
        passed = passed + 1

    # Test 8: evaluate quadratic
    # f(5) = 2*25 + 3*5 + 1 = 66
    if evaluate_quadratic(5, 2, 3, 1) == 66:
        passed = passed + 1

    return passed
