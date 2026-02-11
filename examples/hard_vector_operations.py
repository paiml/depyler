def vec_add(a: list[int], b: list[int]) -> list[int]:
    result: list[int] = []
    i: int = 0
    while i < len(a):
        result.append(a[i] + b[i])
        i = i + 1
    return result


def vec_subtract(a: list[int], b: list[int]) -> list[int]:
    result: list[int] = []
    i: int = 0
    while i < len(a):
        result.append(a[i] - b[i])
        i = i + 1
    return result


def vec_dot(a: list[int], b: list[int]) -> int:
    result: int = 0
    i: int = 0
    while i < len(a):
        result = result + a[i] * b[i]
        i = i + 1
    return result


def vec_cross_3d(a: list[int], b: list[int]) -> list[int]:
    x: int = a[1] * b[2] - a[2] * b[1]
    y: int = a[2] * b[0] - a[0] * b[2]
    z: int = a[0] * b[1] - a[1] * b[0]
    return [x, y, z]


def vec_magnitude_squared(a: list[int]) -> int:
    result: int = 0
    i: int = 0
    while i < len(a):
        result = result + a[i] * a[i]
        i = i + 1
    return result


def vec_scale(a: list[int], s: int) -> list[int]:
    result: list[int] = []
    i: int = 0
    while i < len(a):
        result.append(a[i] * s)
        i = i + 1
    return result


def test_module() -> int:
    passed: int = 0
    if vec_add([1, 2, 3], [4, 5, 6]) == [5, 7, 9]:
        passed = passed + 1
    if vec_subtract([5, 7, 9], [4, 5, 6]) == [1, 2, 3]:
        passed = passed + 1
    if vec_dot([1, 2, 3], [4, 5, 6]) == 32:
        passed = passed + 1
    if vec_cross_3d([1, 0, 0], [0, 1, 0]) == [0, 0, 1]:
        passed = passed + 1
    if vec_cross_3d([0, 1, 0], [1, 0, 0]) == [0, 0, -1]:
        passed = passed + 1
    if vec_magnitude_squared([3, 4]) == 25:
        passed = passed + 1
    if vec_scale([1, 2, 3], 3) == [3, 6, 9]:
        passed = passed + 1
    return passed
