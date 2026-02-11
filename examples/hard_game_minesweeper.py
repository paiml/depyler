def make_field(rows: int, cols: int) -> list[list[int]]:
    field: list[list[int]] = []
    r: int = 0
    while r < rows:
        row: list[int] = []
        c: int = 0
        while c < cols:
            row.append(0)
            c = c + 1
        field.append(row)
        r = r + 1
    return field

def place_mine(field: list[list[int]], r: int, c: int) -> int:
    row: list[int] = field[r]
    v: int = row[c]
    if v == 1:
        return 0
    row[c] = 1
    return 1

def count_adjacent(field: list[list[int]], r: int, c: int, rows: int, cols: int) -> int:
    count: int = 0
    dr: int = 0 - 1
    while dr <= 1:
        dc: int = 0 - 1
        while dc <= 1:
            if dr != 0 or dc != 0:
                nr: int = r + dr
                nc: int = c + dc
                if nr >= 0 and nr < rows and nc >= 0 and nc < cols:
                    row: list[int] = field[nr]
                    v: int = row[nc]
                    if v == 1:
                        count = count + 1
            dc = dc + 1
        dr = dr + 1
    return count

def compute_numbers(field: list[list[int]], rows: int, cols: int) -> list[list[int]]:
    numbers: list[list[int]] = make_field(rows, cols)
    r: int = 0
    while r < rows:
        c: int = 0
        while c < cols:
            mine_row: list[int] = field[r]
            mine_val: int = mine_row[c]
            if mine_val == 1:
                num_row: list[int] = numbers[r]
                num_row[c] = 0 - 1
            else:
                adj: int = count_adjacent(field, r, c, rows, cols)
                num_row2: list[int] = numbers[r]
                num_row2[c] = adj
            c = c + 1
        r = r + 1
    return numbers

def count_mines(field: list[list[int]], rows: int, cols: int) -> int:
    count: int = 0
    r: int = 0
    while r < rows:
        c: int = 0
        while c < cols:
            row: list[int] = field[r]
            v: int = row[c]
            if v == 1:
                count = count + 1
            c = c + 1
        r = r + 1
    return count

def test_module() -> int:
    passed: int = 0
    f: list[list[int]] = make_field(3, 3)
    nf: int = len(f)
    if nf == 3:
        passed = passed + 1
    r: int = place_mine(f, 0, 0)
    if r == 1:
        passed = passed + 1
    r2: int = place_mine(f, 0, 0)
    if r2 == 0:
        passed = passed + 1
    adj: int = count_adjacent(f, 1, 1, 3, 3)
    if adj == 1:
        passed = passed + 1
    mc: int = count_mines(f, 3, 3)
    if mc == 1:
        passed = passed + 1
    return passed
