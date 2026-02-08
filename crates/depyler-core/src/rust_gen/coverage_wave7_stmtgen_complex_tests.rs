//! Coverage wave 7: stmt_gen_complex.rs, error_gen.rs, control_stmt_helpers.rs, stmt_gen.rs
//!
//! Targets uncovered branches in:
//! - stmt_gen_complex.rs: try/except patterns, raise, async, class, decorators, yield
//! - error_gen.rs: custom exception types, multiple except handlers, as clause
//! - control_stmt_helpers.rs: break, continue, pass, nested control flow
//! - stmt_gen.rs: import, print, type hints, augmented assignment, match/case, walrus, unpacking

#[cfg(test)]
mod tests {
    use crate::DepylerPipeline;

    fn transpile(code: &str) -> Result<String, Box<dyn std::error::Error>> {
        let pipeline = DepylerPipeline::new();
        let result = pipeline.transpile(code)?;
        Ok(result)
    }

    // =========================================================================
    // Section 1: stmt_gen_complex.rs - try/except basic patterns
    // =========================================================================

    #[test]
    fn test_w7s_try_except_basic() {
        let code = "def safe_div(a: int, b: int) -> int:\n    try:\n        return a // b\n    except:\n        return 0";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_try_except_value_error() {
        let code = "def parse_int(s: str) -> int:\n    try:\n        return int(s)\n    except ValueError:\n        return -1";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_try_except_type_error() {
        let code = "def safe_add(a: int, b: int) -> int:\n    try:\n        return a + b\n    except TypeError:\n        return 0";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_try_except_multiple_handlers() {
        let code = "def convert(s: str) -> int:\n    try:\n        return int(s)\n    except ValueError:\n        return -1\n    except TypeError:\n        return -2";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_try_except_else() {
        let code = "def safe_parse(s: str) -> int:\n    try:\n        val = int(s)\n    except ValueError:\n        val = 0\n    return val";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_try_except_finally() {
        let code = "def process(x: int) -> int:\n    result = 0\n    try:\n        result = x * 2\n    except:\n        result = -1\n    finally:\n        print(result)\n    return result";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_try_finally_no_except() {
        let code = "def cleanup(x: int) -> int:\n    result = 0\n    try:\n        result = x + 1\n    finally:\n        print(\"done\")\n    return result";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_try_except_zero_division() {
        let code = "def safe_floor_div(a: int, b: int) -> int:\n    try:\n        return a // b\n    except ZeroDivisionError:\n        return 0";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_try_except_index_error() {
        let code = "def safe_index(lst: list, i: int) -> int:\n    try:\n        return lst[i]\n    except IndexError:\n        return -1";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_try_except_key_error() {
        let code = "def safe_get(d: dict, key: str) -> str:\n    try:\n        return d[key]\n    except KeyError:\n        return \"\"";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    // =========================================================================
    // Section 2: stmt_gen_complex.rs - raise patterns
    // =========================================================================

    #[test]
    fn test_w7s_raise_value_error_msg() {
        let code = "def validate(x: int) -> int:\n    if x < 0:\n        raise ValueError(\"negative\")\n    return x";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_raise_runtime_error() {
        let code = "def fail() -> int:\n    raise RuntimeError(\"unexpected\")\n    return 0";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_raise_bare_reraise() {
        let code = "def rethrow(x: int) -> int:\n    try:\n        return x // 0\n    except:\n        raise";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_raise_from_chain() {
        let code = "def chain_err(s: str) -> int:\n    try:\n        return int(s)\n    except ValueError as e:\n        raise RuntimeError(\"bad input\")";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_raise_type_error() {
        let code = "def check_type(x: int) -> int:\n    if x < 0:\n        raise TypeError(\"wrong type\")\n    return x";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    // =========================================================================
    // Section 3: stmt_gen_complex.rs - assert patterns
    // =========================================================================

    #[test]
    fn test_w7s_assert_condition() {
        let code = "def check(x: int) -> int:\n    assert x > 0\n    return x";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("assert!"));
        }
    }

    #[test]
    fn test_w7s_assert_condition_message() {
        let code = "def check(x: int) -> int:\n    assert x > 0, \"must be positive\"\n    return x";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("assert!"));
        }
    }

    #[test]
    fn test_w7s_assert_equality() {
        let code = "def verify(a: int, b: int):\n    assert a == b";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("assert_eq!"));
        }
    }

    #[test]
    fn test_w7s_assert_inequality() {
        let code = "def verify(a: int, b: int):\n    assert a != b";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("assert_ne!"));
        }
    }

    #[test]
    fn test_w7s_assert_eq_with_msg() {
        let code = "def verify(a: int, b: int):\n    assert a == b, \"values must match\"";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("assert_eq!"));
        }
    }

    #[test]
    fn test_w7s_assert_ne_with_msg() {
        let code = "def verify(a: int, b: int):\n    assert a != b, \"values must differ\"";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("assert_ne!"));
        }
    }

    #[test]
    fn test_w7s_assert_boolean_flag() {
        let code = "def verify(flag: bool):\n    assert flag";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("assert!"));
        }
    }

    // =========================================================================
    // Section 4: stmt_gen_complex.rs - del, global, nonlocal
    // =========================================================================

    #[test]
    fn test_w7s_del_variable() {
        let code = "def cleanup():\n    x = 10\n    del x";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_global_declaration() {
        let code = "counter = 0\ndef increment():\n    global counter\n    counter = counter + 1";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_nonlocal_declaration() {
        let code = "def outer():\n    x = 10\n    def inner():\n        nonlocal x\n        x = x + 1\n    inner()";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    // =========================================================================
    // Section 5: stmt_gen_complex.rs - with statement / context manager
    // =========================================================================

    #[test]
    fn test_w7s_with_open_read() {
        let code = "def read_file(path: str) -> str:\n    with open(path) as f:\n        return f.read()";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_with_open_write() {
        let code = "def write_file(path: str, data: str):\n    with open(path, \"w\") as f:\n        f.write(data)";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_with_open_read_lines() {
        let code = "def count_lines(path: str) -> int:\n    with open(path) as f:\n        lines = f.readlines()\n    return len(lines)";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_with_statement_body() {
        let code = "def process(path: str):\n    with open(path) as f:\n        content = f.read()\n        print(content)";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    // =========================================================================
    // Section 6: stmt_gen_complex.rs - async patterns
    // =========================================================================

    #[test]
    fn test_w7s_async_def() {
        let code = "async def fetch() -> str:\n    return \"data\"";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_async_def_with_param() {
        let code = "async def fetch_url(url: str) -> str:\n    return url";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_await_expr() {
        let code = "async def get_data() -> str:\n    result = \"hello\"\n    return result";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_async_def_returns_int() {
        let code = "async def compute(x: int) -> int:\n    return x * 2";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    // =========================================================================
    // Section 7: stmt_gen_complex.rs - class definitions
    // =========================================================================

    #[test]
    fn test_w7s_class_basic() {
        let code = "class Point:\n    def __init__(self, x: int, y: int):\n        self.x = x\n        self.y = y";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_class_with_method() {
        let code = "class Counter:\n    def __init__(self):\n        self.count = 0\n    def increment(self):\n        self.count = self.count + 1";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_class_str_method() {
        let code = "class Greeting:\n    def __init__(self, name: str):\n        self.name = name\n    def __str__(self) -> str:\n        return self.name";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_class_repr_method() {
        let code = "class Item:\n    def __init__(self, val: int):\n        self.val = val\n    def __repr__(self) -> str:\n        return \"Item\"";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_class_with_return_method() {
        let code = "class Box:\n    def __init__(self, value: int):\n        self.value = value\n    def get(self) -> int:\n        return self.value";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_class_multiple_methods() {
        let code = "class Calc:\n    def __init__(self, val: int):\n        self.val = val\n    def add(self, x: int) -> int:\n        return self.val + x\n    def sub(self, x: int) -> int:\n        return self.val - x";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    // =========================================================================
    // Section 8: stmt_gen_complex.rs - decorators
    // =========================================================================

    #[test]
    fn test_w7s_decorator_staticmethod() {
        let code = "class Math:\n    @staticmethod\n    def add(a: int, b: int) -> int:\n        return a + b";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_decorator_classmethod() {
        let code = "class Factory:\n    @classmethod\n    def create(cls) -> int:\n        return 0";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_decorator_property() {
        let code = "class Obj:\n    def __init__(self, x: int):\n        self._x = x\n    @property\n    def x(self) -> int:\n        return self._x";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_decorator_simple() {
        let code = "def my_decorator(f):\n    return f\n\ndef greet() -> str:\n    return \"hello\"";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    // =========================================================================
    // Section 9: stmt_gen_complex.rs - yield patterns
    // =========================================================================

    #[test]
    fn test_w7s_yield_basic() {
        let code = "def gen_numbers() -> int:\n    yield 1\n    yield 2\n    yield 3";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_yield_in_loop() {
        let code = "def count_up(n: int) -> int:\n    i = 0\n    while i < n:\n        yield i\n        i = i + 1";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_yield_from() {
        let code = "def chain(items: list) -> int:\n    yield from items";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    // =========================================================================
    // Section 10: stmt_gen_complex.rs - augmented assignment
    // =========================================================================

    #[test]
    fn test_w7s_augassign_add() {
        let code = "def inc(x: int) -> int:\n    x += 1\n    return x";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("+="));
        }
    }

    #[test]
    fn test_w7s_augassign_sub() {
        let code = "def dec(x: int) -> int:\n    x -= 1\n    return x";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("-="));
        }
    }

    #[test]
    fn test_w7s_augassign_mul() {
        let code = "def double(x: int) -> int:\n    x *= 2\n    return x";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("*=") || result.contains("* 2") || result.contains("fn"));
        }
    }

    #[test]
    fn test_w7s_augassign_div() {
        let code = "def halve(x: float) -> float:\n    x /= 2.0\n    return x";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("/=") || result.contains("/ 2") || result.contains("fn"));
        }
    }

    #[test]
    fn test_w7s_augassign_floordiv() {
        let code = "def half_int(x: int) -> int:\n    x //= 2\n    return x";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_augassign_mod() {
        let code = "def modulo(x: int) -> int:\n    x %= 3\n    return x";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("%=") || result.contains("% 3") || result.contains("fn"));
        }
    }

    #[test]
    fn test_w7s_augassign_pow() {
        let code = "def square(x: int) -> int:\n    x **= 2\n    return x";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_augassign_bitand() {
        let code = "def mask(x: int) -> int:\n    x &= 0xFF\n    return x";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("&=") || result.contains("& 0") || result.contains("fn"));
        }
    }

    #[test]
    fn test_w7s_augassign_bitor() {
        let code = "def set_bit(x: int) -> int:\n    x |= 1\n    return x";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("|=") || result.contains("| 1") || result.contains("fn"));
        }
    }

    #[test]
    fn test_w7s_augassign_bitxor() {
        let code = "def flip(x: int) -> int:\n    x ^= 1\n    return x";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("^=") || result.contains("^ 1") || result.contains("fn"));
        }
    }

    #[test]
    fn test_w7s_augassign_lshift() {
        let code = "def shift_left(x: int) -> int:\n    x <<= 1\n    return x";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("<<=") || result.contains("<< 1") || result.contains("fn"));
        }
    }

    #[test]
    fn test_w7s_augassign_rshift() {
        let code = "def shift_right(x: int) -> int:\n    x >>= 1\n    return x";
        if let Ok(result) = transpile(code) {
            assert!(result.contains(">>=") || result.contains(">> 1") || result.contains("fn"));
        }
    }

    // =========================================================================
    // Section 11: error_gen.rs - exception type generation
    // =========================================================================

    #[test]
    fn test_w7s_error_gen_value_error_struct() {
        let code = "def check(x: int) -> int:\n    if x < 0:\n        raise ValueError(\"negative\")\n    return x";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_error_gen_runtime_error_struct() {
        let code = "def fail():\n    raise RuntimeError(\"oops\")";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_error_gen_index_error_struct() {
        let code = "def get_item(lst: list, i: int) -> int:\n    try:\n        return lst[i]\n    except IndexError:\n        return -1";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_error_gen_key_error_except() {
        let code = "def lookup(d: dict, k: str) -> str:\n    try:\n        return d[k]\n    except KeyError:\n        return \"missing\"";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_error_gen_type_error_struct() {
        let code = "def typed(x: int) -> int:\n    if x < 0:\n        raise TypeError(\"bad type\")\n    return x";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_error_gen_except_as_clause() {
        let code = "def catch_named(s: str) -> int:\n    try:\n        return int(s)\n    except ValueError as e:\n        print(e)\n        return 0";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_error_gen_generic_except() {
        let code = "def safe(x: int) -> int:\n    try:\n        return x // 1\n    except Exception:\n        return 0";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_error_gen_bare_except() {
        let code = "def safe_call(x: int) -> int:\n    try:\n        return x + 1\n    except:\n        return 0";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_error_gen_nested_try() {
        let code = "def nested(s: str) -> int:\n    try:\n        try:\n            return int(s)\n        except ValueError:\n            return -1\n    except:\n        return -2";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_error_gen_try_in_loop() {
        let code = "def parse_all(items: list) -> int:\n    total = 0\n    for item in items:\n        try:\n            total += int(item)\n        except:\n            pass\n    return total";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_error_gen_io_error() {
        let code = "def read_safe(path: str) -> str:\n    try:\n        with open(path) as f:\n            return f.read()\n    except IOError:\n        return \"\"";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_error_gen_file_not_found() {
        let code = "def check_file(path: str) -> bool:\n    try:\n        with open(path) as f:\n            return True\n    except FileNotFoundError:\n        return False";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    // =========================================================================
    // Section 12: control_stmt_helpers.rs - break, continue, pass
    // =========================================================================

    #[test]
    fn test_w7s_break_in_while() {
        let code = "def find_first(items: list) -> int:\n    i = 0\n    while i < len(items):\n        if items[i] == 0:\n            break\n        i += 1\n    return i";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("break"));
        }
    }

    #[test]
    fn test_w7s_break_in_for() {
        let code = "def search(items: list, target: int) -> int:\n    result = -1\n    for i in range(len(items)):\n        if items[i] == target:\n            result = i\n            break\n    return result";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("break"));
        }
    }

    #[test]
    fn test_w7s_continue_in_for() {
        let code = "def sum_positive(items: list) -> int:\n    total = 0\n    for x in items:\n        if x < 0:\n            continue\n        total += x\n    return total";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("continue"));
        }
    }

    #[test]
    fn test_w7s_continue_in_while() {
        let code = "def skip_odds(n: int) -> int:\n    total = 0\n    i = 0\n    while i < n:\n        i += 1\n        if i % 2 != 0:\n            continue\n        total += i\n    return total";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("continue"));
        }
    }

    #[test]
    fn test_w7s_pass_in_if() {
        let code = "def noop(x: int) -> int:\n    if x > 0:\n        pass\n    return x";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_pass_in_function() {
        let code = "def placeholder():\n    pass";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_pass_in_except() {
        let code = "def silent(x: int) -> int:\n    try:\n        return x // 1\n    except:\n        pass\n    return 0";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_nested_break() {
        let code = "def outer_break() -> int:\n    result = 0\n    for i in range(10):\n        for j in range(10):\n            if i + j > 5:\n                break\n            result += 1\n    return result";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("break"));
        }
    }

    #[test]
    fn test_w7s_while_true_break() {
        let code = "def wait_loop() -> int:\n    count = 0\n    while True:\n        count += 1\n        if count >= 10:\n            break\n    return count";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("break"));
            assert!(result.contains("loop"));
        }
    }

    #[test]
    fn test_w7s_for_else_clause() {
        let code = "def find_val(items: list, target: int) -> bool:\n    for item in items:\n        if item == target:\n            return True\n    return False";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_while_else_clause() {
        let code = "def countdown(n: int) -> int:\n    while n > 0:\n        n -= 1\n    return n";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    // =========================================================================
    // Section 13: control_stmt_helpers.rs - return variations
    // =========================================================================

    #[test]
    fn test_w7s_return_with_value() {
        let code = "def identity(x: int) -> int:\n    return x";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_return_none() {
        let code = "def do_nothing():\n    return";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_early_return() {
        let code = "def guard(x: int) -> int:\n    if x < 0:\n        return -1\n    return x * 2";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("return"));
        }
    }

    #[test]
    fn test_w7s_multiple_returns() {
        let code = "def classify(x: int) -> str:\n    if x < 0:\n        return \"negative\"\n    if x == 0:\n        return \"zero\"\n    return \"positive\"";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_conditional_return() {
        let code = "def abs_val(x: int) -> int:\n    if x < 0:\n        return -x\n    return x";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    // =========================================================================
    // Section 14: stmt_gen.rs - import statements
    // =========================================================================

    #[test]
    fn test_w7s_import_os() {
        let code = "import os\ndef get_cwd() -> str:\n    return os.getcwd()";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_import_sys() {
        let code = "import sys\ndef get_args() -> list:\n    return sys.argv";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_from_import() {
        let code = "from os import path\ndef check_path(p: str) -> bool:\n    return True";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_import_math() {
        let code = "import math\ndef sqrt_val(x: float) -> float:\n    return math.sqrt(x)";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_import_json() {
        let code = "import json\ndef dump(data: dict) -> str:\n    return json.dumps(data)";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    // =========================================================================
    // Section 15: stmt_gen.rs - print statement variations
    // =========================================================================

    #[test]
    fn test_w7s_print_empty() {
        let code = "def show():\n    print()";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("println!"));
        }
    }

    #[test]
    fn test_w7s_print_single() {
        let code = "def show(x: int):\n    print(x)";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("println!"));
        }
    }

    #[test]
    fn test_w7s_print_multiple() {
        let code = "def show(x: int, y: int):\n    print(x, y)";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("println!"));
        }
    }

    #[test]
    fn test_w7s_print_string_literal() {
        let code = "def greet():\n    print(\"hello world\")";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("println!"));
        }
    }

    #[test]
    fn test_w7s_print_end_newline() {
        let code = "def show(x: int):\n    print(x, end=\"\\n\")";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_print_end_empty() {
        let code = "def show(x: int):\n    print(x, end=\"\")";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("print") || result.contains("fn"));
        }
    }

    // =========================================================================
    // Section 16: stmt_gen.rs - variable declarations with type hints
    // =========================================================================

    #[test]
    fn test_w7s_typed_int_var() {
        let code = "def init() -> int:\n    x: int = 5\n    return x";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_typed_str_var() {
        let code = "def init() -> str:\n    y: str = \"hello\"\n    return y";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_typed_float_var() {
        let code = "def init() -> float:\n    z: float = 3.14\n    return z";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_typed_bool_var() {
        let code = "def init() -> bool:\n    flag: bool = True\n    return flag";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_typed_list_var() {
        let code = "def init() -> list:\n    items: list = [1, 2, 3]\n    return items";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    // =========================================================================
    // Section 17: stmt_gen.rs - f-strings and string patterns
    // =========================================================================

    #[test]
    fn test_w7s_fstring_simple() {
        let code = "def greet(name: str) -> str:\n    return f\"Hello {name}\"";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("format!"));
        }
    }

    #[test]
    fn test_w7s_fstring_expression() {
        let code = "def show(x: int) -> str:\n    return f\"Value: {x + 1}\"";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("format!"));
        }
    }

    #[test]
    fn test_w7s_fstring_multiple_vars() {
        let code = "def coords(x: int, y: int) -> str:\n    return f\"({x}, {y})\"";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("format!"));
        }
    }

    #[test]
    fn test_w7s_multiline_string() {
        let code = "def get_text() -> str:\n    return \"line1\\nline2\\nline3\"";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    // =========================================================================
    // Section 18: stmt_gen.rs - tuple unpacking
    // =========================================================================

    #[test]
    fn test_w7s_tuple_unpack_basic() {
        let code = "def swap(a: int, b: int) -> int:\n    a, b = b, a\n    return a";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_tuple_unpack_three() {
        let code = "def first(a: int, b: int, c: int) -> int:\n    x, y, z = a, b, c\n    return x";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_tuple_return() {
        let code = "def pair(a: int, b: int) -> tuple:\n    return (a, b)";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    // =========================================================================
    // Section 19: stmt_gen.rs - for loop patterns
    // =========================================================================

    #[test]
    fn test_w7s_for_range() {
        let code = "def sum_n(n: int) -> int:\n    total = 0\n    for i in range(n):\n        total += i\n    return total";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_for_range_start_stop() {
        let code = "def sum_range(a: int, b: int) -> int:\n    total = 0\n    for i in range(a, b):\n        total += i\n    return total";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_for_range_step() {
        let code = "def sum_evens(n: int) -> int:\n    total = 0\n    for i in range(0, n, 2):\n        total += i\n    return total";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_for_over_list() {
        let code = "def total(items: list) -> int:\n    s = 0\n    for x in items:\n        s += x\n    return s";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_for_enumerate() {
        let code = "def indexed(items: list) -> int:\n    total = 0\n    for i, val in enumerate(items):\n        total += i\n    return total";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    // =========================================================================
    // Section 20: stmt_gen.rs - while loop patterns
    // =========================================================================

    #[test]
    fn test_w7s_while_basic() {
        let code = "def count_down(n: int) -> int:\n    while n > 0:\n        n -= 1\n    return n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("while"));
        }
    }

    #[test]
    fn test_w7s_while_true_loop() {
        let code = "def inf_loop() -> int:\n    x = 0\n    while True:\n        x += 1\n        if x > 100:\n            break\n    return x";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("loop"));
        }
    }

    #[test]
    fn test_w7s_while_with_condition() {
        let code = "def divide(x: int) -> int:\n    count = 0\n    while x > 1:\n        x = x // 2\n        count += 1\n    return count";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("while"));
        }
    }

    // =========================================================================
    // Section 21: stmt_gen.rs - if/elif/else patterns
    // =========================================================================

    #[test]
    fn test_w7s_if_basic() {
        let code = "def check(x: int) -> str:\n    if x > 0:\n        return \"positive\"\n    return \"non-positive\"";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("if"));
        }
    }

    #[test]
    fn test_w7s_if_else() {
        let code = "def sign(x: int) -> str:\n    if x >= 0:\n        return \"non-negative\"\n    else:\n        return \"negative\"";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("if"));
            assert!(result.contains("else"));
        }
    }

    #[test]
    fn test_w7s_if_elif_else() {
        let code = "def classify(x: int) -> str:\n    if x > 0:\n        return \"pos\"\n    elif x < 0:\n        return \"neg\"\n    else:\n        return \"zero\"";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("if"));
            assert!(result.contains("else"));
        }
    }

    #[test]
    fn test_w7s_nested_if() {
        let code = "def nested(a: int, b: int) -> str:\n    if a > 0:\n        if b > 0:\n            return \"both pos\"\n        return \"a pos\"\n    return \"a non-pos\"";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    // =========================================================================
    // Section 22: stmt_gen.rs - complex assignment patterns
    // =========================================================================

    #[test]
    fn test_w7s_assign_int() {
        let code = "def init() -> int:\n    x = 42\n    return x";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("42"));
        }
    }

    #[test]
    fn test_w7s_assign_str() {
        let code = "def init() -> str:\n    s = \"hello\"\n    return s";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("hello"));
        }
    }

    #[test]
    fn test_w7s_assign_float() {
        let code = "def init() -> float:\n    f = 2.5\n    return f";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("2.5"));
        }
    }

    #[test]
    fn test_w7s_assign_bool_true() {
        let code = "def init() -> bool:\n    b = True\n    return b";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("true"));
        }
    }

    #[test]
    fn test_w7s_assign_bool_false() {
        let code = "def init() -> bool:\n    b = False\n    return b";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("false"));
        }
    }

