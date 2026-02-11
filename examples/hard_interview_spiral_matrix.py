def spiral_order(matrix: list[list[int]], rows: int, cols: int) -> list[int]:
    result: list[int] = []
    top: int = 0
    bottom: int = rows - 1
    left: int = 0
    right: int = cols - 1
    while top <= bottom and left <= right:
        c: int = left
        while c <= right:
            row: list[int] = matrix[top]
            result.append(row[c])
            c = c + 1
        top = top + 1
        r: int = top
        while r <= bottom:
            row2: list[int] = matrix[r]
            result.append(row2[right])
            r = r + 1
        right = right - 1
        if top <= bottom:
            c2: int = right
            while c2 >= left:
                row3: list[int] = matrix[bottom]
                result.append(row3[c2])
                c2 = c2 - 1
            bottom = bottom - 1
        if left <= right:
            r2: int = bottom
            while r2 >= top:
                row4: list[int] = matrix[r2]
                result.append(row4[left])
                r2 = r2 - 1
            left = left + 1
    return result

def generate_spiral(n: int) -> list[list[int]]:
    matrix: list[list[int]] = []
    i: int = 0
    while i < n:
        row: list[int] = []
        j: int = 0
        while j < n:
            row.append(0)
            j = j + 1
        matrix.append(row)
        i = i + 1
    num: int = 1
    top: int = 0
    bottom: int = n - 1
    left: int = 0
    right: int = n - 1
    while top <= bottom and left <= right:
        c: int = left
        while c <= right:
            row2: list[int] = matrix[top]
            row2[c] = num
            num = num + 1
            c = c + 1
        top = top + 1
        r: int = top
        while r <= bottom:
            row3: list[int] = matrix[r]
            row3[right] = num
            num = num + 1
            r = r + 1
        right = right - 1
        if top <= bottom:
            c2: int = right
            while c2 >= left:
                row4: list[int] = matrix[bottom]
                row4[c2] = num
                num = num + 1
                c2 = c2 - 1
            bottom = bottom - 1
        if left <= right:
            r2: int = bottom
            while r2 >= top:
                row5: list[int] = matrix[r2]
                row5[left] = num
                num = num + 1
                r2 = r2 - 1
            left = left + 1
    return matrix

def test_module() -> int:
    passed: int = 0
    m1: list[list[int]] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
    r1: list[int] = spiral_order(m1, 3, 3)
    if r1 == [1, 2, 3, 6, 9, 8, 7, 4, 5]:
        passed = passed + 1
    m2: list[list[int]] = [[1, 2], [3, 4]]
    r2: list[int] = spiral_order(m2, 2, 2)
    if r2 == [1, 2, 4, 3]:
        passed = passed + 1
    g1: list[list[int]] = generate_spiral(2)
    row0: list[int] = g1[0]
    row1: list[int] = g1[1]
    if row0 == [1, 2] and row1 == [4, 3]:
        passed = passed + 1
    g2: list[list[int]] = generate_spiral(1)
    grow: list[int] = g2[0]
    if grow == [1]:
        passed = passed + 1
    m3: list[list[int]] = [[1, 2, 3, 4]]
    r3: list[int] = spiral_order(m3, 1, 4)
    if r3 == [1, 2, 3, 4]:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
