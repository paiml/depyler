"""Box stacking problem: find maximum height stack of boxes.

Tests: single box, multiple boxes, all rotations, known optimal heights.
"""


def box_stacking(widths: list[int], depths: list[int], heights: list[int]) -> int:
    """Return maximum possible stack height.

    A box can be placed on another only if both width and depth are strictly smaller.
    Each box generates 3 rotations.
    """
    n: int = len(widths)
    rot_w: list[int] = []
    rot_d: list[int] = []
    rot_h: list[int] = []
    i: int = 0
    while i < n:
        w: int = widths[i]
        d: int = depths[i]
        h: int = heights[i]
        a: int = w
        b: int = d
        if a > b:
            rot_w.append(b)
            rot_d.append(a)
        else:
            rot_w.append(a)
            rot_d.append(b)
        rot_h.append(h)
        a = w
        b = h
        if a > b:
            rot_w.append(b)
            rot_d.append(a)
        else:
            rot_w.append(a)
            rot_d.append(b)
        rot_h.append(d)
        a = d
        b = h
        if a > b:
            rot_w.append(b)
            rot_d.append(a)
        else:
            rot_w.append(a)
            rot_d.append(b)
        rot_h.append(w)
        i = i + 1
    m: int = len(rot_w)
    idx: int = 0
    while idx < m - 1:
        jj: int = 0
        while jj < m - 1 - idx:
            area_j: int = rot_w[jj] * rot_d[jj]
            area_j1: int = rot_w[jj + 1] * rot_d[jj + 1]
            if area_j < area_j1:
                tw: int = rot_w[jj]
                rot_w[jj] = rot_w[jj + 1]
                rot_w[jj + 1] = tw
                td: int = rot_d[jj]
                rot_d[jj] = rot_d[jj + 1]
                rot_d[jj + 1] = td
                th: int = rot_h[jj]
                rot_h[jj] = rot_h[jj + 1]
                rot_h[jj + 1] = th
            jj = jj + 1
        idx = idx + 1
    dp: list[int] = []
    k: int = 0
    while k < m:
        dp.append(rot_h[k])
        k = k + 1
    i = 1
    while i < m:
        j: int = 0
        while j < i:
            if rot_w[j] > rot_w[i] and rot_d[j] > rot_d[i]:
                candidate: int = dp[j] + rot_h[i]
                if candidate > dp[i]:
                    dp[i] = candidate
            j = j + 1
        i = i + 1
    best: int = 0
    i = 0
    while i < m:
        if dp[i] > best:
            best = dp[i]
        i = i + 1
    return best


def simple_stack_height(heights: list[int]) -> int:
    """Return sum of all heights (no stacking constraint)."""
    total: int = 0
    i: int = 0
    while i < len(heights):
        total = total + heights[i]
        i = i + 1
    return total


def test_module() -> int:
    """Test box stacking problem."""
    ok: int = 0

    w1: list[int] = [4, 1, 4]
    d1: list[int] = [6, 2, 6]
    h1: list[int] = [7, 3, 7]
    result1: int = box_stacking(w1, d1, h1)
    if result1 >= 10:
        ok = ok + 1

    w2: list[int] = [2]
    d2: list[int] = [3]
    h2: list[int] = [5]
    result2: int = box_stacking(w2, d2, h2)
    if result2 >= 5:
        ok = ok + 1

    w3: list[int] = [1, 2, 3]
    d3: list[int] = [1, 2, 3]
    h3: list[int] = [1, 2, 3]
    result3: int = box_stacking(w3, d3, h3)
    if result3 >= 6:
        ok = ok + 1

    if simple_stack_height(h1) == 17:
        ok = ok + 1

    w4: list[int] = [4, 1]
    d4: list[int] = [6, 2]
    h4: list[int] = [7, 3]
    result4: int = box_stacking(w4, d4, h4)
    if result4 >= 10:
        ok = ok + 1

    empty_w: list[int] = []
    empty_d: list[int] = []
    empty_h: list[int] = []
    if box_stacking(empty_w, empty_d, empty_h) == 0:
        ok = ok + 1

    w5: list[int] = [5, 5, 5]
    d5: list[int] = [5, 5, 5]
    h5: list[int] = [1, 2, 3]
    result5: int = box_stacking(w5, d5, h5)
    if result5 >= 3:
        ok = ok + 1

    if simple_stack_height(h3) == 6:
        ok = ok + 1

    w6: list[int] = [1, 2]
    d6: list[int] = [2, 3]
    h6: list[int] = [10, 20]
    result6: int = box_stacking(w6, d6, h6)
    if result6 >= 30:
        ok = ok + 1

    w7: list[int] = [3]
    d7: list[int] = [4]
    h7: list[int] = [5]
    if box_stacking(w7, d7, h7) >= 5:
        ok = ok + 1

    return ok
