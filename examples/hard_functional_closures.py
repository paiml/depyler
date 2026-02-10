"""Hard functional closures: nonlocal, functools, map/filter/zip, lambda-in-lambda, HOFs."""

from typing import (
    List,
    Dict,
    Tuple,
    Callable,
    TypeVar,
    Optional,
    Sequence,
)
from functools import partial, reduce

T = TypeVar("T")
U = TypeVar("U")
R = TypeVar("R")


# --- Closures capturing mutable state with nonlocal ---

def make_counter(start: int = 0) -> Tuple[Callable[[], int], Callable[[], int], Callable[[], None]]:
    """Return (increment, get_count, reset) closures sharing mutable state."""
    count = start

    def increment() -> int:
        nonlocal count
        count += 1
        return count

    def get_count() -> int:
        return count

    def reset() -> None:
        nonlocal count
        count = start

    return (increment, get_count, reset)


def make_accumulator(initial: float = 0.0) -> Callable[[float], float]:
    """Return a closure that accumulates values."""
    total = initial

    def accumulate(value: float) -> float:
        nonlocal total
        total += value
        return total

    return accumulate


def make_running_stats() -> Tuple[Callable[[float], None], Callable[[], Dict[str, float]]]:
    """Return closures for tracking running mean and variance."""
    count = 0
    total = 0.0
    sum_squares = 0.0

    def add_value(x: float) -> None:
        nonlocal count, total, sum_squares
        count += 1
        total += x
        sum_squares += x * x

    def get_stats() -> Dict[str, float]:
        if count == 0:
            return {"count": 0.0, "mean": 0.0, "variance": 0.0}
        mean = total / count
        variance = (sum_squares / count) - (mean * mean)
        return {"count": float(count), "mean": mean, "variance": max(0.0, variance)}

    return (add_value, get_stats)


def make_history_tracker(max_size: int) -> Tuple[Callable[[str], None], Callable[[], List[str]], Callable[[], Optional[str]]]:
    """Return closures for a bounded history buffer."""
    history: List[str] = []

    def push(item: str) -> None:
        nonlocal history
        history.append(item)
        if len(history) > max_size:
            history = history[-max_size:]

    def get_history() -> List[str]:
        return list(history)

    def latest() -> Optional[str]:
        if history:
            return history[-1]
        return None

    return (push, get_history, latest)


# --- Higher-order functions returning closures ---

def compose(f: Callable[[T], U], g: Callable[[U], R]) -> Callable[[T], R]:
    """Compose two functions: compose(f, g)(x) == g(f(x))."""
    def composed(x: T) -> R:
        return g(f(x))
    return composed


def pipe(*funcs: Callable) -> Callable:
    """Create a pipeline of functions applied left to right."""
    def piped(x: object) -> object:
        result = x
        for fn in funcs:
            result = fn(result)
        return result
    return piped


def memoize(func: Callable[[int], int]) -> Callable[[int], int]:
    """Memoization closure for int->int functions."""
    cache: Dict[int, int] = {}

    def memoized(n: int) -> int:
        if n not in cache:
            cache[n] = func(n)
        return cache[n]

    return memoized


def make_validator(
    rules: List[Tuple[str, Callable[[str], bool]]]
) -> Callable[[str], List[str]]:
    """Return a validator closure that checks all rules."""
    def validate(value: str) -> List[str]:
        errors: List[str] = []
        for name, rule in rules:
            if not rule(value):
                errors.append(name)
        return errors

    return validate


def make_rate_limiter(max_calls: int) -> Callable[[], bool]:
    """Return a closure that limits the number of calls."""
    remaining = max_calls

    def try_call() -> bool:
        nonlocal remaining
        if remaining > 0:
            remaining -= 1
            return True
        return False

    return try_call


def make_retry_wrapper(
    max_retries: int,
) -> Callable[[Callable[[], T]], Callable[[], Optional[T]]]:
    """Return a HOF that wraps a function with retry logic."""
    def wrapper(func: Callable[[], T]) -> Callable[[], Optional[T]]:
        def wrapped() -> Optional[T]:
            attempts = 0
            while attempts < max_retries:
                try:
                    return func()
                except Exception:
                    attempts += 1
            return None
        return wrapped
    return wrapper


# --- functools.partial patterns ---

def add(a: int, b: int) -> int:
    return a + b


def multiply(a: float, b: float) -> float:
    return a * b


