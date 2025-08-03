# Test JSON import mapping and function calls
import json
from typing import Dict, List

def parse_json_string(json_str: str) -> Dict:
    """Parse JSON from string"""
    return json.loads(json_str)

def to_json_string(data: Dict) -> str:
    """Convert data to JSON string"""
    return json.dumps(data)

def parse_json_with_default(json_str: str, default: Dict) -> Dict:
    """Parse JSON with a default value on error"""
    try:
        return json.loads(json_str)
    except:
        return default

def merge_json_objects(json1: str, json2: str) -> Dict:
    """Merge two JSON strings into one dictionary"""
    obj1 = json.loads(json1)
    obj2 = json.loads(json2)
    obj1.update(obj2)
    return obj1