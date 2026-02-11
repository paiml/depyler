"""Parity checks, Gray code conversion, Hamming distance."""


def parity(n: int) -> int:
    """Return 0 if even number of set bits, 1 if odd."""
    val: int = n
    p: int = 0
    while val > 0:
        p = p ^ (val & 1)
        val = val >> 1
    return p


def to_gray(n: int) -> int:
    """Convert binary number to Gray code."""
    return n ^ (n >> 1)


def from_gray(gray: int) -> int:
    """Convert Gray code back to binary."""
    result: int = gray
    shift: int = 1
    while (gray >> shift) > 0:
        result = result ^ (gray >> shift)
        shift = shift + 1
    return result


def hamming_distance(a: int, b: int) -> int:
    """Count the number of differing bits between a and b."""
    diff: int = a ^ b
    count: int = 0
    while diff > 0:
        count = count + (diff & 1)
        diff = diff >> 1
    return count


def gray_code_sequence(bits: int) -> list[int]:
    """Generate the Gray code sequence of given bit width."""
    total: int = 1 << bits
    result: list[int] = []
    idx: int = 0
    while idx < total:
        result.append(to_gray(idx))
        idx = idx + 1
    return result


def test_module() -> int:
    passed: int = 0

    if parity(7) == 1:
        passed = passed + 1
    if parity(3) == 0:
        passed = passed + 1
    if to_gray(5) == 7:
        passed = passed + 1
    if from_gray(7) == 5:
        passed = passed + 1
    if hamming_distance(1, 4) == 3:
        passed = passed + 1
    seq: list[int] = gray_code_sequence(2)
    if seq[0] == 0:
        passed = passed + 1
    if seq[1] == 1:
        passed = passed + 1
    if len(seq) == 4:
        passed = passed + 1

    return passed
