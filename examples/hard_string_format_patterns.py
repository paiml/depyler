"""Pathological string formatting and manipulation patterns for transpiler stress testing.

Tests f-strings with expressions, format specs, method calls inside f-strings,
multi-line string building, and comprehensive string method usage.
"""

from typing import List, Dict, Tuple, Optional


class TextBuffer:
    """Mutable text buffer with builder pattern for multi-line construction."""

    def __init__(self, initial: str = ""):
        self._lines: List[str] = []
        if initial:
            self._lines.append(initial)
        self._indent_level: int = 0

    def append(self, text: str) -> "TextBuffer":
        prefix = "    " * self._indent_level
        self._lines.append(prefix + text)
        return self

    def indent(self) -> "TextBuffer":
        self._indent_level += 1
        return self

    def dedent(self) -> "TextBuffer":
        if self._indent_level > 0:
            self._indent_level -= 1
        return self

    def blank_line(self) -> "TextBuffer":
        self._lines.append("")
        return self

    def build(self) -> str:
        return "\n".join(self._lines)

    def line_count(self) -> int:
        return len(self._lines)

    def __str__(self) -> str:
        return self.build()

    def __repr__(self) -> str:
        return f"TextBuffer(lines={self.line_count()})"


# --- f-string with expressions ---

def format_arithmetic(a: int, b: int) -> str:
    """f-string with arithmetic expressions."""
    return f"{a} + {b} = {a + b}, {a} * {b} = {a * b}, {a} ** 2 = {a ** 2}"


def format_float_precision(value: float) -> str:
    """f-string with float format specs."""
    return f"raw={value}, two_dec={value:.2f}, sci={value:.3e}, pct={value:.1%}"


def format_integer_padding(n: int) -> str:
    """f-string with integer padding and alignment."""
    return f"dec={n:05d}, hex={n:#06x}, bin={n:#010b}, oct={n:#06o}"


def format_alignment(text: str, width: int) -> str:
    """f-string with alignment operators."""
    left = f"{text:<{width}}"
    right = f"{text:>{width}}"
    center = f"{text:^{width}}"
    fill = f"{text:*^{width}}"
    return f"L=[{left}] R=[{right}] C=[{center}] F=[{fill}]"


def format_conditional(x: int) -> str:
    """f-string with ternary expression."""
    return f"x={x} is {'even' if x % 2 == 0 else 'odd'} and {'positive' if x > 0 else 'non-positive'}"


def format_nested_method(s: str) -> str:
    """f-string with method calls on the interpolated value."""
    return f"upper={s.upper()}, lower={s.lower()}, title={s.title()}, len={len(s)}"


def format_dict_values(d: Dict[str, int]) -> str:
    """Format dictionary key-value pairs into a string."""
    parts = []
    for key in sorted(d.keys()):
        parts.append(f"{key}={d[key]}")
    return ", ".join(parts)


def format_table_row(name: str, value: float, unit: str) -> str:
    """Format a table row with fixed-width columns."""
    return f"| {name:<15} | {value:>10.2f} | {unit:<5} |"


# --- String method patterns ---

def split_and_rejoin(text: str, delimiter: str, new_delimiter: str) -> str:
    """Split by one delimiter and rejoin with another."""
    parts = text.split(delimiter)
    cleaned = []
    for part in parts:
        stripped = part.strip()
        if stripped:
            cleaned.append(stripped)
    return new_delimiter.join(cleaned)


def count_words(text: str) -> Dict[str, int]:
    """Count word frequencies in text."""
    counts: Dict[str, int] = {}
    words = text.lower().split()
    for word in words:
        cleaned = ""
        for ch in word:
            if ch.isalpha():
                cleaned += ch
        if cleaned:
            if cleaned in counts:
                counts[cleaned] += 1
            else:
                counts[cleaned] = 1
    return counts


def caesar_cipher(text: str, shift: int) -> str:
    """Apply Caesar cipher to text."""
    result = []
    for ch in text:
        if ch.isalpha():
            base = ord('a') if ch.islower() else ord('A')
            shifted = (ord(ch) - base + shift) % 26 + base
            result.append(chr(shifted))
        else:
            result.append(ch)
    return "".join(result)


def is_palindrome(s: str) -> bool:
    """Check if string is a palindrome (ignoring case and spaces)."""
    cleaned = ""
    for ch in s.lower():
        if ch.isalnum():
            cleaned += ch
    left = 0
    right = len(cleaned) - 1
    while left < right:
        if cleaned[left] != cleaned[right]:
            return False
        left += 1
        right -= 1
    return True


def longest_common_prefix(strings: List[str]) -> str:
    """Find the longest common prefix among a list of strings."""
    if not strings:
        return ""
    prefix = strings[0]
    for s in strings[1:]:
        while not s.startswith(prefix):
            prefix = prefix[:-1]
            if not prefix:
                return ""
    return prefix


