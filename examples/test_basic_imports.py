# Test basic module imports
import os
import sys

def get_current_directory() -> str:
    """Get current working directory"""
    return os.getcwd()

def get_args() -> list:
    """Get command line arguments"""
    return sys.argv

def exit_program(code: int):
    """Exit program with code"""
    sys.exit(code)