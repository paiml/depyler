from typing import List, Tuple

def make_grid(key_vals: List[int]) -> List[List[int]]:
    used: List[int] = [0] * 26
    flat: List[int] = []
    for k in key_vals:
        val: int = k % 26
        if val == 9:
            val = 8
        if used[val] == 0:
            flat.append(val)
            used[val] = 1
    for i in range(26):
        if i == 9:
            continue
        if used[i] == 0:
            flat.append(i)
    grid: List[List[int]] = []
    for r in range(5):
        row: List[int] = []
        for c in range(5):
            row.append(flat[r * 5 + c])
        grid.append(row)
    return grid

def find_pos(grid: List[List[int]], val: int) -> Tuple[int, int]:
    for r in range(5):
        for c in range(5):
            if grid[r][c] == val:
                return (r, c)
    return (0, 0)

def enc_pair(grid: List[List[int]], a: int, b: int) -> Tuple[int, int]:
    pa: Tuple[int, int] = find_pos(grid, a)
    pb: Tuple[int, int] = find_pos(grid, b)
    if pa[0] == pb[0]:
        return (grid[pa[0]][(pa[1] + 1) % 5], grid[pb[0]][(pb[1] + 1) % 5])
    elif pa[1] == pb[1]:
        return (grid[(pa[0] + 1) % 5][pa[1]], grid[(pb[0] + 1) % 5][pb[1]])
    else:
        return (grid[pa[0]][pb[1]], grid[pb[0]][pa[1]])

def dec_pair(grid: List[List[int]], a: int, b: int) -> Tuple[int, int]:
    pa: Tuple[int, int] = find_pos(grid, a)
    pb: Tuple[int, int] = find_pos(grid, b)
    if pa[0] == pb[0]:
        return (grid[pa[0]][(pa[1] + 4) % 5], grid[pb[0]][(pb[1] + 4) % 5])
    elif pa[1] == pb[1]:
        return (grid[(pa[0] + 4) % 5][pa[1]], grid[(pb[0] + 4) % 5][pb[1]])
    else:
        return (grid[pa[0]][pb[1]], grid[pb[0]][pa[1]])

def playfair_encrypt(grid: List[List[int]], text: List[int]) -> List[int]:
    cleaned: List[int] = []
    for b in text:
        val: int = b % 26
        if val == 9:
            val = 8
        cleaned.append(val)
    result: List[int] = []
    i: int = 0
    while i < len(cleaned):
        a: int = cleaned[i]
        b: int = 23
        if i + 1 < len(cleaned):
            b = cleaned[i + 1]
            i = i + 2
        else:
            i = i + 1
        ep: Tuple[int, int] = enc_pair(grid, a, b)
        result.append(ep[0])
        result.append(ep[1])
    return result
