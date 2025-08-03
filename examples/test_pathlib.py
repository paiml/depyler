# Test pathlib module mapping
from pathlib import Path
from typing import List

def get_python_files(directory: str) -> List[str]:
    """Get all Python files in a directory"""
    path = Path(directory)
    return [str(p) for p in path.glob("*.py")]

def create_nested_path(base: str, *parts: str) -> str:
    """Create a nested path from parts"""
    path = Path(base)
    for part in parts:
        path = path / part
    return str(path)

def get_file_info(filepath: str) -> tuple:
    """Get file information"""
    path = Path(filepath)
    return (path.name, path.suffix, path.parent.name)

def check_path_exists(filepath: str) -> bool:
    """Check if a path exists"""
    return Path(filepath).exists()