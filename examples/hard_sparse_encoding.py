"""Sparse array encoding: compress/decompress sparse arrays using index-value pairs."""


def sparse_encode(arr: list[int]) -> list[int]:
    """Encode sparse array as [index, value, index, value, ...] skipping zeros."""
    result: list[int] = []
    i: int = 0
    while i < len(arr):
        if arr[i] != 0:
            result.append(i)
            result.append(arr[i])
        i = i + 1
    return result


def sparse_decode(encoded: list[int], length: int) -> list[int]:
    """Decode sparse-encoded array back to full array of given length."""
    result: list[int] = []
    i: int = 0
    while i < length:
        result.append(0)
        i = i + 1
    j: int = 0
    while j < len(encoded) - 1:
        idx: int = encoded[j]
        val: int = encoded[j + 1]
        if idx >= 0 and idx < length:
            result[idx] = val
        j = j + 2
    return result


def sparse_dot_product(enc_a: list[int], enc_b: list[int]) -> int:
    """Dot product of two sparse-encoded vectors."""
    result: int = 0
    i: int = 0
    while i < len(enc_a) - 1:
        idx_a: int = enc_a[i]
        val_a: int = enc_a[i + 1]
        j: int = 0
        while j < len(enc_b) - 1:
            idx_b: int = enc_b[j]
            if idx_b == idx_a:
                result = result + val_a * enc_b[j + 1]
                j = len(enc_b)
            j = j + 2
        i = i + 2
    return result


def sparse_add(enc_a: list[int], enc_b: list[int]) -> list[int]:
    """Add two sparse-encoded vectors, returning sparse result."""
    # Collect all indices and values
    indices: list[int] = []
    values: list[int] = []
    i: int = 0
    while i < len(enc_a) - 1:
        idx: int = enc_a[i]
        val: int = enc_a[i + 1]
        indices.append(idx)
        values.append(val)
        i = i + 2
    j: int = 0
    while j < len(enc_b) - 1:
        idx2: int = enc_b[j]
        val2: int = enc_b[j + 1]
        found: int = 0
        k: int = 0
        while k < len(indices):
            if indices[k] == idx2:
                values[k] = values[k] + val2
                found = 1
                k = len(indices)
            k = k + 1
        if found == 0:
            indices.append(idx2)
            values.append(val2)
        j = j + 2
    result: list[int] = []
    m: int = 0
    while m < len(indices):
        if values[m] != 0:
            result.append(indices[m])
            result.append(values[m])
        m = m + 1
    return result


def test_module() -> int:
    """Test sparse encoding operations."""
    ok: int = 0

    arr: list[int] = [0, 5, 0, 0, 3, 0, 7]
    enc: list[int] = sparse_encode(arr)
    if len(enc) == 6 and enc[0] == 1 and enc[1] == 5:
        ok = ok + 1

    dec: list[int] = sparse_decode(enc, 7)
    if dec[1] == 5 and dec[4] == 3 and dec[6] == 7 and dec[0] == 0:
        ok = ok + 1

    enc_a: list[int] = [1, 3, 3, 5]
    enc_b: list[int] = [1, 2, 3, 4]
    if sparse_dot_product(enc_a, enc_b) == 26:
        ok = ok + 1

    added: list[int] = sparse_add(enc_a, enc_b)
    if len(added) == 4:
        ok = ok + 1

    empty: list[int] = [0, 0, 0]
    enc_empty: list[int] = sparse_encode(empty)
    if len(enc_empty) == 0:
        ok = ok + 1

    if sparse_dot_product(enc_empty, enc_a) == 0:
        ok = ok + 1

    return ok
