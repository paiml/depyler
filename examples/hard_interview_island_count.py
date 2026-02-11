def flood_fill(grid: list[list[int]], r: int, c: int, rows: int, cols: int) -> int:
    if r < 0 or r >= rows or c < 0 or c >= cols:
        return 0
    row: list[int] = grid[r]
    if row[c] != 1:
        return 0
    row[c] = 0
    flood_fill(grid, r - 1, c, rows, cols)
    flood_fill(grid, r + 1, c, rows, cols)
    flood_fill(grid, r, c - 1, rows, cols)
    flood_fill(grid, r, c + 1, rows, cols)
    return 1

def count_islands(grid: list[list[int]], rows: int, cols: int) -> int:
    count: int = 0
    r: int = 0
    while r < rows:
        c: int = 0
        while c < cols:
            row: list[int] = grid[r]
            if row[c] == 1:
                flood_fill(grid, r, c, rows, cols)
                count = count + 1
            c = c + 1
        r = r + 1
    return count

def island_perimeter(grid: list[list[int]], rows: int, cols: int) -> int:
    perim: int = 0
    r: int = 0
    while r < rows:
        c: int = 0
        while c < cols:
            row: list[int] = grid[r]
            if row[c] == 1:
                perim = perim + 4
                if r > 0:
                    above: list[int] = grid[r - 1]
                    if above[c] == 1:
                        perim = perim - 2
                if c > 0:
                    if row[c - 1] == 1:
                        perim = perim - 2
            c = c + 1
        r = r + 1
    return perim

def max_island_size(grid: list[list[int]], rows: int, cols: int) -> int:
    best: int = 0
    r: int = 0
    while r < rows:
        c: int = 0
        while c < cols:
            row: list[int] = grid[r]
            if row[c] == 1:
                sz: int = measure_island(grid, r, c, rows, cols)
                if sz > best:
                    best = sz
            c = c + 1
        r = r + 1
    return best

def measure_island(grid: list[list[int]], r: int, c: int, rows: int, cols: int) -> int:
    if r < 0 or r >= rows or c < 0 or c >= cols:
        return 0
    row: list[int] = grid[r]
    if row[c] != 1:
        return 0
    row[c] = 0
    sz: int = 1
    sz = sz + measure_island(grid, r - 1, c, rows, cols)
    sz = sz + measure_island(grid, r + 1, c, rows, cols)
    sz = sz + measure_island(grid, r, c - 1, rows, cols)
    sz = sz + measure_island(grid, r, c + 1, rows, cols)
    return sz

def test_module() -> int:
    passed: int = 0
    g1: list[list[int]] = [[1, 1, 0, 0], [1, 0, 0, 1], [0, 0, 1, 1]]
    r1: int = count_islands(g1, 3, 4)
    if r1 == 2:
        passed = passed + 1
    g2: list[list[int]] = [[0, 0, 0], [0, 0, 0]]
    r2: int = count_islands(g2, 2, 3)
    if r2 == 0:
        passed = passed + 1
    g3: list[list[int]] = [[0, 1, 0, 0], [1, 1, 1, 0], [0, 1, 0, 0], [1, 1, 0, 0]]
    r3: int = island_perimeter(g3, 4, 4)
    if r3 == 16:
        passed = passed + 1
    g4: list[list[int]] = [[1, 0, 0], [0, 1, 1], [0, 1, 1]]
    r4: int = max_island_size(g4, 3, 3)
    if r4 == 4:
        passed = passed + 1
    g5: list[list[int]] = [[1, 1, 1], [1, 1, 1], [1, 1, 1]]
    r5: int = count_islands(g5, 3, 3)
    if r5 == 1:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
