from typing import List, Tuple

def fletcher16(data: List[int]) -> int:
    sum1: int = 0
    sum2: int = 0
    for byte in data:
        sum1 = (sum1 + byte) % 255
        sum2 = (sum2 + sum1) % 255
    return (sum2 << 8) | sum1

def fletcher16_verify(data: List[int], expected: int) -> bool:
    sum1: int = 0
    sum2: int = 0
    for byte in data:
        sum1 = (sum1 + byte) % 255
        sum2 = (sum2 + sum1) % 255
    computed: int = (sum2 << 8) | sum1
    return computed == expected

def fletcher32(data: List[int]) -> int:
    sum1: int = 0
    sum2: int = 0
    for word in data:
        sum1 = (sum1 + word) % 65535
        sum2 = (sum2 + sum1) % 65535
    return (sum2 << 16) | sum1

def fletcher_step(s1: int, s2: int, new_byte: int) -> Tuple[int, int]:
    ns1: int = (s1 + new_byte) % 255
    ns2: int = (s2 + ns1) % 255
    return (ns1, ns2)

def fletcher16_batch(chunks: List[List[int]]) -> List[int]:
    results: List[int] = []
    for chunk in chunks:
        s1: int = 0
        s2: int = 0
        for byte in chunk:
            s1 = (s1 + byte) % 255
            s2 = (s2 + s1) % 255
        results.append((s2 << 8) | s1)
    return results