def run_length_encode(s: str) -> str:
    """Run-length encode a string."""
    if not s:
        return ""
    result = []
    current = s[0]
    count = 1
    for i in range(1, len(s)):
        if s[i] == current:
            count += 1
        else:
            if count > 1:
                result.append(f"{current}{count}")
            else:
                result.append(current)
            current = s[i]
            count = 1
    if count > 1:
        result.append(f"{current}{count}")
    else:
        result.append(current)
    return "".join(result)


def run_length_decode(encoded: str) -> str:
    """Decode a run-length encoded string."""
    result = []
    i = 0
    while i < len(encoded):
        ch = encoded[i]
        i += 1
        num_str = ""
        while i < len(encoded) and encoded[i].isdigit():
            num_str += encoded[i]
            i += 1
        if num_str:
            count = int(num_str)
        else:
            count = 1
        result.append(ch * count)
    return "".join(result)


# --- Untyped functions (>30%) ---

def reverse_words(text):
    """Reverse the order of words in text - untyped."""
    words = text.split()
    reversed_words = []
    for i in range(len(words) - 1, -1, -1):
        reversed_words.append(words[i])
    return " ".join(reversed_words)


def camel_to_snake(name):
    """Convert CamelCase to snake_case - untyped."""
    result = []
    for i, ch in enumerate(name):
        if ch.isupper() and i > 0:
            result.append("_")
        result.append(ch.lower())
    return "".join(result)


def snake_to_camel(name):
    """Convert snake_case to CamelCase - untyped."""
    parts = name.split("_")
    result = []
    for part in parts:
        if part:
            result.append(part[0].upper() + part[1:])
    return "".join(result)


def truncate(text, max_len, suffix="..."):
    """Truncate text with suffix if too long - untyped."""
    if len(text) <= max_len:
        return text
    return text[:max_len - len(suffix)] + suffix


def wrap_text(text, width):
    """Simple word-wrap at given width - untyped."""
    words = text.split()
    lines = []
    current_line = []
    current_len = 0
    for word in words:
        if current_len + len(word) + (1 if current_line else 0) > width:
            if current_line:
                lines.append(" ".join(current_line))
                current_line = []
                current_len = 0
        current_line.append(word)
        current_len += len(word) + (1 if len(current_line) > 1 else 0)
    if current_line:
        lines.append(" ".join(current_line))
    return "\n".join(lines)


def extract_numbers(text):
    """Extract all integers from a string - untyped."""
    numbers = []
    current = ""
    for ch in text:
        if ch.isdigit() or (ch == '-' and not current):
            current += ch
        elif current:
            numbers.append(int(current))
            current = ""
    if current:
        numbers.append(int(current))
    return numbers


def simple_glob_match(pattern, text):
    """Simple glob matching supporting * and ? - untyped."""
    pi = 0
    ti = 0
    star_pi = -1
    star_ti = -1

    while ti < len(text):
        if pi < len(pattern) and (pattern[pi] == text[ti] or pattern[pi] == '?'):
            pi += 1
            ti += 1
        elif pi < len(pattern) and pattern[pi] == '*':
            star_pi = pi
            star_ti = ti
            pi += 1
        elif star_pi != -1:
            pi = star_pi + 1
            star_ti += 1
            ti = star_ti
        else:
            return False

    while pi < len(pattern) and pattern[pi] == '*':
        pi += 1

    return pi == len(pattern)


def levenshtein_distance(s1, s2):
    """Compute Levenshtein edit distance between two strings - untyped."""
    m = len(s1)
    n = len(s2)
    dp = []
    for i in range(m + 1):
        row = []
        for j in range(n + 1):
            if i == 0:
                row.append(j)
            elif j == 0:
                row.append(i)
            else:
                row.append(0)
        dp.append(row)

    for i in range(1, m + 1):
        for j in range(1, n + 1):
            if s1[i-1] == s2[j-1]:
                dp[i][j] = dp[i-1][j-1]
            else:
                dp[i][j] = 1 + min(dp[i-1][j], dp[i][j-1], dp[i-1][j-1])

    return dp[m][n]


def build_code_block(func_name, params, body_lines, return_type="void"):
    """Build a formatted code block string - untyped."""
    buf = TextBuffer()
    param_str = ", ".join(params)
    buf.append(f"fn {func_name}({param_str}) -> {return_type} {{")
    buf.indent()
    for line in body_lines:
        buf.append(line)
    buf.dedent()
    buf.append("}")
    return buf.build()


# --- Typed test functions ---

def test_fstring_arithmetic():
    """Test f-strings with arithmetic expressions."""
    result = format_arithmetic(3, 4)
    assert "3 + 4 = 7" in result
    assert "3 * 4 = 12" in result
    assert "3 ** 2 = 9" in result
    return True


def test_fstring_float_precision():
    """Test f-strings with float format specs."""
    result = format_float_precision(3.14159)
    assert "two_dec=3.14" in result
    assert "sci=3.142e+00" in result
    return True