def format_value(value: float, prefix: str, suffix: str, decimals: int) -> str:
    return f"{prefix}{value:.{decimals}f}{suffix}"


# Create partial functions
add_ten = partial(add, b=10)
double = partial(multiply, b=2.0)
format_currency = partial(format_value, prefix="$", suffix="", decimals=2)
format_percentage = partial(format_value, prefix="", suffix="%", decimals=1)


# --- functools.reduce patterns ---

def product_of_list(numbers: List[int]) -> int:
    """Product of all elements using reduce."""
    if not numbers:
        return 0
    return reduce(lambda a, b: a * b, numbers)


def flatten_lists(nested: List[List[T]]) -> List[T]:
    """Flatten a list of lists using reduce."""
    if not nested:
        return []
    return reduce(lambda acc, lst: acc + lst, nested, [])


def build_string_from_parts(parts: List[str], separator: str) -> str:
    """Join strings using reduce."""
    if not parts:
        return ""
    return reduce(lambda a, b: f"{a}{separator}{b}", parts)


def max_by_key(items: List[T], key: Callable[[T], float]) -> Optional[T]:
    """Find item with maximum key value using reduce."""
    if not items:
        return None
    return reduce(lambda a, b: a if key(a) >= key(b) else b, items)


def running_reduce(
    values: List[float], func: Callable[[float, float], float]
) -> List[float]:
    """Produce all intermediate reduce values."""
    if not values:
        return []
    result: List[float] = [values[0]]
    acc = values[0]
    for v in values[1:]:
        acc = func(acc, v)
        result.append(acc)
    return result


# --- map/filter/zip combinations ---

def transform_and_filter(
    items: List[int],
    transform: Callable[[int], int],
    predicate: Callable[[int], bool],
) -> List[int]:
    """Apply transform then filter."""
    return list(filter(predicate, map(transform, items)))


def zip_with(
    func: Callable[[T, U], R], xs: List[T], ys: List[U]
) -> List[R]:
    """Zip two lists with a combining function."""
    return [func(x, y) for x, y in zip(xs, ys)]


def parallel_map(
    funcs: List[Callable[[T], U]], items: List[T]
) -> List[List[U]]:
    """Apply each function to all items."""
    return [list(map(f, items)) for f in funcs]


def chain_map_filter(
    data: List[int],
) -> List[str]:
    """Chain of map/filter operations."""
    # Square -> filter evens -> convert to hex strings
    squared = list(map(lambda x: x * x, data))
    evens = list(filter(lambda x: x % 2 == 0, squared))
    hex_strings = list(map(lambda x: f"0x{x:04x}", evens))
    return hex_strings


def multi_zip_aggregate(
    names: List[str],
    scores: List[float],
    weights: List[float],
) -> List[Tuple[str, float]]:
    """Zip three lists and compute weighted scores."""
    return [
        (name, score * weight)
        for name, score, weight in zip(names, scores, weights)
    ]


# --- Lambda patterns ---

def sort_by_multiple_keys(
    data: List[Dict[str, int]],
    keys: List[str],
) -> List[Dict[str, int]]:
    """Sort by multiple keys using lambda."""
    result = list(data)
    for key in reversed(keys):
        result = sorted(result, key=lambda d, k=key: d.get(k, 0))
    return result


def create_comparators(
    field: str,
) -> Tuple[Callable[[Dict[str, int], Dict[str, int]], bool], Callable[[Dict[str, int], Dict[str, int]], bool]]:
    """Create less-than and greater-than comparators for a dict field."""
    lt = lambda a, b: a.get(field, 0) < b.get(field, 0)
    gt = lambda a, b: a.get(field, 0) > b.get(field, 0)
    return (lt, gt)


def apply_transformations(
    value: float,
    transforms: List[Callable[[float], float]],
) -> List[float]:
    """Apply each transformation to the value."""
    return [t(value) for t in transforms]


class FunctionChain:
    """Builder class for function composition chains."""

    def __init__(self) -> None:
        self._functions: List[Callable] = []

    def then(self, func: Callable) -> "FunctionChain":
        self._functions.append(func)
        return self

    def build(self) -> Callable:
        captured = list(self._functions)
        def chained(x: object) -> object:
            result = x
            for fn in captured:
                result = fn(result)
            return result
        return chained

    def map_over(self, items: List[object]) -> List[object]:
        fn = self.build()
        return [fn(item) for item in items]


