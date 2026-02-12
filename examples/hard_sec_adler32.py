from typing import List, Tuple

def adler32(data: List[int]) -> int:
    mod_val: int = 65521
    a: int = 1
    b: int = 0
    for byte in data:
        a = (a + byte) % mod_val
        b = (b + a) % mod_val
    return (b << 16) | a

def adler32_verify(data: List[int], expected: int) -> bool:
    mod_val: int = 65521
    a: int = 1
    b: int = 0
    for byte in data:
        a = (a + byte) % mod_val
        b = (b + a) % mod_val
    computed: int = (b << 16) | a
    return computed == expected

def adler32_batch(chunks: List[List[int]]) -> List[int]:
    results: List[int] = []
    for chunk in chunks:
        mod_val: int = 65521
        a: int = 1
        b: int = 0
        for byte in chunk:
            a = (a + byte) % mod_val
            b = (b + a) % mod_val
        results.append((b << 16) | a)
    return results

def adler32_incremental(data: List[int], chunk_size: int) -> List[int]:
    checksums: List[int] = []
    i: int = 0
    while i < len(data):
        end: int = i + chunk_size
        if end > len(data):
            end = len(data)
        mod_val: int = 65521
        a: int = 1
        b: int = 0
        for j in range(i, end):
            a = (a + data[j]) % mod_val
            b = (b + a) % mod_val
        checksums.append((b << 16) | a)
        i = i + chunk_size
    return checksums

def adler32_rolling(s1: int, s2: int, old_byte: int, new_byte: int, mod_val: int) -> Tuple[int, int]:
    ns1: int = (s1 - old_byte + new_byte) % mod_val
    ns2: int = (s2 + ns1 - 1) % mod_val
    return (ns1, ns2)
