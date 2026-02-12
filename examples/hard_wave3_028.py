"""Text processing: Text compression and encoding schemes.

Tests: run-length encoding, delta encoding, variable-length encoding,
bit packing simulation, dictionary-based compression.
"""

from typing import Dict, List, Tuple


def rle_encode(data: str) -> str:
    """Run-length encode a string."""
    if len(data) == 0:
        return ""
    result: List[str] = []
    count: int = 1
    prev: str = data[0]
    i: int = 1
    while i < len(data):
        if data[i] == prev:
            count += 1
        else:
            result.append(str(count))
            result.append(prev)
            prev = data[i]
            count = 1
        i += 1
    result.append(str(count))
    result.append(prev)
    return "".join(result)


def rle_decode(encoded: str) -> str:
    """Run-length decode a string."""
    result: List[str] = []
    i: int = 0
    n: int = len(encoded)
    while i < n:
        num_val: int = 0
        has_num: bool = False
        while i < n and encoded[i] >= "0" and encoded[i] <= "9":
            num_val = num_val * 10 + (ord(encoded[i]) - ord("0"))
            has_num = True
            i += 1
        if i < n and has_num:
            ch: str = encoded[i]
            j: int = 0
            while j < num_val:
                result.append(ch)
                j += 1
            i += 1
    return "".join(result)


def delta_encode(values: List[int]) -> List[int]:
    """Delta encode a list of integers."""
    if len(values) == 0:
        return []
    result: List[int] = [values[0]]
    i: int = 1
    while i < len(values):
        result.append(values[i] - values[i - 1])
        i += 1
    return result


def delta_decode(encoded: List[int]) -> List[int]:
    """Delta decode a list of integers."""
    if len(encoded) == 0:
        return []
    result: List[int] = [encoded[0]]
    i: int = 1
    while i < len(encoded):
        result.append(result[i - 1] + encoded[i])
        i += 1
    return result


def char_frequency_map(text: str) -> Dict[str, int]:
    """Build character frequency map."""
    freq: Dict[str, int] = {}
    i: int = 0
    while i < len(text):
        ch: str = text[i]
        if ch in freq:
            freq[ch] = freq[ch] + 1
        else:
            freq[ch] = 1
        i += 1
    return freq


def most_frequent_char(text: str) -> str:
    """Find the most frequent character."""
    if len(text) == 0:
        return ""
    freq: Dict[str, int] = char_frequency_map(text)
    best_ch: str = text[0]
    best_count: int = 0
    for ch in freq:
        if freq[ch] > best_count:
            best_count = freq[ch]
            best_ch = ch
    return best_ch


def zigzag_encode(n: int) -> int:
    """ZigZag encode signed integer to unsigned (protobuf style)."""
    if n >= 0:
        return 2 * n
    neg: int = -n
    return 2 * neg - 1


def zigzag_decode(n: int) -> int:
    """ZigZag decode unsigned to signed."""
    if n % 2 == 0:
        return n // 2
    return -(n + 1) // 2


def test_compression() -> bool:
    """Test compression functions."""
    ok: bool = True
    enc: str = rle_encode("aaabbc")
    if enc != "3a2b1c":
        ok = False
    dec: str = rle_decode("3a2b1c")
    if dec != "aaabbc":
        ok = False
    de: List[int] = delta_encode([10, 12, 15, 20])
    dd: List[int] = delta_decode(de)
    if dd[3] != 20:
        ok = False
    zz: int = zigzag_encode(-1)
    if zz != 1:
        ok = False
    return ok
