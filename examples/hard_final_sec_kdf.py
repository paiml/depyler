"""Key Derivation Function (KDF) using iterated hashing.

Implements PBKDF-like key stretching: repeatedly hash password with salt
to derive a cryptographic key. Uses integer operations throughout.
"""


def hash_word(state: int, word: int) -> int:
    """Mix one word into hash state."""
    state = (state * 37 + word + 1) % 65536
    rotated: int = ((state * 8) % 65536) + (state // 8192)
    return (state + rotated) % 65536


def hash_list(data: list[int], seed: int) -> int:
    """Hash an integer list with a seed."""
    state: int = seed
    i: int = 0
    while i < len(data):
        dv: int = data[i]
        state = hash_word(state, dv)
        i = i + 1
    return state


def pbkdf_derive(password: list[int], salt: list[int], iterations: int) -> int:
    """Derive key from password and salt with iteration count."""
    combined: list[int] = []
    i: int = 0
    while i < len(password):
        pv: int = password[i]
        combined.append(pv)
        i = i + 1
    j: int = 0
    while j < len(salt):
        sv: int = salt[j]
        combined.append(sv)
        j = j + 1
    derived: int = hash_list(combined, 0)
    r: int = 1
    while r < iterations:
        derived = hash_word(derived, r)
        r = r + 1
    return derived


def derive_multiple_keys(password: list[int], salt: list[int], iterations: int, num_keys: int) -> list[int]:
    """Derive multiple keys by varying the salt."""
    keys: list[int] = []
    ki: int = 0
    while ki < num_keys:
        augmented_salt: list[int] = []
        si: int = 0
        while si < len(salt):
            sv: int = salt[si]
            augmented_salt.append(sv)
            si = si + 1
        augmented_salt.append(ki)
        derived: int = pbkdf_derive(password, augmented_salt, iterations)
        keys.append(derived)
        ki = ki + 1
    return keys


def verify_password(password: list[int], salt: list[int], iterations: int, stored_hash: int) -> int:
    """Verify password against stored hash. Returns 1 if match."""
    derived: int = pbkdf_derive(password, salt, iterations)
    if derived == stored_hash:
        return 1
    return 0


def key_strength(derived: int) -> int:
    """Estimate key strength by counting set bits."""
    bits: int = 0
    val: int = derived
    step: int = 0
    while step < 16:
        if val % 2 == 1:
            bits = bits + 1
        val = val // 2
        step = step + 1
    return bits


def test_module() -> int:
    """Test KDF implementation."""
    ok: int = 0
    pw: list[int] = [112, 97, 115, 115]
    salt: list[int] = [1, 2, 3, 4]
    k1: int = pbkdf_derive(pw, salt, 100)
    k2: int = pbkdf_derive(pw, salt, 100)
    if k1 == k2:
        ok = ok + 1
    diff_salt: list[int] = [5, 6, 7, 8]
    k3: int = pbkdf_derive(pw, diff_salt, 100)
    if k3 != k1:
        ok = ok + 1
    if verify_password(pw, salt, 100, k1) == 1:
        ok = ok + 1
    keys: list[int] = derive_multiple_keys(pw, salt, 10, 3)
    if len(keys) == 3:
        ok = ok + 1
    strength: int = key_strength(k1)
    if strength > 0:
        if strength <= 16:
            ok = ok + 1
    return ok
