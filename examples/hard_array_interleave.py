def interleave(a: list[int], b: list[int]) -> list[int]:
    result: list[int] = []
    i: int = 0
    la: int = len(a)
    lb: int = len(b)
    while i < la or i < lb:
        if i < la:
            result.append(a[i])
        if i < lb:
            result.append(b[i])
        i = i + 1
    return result

def deinterleave_even(arr: list[int]) -> list[int]:
    result: list[int] = []
    i: int = 0
    while i < len(arr):
        if i % 2 == 0:
            result.append(arr[i])
        i = i + 1
    return result

def deinterleave_odd(arr: list[int]) -> list[int]:
    result: list[int] = []
    i: int = 0
    while i < len(arr):
        if i % 2 == 1:
            result.append(arr[i])
        i = i + 1
    return result

def zip_sum(a: list[int], b: list[int]) -> list[int]:
    result: list[int] = []
    sz: int = len(a)
    if len(b) < sz:
        sz = len(b)
    i: int = 0
    while i < sz:
        result.append(a[i] + b[i])
        i = i + 1
    return result

def test_module() -> int:
    passed: int = 0
    r1: list[int] = interleave([1, 3, 5], [2, 4, 6])
    if len(r1) == 6 and r1[0] == 1 and r1[1] == 2:
        passed = passed + 1
    r2: list[int] = interleave([1, 2], [10, 20, 30])
    if len(r2) == 5:
        passed = passed + 1
    ev: list[int] = deinterleave_even([10, 20, 30, 40])
    if len(ev) == 2 and ev[0] == 10:
        passed = passed + 1
    od: list[int] = deinterleave_odd([10, 20, 30, 40])
    if len(od) == 2 and od[0] == 20:
        passed = passed + 1
    zs: list[int] = zip_sum([1, 2, 3], [10, 20, 30])
    if zs[0] == 11 and zs[2] == 33:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
