from typing import List, Tuple

def generate_pad(seed: int, length: int) -> List[int]:
    pad: List[int] = []
    val: int = seed
    for i in range(length):
        val = ((val * 1103515245) + 12345) & 0x7FFFFFFF
        pad.append(val & 0xFF)
    return pad

def otp_encrypt(pt: List[int], pad: List[int]) -> List[int]:
    ct: List[int] = []
    for i in range(len(pt)):
        ct.append(pt[i] ^ pad[i])
    return ct

def otp_decrypt(ct: List[int], pad: List[int]) -> List[int]:
    return otp_encrypt(ct, pad)

def verify_otp(pt: List[int], ct: List[int], pad: List[int]) -> bool:
    if len(pt) != len(ct):
        return False
    for i in range(len(pt)):
        if (pt[i] ^ pad[i]) != ct[i]:
            return False
    return True

def pad_entropy(pad: List[int]) -> float:
    freq: List[int] = [0] * 256
    for b in pad:
        freq[b] = freq[b] + 1
    total: float = float(len(pad))
    entropy: float = 0.0
    for f in freq:
        if f > 0:
            p: float = float(f) / total
            entropy = entropy - p * p
    return entropy

def split_pad(pad: List[int], parts: int) -> List[List[int]]:
    result: List[List[int]] = []
    chunk: int = len(pad) // parts
    for i in range(parts):
        start: int = i * chunk
        end: int = start + chunk
        if i == parts - 1:
            end = len(pad)
        part: List[int] = []
        for j in range(start, end):
            part.append(pad[j])
        result.append(part)
    return result
