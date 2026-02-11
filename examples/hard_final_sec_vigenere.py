"""Vigenere cipher with repeating keyword encryption.

Operates on integer arrays (0-25). Key repeats cyclically over plaintext.
"""


def vigenere_encrypt(plaintext: list[int], keyword: list[int]) -> list[int]:
    """Encrypt using Vigenere cipher."""
    result: list[int] = []
    klen: int = len(keyword)
    i: int = 0
    while i < len(plaintext):
        pv: int = plaintext[i]
        kv: int = keyword[i % klen]
        encrypted: int = (pv + kv) % 26
        result.append(encrypted)
        i = i + 1
    return result


def vigenere_decrypt(ciphertext: list[int], keyword: list[int]) -> list[int]:
    """Decrypt using Vigenere cipher."""
    result: list[int] = []
    klen: int = len(keyword)
    i: int = 0
    while i < len(ciphertext):
        cv: int = ciphertext[i]
        kv: int = keyword[i % klen]
        decrypted: int = (cv + 26 - kv) % 26
        result.append(decrypted)
        i = i + 1
    return result


def lists_eq(a: list[int], b: list[int]) -> int:
    """Check equality."""
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


def index_of_coincidence(text: list[int]) -> int:
    """Index of coincidence * 10000 for text (higher = more English-like)."""
    freqs: list[int] = []
    j: int = 0
    while j < 26:
        freqs.append(0)
        j = j + 1
    n: int = len(text)
    i: int = 0
    while i < n:
        tv: int = text[i]
        old: int = freqs[tv]
        freqs[tv] = old + 1
        i = i + 1
    if n <= 1:
        return 0
    numerator: int = 0
    k: int = 0
    while k < 26:
        fk: int = freqs[k]
        numerator = numerator + fk * (fk - 1)
        k = k + 1
    return numerator * 10000 // (n * (n - 1))


def estimate_keylength(ciphertext: list[int], max_len: int) -> int:
    """Estimate key length using IoC. Returns best key length 1..max_len."""
    best_len: int = 1
    best_ioc: int = 0
    kl: int = 1
    while kl <= max_len:
        total_ioc: int = 0
        col: int = 0
        while col < kl:
            subset: list[int] = []
            idx: int = col
            while idx < len(ciphertext):
                sv: int = ciphertext[idx]
                subset.append(sv)
                idx = idx + kl
            ioc: int = index_of_coincidence(subset)
            total_ioc = total_ioc + ioc
            col = col + 1
        avg_ioc: int = total_ioc // kl
        if avg_ioc > best_ioc:
            best_ioc = avg_ioc
            best_len = kl
        kl = kl + 1
    return best_len


def test_module() -> int:
    """Test Vigenere cipher."""
    ok: int = 0
    plain: list[int] = [7, 4, 11, 11, 14]
    kw: list[int] = [10, 4, 24]
    enc: list[int] = vigenere_encrypt(plain, kw)
    dec: list[int] = vigenere_decrypt(enc, kw)
    if lists_eq(dec, plain) == 1:
        ok = ok + 1
    if len(enc) == 5:
        ok = ok + 1
    v0: int = enc[0]
    if v0 == (7 + 10) % 26:
        ok = ok + 1
    ioc_rand: int = index_of_coincidence([0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
    if ioc_rand == 0:
        ok = ok + 1
    ioc_rep: int = index_of_coincidence([0, 0, 0, 0, 0, 1, 1, 1, 1, 1])
    if ioc_rep > 0:
        ok = ok + 1
    return ok
