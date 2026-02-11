"""Real-world INI-style config parsing without imports.

Mimics: configparser, TOML parsing, .env file loaders.
Handles sections, key=value pairs, comments, blank lines.
"""


def cfg_is_blank(line: str) -> int:
    """Check if a line is blank or only whitespace. Returns 1 if blank."""
    idx: int = 0
    while idx < len(line):
        ch: str = line[idx]
        if ch != " " and ch != "\t":
            return 0
        idx = idx + 1
    return 1


def cfg_is_comment(line: str) -> int:
    """Check if line is a comment (starts with # or ;). Returns 1 if comment."""
    idx: int = 0
    while idx < len(line) and (line[idx] == " " or line[idx] == "\t"):
        idx = idx + 1
    if idx >= len(line):
        return 0
    if line[idx] == "#" or line[idx] == ";":
        return 1
    return 0


def cfg_trim(text: str) -> str:
    """Remove leading and trailing whitespace."""
    start: int = 0
    while start < len(text) and (text[start] == " " or text[start] == "\t"):
        start = start + 1
    end: int = len(text)
    while end > start and (text[end - 1] == " " or text[end - 1] == "\t"):
        end = end - 1
    result: str = ""
    idx: int = start
    while idx < end:
        result = result + text[idx]
        idx = idx + 1
    return result


def cfg_is_section(line: str) -> int:
    """Check if line is a [section] header. Returns 1 if section."""
    trimmed: str = cfg_trim(line)
    if len(trimmed) < 3:
        return 0
    if trimmed[0] == "[" and trimmed[len(trimmed) - 1] == "]":
        return 1
    return 0


def cfg_extract_section(line: str) -> str:
    """Extract section name from [section] header."""
    trimmed: str = cfg_trim(line)
    inner: str = ""
    idx: int = 1
    while idx < len(trimmed) - 1:
        inner = inner + trimmed[idx]
        idx = idx + 1
    return cfg_trim(inner)


def cfg_find_equals(text: str) -> int:
    """Find position of first = in text. Returns -1 if not found."""
    idx: int = 0
    while idx < len(text):
        if text[idx] == "=":
            return idx
        idx = idx + 1
    return -1


def cfg_substring(text: str, start: int, end: int) -> str:
    """Extract substring [start, end)."""
    result: str = ""
    idx: int = start
    while idx < end:
        result = result + text[idx]
        idx = idx + 1
    return result


def cfg_split_lines(text: str) -> list[str]:
    """Split text into lines on newline character."""
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


def cfg_parse(text: str) -> list[list[str]]:
    """Parse config into flat list of [section, name, value] triples."""
    entries: list[list[str]] = []
    lines: list[str] = cfg_split_lines(text)
    current_section: str = "default"
    line_idx: int = 0
    while line_idx < len(lines):
        line: str = lines[line_idx]
        if cfg_is_blank(line) == 1 or cfg_is_comment(line) == 1:
            line_idx = line_idx + 1
            continue
        if cfg_is_section(line) == 1:
            current_section = cfg_extract_section(line)
        else:
            eq_pos: int = cfg_find_equals(line)
            if eq_pos > 0:
                entry_name: str = cfg_trim(cfg_substring(line, 0, eq_pos))
                entry_val: str = cfg_trim(cfg_substring(line, eq_pos + 1, len(line)))
                entries.append([current_section, entry_name, entry_val])
        line_idx = line_idx + 1
    return entries


def cfg_get_value(entries: list[list[str]], section: str, name: str) -> str:
    """Look up a config value by section and name."""
    idx: int = 0
    while idx < len(entries):
        entry: list[str] = entries[idx]
        if entry[0] == section and entry[1] == name:
            return entry[2]
        idx = idx + 1
    return ""


def cfg_count_sections(entries: list[list[str]]) -> int:
    """Count unique sections in parsed config."""
    seen: list[str] = []
    idx: int = 0
    while idx < len(entries):
        section: str = entries[idx][0]
        found: int = 0
        si: int = 0
        while si < len(seen):
            if seen[si] == section:
                found = 1
            si = si + 1
        if found == 0:
            seen.append(section)
        idx = idx + 1
    return len(seen)


def test_module() -> int:
    """Test config parsing module."""
    passed: int = 0

    # Test 1: blank line detection
    if cfg_is_blank("   ") == 1 and cfg_is_blank("hello") == 0:
        passed = passed + 1

    # Test 2: comment detection
    if cfg_is_comment("# comment") == 1 and cfg_is_comment("; comment") == 1:
        passed = passed + 1

    # Test 3: section header detection
    if cfg_is_section("[database]") == 1 and cfg_is_section("not a section") == 0:
        passed = passed + 1

    # Test 4: extract section name
    sn: str = cfg_extract_section("[  server  ]")
    if sn == "server":
        passed = passed + 1

    # Test 5: find equals
    if cfg_find_equals("host = localhost") == 5:
        passed = passed + 1

    # Test 6: full config parse
    config_text: str = "[database]\nhost = localhost\nport = 5432\n\n[server]\nworkers = 4"
    entries: list[list[str]] = cfg_parse(config_text)
    if len(entries) == 3:
        passed = passed + 1

    # Test 7: lookup config value
    host_val: str = cfg_get_value(entries, "database", "host")
    if host_val == "localhost":
        passed = passed + 1

    # Test 8: count sections
    if cfg_count_sections(entries) == 2:
        passed = passed + 1

    return passed
