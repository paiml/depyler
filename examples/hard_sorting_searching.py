"""Pathological sorting and searching patterns for transpiler stress testing.

Tests: insertion sort, selection sort, iterative merge sort, iterative quicksort,
Dutch national flag, 3-way partition, binary search variants, interpolation search,
exponential search, ternary search, quickselect, merge k sorted, chunk sort,
counting sort, radix sort LSD, heap operations, topological sort, inversion count,
run-length encoding/decoding, naive pattern match, KMP preprocessing.

All functions are pure: no imports, no I/O, complete type annotations.
"""


# ---------------------------------------------------------------------------
# 1. Insertion Sort
# ---------------------------------------------------------------------------
def insertion_sort(arr: list[int]) -> list[int]:
    """In-place style insertion sort on a copy."""
    result: list[int] = []
    for val in arr:
        result.append(val)
    n: int = len(result)
    i: int = 1
    while i < n:
        key: int = result[i]
        j: int = i - 1
        while j >= 0 and result[j] > key:
            result[j + 1] = result[j]
            j -= 1
        result[j + 1] = key
        i += 1
    return result


def test_insertion_sort() -> int:
    r1: list[int] = insertion_sort([5, 3, 8, 1, 2])
    r2: list[int] = insertion_sort([])
    r3: list[int] = insertion_sort([1])
    r4: list[int] = insertion_sort([3, 3, 3])
    r5: list[int] = insertion_sort([5, 4, 3, 2, 1])
    ok: int = 0
    if r1 == [1, 2, 3, 5, 8]:
        ok += 1
    if r2 == []:
        ok += 1
    if r3 == [1]:
        ok += 1
    if r4 == [3, 3, 3]:
        ok += 1
    if r5 == [1, 2, 3, 4, 5]:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 2. Selection Sort
# ---------------------------------------------------------------------------
def selection_sort(arr: list[int]) -> list[int]:
    """Selection sort with explicit min-finding and swap."""
    result: list[int] = []
    for val in arr:
        result.append(val)
    n: int = len(result)
    i: int = 0
    while i < n - 1:
        min_idx: int = i
        j: int = i + 1
        while j < n:
            if result[j] < result[min_idx]:
                min_idx = j
            j += 1
        if min_idx != i:
            tmp: int = result[i]
            result[i] = result[min_idx]
            result[min_idx] = tmp
        i += 1
    return result


def test_selection_sort() -> int:
    r1: list[int] = selection_sort([9, 1, 4, 7, 2])
    r2: list[int] = selection_sort([1, 2, 3])
    r3: list[int] = selection_sort([])
    r4: list[int] = selection_sort([100, -5, 0, 42])
    ok: int = 0
    if r1 == [1, 2, 4, 7, 9]:
        ok += 1
    if r2 == [1, 2, 3]:
        ok += 1
    if r3 == []:
        ok += 1
    if r4 == [-5, 0, 42, 100]:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 3. Iterative Merge Sort (bottom-up)
# ---------------------------------------------------------------------------
def merge_two(arr: list[int], left: int, mid: int, right: int) -> list[int]:
    """Merge two sorted sub-arrays within arr, return new array."""
    merged: list[int] = []
    for val in arr:
        merged.append(val)
    temp: list[int] = []
    i: int = left
    j: int = mid + 1
    while i <= mid and j <= right:
        if arr[i] <= arr[j]:
            temp.append(arr[i])
            i += 1
        else:
            temp.append(arr[j])
            j += 1
    while i <= mid:
        temp.append(arr[i])
        i += 1
    while j <= right:
        temp.append(arr[j])
        j += 1
    k: int = 0
    while k < len(temp):
        merged[left + k] = temp[k]
        k += 1
    return merged


def iterative_merge_sort(arr: list[int]) -> list[int]:
    """Bottom-up merge sort without recursion."""
    result: list[int] = []
    for val in arr:
        result.append(val)
    n: int = len(result)
    if n <= 1:
        return result
    width: int = 1
    while width < n:
        left: int = 0
        while left < n:
            mid: int = left + width - 1
            right: int = left + 2 * width - 1
            if mid >= n:
                mid = n - 1
            if right >= n:
                right = n - 1
            if mid < right:
                result = merge_two(result, left, mid, right)
            left += 2 * width
        width *= 2
    return result


def test_iterative_merge_sort() -> int:
    r1: list[int] = iterative_merge_sort([8, 3, 1, 5, 2, 7, 4, 6])
    r2: list[int] = iterative_merge_sort([])
    r3: list[int] = iterative_merge_sort([42])
    r4: list[int] = iterative_merge_sort([2, 1])
    r5: list[int] = iterative_merge_sort([5, 5, 5, 5])
    ok: int = 0
    if r1 == [1, 2, 3, 4, 5, 6, 7, 8]:
        ok += 1
    if r2 == []:
        ok += 1
    if r3 == [42]:
        ok += 1
    if r4 == [1, 2]:
        ok += 1
    if r5 == [5, 5, 5, 5]:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 4. Iterative Quicksort (explicit stack)
# ---------------------------------------------------------------------------
def partition(arr: list[int], low: int, high: int) -> int:
    """Lomuto partition scheme, modifies arr in place."""
    pivot: int = arr[high]
    i: int = low - 1
    j: int = low
    while j < high:
        if arr[j] <= pivot:
            i += 1
            tmp: int = arr[i]
            arr[i] = arr[j]
            arr[j] = tmp
        j += 1
    tmp2: int = arr[i + 1]
    arr[i + 1] = arr[high]
    arr[high] = tmp2
    return i + 1


def iterative_quicksort(arr: list[int]) -> list[int]:
    """Quicksort using explicit stack instead of recursion."""
    result: list[int] = []
    for val in arr:
        result.append(val)
    n: int = len(result)
    if n <= 1:
        return result
    stack: list[int] = []
    stack.append(0)
    stack.append(n - 1)
    while len(stack) > 0:
        high: int = stack.pop()
        low: int = stack.pop()
        if low < high:
            p: int = partition(result, low, high)
            if p - 1 > low:
                stack.append(low)
                stack.append(p - 1)
            if p + 1 < high:
                stack.append(p + 1)
                stack.append(high)
    return result


def test_iterative_quicksort() -> int:
    r1: list[int] = iterative_quicksort([10, 7, 8, 9, 1, 5])
    r2: list[int] = iterative_quicksort([])
    r3: list[int] = iterative_quicksort([1])
    r4: list[int] = iterative_quicksort([3, 2, 1])
    ok: int = 0
    if r1 == [1, 5, 7, 8, 9, 10]:
        ok += 1
    if r2 == []:
        ok += 1
    if r3 == [1]:
        ok += 1
    if r4 == [1, 2, 3]:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 5. Dutch National Flag (3-way partition by value)
# ---------------------------------------------------------------------------
def dutch_national_flag(arr: list[int], pivot: int) -> list[int]:
    """Partition into <pivot, ==pivot, >pivot regions."""
    result: list[int] = []
    for val in arr:
        result.append(val)
    low: int = 0
    mid: int = 0
    high: int = len(result) - 1
    while mid <= high:
        if result[mid] < pivot:
            tmp: int = result[low]
            result[low] = result[mid]
            result[mid] = tmp
            low += 1
            mid += 1
        elif result[mid] == pivot:
            mid += 1
        else:
            tmp2: int = result[mid]
            result[mid] = result[high]
            result[high] = tmp2
            high -= 1
    return result


def test_dutch_national_flag() -> int:
    r1: list[int] = dutch_national_flag([2, 0, 1, 2, 0, 1, 1], 1)
    ok: int = 0
    # Check: all <1 come first, then ==1, then >1
    phase: int = 0
    good: bool = True
    for val in r1:
        if phase == 0:
            if val == 1:
                phase = 1
            elif val > 1:
                phase = 2
        elif phase == 1:
            if val < 1:
                good = False
            elif val > 1:
                phase = 2
        elif phase == 2:
            if val <= 1:
                good = False
    if good and len(r1) == 7:
        ok += 1
    r2: list[int] = dutch_national_flag([], 5)
    if r2 == []:
        ok += 1
    r3: list[int] = dutch_national_flag([1, 1, 1], 1)
    if r3 == [1, 1, 1]:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 6. Three-way partition (generalized)
