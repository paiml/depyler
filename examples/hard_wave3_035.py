"""Text processing: Serialization and deserialization primitives.

Tests: key-value encoding, list encoding, integer serialization,
structure flattening, format conversion.
"""

from typing import Dict, List, Tuple


def serialize_int_list(values: List[int]) -> str:
    """Serialize list of ints to comma-separated string."""
    result: List[str] = []
    i: int = 0
    while i < len(values):
        if i > 0:
            result.append(",")
        result.append(str(values[i]))
        i += 1
    return "".join(result)


def deserialize_int_list(s: str) -> List[int]:
    """Deserialize comma-separated string to list of ints."""
    result: List[int] = []
    current: int = 0
    negative: bool = False
    has_digit: bool = False
    i: int = 0
    while i < len(s):
        if s[i] == "-":
            negative = True
        elif s[i] == ",":
            if has_digit:
                if negative:
                    result.append(-current)
                else:
                    result.append(current)
            current = 0
            negative = False
            has_digit = False
        elif s[i] >= "0" and s[i] <= "9":
            current = current * 10 + (ord(s[i]) - ord("0"))
            has_digit = True
        i += 1
    if has_digit:
        if negative:
            result.append(-current)
        else:
            result.append(current)
    return result


def encode_kv_pairs(ks: List[str], vs: List[str]) -> str:
    """Encode key-value pairs as 'k1=v1;k2=v2;...'."""
    result: List[str] = []
    n: int = len(ks)
    if len(vs) < n:
        n = len(vs)
    i: int = 0
    while i < n:
        if i > 0:
            result.append(";")
        result.append(ks[i])
        result.append("=")
        result.append(vs[i])
        i += 1
    return "".join(result)


def decode_kv_string(encoded: str) -> Dict[str, str]:
    """Decode 'k1=v1;k2=v2;...' into a dictionary."""
    result: Dict[str, str] = {}
    i: int = 0
    n: int = len(encoded)
    while i < n:
        k_parts: List[str] = []
        while i < n and encoded[i] != "=" and encoded[i] != ";":
            k_parts.append(encoded[i])
            i += 1
        if i < n and encoded[i] == "=":
            i += 1
        v_parts: List[str] = []
        while i < n and encoded[i] != ";":
            v_parts.append(encoded[i])
            i += 1
        if i < n and encoded[i] == ";":
            i += 1
        k_str: str = "".join(k_parts)
        v_str: str = "".join(v_parts)
        if len(k_str) > 0:
            result[k_str] = v_str
    return result


def flatten_keys(prefix: str, keys: List[str]) -> List[str]:
    """Create flattened key paths like 'prefix.key1'."""
    result: List[str] = []
    for k in keys:
        parts: List[str] = [prefix, ".", k]
        result.append("".join(parts))
    return result


def test_serialization() -> bool:
    """Test serialization functions."""
    ok: bool = True
    s: str = serialize_int_list([1, 2, 3])
    if s != "1,2,3":
        ok = False
    lst: List[int] = deserialize_int_list("10,20,30")
    if len(lst) != 3:
        ok = False
    if lst[0] != 10:
        ok = False
    enc: str = encode_kv_pairs(["a", "b"], ["1", "2"])
    if enc != "a=1;b=2":
        ok = False
    dec: Dict[str, str] = decode_kv_string("x=1;y=2")
    if "x" not in dec:
        ok = False
    return ok
