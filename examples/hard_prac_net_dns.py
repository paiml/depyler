"""DNS resolver cache simulation.

Caches DNS lookups with TTL. Supports domain name hashing
and iterative resolution through hierarchy levels.
"""


def dns_hash(domain: int, table_size: int) -> int:
    """Simple hash function for domain (represented as int)."""
    h: int = ((domain * 2654435761) // 1) % table_size
    if h < 0:
        h = 0 - h
    return h


def dns_init(size: int) -> list[int]:
    """Initialize with -1."""
    result: list[int] = []
    i: int = 0
    while i < size:
        result.append(0 - 1)
        i = i + 1
    return result


def dns_init_zeros(size: int) -> list[int]:
    """Initialize with zeros."""
    result: list[int] = []
    i: int = 0
    while i < size:
        result.append(0)
        i = i + 1
    return result


def dns_insert(domains: list[int], addrs: list[int], ttls: list[int],
               insert_times: list[int], table_size: int,
               domain: int, addr: int, ttl_val: int, now: int) -> int:
    """Insert DNS record. Uses linear probing. Returns slot index."""
    h: int = dns_hash(domain, table_size)
    i: int = 0
    while i < table_size:
        idx: int = (h + i) % table_size
        d: int = domains[idx]
        if d == (0 - 1):
            domains[idx] = domain
            addrs[idx] = addr
            ttls[idx] = ttl_val
            insert_times[idx] = now
            return idx
        if d == domain:
            addrs[idx] = addr
            ttls[idx] = ttl_val
            insert_times[idx] = now
            return idx
        i = i + 1
    return 0 - 1


def dns_resolve(domains: list[int], addrs: list[int], ttls: list[int],
                insert_times: list[int], table_size: int,
                domain: int, now: int) -> int:
    """Resolve domain to address. Returns address or -1 if not found/expired."""
    h: int = dns_hash(domain, table_size)
    i: int = 0
    while i < table_size:
        idx: int = (h + i) % table_size
        d: int = domains[idx]
        if d == (0 - 1):
            return 0 - 1
        if d == domain:
            t: int = insert_times[idx]
            ttl_val: int = ttls[idx]
            if now - t > ttl_val:
                domains[idx] = 0 - 1
                return 0 - 1
            result: int = addrs[idx]
            return result
        i = i + 1
    return 0 - 1


def dns_count_valid(domains: list[int], table_size: int) -> int:
    """Count valid entries."""
    count: int = 0
    i: int = 0
    while i < table_size:
        d: int = domains[i]
        if d != (0 - 1):
            count = count + 1
        i = i + 1
    return count


def dns_purge_expired(domains: list[int], insert_times: list[int],
                      ttls: list[int], table_size: int, now: int) -> int:
    """Remove expired entries. Returns count purged."""
    purged: int = 0
    i: int = 0
    while i < table_size:
        d: int = domains[i]
        if d != (0 - 1):
            t: int = insert_times[i]
            ttl_val: int = ttls[i]
            if now - t > ttl_val:
                domains[i] = 0 - 1
                purged = purged + 1
        i = i + 1
    return purged


def test_module() -> int:
    """Test DNS resolver cache."""
    passed: int = 0
    tsize: int = 16
    domains: list[int] = dns_init(tsize)
    addrs: list[int] = dns_init(tsize)
    ttls: list[int] = dns_init_zeros(tsize)
    ins_times: list[int] = dns_init_zeros(tsize)

    # Test 1: insert and resolve
    dns_insert(domains, addrs, ttls, ins_times, tsize, 1000, 2000, 60, 10)
    resolved: int = dns_resolve(domains, addrs, ttls, ins_times, tsize, 1000, 20)
    if resolved == 2000:
        passed = passed + 1

    # Test 2: miss returns -1
    miss: int = dns_resolve(domains, addrs, ttls, ins_times, tsize, 9999, 20)
    if miss == (0 - 1):
        passed = passed + 1

    # Test 3: expired entry
    dns_insert(domains, addrs, ttls, ins_times, tsize, 1001, 2001, 10, 5)
    expired: int = dns_resolve(domains, addrs, ttls, ins_times, tsize, 1001, 20)
    if expired == (0 - 1):
        passed = passed + 1

    # Test 4: multiple entries
    dns_insert(domains, addrs, ttls, ins_times, tsize, 1002, 2002, 60, 10)
    dns_insert(domains, addrs, ttls, ins_times, tsize, 1003, 2003, 60, 10)
    cnt: int = dns_count_valid(domains, tsize)
    if cnt >= 3:
        passed = passed + 1

    # Test 5: purge works
    dns_insert(domains, addrs, ttls, ins_times, tsize, 1004, 2004, 5, 0)
    purged: int = dns_purge_expired(domains, ttls, ins_times, tsize, 100)
    if purged >= 1:
        passed = passed + 1

    return passed
