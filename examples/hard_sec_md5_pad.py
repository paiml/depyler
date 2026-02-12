from typing import List, Tuple

def left_rotate(x: int, amount: int) -> int:
    x = x & 0xFFFFFFFF
    return ((x << amount) | (x >> (32 - amount))) & 0xFFFFFFFF

def md5_pad(message: List[int]) -> List[int]:
    msg_len: int = len(message)
    bit_len: int = msg_len * 8
    padded: List[int] = []
    for b in message:
        padded.append(b)
    padded.append(0x80)
    while (len(padded) % 64) != 56:
        padded.append(0)
    for i in range(8):
        padded.append((bit_len >> (i * 8)) & 0xFF)
    return padded

def md5_f(x: int, y: int, z: int) -> int:
    return (x & y) | ((x ^ 0xFFFFFFFF) & z)

def md5_g(x: int, y: int, z: int) -> int:
    return (x & z) | (y & (z ^ 0xFFFFFFFF))

def md5_h(x: int, y: int, z: int) -> int:
    return x ^ y ^ z

def bytes_to_word_le(data: List[int], offset: int) -> int:
    return data[offset] | (data[offset + 1] << 8) | (data[offset + 2] << 16) | (data[offset + 3] << 24)

def md5_init() -> List[int]:
    return [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476]

def count_blocks(msg_len: int) -> int:
    padded_len: int = msg_len + 1
    while padded_len % 64 != 56:
        padded_len = padded_len + 1
    padded_len = padded_len + 8
    return padded_len // 64