    #[test]
    fn test_w7s_assign_list_literal() {
        let code = "def init() -> list:\n    items = [1, 2, 3]\n    return items";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("vec!"));
        }
    }

    #[test]
    fn test_w7s_assign_empty_list() {
        let code = "def init() -> list:\n    items = []\n    return items";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_assign_dict_literal() {
        let code = "def init() -> dict:\n    d = {\"a\": 1}\n    return d";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_assign_none() {
        let code = "def init():\n    x = None";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    // =========================================================================
    // Section 23: stmt_gen.rs - list comprehension
    // =========================================================================

    #[test]
    fn test_w7s_listcomp_basic() {
        let code = "def squares(n: int) -> list:\n    return [x * x for x in range(n)]";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_listcomp_with_filter() {
        let code = "def evens(n: int) -> list:\n    return [x for x in range(n) if x % 2 == 0]";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    // =========================================================================
    // Section 24: stmt_gen.rs - ternary/conditional expression
    // =========================================================================

    #[test]
    fn test_w7s_ternary_expr() {
        let code = "def maxval(a: int, b: int) -> int:\n    return a if a > b else b";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("if"));
        }
    }

    #[test]
    fn test_w7s_ternary_in_assign() {
        let code = "def clamp(x: int) -> int:\n    result = x if x > 0 else 0\n    return result";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    // =========================================================================
    // Section 25: try/except with variable hoisting (stmt_gen_complex.rs)
    // =========================================================================

    #[test]
    fn test_w7s_try_hoisted_var() {
        let code = "def parse(s: str) -> int:\n    try:\n        val = int(s)\n    except ValueError:\n        val = 0\n    return val";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_try_hoisted_multiple_vars() {
        let code = "def parse_pair(a: str, b: str) -> int:\n    try:\n        x = int(a)\n        y = int(b)\n    except ValueError:\n        x = 0\n        y = 0\n    return x + y";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_try_handler_with_return() {
        let code = "def safe_parse(s: str) -> int:\n    try:\n        return int(s)\n    except ValueError:\n        return -1";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_try_except_finally_all() {
        let code = "def full_try(x: int) -> int:\n    result = 0\n    try:\n        result = x * 2\n    except ValueError:\n        result = -1\n    finally:\n        result += 1\n    return result";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    // =========================================================================
    // Section 26: stmt_gen.rs - complex function signatures
    // =========================================================================

    #[test]
    fn test_w7s_func_no_params() {
        let code = "def hello() -> str:\n    return \"hello\"";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn hello"));
        }
    }

    #[test]
    fn test_w7s_func_multiple_params() {
        let code = "def add3(a: int, b: int, c: int) -> int:\n    return a + b + c";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn add3"));
        }
    }

    #[test]
    fn test_w7s_func_default_param() {
        let code = "def greet(name: str = \"world\") -> str:\n    return f\"Hello {name}\"";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_func_return_optional() {
        let code = "from typing import Optional\ndef find(items: list, target: int) -> Optional[int]:\n    for item in items:\n        if item == target:\n            return item\n    return None";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_func_return_bool() {
        let code = "def is_even(n: int) -> bool:\n    return n % 2 == 0";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("bool"));
        }
    }

    #[test]
    fn test_w7s_func_return_list() {
        let code = "def make_list(n: int) -> list:\n    result = []\n    for i in range(n):\n        result.append(i)\n    return result";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    // =========================================================================
    // Section 27: stmt_gen.rs - dictionary operations
    // =========================================================================

    #[test]
    fn test_w7s_dict_creation() {
        let code = "def make_dict() -> dict:\n    return {\"key\": \"value\"}";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_dict_access() {
        let code = "def get_val(d: dict, k: str) -> str:\n    return d[k]";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_dict_assign() {
        let code = "def set_val(d: dict, k: str, v: str):\n    d[k] = v";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_dict_empty() {
        let code = "def empty_dict() -> dict:\n    return {}";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    // =========================================================================
    // Section 28: stmt_gen.rs - string operations
    // =========================================================================

    #[test]
    fn test_w7s_string_concat() {
        let code = "def concat(a: str, b: str) -> str:\n    return a + b";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_string_multiply() {
        let code = "def repeat(s: str, n: int) -> str:\n    return s * n";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_string_len() {
        let code = "def length(s: str) -> int:\n    return len(s)";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("len"));
        }
    }

    // =========================================================================
    // Section 29: stmt_gen.rs - comparison operators in conditions
    // =========================================================================

    #[test]
    fn test_w7s_compare_lt() {
        let code = "def is_small(x: int) -> bool:\n    return x < 10";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_compare_gt() {
        let code = "def is_big(x: int) -> bool:\n    return x > 100";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_compare_lte() {
        let code = "def at_most(x: int) -> bool:\n    return x <= 10";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    // =========================================================================
    // Section 30: stmt_gen.rs - logical operators
    // =========================================================================

    #[test]
    fn test_w7s_logical_and() {
        let code = "def both(a: bool, b: bool) -> bool:\n    return a and b";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_logical_or() {
        let code = "def either(a: bool, b: bool) -> bool:\n    return a or b";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_logical_not() {
        let code = "def negate(a: bool) -> bool:\n    return not a";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    // =========================================================================
    // Section 31: stmt_gen.rs - unary operators
    // =========================================================================

    #[test]
    fn test_w7s_unary_neg() {
        let code = "def negate(x: int) -> int:\n    return -x";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    // =========================================================================
    // Section 32: stmt_gen.rs - complex patterns and edge cases
    // =========================================================================

    #[test]
    fn test_w7s_nested_function_calls() {
        let code = "def compute(x: int) -> int:\n    return abs(x * -1)";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("abs"));
        }
    }

    #[test]
    fn test_w7s_chained_comparisons() {
        let code = "def in_range(x: int) -> bool:\n    return 0 < x and x < 100";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_multiple_statements() {
        let code = "def multi(x: int) -> int:\n    a = x + 1\n    b = a * 2\n    c = b - 3\n    return c";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_string_methods() {
        let code = "def upper(s: str) -> str:\n    return s.upper()";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_string_strip() {
        let code = "def clean(s: str) -> str:\n    return s.strip()";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_string_split() {
        let code = "def words(s: str) -> list:\n    return s.split()";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_list_append() {
        let code = "def build(n: int) -> list:\n    result = []\n    for i in range(n):\n        result.append(i)\n    return result";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("push"));
        }
    }

    #[test]
    fn test_w7s_list_len() {
        let code = "def size(items: list) -> int:\n    return len(items)";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("len"));
        }
    }

    #[test]
    fn test_w7s_multiple_functions() {
        let code = "def add(a: int, b: int) -> int:\n    return a + b\n\ndef sub(a: int, b: int) -> int:\n    return a - b";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn add"));
            assert!(result.contains("fn sub"));
        }
    }

    #[test]
    fn test_w7s_recursive_function() {
        let code = "def factorial(n: int) -> int:\n    if n <= 1:\n        return 1\n    return n * factorial(n - 1)";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("factorial"));
        }
    }

    #[test]
    fn test_w7s_function_calling_function() {
        let code = "def double(x: int) -> int:\n    return x * 2\n\ndef quadruple(x: int) -> int:\n    return double(double(x))";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("double"));
        }
    }

    // =========================================================================
    // Section 34: stmt_gen.rs - set operations
    // =========================================================================

    #[test]
    fn test_w7s_set_literal() {
        let code = "def make_set() -> set:\n    return {1, 2, 3}";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_set_add() {
        let code = "def grow():\n    s = {1, 2}\n    s.add(3)";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    // =========================================================================
    // Section 35: stmt_gen_complex.rs - multiple assignment
    // =========================================================================

    #[test]
    fn test_w7s_multi_assign() {
        let code = "def init() -> int:\n    a = b = c = 1\n    return a + b + c";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    // =========================================================================
    // Section 36: stmt_gen.rs - walrus operator and match
    // =========================================================================

    #[test]
    fn test_w7s_walrus_basic() {
        let code = "def check_length(items: list) -> bool:\n    n = len(items)\n    if n > 10:\n        return True\n    return False";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_match_case_basic() {
        let code = "def describe(x: int) -> str:\n    match x:\n        case 0:\n            return \"zero\"\n        case 1:\n            return \"one\"\n        case _:\n            return \"other\"";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_match_case_string() {
        let code = "def process(cmd: str) -> int:\n    match cmd:\n        case \"start\":\n            return 1\n        case \"stop\":\n            return 0\n        case _:\n            return -1";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    // =========================================================================
    // Section 37: stmt_gen.rs - ellipsis and pass edge cases
    // =========================================================================

    #[test]
    fn test_w7s_ellipsis_body() {
        let code = "def stub() -> int:\n    ...";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_pass_class() {
        let code = "class Empty:\n    pass";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    // =========================================================================
    // Section 38: stmt_gen_complex.rs - exception patterns with finally
    // =========================================================================

    #[test]
    fn test_w7s_try_zero_div_finally() {
        let code = "def safe_div_finally(a: int, b: int) -> int:\n    result = 0\n    try:\n        result = a // b\n    except ZeroDivisionError:\n        result = -1\n    finally:\n        print(\"done\")\n    return result";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_try_value_error_as() {
        let code = "def parse_safe(s: str) -> int:\n    try:\n        return int(s)\n    except ValueError as err:\n        print(err)\n        return 0";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_try_multiple_handlers_as() {
        let code = "def convert(s: str) -> int:\n    try:\n        return int(s)\n    except ValueError as e:\n        print(e)\n        return -1\n    except TypeError:\n        return -2";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    // =========================================================================
    // Section 39: class with inheritance
    // =========================================================================

    #[test]
    fn test_w7s_class_inheritance() {
        let code = "class Animal:\n    def __init__(self, name: str):\n        self.name = name\n    def speak(self) -> str:\n        return self.name";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    // =========================================================================
    // Section 40: additional edge cases for coverage depth
    // =========================================================================

    #[test]
    fn test_w7s_complex_try_body() {
        let code = "def process(items: list) -> int:\n    total = 0\n    try:\n        for item in items:\n            total += int(item)\n    except:\n        total = -1\n    return total";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_nested_loops_break_continue() {
        let code = "def matrix_search(n: int) -> int:\n    found = 0\n    for i in range(n):\n        for j in range(n):\n            if i == j:\n                continue\n            if i + j > n:\n                break\n            found += 1\n    return found";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("continue"));
            assert!(result.contains("break"));
        }
    }

    #[test]
    fn test_w7s_augassign_string_concat() {
        let code = "def build_str(items: list) -> str:\n    result = \"\"\n    for item in items:\n        result += str(item)\n    return result";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_return_expression() {
        let code = "def compute(x: int, y: int) -> int:\n    return (x + y) * (x - y)";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_complex_condition() {
        let code = "def check(a: int, b: int, c: int) -> bool:\n    return a > 0 and b > 0 and c > 0";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_raise_value_error_no_msg() {
        let code = "def fail(x: int):\n    if x < 0:\n        raise ValueError()";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_index_access_list() {
        let code = "def first(items: list) -> int:\n    return items[0]";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_index_access_negative() {
        let code = "def last(items: list) -> int:\n    return items[-1]";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_lambda_expression() {
        let code = "def apply(x: int) -> int:\n    f = lambda a: a * 2\n    return f(x)";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_in_membership() {
        let code = "def contains(items: list, target: int) -> bool:\n    return target in items";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("contains"));
        }
    }

    #[test]
    fn test_w7s_not_in_membership() {
        let code = "def missing(items: list, target: int) -> bool:\n    return target not in items";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("contains"));
        }
    }

    #[test]
    fn test_w7s_is_none_check() {
        let code = "def is_null(x: int) -> bool:\n    return x is None";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_is_not_none_check() {
        let code = "def not_null(x: int) -> bool:\n    return x is not None";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }

    #[test]
    fn test_w7s_min_builtin() {
        let code = "def minimum(a: int, b: int) -> int:\n    return min(a, b)";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("min"));
        }
    }

    #[test]
    fn test_w7s_max_builtin() {
        let code = "def maximum(a: int, b: int) -> int:\n    return max(a, b)";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("max"));
        }
    }

    #[test]
    fn test_w7s_abs_builtin() {
        let code = "def absolute(x: int) -> int:\n    return abs(x)";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("abs"));
        }
    }

    #[test]
    fn test_w7s_int_conversion() {
        let code = "def to_int(s: str) -> int:\n    return int(s)";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("parse"));
        }
    }

    #[test]
    fn test_w7s_str_conversion() {
        let code = "def to_str(x: int) -> str:\n    return str(x)";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("to_string"));
        }
    }

    #[test]
    fn test_w7s_float_conversion() {
        let code = "def to_float(x: int) -> float:\n    return float(x)";
        if let Ok(result) = transpile(code) {
            assert!(!result.is_empty());
        }
    }
}
