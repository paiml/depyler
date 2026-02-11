"""QoS (Quality of Service) packet classifier and scheduler simulation.

Classifies packets into priority queues and schedules using weighted
round-robin across traffic classes.
"""


def qos_init_zeros(size: int) -> list[int]:
    """Initialize with zeros."""
    result: list[int] = []
    i: int = 0
    while i < size:
        result.append(0)
        i = i + 1
    return result


def qos_init_neg(size: int) -> list[int]:
    """Initialize with -1."""
    result: list[int] = []
    i: int = 0
    while i < size:
        result.append(0 - 1)
        i = i + 1
    return result


def qos_classify(dscp: int) -> int:
    """Classify packet by DSCP value into queue (0=best-effort, 1=normal, 2=priority, 3=critical)."""
    if dscp >= 46:
        return 3
    if dscp >= 34:
        return 2
    if dscp >= 18:
        return 1
    return 0


def qos_enqueue(queues: list[int], tails: list[int],
                queue_class: int, pkt_id: int,
                max_per_q: int, num_classes: int) -> int:
    """Enqueue packet into class queue. Returns 1 on success, 0 if full."""
    offset: int = queue_class * max_per_q
    t: int = tails[queue_class]
    if t >= max_per_q:
        return 0
    queues[offset + t] = pkt_id
    tails[queue_class] = t + 1
    return 1


def qos_dequeue(queues: list[int], tails: list[int],
                queue_class: int, max_per_q: int) -> int:
    """Dequeue from front of class queue. Returns pkt_id or -1."""
    t: int = tails[queue_class]
    if t == 0:
        return 0 - 1
    offset: int = queue_class * max_per_q
    result: int = queues[offset]
    j: int = 0
    while j < t - 1:
        nv: int = queues[offset + j + 1]
        queues[offset + j] = nv
        j = j + 1
    queues[offset + t - 1] = 0 - 1
    tails[queue_class] = t - 1
    return result


def qos_wrr_schedule(queues: list[int], tails: list[int],
                     weights: list[int], credits: list[int],
                     num_classes: int, max_per_q: int) -> int:
    """Weighted round-robin: serve from highest class with credits.
    Returns pkt_id served, or -1."""
    c: int = num_classes - 1
    while c >= 0:
        t: int = tails[c]
        cr: int = credits[c]
        if t > 0:
            if cr > 0:
                credits[c] = cr - 1
                pkt: int = qos_dequeue(queues, tails, c, max_per_q)
                return pkt
        c = c - 1
    j: int = 0
    while j < num_classes:
        w: int = weights[j]
        credits[j] = w
        j = j + 1
    return 0 - 1


def qos_total_queued(tails: list[int], num_classes: int) -> int:
    """Total packets across all queues."""
    total: int = 0
    i: int = 0
    while i < num_classes:
        t: int = tails[i]
        total = total + t
        i = i + 1
    return total


def test_module() -> int:
    """Test QoS scheduler."""
    passed: int = 0
    num_classes: int = 4
    max_per_q: int = 8
    total_slots: int = num_classes * max_per_q
    queues: list[int] = qos_init_neg(total_slots)
    tails: list[int] = qos_init_zeros(num_classes)
    weights: list[int] = [1, 2, 3, 4]
    credits: list[int] = [1, 2, 3, 4]

    # Test 1: classify packets
    c1: int = qos_classify(0)
    c2: int = qos_classify(46)
    if c1 == 0:
        if c2 == 3:
            passed = passed + 1

    # Test 2: enqueue to different classes
    qos_enqueue(queues, tails, 0, 100, max_per_q, num_classes)
    qos_enqueue(queues, tails, 3, 200, max_per_q, num_classes)
    qos_enqueue(queues, tails, 1, 300, max_per_q, num_classes)
    total: int = qos_total_queued(tails, num_classes)
    if total == 3:
        passed = passed + 1

    # Test 3: WRR serves highest class first
    pkt: int = qos_wrr_schedule(queues, tails, weights, credits, num_classes, max_per_q)
    if pkt == 200:
        passed = passed + 1

    # Test 4: after serving critical, serves next
    pkt2: int = qos_wrr_schedule(queues, tails, weights, credits, num_classes, max_per_q)
    if pkt2 == 300:
        passed = passed + 1

    # Test 5: total queued decreases
    total2: int = qos_total_queued(tails, num_classes)
    if total2 == 1:
        passed = passed + 1

    return passed
