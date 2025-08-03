# Test os.path module imports
import os
from os.path import join, exists, basename, dirname, splitext
from typing import List, Tuple

def build_file_path(base_dir: str, *components: str) -> str:
    """Build a file path from components"""
    return join(base_dir, *components)

def check_file_exists(path: str) -> bool:
    """Check if a file exists"""
    return exists(path)

def get_file_info(path: str) -> Tuple[str, str, str]:
    """Get directory, filename, and extension"""
    dir_path = dirname(path)
    base_name = basename(path)
    name, ext = splitext(base_name)
    return dir_path, name, ext

def find_python_files(directory: str) -> List[str]:
    """Find all Python files in a directory"""
    python_files = []
    for root, dirs, files in os.walk(directory):
        for file in files:
            if file.endswith('.py'):
                python_files.append(join(root, file))
    return python_files

def normalize_path(path: str) -> str:
    """Normalize a file path"""
    return os.path.normpath(path)

def get_relative_path(path: str, start: str) -> str:
    """Get relative path from start to path"""
    return os.path.relpath(path, start)