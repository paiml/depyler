def rle_encode(data: list[int]) -> list[int]:
    if len(data) == 0:
        return []
    result: list[int] = []
    current: int = data[0]
    count: int = 1
    i: int = 1
    while i < len(data):
        if data[i] == current:
            count = count + 1
        else:
            result.append(current)
            result.append(count)
            current = data[i]
            count = 1
        i = i + 1
    result.append(current)
    result.append(count)
    return result


def rle_decode(encoded: list[int]) -> list[int]:
    result: list[int] = []
    i: int = 0
    while i < len(encoded):
        val: int = encoded[i]
        count: int = encoded[i + 1]
        j: int = 0
        while j < count:
            result.append(val)
            j = j + 1
        i = i + 2
    return result


def rle_compressed_size(data: list[int]) -> int:
    encoded: list[int] = rle_encode(data)
    return len(encoded)


def compression_ratio_x100(original_len: int, compressed_len: int) -> int:
    if original_len == 0:
        return 100
    return compressed_len * 100 // original_len


def test_module() -> int:
    passed: int = 0
    enc: list[int] = rle_encode([1, 1, 1, 2, 2, 3])
    if enc == [1, 3, 2, 2, 3, 1]:
        passed = passed + 1
    dec: list[int] = rle_decode([1, 3, 2, 2, 3, 1])
    if dec == [1, 1, 1, 2, 2, 3]:
        passed = passed + 1
    if rle_encode([]) == []:
        passed = passed + 1
    if rle_decode([]) == []:
        passed = passed + 1
    if rle_compressed_size([5, 5, 5, 5, 5]) == 2:
        passed = passed + 1
    rt: list[int] = rle_decode(rle_encode([7, 7, 8, 8, 8, 9]))
    if rt == [7, 7, 8, 8, 8, 9]:
        passed = passed + 1
    if compression_ratio_x100(10, 4) == 40:
        passed = passed + 1
    return passed
