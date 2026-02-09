"""Pathological decorator and protocol patterns for Python-to-Rust transpilation stress testing.

Every pattern is simulated with pure functions and dicts - no classes, no imports, no actual decorators.
All functions have full type annotations and docstrings.
"""


# =============================================================================
# 1. Manual decorator pattern: wrapper functions
# =============================================================================


def apply_double(x: int) -> int:
    """Simulate a decorator that doubles the result of a function."""
    return x * 2


def apply_negate(x: int) -> int:
    """Simulate a decorator that negates the result of a function."""
    return -x


def apply_double_then_negate(x: int) -> int:
    """Chain two decorator simulations: double then negate."""
    doubled: int = apply_double(x)
    negated: int = apply_negate(doubled)
    return negated


def apply_negate_then_double(x: int) -> int:
    """Chain two decorator simulations: negate then double."""
    negated: int = apply_negate(x)
    doubled: int = apply_double(negated)
    return doubled


def apply_increment(x: int) -> int:
    """Simulate a decorator that increments the result by one."""
    return x + 1


def apply_triple_chain(x: int) -> int:
    """Chain three decorator simulations: double, increment, negate."""
    step1: int = apply_double(x)
    step2: int = apply_increment(step1)
    step3: int = apply_negate(step2)
    return step3


# =============================================================================
# 2. Timing simulation: count iterations as proxy for timing
# =============================================================================


def count_operations_linear(n: int) -> int:
    """Count the number of operations in a linear scan."""
    ops: int = 0
    i: int = 0
    while i < n:
        ops = ops + 1
        i = i + 1
    return ops


def count_operations_quadratic(n: int) -> int:
    """Count the number of operations in a quadratic nested loop."""
    ops: int = 0
    i: int = 0
    while i < n:
        j: int = 0
        while j < n:
            ops = ops + 1
            j = j + 1
        i = i + 1
    return ops


def count_operations_halving(n: int) -> int:
    """Count the number of halvings until value reaches zero (log-like)."""
    ops: int = 0
    val: int = n
    while val > 0:
        val = val // 2
        ops = ops + 1
    return ops


# =============================================================================
# 3. Memoization decorator simulation: explicit cache dict
# =============================================================================


def fib_no_cache(n: int) -> int:
    """Compute fibonacci without cache (iterative to avoid stack issues)."""
    if n <= 0:
        return 0
    if n == 1:
        return 1
    a: int = 0
    b: int = 1
    i: int = 2
    while i <= n:
        temp: int = a + b
        a = b
        b = temp
        i = i + 1
    return b


def fib_with_cache(n: int, cache: dict[int, int]) -> int:
    """Compute fibonacci with explicit memoization cache dict."""
    if n in cache:
        return cache[n]
    if n <= 0:
        cache[0] = 0
        return 0
    if n == 1:
        cache[1] = 1
        return 1
    a: int = 0
    b: int = 1
    i: int = 2
    while i <= n:
        temp: int = a + b
        a = b
        b = temp
        i = i + 1
    cache[n] = b
    return b


def cache_hit_count(keys: list[int], cache: dict[int, int]) -> int:
    """Count how many keys are already present in the cache."""
    hits: int = 0
    for k in keys:
        if k in cache:
            hits = hits + 1
    return hits


# =============================================================================
# 4. Retry logic simulation: loop with counter, break on success
# =============================================================================


def retry_until_positive(values: list[int]) -> int:
    """Simulate retry: iterate values, return first positive or -1."""
    result: int = -1
    attempt: int = 0
    for v in values:
        attempt = attempt + 1
        if v > 0:
            result = v
            break
    return result


def retry_with_max_attempts(values: list[int], max_attempts: int) -> int:
    """Simulate retry with a maximum attempt count."""
    result: int = -1
    attempt: int = 0
    for v in values:
        if attempt >= max_attempts:
            break
        attempt = attempt + 1
        if v > 0:
            result = v
            break
    return result


