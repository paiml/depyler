#!/usr/bin/env python3
"""Test type inference for unannotated parameters (DEPYLER-0492)"""
import subprocess

def run_command(cmd, capture=False):
    """Test function with unannotated parameters.

    Type inference should infer:
    - cmd: Vec<String> (from subprocess.run signature)
    - capture: bool (from default value False)
    """
    if capture:
        result = subprocess.run(cmd, capture_output=True)
        return result.returncode, result.stdout
    else:
        result = subprocess.run(cmd)
        return result.returncode

def get_first(items):
    """Test indexing constraint.

    Type inference should infer:
    - items: Vec<T> (from indexing operation)
    """
    return items[0]

def get_rest(items):
    """Test slicing constraint.

    Type inference should infer:
    - items: Vec<T> (from slicing operation)
    """
    return items[1:]
