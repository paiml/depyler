# Test sys module imports
import sys
from typing import List

def get_command_args() -> List[str]:
    """Get command line arguments"""
    return sys.argv[1:]

def exit_with_error(message: str, code: int = 1) -> None:
    """Print error to stderr and exit"""
    sys.stderr.write(f"Error: {message}\n")
    sys.exit(code)

def print_to_stdout(message: str) -> None:
    """Print message to stdout"""
    sys.stdout.write(message + "\n")
    sys.stdout.flush()

def read_from_stdin() -> str:
    """Read all input from stdin"""
    return sys.stdin.read()

def check_python_version() -> bool:
    """Check if Python version is at least 3.6"""
    return sys.version_info >= (3, 6)