def count_retries_before_success(values: list[int]) -> int:
    """Count how many attempts before finding a positive value."""
    attempt: int = 0
    for v in values:
        attempt = attempt + 1
        if v > 0:
            return attempt
    return attempt


# =============================================================================
# 5. Validation decorator simulation: check args before computation
# =============================================================================


def validate_positive(x: int) -> int:
    """Return 1 if x is positive, 0 otherwise (validation check)."""
    if x > 0:
        return 1
    return 0


def validated_square(x: int) -> int:
    """Compute square only if x is positive, else return -1 (sentinel)."""
    valid: int = validate_positive(x)
    if valid == 0:
        return -1
    return x * x


def validated_factorial(n: int) -> int:
    """Compute factorial only if n is non-negative and <= 12, else return -1."""
    if n < 0:
        return -1
    if n > 12:
        return -1
    result: int = 1
    i: int = 2
    while i <= n:
        result = result * i
        i = i + 1
    return result


def validate_in_range(x: int, low: int, high: int) -> int:
    """Return 1 if low <= x <= high, 0 otherwise."""
    if x >= low and x <= high:
        return 1
    return 0


def validated_divide(a: int, b: int) -> int:
    """Integer division only if b is nonzero, else return -1."""
    if b == 0:
        return -1
    return a // b


# =============================================================================
# 6. Property-like patterns: getter/setter on dict-based objects
# =============================================================================


def make_point(x: int, y: int) -> dict[str, int]:
    """Create a point as a dict with x and y keys."""
    point: dict[str, int] = {}
    point["x"] = x
    point["y"] = y
    return point


def get_x(point: dict[str, int]) -> int:
    """Getter for x coordinate of a point dict."""
    return point["x"]


def get_y(point: dict[str, int]) -> int:
    """Getter for y coordinate of a point dict."""
    return point["y"]


def set_x(point: dict[str, int], val: int) -> dict[str, int]:
    """Setter for x coordinate, returns modified point."""
    point["x"] = val
    return point


def set_y(point: dict[str, int], val: int) -> dict[str, int]:
    """Setter for y coordinate, returns modified point."""
    point["y"] = val
    return point


def point_distance_squared(p: dict[str, int]) -> int:
    """Compute squared distance from origin using getters."""
    px: int = get_x(p)
    py: int = get_y(p)
    return px * px + py * py


# =============================================================================
# 7. Static method patterns: logically grouped functions
# =============================================================================


def math_abs(x: int) -> int:
    """Static-like method: compute absolute value."""
    if x < 0:
        return -x
    return x


def math_sign(x: int) -> int:
    """Static-like method: return sign of x (-1, 0, or 1)."""
    if x > 0:
        return 1
    if x < 0:
        return -1
    return 0


def math_clamp(x: int, lo: int, hi: int) -> int:
    """Static-like method: clamp x to [lo, hi] range."""
    if x < lo:
        return lo
    if x > hi:
        return hi
    return x


# =============================================================================
# 8. Class method patterns: factory functions returning dict objects
# =============================================================================


def make_counter(start: int) -> dict[str, int]:
    """Factory: create a counter dict-object with value and step."""
    counter: dict[str, int] = {}
    counter["value"] = start
    counter["step"] = 1
    return counter


def counter_increment(counter: dict[str, int]) -> dict[str, int]:
    """Increment counter value by step, return modified counter."""
    counter["value"] = counter["value"] + counter["step"]
    return counter


def counter_get(counter: dict[str, int]) -> int:
    """Get current counter value."""
    return counter["value"]


def make_counter_with_step(start: int, step: int) -> dict[str, int]:
    """Factory: create a counter with custom step."""
    counter: dict[str, int] = {}
    counter["value"] = start
    counter["step"] = step
    return counter


# =============================================================================
# 9. Abstract method simulation: base returns sentinel, derived overrides
# =============================================================================


def shape_area_base(kind: int) -> int:
    """Abstract base: return -1 sentinel for unknown shape kind."""
    return -1