def test_fstring_integer_padding():
    """Test f-strings with integer padding."""
    result = format_integer_padding(42)
    assert "dec=00042" in result
    assert "hex=0x002a" in result
    return True


def test_fstring_alignment():
    """Test f-strings with alignment."""
    result = format_alignment("hi", 10)
    assert "L=[hi        ]" in result
    assert "R=[        hi]" in result
    return True


def test_fstring_conditional():
    """Test f-strings with ternary expressions."""
    assert "even" in format_conditional(4)
    assert "odd" in format_conditional(3)
    assert "positive" in format_conditional(5)
    assert "non-positive" in format_conditional(-1)
    return True


def test_fstring_methods():
    """Test f-strings with method calls."""
    result = format_nested_method("hello World")
    assert "upper=HELLO WORLD" in result
    assert "lower=hello world" in result
    assert "title=Hello World" in result
    assert "len=11" in result
    return True


def test_string_methods():
    """Test various string manipulation functions."""
    assert split_and_rejoin("a, b, c, d", ",", " | ") == "a | b | c | d"

    words = count_words("the cat sat on the mat the cat")
    assert words["the"] == 3
    assert words["cat"] == 2
    assert words["mat"] == 1

    assert caesar_cipher("abc", 3) == "def"
    assert caesar_cipher("xyz", 3) == "abc"
    assert caesar_cipher("Hello", 13) == "Uryyb"

    assert is_palindrome("racecar")
    assert is_palindrome("A man a plan a canal Panama")
    assert not is_palindrome("hello")
    return True


def test_string_algorithms():
    """Test string algorithm functions."""
    assert longest_common_prefix(["flower", "flow", "flight"]) == "fl"
    assert longest_common_prefix(["dog", "car", "race"]) == ""
    assert longest_common_prefix(["abc"]) == "abc"

    encoded = run_length_encode("aaabbbccddddde")
    assert encoded == "a3b3c2d5e"
    decoded = run_length_decode(encoded)
    assert decoded == "aaabbbccddddde"

    assert levenshtein_distance("kitten", "sitting") == 3
    assert levenshtein_distance("", "abc") == 3
    assert levenshtein_distance("abc", "abc") == 0
    return True


def test_untyped_helpers() -> bool:
    """Test untyped helper functions."""
    assert reverse_words("hello world foo") == "foo world hello"

    assert camel_to_snake("CamelCase") == "camel_case"
    assert camel_to_snake("MyHTTPHandler") == "my_h_t_t_p_handler"
    assert snake_to_camel("snake_case") == "SnakeCase"

    assert truncate("hello world", 8) == "hello..."
    assert truncate("short", 10) == "short"

    wrapped = wrap_text("the quick brown fox jumps over the lazy dog", 15)
    lines = wrapped.split("\n")
    for line in lines:
        assert len(line) <= 15

    nums = extract_numbers("abc 42 def -7 ghi 100")
    assert nums == [42, -7, 100]
    return True


def test_glob_matching() -> bool:
    """Test simple glob pattern matching."""
    assert simple_glob_match("*.py", "test.py")
    assert simple_glob_match("test_?.py", "test_a.py")
    assert not simple_glob_match("test_?.py", "test_ab.py")
    assert simple_glob_match("**", "anything")
    assert simple_glob_match("a*b*c", "aXXbYYc")
    assert not simple_glob_match("a*b*c", "aXXbYY")
    return True


def test_text_buffer() -> bool:
    """Test TextBuffer builder pattern."""
    code = build_code_block(
        "add", ["a: i32", "b: i32"],
        ["let result = a + b;", "result"],
        "i32"
    )
    assert "fn add(a: i32, b: i32) -> i32 {" in code
    assert "    let result = a + b;" in code
    assert "}" in code

    buf = TextBuffer("header")
    buf.append("line1").indent().append("indented").dedent().append("line3")
    text = buf.build()
    assert "header" in text
    assert "    indented" in text
    assert buf.line_count() == 4
    return True


def test_format_table() -> bool:
    """Test table row formatting."""
    row = format_table_row("Temperature", 98.6, "F")
    assert "Temperature" in row
    assert "98.60" in row
    assert "F" in row
    return True


def test_all() -> bool:
    """Run all tests."""
    assert test_fstring_arithmetic()
    assert test_fstring_float_precision()
    assert test_fstring_integer_padding()
    assert test_fstring_alignment()
    assert test_fstring_conditional()
    assert test_fstring_methods()
    assert test_string_methods()
    assert test_string_algorithms()
    assert test_untyped_helpers()
    assert test_glob_matching()
    assert test_text_buffer()
    assert test_format_table()
    return True


def main():
    """Entry point."""
    if test_all():
        print("hard_string_format_patterns: ALL TESTS PASSED")
    else:
        print("hard_string_format_patterns: TESTS FAILED")


if __name__ == "__main__":
    main()
