"""Basic crypto: XOR cipher, Caesar cipher, Vigenere cipher.

Tests: xor_cipher, caesar_encrypt, caesar_decrypt, vigenere_encrypt, vigenere_decrypt.
"""


def xor_cipher(data: list[int], xor_val: int) -> list[int]:
    """XOR each byte with given value. Symmetric cipher."""
    result: list[int] = []
    i: int = 0
    n: int = len(data)
    while i < n:
        result.append(data[i] ^ xor_val)
        i = i + 1
    return result


def xor_cipher_repeated(data: list[int], xor_arr: list[int]) -> list[int]:
    """XOR each byte with repeating array of values."""
    result: list[int] = []
    n: int = len(data)
    klen: int = len(xor_arr)
    i: int = 0
    while i < n:
        ki: int = i % klen
        result.append(data[i] ^ xor_arr[ki])
        i = i + 1
    return result


def caesar_shift_char(ch_val: int, shift: int) -> int:
    """Shift a lowercase letter (97-122) by shift positions."""
    if ch_val < 97:
        return ch_val
    if ch_val > 122:
        return ch_val
    shifted: int = ((ch_val - 97 + shift) % 26) + 97
    return shifted


def caesar_encrypt(data: list[int], shift: int) -> list[int]:
    """Caesar cipher: shift lowercase letters by shift positions."""
    result: list[int] = []
    i: int = 0
    n: int = len(data)
    while i < n:
        shifted: int = caesar_shift_char(data[i], shift)
        result.append(shifted)
        i = i + 1
    return result


def caesar_decrypt(data: list[int], shift: int) -> list[int]:
    """Caesar cipher decrypt: shift back."""
    return caesar_encrypt(data, 26 - (shift % 26))


def vigenere_encrypt(data: list[int], shifts: list[int]) -> list[int]:
    """Vigenere cipher: each position shifted by corresponding key value."""
    result: list[int] = []
    n: int = len(data)
    klen: int = len(shifts)
    i: int = 0
    while i < n:
        ki: int = i % klen
        s: int = shifts[ki]
        shifted: int = caesar_shift_char(data[i], s)
        result.append(shifted)
        i = i + 1
    return result


def vigenere_decrypt(data: list[int], shifts: list[int]) -> list[int]:
    """Vigenere cipher decrypt."""
    result: list[int] = []
    n: int = len(data)
    klen: int = len(shifts)
    i: int = 0
    while i < n:
        ki: int = i % klen
        s: int = shifts[ki]
        back_shift: int = 26 - (s % 26)
        shifted: int = caesar_shift_char(data[i], back_shift)
        result.append(shifted)
        i = i + 1
    return result


def frequency_analysis(data: list[int]) -> list[int]:
    """Count frequency of each value 0-255. Returns 256-element list."""
    freq: list[int] = []
    i: int = 0
    while i < 256:
        freq.append(0)
        i = i + 1
    j: int = 0
    n: int = len(data)
    while j < n:
        val: int = data[j]
        if val >= 0:
            if val < 256:
                freq[val] = freq[val] + 1
        j = j + 1
    return freq


def test_module() -> int:
    """Test crypto algorithms."""
    passed: int = 0

    data: list[int] = [1, 2, 3, 4, 5]
    enc: list[int] = xor_cipher(data, 42)
    dec: list[int] = xor_cipher(enc, 42)
    if dec == [1, 2, 3, 4, 5]:
        passed = passed + 1

    abc: list[int] = [97, 98, 99]
    ce: list[int] = caesar_encrypt(abc, 3)
    if ce == [100, 101, 102]:
        passed = passed + 1

    cd: list[int] = caesar_decrypt(ce, 3)
    if cd == [97, 98, 99]:
        passed = passed + 1

    ve: list[int] = vigenere_encrypt([97, 98, 99], [1, 2, 3])
    vd: list[int] = vigenere_decrypt(ve, [1, 2, 3])
    if vd == [97, 98, 99]:
        passed = passed + 1

    rk: list[int] = xor_cipher_repeated([10, 20, 30], [5, 7])
    rk2: list[int] = xor_cipher_repeated(rk, [5, 7])
    if rk2 == [10, 20, 30]:
        passed = passed + 1

    freq: list[int] = frequency_analysis([1, 2, 1, 3, 1])
    if freq[1] == 3:
        passed = passed + 1

    return passed
