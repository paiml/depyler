def make_snake() -> list[list[int]]:
    snake: list[list[int]] = [[5, 5], [5, 4], [5, 3]]
    return snake

def move_head(head_r: int, head_c: int, direction: int) -> list[int]:
    nr: int = head_r
    nc: int = head_c
    if direction == 0:
        nr = head_r - 1
    if direction == 1:
        nc = head_c + 1
    if direction == 2:
        nr = head_r + 1
    if direction == 3:
        nc = head_c - 1
    result: list[int] = [nr, nc]
    return result

def check_wall(r: int, c: int, rows: int, cols: int) -> int:
    if r < 0 or r >= rows or c < 0 or c >= cols:
        return 1
    return 0

def check_self_collision(snake: list[list[int]], r: int, c: int) -> int:
    n: int = len(snake)
    i: int = 0
    while i < n:
        seg: list[int] = snake[i]
        sr: int = seg[0]
        sc: int = seg[1]
        if sr == r and sc == c:
            return 1
        i = i + 1
    return 0

def snake_length(snake: list[list[int]]) -> int:
    return len(snake)

def grow_snake(snake: list[list[int]], new_head: list[int]) -> list[list[int]]:
    result: list[list[int]] = [new_head]
    n: int = len(snake)
    i: int = 0
    while i < n:
        result.append(snake[i])
        i = i + 1
    return result

def move_snake(snake: list[list[int]], new_head: list[int]) -> list[list[int]]:
    result: list[list[int]] = [new_head]
    n: int = len(snake)
    i: int = 0
    while i < n - 1:
        result.append(snake[i])
        i = i + 1
    return result

def test_module() -> int:
    passed: int = 0
    s: list[list[int]] = make_snake()
    sl: int = snake_length(s)
    if sl == 3:
        passed = passed + 1
    nh: list[int] = move_head(5, 5, 0)
    nr: int = nh[0]
    if nr == 4:
        passed = passed + 1
    cw: int = check_wall(0 - 1, 0, 10, 10)
    if cw == 1:
        passed = passed + 1
    cw2: int = check_wall(5, 5, 10, 10)
    if cw2 == 0:
        passed = passed + 1
    sc: int = check_self_collision(s, 5, 5)
    if sc == 1:
        passed = passed + 1
    return passed
