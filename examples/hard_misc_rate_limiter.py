def token_bucket_init(capacity: int, rate: float) -> list[float]:
    state: list[float] = [capacity * 1.0, rate, capacity * 1.0, 0.0]
    return state

def token_bucket_request(state: list[float], current_time: float, tokens: float) -> int:
    capacity: float = state[0]
    rate: float = state[1]
    available: float = state[2]
    last_time: float = state[3]
    elapsed: float = current_time - last_time
    new_tokens: float = available + elapsed * rate
    if new_tokens > capacity:
        new_tokens = capacity
    if new_tokens >= tokens:
        state[2] = new_tokens - tokens
        state[3] = current_time
        return 1
    state[2] = new_tokens
    state[3] = current_time
    return 0

def sliding_window_count(timestamps: list[float], window: float, current_time: float) -> int:
    count: int = 0
    n: int = len(timestamps)
    i: int = 0
    while i < n:
        ts: float = timestamps[i]
        diff: float = current_time - ts
        if diff >= 0.0 and diff <= window:
            count = count + 1
        i = i + 1
    return count

def sliding_window_allow(timestamps: list[float], window: float, max_requests: int, current_time: float) -> int:
    count: int = sliding_window_count(timestamps, window, current_time)
    if count < max_requests:
        return 1
    return 0

def fixed_window_allow(counter: int, max_requests: int) -> int:
    if counter < max_requests:
        return 1
    return 0

def leaky_bucket_drain(queue_size: int, drain_rate: float, elapsed: float) -> int:
    drained: int = int(drain_rate * elapsed)
    new_size: int = queue_size - drained
    if new_size < 0:
        new_size = 0
    return new_size

def test_module() -> int:
    passed: int = 0
    state: list[float] = token_bucket_init(10, 1.0)
    r: int = token_bucket_request(state, 0.0, 5.0)
    if r == 1:
        passed = passed + 1
    r2: int = token_bucket_request(state, 0.0, 6.0)
    if r2 == 0:
        passed = passed + 1
    ts: list[float] = [1.0, 2.0, 3.0]
    c: int = sliding_window_count(ts, 5.0, 4.0)
    if c == 3:
        passed = passed + 1
    a: int = sliding_window_allow(ts, 5.0, 5, 4.0)
    if a == 1:
        passed = passed + 1
    fw: int = fixed_window_allow(5, 10)
    if fw == 1:
        passed = passed + 1
    return passed
