def fifo_cache_sim(requests: list[int], capacity: int) -> int:
    cache_items: list[int] = []
    hits: int = 0
    n: int = len(requests)
    i: int = 0
    while i < n:
        item: int = requests[i]
        found: int = 0
        j: int = 0
        cn: int = len(cache_items)
        while j < cn:
            cv: int = cache_items[j]
            if cv == item:
                found = 1
            j = j + 1
        if found == 1:
            hits = hits + 1
        else:
            cn2: int = len(cache_items)
            if cn2 >= capacity:
                new_cache: list[int] = []
                k: int = 1
                while k < cn2:
                    new_cache.append(cache_items[k])
                    k = k + 1
                new_cache.append(item)
                cache_items = new_cache
            else:
                cache_items.append(item)
        i = i + 1
    return hits

def lfu_count_sim(requests: list[int], capacity: int) -> int:
    cache_items: list[int] = []
    freqs: list[int] = []
    hits: int = 0
    n: int = len(requests)
    i: int = 0
    while i < n:
        item: int = requests[i]
        found: int = 0 - 1
        j: int = 0
        cn: int = len(cache_items)
        while j < cn:
            cv: int = cache_items[j]
            if cv == item:
                found = j
            j = j + 1
        if found >= 0:
            hits = hits + 1
            old_f: int = freqs[found]
            freqs[found] = old_f + 1
        else:
            cn2: int = len(cache_items)
            if cn2 >= capacity:
                min_idx: int = 0
                min_freq: int = freqs[0]
                m: int = 1
                while m < cn2:
                    fm: int = freqs[m]
                    if fm < min_freq:
                        min_freq = fm
                        min_idx = m
                    m = m + 1
                cache_items[min_idx] = item
                freqs[min_idx] = 1
            else:
                cache_items.append(item)
                freqs.append(1)
        i = i + 1
    return hits

def cache_hit_rate(hits: int, total: int) -> float:
    if total == 0:
        return 0.0
    return hits * 1.0 / (total * 1.0)

def test_module() -> int:
    passed: int = 0
    reqs: list[int] = [1, 2, 3, 1, 2, 4, 1]
    h: int = fifo_cache_sim(reqs, 3)
    if h >= 2:
        passed = passed + 1
    h2: int = fifo_cache_sim(reqs, 1)
    if h2 >= 0:
        passed = passed + 1
    h3: int = lfu_count_sim(reqs, 3)
    if h3 >= 2:
        passed = passed + 1
    rate: float = cache_hit_rate(3, 10)
    diff: float = rate - 0.3
    if diff < 0.01 and diff > (0.0 - 0.01):
        passed = passed + 1
    rate2: float = cache_hit_rate(0, 0)
    if rate2 == 0.0:
        passed = passed + 1
    return passed
