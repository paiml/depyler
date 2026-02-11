def zigzag_convert(s: str, num_rows: int) -> str:
    if num_rows <= 1:
        return s
    n: int = len(s)
    rows: list[str] = []
    i: int = 0
    while i < num_rows:
        rows.append("")
        i = i + 1
    cur_row: int = 0
    going_down: int = 0
    j: int = 0
    while j < n:
        ch: str = s[j]
        rows[cur_row] = rows[cur_row] + ch
        if cur_row == 0:
            going_down = 1
        if cur_row == num_rows - 1:
            going_down = 0
        if going_down == 1:
            cur_row = cur_row + 1
        else:
            cur_row = cur_row - 1
        j = j + 1
    result: str = ""
    k: int = 0
    while k < num_rows:
        result = result + rows[k]
        k = k + 1
    return result

def zigzag_decode(s: str, num_rows: int) -> str:
    if num_rows <= 1:
        return s
    n: int = len(s)
    cycle: int = 2 * (num_rows - 1)
    result: list[str] = []
    idx: int = 0
    while idx < n:
        result.append("")
        idx = idx + 1
    positions: list[int] = []
    row: int = 0
    while row < num_rows:
        col: int = 0
        while col < n:
            first: int = col + row
            if first < n:
                positions.append(first)
            if row != 0 and row != num_rows - 1:
                second: int = col + cycle - row
                if second < n:
                    positions.append(second)
            col = col + cycle
        row = row + 1
    m: int = 0
    while m < n:
        pos: int = positions[m]
        result[pos] = s[m]
        m = m + 1
    out: str = ""
    p: int = 0
    while p < n:
        out = out + result[p]
        p = p + 1
    return out

def test_module() -> int:
    passed: int = 0
    r1: str = zigzag_convert("PAYPALISHIRING", 3)
    if r1 == "PAHNAPLSIIGYIR":
        passed = passed + 1
    r2: str = zigzag_convert("PAYPALISHIRING", 4)
    if r2 == "PINALSIGYAHRPI":
        passed = passed + 1
    r3: str = zigzag_convert("AB", 1)
    if r3 == "AB":
        passed = passed + 1
    r4: str = zigzag_decode("PAHNAPLSIIGYIR", 3)
    if r4 == "PAYPALISHIRING":
        passed = passed + 1
    r5: str = zigzag_convert("A", 1)
    if r5 == "A":
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
