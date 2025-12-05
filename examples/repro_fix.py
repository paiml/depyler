# DEPYLER-0714: Dict[str, int] uses serde_json for integer values
# Python pattern: d: Dict[str, int] = {}; d["key"] = 1
# Problem: Integer value wrapped in serde_json::json!() instead of native i32

from typing import Dict

def test_typed_dict():
    """Typed dict should use native types."""
    data: Dict[str, int] = {}
    data["a"] = 1
    data["b"] = 2
    return data["a"]
