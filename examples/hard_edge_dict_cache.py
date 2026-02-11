"""Memoization with dict and LRU-like cache patterns."""


def fib_memo(n: int, memo: dict[int, int]) -> int:
    """Fibonacci with memoization dict."""
    if n in memo:
        return memo[n]
    if n <= 0:
        return 0
    if n == 1:
        return 1
    r1: int = fib_memo(n - 1, memo)
    r2: int = fib_memo(n - 2, memo)
    result: int = r1 + r2
    memo[n] = result
    return result


def fib_with_cache(n: int) -> int:
    """Fibonacci using iterative bottom-up cache."""
    cache_map: dict[int, int] = {}
    cache_map[0] = 0
    cache_map[1] = 1
    i: int = 2
    while i <= n:
        prev1: int = cache_map[i - 1]
        prev2: int = cache_map[i - 2]
        cache_map[i] = prev1 + prev2
        i = i + 1
    return cache_map[n]


def tribonacci_cached(n: int) -> int:
    """Tribonacci with dict cache."""
    cache_map: dict[int, int] = {}
    cache_map[0] = 0
    cache_map[1] = 0
    cache_map[2] = 1
    i: int = 3
    while i <= n:
        v1: int = cache_map[i - 1]
        v2: int = cache_map[i - 2]
        v3: int = cache_map[i - 3]
        cache_map[i] = v1 + v2 + v3
        i = i + 1
    return cache_map[n]


def collatz_steps_cached(n: int) -> int:
    """Count Collatz steps with caching."""
    cache_map: dict[int, int] = {}
    cache_map[1] = 0
    val: int = n
    steps: int = 0
    path: list[int] = []
    while val != 1:
        if val in cache_map:
            steps = steps + cache_map[val]
            break
        path.append(val)
        if val % 2 == 0:
            val = val // 2
        else:
            val = 3 * val + 1
        steps = steps + 1
    i: int = 0
    while i < len(path):
        pv: int = path[i]
        cache_map[pv] = steps - i
        i = i + 1
    return steps


def lru_cache_sim(capacity: int, requests: list[int]) -> list[int]:
    """Simulate an LRU cache. Returns list of hits (1) and misses (0)."""
    cache_order: list[int] = []
    cache_set: dict[int, int] = {}
    results: list[int] = []
    i: int = 0
    while i < len(requests):
        req: int = requests[i]
        if req in cache_set:
            results.append(1)
            new_order: list[int] = []
            j: int = 0
            while j < len(cache_order):
                if cache_order[j] != req:
                    new_order.append(cache_order[j])
                j = j + 1
            new_order.append(req)
            cache_order = new_order
        else:
            results.append(0)
            if len(cache_order) >= capacity:
                evicted: int = cache_order[0]
                new_order2: list[int] = []
                j = 1
                while j < len(cache_order):
                    new_order2.append(cache_order[j])
                    j = j + 1
                cache_order = new_order2
                del cache_set[evicted]
            cache_order.append(req)
            cache_set[req] = 1
        i = i + 1
    return results


def count_cache_hits(results: list[int]) -> int:
    """Count number of cache hits from results."""
    total: int = 0
    i: int = 0
    while i < len(results):
        total = total + results[i]
        i = i + 1
    return total


def test_module() -> int:
    """Test all cache and memoization functions."""
    passed: int = 0
    memo: dict[int, int] = {}
    if fib_memo(10, memo) == 55:
        passed = passed + 1
    if fib_memo(0, memo) == 0:
        passed = passed + 1
    if fib_with_cache(10) == 55:
        passed = passed + 1
    if fib_with_cache(1) == 1:
        passed = passed + 1
    if tribonacci_cached(7) == 13:
        passed = passed + 1
    cs: int = collatz_steps_cached(6)
    if cs == 8:
        passed = passed + 1
    lru_results: list[int] = lru_cache_sim(3, [1, 2, 3, 1, 4, 2, 5])
    if lru_results[0] == 0:
        passed = passed + 1
    if lru_results[3] == 1:
        passed = passed + 1
    hits: int = count_cache_hits(lru_results)
    if hits >= 1:
        passed = passed + 1
    return passed


if __name__ == "__main__":
    print(test_module())
