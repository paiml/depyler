"""Sequence generation: Stern-Brocot, Sylvester, and other integer sequences."""


def stern_brocot(n: int) -> list[int]:
    """Generate first n terms of the Stern-Brocot sequence."""
    if n <= 0:
        result: list[int] = []
        return result
    if n == 1:
        result2: list[int] = [1]
        return result2
    seq: list[int] = [1, 1]
    while len(seq) < n:
        sz: int = len(seq)
        idx: int = sz // 2
        val: int = seq[idx]
        prev_idx: int = idx - 1
        if sz % 2 == 0:
            seq.append(val + seq[prev_idx])
        else:
            seq.append(val)
    result3: list[int] = seq
    return result3


def sylvester_sequence(n: int) -> list[int]:
    """Generate first n terms of Sylvester's sequence (starts at 2)."""
    result: list[int] = []
    if n <= 0:
        return result
    result.append(2)
    i: int = 1
    while i < n:
        product: int = 1
        j: int = 0
        while j < len(result):
            product = product * result[j]
            j = j + 1
        result.append(product + 1)
        i = i + 1
    return result


def recaman_sequence(n: int) -> list[int]:
    """Generate first n terms of the Recaman sequence."""
    result: list[int] = []
    if n <= 0:
        return result
    result.append(0)
    seen: list[int] = [0]
    i: int = 1
    while i < n:
        last_idx: int = len(result) - 1
        prev: int = result[last_idx]
        candidate: int = prev - i
        found: int = 0
        if candidate > 0:
            j: int = 0
            while j < len(seen):
                if seen[j] == candidate:
                    found = 1
                j = j + 1
        else:
            found = 1
        if found == 0:
            result.append(candidate)
            seen.append(candidate)
        else:
            val: int = prev + i
            result.append(val)
            seen.append(val)
        i = i + 1
    return result


def juggler_sequence(n: int) -> list[int]:
    """Generate juggler sequence starting from n until it reaches 1."""
    result: list[int] = [n]
    current: int = n
    steps: int = 0
    while current != 1 and steps < 100:
        if current % 2 == 0:
            # Integer square root for even
            root: int = 1
            while (root + 1) * (root + 1) <= current:
                root = root + 1
            current = root
        else:
            # n^(3/2) = n * sqrt(n)
            root2: int = 1
            while (root2 + 1) * (root2 + 1) <= current:
                root2 = root2 + 1
            current = current * root2
        result.append(current)
        steps = steps + 1
    return result


def test_module() -> int:
    """Test sequence generation functions."""
    ok: int = 0

    sb: list[int] = stern_brocot(6)
    if len(sb) >= 6 and sb[0] == 1 and sb[1] == 1:
        ok = ok + 1

    syl: list[int] = sylvester_sequence(3)
    if syl[0] == 2 and syl[1] == 3 and syl[2] == 7:
        ok = ok + 1

    rec: list[int] = recaman_sequence(5)
    if rec[0] == 0 and rec[1] == 1 and rec[2] == 3 and rec[3] == 6 and rec[4] == 2:
        ok = ok + 1

    jug: list[int] = juggler_sequence(3)
    if jug[0] == 3 and jug[1] == 5:
        ok = ok + 1

    empty: list[int] = stern_brocot(0)
    if len(empty) == 0:
        ok = ok + 1

    syl1: list[int] = sylvester_sequence(1)
    if len(syl1) == 1 and syl1[0] == 2:
        ok = ok + 1

    return ok
