# Test path utility functions (manual implementations without os module)
from typing import List


def join_path(base_dir: str, component: str) -> str:
    """Join two path components with separator."""
    if len(base_dir) == 0:
        return component
    if base_dir[len(base_dir) - 1] == "/":
        return base_dir + component
    return base_dir + "/" + component


def basename(path: str) -> str:
    """Get the base name (last component) of a path."""
    if len(path) == 0:
        return ""
    i: int = len(path) - 1
    while i >= 0:
        if path[i] == "/":
            return path[i + 1:]
        i = i - 1
    return path


def dirname(path: str) -> str:
    """Get the directory part of a path."""
    if len(path) == 0:
        return ""
    i: int = len(path) - 1
    while i >= 0:
        if path[i] == "/":
            return path[:i]
        i = i - 1
    return ""


def get_extension(path: str) -> str:
    """Get the file extension from a path."""
    base: str = basename(path)
    i: int = len(base) - 1
    while i >= 0:
        if base[i] == ".":
            return base[i:]
        i = i - 1
    return ""


def get_name_without_ext(path: str) -> str:
    """Get filename without extension."""
    base: str = basename(path)
    i: int = len(base) - 1
    while i >= 0:
        if base[i] == ".":
            return base[:i]
        i = i - 1
    return base


def ends_with_py(filename: str) -> int:
    """Check if filename ends with .py. Returns 1 if true, 0 if false."""
    ext: str = get_extension(filename)
    if ext == ".py":
        return 1
    return 0


def test_module() -> int:
    """Run all tests and return pass count."""
    passed: int = 0

    # Test join_path
    if join_path("/home", "user") == "/home/user":
        passed = passed + 1
    if join_path("/home/", "user") == "/home/user":
        passed = passed + 1
    if join_path("", "user") == "user":
        passed = passed + 1

    # Test basename
    if basename("/home/user/file.txt") == "file.txt":
        passed = passed + 1
    if basename("file.txt") == "file.txt":
        passed = passed + 1

    # Test dirname
    if dirname("/home/user/file.txt") == "/home/user":
        passed = passed + 1

    # Test get_extension
    if get_extension("/home/user/file.txt") == ".txt":
        passed = passed + 1
    if get_extension("script.py") == ".py":
        passed = passed + 1

    # Test get_name_without_ext
    if get_name_without_ext("script.py") == "script":
        passed = passed + 1

    # Test ends_with_py
    if ends_with_py("script.py") == 1:
        passed = passed + 1
    if ends_with_py("readme.md") == 0:
        passed = passed + 1

    return passed
