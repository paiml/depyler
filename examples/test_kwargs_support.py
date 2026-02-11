"""
Test keyword argument support via transpiler-friendly patterns.

Since the transpiler does not support **kwargs, keyword arguments,
default parameter values in calls, or class definitions, this file
uses explicit positional parameters and pure functions to test the
same algorithmic content: configuring settings, greeting, calculating,
and composing function calls.
"""


def greet_person(name: str, greeting: str) -> str:
    """Greet a person with a given greeting."""
    result: str = greeting + ", " + name + "!"
    return result


def calculate_op(a: int, b: int, mode: int) -> int:
    """Calculate based on mode: 1=add, 2=sub."""
    result: int = 0
    if mode == 1:
        result = a + b
    else:
        result = a - b
    return result


def make_config(width: int, height: int, title: str) -> dict[str, str]:
    """Create a configuration dictionary."""
    cfg: dict[str, str] = {}
    ws: str = str(width)
    hs: str = str(height)
    ts: str = title + ""
    cfg["width"] = ws
    cfg["height"] = hs
    cfg["title"] = ts
    return cfg


def inner_calc(x: int, y: int) -> int:
    """Simple addition helper."""
    result: int = x + y
    return result


def outer_calc(inner_result: int, scale: int, off: int) -> int:
    """Scale a result and add offset."""
    result: int = inner_result * scale + off
    return result


def get_height() -> int:
    """Return a default height."""
    return 600


def demo_function_calls() -> int:
    """Test function calls with explicit parameters."""
    r1: str = greet_person("Alice", "Hello")
    r2: int = calculate_op(10, 20, 1)
    r3: int = calculate_op(10, 20, 2)
    total: int = r2 + r3
    return total


def demo_config() -> dict[str, str]:
    """Test creating a configuration."""
    cfg: dict[str, str] = make_config(800, 600, "My App")
    return cfg


def demo_nested_calls() -> int:
    """Test nested function calls."""
    v1: int = inner_calc(10, 20)
    v2: int = inner_calc(5, 5)
    result: int = outer_calc(v1, 2, v2)
    return result


def demo_complex_calls() -> int:
    """Test complex expression parameters."""
    w: int = 100 + 200
    h: int = get_height()
    cfg: dict[str, str] = make_config(w, h, "App42")
    len_title: int = len(cfg["title"])
    return len_title


def setup_state(mode: int, timeout: int, retry: int) -> dict[str, str]:
    """Set up state using a dictionary instead of a class."""
    state: dict[str, str] = {}
    ms: str = str(mode)
    ts: str = str(timeout)
    rs: str = str(retry)
    state["mode"] = ms
    state["timeout"] = ts
    state["retry"] = rs
    return state


def demo_method_calls() -> str:
    """Test method-like calls using state dict."""
    state: dict[str, str] = setup_state(2, 30, 1)
    text: str = "hello world"
    formatted: str = text.replace("world", "Python")
    return formatted


def test_module() -> int:
    """Run all kwargs-equivalent tests and count passes."""
    ok: int = 0

    g: str = greet_person("Alice", "Hello")
    if g == "Hello, Alice!":
        ok = ok + 1

    r1: int = calculate_op(10, 20, 1)
    if r1 == 30:
        ok = ok + 1

    r2: int = calculate_op(10, 20, 2)
    if r2 == 0 - 10:
        ok = ok + 1

    cfg: dict[str, str] = make_config(800, 600, "MyApp")
    if cfg["title"] == "MyApp":
        ok = ok + 1
    if cfg["width"] == "800":
        ok = ok + 1

    v1: int = inner_calc(10, 20)
    if v1 == 30:
        ok = ok + 1

    v2: int = inner_calc(5, 5)
    result: int = outer_calc(v1, 2, v2)
    if result == 70:
        ok = ok + 1

    h: int = get_height()
    if h == 600:
        ok = ok + 1

    total: int = demo_function_calls()
    if total == 20:
        ok = ok + 1

    nested: int = demo_nested_calls()
    if nested == 70:
        ok = ok + 1

    fmt: str = demo_method_calls()
    if fmt == "hello Python":
        ok = ok + 1

    state: dict[str, str] = setup_state(2, 30, 1)
    if state["mode"] == "2":
        ok = ok + 1

    return ok
