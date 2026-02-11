"""Real-world log line parsing without imports.

Mimics: log file analysis tools, syslog parsers, ELK stack ingest.
Extracts timestamps, severity levels, messages from structured log lines.
"""


def log_find_bracket(text: str, start: int) -> int:
    """Find first ] in text from start. Returns -1 if not found."""
    idx: int = start
    while idx < len(text):
        if text[idx] == "]":
            return idx
        idx = idx + 1
    return -1


def log_find_colon(text: str, start: int) -> int:
    """Find first : in text from start. Returns -1 if not found."""
    idx: int = start
    while idx < len(text):
        if text[idx] == ":":
            return idx
        idx = idx + 1
    return -1


def log_find_space(text: str, start: int) -> int:
    """Find first space in text from start. Returns -1 if not found."""
    idx: int = start
    while idx < len(text):
        if text[idx] == " ":
            return idx
        idx = idx + 1
    return -1


def log_substr(text: str, start: int, end: int) -> str:
    """Extract substring from start to end (exclusive)."""
    result: str = ""
    idx: int = start
    while idx < end and idx < len(text):
        result = result + text[idx]
        idx = idx + 1
    return result


def log_strip(text: str) -> str:
    """Remove leading and trailing spaces."""
    start: int = 0
    while start < len(text) and text[start] == " ":
        start = start + 1
    end: int = len(text)
    while end > start and text[end - 1] == " ":
        end = end - 1
    return log_substr(text, start, end)


def parse_timestamp(line: str) -> str:
    """Extract timestamp from log line. Expects [TIMESTAMP] prefix."""
    if len(line) == 0 or line[0] != "[":
        return ""
    bracket_end: int = log_find_bracket(line, 1)
    if bracket_end == -1:
        return ""
    return log_substr(line, 1, bracket_end)


def parse_level(line: str) -> str:
    """Extract severity level from log line."""
    bracket_end: int = log_find_bracket(line, 0)
    if bracket_end == -1:
        return ""
    rest: str = log_substr(line, bracket_end + 1, len(line))
    rest = log_strip(rest)
    colon_pos: int = log_find_colon(rest, 0)
    space_pos: int = log_find_space(rest, 0)
    if colon_pos != -1:
        return log_substr(rest, 0, colon_pos)
    if space_pos != -1:
        return log_substr(rest, 0, space_pos)
    return rest


def parse_message(line: str) -> str:
    """Extract message body from log line after level."""
    bracket_end: int = log_find_bracket(line, 0)
    if bracket_end == -1:
        return line
    rest: str = log_substr(line, bracket_end + 1, len(line))
    rest = log_strip(rest)
    colon_pos: int = log_find_colon(rest, 0)
    if colon_pos == -1:
        return rest
    msg: str = log_substr(rest, colon_pos + 1, len(rest))
    return log_strip(msg)


def to_upper(text: str) -> str:
    """Convert string to uppercase."""
    result: str = ""
    idx: int = 0
    while idx < len(text):
        ch: str = text[idx]
        code: int = ord(ch)
        if code >= 97 and code <= 122:
            result = result + chr(code - 32)
        else:
            result = result + ch
        idx = idx + 1
    return result


def classify_severity(level_str: str) -> int:
    """Classify log level. 0=unknown,1=debug,2=info,3=warn,4=error,5=fatal."""
    upper: str = to_upper(level_str)
    if upper == "DEBUG":
        return 1
    if upper == "INFO":
        return 2
    if upper == "WARN" or upper == "WARNING":
        return 3
    if upper == "ERROR":
        return 4
    if upper == "FATAL" or upper == "CRITICAL":
        return 5
    return 0


def count_by_severity(lines: list[str]) -> list[int]:
    """Count log lines by severity. Returns [unknown, debug, info, warn, error, fatal]."""
    counts: list[int] = [0, 0, 0, 0, 0, 0]
    idx: int = 0
    while idx < len(lines):
        level: str = parse_level(lines[idx])
        sev: int = classify_severity(level)
        counts[sev] = counts[sev] + 1
        idx = idx + 1
    return counts


def filter_min_severity(lines: list[str], min_sev: int) -> list[str]:
    """Filter log lines to only those at or above minimum severity."""
    result: list[str] = []
    idx: int = 0
    while idx < len(lines):
        level: str = parse_level(lines[idx])
        sev: int = classify_severity(level)
        if sev >= min_sev:
            result.append(lines[idx])
        idx = idx + 1
    return result


def contains_text(text: str, needle: str) -> int:
    """Check if text contains needle. Returns 1 if yes."""
    if len(needle) == 0:
        return 1
    if len(needle) > len(text):
        return 0
    idx: int = 0
    while idx <= len(text) - len(needle):
        match_ok: int = 1
        j: int = 0
        while j < len(needle):
            if text[idx + j] != needle[j]:
                match_ok = 0
                j = len(needle)
            else:
                j = j + 1
        if match_ok == 1:
            return 1
        idx = idx + 1
    return 0


def search_logs(lines: list[str], search_term: str) -> list[str]:
    """Search log lines for a substring in the message."""
    result: list[str] = []
    idx: int = 0
    while idx < len(lines):
        msg: str = parse_message(lines[idx])
        if contains_text(msg, search_term) == 1:
            result.append(lines[idx])
        idx = idx + 1
    return result


def test_module() -> int:
    """Test log parsing module."""
    passed: int = 0

    # Test 1: parse timestamp
    ts: str = parse_timestamp("[2024-01-15 10:30:00] INFO: started")
    if ts == "2024-01-15 10:30:00":
        passed = passed + 1

    # Test 2: parse log level
    level: str = parse_level("[2024-01-15] ERROR: disk full")
    if level == "ERROR":
        passed = passed + 1

    # Test 3: parse message
    msg: str = parse_message("[2024-01-15] WARN: low memory")
    if msg == "low memory":
        passed = passed + 1

    # Test 4: classify severity
    if classify_severity("error") == 4 and classify_severity("info") == 2:
        passed = passed + 1

    # Test 5: count by severity
    logs: list[str] = [
        "[ts] INFO: ok",
        "[ts] ERROR: bad",
        "[ts] WARN: slow",
        "[ts] ERROR: fail",
    ]
    counts: list[int] = count_by_severity(logs)
    if counts[2] == 1 and counts[4] == 2 and counts[3] == 1:
        passed = passed + 1

    # Test 6: filter by severity
    errors: list[str] = filter_min_severity(logs, 4)
    if len(errors) == 2:
        passed = passed + 1

    # Test 7: search logs
    found: list[str] = search_logs(logs, "bad")
    if len(found) == 1:
        passed = passed + 1

    # Test 8: strip spaces
    stripped: str = log_strip("  hello  ")
    if stripped == "hello":
        passed = passed + 1

    return passed