def shape_area_square(side: int) -> int:
    """Derived: compute area of square."""
    return side * side


def shape_area_rect(width: int, height: int) -> int:
    """Derived: compute area of rectangle."""
    return width * height


def shape_area_dispatch(kind: int, a: int, b: int) -> int:
    """Dispatch to correct area function based on kind (0=base, 1=square, 2=rect)."""
    if kind == 1:
        return shape_area_square(a)
    if kind == 2:
        return shape_area_rect(a, b)
    return shape_area_base(kind)


# =============================================================================
# 10. Chained method simulation: functions return modified dict
# =============================================================================


def builder_new() -> dict[str, int]:
    """Create a new empty builder dict."""
    b: dict[str, int] = {}
    b["width"] = 0
    b["height"] = 0
    b["depth"] = 0
    return b


def builder_set_width(b: dict[str, int], w: int) -> dict[str, int]:
    """Set width on builder, return builder for chaining."""
    b["width"] = w
    return b


def builder_set_height(b: dict[str, int], h: int) -> dict[str, int]:
    """Set height on builder, return builder for chaining."""
    b["height"] = h
    return b


def builder_set_depth(b: dict[str, int], d: int) -> dict[str, int]:
    """Set depth on builder, return builder for chaining."""
    b["depth"] = d
    return b


def builder_volume(b: dict[str, int]) -> int:
    """Compute volume from builder dimensions."""
    return b["width"] * b["height"] * b["depth"]


# =============================================================================
# 11. Context manager simulation: setup/teardown with counter tracking
# =============================================================================


def context_enter(state: dict[str, int]) -> dict[str, int]:
    """Simulate __enter__: increment open count, set active flag."""
    state["open_count"] = state["open_count"] + 1
    state["active"] = 1
    return state


def context_exit(state: dict[str, int]) -> dict[str, int]:
    """Simulate __exit__: decrement open count, clear active flag."""
    state["open_count"] = state["open_count"] - 1
    state["active"] = 0
    return state


def context_do_work(state: dict[str, int], work_units: int) -> dict[str, int]:
    """Perform work only if context is active."""
    if state["active"] == 1:
        state["work_done"] = state["work_done"] + work_units
    return state


def make_context_state() -> dict[str, int]:
    """Create initial context manager state dict."""
    state: dict[str, int] = {}
    state["open_count"] = 0
    state["active"] = 0
    state["work_done"] = 0
    return state


def context_managed_operation(work_units: int) -> int:
    """Run a full enter-work-exit cycle, return work done."""
    state: dict[str, int] = make_context_state()
    state = context_enter(state)
    state = context_do_work(state, work_units)
    state = context_exit(state)
    return state["work_done"]


# =============================================================================
# 12. Singleton pattern: check dict for existing instance
# =============================================================================


def singleton_get_or_create(registry: dict[str, int], key: str, default_val: int) -> int:
    """Get existing value or create with default (singleton-like)."""
    if key in registry:
        return registry[key]
    registry[key] = default_val
    return default_val


def singleton_exists(registry: dict[str, int], key: str) -> int:
    """Check if singleton key exists. Return 1 if yes, 0 if no."""
    if key in registry:
        return 1
    return 0


def singleton_reset(registry: dict[str, int], key: str) -> dict[str, int]:
    """Remove a singleton key if it exists, return registry."""
    if key in registry:
        del registry[key]
    return registry


# =============================================================================
# 13. Observer pattern: registration list + notify-all
# =============================================================================


def observer_register(observers: list[int], observer_id: int) -> list[int]:
    """Register an observer by appending its id."""
    observers.append(observer_id)
    return observers


def observer_notify_all(observers: list[int], event_value: int) -> int:
    """Simulate notify: sum observer_id * event_value for all observers."""
    total: int = 0
    for obs_id in observers:
        total = total + obs_id * event_value
    return total


def observer_count(observers: list[int]) -> int:
    """Return the number of registered observers."""
    return len(observers)


