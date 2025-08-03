from typing import Dict

def test_nested_assignment() -> Dict[str, Dict[str, str]]:
    """Test nested dictionary assignment"""
    d: Dict[str, Dict[str, str]] = {}
    
    # First create the outer dict entry
    d["outer"] = {}
    
    # Then do nested assignment
    d["outer"]["inner"] = "value"
    d["outer"]["another"] = "value2"
    
    return d