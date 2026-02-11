"""Real-world event bus / dispatcher system.

Mimics: Observer pattern, event-driven architectures, signal/slot systems.
Uses integer event types and handler registration tables.
"""


def create_event_log() -> list[list[int]]:
    """Create empty event log. Each entry is [event_type, timestamp, payload]."""
    return []


def log_event(event_log: list[list[int]], event_type: int, timestamp: int, payload: int) -> int:
    """Log an event. Returns new log size."""
    event_log.append([event_type, timestamp, payload])
    return len(event_log)


def dispatch_event(handlers: list[list[int]], event_type: int, payload: int) -> list[int]:
    """Dispatch event to matching handlers. handlers is [[event_type, handler_id], ...].
    Returns list of handler_ids that matched."""
    matched: list[int] = []
    idx: int = 0
    while idx < len(handlers):
        if handlers[idx][0] == event_type:
            matched.append(handlers[idx][1])
        idx = idx + 1
    return matched


def register_handler(handlers: list[list[int]], event_type: int, handler_id: int) -> int:
    """Register a handler for an event type. Returns handler count."""
    handlers.append([event_type, handler_id])
    return len(handlers)


def unregister_handler(handlers: list[list[int]], handler_id: int) -> list[list[int]]:
    """Remove all registrations for a handler_id."""
    result: list[list[int]] = []
    idx: int = 0
    while idx < len(handlers):
        if handlers[idx][1] != handler_id:
            result.append(handlers[idx])
        idx = idx + 1
    return result


def count_events_by_type(event_log: list[list[int]], event_type: int) -> int:
    """Count occurrences of a specific event type in log."""
    count: int = 0
    idx: int = 0
    while idx < len(event_log):
        if event_log[idx][0] == event_type:
            count = count + 1
        idx = idx + 1
    return count


def events_in_time_range(event_log: list[list[int]], start_time: int, end_time: int) -> list[list[int]]:
    """Get events within a time range [start, end]."""
    result: list[list[int]] = []
    idx: int = 0
    while idx < len(event_log):
        ts: int = event_log[idx][1]
        if ts >= start_time and ts <= end_time:
            result.append(event_log[idx])
        idx = idx + 1
    return result


def event_rate(event_log: list[list[int]], window_size: int) -> list[int]:
    """Compute event count per time window. Returns counts for each window."""
    if len(event_log) == 0:
        return []
    max_time: int = 0
    idx: int = 0
    while idx < len(event_log):
        if event_log[idx][1] > max_time:
            max_time = event_log[idx][1]
        idx = idx + 1
    num_windows: int = (max_time // window_size) + 1
    counts: list[int] = []
    wi: int = 0
    while wi < num_windows:
        counts.append(0)
        wi = wi + 1
    idx2: int = 0
    while idx2 < len(event_log):
        bucket: int = event_log[idx2][1] // window_size
        counts[bucket] = counts[bucket] + 1
        idx2 = idx2 + 1
    return counts


def chain_events(log1: list[list[int]], log2: list[list[int]]) -> list[list[int]]:
    """Merge two event logs into one, ordered by timestamp."""
    merged: list[list[int]] = []
    i: int = 0
    j: int = 0
    while i < len(log1) and j < len(log2):
        if log1[i][1] <= log2[j][1]:
            merged.append(log1[i])
            i = i + 1
        else:
            merged.append(log2[j])
            j = j + 1
    while i < len(log1):
        merged.append(log1[i])
        i = i + 1
    while j < len(log2):
        merged.append(log2[j])
        j = j + 1
    return merged


def total_payload(event_log: list[list[int]]) -> int:
    """Sum all payloads in the event log."""
    total: int = 0
    idx: int = 0
    while idx < len(event_log):
        total = total + event_log[idx][2]
        idx = idx + 1
    return total


def test_module() -> int:
    """Test event system module."""
    passed: int = 0

    # Test 1: create and log events
    elog: list[list[int]] = create_event_log()
    log_event(elog, 1, 100, 42)
    log_event(elog, 2, 200, 10)
    log_event(elog, 1, 300, 20)
    if len(elog) == 3:
        passed = passed + 1

    # Test 2: dispatch to handlers
    handlers: list[list[int]] = []
    register_handler(handlers, 1, 101)
    register_handler(handlers, 1, 102)
    register_handler(handlers, 2, 103)
    matched: list[int] = dispatch_event(handlers, 1, 42)
    if len(matched) == 2:
        passed = passed + 1

    # Test 3: unregister handler
    remaining: list[list[int]] = unregister_handler(handlers, 102)
    if len(remaining) == 2:
        passed = passed + 1

    # Test 4: count events by type
    if count_events_by_type(elog, 1) == 2:
        passed = passed + 1

    # Test 5: time range filter
    ranged: list[list[int]] = events_in_time_range(elog, 150, 250)
    if len(ranged) == 1:
        passed = passed + 1

    # Test 6: event rate
    rates: list[int] = event_rate(elog, 200)
    if len(rates) == 2:
        passed = passed + 1

    # Test 7: chain merge
    log2: list[list[int]] = [[3, 150, 5], [3, 250, 7]]
    merged: list[list[int]] = chain_events(elog, log2)
    if len(merged) == 5:
        passed = passed + 1

    # Test 8: total payload
    tp: int = total_payload(elog)
    if tp == 72:
        passed = passed + 1

    return passed
