"""Real-world simple markdown-like text formatter.

Mimics: markdown renderers, rich text formatting, docstring processors.
Converts simple markup to HTML-like output strings.
"""


def md_slice(text: str, start: int, end: int) -> str:
    """Extract substring [start, end)."""
    result: str = ""
    idx: int = start
    while idx < end and idx < len(text):
        result = result + text[idx]
        idx = idx + 1
    return result


def md_count_hashes(line: str) -> int:
    """Count leading # characters in line."""
    count: int = 0
    while count < len(line) and line[count] == "#":
        count = count + 1
    return count


def md_after_prefix(line: str, prefix_len: int) -> str:
    """Get line content after skipping prefix and leading spaces."""
    idx: int = prefix_len
    while idx < len(line) and line[idx] == " ":
        idx = idx + 1
    return md_slice(line, idx, len(line))


def md_level_tag(level: int) -> str:
    """Convert heading level to tag string."""
    if level == 1:
        return "1"
    if level == 2:
        return "2"
    if level == 3:
        return "3"
    if level == 4:
        return "4"
    if level == 5:
        return "5"
    return "6"


def format_heading(line: str) -> str:
    """Convert # heading to <h1>, ## to <h2>, etc."""
    level: int = md_count_hashes(line)
    if level == 0 or level > 6:
        return line
    content: str = md_after_prefix(line, level)
    level_str: str = md_level_tag(level)
    return "<h" + level_str + ">" + content + "</h" + level_str + ">"


def format_bold(text: str) -> str:
    """Replace **bold** with <b>bold</b>."""
    result: str = ""
    idx: int = 0
    while idx < len(text):
        if idx + 1 < len(text) and text[idx] == "*" and text[idx + 1] == "*":
            end: int = idx + 2
            found_close: int = 0
            while end + 1 < len(text) and found_close == 0:
                if text[end] == "*" and text[end + 1] == "*":
                    found_close = 1
                    inner: str = md_slice(text, idx + 2, end)
                    result = result + "<b>" + inner + "</b>"
                    idx = end + 2
                else:
                    end = end + 1
            if found_close == 0:
                result = result + text[idx]
                idx = idx + 1
        else:
            result = result + text[idx]
            idx = idx + 1
    return result


def md_is_list_line(line: str) -> int:
    """Check if line starts with - or * (list item). Returns 1 if yes."""
    idx: int = 0
    while idx < len(line) and line[idx] == " ":
        idx = idx + 1
    if idx < len(line) and (line[idx] == "-" or line[idx] == "*"):
        return 1
    return 0


def format_list_item(line: str) -> str:
    """Convert '- item' to <li>item</li>."""
    stripped_start: int = 0
    while stripped_start < len(line) and line[stripped_start] == " ":
        stripped_start = stripped_start + 1
    if stripped_start < len(line) and (line[stripped_start] == "-" or line[stripped_start] == "*"):
        content: str = md_after_prefix(line, stripped_start + 1)
        return "<li>" + content + "</li>"
    return line


def format_code_inline(text: str) -> str:
    """Replace `code` with <code>code</code>."""
    result: str = ""
    idx: int = 0
    while idx < len(text):
        if text[idx] == "`":
            end: int = idx + 1
            found_close: int = 0
            while end < len(text) and found_close == 0:
                if text[end] == "`":
                    found_close = 1
                    inner: str = md_slice(text, idx + 1, end)
                    result = result + "<code>" + inner + "</code>"
                    idx = end + 1
                else:
                    end = end + 1
            if found_close == 0:
                result = result + text[idx]
                idx = idx + 1
        else:
            result = result + text[idx]
            idx = idx + 1
    return result


def md_split_lines(text: str) -> list[str]:
    """Split text into lines on newline."""
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
    return lines


def render_markdown(text: str) -> str:
    """Render simple markdown text to HTML-like output."""
    lines: list[str] = md_split_lines(text)
    result: str = ""
    idx: int = 0
    while idx < len(lines):
        line: str = lines[idx]
        if len(line) > 0 and line[0] == "#":
            formatted: str = format_heading(line)
            result = result + formatted
        elif md_is_list_line(line) == 1:
            fmt_item: str = format_list_item(line)
            result = result + fmt_item
        else:
            bold_line: str = format_bold(line)
            result = result + bold_line
        if idx < len(lines) - 1:
            result = result + "\n"
        idx = idx + 1
    return result


def test_module() -> int:
    """Test markdown formatter module."""
    passed: int = 0

    # Test 1: heading format
    h1: str = format_heading("# Hello")
    if h1 == "<h1>Hello</h1>":
        passed = passed + 1

    # Test 2: h2 heading
    h2: str = format_heading("## World")
    if h2 == "<h2>World</h2>":
        passed = passed + 1

    # Test 3: bold formatting
    bold_result: str = format_bold("this is **bold** text")
    if bold_result == "this is <b>bold</b> text":
        passed = passed + 1

    # Test 4: list item
    li_result: str = format_list_item("- item one")
    if li_result == "<li>item one</li>":
        passed = passed + 1

    # Test 5: inline code
    code_result: str = format_code_inline("use `print` here")
    if code_result == "use <code>print</code> here":
        passed = passed + 1

    # Test 6: is_list_line
    if md_is_list_line("- test") == 1 and md_is_list_line("not a list") == 0:
        passed = passed + 1

    # Test 7: split lines
    lines: list[str] = md_split_lines("a\nb\nc")
    if len(lines) == 3:
        passed = passed + 1

    # Test 8: full render
    md: str = "# Title\n- item1"
    rendered: str = render_markdown(md)
    has_h1: int = 0
    ri: int = 0
    while ri < len(rendered) - 3:
        if rendered[ri] == "<" and rendered[ri + 1] == "h" and rendered[ri + 2] == "1":
            has_h1 = 1
        ri = ri + 1
    if has_h1 == 1:
        passed = passed + 1

    return passed