class EventEmitter:
    """Event system using closures as handlers."""

    def __init__(self) -> None:
        self._handlers: Dict[str, List[Callable]] = {}

    def on(self, event: str, handler: Callable) -> None:
        if event not in self._handlers:
            self._handlers[event] = []
        self._handlers[event].append(handler)

    def emit(self, event: str, data: object) -> List[object]:
        results: List[object] = []
        for handler in self._handlers.get(event, []):
            results.append(handler(data))
        return results

    def once(self, event: str, handler: Callable) -> None:
        fired = False

        def wrapper(data: object) -> object:
            nonlocal fired
            if not fired:
                fired = True
                return handler(data)
            return None

        self.on(event, wrapper)

    def handler_count(self, event: str) -> int:
        return len(self._handlers.get(event, []))


# Untyped function 1: test inference on closure patterns
def make_filtered_aggregator(predicate, aggregator):
    collected = []

    def add(value):
        nonlocal collected
        if predicate(value):
            collected.append(value)

    def result():
        if not collected:
            return None
        return aggregator(collected)

    def count():
        return len(collected)

    return add, result, count


# Untyped function 2: test inference on HOF chains
def process_pipeline(data, steps):
    current = data
    log = []
    for step_name, step_func in steps:
        prev_len = len(current) if isinstance(current, (list, str)) else 0
        current = step_func(current)
        new_len = len(current) if isinstance(current, (list, str)) else 0
        log.append(f"{step_name}: {prev_len} -> {new_len}")
    return current, log


def partition_by(
    items: List[T], predicate: Callable[[T], bool]
) -> Tuple[List[T], List[T]]:
    """Partition a list into (matching, non-matching)."""
    matching: List[T] = []
    non_matching: List[T] = []
    for item in items:
        if predicate(item):
            matching.append(item)
        else:
            non_matching.append(item)
    return (matching, non_matching)


def group_by_key(
    items: List[T], key_func: Callable[[T], str]
) -> Dict[str, List[T]]:
    """Group items by a key function."""
    groups: Dict[str, List[T]] = {}
    for item in items:
        k = key_func(item)
        if k not in groups:
            groups[k] = []
        groups[k].append(item)
    return groups


def scan(
    values: List[T],
    func: Callable[[U, T], U],
    initial: U,
) -> List[U]:
    """Produce all intermediate accumulator states (like Haskell scanl)."""
    result: List[U] = [initial]
    acc = initial
    for v in values:
        acc = func(acc, v)
        result.append(acc)
    return result