# ---------------------------------------------------------------------------
def three_way_partition(arr: list[int], lo_val: int, hi_val: int) -> list[int]:
    """Partition into <lo_val, [lo_val..hi_val], >hi_val."""
    result: list[int] = []
    for val in arr:
        result.append(val)
    n: int = len(result)
    if n == 0:
        return result
    start: int = 0
    end: int = n - 1
    i: int = 0
    while i <= end:
        if result[i] < lo_val:
            tmp: int = result[i]
            result[i] = result[start]
            result[start] = tmp
            start += 1
            i += 1
        elif result[i] > hi_val:
            tmp2: int = result[i]
            result[i] = result[end]
            result[end] = tmp2
            end -= 1
        else:
            i += 1
    return result


def test_three_way_partition() -> int:
    r: list[int] = three_way_partition([1, 14, 5, 20, 4, 2, 54, 20, 87, 98, 3, 1, 32], 10, 20)
    ok: int = 0
    phase: int = 0
    valid: bool = True
    for val in r:
        if phase == 0:
            if 10 <= val <= 20:
                phase = 1
            elif val > 20:
                phase = 2
        elif phase == 1:
            if val < 10:
                valid = False
            elif val > 20:
                phase = 2
        elif phase == 2:
            if val <= 20:
                valid = False
    if valid:
        ok += 1
    r2: list[int] = three_way_partition([], 1, 5)
    if r2 == []:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 7. Binary Search - Lower Bound
# ---------------------------------------------------------------------------
def lower_bound(arr: list[int], target: int) -> int:
    """First position where target could be inserted (leftmost)."""
    lo: int = 0
    hi: int = len(arr)
    while lo < hi:
        mid: int = (lo + hi) // 2
        if arr[mid] < target:
            lo = mid + 1
        else:
            hi = mid
    return lo


def test_lower_bound() -> int:
    ok: int = 0
    if lower_bound([1, 2, 4, 4, 4, 7, 9], 4) == 2:
        ok += 1
    if lower_bound([1, 2, 4, 4, 4, 7, 9], 5) == 5:
        ok += 1
    if lower_bound([1, 2, 3], 0) == 0:
        ok += 1
    if lower_bound([1, 2, 3], 10) == 3:
        ok += 1
    if lower_bound([], 5) == 0:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 8. Binary Search - Upper Bound
# ---------------------------------------------------------------------------
def upper_bound(arr: list[int], target: int) -> int:
    """First position strictly greater than target."""
    lo: int = 0
    hi: int = len(arr)
    while lo < hi:
        mid: int = (lo + hi) // 2
        if arr[mid] <= target:
            lo = mid + 1
        else:
            hi = mid
    return lo


def test_upper_bound() -> int:
    ok: int = 0
    if upper_bound([1, 2, 4, 4, 4, 7, 9], 4) == 5:
        ok += 1
    if upper_bound([1, 2, 4, 4, 4, 7, 9], 0) == 0:
        ok += 1
    if upper_bound([1, 2, 3], 3) == 3:
        ok += 1
    if upper_bound([], 1) == 0:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 9. Search Insert Position
# ---------------------------------------------------------------------------
def search_insert_position(arr: list[int], target: int) -> int:
    """Index where target is found or would be inserted."""
    lo: int = 0
    hi: int = len(arr)
    while lo < hi:
        mid: int = (lo + hi) // 2
        if arr[mid] < target:
            lo = mid + 1
        else:
            hi = mid
    return lo


def test_search_insert_position() -> int:
    ok: int = 0
    if search_insert_position([1, 3, 5, 6], 5) == 2:
        ok += 1
    if search_insert_position([1, 3, 5, 6], 2) == 1:
        ok += 1
    if search_insert_position([1, 3, 5, 6], 7) == 4:
        ok += 1
    if search_insert_position([1, 3, 5, 6], 0) == 0:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 10. Interpolation Search
# ---------------------------------------------------------------------------
def interpolation_search(arr: list[int], target: int) -> int:
    """Interpolation search for uniformly distributed sorted arrays."""
    lo: int = 0
    hi: int = len(arr) - 1
    while lo <= hi and len(arr) > 0:
        if arr[hi] == arr[lo]:
            if arr[lo] == target:
                return lo
            else:
                return -1
        pos: int = lo + ((target - arr[lo]) * (hi - lo)) // (arr[hi] - arr[lo])
        if pos < lo or pos > hi:
            return -1
        if arr[pos] == target:
            return pos
        elif arr[pos] < target:
            lo = pos + 1
        else:
            hi = pos - 1
    return -1


def test_interpolation_search() -> int:
    ok: int = 0
    data: list[int] = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100]
    if interpolation_search(data, 50) == 4:
        ok += 1
    if interpolation_search(data, 10) == 0:
        ok += 1
    if interpolation_search(data, 100) == 9:
        ok += 1
    if interpolation_search(data, 55) == -1:
        ok += 1
    if interpolation_search([], 5) == -1:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 11. Exponential Search
# ---------------------------------------------------------------------------
def binary_search_range(arr: list[int], target: int, lo: int, hi: int) -> int:
    """Binary search within [lo, hi] range."""
    while lo <= hi:
        mid: int = (lo + hi) // 2
        if arr[mid] == target:
            return mid
        elif arr[mid] < target:
            lo = mid + 1
        else:
            hi = mid - 1
    return -1


def exponential_search(arr: list[int], target: int) -> int:
    """Exponential search: find range then binary search."""
    n: int = len(arr)
    if n == 0:
        return -1
    if arr[0] == target:
        return 0
    bound: int = 1
    while bound < n and arr[bound] <= target:
        bound *= 2
    lo: int = bound // 2
    hi: int = bound
    if hi >= n:
        hi = n - 1
    return binary_search_range(arr, target, lo, hi)


def test_exponential_search() -> int:
    ok: int = 0
    data: list[int] = [2, 3, 4, 10, 40, 50, 60, 70, 80, 90]
    if exponential_search(data, 10) == 3:
        ok += 1
    if exponential_search(data, 2) == 0:
        ok += 1
    if exponential_search(data, 90) == 9:
        ok += 1
    if exponential_search(data, 5) == -1:
        ok += 1
    if exponential_search([], 1) == -1:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 12. Ternary Search (discrete)
# ---------------------------------------------------------------------------
def ternary_search_max(arr: list[int]) -> int:
    """Find index of maximum in a unimodal array using ternary search."""
    n: int = len(arr)
    if n == 0:
        return -1
    if n == 1:
        return 0
    lo: int = 0
    hi: int = n - 1
    while hi - lo > 2:
        m1: int = lo + (hi - lo) // 3
        m2: int = hi - (hi - lo) // 3
        if arr[m1] < arr[m2]:
            lo = m1 + 1
        else:
            hi = m2 - 1
    best: int = lo
    i: int = lo + 1
    while i <= hi:
        if arr[i] > arr[best]:
            best = i
        i += 1
    return best


def test_ternary_search_max() -> int:
    ok: int = 0
    if ternary_search_max([1, 3, 5, 7, 9, 8, 6, 4, 2]) == 4:
        ok += 1
    if ternary_search_max([1, 10, 1]) == 1:
        ok += 1
    if ternary_search_max([42]) == 0:
        ok += 1
    if ternary_search_max([]) == -1:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 13. Quickselect (k-th smallest)
# ---------------------------------------------------------------------------
def quickselect_partition(arr: list[int], lo: int, hi: int) -> int:
    """Partition for quickselect, returns pivot index."""
    pivot: int = arr[hi]
    i: int = lo
    j: int = lo
    while j < hi:
        if arr[j] <= pivot:
            tmp: int = arr[i]
            arr[i] = arr[j]
            arr[j] = tmp
            i += 1
        j += 1
    tmp2: int = arr[i]
    arr[i] = arr[hi]
    arr[hi] = tmp2
    return i


def quickselect(arr: list[int], k: int) -> int:
    """Find k-th smallest element (0-indexed). Modifies a copy."""
    work: list[int] = []
    for val in arr:
        work.append(val)
    lo: int = 0
    hi: int = len(work) - 1
    while lo <= hi:
        p: int = quickselect_partition(work, lo, hi)
        if p == k:
            return work[p]
        elif p < k:
            lo = p + 1
        else:
            hi = p - 1
    return -1


def test_quickselect() -> int:
    ok: int = 0
    if quickselect([7, 10, 4, 3, 20, 15], 0) == 3:
        ok += 1
    if quickselect([7, 10, 4, 3, 20, 15], 3) == 10:
        ok += 1
    if quickselect([7, 10, 4, 3, 20, 15], 5) == 20:
        ok += 1
    if quickselect([1], 0) == 1:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 14. Merge K Sorted Lists
