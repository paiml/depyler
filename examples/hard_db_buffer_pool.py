from typing import List, Tuple

def create_pool(size: int) -> List[int]:
    return [-1] * size

def find_page(pool: List[int], page_id: int) -> int:
    for i in range(len(pool)):
        if pool[i] == page_id:
            return i
    return -1

def evict_lru(access_times: List[int]) -> int:
    if len(access_times) == 0:
        return -1
    oldest: int = 0
    for i in range(1, len(access_times)):
        if access_times[i] < access_times[oldest]:
            oldest = i
    return oldest

def fetch_page(pool: List[int], access_times: List[int], page_id: int, clock: int) -> Tuple[List[int], List[int]]:
    new_pool: List[int] = []
    for p in pool:
        new_pool.append(p)
    new_times: List[int] = []
    for t in access_times:
        new_times.append(t)
    for i in range(len(new_pool)):
        if new_pool[i] == page_id:
            new_times[i] = clock
            return (new_pool, new_times)
    for i in range(len(new_pool)):
        if new_pool[i] == -1:
            new_pool[i] = page_id
            new_times[i] = clock
            return (new_pool, new_times)
    oldest: int = 0
    for i in range(1, len(new_times)):
        if new_times[i] < new_times[oldest]:
            oldest = i
    new_pool[oldest] = page_id
    new_times[oldest] = clock
    return (new_pool, new_times)

def hit_rate(hits: int, total: int) -> float:
    if total == 0:
        return 0.0
    return float(hits) / float(total)
