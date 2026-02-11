"""Real-world code formatter with indent tracking.

Mimics: black, autopep8, prettier - code formatting tools.
Tracks bracket nesting depth, adjusts indentation, normalizes whitespace.
"""


def count_leading_spaces(line: str) -> int:
    """Count leading spaces in a line."""
    count: int = 0
    while count < len(line) and line[count] == " ":
        count = count + 1
    return count


def make_indent(level: int, indent_size: int) -> str:
    """Create indentation string for given level."""
    result: str = ""
    total: int = level * indent_size
    idx: int = 0
    while idx < total:
        result = result + " "
        idx = idx + 1
    return result


def trim_leading(text: str) -> str:
    """Remove leading whitespace from text."""
    idx: int = 0
    while idx < len(text) and (text[idx] == " " or text[idx] == "\t"):
        idx = idx + 1
    result: str = ""
    while idx < len(text):
        result = result + text[idx]
        idx = idx + 1
    return result


def trim_trailing(text: str) -> str:
    """Remove trailing whitespace from text."""
    end: int = len(text)
    while end > 0 and (text[end - 1] == " " or text[end - 1] == "\t"):
        end = end - 1
    result: str = ""
    idx: int = 0
    while idx < end:
        result = result + text[idx]
        idx = idx + 1
    return result


def is_open_bracket(ch: str) -> bool:
    """Check if character is an opening bracket."""
    return ch == "(" or ch == "[" or ch == "{"


def is_close_bracket(ch: str) -> bool:
    """Check if character is a closing bracket."""
    return ch == ")" or ch == "]" or ch == "}"


def net_bracket_change(line: str) -> int:
    """Calculate net bracket depth change for a line."""
    depth: int = 0
    idx: int = 0
    while idx < len(line):
        if is_open_bracket(line[idx]):
            depth = depth + 1
        elif is_close_bracket(line[idx]):
            depth = depth - 1
        idx = idx + 1
    return depth


def starts_with_close(line: str) -> bool:
    """Check if trimmed line starts with a closing bracket."""
    trimmed: str = trim_leading(line)
    if len(trimmed) == 0:
        return False
    return is_close_bracket(trimmed[0])


def format_lines(text: str, indent_size: int) -> str:
    """Format code text with proper indentation based on bracket nesting."""
    lines: list[str] = []
    current: str = ""
    idx: int = 0
    while idx < len(text):
        if text[idx] == "\n":
            lines.append(current)
            current = ""
        else:
            current = current + text[idx]
        idx = idx + 1
    if len(current) > 0:
        lines.append(current)

    result: str = ""
    depth: int = 0
    li: int = 0
    while li < len(lines):
        stripped: str = trim_leading(lines[li])
        stripped = trim_trailing(stripped)
        if len(stripped) == 0:
            result = result + "\n"
        else:
            # Decrease indent before closing bracket
            if starts_with_close(stripped) and depth > 0:
                depth = depth - 1
            indented: str = make_indent(depth, indent_size) + stripped
            result = result + indented + "\n"
            # Adjust depth for next line
            change: int = net_bracket_change(stripped)
            if starts_with_close(stripped):
                # We already decremented, add back the close bracket count
                depth = depth + change + 1
            else:
                depth = depth + change
            if depth < 0:
                depth = 0
        li = li + 1
    return result


def normalize_whitespace(text: str) -> str:
    """Collapse multiple spaces into single space within a line."""
    result: str = ""
    prev_space: bool = False
    idx: int = 0
    while idx < len(text):
        ch: str = text[idx]
        if ch == " ":
            if not prev_space:
                result = result + ch
            prev_space = True
        else:
            result = result + ch
            prev_space = False
        idx = idx + 1
    return result


def count_lines(text: str) -> int:
    """Count number of lines in text."""
    if len(text) == 0:
        return 0
    count: int = 1
    idx: int = 0
    while idx < len(text):
        if text[idx] == "\n":
            count = count + 1
        idx = idx + 1
    return count


def test_module() -> int:
    """Test code formatter module."""
    passed: int = 0

    # Test 1: count leading spaces
    if count_leading_spaces("    hello") == 4:
        passed = passed + 1

    # Test 2: make indent
    if make_indent(2, 4) == "        ":
        passed = passed + 1

    # Test 3: trim leading
    if trim_leading("  hello") == "hello":
        passed = passed + 1

    # Test 4: trim trailing
    if trim_trailing("hello   ") == "hello":
        passed = passed + 1

    # Test 5: net bracket change
    if net_bracket_change("fn(a, {b") == 2:
        passed = passed + 1

    # Test 6: starts with close
    if starts_with_close("  }") and not starts_with_close("  a"):
        passed = passed + 1

    # Test 7: normalize whitespace
    if normalize_whitespace("a   b  c") == "a b c":
        passed = passed + 1

    # Test 8: format with indentation
    code: str = "fn main() {\nprint();\n}"
    formatted: str = format_lines(code, 4)
    # Check that "print" line is indented
    flines: list[str] = []
    cur: str = ""
    fi: int = 0
    while fi < len(formatted):
        if formatted[fi] == "\n":
            flines.append(cur)
            cur = ""
        else:
            cur = cur + formatted[fi]
        fi = fi + 1
    if len(cur) > 0:
        flines.append(cur)
    if len(flines) >= 2 and count_leading_spaces(flines[1]) == 4:
        passed = passed + 1

    return passed
