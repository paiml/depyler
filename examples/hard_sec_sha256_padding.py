from typing import List, Tuple

def to_bits(byte: int) -> List[int]:
    """Convert a byte to a list of 8 bits."""
    bits: List[int] = []
    for i in range(8):
        bits.append((byte >> (7 - i)) & 1)
    return bits

def from_bits(bits: List[int]) -> int:
    """Convert 8 bits back to a byte."""
    result: int = 0
    for i in range(8):
        result = result | (bits[i] << (7 - i))
    return result

def pad_message(msg_bytes: List[int]) -> List[int]:
    """Apply SHA-256 padding to a message."""
    msg_len: int = len(msg_bytes)
    bit_len: int = msg_len * 8
    padded: List[int] = []
    for b in msg_bytes:
        padded.append(b)
    padded.append(0x80)
    while (len(padded) % 64) != 56:
        padded.append(0x00)
    for i in range(8):
        shift: int = (7 - i) * 8
        padded.append((bit_len >> shift) & 0xFF)
    return padded

def right_rotate(value: int, amount: int) -> int:
    """32-bit right rotation."""
    value = value & 0xFFFFFFFF
    return ((value >> amount) | (value << (32 - amount))) & 0xFFFFFFFF

def right_shift(value: int, amount: int) -> int:
    """Logical right shift for 32-bit value."""
    return (value & 0xFFFFFFFF) >> amount

def sigma0(x: int) -> int:
    """SHA-256 sigma0 function."""
    return right_rotate(x, 2) ^ right_rotate(x, 13) ^ right_rotate(x, 22)

def sigma1(x: int) -> int:
    """SHA-256 sigma1 function."""
    return right_rotate(x, 6) ^ right_rotate(x, 11) ^ right_rotate(x, 25)

def little_sigma0(x: int) -> int:
    """SHA-256 message schedule sigma0."""
    return right_rotate(x, 7) ^ right_rotate(x, 18) ^ right_shift(x, 3)

def little_sigma1(x: int) -> int:
    """SHA-256 message schedule sigma1."""
    return right_rotate(x, 17) ^ right_rotate(x, 19) ^ right_shift(x, 10)

def ch(x: int, y: int, z: int) -> int:
    """SHA-256 Ch function."""
    return (x & y) ^ ((x ^ 0xFFFFFFFF) & z)

def maj(x: int, y: int, z: int) -> int:
    """SHA-256 Maj function."""
    return (x & y) ^ (x & z) ^ (y & z)

def parse_block(padded: List[int], block_idx: int) -> List[int]:
    """Parse a 512-bit block into 16 32-bit words."""
    words: List[int] = []
    start: int = block_idx * 64
    for i in range(16):
        w: int = 0
        for j in range(4):
            w = (w << 8) | padded[start + i * 4 + j]
        words.append(w)
    return words

def expand_schedule(words: List[int]) -> List[int]:
    """Expand 16 words to 64-word message schedule."""
    w: List[int] = []
    for i in range(16):
        w.append(words[i])
    for i in range(16, 64):
        val: int = (little_sigma1(w[i - 2]) + w[i - 7] + little_sigma0(w[i - 15]) + w[i - 16]) & 0xFFFFFFFF
        w.append(val)
    return w

def count_padding_bytes(msg_len: int) -> int:
    """Count how many padding bytes are needed."""
    remainder: int = (msg_len + 1) % 64
    if remainder <= 56:
        return 56 - remainder + 1 + 8
    else:
        return 64 - remainder + 56 + 1 + 8
