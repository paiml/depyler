from typing import List, Dict, Any

def rc1_string_matching_exploit(values: List[str]) -> int:
    """
    RC-1 FALSIFICATION:
    The compiler uses string matching `if "parse" in stmt`.
    
    Attack Vectors:
    1. A variable named 'parse_parser' (should be valid, might trigger false positives).
    2. A comment containing 'parse' (might trigger logic).
    3. A for-loop containing a parse (current known failure).
    """
    total: int = 0
    # Attack 1: Variable naming conflict
    parse_parser: str = "10"
    
    try:
        # Attack 2: The comment below contains the trigger word
        # parse the values carefully
        
        # Attack 3: The known failure - logic inside a compound statement
        for v in values:
            num: int = int(v)
            total += num
            
        return total + int(parse_parser)
    except ValueError:
        return -1

def rc2_heterogeneous_dict_exploit(arg: str) -> Dict[str, Any]:
    """
    RC-2 FALSIFICATION:
    The compiler assumes Dicts are homogeneous HashMap<T, T>.
    
    Attack Vector:
    1. Create a dict with mixed types (str, int, bool).
    2. The compiler will likely infer the type of the first element 
       and fail on the subsequent ones.
    """
    # Attack 1: Mixed types
    payload = {
        "key_str": "value",      # Inferred as HashMap<String, String>?
        "key_int": 42,           # Boom: expected String, found integer
        "key_bool": True,        # Boom: expected String, found bool
        "key_param": arg         # Runtime variable mixed in
    }
    return payload

def rc3_string_index_exploit(s: str) -> str:
    """
    RC-3 FALSIFICATION:
    The compiler blindly translates s[i] to s[i as usize].
    
    Attack Vector:
    1. Index a string. Rust Strings are not indexable.
    """
    try:
        # Attack 1: Direct indexing
        c: str = s[0]
        return c
    except IndexError:
        return ""
