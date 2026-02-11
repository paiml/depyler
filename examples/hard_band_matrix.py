# Band matrix operations (tridiagonal)
# Stored as three diagonals: lower, main, upper


def band_create(n: int) -> list[list[int]]:
    lower: list[int] = []
    main: list[int] = []
    upper: list[int] = []
    i: int = 0
    while i < n:
        main.append(0)
        i = i + 1
    i = 0
    while i < n - 1:
        lower.append(0)
        upper.append(0)
        i = i + 1
    return [lower, main, upper]


def band_set_main(main: list[int], idx: int, val: int) -> list[int]:
    result: list[int] = []
    i: int = 0
    while i < len(main):
        if i == idx:
            result.append(val)
        else:
            result.append(main[i])
        i = i + 1
    return result


def band_set_lower(lower: list[int], idx: int, val: int) -> list[int]:
    result: list[int] = []
    i: int = 0
    while i < len(lower):
        if i == idx:
            result.append(val)
        else:
            result.append(lower[i])
        i = i + 1
    return result


def band_set_upper(upper: list[int], idx: int, val: int) -> list[int]:
    result: list[int] = []
    i: int = 0
    while i < len(upper):
        if i == idx:
            result.append(val)
        else:
            result.append(upper[i])
        i = i + 1
    return result


def band_get(lower: list[int], main: list[int], upper: list[int], row: int, col: int) -> int:
    if row == col:
        return main[row]
    if row == col + 1 and row - 1 < len(lower):
        return lower[row - 1]
    if col == row + 1 and row < len(upper):
        return upper[row]
    return 0


def band_matvec(lower: list[int], main: list[int], upper: list[int], vec: list[int]) -> list[int]:
    n: int = len(main)
    result: list[int] = []
    i: int = 0
    while i < n:
        val: int = main[i] * vec[i]
        if i > 0:
            val = val + lower[i - 1] * vec[i - 1]
        if i < n - 1:
            val = val + upper[i] * vec[i + 1]
        result.append(val)
        i = i + 1
    return result


def band_trace(main: list[int]) -> int:
    total: int = 0
    i: int = 0
    while i < len(main):
        total = total + main[i]
        i = i + 1
    return total


def test_module() -> int:
    passed: int = 0

    # Test 1: create 3x3 band matrix
    bands: list[list[int]] = band_create(3)
    if len(bands[1]) == 3:
        passed = passed + 1

    # Test 2: set main diagonal
    main: list[int] = band_set_main(bands[1], 0, 2)
    main = band_set_main(main, 1, 3)
    main = band_set_main(main, 2, 4)
    if band_trace(main) == 9:
        passed = passed + 1

    # Test 3: set sub and super diagonals
    lower: list[int] = band_set_lower(bands[0], 0, 1)
    lower = band_set_lower(lower, 1, 1)
    upper: list[int] = band_set_upper(bands[2], 0, 1)
    upper = band_set_upper(upper, 1, 1)
    if band_get(lower, main, upper, 1, 0) == 1:
        passed = passed + 1

    # Test 4: get diagonal element
    if band_get(lower, main, upper, 1, 1) == 3:
        passed = passed + 1

    # Test 5: get zero (out of band)
    if band_get(lower, main, upper, 0, 2) == 0:
        passed = passed + 1

    # Test 6: matvec [1,1,1]
    vec: list[int] = [1, 1, 1]
    result: list[int] = band_matvec(lower, main, upper, vec)
    if result[0] == 3 and result[1] == 5 and result[2] == 5:
        passed = passed + 1

    # Test 7: matvec [1,0,0]
    vec2: list[int] = [1, 0, 0]
    r2: list[int] = band_matvec(lower, main, upper, vec2)
    if r2[0] == 2 and r2[1] == 1 and r2[2] == 0:
        passed = passed + 1

    return passed
