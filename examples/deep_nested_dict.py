from typing import Dict

def test_deep_nested() -> Dict[str, Dict[str, Dict[str, str]]]:
    """Test deeply nested dictionary assignment"""
    d: Dict[str, Dict[str, Dict[str, str]]] = {}
    
    # Build structure step by step
    d["level1"] = {}
    d["level1"]["level2"] = {}
    d["level1"]["level2"]["level3"] = "deep value"
    
    return d