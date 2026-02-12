from typing import List, Tuple

def vig_encrypt(pt: List[int], key: List[int]) -> List[int]:
    result: List[int] = []
    ki: int = 0
    for b in pt:
        if b >= 65 and b <= 90:
            result.append((b - 65 + key[ki % len(key)]) % 26 + 65)
            ki = ki + 1
        elif b >= 97 and b <= 122:
            result.append((b - 97 + key[ki % len(key)]) % 26 + 97)
            ki = ki + 1
        else:
            result.append(b)
    return result

def vig_decrypt(ct: List[int], key: List[int]) -> List[int]:
    result: List[int] = []
    ki: int = 0
    for b in ct:
        if b >= 65 and b <= 90:
            inv_shift: int = 26 - (key[ki % len(key)] % 26)
            result.append((b - 65 + inv_shift) % 26 + 65)
            ki = ki + 1
        elif b >= 97 and b <= 122:
            inv_shift2: int = 26 - (key[ki % len(key)] % 26)
            result.append((b - 97 + inv_shift2) % 26 + 97)
            ki = ki + 1
        else:
            result.append(b)
    return result

def index_of_coincidence_scaled(data: List[int]) -> int:
    freq: List[int] = [0] * 26
    total: int = 0
    for b in data:
        if b >= 65 and b <= 90:
            freq[b - 65] = freq[b - 65] + 1
            total = total + 1
        elif b >= 97 and b <= 122:
            freq[b - 97] = freq[b - 97] + 1
            total = total + 1
    if total <= 1:
        return 0
    ic: int = 0
    for f in freq:
        ic = ic + f * (f - 1)
    denom: int = total * (total - 1)
    if denom == 0:
        return 0
    return (ic * 10000) // denom

def count_letters(data: List[int]) -> int:
    count: int = 0
    for b in data:
        if (b >= 65 and b <= 90) or (b >= 97 and b <= 122):
            count = count + 1
    return count

def estimate_key_len(ct: List[int], max_len: int) -> int:
    best: int = 1
    best_ic: int = 0
    for kl in range(1, max_len + 1):
        total_ic: int = 0
        for offset in range(kl):
            freq: List[int] = [0] * 26
            total: int = 0
            i: int = offset
            while i < len(ct):
                b: int = ct[i]
                if b >= 65 and b <= 90:
                    freq[b - 65] = freq[b - 65] + 1
                    total = total + 1
                elif b >= 97 and b <= 122:
                    freq[b - 97] = freq[b - 97] + 1
                    total = total + 1
                i = i + kl
            if total > 1:
                ic: int = 0
                for f in freq:
                    ic = ic + f * (f - 1)
                denom: int = total * (total - 1)
                if denom > 0:
                    total_ic = total_ic + (ic * 10000) // denom
        avg: int = total_ic // kl
        if avg > best_ic:
            best_ic = avg
            best = kl
    return best