def observer_remove(observers: list[int], observer_id: int) -> list[int]:
    """Remove first occurrence of observer_id from list."""
    result: list[int] = []
    found: int = 0
    for obs_id in observers:
        if obs_id == observer_id and found == 0:
            found = 1
        else:
            result.append(obs_id)
    return result


# =============================================================================
# 14. Strategy pattern: dispatch based on string key
# =============================================================================


def strategy_add(a: int, b: int) -> int:
    """Strategy: addition."""
    return a + b


def strategy_sub(a: int, b: int) -> int:
    """Strategy: subtraction."""
    return a - b


def strategy_mul(a: int, b: int) -> int:
    """Strategy: multiplication."""
    return a * b


def strategy_dispatch(op: str, a: int, b: int) -> int:
    """Dispatch to strategy based on operation string."""
    if op == "add":
        return strategy_add(a, b)
    if op == "sub":
        return strategy_sub(a, b)
    if op == "mul":
        return strategy_mul(a, b)
    return -1


def strategy_execute_sequence(ops: list[str], values: list[int]) -> int:
    """Execute a sequence of operations, accumulating results left to right."""
    if len(values) == 0:
        return 0
    result: int = values[0]
    i: int = 0
    while i < len(ops) and i + 1 < len(values):
        result = strategy_dispatch(ops[i], result, values[i + 1])
        i = i + 1
    return result


# =============================================================================
# 15. Builder pattern: step-by-step dict construction with validation
# =============================================================================


def config_new() -> dict[str, int]:
    """Create empty config builder."""
    cfg: dict[str, int] = {}
    cfg["timeout"] = 0
    cfg["retries"] = 0
    cfg["port"] = 0
    cfg["valid"] = 0
    return cfg


def config_set_timeout(cfg: dict[str, int], timeout: int) -> dict[str, int]:
    """Set timeout on config builder."""
    if timeout > 0:
        cfg["timeout"] = timeout
    return cfg


def config_set_retries(cfg: dict[str, int], retries: int) -> dict[str, int]:
    """Set retries on config builder."""
    if retries >= 0 and retries <= 10:
        cfg["retries"] = retries
    return cfg


def config_set_port(cfg: dict[str, int], port: int) -> dict[str, int]:
    """Set port on config builder."""
    if port > 0 and port < 65536:
        cfg["port"] = port
    return cfg


def config_validate(cfg: dict[str, int]) -> dict[str, int]:
    """Validate config: set valid=1 if all fields are set properly."""
    if cfg["timeout"] > 0 and cfg["retries"] >= 0 and cfg["port"] > 0:
        cfg["valid"] = 1
    else:
        cfg["valid"] = 0
    return cfg


def config_is_valid(cfg: dict[str, int]) -> int:
    """Return 1 if config is valid, 0 otherwise."""
    return cfg["valid"]


# =============================================================================
# Test functions - each returns int for verification
# =============================================================================


def test_decorator_chain() -> int:
    """Test manual decorator chaining: double then negate of 5 = -10."""
    result: int = apply_double_then_negate(5)
    if result == -10:
        return 1
    return 0


def test_triple_chain() -> int:
    """Test triple chain: double(3)=6, inc(6)=7, neg(7)=-7."""
    result: int = apply_triple_chain(3)
    if result == -7:
        return 1
    return 0


def test_timing_linear() -> int:
    """Test linear operation count: 100 iterations = 100 ops."""
    ops: int = count_operations_linear(100)
    if ops == 100:
        return 1
    return 0


def test_timing_quadratic() -> int:
    """Test quadratic operation count: 10x10 = 100 ops."""
    ops: int = count_operations_quadratic(10)
    if ops == 100:
        return 1
    return 0


def test_memoization() -> int:
    """Test memoization: fib(10) cached, second lookup is a hit."""
    cache: dict[int, int] = {}
    val1: int = fib_with_cache(10, cache)
    hits_before: int = cache_hit_count([10], cache)
    val2: int = fib_with_cache(10, cache)
    if val1 == 55 and val2 == 55 and hits_before == 1:
        return 1
    return 0


