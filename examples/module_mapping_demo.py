#!/usr/bin/env python3
"""
Example: Module Mapping Demonstration

This example shows how various Python imports are mapped to their
Rust equivalents by Depyler's module mapping system.
"""

# Standard library imports
import os
import sys
from os.path import join, exists
from sys import argv, exit as sys_exit

# External package imports  
import json
import re
from datetime import datetime, timedelta
from typing import List, Dict, Optional, Union

# Collections and itertools
from collections import defaultdict, deque
import itertools

# Other common modules
import random
import math
import hashlib
import base64
from pathlib import Path
import tempfile
import csv

def demonstrate_os_operations():
    """Show how OS operations map to Rust."""
    # os.getcwd() -> std::env::current_dir()
    current_dir = os.getcwd()
    print(f"Current directory: {current_dir}")
    
    # os.environ -> std::env::vars()
    for key, value in os.environ.items():
        if key.startswith("PATH"):
            print(f"{key}: {value[:50]}...")
    
    # os.path operations -> std::path::Path methods
    path = join("/tmp", "test.txt")
    if exists(path):
        print(f"Path exists: {path}")

def demonstrate_json_operations(data: Dict[str, any]):
    """Show how JSON operations map to serde_json."""
    # json.dumps() -> serde_json::to_string()
    json_str = json.dumps(data, indent=2)
    
    # json.loads() -> serde_json::from_str()
    parsed = json.loads(json_str)
    
    return parsed

def demonstrate_regex_operations(text: str, pattern: str):
    """Show how regex operations map to the regex crate."""
    # re.compile() -> regex::Regex::new()
    regex = re.compile(pattern)
    
    # re.search() -> regex::Regex::find()
    match = regex.search(text)
    if match:
        return match.group()
    
    # re.findall() -> regex::Regex::find_iter()
    all_matches = regex.findall(text)
    return all_matches

def demonstrate_datetime_operations():
    """Show how datetime operations map to chrono."""
    # datetime.datetime -> chrono::DateTime
    now = datetime.now()
    
    # timedelta -> chrono::Duration
    delta = timedelta(days=7, hours=3, minutes=30)
    
    future = now + delta
    return future

def demonstrate_typing_annotations(
    items: List[int],
    config: Dict[str, str],
    value: Optional[float],
    result: Union[int, str]
) -> List[str]:
    """Show how typing annotations map to Rust types.
    
    List -> Vec
    Dict -> HashMap
    Optional -> Option
    Union -> (custom enum or trait object)
    """
    return [str(item) for item in items]

def demonstrate_collections():
    """Show how collections map to Rust standard library."""
    # defaultdict -> HashMap with default
    counts = defaultdict(int)
    
    # deque -> VecDeque
    queue = deque([1, 2, 3])
    queue.append(4)
    queue.appendleft(0)
    
    return dict(counts), list(queue)

def demonstrate_math_operations(x: float) -> float:
    """Show how math operations map to std::f64."""
    # math functions -> f64 methods
    result = math.sqrt(x)
    result += math.sin(math.pi / 4)
    result *= math.cos(math.e)
    
    return result

def demonstrate_random_operations(items: List[str]) -> str:
    """Show how random operations map to the rand crate."""
    # random.choice() -> rand::seq::SliceRandom::choose()
    choice = random.choice(items)
    
    # random.randint() -> rand::Rng::gen_range()
    num = random.randint(1, 100)
    
    return f"{choice}:{num}"

def demonstrate_itertools_operations(data: List[int]):
    """Show how itertools maps to the itertools crate."""
    # itertools.chain() -> itertools::chain()
    chained = itertools.chain(data, [100, 200])
    
    # itertools.combinations() -> itertools::combinations()
    combos = itertools.combinations(data, 2)
    
    # itertools.groupby() -> itertools::group_by()
    grouped = itertools.groupby(data, key=lambda x: x % 2)
    
    return list(chained), list(combos), list(grouped)

def demonstrate_hash_operations(data: bytes) -> str:
    """Show how hashlib maps to crypto crates."""
    # hashlib.sha256() -> sha2::Sha256
    hash_obj = hashlib.sha256()
    hash_obj.update(data)
    digest = hash_obj.hexdigest()
    
    # base64.b64encode() -> base64::encode()
    encoded = base64.b64encode(data).decode('utf-8')
    
    return f"SHA256: {digest}, Base64: {encoded}"

def demonstrate_path_operations():
    """Show how pathlib maps to std::path."""
    # Path -> PathBuf
    path = Path("/tmp/test.txt")
    
    # Path methods map to Path/PathBuf methods
    if path.exists():
        parent = path.parent
        name = path.name
        return f"Parent: {parent}, Name: {name}"
    
    return "Path does not exist"

def demonstrate_file_operations():
    """Show how tempfile maps to the tempfile crate."""
    # NamedTemporaryFile -> tempfile::NamedTempFile
    with tempfile.NamedTemporaryFile(mode='w', delete=False) as tmp:
        tmp.write("Test data")
        return tmp.name

def demonstrate_csv_operations(filename: str):
    """Show how csv module maps to the csv crate."""
    # csv.reader() -> csv::Reader
    # csv.writer() -> csv::Writer
    data = [
        ['Name', 'Age', 'City'],
        ['Alice', '30', 'New York'],
        ['Bob', '25', 'San Francisco']
    ]
    
    # Write CSV
    with open(filename, 'w', newline='') as f:
        writer = csv.writer(f)
        writer.writerows(data)
    
    # Read CSV
    with open(filename, 'r') as f:
        reader = csv.reader(f)
        rows = list(reader)
    
    return rows

if __name__ == "__main__":
    print("Module Mapping Demonstration")
    print("=" * 40)
    
    # Demonstrate various module mappings
    demonstrate_os_operations()
    
    data = {"name": "test", "value": 42}
    result = demonstrate_json_operations(data)
    print(f"JSON result: {result}")
    
    matches = demonstrate_regex_operations("Hello 123 World 456", r"\d+")
    print(f"Regex matches: {matches}")
    
    future = demonstrate_datetime_operations()
    print(f"Future date: {future}")
    
    typed_result = demonstrate_typing_annotations(
        [1, 2, 3],
        {"key": "value"},
        3.14,
        "result"
    )
    print(f"Typed result: {typed_result}")
    
    collections_result = demonstrate_collections()
    print(f"Collections: {collections_result}")
    
    math_result = demonstrate_math_operations(16.0)
    print(f"Math result: {math_result}")
    
    random_result = demonstrate_random_operations(["apple", "banana", "cherry"])
    print(f"Random result: {random_result}")
    
    hash_result = demonstrate_hash_operations(b"Hello, World!")
    print(f"Hash result: {hash_result}")
    
    path_result = demonstrate_path_operations()
    print(f"Path result: {path_result}")
    
    temp_file = demonstrate_file_operations()
    print(f"Temp file: {temp_file}")
    
    # Clean up temp file
    os.unlink(temp_file)