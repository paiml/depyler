"""ARP table simulation.

Maps IP addresses (ints) to MAC addresses (ints) with aging.
Entries expire after a timeout and can be refreshed.
"""


def arp_init(capacity: int) -> list[int]:
    """Initialize with -1."""
    result: list[int] = []
    i: int = 0
    while i < capacity:
        result.append(0 - 1)
        i = i + 1
    return result


def arp_init_zeros(capacity: int) -> list[int]:
    """Initialize with zeros."""
    result: list[int] = []
    i: int = 0
    while i < capacity:
        result.append(0)
        i = i + 1
    return result


def arp_find(ips: list[int], target_ip: int, capacity: int) -> int:
    """Find IP in table. Returns index or -1."""
    i: int = 0
    while i < capacity:
        ip: int = ips[i]
        if ip == target_ip:
            return i
        i = i + 1
    return 0 - 1


def arp_first_empty(ips: list[int], capacity: int) -> int:
    """Find first empty slot. Returns index or -1."""
    i: int = 0
    while i < capacity:
        ip: int = ips[i]
        if ip == (0 - 1):
            return i
        i = i + 1
    return 0 - 1


def arp_oldest(timestamps: list[int], ips: list[int], capacity: int) -> int:
    """Find oldest valid entry. Returns index."""
    oldest: int = 0
    oldest_time: int = 2147483647
    i: int = 0
    while i < capacity:
        ip: int = ips[i]
        if ip != (0 - 1):
            t: int = timestamps[i]
            if t < oldest_time:
                oldest_time = t
                oldest = i
        i = i + 1
    return oldest


def arp_insert(ips: list[int], macs: list[int], timestamps: list[int],
               capacity: int, ip_addr: int, mac_addr: int, now: int) -> int:
    """Insert or update ARP entry. Returns 1 on success."""
    idx: int = arp_find(ips, ip_addr, capacity)
    if idx >= 0:
        macs[idx] = mac_addr
        timestamps[idx] = now
        return 1
    slot: int = arp_first_empty(ips, capacity)
    if slot >= 0:
        ips[slot] = ip_addr
        macs[slot] = mac_addr
        timestamps[slot] = now
        return 1
    victim: int = arp_oldest(timestamps, ips, capacity)
    ips[victim] = ip_addr
    macs[victim] = mac_addr
    timestamps[victim] = now
    return 1


def arp_lookup(ips: list[int], macs: list[int], timestamps: list[int],
               capacity: int, ip_addr: int, now: int, timeout: int) -> int:
    """Lookup MAC for IP. Returns MAC or -1 if not found or expired."""
    idx: int = arp_find(ips, ip_addr, capacity)
    if idx < 0:
        return 0 - 1
    t: int = timestamps[idx]
    if now - t > timeout:
        ips[idx] = 0 - 1
        return 0 - 1
    result: int = macs[idx]
    return result


def arp_count(ips: list[int], capacity: int) -> int:
    """Count valid entries."""
    count: int = 0
    i: int = 0
    while i < capacity:
        ip: int = ips[i]
        if ip != (0 - 1):
            count = count + 1
        i = i + 1
    return count


def test_module() -> int:
    """Test ARP table operations."""
    passed: int = 0
    cap: int = 4
    ips: list[int] = arp_init(cap)
    macs: list[int] = arp_init(cap)
    ts: list[int] = arp_init_zeros(cap)
    timeout: int = 100

    # Test 1: insert and lookup
    arp_insert(ips, macs, ts, cap, 1001, 5001, 10)
    mac: int = arp_lookup(ips, macs, ts, cap, 1001, 20, timeout)
    if mac == 5001:
        passed = passed + 1

    # Test 2: miss returns -1
    miss: int = arp_lookup(ips, macs, ts, cap, 9999, 20, timeout)
    if miss == (0 - 1):
        passed = passed + 1

    # Test 3: expired entry returns -1
    arp_insert(ips, macs, ts, cap, 1002, 5002, 10)
    expired: int = arp_lookup(ips, macs, ts, cap, 1002, 200, timeout)
    if expired == (0 - 1):
        passed = passed + 1

    # Test 4: update refreshes timestamp
    arp_insert(ips, macs, ts, cap, 1001, 5099, 150)
    mac2: int = arp_lookup(ips, macs, ts, cap, 1001, 200, timeout)
    if mac2 == 5099:
        passed = passed + 1

    # Test 5: overflow evicts oldest
    arp_insert(ips, macs, ts, cap, 1003, 5003, 160)
    arp_insert(ips, macs, ts, cap, 1004, 5004, 170)
    arp_insert(ips, macs, ts, cap, 1005, 5005, 180)
    cnt: int = arp_count(ips, cap)
    if cnt == 4:
        passed = passed + 1

    return passed
