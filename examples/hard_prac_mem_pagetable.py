"""Page table walk simulation.

Two-level page table: directory -> table -> physical frame.
Simulates virtual-to-physical address translation.
"""


def pt_init_neg(size: int) -> list[int]:
    """Initialize with -1 (invalid)."""
    result: list[int] = []
    i: int = 0
    while i < size:
        result.append(0 - 1)
        i = i + 1
    return result


def pt_init_zeros(size: int) -> list[int]:
    """Initialize with zeros."""
    result: list[int] = []
    i: int = 0
    while i < size:
        result.append(0)
        i = i + 1
    return result


def pt_extract_dir_idx(vaddr: int, page_bits: int, table_bits: int) -> int:
    """Extract directory index from virtual address."""
    shift: int = page_bits + table_bits
    result: int = vaddr // (1 * (2 ** shift))
    return result


def pt_extract_table_idx(vaddr: int, page_bits: int, table_bits: int) -> int:
    """Extract table index from virtual address."""
    table_size: int = 1
    i: int = 0
    while i < table_bits:
        table_size = table_size * 2
        i = i + 1
    page_size: int = 1
    j: int = 0
    while j < page_bits:
        page_size = page_size * 2
        j = j + 1
    shifted: int = vaddr // page_size
    result: int = shifted % table_size
    return result


def pt_extract_offset(vaddr: int, page_bits: int) -> int:
    """Extract page offset from virtual address."""
    page_size: int = 1
    i: int = 0
    while i < page_bits:
        page_size = page_size * 2
        i = i + 1
    return vaddr % page_size


def pt_map_page(page_dir: list[int], page_tables: list[int],
                dir_idx: int, table_idx: int, frame: int,
                table_entries: int) -> int:
    """Map a page: set directory entry and page table entry. Returns 1."""
    page_dir[dir_idx] = 1
    offset: int = dir_idx * table_entries + table_idx
    page_tables[offset] = frame
    return 1


def pt_translate(page_dir: list[int], page_tables: list[int],
                 vaddr: int, page_bits: int, table_bits: int,
                 table_entries: int) -> int:
    """Translate virtual address to physical. Returns physical addr or -1."""
    dir_idx: int = pt_extract_dir_idx(vaddr, page_bits, table_bits)
    d: int = page_dir[dir_idx]
    if d < 0:
        return 0 - 1
    table_idx: int = pt_extract_table_idx(vaddr, page_bits, table_bits)
    offset: int = dir_idx * table_entries + table_idx
    frame: int = page_tables[offset]
    if frame < 0:
        return 0 - 1
    pg_offset: int = pt_extract_offset(vaddr, page_bits)
    page_size: int = 1
    i: int = 0
    while i < page_bits:
        page_size = page_size * 2
        i = i + 1
    phys: int = frame * page_size + pg_offset
    return phys


def pt_count_mapped(page_tables: list[int], total_entries: int) -> int:
    """Count mapped pages."""
    count: int = 0
    i: int = 0
    while i < total_entries:
        f: int = page_tables[i]
        if f >= 0:
            count = count + 1
        i = i + 1
    return count


def test_module() -> int:
    """Test page table walk."""
    passed: int = 0
    page_bits: int = 4
    table_bits: int = 4
    table_entries: int = 16
    dir_entries: int = 16
    total_pt: int = dir_entries * table_entries

    page_dir: list[int] = pt_init_neg(dir_entries)
    page_tables: list[int] = pt_init_neg(total_pt)

    # Test 1: map page and translate
    pt_map_page(page_dir, page_tables, 0, 0, 5, table_entries)
    phys: int = pt_translate(page_dir, page_tables, 0, page_bits, table_bits, table_entries)
    if phys == 80:
        passed = passed + 1

    # Test 2: offset preservation
    phys2: int = pt_translate(page_dir, page_tables, 7, page_bits, table_bits, table_entries)
    if phys2 == 87:
        passed = passed + 1

    # Test 3: unmapped page returns -1
    miss: int = pt_translate(page_dir, page_tables, 256, page_bits, table_bits, table_entries)
    if miss == (0 - 1):
        passed = passed + 1

    # Test 4: map second page
    pt_map_page(page_dir, page_tables, 0, 1, 10, table_entries)
    phys3: int = pt_translate(page_dir, page_tables, 16, page_bits, table_bits, table_entries)
    if phys3 == 160:
        passed = passed + 1

    # Test 5: count mapped
    mapped: int = pt_count_mapped(page_tables, total_pt)
    if mapped == 2:
        passed = passed + 1

    return passed
