"""Pathological decorator and metaclass-like patterns for transpiler stress testing.

Tests decorator stacking, decorator factories, class decorators,
__init_subclass__ hooks, and signature-modifying decorators.
"""

from typing import List, Dict, Callable, Any, TypeVar

T = TypeVar("T")


# --- Decorator Factories ---

def repeat(n: int) -> Callable:
    """Decorator factory: call the wrapped function n times, collect results."""
    def decorator(func: Callable) -> Callable:
        def wrapper(*args, **kwargs):
            results = []
            for _ in range(n):
                results.append(func(*args, **kwargs))
            return results
        wrapper.__name__ = func.__name__
        wrapper.__doc__ = func.__doc__
        return wrapper
    return decorator


def validate_positive(func: Callable) -> Callable:
    """Decorator that validates all numeric arguments are positive."""
    def wrapper(*args, **kwargs):
        for i, arg in enumerate(args):
            if isinstance(arg, (int, float)) and arg < 0:
                raise ValueError(f"Argument {i} must be positive, got {arg}")
        for key, val in kwargs.items():
            if isinstance(val, (int, float)) and val < 0:
                raise ValueError(f"Keyword argument '{key}' must be positive, got {val}")
        return func(*args, **kwargs)
    wrapper.__name__ = func.__name__
    return wrapper


def memoize(func: Callable) -> Callable:
    """Memoization decorator using a closure-captured dict."""
    cache: Dict = {}
    def wrapper(*args):
        if args not in cache:
            cache[args] = func(*args)
        return cache[args]
    wrapper.__name__ = func.__name__
    wrapper.cache = cache  # type: ignore
    return wrapper


def trace(label: str) -> Callable:
    """Decorator factory that logs entry/exit with a label."""
    def decorator(func: Callable) -> Callable:
        def wrapper(*args, **kwargs):
            call_info = f"[{label}] ENTER {func.__name__}({args}, {kwargs})"
            result = func(*args, **kwargs)
            exit_info = f"[{label}] EXIT  {func.__name__} -> {result}"
            return result
        wrapper.__name__ = func.__name__
        wrapper._trace_label = label  # type: ignore
        return wrapper
    return decorator


def deprecated(message: str) -> Callable:
    """Decorator factory marking a function as deprecated."""
    def decorator(func):
        def wrapper(*args, **kwargs):
            # In a real system this would emit a warning
            return func(*args, **kwargs)
        wrapper.__name__ = func.__name__
        wrapper._deprecated_msg = message  # type: ignore
        wrapper._is_deprecated = True  # type: ignore
        return wrapper
    return decorator


# --- Class Decorator ---

def add_comparison_methods(cls):
    """Class decorator that adds __eq__ and __lt__ based on a 'value' attribute."""
    original_init = cls.__init__

    def new_eq(self, other):
        if not isinstance(other, cls):
            return NotImplemented
        return self.value == other.value

    def new_lt(self, other):
        if not isinstance(other, cls):
            return NotImplemented
        return self.value < other.value

    def new_le(self, other):
        if not isinstance(other, cls):
            return NotImplemented
        return self.value <= other.value

    cls.__eq__ = new_eq
    cls.__lt__ = new_lt
    cls.__le__ = new_le
    return cls


def singleton(cls):
    """Class decorator that makes a class a singleton."""
    instances: Dict = {}
    original_new = cls.__new__ if hasattr(cls, '__new__') else None

    def get_instance(*args, **kwargs):
        if cls not in instances:
            instances[cls] = cls.__new__(cls)
            cls.__init__(instances[cls], *args, **kwargs)
        return instances[cls]

    get_instance._cls = cls  # type: ignore
    get_instance._instances = instances  # type: ignore
    return get_instance


# --- Stacked decorators on functions ---

@validate_positive
@memoize
def fibonacci(n: int) -> int:
    """Compute fibonacci with memoization and positive validation."""
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)


@trace("MATH")
@validate_positive
def safe_divide(a: float, b: float) -> float:
    """Divide a by b, both must be positive."""
    if b == 0.0:
        return 0.0
    return a / b


@repeat(3)
def generate_square(x: int) -> int:
    """Return x squared, repeated by decorator."""
    return x * x


@deprecated("Use new_compute instead")
@trace("LEGACY")
def old_compute(x, y):
    """Legacy computation function - untyped on purpose."""
    return x * 2 + y * 3


# --- __init_subclass__ pattern ---

class PluginBase:
    """Base class that auto-registers subclasses via __init_subclass__."""
    _registry: Dict[str, type] = {}

    def __init_subclass__(cls, plugin_name: str = "", **kwargs):
        super().__init_subclass__(**kwargs)
        name = plugin_name if plugin_name else cls.__name__.lower()
        PluginBase._registry[name] = cls

    @classmethod
    def get_registry(cls) -> Dict[str, type]:
        return dict(cls._registry)

    def execute(self) -> str:
        return f"PluginBase.execute()"


