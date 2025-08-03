# Test module imports

import os
import sys
from json import loads, dumps
from os.path import join as path_join
import re
from typing import List, Dict

def get_current_dir() -> str:
    return os.getcwd()

def parse_json(data: str) -> Dict:
    return loads(data)

def join_paths(base: str, *paths: str) -> str:
    result = base
    for p in paths:
        result = path_join(result, p)
    return result

def find_pattern(text: str, pattern: str) -> List[str]:
    regex = re.compile(pattern)
    return regex.findall(text)