"""Generate Gray codes for n bits."""


def gray_code(n: int) -> list[int]:
    """Generate Gray code sequence for n bits."""
    if n == 0:
        result: list[int] = [0]
        return result
    count: int = 1
    i: int = 0
    while i < n:
        count = count * 2
        i = i + 1
    codes: list[int] = []
    i = 0
    while i < count:
        code: int = i ^ (i // 2)
        codes.append(code)
        i = i + 1
    return codes


def gray_to_binary(gray: int) -> int:
    """Convert Gray code to binary."""
    mask: int = gray
    result: int = gray
    mask = mask // 2
    while mask > 0:
        result = result ^ mask
        mask = mask // 2
    return result


def binary_to_gray(num: int) -> int:
    """Convert binary to Gray code."""
    return num ^ (num // 2)


def count_bits_set(n: int) -> int:
    """Count number of 1-bits in integer."""
    count: int = 0
    val: int = n
    while val > 0:
        count = count + (val % 2)
        val = val // 2
    return count


def hamming_distance(a: int, b: int) -> int:
    """Hamming distance between two integers."""
    xor_val: int = a ^ b
    return count_bits_set(xor_val)


def test_module() -> int:
    """Test Gray code generation."""
    ok: int = 0
    g2: list[int] = gray_code(2)
    if len(g2) == 4:
        ok = ok + 1
    if g2[0] == 0:
        ok = ok + 1
    if g2[1] == 1:
        ok = ok + 1
    if g2[2] == 3:
        ok = ok + 1
    if g2[3] == 2:
        ok = ok + 1
    if gray_to_binary(3) == 2:
        ok = ok + 1
    if binary_to_gray(2) == 3:
        ok = ok + 1
    i: int = 0
    adjacent_ok: int = 1
    g3: list[int] = gray_code(3)
    while i < len(g3) - 1:
        if hamming_distance(g3[i], g3[i + 1]) != 1:
            adjacent_ok = 0
        i = i + 1
    if adjacent_ok == 1:
        ok = ok + 1
    return ok
