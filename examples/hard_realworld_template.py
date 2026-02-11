"""Real-world template engine for string interpolation.

Mimics: Jinja2, string.Template, Mustache-style templating.
Replaces {{variable}} placeholders with values from a lookup table.
"""


def tpl_find_open(tpl: str, start: int) -> int:
    """Find next {{ in template from start position. Returns -1 if not found."""
    idx: int = start
    while idx < len(tpl) - 1:
        if tpl[idx] == "{" and tpl[idx + 1] == "{":
            return idx
        idx = idx + 1
    return -1


def tpl_find_close(tpl: str, start: int) -> int:
    """Find next }} in template from start position. Returns -1 if not found."""
    idx: int = start
    while idx < len(tpl) - 1:
        if tpl[idx] == "}" and tpl[idx + 1] == "}":
            return idx
        idx = idx + 1
    return -1


def tpl_extract(text: str, start: int, end: int) -> str:
    """Extract substring [start, end)."""
    result: str = ""
    idx: int = start
    while idx < end:
        result = result + text[idx]
        idx = idx + 1
    return result


def tpl_trim(raw_name: str) -> str:
    """Trim whitespace from variable name."""
    start: int = 0
    while start < len(raw_name) and raw_name[start] == " ":
        start = start + 1
    end: int = len(raw_name)
    while end > start and raw_name[end - 1] == " ":
        end = end - 1
    return tpl_extract(raw_name, start, end)


def tpl_lookup(var_names: list[str], var_values: list[str], name: str) -> str:
    """Look up variable value by name. Returns empty string if not found."""
    idx: int = 0
    while idx < len(var_names):
        if var_names[idx] == name:
            return var_values[idx]
        idx = idx + 1
    return ""


def tpl_render(tpl: str, var_names: list[str], var_values: list[str]) -> str:
    """Render template by replacing {{var}} with values."""
    result: str = ""
    pos: int = 0
    while pos < len(tpl):
        open_pos: int = tpl_find_open(tpl, pos)
        if open_pos == -1:
            rest: str = tpl_extract(tpl, pos, len(tpl))
            result = result + rest
            pos = len(tpl)
        else:
            before: str = tpl_extract(tpl, pos, open_pos)
            result = result + before
            close_pos: int = tpl_find_close(tpl, open_pos + 2)
            if close_pos == -1:
                remaining: str = tpl_extract(tpl, open_pos, len(tpl))
                result = result + remaining
                pos = len(tpl)
            else:
                raw_name: str = tpl_extract(tpl, open_pos + 2, close_pos)
                var_name: str = tpl_trim(raw_name)
                replacement: str = tpl_lookup(var_names, var_values, var_name)
                result = result + replacement
                pos = close_pos + 2
    return result


def tpl_count_placeholders(tpl: str) -> int:
    """Count number of {{...}} placeholders."""
    count: int = 0
    pos: int = 0
    while pos < len(tpl):
        open_pos: int = tpl_find_open(tpl, pos)
        if open_pos == -1:
            pos = len(tpl)
        else:
            close_pos: int = tpl_find_close(tpl, open_pos + 2)
            if close_pos == -1:
                pos = len(tpl)
            else:
                count = count + 1
                pos = close_pos + 2
    return count


def tpl_extract_names(tpl: str) -> list[str]:
    """Extract all variable names from template placeholders."""
    names: list[str] = []
    pos: int = 0
    while pos < len(tpl):
        open_pos: int = tpl_find_open(tpl, pos)
        if open_pos == -1:
            pos = len(tpl)
        else:
            close_pos: int = tpl_find_close(tpl, open_pos + 2)
            if close_pos == -1:
                pos = len(tpl)
            else:
                raw_name: str = tpl_extract(tpl, open_pos + 2, close_pos)
                trimmed: str = tpl_trim(raw_name)
                names.append(trimmed)
                pos = close_pos + 2
    return names


def tpl_has_placeholders(tpl: str) -> int:
    """Check if template has any placeholders. Returns 1 if yes."""
    if tpl_find_open(tpl, 0) != -1:
        return 1
    return 0


def test_module() -> int:
    """Test template engine module."""
    passed: int = 0

    # Test 1: find open bracket
    if tpl_find_open("hello {{world}}", 0) == 6:
        passed = passed + 1

    # Test 2: find close bracket
    if tpl_find_close("hello {{world}}", 8) == 13:
        passed = passed + 1

    # Test 3: basic render
    tpl: str = "Hello, {{name}}!"
    result: str = tpl_render(tpl, ["name"], ["Alice"])
    if result == "Hello, Alice!":
        passed = passed + 1

    # Test 4: multiple variables
    tpl2: str = "{{greeting}}, {{ name }}! You are {{ age }}."
    names: list[str] = ["greeting", "name", "age"]
    vals: list[str] = ["Hi", "Bob", "30"]
    result2: str = tpl_render(tpl2, names, vals)
    if result2 == "Hi, Bob! You are 30.":
        passed = passed + 1

    # Test 5: count placeholders
    if tpl_count_placeholders(tpl2) == 3:
        passed = passed + 1

    # Test 6: extract var names
    extracted: list[str] = tpl_extract_names(tpl2)
    if len(extracted) == 3 and extracted[0] == "greeting":
        passed = passed + 1

    # Test 7: missing variable renders empty
    result3: str = tpl_render("{{missing}}", [], [])
    if result3 == "":
        passed = passed + 1

    # Test 8: no placeholders passthrough
    result4: str = tpl_render("plain text", [], [])
    if result4 == "plain text":
        passed = passed + 1

    return passed
