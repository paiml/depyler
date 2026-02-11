def josephus(n: int, step: int) -> int:
    if n == 1:
        return 0
    result: int = 0
    i: int = 2
    while i <= n:
        result = (result + step) % i
        i = i + 1
    return result

def josephus_recursive(n: int, step: int) -> int:
    if n == 1:
        return 0
    sub: int = josephus_recursive(n - 1, step)
    return (sub + step) % n

def josephus_one_indexed(n: int, step: int) -> int:
    return josephus(n, step) + 1

def josephus_sequence(n: int, step: int) -> list[int]:
    circle: list[int] = []
    i: int = 0
    while i < n:
        circle.append(i)
        i = i + 1
    order: list[int] = []
    idx: int = 0
    while len(circle) > 0:
        sz: int = len(circle)
        idx = (idx + step - 1) % sz
        eliminated: int = circle[idx]
        order.append(eliminated)
        new_circle: list[int] = []
        j: int = 0
        while j < sz:
            if j != idx:
                new_circle.append(circle[j])
            j = j + 1
        circle = new_circle
        if idx >= len(circle) and len(circle) > 0:
            idx = idx % len(circle)
    return order

def last_two_standing(n: int, step: int) -> list[int]:
    circle: list[int] = []
    i: int = 0
    while i < n:
        circle.append(i)
        i = i + 1
    idx: int = 0
    while len(circle) > 2:
        sz: int = len(circle)
        idx = (idx + step - 1) % sz
        new_circle: list[int] = []
        j: int = 0
        while j < sz:
            if j != idx:
                new_circle.append(circle[j])
            j = j + 1
        circle = new_circle
        if idx >= len(circle) and len(circle) > 0:
            idx = idx % len(circle)
    return circle

def test_module() -> int:
    passed: int = 0
    r1: int = josephus(7, 3)
    if r1 == 3:
        passed = passed + 1
    r2: int = josephus_one_indexed(7, 3)
    if r2 == 4:
        passed = passed + 1
    r3: int = josephus_recursive(5, 2)
    r3b: int = josephus(5, 2)
    if r3 == r3b:
        passed = passed + 1
    seq: list[int] = josephus_sequence(5, 2)
    ns: int = len(seq)
    if ns == 5:
        passed = passed + 1
    pair: list[int] = last_two_standing(6, 2)
    np: int = len(pair)
    if np == 2:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
