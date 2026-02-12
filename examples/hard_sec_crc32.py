from typing import List, Tuple

def make_crc_table() -> List[int]:
    table: List[int] = []
    for i in range(256):
        crc: int = i
        for j in range(8):
            if (crc & 1) != 0:
                crc = (crc >> 1) ^ 0xEDB88320
            else:
                crc = crc >> 1
        table.append(crc & 0xFFFFFFFF)
    return table

def crc32_compute(data: List[int], table: List[int]) -> int:
    crc: int = 0xFFFFFFFF
    for byte in data:
        idx: int = (crc ^ byte) & 0xFF
        crc = (crc >> 8) ^ table[idx]
    return crc ^ 0xFFFFFFFF

def crc32_verify(data: List[int], expected: int, table: List[int]) -> bool:
    crc: int = 0xFFFFFFFF
    for byte in data:
        idx: int = (crc ^ byte) & 0xFF
        crc = (crc >> 8) ^ table[idx]
    computed: int = crc ^ 0xFFFFFFFF
    return computed == expected

def reflect_bits(value: int, width: int) -> int:
    result: int = 0
    for i in range(width):
        if (value & (1 << i)) != 0:
            result = result | (1 << (width - 1 - i))
    return result

def crc32_update(prev_crc: int, byte: int, table: List[int]) -> int:
    idx: int = (prev_crc ^ byte) & 0xFF
    return (prev_crc >> 8) ^ table[idx]
