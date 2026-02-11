def egg_drop(eggs: int, floors: int) -> int:
    if floors <= 1:
        return floors
    if eggs == 1:
        return floors
    dp: list[list[int]] = []
    e: int = 0
    while e <= eggs:
        row: list[int] = []
        f: int = 0
        while f <= floors:
            row.append(0)
            f = f + 1
        dp.append(row)
        e = e + 1
    f2: int = 1
    while f2 <= floors:
        row1: list[int] = dp[1]
        row1[f2] = f2
        f2 = f2 + 1
    ei: int = 2
    while ei <= eggs:
        row_e: list[int] = dp[ei]
        row_e[1] = 1
        fi: int = 2
        while fi <= floors:
            row_e[fi] = fi
            x: int = 1
            while x <= fi:
                row_prev: list[int] = dp[ei - 1]
                breaks_val: int = row_prev[x - 1]
                survives_val: int = row_e[fi - x]
                worst: int = breaks_val
                if survives_val > worst:
                    worst = survives_val
                candidate: int = worst + 1
                if candidate < row_e[fi]:
                    row_e[fi] = candidate
                x = x + 1
            fi = fi + 1
        ei = ei + 1
    final_row: list[int] = dp[eggs]
    return final_row[floors]

def egg_drop_recursive(eggs: int, floors: int) -> int:
    if floors <= 1:
        return floors
    if eggs == 1:
        return floors
    best: int = floors
    x: int = 1
    while x <= floors:
        breaks_case: int = egg_drop_recursive(eggs - 1, x - 1)
        survives_case: int = egg_drop_recursive(eggs, floors - x)
        worst: int = breaks_case
        if survives_case > worst:
            worst = survives_case
        candidate: int = worst + 1
        if candidate < best:
            best = candidate
        x = x + 1
    return best

def egg_drop_two_eggs(floors: int) -> int:
    t: int = 1
    while t * (t + 1) // 2 < floors:
        t = t + 1
    return t

def test_module() -> int:
    passed: int = 0
    r1: int = egg_drop(2, 10)
    if r1 == 4:
        passed = passed + 1
    r2: int = egg_drop(1, 10)
    if r2 == 10:
        passed = passed + 1
    r3: int = egg_drop(2, 6)
    if r3 == 3:
        passed = passed + 1
    r4: int = egg_drop(3, 14)
    if r4 == 4:
        passed = passed + 1
    r5: int = egg_drop_two_eggs(100)
    if r5 == 14:
        passed = passed + 1
    r6: int = egg_drop_recursive(2, 6)
    if r6 == 3:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
