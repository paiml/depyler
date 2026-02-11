"""Path resolution simulation.

Resolves path components through a directory hierarchy.
Path represented as list of name IDs. Directory entries stored flat.
"""


def pr_init(capacity: int) -> list[int]:
    """Initialize with -1."""
    result: list[int] = []
    i: int = 0
    while i < capacity:
        result.append(0 - 1)
        i = i + 1
    return result


def pr_add_entry(parent_ids: list[int], name_ids: list[int],
                 inode_ids: list[int], is_dir: list[int],
                 count_arr: list[int], parent: int, name: int,
                 inode: int, directory: int) -> int:
    """Add directory entry. Returns index."""
    idx: int = count_arr[0]
    parent_ids[idx] = parent
    name_ids[idx] = name
    inode_ids[idx] = inode
    is_dir[idx] = directory
    count_arr[0] = idx + 1
    return idx


def pr_lookup_in_dir(parent_ids: list[int], name_ids: list[int],
                     inode_ids: list[int], count: int,
                     dir_inode: int, target_name: int) -> int:
    """Find entry with given name in directory. Returns inode or -1."""
    i: int = 0
    while i < count:
        p: int = parent_ids[i]
        n: int = name_ids[i]
        if p == dir_inode:
            if n == target_name:
                result: int = inode_ids[i]
                return result
        i = i + 1
    return 0 - 1


def pr_is_directory(parent_ids: list[int], inode_ids: list[int],
                    is_dir: list[int], count: int, inode: int) -> int:
    """Check if inode is a directory. Returns 1 if yes."""
    i: int = 0
    while i < count:
        ino: int = inode_ids[i]
        if ino == inode:
            d: int = is_dir[i]
            return d
        i = i + 1
    return 0


def pr_resolve(parent_ids: list[int], name_ids: list[int],
               inode_ids: list[int], count: int,
               path_components: list[int], path_len: int,
               root_inode: int) -> int:
    """Resolve a path from root. Returns final inode or -1 on error."""
    current: int = root_inode
    step: int = 0
    while step < path_len:
        component: int = path_components[step]
        found: int = pr_lookup_in_dir(parent_ids, name_ids, inode_ids, count, current, component)
        if found < 0:
            return 0 - 1
        current = found
        step = step + 1
    return current


def pr_count_entries_in_dir(parent_ids: list[int], count: int, dir_inode: int) -> int:
    """Count entries in a directory."""
    total: int = 0
    i: int = 0
    while i < count:
        p: int = parent_ids[i]
        if p == dir_inode:
            total = total + 1
        i = i + 1
    return total


def test_module() -> int:
    """Test path resolution."""
    passed: int = 0
    cap: int = 16
    pids: list[int] = pr_init(cap)
    nids: list[int] = pr_init(cap)
    inids: list[int] = pr_init(cap)
    isdir: list[int] = pr_init(cap)
    cnt: list[int] = [0]

    # Build: / (inode 0) -> home (inode 1) -> user (inode 2), / -> etc (inode 3)
    pr_add_entry(pids, nids, inids, isdir, cnt, 0 - 1, 100, 0, 1)
    pr_add_entry(pids, nids, inids, isdir, cnt, 0, 101, 1, 1)
    pr_add_entry(pids, nids, inids, isdir, cnt, 1, 102, 2, 1)
    pr_add_entry(pids, nids, inids, isdir, cnt, 0, 103, 3, 1)
    pr_add_entry(pids, nids, inids, isdir, cnt, 2, 104, 4, 0)

    # Test 1: resolve /home -> inode 1
    path1: list[int] = [101]
    r1: int = pr_resolve(pids, nids, inids, cnt[0], path1, 1, 0)
    if r1 == 1:
        passed = passed + 1

    # Test 2: resolve /home/user -> inode 2
    path2: list[int] = [101, 102]
    r2: int = pr_resolve(pids, nids, inids, cnt[0], path2, 2, 0)
    if r2 == 2:
        passed = passed + 1

    # Test 3: resolve non-existent path returns -1
    path3: list[int] = [101, 999]
    r3: int = pr_resolve(pids, nids, inids, cnt[0], path3, 2, 0)
    if r3 == (0 - 1):
        passed = passed + 1

    # Test 4: count entries in root
    cnt_root: int = pr_count_entries_in_dir(pids, cnt[0], 0)
    if cnt_root == 2:
        passed = passed + 1

    # Test 5: resolve /home/user/file.txt -> inode 4
    path4: list[int] = [101, 102, 104]
    r4: int = pr_resolve(pids, nids, inids, cnt[0], path4, 3, 0)
    if r4 == 4:
        passed = passed + 1

    return passed
