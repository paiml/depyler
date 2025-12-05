# DEPYLER-0718: Dict without type parameters generates undefined type V
# Python pattern: Dict (bare annotation) or Dict[str, Any]
# Problem: Generates HashMap<String, V> where V is undefined
# Expected: HashMap<String, serde_json::Value> or similar

from typing import Dict

def parse_data(text: str) -> Dict:
    """Parse some data into a dictionary."""
    result: Dict = {}
    result["key"] = "value"
    return result
