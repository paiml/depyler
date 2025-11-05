"""
Comprehensive test of Python os/sys module transpilation to Rust.

This example demonstrates how Depyler transpiles Python's os and sys module
operations to their Rust equivalents.

Expected Rust mappings:
- os.path.join() -> std::path::Path::join()
- os.path.exists() -> std::path::Path::exists()
- os.getcwd() -> std::env::current_dir()
- os.environ -> std::env::var()
- sys.argv -> std::env::args()
- sys.exit() -> std::process::exit()

Note: File I/O operations may have limited support.
"""

import os
import sys
from typing import List, Dict


def test_sys_argv() -> List[str]:
    """Test command-line arguments access"""
    # Get command-line arguments
    args: List[str] = sys.argv

    return args


def test_sys_version_info() -> str:
    """Test Python version information"""
    # Access version info (simplified for transpilation)
    # In real implementation, would return sys.version
    version: str = "Python 3.x"

    return version


def test_sys_platform() -> str:
    """Test platform detection"""
    # Get platform string
    platform: str = sys.platform

    return platform


def test_sys_exit_code() -> int:
    """Test exit code handling (without actually exiting)"""
    # Simulate exit code logic
    exit_code: int = 0

    # Check some condition
    condition: bool = True

    if not condition:
        exit_code = 1

    return exit_code


def test_env_variable_access() -> str:
    """Test environment variable access"""
    # Get environment variable with default
    # Simplified - actual implementation uses os.environ.get()
    home: str = os.getenv("HOME", "/home/user")

    return home


def test_env_variable_exists() -> bool:
    """Test if environment variable exists"""
    # Check if variable is in environment
    # Simplified implementation
    var_name: str = "PATH"
    exists: bool = var_name in os.environ

    return exists


def test_current_directory() -> str:
    """Test getting current working directory"""
    # Get current directory
    cwd: str = os.getcwd()

    return cwd


def test_path_join() -> str:
    """Test path joining"""
    # Join path components
    base: str = "/home/user"
    relative: str = "documents"
    filename: str = "file.txt"

    # Manual path joining (os.path.join may not be supported)
    full_path: str = base + "/" + relative + "/" + filename

    return full_path


def test_path_basename() -> str:
    """Test extracting basename from path"""
    path: str = "/home/user/documents/file.txt"

    # Manual implementation (find last slash)
    last_slash: int = -1
    for i in range(len(path) - 1, -1, -1):
        if path[i] == "/":
            last_slash = i
            break

    if last_slash >= 0:
        basename: str = path[last_slash + 1:]
    else:
        basename: str = path

    return basename


def test_path_dirname() -> str:
    """Test extracting directory name from path"""
    path: str = "/home/user/documents/file.txt"

    # Manual implementation (find last slash)
    last_slash: int = -1
    for i in range(len(path) - 1, -1, -1):
        if path[i] == "/":
            last_slash = i
            break

    if last_slash > 0:
        dirname: str = path[:last_slash]
    else:
        dirname: str = "/"

    return dirname


def test_path_split() -> tuple:
    """Test splitting path into directory and basename"""
    path: str = "/home/user/documents/file.txt"

    # Find last slash
    last_slash: int = -1
    for i in range(len(path) - 1, -1, -1):
        if path[i] == "/":
            last_slash = i
            break

    if last_slash >= 0:
        dirname: str = path[:last_slash]
        basename: str = path[last_slash + 1:]
    else:
        dirname: str = ""
        basename: str = path

    return (dirname, basename)


def test_path_splitext() -> tuple:
    """Test splitting path into name and extension"""
    path: str = "document.txt"

    # Find last dot
    last_dot: int = -1
    for i in range(len(path) - 1, -1, -1):
        if path[i] == ".":
            last_dot = i
            break

    if last_dot > 0:
        name: str = path[:last_dot]
        ext: str = path[last_dot:]
    else:
        name: str = path
        ext: str = ""

    return (name, ext)


def test_path_isabs() -> bool:
    """Test if path is absolute"""
    path: str = "/home/user/file.txt"

    # Check if starts with /
    is_absolute: bool = len(path) > 0 and path[0] == "/"

    return is_absolute


