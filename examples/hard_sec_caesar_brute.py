from typing import List, Tuple

def caesar_enc_byte(byte: int, shift: int) -> int:
    if byte >= 65 and byte <= 90:
        return (byte - 65 + shift) % 26 + 65
    if byte >= 97 and byte <= 122:
        return (byte - 97 + shift) % 26 + 97
    return byte

def caesar_encrypt(pt: List[int], shift: int) -> List[int]:
    result: List[int] = []
    for b in pt:
        if b >= 65 and b <= 90:
            result.append((b - 65 + shift) % 26 + 65)
        elif b >= 97 and b <= 122:
            result.append((b - 97 + shift) % 26 + 97)
        else:
            result.append(b)
    return result

def caesar_decrypt(ct: List[int], shift: int) -> List[int]:
    actual_shift: int = 26 - (shift % 26)
    result: List[int] = []
    for b in ct:
        if b >= 65 and b <= 90:
            result.append((b - 65 + actual_shift) % 26 + 65)
        elif b >= 97 and b <= 122:
            result.append((b - 97 + actual_shift) % 26 + 97)
        else:
            result.append(b)
    return result

def frequency_analysis(data: List[int]) -> List[int]:
    freq: List[int] = [0] * 26
    for b in data:
        if b >= 65 and b <= 90:
            freq[b - 65] = freq[b - 65] + 1
        elif b >= 97 and b <= 122:
            freq[b - 97] = freq[b - 97] + 1
    return freq

def brute_force_best_shift(ct: List[int]) -> int:
    best_shift: int = 0
    best_score: int = 0
    for shift in range(26):
        actual_shift: int = 26 - (shift % 26)
        score: int = 0
        for b in ct:
            decrypted: int = b
            if b >= 65 and b <= 90:
                decrypted = (b - 65 + actual_shift) % 26 + 65
            elif b >= 97 and b <= 122:
                decrypted = (b - 97 + actual_shift) % 26 + 97
            if decrypted == 101 or decrypted == 116 or decrypted == 97:
                score = score + 1
        if score > best_score:
            best_score = score
            best_shift = shift
    return best_shift
