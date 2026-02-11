"""Caesar cipher encryption and decryption.

Operates on integer arrays representing character codes (0-25 for a-z).
Shift-based substitution cipher with brute-force cracking.
"""


def caesar_encrypt(plaintext: list[int], shift: int) -> list[int]:
    """Encrypt by shifting each value by shift mod 26."""
    result: list[int] = []
    i: int = 0
    while i < len(plaintext):
        pv: int = plaintext[i]
        encrypted: int = (pv + shift) % 26
        result.append(encrypted)
        i = i + 1
    return result


def caesar_decrypt(ciphertext: list[int], shift: int) -> list[int]:
    """Decrypt by shifting back."""
    result: list[int] = []
    i: int = 0
    while i < len(ciphertext):
        cv: int = ciphertext[i]
        decrypted: int = (cv + 26 - shift) % 26
        result.append(decrypted)
        i = i + 1
    return result


def lists_equal(a: list[int], b: list[int]) -> int:
    """Check if two lists are equal. Returns 1 if equal."""
    if len(a) != len(b):
        return 0
    i: int = 0
    while i < len(a):
        va: int = a[i]
        vb: int = b[i]
        if va != vb:
            return 0
        i = i + 1
    return 1


def frequency_score(text: list[int]) -> int:
    """Score text by English-like frequency distribution (higher = more English-like).

    Uses simplified frequency: e(4)=100, t(19)=90, a(0)=80, o(14)=70, etc.
    """
    freqs: list[int] = []
    j: int = 0
    while j < 26:
        freqs.append(0)
        j = j + 1
    i: int = 0
    while i < len(text):
        tv: int = text[i]
        old: int = freqs[tv]
        freqs[tv] = old + 1
        i = i + 1
    score: int = 0
    e_freq: int = freqs[4]
    t_freq: int = freqs[19]
    a_freq: int = freqs[0]
    score = e_freq * 100 + t_freq * 90 + a_freq * 80
    return score


def brute_force_caesar(ciphertext: list[int]) -> int:
    """Try all 26 shifts, return the one with best frequency score."""
    best_shift: int = 0
    best_score: int = 0 - 1
    shift: int = 0
    while shift < 26:
        decrypted: list[int] = caesar_decrypt(ciphertext, shift)
        sc: int = frequency_score(decrypted)
        if sc > best_score:
            best_score = sc
            best_shift = shift
        shift = shift + 1
    return best_shift


def caesar_double_encrypt(plaintext: list[int], s1: int, s2: int) -> list[int]:
    """Double encryption with two shifts."""
    first_pass: list[int] = caesar_encrypt(plaintext, s1)
    return caesar_encrypt(first_pass, s2)


def test_module() -> int:
    """Test Caesar cipher."""
    ok: int = 0
    plain: list[int] = [0, 1, 2, 3, 4]
    enc: list[int] = caesar_encrypt(plain, 3)
    dec: list[int] = caesar_decrypt(enc, 3)
    if lists_equal(dec, plain) == 1:
        ok = ok + 1
    v0: int = enc[0]
    if v0 == 3:
        ok = ok + 1
    enc25: list[int] = caesar_encrypt([25], 1)
    w0: int = enc25[0]
    if w0 == 0:
        ok = ok + 1
    dbl: list[int] = caesar_double_encrypt(plain, 3, 5)
    single: list[int] = caesar_encrypt(plain, 8)
    if lists_equal(dbl, single) == 1:
        ok = ok + 1
    dec_dbl: list[int] = caesar_decrypt(caesar_decrypt(dbl, 5), 3)
    if lists_equal(dec_dbl, plain) == 1:
        ok = ok + 1
    return ok
