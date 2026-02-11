def bwt_transform(s: str) -> str:
    n: int = len(s)
    indices: list[int] = []
    i: int = 0
    while i < n:
        indices.append(i)
        i = i + 1
    i = 0
    while i < n:
        j: int = i + 1
        while j < n:
            if rotation_less(s, indices[j], indices[i]) == 1:
                tmp: int = indices[i]
                indices[i] = indices[j]
                indices[j] = tmp
            j = j + 1
        i = i + 1
    result: str = ""
    k: int = 0
    while k < n:
        pos: int = (indices[k] + n - 1) % n
        result = result + s[pos]
        k = k + 1
    return result

def rotation_less(s: str, a: int, b: int) -> int:
    n: int = len(s)
    k: int = 0
    while k < n:
        ca: int = (a + k) % n
        cb: int = (b + k) % n
        if s[ca] < s[cb]:
            return 1
        if s[ca] > s[cb]:
            return 0
        k = k + 1
    return 0

def bwt_original_index(s: str) -> int:
    n: int = len(s)
    indices: list[int] = []
    i: int = 0
    while i < n:
        indices.append(i)
        i = i + 1
    i = 0
    while i < n:
        j: int = i + 1
        while j < n:
            if rotation_less(s, indices[j], indices[i]) == 1:
                tmp: int = indices[i]
                indices[i] = indices[j]
                indices[j] = tmp
            j = j + 1
        i = i + 1
    k: int = 0
    while k < n:
        if indices[k] == 0:
            return k
        k = k + 1
    return -1

def count_runs(s: str) -> int:
    if len(s) == 0:
        return 0
    runs: int = 1
    i: int = 1
    while i < len(s):
        im: int = i - 1
        if s[i] != s[im]:
            runs = runs + 1
        i = i + 1
    return runs

def test_module() -> int:
    passed: int = 0
    bwt: str = bwt_transform("banana")
    if bwt == "nnbaaa":
        passed = passed + 1
    idx: int = bwt_original_index("banana")
    if idx == 3:
        passed = passed + 1
    if count_runs("nnbaaa") == 3:
        passed = passed + 1
    bwt2: str = bwt_transform("abcd")
    if len(bwt2) == 4:
        passed = passed + 1
    if count_runs("") == 0:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