def test_retry_logic() -> int:
    """Test retry: first positive in [-1, -2, 3, -4] is 3."""
    values: list[int] = [-1, -2, 3, -4]
    result: int = retry_until_positive(values)
    if result == 3:
        return 1
    return 0


def test_retry_max_attempts() -> int:
    """Test retry with max 2 attempts on [-1, -2, 3]: should fail (-1)."""
    values: list[int] = [-1, -2, 3]
    result: int = retry_with_max_attempts(values, 2)
    if result == -1:
        return 1
    return 0


def test_validation() -> int:
    """Test validated_square: positive gives square, negative gives -1."""
    pos: int = validated_square(4)
    neg: int = validated_square(-3)
    if pos == 16 and neg == -1:
        return 1
    return 0


def test_property_pattern() -> int:
    """Test getter/setter on point dict."""
    p: dict[str, int] = make_point(3, 4)
    p = set_x(p, 5)
    x: int = get_x(p)
    dist_sq: int = point_distance_squared(p)
    if x == 5 and dist_sq == 41:
        return 1
    return 0


def test_static_methods() -> int:
    """Test static-like math functions."""
    a: int = math_abs(-7)
    s: int = math_sign(-3)
    c: int = math_clamp(15, 0, 10)
    if a == 7 and s == -1 and c == 10:
        return 1
    return 0


def test_factory_counter() -> int:
    """Test counter factory and increment."""
    ctr: dict[str, int] = make_counter(0)
    ctr = counter_increment(ctr)
    ctr = counter_increment(ctr)
    ctr = counter_increment(ctr)
    val: int = counter_get(ctr)
    if val == 3:
        return 1
    return 0


def test_abstract_dispatch() -> int:
    """Test shape area dispatch: square(5)=25, rect(3,4)=12, unknown=-1."""
    sq: int = shape_area_dispatch(1, 5, 0)
    rect: int = shape_area_dispatch(2, 3, 4)
    base: int = shape_area_dispatch(0, 0, 0)
    if sq == 25 and rect == 12 and base == -1:
        return 1
    return 0


def test_chained_builder() -> int:
    """Test chained builder: 3*4*5 = 60 volume."""
    b: dict[str, int] = builder_new()
    b = builder_set_width(b, 3)
    b = builder_set_height(b, 4)
    b = builder_set_depth(b, 5)
    vol: int = builder_volume(b)
    if vol == 60:
        return 1
    return 0


def test_context_manager() -> int:
    """Test context manager simulation: 42 work units done."""
    result: int = context_managed_operation(42)
    if result == 42:
        return 1
    return 0


def test_singleton() -> int:
    """Test singleton get_or_create: first call creates, second retrieves."""
    reg: dict[str, int] = {}
    val1: int = singleton_get_or_create(reg, "db", 100)
    exists: int = singleton_exists(reg, "db")
    val2: int = singleton_get_or_create(reg, "db", 999)
    if val1 == 100 and exists == 1 and val2 == 100:
        return 1
    return 0


def test_observer_pattern() -> int:
    """Test observer register and notify: observers [1,2,3], event=10 => 60."""
    obs: list[int] = []
    obs = observer_register(obs, 1)
    obs = observer_register(obs, 2)
    obs = observer_register(obs, 3)
    total: int = observer_notify_all(obs, 10)
    count: int = observer_count(obs)
    if total == 60 and count == 3:
        return 1
    return 0


def test_strategy_dispatch() -> int:
    """Test strategy dispatch: add(3,4)=7, sub(10,3)=7, mul(2,5)=10."""
    a: int = strategy_dispatch("add", 3, 4)
    s: int = strategy_dispatch("sub", 10, 3)
    m: int = strategy_dispatch("mul", 2, 5)
    if a == 7 and s == 7 and m == 10:
        return 1
    return 0