class AlphaPlugin(PluginBase, plugin_name="alpha"):
    def __init__(self, power: int = 1):
        self.power = power

    def execute(self) -> str:
        return f"Alpha(power={self.power})"

    def compute(self, x: int) -> int:
        result = 1
        for _ in range(self.power):
            result *= x
        return result


class BetaPlugin(PluginBase, plugin_name="beta"):
    def __init__(self, offset: int = 0):
        self.offset = offset

    def execute(self) -> str:
        return f"Beta(offset={self.offset})"

    def transform(self, values: List[int]) -> List[int]:
        return [v + self.offset for v in values]


class GammaPlugin(PluginBase, plugin_name="gamma"):
    """Inherits from PluginBase with custom behavior."""
    def __init__(self):
        self.history: List[str] = []

    def execute(self) -> str:
        self.history.append("executed")
        return f"Gamma(calls={len(self.history)})"


# --- Decorated class ---

@add_comparison_methods
class Priority:
    def __init__(self, name: str, value: int):
        self.name = name
        self.value = value

    def __repr__(self) -> str:
        return f"Priority({self.name!r}, {self.value})"


# --- Untyped helper functions (>30% untyped) ---

def apply_chain(value, *funcs):
    """Apply a chain of functions left to right - untyped."""
    result = value
    for f in funcs:
        result = f(result)
    return result


def make_adder(n):
    """Return a closure that adds n - untyped."""
    def adder(x):
        return x + n
    return adder


def compose(f, g):
    """Compose two functions: compose(f, g)(x) == f(g(x)) - untyped."""
    def composed(x):
        return f(g(x))
    composed.__name__ = f"compose({getattr(f, '__name__', '?')}, {getattr(g, '__name__', '?')})"
    return composed


def power_of(exp):
    """Return a function that raises its argument to exp - untyped."""
    def raiser(x):
        result = 1
        for _ in range(exp):
            result *= x
        return result
    return raiser


def conditional_decorator(condition, decorator):
    """Apply decorator only if condition is true - untyped."""
    def wrapper(func):
        if condition:
            return decorator(func)
        return func
    return wrapper


def build_pipeline(*steps):
    """Build a data processing pipeline from steps - untyped."""
    def pipeline(data):
        result = data
        for step in steps:
            result = step(result)
        return result
    return pipeline


# --- Integration functions ---

def test_decorator_stacking() -> bool:
    """Test that stacked decorators work correctly."""
    # fibonacci with memoize + validate_positive
    assert fibonacci(10) == 55
    assert fibonacci(0) == 0
    assert fibonacci(1) == 1

    # safe_divide with trace + validate_positive
    result = safe_divide(10.0, 3.0)
    assert abs(result - 3.3333333333333335) < 1e-9

    # repeat decorator
    squares = generate_square(5)
    assert squares == [25, 25, 25]
    assert len(squares) == 3

    # deprecated + trace stacking
    assert old_compute(2, 3) == 13

    return True


def test_plugin_registry() -> bool:
    """Test __init_subclass__ plugin registration."""
    registry = PluginBase.get_registry()
    assert "alpha" in registry
    assert "beta" in registry
    assert "gamma" in registry
    assert len(registry) >= 3

    alpha = AlphaPlugin(power=3)
    assert alpha.execute() == "Alpha(power=3)"
    assert alpha.compute(2) == 8

    beta = BetaPlugin(offset=10)
    assert beta.transform([1, 2, 3]) == [11, 12, 13]

    gamma = GammaPlugin()
    gamma.execute()
    gamma.execute()
    assert gamma.execute() == "Gamma(calls=3)"

    return True


def test_class_decorator() -> bool:
    """Test class decorator that adds comparison methods."""
    p1 = Priority("low", 1)
    p2 = Priority("med", 5)
    p3 = Priority("high", 10)
    p4 = Priority("also_med", 5)

    assert p1 < p2  # type: ignore
    assert p3 > p2  # type: ignore
    assert p2 == p4
    assert p1 <= p2  # type: ignore
    assert repr(p1) == "Priority('low', 1)"

    return True


def test_function_composition() -> bool:
    """Test untyped composition and pipeline functions."""
    add5 = make_adder(5)
    assert add5(10) == 15

    square = power_of(2)
    cube = power_of(3)
    assert square(4) == 16
    assert cube(3) == 27

    add5_then_square = compose(square, add5)
    assert add5_then_square(3) == 64  # (3+5)^2 = 64

    result = apply_chain(2, make_adder(3), power_of(2), make_adder(-1))
    assert result == 24  # ((2+3)^2) - 1 = 24

    pipeline = build_pipeline(
        make_adder(10),
        power_of(2),
        make_adder(-100),
    )
    assert pipeline(5) == 125  # (5+10)^2 - 100 = 125

    return True


def test_all() -> bool:
    """Run all tests."""
    assert test_decorator_stacking()
    assert test_plugin_registry()
    assert test_class_decorator()
    assert test_function_composition()
    return True


def main():
    """Entry point."""
    if test_all():
        print("hard_decorator_metaclass: ALL TESTS PASSED")
    else:
        print("hard_decorator_metaclass: TESTS FAILED")


if __name__ == "__main__":
    main()