def main() -> None:
    # Test counter closures
    inc, get, reset = make_counter(0)
    assert inc() == 1
    assert inc() == 2
    assert get() == 2
    reset()
    assert get() == 0

    # Test accumulator
    acc = make_accumulator(0.0)
    assert abs(acc(10.0) - 10.0) < 0.01
    assert abs(acc(5.0) - 15.0) < 0.01

    # Test running stats
    add_val, get_stats = make_running_stats()
    add_val(10.0)
    add_val(20.0)
    add_val(30.0)
    stats = get_stats()
    assert abs(stats["mean"] - 20.0) < 0.01
    assert stats["count"] == 3.0

    # Test history tracker
    push, get_hist, latest_fn = make_history_tracker(3)
    push("a")
    push("b")
    push("c")
    push("d")
    hist = get_hist()
    assert len(hist) == 3
    assert hist[0] == "b"
    assert latest_fn() == "d"

    # Test compose
    double_fn: Callable[[int], int] = lambda x: x * 2
    add_one: Callable[[int], int] = lambda x: x + 1
    double_then_add = compose(double_fn, add_one)
    assert double_then_add(5) == 11

    # Test pipe
    pipeline = pipe(
        lambda x: x * 2,
        lambda x: x + 10,
        lambda x: x * 3,
    )
    assert pipeline(5) == 60  # (5*2+10)*3 = 60

    # Test memoize
    call_count = 0

    def expensive_fib(n: int) -> int:
        nonlocal call_count
        call_count += 1
        if n <= 1:
            return n
        return memo_fib(n - 1) + memo_fib(n - 2)

    memo_fib = memoize(expensive_fib)
    result = memo_fib(10)
    assert result == 55

    # Test validator
    validator = make_validator([
        ("not_empty", lambda s: len(s) > 0),
        ("no_spaces", lambda s: " " not in s),
        ("lowercase", lambda s: s == s.lower()),
    ])
    assert validator("hello") == []
    assert "not_empty" in validator("")
    assert "no_spaces" in validator("hello world")

    # Test rate limiter
    limiter = make_rate_limiter(3)
    assert limiter() is True
    assert limiter() is True
    assert limiter() is True
    assert limiter() is False

    # Test partial functions
    assert add_ten(5) == 15
    assert abs(double(3.5) - 7.0) < 0.01
    assert format_currency(42.5) == "$42.50"
    assert format_percentage(99.5) == "99.5%"

    # Test reduce patterns
    assert product_of_list([1, 2, 3, 4]) == 24
    assert flatten_lists([[1, 2], [3], [4, 5]]) == [1, 2, 3, 4, 5]
    assert build_string_from_parts(["a", "b", "c"], "-") == "a-b-c"

    best = max_by_key([(1, "a"), (3, "b"), (2, "c")], lambda t: float(t[0]))
    assert best is not None and best[0] == 3

    running = running_reduce([1.0, 2.0, 3.0, 4.0], lambda a, b: a + b)
    assert running == [1.0, 3.0, 6.0, 10.0]

    # Test map/filter/zip
    result = transform_and_filter([1, 2, 3, 4, 5], lambda x: x * x, lambda x: x > 10)
    assert result == [16, 25]

    zipped = zip_with(lambda a, b: a + b, [1, 2, 3], [10, 20, 30])
    assert zipped == [11, 22, 33]

    hex_result = chain_map_filter([1, 2, 3, 4, 5])
    assert all(s.startswith("0x") for s in hex_result)

    weighted = multi_zip_aggregate(["a", "b"], [80.0, 90.0], [0.5, 0.5])
    assert abs(weighted[0][1] - 40.0) < 0.01

    # Test lambda patterns
    data = [{"x": 3, "y": 1}, {"x": 1, "y": 2}, {"x": 2, "y": 3}]
    sorted_data = sort_by_multiple_keys(data, ["x"])
    assert sorted_data[0]["x"] == 1

    lt_fn, gt_fn = create_comparators("x")
    assert lt_fn({"x": 1}, {"x": 2})
    assert gt_fn({"x": 3}, {"x": 1})

    transforms = [
        lambda x: x * 2,
        lambda x: x + 10,
        lambda x: x ** 2,
    ]
    results = apply_transformations(5.0, transforms)
    assert abs(results[0] - 10.0) < 0.01
    assert abs(results[1] - 15.0) < 0.01

    # Test FunctionChain
    chain = FunctionChain()
    fn = chain.then(lambda x: x * 2).then(lambda x: x + 1).build()
    assert fn(5) == 11

    # Test EventEmitter
    emitter = EventEmitter()
    results_list: List[object] = []
    emitter.on("data", lambda d: d * 2)
    emitter.on("data", lambda d: d + 10)
    results_list = emitter.emit("data", 5)
    assert results_list == [10, 15]

    # Test once
    emitter.once("init", lambda d: f"initialized:{d}")
    r1 = emitter.emit("init", "v1")
    r2 = emitter.emit("init", "v2")
    assert r1[0] == "initialized:v1"
    assert r2[0] is None

    # Test partition and group_by
    evens, odds = partition_by([1, 2, 3, 4, 5, 6], lambda x: x % 2 == 0)
    assert evens == [2, 4, 6]
    assert odds == [1, 3, 5]

    groups = group_by_key(["apple", "banana", "avocado", "blueberry"], lambda s: s[0])
    assert len(groups["a"]) == 2
    assert len(groups["b"]) == 2

    # Test scan
    cumsum = scan([1, 2, 3, 4], lambda acc, x: acc + x, 0)
    assert cumsum == [0, 1, 3, 6, 10]

    # Test untyped functions
    add_fn, result_fn, count_fn = make_filtered_aggregator(
        lambda x: x > 0,
        lambda xs: sum(xs) / len(xs),
    )
    add_fn(-1)
    add_fn(10)
    add_fn(20)
    add_fn(-5)
    assert count_fn() == 2
    assert abs(result_fn() - 15.0) < 0.01

    processed, log = process_pipeline(
        [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        [
            ("filter_even", lambda xs: [x for x in xs if x % 2 == 0]),
            ("double", lambda xs: [x * 2 for x in xs]),
        ],
    )
    assert processed == [4, 8, 12, 16, 20]
    assert len(log) == 2


if __name__ == "__main__":
    main()
