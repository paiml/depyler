# Test path utility functions (manual implementations without pathlib)
from typing import List


def path_join(base: str, part: str) -> str:
    """Join two path parts."""
    if len(base) == 0:
        return part
    if base[len(base) - 1] == "/":
        return base + part
    return base + "/" + part


def path_name(filepath: str) -> str:
    """Get the filename from a path (like Path.name)."""
    if len(filepath) == 0:
        return ""
    i: int = len(filepath) - 1
    while i >= 0:
        if filepath[i] == "/":
            return filepath[i + 1:]
        i = i - 1
    return filepath


def path_suffix(filepath: str) -> str:
    """Get the file suffix/extension (like Path.suffix)."""
    name: str = path_name(filepath)
    i: int = len(name) - 1
    while i >= 0:
        if name[i] == ".":
            return name[i:]
        i = i - 1
    return ""


def path_parent_name(filepath: str) -> str:
    """Get the parent directory name (like Path.parent.name)."""
    # Find the last slash
    last_slash: int = -1
    i: int = len(filepath) - 1
    while i >= 0:
        if filepath[i] == "/":
            last_slash = i
            i = -1  # break
        else:
            i = i - 1
    if last_slash <= 0:
        return ""
    # Find the second-to-last slash
    j: int = last_slash - 1
    while j >= 0:
        if filepath[j] == "/":
            return filepath[j + 1:last_slash]
        j = j - 1
    return filepath[:last_slash]


def create_nested_path(base: str, part1: str, part2: str) -> str:
    """Create a nested path from base and two parts."""
    step1: str = path_join(base, part1)
    return path_join(step1, part2)


def check_has_extension(filepath: str, ext: str) -> int:
    """Check if filepath has given extension. Returns 1 if yes, 0 if no."""
    actual: str = path_suffix(filepath)
    if actual == ext:
        return 1
    return 0


def test_module() -> int:
    """Run all tests."""
    passed: int = 0

    # Test path_join
    if path_join("/home", "user") == "/home/user":
        passed = passed + 1
    if path_join("/home/", "user") == "/home/user":
        passed = passed + 1

    # Test path_name
    if path_name("/home/user/file.txt") == "file.txt":
        passed = passed + 1
    if path_name("file.txt") == "file.txt":
        passed = passed + 1

    # Test path_suffix
    if path_suffix("/home/user/file.txt") == ".txt":
        passed = passed + 1
    if path_suffix("script.py") == ".py":
        passed = passed + 1

    # Test path_parent_name
    if path_parent_name("/home/user/file.txt") == "user":
        passed = passed + 1

    # Test create_nested_path
    if create_nested_path("/home", "user", "docs") == "/home/user/docs":
        passed = passed + 1

    # Test check_has_extension
    if check_has_extension("script.py", ".py") == 1:
        passed = passed + 1
    if check_has_extension("readme.md", ".py") == 0:
        passed = passed + 1

    return passed
