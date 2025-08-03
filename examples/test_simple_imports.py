# Test simple module imports without complex features
import json
import os
import sys
from typing import Dict, List

def parse_json(text: str) -> Dict:
    """Parse JSON from string"""
    return json.loads(text)

def serialize_json(data: Dict) -> str:
    """Convert dictionary to JSON string"""
    return json.dumps(data)

def get_env_var(name: str) -> str:
    """Get environment variable"""
    return os.getenv(name, "")

def get_current_directory() -> str:
    """Get current working directory"""
    return os.getcwd()

def get_args() -> List[str]:
    """Get command line arguments"""
    return sys.argv

def exit_program(code: int) -> None:
    """Exit program with code"""
    sys.exit(code)