# ---------------------------------------------------------------------------
def merge_two_sorted(a: list[int], b: list[int]) -> list[int]:
    """Merge two sorted lists into one sorted list."""
    result: list[int] = []
    i: int = 0
    j: int = 0
    while i < len(a) and j < len(b):
        if a[i] <= b[j]:
            result.append(a[i])
            i += 1
        else:
            result.append(b[j])
            j += 1
    while i < len(a):
        result.append(a[i])
        i += 1
    while j < len(b):
        result.append(b[j])
        j += 1
    return result


def merge_k_sorted(lists: list[list[int]]) -> list[int]:
    """Merge k sorted lists using pairwise merge (tournament style)."""
    if len(lists) == 0:
        return []
    current: list[list[int]] = []
    for lst in lists:
        cloned: list[int] = []
        for val in lst:
            cloned.append(val)
        current.append(cloned)
    while len(current) > 1:
        next_round: list[list[int]] = []
        i: int = 0
        while i < len(current):
            if i + 1 < len(current):
                merged: list[int] = merge_two_sorted(current[i], current[i + 1])
                next_round.append(merged)
            else:
                next_round.append(current[i])
            i += 2
        current = next_round
    return current[0]


def test_merge_k_sorted() -> int:
    ok: int = 0
    r1: list[int] = merge_k_sorted([[1, 4, 7], [2, 5, 8], [3, 6, 9]])
    if r1 == [1, 2, 3, 4, 5, 6, 7, 8, 9]:
        ok += 1
    r2: list[int] = merge_k_sorted([[1], [2], [3], [4]])
    if r2 == [1, 2, 3, 4]:
        ok += 1
    r3: list[int] = merge_k_sorted([])
    if r3 == []:
        ok += 1
    r4: list[int] = merge_k_sorted([[10, 20, 30]])
    if r4 == [10, 20, 30]:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 15. External Sort Simulation (chunk-based)
# ---------------------------------------------------------------------------
def chunk_sort(arr: list[int], chunk_size: int) -> list[int]:
    """Sort array in chunks then merge all chunks."""
    n: int = len(arr)
    if n == 0:
        return []
    chunks: list[list[int]] = []
    i: int = 0
    while i < n:
        end: int = i + chunk_size
        if end > n:
            end = n
        chunk: list[int] = []
        j: int = i
        while j < end:
            chunk.append(arr[j])
            j += 1
        chunk = insertion_sort(chunk)
        chunks.append(chunk)
        i = end
    return merge_k_sorted(chunks)


def test_chunk_sort() -> int:
    ok: int = 0
    r1: list[int] = chunk_sort([9, 3, 7, 1, 8, 2, 6, 4, 5], 3)
    if r1 == [1, 2, 3, 4, 5, 6, 7, 8, 9]:
        ok += 1
    r2: list[int] = chunk_sort([], 5)
    if r2 == []:
        ok += 1
    r3: list[int] = chunk_sort([5, 1, 3], 10)
    if r3 == [1, 3, 5]:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 16. Counting Sort
# ---------------------------------------------------------------------------
def counting_sort(arr: list[int], max_val: int) -> list[int]:
    """Counting sort for non-negative integers up to max_val."""
    if len(arr) == 0:
        return []
    count: list[int] = []
    i: int = 0
    while i <= max_val:
        count.append(0)
        i += 1
    for val in arr:
        if 0 <= val <= max_val:
            count[val] += 1
    result: list[int] = []
    idx: int = 0
    while idx <= max_val:
        c: int = 0
        while c < count[idx]:
            result.append(idx)
            c += 1
        idx += 1
    return result


def test_counting_sort() -> int:
    ok: int = 0
    r1: list[int] = counting_sort([4, 2, 2, 8, 3, 3, 1], 9)
    if r1 == [1, 2, 2, 3, 3, 4, 8]:
        ok += 1
    r2: list[int] = counting_sort([], 5)
    if r2 == []:
        ok += 1
    r3: list[int] = counting_sort([0, 0, 0], 1)
    if r3 == [0, 0, 0]:
        ok += 1
    r4: list[int] = counting_sort([5, 5, 5], 5)
    if r4 == [5, 5, 5]:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 17. Radix Sort (LSD, base 10, non-negative)
