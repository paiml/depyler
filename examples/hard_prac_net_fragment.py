"""Packet fragmentation and reassembly simulation.

Splits large packets into MTU-sized fragments and reassembles them.
Tracks fragment offsets and more-fragments flag.
"""


def frag_split(packet_size: int, mtu: int) -> int:
    """Calculate number of fragments needed."""
    if packet_size <= mtu:
        return 1
    count: int = packet_size // mtu
    remainder: int = packet_size % mtu
    if remainder > 0:
        count = count + 1
    return count


def frag_create_offsets(packet_size: int, mtu: int, offsets: list[int],
                        sizes: list[int], more_flags: list[int]) -> int:
    """Fill fragment metadata arrays. Returns number of fragments."""
    num_frags: int = frag_split(packet_size, mtu)
    offset: int = 0
    i: int = 0
    while i < num_frags:
        offsets[i] = offset
        remaining: int = packet_size - offset
        if remaining > mtu:
            sizes[i] = mtu
            more_flags[i] = 1
        else:
            sizes[i] = remaining
            more_flags[i] = 0
        offset = offset + mtu
        i = i + 1
    return num_frags


def frag_total_size(sizes: list[int], num_frags: int) -> int:
    """Sum of all fragment sizes."""
    total: int = 0
    i: int = 0
    while i < num_frags:
        s: int = sizes[i]
        total = total + s
        i = i + 1
    return total


def frag_is_complete(received: list[int], num_frags: int) -> int:
    """Check if all fragments received. Returns 1 if complete."""
    i: int = 0
    while i < num_frags:
        r: int = received[i]
        if r == 0:
            return 0
        i = i + 1
    return 1


def frag_receive(received: list[int], frag_idx: int) -> int:
    """Mark fragment as received. Returns 1."""
    received[frag_idx] = 1
    return 1


def frag_count_received(received: list[int], num_frags: int) -> int:
    """Count received fragments."""
    count: int = 0
    i: int = 0
    while i < num_frags:
        r: int = received[i]
        if r == 1:
            count = count + 1
        i = i + 1
    return count


def frag_init_zeros(size: int) -> list[int]:
    """Initialize with zeros."""
    result: list[int] = []
    i: int = 0
    while i < size:
        result.append(0)
        i = i + 1
    return result


def test_module() -> int:
    """Test packet fragmentation."""
    passed: int = 0
    packet_size: int = 5000
    mtu: int = 1500
    max_frags: int = 10

    # Test 1: fragment count calculation
    nf: int = frag_split(packet_size, mtu)
    if nf == 4:
        passed = passed + 1

    # Test 2: no fragmentation for small packets
    nf2: int = frag_split(1000, mtu)
    if nf2 == 1:
        passed = passed + 1

    # Test 3: fragment offsets and sizes
    offsets: list[int] = frag_init_zeros(max_frags)
    sizes: list[int] = frag_init_zeros(max_frags)
    mf: list[int] = frag_init_zeros(max_frags)
    frag_create_offsets(packet_size, mtu, offsets, sizes, mf)
    total: int = frag_total_size(sizes, nf)
    if total == packet_size:
        passed = passed + 1

    # Test 4: last fragment has more_fragments=0
    last_mf: int = mf[nf - 1]
    if last_mf == 0:
        passed = passed + 1

    # Test 5: reassembly tracking
    received: list[int] = frag_init_zeros(max_frags)
    frag_receive(received, 0)
    frag_receive(received, 1)
    frag_receive(received, 2)
    frag_receive(received, 3)
    complete: int = frag_is_complete(received, nf)
    if complete == 1:
        passed = passed + 1

    return passed
