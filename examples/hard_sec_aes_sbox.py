from typing import List, Tuple

def gf_multiply(a: int, b: int) -> int:
    """Multiply two numbers in GF(2^8) with irreducible polynomial x^8+x^4+x^3+x+1."""
    result: int = 0
    for i in range(8):
        if (b & 1) != 0:
            result = result ^ a
        hi_bit: int = a & 0x80
        a = (a << 1) & 0xFF
        if hi_bit != 0:
            a = a ^ 0x1B
        b = b >> 1
    return result

def gf_inverse(a: int) -> int:
    """Compute multiplicative inverse in GF(2^8) using extended Euclidean."""
    if a == 0:
        return 0
    power: int = a
    for i in range(253):
        power = gf_multiply(power, a)
    return power

def affine_transform(byte: int) -> int:
    """Apply the AES affine transformation."""
    result: int = 0
    b: int = byte
    for i in range(8):
        bit: int = 0
        temp: int = b
        for j in range(8):
            bit = bit ^ (temp & 1)
            temp = temp >> 1
        result = result | ((bit & 1) << i)
        b = ((b >> 1) | ((b & 1) << 7)) & 0xFF
    result = result ^ 0x63
    return result & 0xFF

def generate_sbox() -> List[int]:
    """Generate the full AES S-box lookup table."""
    sbox: List[int] = []
    for i in range(256):
        inv: int = gf_inverse(i)
        val: int = affine_transform(inv)
        sbox.append(val)
    return sbox

def generate_inverse_sbox(sbox: List[int]) -> List[int]:
    """Generate the inverse S-box from the S-box."""
    inv_sbox: List[int] = [0] * 256
    for i in range(256):
        inv_sbox[sbox[i]] = i
    return inv_sbox

def sub_bytes(state: List[int], sbox: List[int]) -> List[int]:
    """Apply SubBytes transformation to AES state."""
    result: List[int] = []
    for i in range(len(state)):
        result.append(sbox[state[i] & 0xFF])
    return result

def verify_sbox_properties(sbox: List[int]) -> bool:
    """Verify that the S-box has expected cryptographic properties."""
    if len(sbox) != 256:
        return False
    seen: List[int] = [0] * 256
    for i in range(256):
        val: int = sbox[i]
        if val < 0 or val > 255:
            return False
        seen[val] = seen[val] + 1
    for i in range(256):
        if seen[i] != 1:
            return False
    return True

def nonlinearity_score(sbox: List[int]) -> int:
    """Compute a simple nonlinearity metric for the S-box."""
    score: int = 0
    for i in range(256):
        for bit in range(8):
            input_bit: int = (i >> bit) & 1
            output_bit: int = (sbox[i] >> bit) & 1
            if input_bit != output_bit:
                score = score + 1
    return score
