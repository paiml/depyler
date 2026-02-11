def lru_create(capacity: int) -> dict[str, int]:
    state: dict[str, int] = {}
    state["cap"] = capacity
    state["size"] = 0
    state["tick"] = 0
    return state

def lru_put(cache: dict[str, int], item: int, tick: int) -> int:
    cache[str(item)] = tick
    return tick + 1

def lru_evict_oldest(cache: dict[str, int], items: list[int], sz: int) -> int:
    oldest_tick: int = 999999
    oldest_idx: int = 0
    i: int = 0
    while i < sz:
        tag: str = str(items[i])
        if tag in cache:
            t: int = cache[tag]
            if t < oldest_tick:
                oldest_tick = t
                oldest_idx = i
        i = i + 1
    evicted: int = items[oldest_idx]
    del cache[str(evicted)]
    return evicted

def lru_contains(cache: dict[str, int], item: int) -> int:
    tag: str = str(item)
    if tag in cache:
        return 1
    return 0

def lru_access(cache: dict[str, int], item: int, tick: int) -> int:
    tag: str = str(item)
    if tag in cache:
        cache[tag] = tick
    return tick + 1

def test_module() -> int:
    passed: int = 0
    c: dict[str, int] = lru_create(3)
    if c["cap"] == 3:
        passed = passed + 1
    t: int = 0
    t = lru_put(c, 10, t)
    t = lru_put(c, 20, t)
    t = lru_put(c, 30, t)
    if lru_contains(c, 10) == 1:
        passed = passed + 1
    if lru_contains(c, 99) == 0:
        passed = passed + 1
    t = lru_access(c, 10, t)
    evicted: int = lru_evict_oldest(c, [10, 20, 30], 3)
    if evicted == 20:
        passed = passed + 1
    if lru_contains(c, 20) == 0:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
