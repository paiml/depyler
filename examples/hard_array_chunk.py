def chunk_array(arr: list[int], size: int) -> list[list[int]]:
    result: list[list[int]] = []
    i: int = 0
    while i < len(arr):
        chunk: list[int] = []
        j: int = 0
        while j < size and i + j < len(arr):
            idx: int = i + j
            chunk.append(arr[idx])
            j = j + 1
        result.append(chunk)
        i = i + size
    return result

def count_chunks(arr: list[int], size: int) -> int:
    total: int = len(arr)
    if total == 0:
        return 0
    result: int = total // size
    if total % size != 0:
        result = result + 1
    return result

def last_chunk_size(arr: list[int], size: int) -> int:
    total: int = len(arr)
    if total == 0:
        return 0
    remainder: int = total % size
    if remainder == 0:
        return size
    return remainder

def flatten_chunks(chunks: list[list[int]]) -> list[int]:
    result: list[int] = []
    i: int = 0
    while i < len(chunks):
        c: list[int] = chunks[i]
        j: int = 0
        while j < len(c):
            result.append(c[j])
            j = j + 1
        i = i + 1
    return result

def test_module() -> int:
    passed: int = 0
    c1: list[list[int]] = chunk_array([1, 2, 3, 4, 5], 2)
    if len(c1) == 3:
        passed = passed + 1
    c1_0: list[int] = c1[0]
    if len(c1_0) == 2:
        passed = passed + 1
    if count_chunks([1, 2, 3, 4, 5], 2) == 3:
        passed = passed + 1
    if last_chunk_size([1, 2, 3, 4, 5], 2) == 1:
        passed = passed + 1
    flat: list[int] = flatten_chunks(chunk_array([10, 20, 30], 2))
    if len(flat) == 3 and flat[0] == 10:
        passed = passed + 1
    if count_chunks([], 3) == 0:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