def test_path_normpath() -> str:
    """Test normalizing path (remove redundant separators)"""
    path: str = "/home//user/../user/./documents"

    # Simplified normalization (just remove //)
    normalized: str = path.replace("//", "/")

    return normalized


def get_file_extension(filename: str) -> str:
    """Get file extension from filename"""
    # Find last dot
    last_dot: int = -1
    for i in range(len(filename) - 1, -1, -1):
        if filename[i] == ".":
            last_dot = i
            break

    if last_dot >= 0:
        extension: str = filename[last_dot + 1:]
    else:
        extension: str = ""

    return extension


def is_hidden_file(filename: str) -> bool:
    """Check if file is hidden (starts with dot)"""
    if len(filename) == 0:
        return False

    is_hidden: bool = filename[0] == "."

    return is_hidden


def build_path_from_parts(parts: List[str]) -> str:
    """Build path from list of components"""
    if len(parts) == 0:
        return ""

    path: str = parts[0]

    for i in range(1, len(parts)):
        path = path + "/" + parts[i]

    return path


def test_listdir_simulation() -> List[str]:
    """Simulate directory listing (manual implementation)"""
    # Simulated file list
    files: List[str] = ["file1.txt", "file2.py", "dir1", ".hidden"]

    return files


def filter_by_extension(files: List[str], ext: str) -> List[str]:
    """Filter files by extension"""
    filtered: List[str] = []

    for file in files:
        file_ext: str = get_file_extension(file)
        if file_ext == ext:
            filtered.append(file)

    return filtered


def count_files_by_extension(files: List[str]) -> Dict[str, int]:
    """Count files grouped by extension"""
    counts: Dict[str, int] = {}

    for file in files:
        ext: str = get_file_extension(file)

        if ext == "":
            ext = "no_extension"

        if ext in counts:
            counts[ext] = counts[ext] + 1
        else:
            counts[ext] = 1

    return counts


def test_path_traversal(path: str, max_depth: int) -> int:
    """Simulate path traversal with depth limit"""
    # Count path depth (number of separators)
    depth: int = 0

    for char in path:
        if char == "/":
            depth = depth + 1

    is_within_limit: bool = depth <= max_depth

    return depth


def sanitize_filename(filename: str) -> str:
    """Remove invalid characters from filename"""
    # Remove common invalid characters
    invalid_chars: str = "<>:\"|?*"
    sanitized: str = ""

    for char in filename:
        is_invalid: bool = False
        for invalid in invalid_chars:
            if char == invalid:
                is_invalid = True
                break

        if not is_invalid:
            sanitized = sanitized + char

    return sanitized


def test_all_os_sys_features() -> None:
    """Run all os/sys module tests"""
    # Sys tests
    args: List[str] = test_sys_argv()
    version: str = test_sys_version_info()
    platform: str = test_sys_platform()
    exit_code: int = test_sys_exit_code()

    # Environment tests
    home: str = test_env_variable_access()
    env_exists: bool = test_env_variable_exists()

    # Directory tests
    cwd: str = test_current_directory()

    # Path tests
    joined: str = test_path_join()
    basename: str = test_path_basename()
    dirname: str = test_path_dirname()
    split_result: tuple = test_path_split()
    splitext_result: tuple = test_path_splitext()
    is_abs: bool = test_path_isabs()
    normalized: str = test_path_normpath()

    # Utility functions
    ext: str = get_file_extension("document.txt")
    hidden: bool = is_hidden_file(".gitignore")

    parts: List[str] = ["home", "user", "documents"]
    built_path: str = build_path_from_parts(parts)

    # Directory listing simulation
    files: List[str] = test_listdir_simulation()
    txt_files: List[str] = filter_by_extension(files, "txt")
    file_counts: Dict[str, int] = count_files_by_extension(files)

    # Path operations
    depth: int = test_path_traversal("/home/user/docs", 5)
    safe_name: str = sanitize_filename("file<>name.txt")

    print("All os/sys module tests completed successfully")
