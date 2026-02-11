# Test module imports (simplified for transpiler compatibility)


def get_current_dir_name() -> str:
    """Return a placeholder current directory name."""
    return "/tmp"


def parse_simple_value(data: str) -> str:
    """Extract a simple value from a key-value string."""
    parts: list[str] = data.split(":")
    if len(parts) > 1:
        return parts[1]
    return data


def join_two_paths(base: str, suffix: str) -> str:
    """Join two path segments with a separator."""
    if base.endswith("/"):
        return base + suffix
    return base + "/" + suffix


def find_substring_count(text: str, pattern: str) -> int:
    """Count occurrences of a pattern in text using simple search."""
    count: int = 0
    text_len: int = len(text)
    pat_len: int = len(pattern)
    if pat_len == 0:
        return 0
    pos: int = 0
    while pos <= text_len - pat_len:
        match: int = 1
        ci: int = 0
        while ci < pat_len:
            if text[pos + ci] != pattern[ci]:
                match = 0
                ci = pat_len
            ci = ci + 1
        if match == 1:
            count = count + 1
        pos = pos + 1
    return count


def test_imports() -> int:
    """Test the simplified import replacements."""
    dir_name: str = get_current_dir_name()
    if len(dir_name) == 0:
        return 0

    val: str = parse_simple_value("name:hello")
    if val != "hello":
        return 0

    joined: str = join_two_paths("/home", "user")
    if joined != "/home/user":
        return 0

    cnt: int = find_substring_count("hello hello hello", "hello")
    if cnt != 3:
        return 0

    return 1


if __name__ == "__main__":
    result: int = test_imports()
    if result != 1:
        raise ValueError("test_imports failed")