def test_config_builder() -> int:
    """Test config builder with validation."""
    cfg: dict[str, int] = config_new()
    cfg = config_set_timeout(cfg, 30)
    cfg = config_set_retries(cfg, 3)
    cfg = config_set_port(cfg, 8080)
    cfg = config_validate(cfg)
    valid: int = config_is_valid(cfg)
    if valid == 1:
        return 1
    return 0


def test_config_invalid() -> int:
    """Test config builder rejects invalid config (port=0)."""
    cfg: dict[str, int] = config_new()
    cfg = config_set_timeout(cfg, 30)
    cfg = config_set_retries(cfg, 3)
    cfg = config_validate(cfg)
    valid: int = config_is_valid(cfg)
    if valid == 0:
        return 1
    return 0


def test_observer_remove() -> int:
    """Test observer removal: remove id=2 from [1,2,3], notify event=5 => 20."""
    obs: list[int] = []
    obs = observer_register(obs, 1)
    obs = observer_register(obs, 2)
    obs = observer_register(obs, 3)
    obs = observer_remove(obs, 2)
    total: int = observer_notify_all(obs, 5)
    if total == 20:
        return 1
    return 0


def test_validated_factorial() -> int:
    """Test validated factorial: 5!=120, negative=-1, too large=-1."""
    f5: int = validated_factorial(5)
    fn: int = validated_factorial(-1)
    fb: int = validated_factorial(13)
    if f5 == 120 and fn == -1 and fb == -1:
        return 1
    return 0


def test_halving_ops() -> int:
    """Test halving operation count: 16 requires 5 halvings (16,8,4,2,1,0)."""
    ops: int = count_operations_halving(16)
    if ops == 5:
        return 1
    return 0


def test_strategy_sequence() -> int:
    """Test strategy sequence: [add, mul] on [2, 3, 4] = (2+3)*4 = 20."""
    ops: list[str] = ["add", "mul"]
    vals: list[int] = [2, 3, 4]
    result: int = strategy_execute_sequence(ops, vals)
    if result == 20:
        return 1
    return 0


def test_counter_custom_step() -> int:
    """Test counter with step=5: start=10, 3 increments => 25."""
    ctr: dict[str, int] = make_counter_with_step(10, 5)
    ctr = counter_increment(ctr)
    ctr = counter_increment(ctr)
    ctr = counter_increment(ctr)
    val: int = counter_get(ctr)
    if val == 25:
        return 1
    return 0


def test_negate_then_double() -> int:
    """Test reverse chain: negate then double of 5 = -10."""
    result: int = apply_negate_then_double(5)
    if result == -10:
        return 1
    return 0


# =============================================================================
# Master test runner
# =============================================================================


def run_all_tests() -> int:
    """Run all tests and return sum of results (each test returns 0 or 1)."""
    total: int = 0
    total = total + test_decorator_chain()
    total = total + test_triple_chain()
    total = total + test_timing_linear()
    total = total + test_timing_quadratic()
    total = total + test_memoization()
    total = total + test_retry_logic()
    total = total + test_retry_max_attempts()
    total = total + test_validation()
    total = total + test_property_pattern()
    total = total + test_static_methods()
    total = total + test_factory_counter()
    total = total + test_abstract_dispatch()
    total = total + test_chained_builder()
    total = total + test_context_manager()
    total = total + test_singleton()
    total = total + test_observer_pattern()
    total = total + test_strategy_dispatch()
    total = total + test_config_builder()
    total = total + test_config_invalid()
    total = total + test_observer_remove()
    total = total + test_validated_factorial()
    total = total + test_halving_ops()
    total = total + test_strategy_sequence()
    total = total + test_counter_custom_step()
    total = total + test_negate_then_double()
    return total


if __name__ == "__main__":
    passed: int = run_all_tests()
    print(f"Tests passed: {passed}/25")
    assert passed == 25, f"Expected 25, got {passed}"