# ---------------------------------------------------------------------------
def counting_sort_by_digit(arr: list[int], exp: int) -> list[int]:
    """Stable counting sort by a specific digit position."""
    n: int = len(arr)
    if n == 0:
        return []
    output: list[int] = []
    i: int = 0
    while i < n:
        output.append(0)
        i += 1
    count: list[int] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    for val in arr:
        digit: int = (val // exp) % 10
        count[digit] += 1
    j: int = 1
    while j < 10:
        count[j] += count[j - 1]
        j += 1
    k: int = n - 1
    while k >= 0:
        digit2: int = (arr[k] // exp) % 10
        count[digit2] -= 1
        output[count[digit2]] = arr[k]
        k -= 1
    return output


def radix_sort_lsd(arr: list[int]) -> list[int]:
    """LSD radix sort for non-negative integers."""
    if len(arr) == 0:
        return []
    max_val: int = arr[0]
    for val in arr:
        if val > max_val:
            max_val = val
    result: list[int] = []
    for val in arr:
        result.append(val)
    exp: int = 1
    while max_val // exp > 0:
        result = counting_sort_by_digit(result, exp)
        exp *= 10
    return result


def test_radix_sort_lsd() -> int:
    ok: int = 0
    r1: list[int] = radix_sort_lsd([170, 45, 75, 90, 802, 24, 2, 66])
    if r1 == [2, 24, 45, 66, 75, 90, 170, 802]:
        ok += 1
    r2: list[int] = radix_sort_lsd([])
    if r2 == []:
        ok += 1
    r3: list[int] = radix_sort_lsd([1])
    if r3 == [1]:
        ok += 1
    r4: list[int] = radix_sort_lsd([999, 1, 100, 10])
    if r4 == [1, 10, 100, 999]:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 18. Heap - Sift Down
# ---------------------------------------------------------------------------
def sift_down(arr: list[int], n: int, i: int) -> list[int]:
    """Max-heap sift down at index i, modifies copy."""
    result: list[int] = []
    for val in arr:
        result.append(val)
    idx: int = i
    while True:
        largest: int = idx
        left: int = 2 * idx + 1
        right: int = 2 * idx + 2
        if left < n and result[left] > result[largest]:
            largest = left
        if right < n and result[right] > result[largest]:
            largest = right
        if largest == idx:
            break
        tmp: int = result[idx]
        result[idx] = result[largest]
        result[largest] = tmp
        idx = largest
    return result


def test_sift_down() -> int:
    ok: int = 0
    r: list[int] = sift_down([1, 5, 3, 4, 2], 5, 0)
    if r[0] == 5:
        ok += 1
    r2: list[int] = sift_down([10, 5, 3], 3, 0)
    if r2[0] == 10:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 19. Heap - Sift Up
# ---------------------------------------------------------------------------
def sift_up(arr: list[int], i: int) -> list[int]:
    """Max-heap sift up from index i, modifies copy."""
    result: list[int] = []
    for val in arr:
        result.append(val)
    idx: int = i
    while idx > 0:
        parent: int = (idx - 1) // 2
        if result[idx] > result[parent]:
            tmp: int = result[idx]
            result[idx] = result[parent]
            result[parent] = tmp
            idx = parent
        else:
            break
    return result


def test_sift_up() -> int:
    ok: int = 0
    r: list[int] = sift_up([5, 3, 4, 1, 2, 10], 5)
    if r[0] == 10:
        ok += 1
    r2: list[int] = sift_up([10, 5, 3], 2)
    if r2[0] == 10:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 20. Heapify (build max-heap)
# ---------------------------------------------------------------------------
def heapify(arr: list[int]) -> list[int]:
    """Build a max-heap from array using bottom-up sift-down."""
    result: list[int] = []
    for val in arr:
        result.append(val)
    n: int = len(result)
    i: int = n // 2 - 1
    while i >= 0:
        idx: int = i
        done: bool = False
        while not done:
            largest: int = idx
            left: int = 2 * idx + 1
            right: int = 2 * idx + 2
            if left < n and result[left] > result[largest]:
                largest = left
            if right < n and result[right] > result[largest]:
                largest = right
            if largest == idx:
                done = True
            else:
                tmp: int = result[idx]
                result[idx] = result[largest]
                result[largest] = tmp
                idx = largest
        i -= 1
    return result


def test_heapify() -> int:
    ok: int = 0
    r: list[int] = heapify([3, 1, 6, 5, 2, 4])
    if r[0] == 6:
        ok += 1
    r2: list[int] = heapify([1])
    if r2[0] == 1:
        ok += 1
    r3: list[int] = heapify([1, 2, 3, 4, 5])
    if r3[0] == 5:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 21. Heap Sort
# ---------------------------------------------------------------------------
def heap_sort(arr: list[int]) -> list[int]:
    """Heap sort using in-place heapify then extract."""
    result: list[int] = []
    for val in arr:
        result.append(val)
    n: int = len(result)
    if n <= 1:
        return result
    # Build max heap
    i: int = n // 2 - 1
    while i >= 0:
        idx: int = i
        size: int = n
        cont: bool = True
        while cont:
            largest: int = idx
            left: int = 2 * idx + 1
            right: int = 2 * idx + 2
            if left < size and result[left] > result[largest]:
                largest = left
            if right < size and result[right] > result[largest]:
                largest = right
            if largest == idx:
                cont = False
            else:
                tmp: int = result[idx]
                result[idx] = result[largest]
                result[largest] = tmp
                idx = largest
        i -= 1
    # Extract elements
    end: int = n - 1
    while end > 0:
        tmp2: int = result[0]
        result[0] = result[end]
        result[end] = tmp2
        idx2: int = 0
        cont2: bool = True
        while cont2:
            largest2: int = idx2
            left2: int = 2 * idx2 + 1
            right2: int = 2 * idx2 + 2
            if left2 < end and result[left2] > result[largest2]:
                largest2 = left2
            if right2 < end and result[right2] > result[largest2]:
                largest2 = right2
            if largest2 == idx2:
                cont2 = False
            else:
                tmp3: int = result[idx2]
                result[idx2] = result[largest2]
                result[largest2] = tmp3
                idx2 = largest2
        end -= 1
    return result


def test_heap_sort() -> int:
    ok: int = 0
    r1: list[int] = heap_sort([12, 11, 13, 5, 6, 7])
    if r1 == [5, 6, 7, 11, 12, 13]:
        ok += 1
    r2: list[int] = heap_sort([])
    if r2 == []:
        ok += 1
    r3: list[int] = heap_sort([1])
    if r3 == [1]:
        ok += 1
    r4: list[int] = heap_sort([3, 1, 2])
    if r4 == [1, 2, 3]:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 22. Topological Sort (Kahn's algorithm, adjacency list as flat pairs)
# ---------------------------------------------------------------------------
def topological_sort(num_nodes: int, edges: list[list[int]]) -> list[int]:
    """Kahn's algorithm. edges[i] = [from, to]. Returns order or empty if cycle."""
    in_degree: list[int] = []
    i: int = 0
    while i < num_nodes:
        in_degree.append(0)
        i += 1
    # Build adjacency + in-degree
    adj: list[list[int]] = []
    k: int = 0
    while k < num_nodes:
        adj.append([])
        k += 1
    for edge in edges:
        src: int = edge[0]
        dst: int = edge[1]
        adj[src].append(dst)
        in_degree[dst] += 1
    # Find all sources
    queue: list[int] = []
    q: int = 0
    while q < num_nodes:
        if in_degree[q] == 0:
            queue.append(q)
        q += 1
    result: list[int] = []
    while len(queue) > 0:
        node: int = queue[0]
        # Shift left (simulate dequeue)
        new_queue: list[int] = []
        qi: int = 1
        while qi < len(queue):
            new_queue.append(queue[qi])
            qi += 1
        queue = new_queue
        result.append(node)
        for neighbor in adj[node]:
            in_degree[neighbor] -= 1
            if in_degree[neighbor] == 0:
                queue.append(neighbor)
    if len(result) != num_nodes:
        return []
    return result


def test_topological_sort() -> int:
    ok: int = 0
    r1: list[int] = topological_sort(6, [[5, 2], [5, 0], [4, 0], [4, 1], [2, 3], [3, 1]])
    if len(r1) == 6:
        # Verify ordering constraints
        pos: list[int] = [0, 0, 0, 0, 0, 0]
        idx: int = 0
        while idx < 6:
            pos[r1[idx]] = idx
            idx += 1
        valid: bool = True
        if pos[5] > pos[2]:
            valid = False
        if pos[5] > pos[0]:
            valid = False
        if pos[4] > pos[0]:
            valid = False
        if pos[4] > pos[1]:
            valid = False
        if pos[2] > pos[3]:
            valid = False
        if pos[3] > pos[1]:
            valid = False
        if valid:
            ok += 1
    # Cycle detection
    r2: list[int] = topological_sort(3, [[0, 1], [1, 2], [2, 0]])
    if r2 == []:
        ok += 1
    r3: list[int] = topological_sort(1, [])
    if r3 == [0]:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 23. Count Inversions (merge sort based)
# ---------------------------------------------------------------------------
def merge_count(arr: list[int], temp: list[int], left: int, mid: int, right: int) -> int:
    """Merge and count inversions across halves."""
    i: int = left
    j: int = mid + 1
    k: int = left
    inv_count: int = 0
    while i <= mid and j <= right:
        if arr[i] <= arr[j]:
            temp[k] = arr[i]
            i += 1
        else:
            temp[k] = arr[j]
            inv_count += (mid - i + 1)
            j += 1
        k += 1
    while i <= mid:
        temp[k] = arr[i]
        i += 1
        k += 1
    while j <= right:
        temp[k] = arr[j]
        j += 1
        k += 1
    copy_idx: int = left
    while copy_idx <= right:
        arr[copy_idx] = temp[copy_idx]
        copy_idx += 1
    return inv_count


def count_inversions(arr: list[int]) -> int:
    """Count inversions using iterative merge sort approach."""
    n: int = len(arr)
    if n <= 1:
        return 0
    work: list[int] = []
    temp: list[int] = []
    for val in arr:
        work.append(val)
        temp.append(val)
    total: int = 0
    width: int = 1
    while width < n:
        left: int = 0
        while left < n:
            mid: int = left + width - 1
            right: int = left + 2 * width - 1
            if mid >= n:
                mid = n - 1
            if right >= n:
                right = n - 1
            if mid < right:
                total += merge_count(work, temp, left, mid, right)
            left += 2 * width
        width *= 2
    return total


def test_count_inversions() -> int:
    ok: int = 0
    if count_inversions([1, 20, 6, 4, 5]) == 5:
        ok += 1
    if count_inversions([1, 2, 3, 4, 5]) == 0:
        ok += 1
    if count_inversions([5, 4, 3, 2, 1]) == 10:
        ok += 1
    if count_inversions([]) == 0:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 24. Run-Length Encoding
# ---------------------------------------------------------------------------
def run_length_encode(arr: list[int]) -> list[list[int]]:
    """Encode runs as [[value, count], ...]."""
    if len(arr) == 0:
        return []
    result: list[list[int]] = []
    current: int = arr[0]
    count: int = 1
    i: int = 1
    while i < len(arr):
        if arr[i] == current:
            count += 1
        else:
            result.append([current, count])
            current = arr[i]
            count = 1
        i += 1
    result.append([current, count])
    return result


def test_run_length_encode() -> int:
    ok: int = 0
    r1: list[list[int]] = run_length_encode([1, 1, 2, 2, 2, 3, 1, 1])
    if r1 == [[1, 2], [2, 3], [3, 1], [1, 2]]:
        ok += 1
    r2: list[list[int]] = run_length_encode([])
    if r2 == []:
        ok += 1
    r3: list[list[int]] = run_length_encode([5])
    if r3 == [[5, 1]]:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 25. Run-Length Decoding
# ---------------------------------------------------------------------------
def run_length_decode(encoded: list[list[int]]) -> list[int]:
    """Decode [[value, count], ...] back to flat list."""
    result: list[int] = []
    for pair in encoded:
        val: int = pair[0]
        count: int = pair[1]
        i: int = 0
        while i < count:
            result.append(val)
            i += 1
    return result


def test_run_length_decode() -> int:
    ok: int = 0
    r1: list[int] = run_length_decode([[1, 2], [2, 3], [3, 1], [1, 2]])
    if r1 == [1, 1, 2, 2, 2, 3, 1, 1]:
        ok += 1
    r2: list[int] = run_length_decode([])
    if r2 == []:
        ok += 1
    r3: list[int] = run_length_decode([[7, 4]])
    if r3 == [7, 7, 7, 7]:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 26. RLE Roundtrip
# ---------------------------------------------------------------------------
def test_rle_roundtrip() -> int:
    ok: int = 0
    original: list[int] = [4, 4, 4, 1, 1, 2, 2, 2, 2, 3]
    encoded: list[list[int]] = run_length_encode(original)
    decoded: list[int] = run_length_decode(encoded)
    if decoded == original:
        ok += 1
    empty_rt: list[int] = run_length_decode(run_length_encode([]))
    if empty_rt == []:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 27. Naive Pattern Match
# ---------------------------------------------------------------------------
def naive_pattern_match(text: list[int], pattern: list[int]) -> list[int]:
    """Find all starting indices where pattern appears in text."""
    result: list[int] = []
    n: int = len(text)
    m: int = len(pattern)
    if m == 0 or m > n:
        return result
    i: int = 0
    while i <= n - m:
        match: bool = True
        j: int = 0
        while j < m:
            if text[i + j] != pattern[j]:
                match = False
                break
            j += 1
        if match:
            result.append(i)
        i += 1
    return result


def test_naive_pattern_match() -> int:
    ok: int = 0
    r1: list[int] = naive_pattern_match([1, 2, 3, 1, 2, 3, 1], [1, 2, 3])
    if r1 == [0, 3]:
        ok += 1
    r2: list[int] = naive_pattern_match([1, 2, 3], [4, 5])
    if r2 == []:
        ok += 1
    r3: list[int] = naive_pattern_match([1, 1, 1, 1], [1, 1])
    if r3 == [0, 1, 2]:
        ok += 1
    r4: list[int] = naive_pattern_match([], [1])
    if r4 == []:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 28. KMP Failure Function (prefix table)
# ---------------------------------------------------------------------------
def kmp_failure(pattern: list[int]) -> list[int]:
    """Compute KMP failure function / prefix table."""
    m: int = len(pattern)
    if m == 0:
        return []
    fail: list[int] = []
    i: int = 0
    while i < m:
        fail.append(0)
        i += 1
    k: int = 0
    j: int = 1
    while j < m:
        while k > 0 and pattern[k] != pattern[j]:
            k = fail[k - 1]
        if pattern[k] == pattern[j]:
            k += 1
        fail[j] = k
        j += 1
    return fail


def test_kmp_failure() -> int:
    ok: int = 0
    r1: list[int] = kmp_failure([1, 2, 1, 2, 3])
    if r1 == [0, 0, 1, 2, 0]:
        ok += 1
    r2: list[int] = kmp_failure([1, 1, 1, 1])
    if r2 == [0, 1, 2, 3]:
        ok += 1
    r3: list[int] = kmp_failure([])
    if r3 == []:
        ok += 1
    r4: list[int] = kmp_failure([5])
    if r4 == [0]:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 29. KMP Search
# ---------------------------------------------------------------------------
def kmp_search(text: list[int], pattern: list[int]) -> list[int]:
    """KMP pattern matching, returns list of match start indices."""
    n: int = len(text)
    m: int = len(pattern)
    if m == 0:
        return []
    fail: list[int] = kmp_failure(pattern)
    result: list[int] = []
    j: int = 0
    i: int = 0
    while i < n:
        while j > 0 and pattern[j] != text[i]:
            j = fail[j - 1]
        if pattern[j] == text[i]:
            j += 1
        if j == m:
            result.append(i - m + 1)
            j = fail[j - 1]
        i += 1
    return result


def test_kmp_search() -> int:
    ok: int = 0
    r1: list[int] = kmp_search([1, 2, 1, 2, 1, 2, 3], [1, 2, 1])
    if r1 == [0, 2]:
        ok += 1
    r2: list[int] = kmp_search([1, 2, 3], [4])
    if r2 == []:
        ok += 1
    r3: list[int] = kmp_search([1, 1, 1, 1, 1], [1, 1])
    if r3 == [0, 1, 2, 3]:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 30. Shell Sort
# ---------------------------------------------------------------------------
def shell_sort(arr: list[int]) -> list[int]:
    """Shell sort with gap sequence n/2, n/4, ..., 1."""
    result: list[int] = []
    for val in arr:
        result.append(val)
    n: int = len(result)
    gap: int = n // 2
    while gap > 0:
        i: int = gap
        while i < n:
            temp: int = result[i]
            j: int = i
            while j >= gap and result[j - gap] > temp:
                result[j] = result[j - gap]
                j -= gap
            result[j] = temp
            i += 1
        gap //= 2
    return result


def test_shell_sort() -> int:
    ok: int = 0
    r1: list[int] = shell_sort([23, 12, 1, 8, 34, 54, 2, 3])
    if r1 == [1, 2, 3, 8, 12, 23, 34, 54]:
        ok += 1
    r2: list[int] = shell_sort([])
    if r2 == []:
        ok += 1
    r3: list[int] = shell_sort([5, 4, 3, 2, 1])
    if r3 == [1, 2, 3, 4, 5]:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 31. Comb Sort
# ---------------------------------------------------------------------------
def comb_sort(arr: list[int]) -> list[int]:
    """Comb sort with shrink factor 1.3 approximated as 10/13."""
    result: list[int] = []
    for val in arr:
        result.append(val)
    n: int = len(result)
    gap: int = n
    swapped: bool = True
    while gap > 1 or swapped:
        gap = (gap * 10) // 13
        if gap < 1:
            gap = 1
        swapped = False
        i: int = 0
        while i + gap < n:
            if result[i] > result[i + gap]:
                tmp: int = result[i]
                result[i] = result[i + gap]
                result[i + gap] = tmp
                swapped = True
            i += 1
    return result


def test_comb_sort() -> int:
    ok: int = 0
    r1: list[int] = comb_sort([8, 4, 1, 56, 3, 5, 7, 2])
    if r1 == [1, 2, 3, 4, 5, 7, 8, 56]:
        ok += 1
    r2: list[int] = comb_sort([])
    if r2 == []:
        ok += 1
    r3: list[int] = comb_sort([1])
    if r3 == [1]:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 32. Cocktail Shaker Sort
# ---------------------------------------------------------------------------
def cocktail_shaker_sort(arr: list[int]) -> list[int]:
    """Bidirectional bubble sort."""
    result: list[int] = []
    for val in arr:
        result.append(val)
    n: int = len(result)
    if n <= 1:
        return result
    start: int = 0
    end: int = n - 1
    swapped: bool = True
    while swapped:
        swapped = False
        i: int = start
        while i < end:
            if result[i] > result[i + 1]:
                tmp: int = result[i]
                result[i] = result[i + 1]
                result[i + 1] = tmp
                swapped = True
            i += 1
        if not swapped:
            break
        end -= 1
        swapped = False
        j: int = end
        while j > start:
            if result[j] < result[j - 1]:
                tmp2: int = result[j]
                result[j] = result[j - 1]
                result[j - 1] = tmp2
                swapped = True
            j -= 1
        start += 1
    return result


def test_cocktail_shaker_sort() -> int:
    ok: int = 0
    r1: list[int] = cocktail_shaker_sort([5, 1, 4, 2, 8, 0, 2])
    if r1 == [0, 1, 2, 2, 4, 5, 8]:
        ok += 1
    r2: list[int] = cocktail_shaker_sort([])
    if r2 == []:
        ok += 1
    r3: list[int] = cocktail_shaker_sort([3, 2, 1])
    if r3 == [1, 2, 3]:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 33. Gnome Sort
# ---------------------------------------------------------------------------
def gnome_sort(arr: list[int]) -> list[int]:
    """Gnome sort (stupid sort variant)."""
    result: list[int] = []
    for val in arr:
        result.append(val)
    n: int = len(result)
    pos: int = 0
    while pos < n:
        if pos == 0 or result[pos] >= result[pos - 1]:
            pos += 1
        else:
            tmp: int = result[pos]
            result[pos] = result[pos - 1]
            result[pos - 1] = tmp
            pos -= 1
    return result


def test_gnome_sort() -> int:
    ok: int = 0
    r1: list[int] = gnome_sort([34, 2, 10, -9, 1])
    if r1 == [-9, 1, 2, 10, 34]:
        ok += 1
    r2: list[int] = gnome_sort([])
    if r2 == []:
        ok += 1
    r3: list[int] = gnome_sort([1, 2, 3])
    if r3 == [1, 2, 3]:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 34. Pancake Sort
# ---------------------------------------------------------------------------
def flip(arr: list[int], k: int) -> list[int]:
    """Reverse first k+1 elements of arr (copy)."""
    result: list[int] = []
    for val in arr:
        result.append(val)
    lo: int = 0
    hi: int = k
    while lo < hi:
        tmp: int = result[lo]
        result[lo] = result[hi]
        result[hi] = tmp
        lo += 1
        hi -= 1
    return result


def pancake_sort(arr: list[int]) -> list[int]:
    """Pancake sort by finding max, flipping to front, flipping to position."""
    result: list[int] = []
    for val in arr:
        result.append(val)
    n: int = len(result)
    curr_size: int = n
    while curr_size > 1:
        max_idx: int = 0
        i: int = 1
        while i < curr_size:
            if result[i] > result[max_idx]:
                max_idx = i
            i += 1
        if max_idx != curr_size - 1:
            if max_idx != 0:
                result = flip(result, max_idx)
            result = flip(result, curr_size - 1)
        curr_size -= 1
    return result


def test_pancake_sort() -> int:
    ok: int = 0
    r1: list[int] = pancake_sort([3, 6, 2, 7, 4, 5, 1])
    if r1 == [1, 2, 3, 4, 5, 6, 7]:
        ok += 1
    r2: list[int] = pancake_sort([])
    if r2 == []:
        ok += 1
    r3: list[int] = pancake_sort([1])
    if r3 == [1]:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 35. Cycle Sort
# ---------------------------------------------------------------------------
def cycle_sort(arr: list[int]) -> list[int]:
    """Cycle sort - minimizes writes, O(n^2)."""
    result: list[int] = []
    for val in arr:
        result.append(val)
    n: int = len(result)
    cycle_start: int = 0
    while cycle_start < n - 1:
        item: int = result[cycle_start]
        pos: int = cycle_start
        i: int = cycle_start + 1
        while i < n:
            if result[i] < item:
                pos += 1
            i += 1
        if pos == cycle_start:
            cycle_start += 1
            continue
        while item == result[pos]:
            pos += 1
        if pos != cycle_start:
            tmp: int = result[pos]
            result[pos] = item
            item = tmp
        while pos != cycle_start:
            pos = cycle_start
            j: int = cycle_start + 1
            while j < n:
                if result[j] < item:
                    pos += 1
                j += 1
            while item == result[pos]:
                pos += 1
            if item != result[pos]:
                tmp2: int = result[pos]
                result[pos] = item
                item = tmp2
        cycle_start += 1
    return result


def test_cycle_sort() -> int:
    ok: int = 0
    r1: list[int] = cycle_sort([5, 2, 3, 1, 4])
    if r1 == [1, 2, 3, 4, 5]:
        ok += 1
    r2: list[int] = cycle_sort([])
    if r2 == []:
        ok += 1
    r3: list[int] = cycle_sort([1, 2, 3])
    if r3 == [1, 2, 3]:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 36. Binary Search - First and Last Occurrence
# ---------------------------------------------------------------------------
def first_occurrence(arr: list[int], target: int) -> int:
    """Find first occurrence of target, or -1."""
    lo: int = 0
    hi: int = len(arr) - 1
    result: int = -1
    while lo <= hi:
        mid: int = (lo + hi) // 2
        if arr[mid] == target:
            result = mid
            hi = mid - 1
        elif arr[mid] < target:
            lo = mid + 1
        else:
            hi = mid - 1
    return result


def last_occurrence(arr: list[int], target: int) -> int:
    """Find last occurrence of target, or -1."""
    lo: int = 0
    hi: int = len(arr) - 1
    result: int = -1
    while lo <= hi:
        mid: int = (lo + hi) // 2
        if arr[mid] == target:
            result = mid
            lo = mid + 1
        elif arr[mid] < target:
            lo = mid + 1
        else:
            hi = mid - 1
    return result


def test_first_last_occurrence() -> int:
    ok: int = 0
    data: list[int] = [1, 2, 2, 2, 3, 4, 4, 5]
    if first_occurrence(data, 2) == 1:
        ok += 1
    if last_occurrence(data, 2) == 3:
        ok += 1
    if first_occurrence(data, 4) == 5:
        ok += 1
    if last_occurrence(data, 4) == 6:
        ok += 1
    if first_occurrence(data, 6) == -1:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 37. Count Occurrences (using binary search)
# ---------------------------------------------------------------------------
def count_occurrences(arr: list[int], target: int) -> int:
    """Count occurrences of target in sorted array using binary search."""
    first: int = first_occurrence(arr, target)
    if first == -1:
        return 0
    last: int = last_occurrence(arr, target)
    return last - first + 1


def test_count_occurrences() -> int:
    ok: int = 0
    data: list[int] = [1, 1, 2, 2, 2, 3, 3, 3, 3, 4]
    if count_occurrences(data, 3) == 4:
        ok += 1
    if count_occurrences(data, 2) == 3:
        ok += 1
    if count_occurrences(data, 5) == 0:
        ok += 1
    if count_occurrences([], 1) == 0:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 38. Rotated Array Search
# ---------------------------------------------------------------------------
def search_rotated(arr: list[int], target: int) -> int:
    """Search in a rotated sorted array."""
    lo: int = 0
    hi: int = len(arr) - 1
    while lo <= hi:
        mid: int = (lo + hi) // 2
        if arr[mid] == target:
            return mid
        if arr[lo] <= arr[mid]:
            if arr[lo] <= target < arr[mid]:
                hi = mid - 1
            else:
                lo = mid + 1
        else:
            if arr[mid] < target <= arr[hi]:
                lo = mid + 1
            else:
                hi = mid - 1
    return -1


def test_search_rotated() -> int:
    ok: int = 0
    if search_rotated([4, 5, 6, 7, 0, 1, 2], 0) == 4:
        ok += 1
    if search_rotated([4, 5, 6, 7, 0, 1, 2], 3) == -1:
        ok += 1
    if search_rotated([1], 0) == -1:
        ok += 1
    if search_rotated([1], 1) == 0:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 39. Find Minimum in Rotated Array
# ---------------------------------------------------------------------------
def find_min_rotated(arr: list[int]) -> int:
    """Find minimum element in rotated sorted array (no duplicates)."""
    if len(arr) == 0:
        return -1
    lo: int = 0
    hi: int = len(arr) - 1
    while lo < hi:
        mid: int = (lo + hi) // 2
        if arr[mid] > arr[hi]:
            lo = mid + 1
        else:
            hi = mid
    return arr[lo]


def test_find_min_rotated() -> int:
    ok: int = 0
    if find_min_rotated([3, 4, 5, 1, 2]) == 1:
        ok += 1
    if find_min_rotated([4, 5, 6, 7, 0, 1, 2]) == 0:
        ok += 1
    if find_min_rotated([1, 2, 3]) == 1:
        ok += 1
    if find_min_rotated([2, 1]) == 1:
        ok += 1
    if find_min_rotated([]) == -1:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 40. Peak Element
# ---------------------------------------------------------------------------
def find_peak_element(arr: list[int]) -> int:
    """Find index of any peak element using binary search."""
    n: int = len(arr)
    if n == 0:
        return -1
    if n == 1:
        return 0
    lo: int = 0
    hi: int = n - 1
    while lo <= hi:
        mid: int = (lo + hi) // 2
        left_ok: bool = (mid == 0) or (arr[mid] >= arr[mid - 1])
        right_ok: bool = (mid == n - 1) or (arr[mid] >= arr[mid + 1])
        if left_ok and right_ok:
            return mid
        elif mid > 0 and arr[mid - 1] > arr[mid]:
            hi = mid - 1
        else:
            lo = mid + 1
    return lo


def test_find_peak_element() -> int:
    ok: int = 0
    p1: int = find_peak_element([1, 2, 3, 1])
    if p1 == 2:
        ok += 1
    p2: int = find_peak_element([1, 2, 1, 3, 5, 6, 4])
    arr2: list[int] = [1, 2, 1, 3, 5, 6, 4]
    is_peak: bool = True
    if p2 > 0 and arr2[p2] < arr2[p2 - 1]:
        is_peak = False
    if p2 < len(arr2) - 1 and arr2[p2] < arr2[p2 + 1]:
        is_peak = False
    if is_peak and p2 >= 0:
        ok += 1
    if find_peak_element([1]) == 0:
        ok += 1
    if find_peak_element([]) == -1:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 41. Two Sum (sorted array)
# ---------------------------------------------------------------------------
def two_sum_sorted(arr: list[int], target: int) -> list[int]:
    """Find two indices whose elements sum to target (sorted array)."""
    lo: int = 0
    hi: int = len(arr) - 1
    while lo < hi:
        s: int = arr[lo] + arr[hi]
        if s == target:
            return [lo, hi]
        elif s < target:
            lo += 1
        else:
            hi -= 1
    return [-1, -1]


def test_two_sum_sorted() -> int:
    ok: int = 0
    r1: list[int] = two_sum_sorted([2, 7, 11, 15], 9)
    if r1 == [0, 1]:
        ok += 1
    r2: list[int] = two_sum_sorted([1, 2, 3, 4, 5], 8)
    if r2 == [2, 4]:
        ok += 1
    r3: list[int] = two_sum_sorted([1, 2], 10)
    if r3 == [-1, -1]:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 42. Merge Sorted Arrays In-Place Simulation
# ---------------------------------------------------------------------------
def merge_in_place_sim(a: list[int], b: list[int]) -> list[int]:
    """Merge two sorted arrays simulating in-place merge with gap method."""
    combined: list[int] = []
    for val in a:
        combined.append(val)
    for val in b:
        combined.append(val)
    n: int = len(combined)
    gap: int = n
    while gap > 0:
        gap = (gap + 1) // 2
        i: int = 0
        while i + gap < n:
            if combined[i] > combined[i + gap]:
                tmp: int = combined[i]
                combined[i] = combined[i + gap]
                combined[i + gap] = tmp
            i += 1
        if gap == 1:
            # One more pass
            did_swap: bool = True
            while did_swap:
                did_swap = False
                j: int = 0
                while j + 1 < n:
                    if combined[j] > combined[j + 1]:
                        tmp2: int = combined[j]
                        combined[j] = combined[j + 1]
                        combined[j + 1] = tmp2
                        did_swap = True
                    j += 1
            break
    return combined


def test_merge_in_place_sim() -> int:
    ok: int = 0
    r1: list[int] = merge_in_place_sim([1, 3, 5, 7], [2, 4, 6, 8])
    if r1 == [1, 2, 3, 4, 5, 6, 7, 8]:
        ok += 1
    r2: list[int] = merge_in_place_sim([], [1, 2, 3])
    if r2 == [1, 2, 3]:
        ok += 1
    r3: list[int] = merge_in_place_sim([5, 10], [1, 2, 3])
    if r3 == [1, 2, 3, 5, 10]:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 43. Bitonic Sort (power-of-2 length)
# ---------------------------------------------------------------------------
def bitonic_compare_swap(arr: list[int], i: int, j: int, ascending: bool) -> list[int]:
    """Compare and swap for bitonic sort."""
    if ascending:
        if arr[i] > arr[j]:
            tmp: int = arr[i]
            arr[i] = arr[j]
            arr[j] = tmp
    else:
        if arr[i] < arr[j]:
            tmp2: int = arr[i]
            arr[i] = arr[j]
            arr[j] = tmp2
    return arr


def bitonic_sort(arr: list[int]) -> list[int]:
    """Bitonic sort for arrays with power-of-2 length."""
    result: list[int] = []
    for val in arr:
        result.append(val)
    n: int = len(result)
    if n <= 1:
        return result
    k: int = 2
    while k <= n:
        j: int = k // 2
        while j > 0:
            i: int = 0
            while i < n:
                partner: int = i ^ j
                if partner > i:
                    if (i & k) == 0:
                        result = bitonic_compare_swap(result, i, partner, True)
                    else:
                        result = bitonic_compare_swap(result, i, partner, False)
                i += 1
            j //= 2
        k *= 2
    return result


def test_bitonic_sort() -> int:
    ok: int = 0
    r1: list[int] = bitonic_sort([3, 7, 4, 8, 6, 2, 1, 5])
    if r1 == [1, 2, 3, 4, 5, 6, 7, 8]:
        ok += 1
    r2: list[int] = bitonic_sort([4, 3, 2, 1])
    if r2 == [1, 2, 3, 4]:
        ok += 1
    r3: list[int] = bitonic_sort([1])
    if r3 == [1]:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 44. Patience Sort (longest increasing subsequence length)
# ---------------------------------------------------------------------------
def patience_sort_lis_length(arr: list[int]) -> int:
    """Find length of longest increasing subsequence via patience sort piles."""
    piles: list[int] = []
    for card in arr:
        lo: int = 0
        hi: int = len(piles)
        while lo < hi:
            mid: int = (lo + hi) // 2
            if piles[mid] >= card:
                hi = mid
            else:
                lo = mid + 1
        if lo == len(piles):
            piles.append(card)
        else:
            piles[lo] = card
    return len(piles)


def test_patience_sort_lis_length() -> int:
    ok: int = 0
    if patience_sort_lis_length([10, 9, 2, 5, 3, 7, 101, 18]) == 4:
        ok += 1
    if patience_sort_lis_length([0, 1, 0, 3, 2, 3]) == 4:
        ok += 1
    if patience_sort_lis_length([7, 7, 7, 7]) == 1:
        ok += 1
    if patience_sort_lis_length([]) == 0:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 45. Counting Sort Stable (for strings by length)
# ---------------------------------------------------------------------------
def counting_sort_stable(arr: list[int], max_val: int) -> list[int]:
    """Stable counting sort preserving relative order of equal elements."""
    n: int = len(arr)
    if n == 0:
        return []
    count: list[int] = []
    i: int = 0
    while i <= max_val:
        count.append(0)
        i += 1
    for val in arr:
        count[val] += 1
    j: int = 1
    while j <= max_val:
        count[j] += count[j - 1]
        j += 1
    output: list[int] = []
    k: int = 0
    while k < n:
        output.append(0)
        k += 1
    m: int = n - 1
    while m >= 0:
        val2: int = arr[m]
        count[val2] -= 1
        output[count[val2]] = val2
        m -= 1
    return output


def test_counting_sort_stable() -> int:
    ok: int = 0
    r1: list[int] = counting_sort_stable([4, 2, 2, 8, 3, 3, 1], 9)
    if r1 == [1, 2, 2, 3, 3, 4, 8]:
        ok += 1
    r2: list[int] = counting_sort_stable([], 5)
    if r2 == []:
        ok += 1
    r3: list[int] = counting_sort_stable([0, 0, 0], 0)
    if r3 == [0, 0, 0]:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 46. Next Greater Element (stack-based)
# ---------------------------------------------------------------------------
def next_greater_element(arr: list[int]) -> list[int]:
    """For each element, find the next greater element to its right."""
    n: int = len(arr)
    result: list[int] = []
    i: int = 0
    while i < n:
        result.append(-1)
        i += 1
    stack: list[int] = []
    j: int = 0
    while j < n:
        while len(stack) > 0 and arr[stack[len(stack) - 1]] < arr[j]:
            idx: int = stack.pop()
            result[idx] = arr[j]
        stack.append(j)
        j += 1
    return result


def test_next_greater_element() -> int:
    ok: int = 0
    r1: list[int] = next_greater_element([4, 5, 2, 25])
    if r1 == [5, 25, 25, -1]:
        ok += 1
    r2: list[int] = next_greater_element([13, 7, 6, 12])
    if r2 == [-1, 12, 12, -1]:
        ok += 1
    r3: list[int] = next_greater_element([])
    if r3 == []:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 47. Inversion Pairs Check (brute force for small input validation)
# ---------------------------------------------------------------------------
def count_inversions_brute(arr: list[int]) -> int:
    """O(n^2) brute force inversion count for validation."""
    n: int = len(arr)
    count: int = 0
    i: int = 0
    while i < n:
        j: int = i + 1
        while j < n:
            if arr[i] > arr[j]:
                count += 1
            j += 1
        i += 1
    return count


def test_inversions_cross_check() -> int:
    """Cross-check merge-based and brute-force inversion counts."""
    ok: int = 0
    test_cases: list[list[int]] = [
        [3, 1, 2],
        [1, 2, 3],
        [5, 4, 3, 2, 1],
        [1, 5, 2, 4, 3],
    ]
    for tc in test_cases:
        merge_count_val: int = count_inversions(tc)
        brute_count_val: int = count_inversions_brute(tc)
        if merge_count_val == brute_count_val:
            ok += 1
    return ok


# ---------------------------------------------------------------------------
# 48. Minimum Swaps to Sort
# ---------------------------------------------------------------------------
def min_swaps_to_sort(arr: list[int]) -> int:
    """Minimum swaps to sort array using cycle detection."""
    n: int = len(arr)
    if n <= 1:
        return 0
    # Create indexed pairs and sort
    indexed: list[list[int]] = []
    i: int = 0
    while i < n:
        indexed.append([arr[i], i])
        i += 1
    # Sort by value (insertion sort on pairs)
    j: int = 1
    while j < n:
        key_val: int = indexed[j][0]
        key_idx: int = indexed[j][1]
        k: int = j - 1
        while k >= 0 and indexed[k][0] > key_val:
            indexed[k + 1][0] = indexed[k][0]
            indexed[k + 1][1] = indexed[k][1]
            k -= 1
        indexed[k + 1][0] = key_val
        indexed[k + 1][1] = key_idx
        j += 1
    visited: list[bool] = []
    vi: int = 0
    while vi < n:
        visited.append(False)
        vi += 1
    swaps: int = 0
    m: int = 0
    while m < n:
        if visited[m] or indexed[m][1] == m:
            visited[m] = True
            m += 1
            continue
        cycle_size: int = 0
        node: int = m
        while not visited[node]:
            visited[node] = True
            node = indexed[node][1]
            cycle_size += 1
        if cycle_size > 1:
            swaps += cycle_size - 1
        m += 1
    return swaps


def test_min_swaps_to_sort() -> int:
    ok: int = 0
    if min_swaps_to_sort([4, 3, 2, 1]) == 2:
        ok += 1
    if min_swaps_to_sort([1, 5, 4, 3, 2]) == 2:
        ok += 1
    if min_swaps_to_sort([1, 2, 3]) == 0:
        ok += 1
    if min_swaps_to_sort([]) == 0:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 49. Sort Array by Parity (evens first, odds last)
# ---------------------------------------------------------------------------
def sort_by_parity(arr: list[int]) -> list[int]:
    """Move all even numbers before odd numbers, preserve relative order within."""
    evens: list[int] = []
    odds: list[int] = []
    for val in arr:
        if val % 2 == 0:
            evens.append(val)
        else:
            odds.append(val)
    result: list[int] = []
    for val in evens:
        result.append(val)
    for val in odds:
        result.append(val)
    return result


def sort_by_parity_inplace(arr: list[int]) -> list[int]:
    """In-place style parity sort using two pointers."""
    result: list[int] = []
    for val in arr:
        result.append(val)
    n: int = len(result)
    lo: int = 0
    hi: int = n - 1
    while lo < hi:
        while lo < hi and result[lo] % 2 == 0:
            lo += 1
        while lo < hi and result[hi] % 2 == 1:
            hi -= 1
        if lo < hi:
            tmp: int = result[lo]
            result[lo] = result[hi]
            result[hi] = tmp
            lo += 1
            hi -= 1
    return result


def test_sort_by_parity() -> int:
    ok: int = 0
    r1: list[int] = sort_by_parity([3, 1, 2, 4])
    if r1 == [2, 4, 3, 1]:
        ok += 1
    r2: list[int] = sort_by_parity([])
    if r2 == []:
        ok += 1
    r3: list[int] = sort_by_parity_inplace([3, 1, 2, 4])
    # All evens before all odds
    phase: int = 0
    valid: bool = True
    for val in r3:
        if phase == 0:
            if val % 2 == 1:
                phase = 1
        elif phase == 1:
            if val % 2 == 0:
                valid = False
    if valid:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 50. Wiggle Sort
# ---------------------------------------------------------------------------
def wiggle_sort(arr: list[int]) -> list[int]:
    """Produce arr[0]<=arr[1]>=arr[2]<=arr[3]... pattern."""
    result: list[int] = []
    for val in arr:
        result.append(val)
    n: int = len(result)
    i: int = 0
    while i < n - 1:
        if i % 2 == 0:
            if result[i] > result[i + 1]:
                tmp: int = result[i]
                result[i] = result[i + 1]
                result[i + 1] = tmp
        else:
            if result[i] < result[i + 1]:
                tmp2: int = result[i]
                result[i] = result[i + 1]
                result[i + 1] = tmp2
        i += 1
    return result


def test_wiggle_sort() -> int:
    ok: int = 0
    r: list[int] = wiggle_sort([3, 5, 2, 1, 6, 4])
    n: int = len(r)
    valid: bool = True
    i: int = 0
    while i < n - 1:
        if i % 2 == 0:
            if r[i] > r[i + 1]:
                valid = False
        else:
            if r[i] < r[i + 1]:
                valid = False
        i += 1
    if valid and n == 6:
        ok += 1
    r2: list[int] = wiggle_sort([])
    if r2 == []:
        ok += 1
    r3: list[int] = wiggle_sort([1])
    if r3 == [1]:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 51. Squash Duplicates from Sorted Array
# ---------------------------------------------------------------------------
def remove_duplicates_sorted(arr: list[int]) -> list[int]:
    """Remove duplicates from sorted array, return new array."""
    if len(arr) == 0:
        return []
    result: list[int] = [arr[0]]
    i: int = 1
    while i < len(arr):
        if arr[i] != arr[i - 1]:
            result.append(arr[i])
        i += 1
    return result


def test_remove_duplicates_sorted() -> int:
    ok: int = 0
    r1: list[int] = remove_duplicates_sorted([1, 1, 2, 2, 3, 4, 4, 5])
    if r1 == [1, 2, 3, 4, 5]:
        ok += 1
    r2: list[int] = remove_duplicates_sorted([])
    if r2 == []:
        ok += 1
    r3: list[int] = remove_duplicates_sorted([7, 7, 7])
    if r3 == [7]:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 52. Sort Colors (variant of Dutch flag for 0, 1, 2)
# ---------------------------------------------------------------------------
def sort_colors(arr: list[int]) -> list[int]:
    """Sort array containing only 0, 1, 2 using single pass."""
    result: list[int] = []
    for val in arr:
        result.append(val)
    n: int = len(result)
    lo: int = 0
    mid: int = 0
    hi: int = n - 1
    while mid <= hi:
        if result[mid] == 0:
            tmp: int = result[lo]
            result[lo] = result[mid]
            result[mid] = tmp
            lo += 1
            mid += 1
        elif result[mid] == 1:
            mid += 1
        else:
            tmp2: int = result[mid]
            result[mid] = result[hi]
            result[hi] = tmp2
            hi -= 1
    return result


def test_sort_colors() -> int:
    ok: int = 0
    r1: list[int] = sort_colors([2, 0, 2, 1, 1, 0])
    if r1 == [0, 0, 1, 1, 2, 2]:
        ok += 1
    r2: list[int] = sort_colors([2, 0, 1])
    if r2 == [0, 1, 2]:
        ok += 1
    r3: list[int] = sort_colors([])
    if r3 == []:
        ok += 1
    r4: list[int] = sort_colors([0])
    if r4 == [0]:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# 53. Median of Two Sorted Arrays (simplified: merge + pick)
# ---------------------------------------------------------------------------
def median_two_sorted(a: list[int], b: list[int]) -> int:
    """Find median of two sorted arrays (integer division for simplicity)."""
    merged: list[int] = merge_two_sorted(a, b)
    n: int = len(merged)
    if n == 0:
        return 0
    if n % 2 == 1:
        return merged[n // 2]
    else:
        return (merged[n // 2 - 1] + merged[n // 2]) // 2


def test_median_two_sorted() -> int:
    ok: int = 0
    if median_two_sorted([1, 3], [2]) == 2:
        ok += 1
    if median_two_sorted([1, 2], [3, 4]) == 2:
        ok += 1
    if median_two_sorted([], [1]) == 1:
        ok += 1
    return ok


# ---------------------------------------------------------------------------
# Master test runner
# ---------------------------------------------------------------------------
def run_all_tests() -> int:
    total: int = 0
    total += test_insertion_sort()
    total += test_selection_sort()
    total += test_iterative_merge_sort()
    total += test_iterative_quicksort()
    total += test_dutch_national_flag()
    total += test_three_way_partition()
    total += test_lower_bound()
    total += test_upper_bound()
    total += test_search_insert_position()
    total += test_interpolation_search()
    total += test_exponential_search()
    total += test_ternary_search_max()
    total += test_quickselect()
    total += test_merge_k_sorted()
    total += test_chunk_sort()
    total += test_counting_sort()
    total += test_radix_sort_lsd()
    total += test_sift_down()
    total += test_sift_up()
    total += test_heapify()
    total += test_heap_sort()
    total += test_topological_sort()
    total += test_count_inversions()
    total += test_run_length_encode()
    total += test_run_length_decode()
    total += test_rle_roundtrip()
    total += test_naive_pattern_match()
    total += test_kmp_failure()
    total += test_kmp_search()
    total += test_shell_sort()
    total += test_comb_sort()
    total += test_cocktail_shaker_sort()
    total += test_gnome_sort()
    total += test_pancake_sort()
    total += test_cycle_sort()
    total += test_first_last_occurrence()
    total += test_count_occurrences()
    total += test_search_rotated()
    total += test_find_min_rotated()
    total += test_find_peak_element()
    total += test_two_sum_sorted()
    total += test_merge_in_place_sim()
    total += test_bitonic_sort()
    total += test_patience_sort_lis_length()
    total += test_counting_sort_stable()
    total += test_next_greater_element()
    total += test_inversions_cross_check()
    total += test_min_swaps_to_sort()
    total += test_sort_by_parity()
    total += test_wiggle_sort()
    total += test_remove_duplicates_sorted()
    total += test_sort_colors()
    total += test_median_two_sorted()
    return total


if __name__ == "__main__":
    result: int = run_all_tests()
    assert result > 0
