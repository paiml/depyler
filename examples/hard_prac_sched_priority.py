"""Priority queue scheduler simulation.

Uses a max-heap stored in a flat list for priority-based task scheduling.
Higher priority value = higher priority.
"""


def pq_init(capacity: int) -> list[int]:
    """Initialize with -1."""
    result: list[int] = []
    i: int = 0
    while i < capacity:
        result.append(0 - 1)
        i = i + 1
    return result


def pq_swap(arr: list[int], i: int, j: int) -> int:
    """Swap elements at i and j. Returns 0."""
    tmp: int = arr[i]
    arr[i] = arr[j]
    arr[j] = tmp
    return 0


def pq_sift_up(priorities: list[int], tasks: list[int], idx: int) -> int:
    """Sift up element at idx to maintain max-heap. Returns final index."""
    pos: int = idx
    while pos > 0:
        parent: int = (pos - 1) // 2
        p_val: int = priorities[pos]
        par_val: int = priorities[parent]
        if p_val > par_val:
            pq_swap(priorities, pos, parent)
            pq_swap(tasks, pos, parent)
            pos = parent
        else:
            return pos
    return pos


def pq_sift_down(priorities: list[int], tasks: list[int], idx: int, size: int) -> int:
    """Sift down element at idx. Returns final index."""
    pos: int = idx
    while pos < size:
        left: int = 2 * pos + 1
        right: int = 2 * pos + 2
        largest: int = pos
        if left < size:
            l_val: int = priorities[left]
            lg_val: int = priorities[largest]
            if l_val > lg_val:
                largest = left
        if right < size:
            r_val: int = priorities[right]
            lg_val2: int = priorities[largest]
            if r_val > lg_val2:
                largest = right
        if largest != pos:
            pq_swap(priorities, pos, largest)
            pq_swap(tasks, pos, largest)
            pos = largest
        else:
            return pos
    return pos


def pq_insert(priorities: list[int], tasks: list[int],
              size_arr: list[int], priority: int, task_id: int) -> int:
    """Insert task with priority. size_arr[0] is current size. Returns 1 on success."""
    sz: int = size_arr[0]
    priorities[sz] = priority
    tasks[sz] = task_id
    size_arr[0] = sz + 1
    pq_sift_up(priorities, tasks, sz)
    return 1


def pq_extract_max(priorities: list[int], tasks: list[int],
                   size_arr: list[int]) -> int:
    """Extract highest priority task. Returns task_id or -1 if empty."""
    sz: int = size_arr[0]
    if sz == 0:
        return 0 - 1
    result: int = tasks[0]
    new_sz: int = sz - 1
    priorities[0] = priorities[new_sz]
    tasks[0] = tasks[new_sz]
    priorities[new_sz] = 0 - 1
    tasks[new_sz] = 0 - 1
    size_arr[0] = new_sz
    if new_sz > 0:
        pq_sift_down(priorities, tasks, 0, new_sz)
    return result


def pq_peek(tasks: list[int], size_arr: list[int]) -> int:
    """Peek at highest priority task without removing. Returns -1 if empty."""
    sz: int = size_arr[0]
    if sz == 0:
        return 0 - 1
    result: int = tasks[0]
    return result


def test_module() -> int:
    """Test priority queue scheduler."""
    passed: int = 0
    cap: int = 10
    priorities: list[int] = pq_init(cap)
    tasks: list[int] = pq_init(cap)
    size_arr: list[int] = [0]

    # Test 1: insert and peek
    pq_insert(priorities, tasks, size_arr, 5, 100)
    top: int = pq_peek(tasks, size_arr)
    if top == 100:
        passed = passed + 1

    # Test 2: higher priority goes to top
    pq_insert(priorities, tasks, size_arr, 10, 200)
    top2: int = pq_peek(tasks, size_arr)
    if top2 == 200:
        passed = passed + 1

    # Test 3: extract max returns highest priority
    extracted: int = pq_extract_max(priorities, tasks, size_arr)
    if extracted == 200:
        passed = passed + 1

    # Test 4: next extract returns next highest
    pq_insert(priorities, tasks, size_arr, 3, 300)
    pq_insert(priorities, tasks, size_arr, 8, 400)
    ext2: int = pq_extract_max(priorities, tasks, size_arr)
    if ext2 == 400:
        passed = passed + 1

    # Test 5: empty extract returns -1
    pq_extract_max(priorities, tasks, size_arr)
    pq_extract_max(priorities, tasks, size_arr)
    empty: int = pq_extract_max(priorities, tasks, size_arr)
    if empty == (0 - 1):
        passed = passed + 1

    return passed
