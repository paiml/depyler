"""Hard generator patterns for depyler transpiler stress testing.
Tests lazy evaluation patterns, iteration protocols, and sequence generation."""


# --- Fibonacci-style lazy sequences using lists ---

def fibonacci_sequence(n: int) -> list[int]:
    """Generate first n Fibonacci numbers as a list."""
    if n <= 0:
        return []
    if n == 1:
        return [0]
    result: list[int] = [0, 1]
    for i in range(2, n):
        result.append(result[i - 1] + result[i - 2])
    return result


def lucas_sequence(n: int) -> list[int]:
    """Generate first n Lucas numbers (2, 1, 3, 4, 7, ...)."""
    if n <= 0:
        return []
    if n == 1:
        return [2]
    result: list[int] = [2, 1]
    for i in range(2, n):
        result.append(result[i - 1] + result[i - 2])
    return result


def tribonacci_sequence(n: int) -> list[int]:
    """Generate first n Tribonacci numbers (0, 0, 1, 1, 2, 4, 7, ...)."""
    if n <= 0:
        return []
    if n == 1:
        return [0]
    if n == 2:
        return [0, 0]
    result: list[int] = [0, 0, 1]
    for i in range(3, n):
        result.append(result[i - 1] + result[i - 2] + result[i - 3])
    return result


# --- Infinite sequence simulation with limits ---

def naturals_up_to(limit: int) -> list[int]:
    """Simulate infinite natural number generator with cutoff."""
    result: list[int] = []
    n: int = 1
    while n <= limit:
        result.append(n)
        n += 1
    return result


def powers_of_two(count: int) -> list[int]:
    """Generate first count powers of two."""
    result: list[int] = []
    val: int = 1
    for i in range(count):
        result.append(val)
        val *= 2
    return result


def geometric_sequence(start: int, ratio: int, count: int) -> list[int]:
    """Generate geometric sequence: start, start*ratio, start*ratio^2, ..."""
    result: list[int] = []
    current: int = start
    for i in range(count):
        result.append(current)
        current *= ratio
    return result


def arithmetic_sequence(start: int, step: int, count: int) -> list[int]:
    """Generate arithmetic sequence with given start, step, and count."""
    result: list[int] = []
    current: int = start
    for i in range(count):
        result.append(current)
        current += step
    return result


def collatz_sequence(n: int) -> list[int]:
    """Generate Collatz sequence starting from n until reaching 1."""
    result: list[int] = [n]
    current: int = n
    while current != 1:
        if current % 2 == 0:
            current = current // 2
        else:
            current = 3 * current + 1
        result.append(current)
    return result


# --- Takewhile / Dropwhile simulation ---

def take_while_positive(nums: list[int]) -> list[int]:
    """Take elements while they are positive."""
    result: list[int] = []
    for x in nums:
        if x <= 0:
            break
        result.append(x)
    return result


def drop_while_negative(nums: list[int]) -> list[int]:
    """Drop elements while they are negative, keep the rest."""
    result: list[int] = []
    dropping: bool = True
    for x in nums:
        if dropping and x < 0:
            continue
        dropping = False
        result.append(x)
    return result


def take_first_n(items: list[int], n: int) -> list[int]:
    """Take first n items from a list (simulates islice)."""
    result: list[int] = []
    count: int = 0
    for x in items:
        if count >= n:
            break
        result.append(x)
        count += 1
    return result


def drop_first_n(items: list[int], n: int) -> list[int]:
    """Drop first n items from a list."""
    result: list[int] = []
    count: int = 0
    for x in items:
        if count < n:
            count += 1
            continue
        result.append(x)
    return result


# --- Accumulator patterns that simulate yield behavior ---

def running_sum(nums: list[int]) -> list[int]:
    """Produce running sum (like itertools.accumulate)."""
    result: list[int] = []
    total: int = 0
    for x in nums:
        total += x
        result.append(total)
    return result


def running_product(nums: list[int]) -> list[int]:
    """Produce running product of elements."""
    result: list[int] = []
    prod: int = 1
    for x in nums:
        prod *= x
        result.append(prod)
    return result


def running_max(nums: list[int]) -> list[int]:
    """Track the running maximum across the sequence."""
    if not nums:
        return []
    result: list[int] = [nums[0]]
    current_max: int = nums[0]
    for i in range(1, len(nums)):
        if nums[i] > current_max:
            current_max = nums[i]
        result.append(current_max)
    return result


def running_min(nums: list[int]) -> list[int]:
    """Track the running minimum across the sequence."""
    if not nums:
        return []
    result: list[int] = [nums[0]]
    current_min: int = nums[0]
    for i in range(1, len(nums)):
        if nums[i] < current_min:
            current_min = nums[i]
        result.append(current_min)
    return result


def scan_with_op(nums: list[int], initial: int, add: bool) -> list[int]:
    """Generalized scan: accumulate with add or multiply."""
    result: list[int] = []
    acc: int = initial
    for x in nums:
        if add:
            acc = acc + x
        else:
            acc = acc * x
        result.append(acc)
    return result


# --- Chain / Flatten patterns (nested iteration) ---

def chain_lists(a: list[int], b: list[int], c: list[int]) -> list[int]:
    """Chain three lists together (like itertools.chain)."""
    result: list[int] = []
    for x in a:
        result.append(x)
    for x in b:
        result.append(x)
    for x in c:
        result.append(x)
    return result


def flatten_nested(nested: list[list[int]]) -> list[int]:
    """Flatten a list of lists into a single list."""
    result: list[int] = []
    for inner in nested:
        for x in inner:
            result.append(x)
    return result


def flatten_and_filter(nested: list[list[int]], threshold: int) -> list[int]:
    """Flatten and keep only elements above threshold."""
    result: list[int] = []
    for inner in nested:
        for x in inner:
            if x > threshold:
                result.append(x)
    return result


def interleave(a: list[int], b: list[int]) -> list[int]:
    """Interleave two lists element by element."""
    result: list[int] = []
    min_len: int = len(a)
    if len(b) < min_len:
        min_len = len(b)
    for i in range(min_len):
        result.append(a[i])
        result.append(b[i])
    for i in range(min_len, len(a)):
        result.append(a[i])
    for i in range(min_len, len(b)):
        result.append(b[i])
    return result


def roundrobin(lists: list[list[int]]) -> list[int]:
    """Round-robin iteration across multiple lists."""
    result: list[int] = []
    max_len: int = 0
    for lst in lists:
        if len(lst) > max_len:
            max_len = len(lst)
    for i in range(max_len):
        for lst in lists:
            if i < len(lst):
                result.append(lst[i])
    return result


# --- Zip-like patterns combining multiple sequences ---

def zip_sum(a: list[int], b: list[int]) -> list[int]:
    """Zip two lists and sum corresponding pairs."""
    result: list[int] = []
    min_len: int = len(a)
    if len(b) < min_len:
        min_len = len(b)
    for i in range(min_len):
        result.append(a[i] + b[i])
    return result


def zip_product(a: list[int], b: list[int]) -> list[int]:
    """Zip two lists and multiply corresponding pairs."""
    result: list[int] = []
    min_len: int = len(a)
    if len(b) < min_len:
        min_len = len(b)
    for i in range(min_len):
        result.append(a[i] * b[i])
    return result


def zip_max(a: list[int], b: list[int]) -> list[int]:
    """Zip two lists and take the max of each pair."""
    result: list[int] = []
    min_len: int = len(a)
    if len(b) < min_len:
        min_len = len(b)
    for i in range(min_len):
        if a[i] > b[i]:
            result.append(a[i])
        else:
            result.append(b[i])
    return result


def enumerate_list(items: list[int]) -> list[list[int]]:
    """Simulate enumerate: return list of [index, value] pairs."""
    result: list[list[int]] = []
    for i in range(len(items)):
        result.append([i, items[i]])
    return result


def zip_with_index(items: list[int], start: int) -> list[list[int]]:
    """Zip items with their index starting from a given offset."""
    result: list[list[int]] = []
    for i in range(len(items)):
        result.append([start + i, items[i]])
    return result


# --- Batching / Chunking lists ---

def chunk_list(items: list[int], size: int) -> list[list[int]]:
    """Split a list into chunks of given size."""
    result: list[list[int]] = []
    chunk: list[int] = []
    for i in range(len(items)):
        chunk.append(items[i])
        if len(chunk) == size:
            result.append(chunk)
            chunk = []
    if len(chunk) > 0:
        result.append(chunk)
    return result


def chunk_sum(items: list[int], size: int) -> list[int]:
    """Split into chunks and sum each chunk."""
    result: list[int] = []
    total: int = 0
    count: int = 0
    for x in items:
        total += x
        count += 1
        if count == size:
            result.append(total)
            total = 0
            count = 0
    if count > 0:
        result.append(total)
    return result


def pairwise(items: list[int]) -> list[list[int]]:
    """Generate consecutive pairs: [a,b], [b,c], [c,d], ..."""
    result: list[list[int]] = []
    for i in range(len(items) - 1):
        result.append([items[i], items[i + 1]])
    return result


def triples(items: list[int]) -> list[list[int]]:
    """Generate consecutive triples from a list."""
    result: list[list[int]] = []
    for i in range(len(items) - 2):
        result.append([items[i], items[i + 1], items[i + 2]])
    return result


# --- Sliding window generators ---

def sliding_window_sum(nums: list[int], window: int) -> list[int]:
    """Compute sum over a sliding window of given size."""
    result: list[int] = []
    if len(nums) < window:
        return result
    current_sum: int = 0
    for i in range(window):
        current_sum += nums[i]
    result.append(current_sum)
    for i in range(window, len(nums)):
        current_sum += nums[i] - nums[i - window]
        result.append(current_sum)
    return result


def sliding_window_max(nums: list[int], window: int) -> list[int]:
    """Compute max over a sliding window (brute force)."""
    result: list[int] = []
    for i in range(len(nums) - window + 1):
        w_max: int = nums[i]
        for j in range(i + 1, i + window):
            if nums[j] > w_max:
                w_max = nums[j]
        result.append(w_max)
    return result


def sliding_window_min(nums: list[int], window: int) -> list[int]:
    """Compute min over a sliding window (brute force)."""
    result: list[int] = []
    for i in range(len(nums) - window + 1):
        w_min: int = nums[i]
        for j in range(i + 1, i + window):
            if nums[j] < w_min:
                w_min = nums[j]
        result.append(w_min)
    return result


def sliding_window_avg_int(nums: list[int], window: int) -> list[int]:
    """Integer average over a sliding window (truncated)."""
    result: list[int] = []
    if len(nums) < window or window <= 0:
        return result
    current_sum: int = 0
    for i in range(window):
        current_sum += nums[i]
    result.append(current_sum // window)
    for i in range(window, len(nums)):
        current_sum += nums[i] - nums[i - window]
        result.append(current_sum // window)
    return result


# --- Generator composition / pipeline patterns ---

def map_then_filter(nums: list[int], multiplier: int, threshold: int) -> list[int]:
    """Map (multiply) then filter (keep above threshold)."""
    result: list[int] = []
    for x in nums:
        mapped: int = x * multiplier
        if mapped > threshold:
            result.append(mapped)
    return result


def filter_then_map(nums: list[int], threshold: int, offset: int) -> list[int]:
    """Filter (keep above threshold) then map (add offset)."""
    result: list[int] = []
    for x in nums:
        if x > threshold:
            result.append(x + offset)
    return result


def pipeline_square_filter_sum(nums: list[int], limit: int) -> int:
    """Pipeline: square each -> filter below limit -> sum."""
    total: int = 0
    for x in nums:
        squared: int = x * x
        if squared < limit:
            total += squared
    return total


def pipeline_abs_dedup_sort(nums: list[int]) -> list[int]:
    """Pipeline: absolute value -> deduplicate -> sort."""
    abs_vals: list[int] = []
    for x in nums:
        if x < 0:
            abs_vals.append(-x)
        else:
            abs_vals.append(x)
    seen: list[int] = []
    for x in abs_vals:
        found: bool = False
        for s in seen:
            if s == x:
                found = True
                break
        if not found:
            seen.append(x)
    for i in range(len(seen)):
        for j in range(i + 1, len(seen)):
            if seen[i] > seen[j]:
                temp: int = seen[i]
                seen[i] = seen[j]
                seen[j] = temp
    return seen


def multi_stage_transform(nums: list[int]) -> list[int]:
    """Three-stage pipeline: double -> filter even -> subtract one."""
    stage1: list[int] = []
    for x in nums:
        stage1.append(x * 2)
    stage2: list[int] = []
    for x in stage1:
        if x % 2 == 0:
            stage2.append(x)
    stage3: list[int] = []
    for x in stage2:
        stage3.append(x - 1)
    return stage3


# --- State machine simulators ---

def state_machine_even_odd(nums: list[int]) -> int:
    """State machine: count transitions between even/odd states."""
    if not nums:
        return 0
    transitions: int = 0
    is_even: bool = nums[0] % 2 == 0
    for i in range(1, len(nums)):
        current_even: bool = nums[i] % 2 == 0
        if current_even != is_even:
            transitions += 1
        is_even = current_even
    return transitions


def state_machine_sign_changes(nums: list[int]) -> int:
    """Count sign changes (positive <-> negative) in sequence."""
    if len(nums) < 2:
        return 0
    changes: int = 0
    prev_positive: bool = nums[0] >= 0
    for i in range(1, len(nums)):
        curr_positive: bool = nums[i] >= 0
        if curr_positive != prev_positive:
            changes += 1
        prev_positive = curr_positive
    return changes


def state_machine_run_lengths(nums: list[int]) -> list[int]:
    """Encode run lengths of consecutive equal elements."""
    if not nums:
        return []
    result: list[int] = []
    count: int = 1
    for i in range(1, len(nums)):
        if nums[i] == nums[i - 1]:
            count += 1
        else:
            result.append(count)
            count = 1
    result.append(count)
    return result


def state_machine_bracket_depth(opens: list[int], closes: list[int]) -> list[int]:
    """Simulate bracket depth tracking. opens=[positions of '('], closes=[positions of ')'].
    Returns depth at each position from 0 to max_pos."""
    max_pos: int = 0
    for p in opens:
        if p > max_pos:
            max_pos = p
    for p in closes:
        if p > max_pos:
            max_pos = p
    depths: list[int] = []
    depth: int = 0
    for pos in range(max_pos + 1):
        for o in opens:
            if o == pos:
                depth += 1
        for c in closes:
            if c == pos:
                depth -= 1
        depths.append(depth)
    return depths


# --- Cartesian product generation ---

def cartesian_product_flat(a: list[int], b: list[int]) -> list[int]:
    """Cartesian product flattened: [a0*b0, a0*b1, ..., a1*b0, ...]."""
    result: list[int] = []
    for x in a:
        for y in b:
            result.append(x * y)
    return result


def cartesian_product_pairs(a: list[int], b: list[int]) -> list[list[int]]:
    """Cartesian product as pairs: [[a0,b0], [a0,b1], ...]."""
    result: list[list[int]] = []
    for x in a:
        for y in b:
            result.append([x, y])
    return result


def cartesian_triple_sum(a: list[int], b: list[int], c: list[int]) -> list[int]:
    """Triple cartesian product, returning sums of triples."""
    result: list[int] = []
    for x in a:
        for y in b:
            for z in c:
                result.append(x + y + z)
    return result


def self_cartesian_filter(items: list[int], target_sum: int) -> list[list[int]]:
    """Find all pairs from items where pair sums to target."""
    result: list[list[int]] = []
    for i in range(len(items)):
        for j in range(i + 1, len(items)):
            if items[i] + items[j] == target_sum:
                result.append([items[i], items[j]])
    return result


# --- Manual iterator protocol / stateful iteration ---

def step_iterator(start: int, stop: int, step: int) -> list[int]:
    """Simulate range with arbitrary step."""
    result: list[int] = []
    current: int = start
    if step > 0:
        while current < stop:
            result.append(current)
            current += step
    elif step < 0:
        while current > stop:
            result.append(current)
            current += step
    return result


def cycle_n(items: list[int], n: int) -> list[int]:
    """Cycle through items n times total (like itertools.cycle limited)."""
    result: list[int] = []
    if not items:
        return result
    count: int = 0
    while count < n:
        for x in items:
            if count >= n:
                break
            result.append(x)
            count += 1
    return result


def repeat_each(items: list[int], times: int) -> list[int]:
    """Repeat each element a given number of times."""
    result: list[int] = []
    for x in items:
        for t in range(times):
            result.append(x)
    return result


def unique_elements(items: list[int]) -> list[int]:
    """Yield only unique elements preserving first-seen order."""
    result: list[int] = []
    for x in items:
        found: bool = False
        for r in result:
            if r == x:
                found = True
                break
        if not found:
            result.append(x)
    return result


def compress_select(data: list[int], selectors: list[int]) -> list[int]:
    """Select elements where corresponding selector is nonzero (like itertools.compress)."""
    result: list[int] = []
    min_len: int = len(data)
    if len(selectors) < min_len:
        min_len = len(selectors)
    for i in range(min_len):
        if selectors[i] != 0:
            result.append(data[i])
    return result


# --- Sequence generation edge cases ---

def prime_sieve(limit: int) -> list[int]:
    """Generate primes up to limit using sieve approach with list."""
    if limit < 2:
        return []
    is_prime: list[bool] = []
    for i in range(limit + 1):
        is_prime.append(True)
    is_prime[0] = False
    is_prime[1] = False
    p: int = 2
    while p * p <= limit:
        if is_prime[p]:
            multiple: int = p * p
            while multiple <= limit:
                is_prime[multiple] = False
                multiple += p
        p += 1
    result: list[int] = []
    for i in range(2, limit + 1):
        if is_prime[i]:
            result.append(i)
    return result


def pascal_row(n: int) -> list[int]:
    """Generate nth row of Pascal's triangle."""
    row: list[int] = [1]
    for k in range(1, n + 1):
        val: int = row[k - 1] * (n - k + 1) // k
        row.append(val)
    return row


def look_and_say_step(seq: list[int]) -> list[int]:
    """One step of the look-and-say sequence."""
    if not seq:
        return []
    result: list[int] = []
    count: int = 1
    current: int = seq[0]
    for i in range(1, len(seq)):
        if seq[i] == current:
            count += 1
        else:
            result.append(count)
            result.append(current)
            current = seq[i]
            count = 1
    result.append(count)
    result.append(current)
    return result


def catalan_numbers(n: int) -> list[int]:
    """Generate first n Catalan numbers."""
    if n <= 0:
        return []
    result: list[int] = [1]
    for i in range(1, n):
        val: int = result[i - 1] * 2 * (2 * i - 1) // (i + 1)
        result.append(val)
    return result


# --- Test functions ---

def test_fibonacci() -> int:
    """Test fibonacci sequence generation."""
    fib: list[int] = fibonacci_sequence(10)
    return fib[9]  # 34


def test_lucas() -> int:
    """Test lucas sequence."""
    luc: list[int] = lucas_sequence(7)
    return luc[6]  # 18


def test_tribonacci() -> int:
    """Test tribonacci sequence."""
    tri: list[int] = tribonacci_sequence(8)
    return tri[7]  # 13


def test_naturals() -> int:
    """Test naturals up to limit."""
    nums: list[int] = naturals_up_to(5)
    total: int = 0
    for x in nums:
        total += x
    return total  # 15


def test_powers_of_two() -> int:
    """Test powers of two generation."""
    pows: list[int] = powers_of_two(6)
    return pows[5]  # 32


def test_geometric() -> int:
    """Test geometric sequence."""
    geo: list[int] = geometric_sequence(3, 2, 5)
    return geo[4]  # 48


def test_arithmetic() -> int:
    """Test arithmetic sequence."""
    arith: list[int] = arithmetic_sequence(10, 3, 4)
    return arith[3]  # 19


def test_collatz() -> int:
    """Test collatz sequence length for 27."""
    seq: list[int] = collatz_sequence(27)
    return len(seq)  # 112


def test_take_while() -> int:
    """Test take_while_positive."""
    result: list[int] = take_while_positive([3, 5, 2, -1, 4])
    return len(result)  # 3


def test_drop_while() -> int:
    """Test drop_while_negative."""
    result: list[int] = drop_while_negative([-3, -1, 5, -2, 8])
    return len(result)  # 3


def test_take_first() -> int:
    """Test take_first_n."""
    result: list[int] = take_first_n([10, 20, 30, 40, 50], 3)
    total: int = 0
    for x in result:
        total += x
    return total  # 60


def test_drop_first() -> int:
    """Test drop_first_n."""
    result: list[int] = drop_first_n([10, 20, 30, 40, 50], 2)
    total: int = 0
    for x in result:
        total += x
    return total  # 120


def test_running_sum() -> int:
    """Test running sum accumulator."""
    result: list[int] = running_sum([1, 2, 3, 4, 5])
    return result[4]  # 15


def test_running_product() -> int:
    """Test running product."""
    result: list[int] = running_product([1, 2, 3, 4])
    return result[3]  # 24


def test_running_max() -> int:
    """Test running max tracker."""
    result: list[int] = running_max([3, 1, 4, 1, 5, 9])
    return result[5]  # 9


def test_running_min() -> int:
    """Test running min tracker."""
    result: list[int] = running_min([5, 3, 7, 2, 8])
    return result[3]  # 2


def test_scan() -> int:
    """Test scan with addition."""
    result: list[int] = scan_with_op([1, 2, 3, 4], 0, True)
    return result[3]  # 10


def test_chain() -> int:
    """Test chaining lists."""
    result: list[int] = chain_lists([1, 2], [3, 4], [5, 6])
    return len(result)  # 6


def test_flatten() -> int:
    """Test flatten nested lists."""
    result: list[int] = flatten_nested([[1, 2], [3], [4, 5, 6]])
    total: int = 0
    for x in result:
        total += x
    return total  # 21


def test_flatten_filter() -> int:
    """Test flatten and filter."""
    result: list[int] = flatten_and_filter([[1, 5], [2, 8], [3, 7]], 4)
    return len(result)  # 3 (5, 8, 7)


def test_interleave() -> int:
    """Test interleave two lists."""
    result: list[int] = interleave([1, 3, 5], [2, 4, 6])
    total: int = 0
    for x in result:
        total += x
    return total  # 21


def test_roundrobin() -> int:
    """Test round-robin iteration."""
    result: list[int] = roundrobin([[1, 4], [2, 5], [3, 6]])
    return result[0] + result[1] + result[2]  # 1+2+3 = 6


def test_zip_sum() -> int:
    """Test zip sum."""
    result: list[int] = zip_sum([1, 2, 3], [10, 20, 30])
    total: int = 0
    for x in result:
        total += x
    return total  # 66


def test_zip_product() -> int:
    """Test zip product."""
    result: list[int] = zip_product([2, 3, 4], [5, 6, 7])
    total: int = 0
    for x in result:
        total += x
    return total  # 10+18+28 = 56


def test_zip_max() -> int:
    """Test zip max."""
    result: list[int] = zip_max([1, 5, 3], [4, 2, 6])
    total: int = 0
    for x in result:
        total += x
    return total  # 4+5+6 = 15


def test_enumerate() -> int:
    """Test enumerate simulation."""
    result: list[list[int]] = enumerate_list([10, 20, 30])
    return result[2][0] + result[2][1]  # 2+30 = 32


def test_chunk() -> int:
    """Test chunking."""
    chunks: list[list[int]] = chunk_list([1, 2, 3, 4, 5, 6, 7], 3)
    return len(chunks)  # 3 chunks: [1,2,3],[4,5,6],[7]


def test_chunk_sum() -> int:
    """Test chunk sum."""
    result: list[int] = chunk_sum([1, 2, 3, 4, 5, 6], 2)
    total: int = 0
    for x in result:
        total += x
    return total  # 3+7+11 = 21


def test_pairwise() -> int:
    """Test pairwise generation."""
    pairs: list[list[int]] = pairwise([1, 2, 3, 4])
    return len(pairs)  # 3


def test_triples() -> int:
    """Test triple generation."""
    trips: list[list[int]] = triples([1, 2, 3, 4, 5])
    return len(trips)  # 3


def test_sliding_sum() -> int:
    """Test sliding window sum."""
    result: list[int] = sliding_window_sum([1, 3, 5, 7, 9], 3)
    return result[0] + result[2]  # 9 + 21 = 30


def test_sliding_max() -> int:
    """Test sliding window max."""
    result: list[int] = sliding_window_max([1, 3, 2, 5, 4, 1], 3)
    return result[0] + result[1]  # 3+5 = 8


def test_sliding_min() -> int:
    """Test sliding window min."""
    result: list[int] = sliding_window_min([4, 2, 5, 1, 3, 6], 3)
    return result[0] + result[1]  # 2+1 = 3


def test_sliding_avg() -> int:
    """Test sliding window integer average."""
    result: list[int] = sliding_window_avg_int([10, 20, 30, 40, 50], 3)
    return result[0]  # 20


def test_map_filter_pipeline() -> int:
    """Test map then filter pipeline."""
    result: list[int] = map_then_filter([1, 2, 3, 4, 5], 3, 9)
    total: int = 0
    for x in result:
        total += x
    return total  # 12+15 = 27


def test_filter_map_pipeline() -> int:
    """Test filter then map pipeline."""
    result: list[int] = filter_then_map([1, 5, 3, 8, 2], 3, 10)
    total: int = 0
    for x in result:
        total += x
    return total  # 15+18 = 33


def test_square_filter_sum() -> int:
    """Test pipeline square filter sum."""
    return pipeline_square_filter_sum([1, 2, 3, 4, 5], 20)  # 1+4+9+16 = 30


def test_abs_dedup_sort() -> int:
    """Test abs dedup sort pipeline."""
    result: list[int] = pipeline_abs_dedup_sort([-3, 1, -1, 3, 2, -2])
    return len(result)  # 3 unique: [1, 2, 3]


def test_multi_stage() -> int:
    """Test multi-stage transform pipeline."""
    result: list[int] = multi_stage_transform([1, 2, 3, 4, 5])
    total: int = 0
    for x in result:
        total += x
    return total  # double: 2,4,6,8,10 -> all even -> sub 1: 1,3,5,7,9 -> sum=25


def test_state_even_odd() -> int:
    """Test even/odd state machine."""
    return state_machine_even_odd([2, 3, 4, 6, 7])  # 3 transitions


def test_sign_changes() -> int:
    """Test sign change detection."""
    return state_machine_sign_changes([1, -2, 3, -4, 5])  # 4 changes


def test_run_lengths() -> int:
    """Test run length encoding."""
    result: list[int] = state_machine_run_lengths([1, 1, 2, 2, 2, 3])
    return len(result)  # 3 runs


def test_bracket_depth() -> int:
    """Test bracket depth simulation."""
    depths: list[int] = state_machine_bracket_depth([0, 2], [3, 4])
    return depths[2]  # depth at pos 2 = 2


def test_cartesian_flat() -> int:
    """Test flattened cartesian product."""
    result: list[int] = cartesian_product_flat([1, 2], [3, 4])
    total: int = 0
    for x in result:
        total += x
    return total  # 3+4+6+8 = 21


def test_cartesian_pairs() -> int:
    """Test cartesian product pairs."""
    result: list[list[int]] = cartesian_product_pairs([1, 2], [3, 4])
    return len(result)  # 4


def test_triple_sum() -> int:
    """Test triple cartesian sum."""
    result: list[int] = cartesian_triple_sum([1], [2], [3])
    return result[0]  # 6


def test_self_cartesian() -> int:
    """Test self cartesian filter."""
    result: list[list[int]] = self_cartesian_filter([1, 2, 3, 4, 5], 6)
    return len(result)  # 2 pairs: (1,5), (2,4)


def test_step_iterator() -> int:
    """Test step iterator."""
    result: list[int] = step_iterator(0, 20, 3)
    return len(result)  # 7: [0,3,6,9,12,15,18]


def test_cycle() -> int:
    """Test cycle_n."""
    result: list[int] = cycle_n([1, 2, 3], 7)
    total: int = 0
    for x in result:
        total += x
    return total  # 1+2+3+1+2+3+1 = 13


def test_repeat_each() -> int:
    """Test repeat each element."""
    result: list[int] = repeat_each([5, 10], 3)
    total: int = 0
    for x in result:
        total += x
    return total  # 5+5+5+10+10+10 = 45


def test_unique() -> int:
    """Test unique elements preserving order."""
    result: list[int] = unique_elements([3, 1, 4, 1, 5, 9, 2, 6, 5, 3])
    return len(result)  # 7


def test_compress() -> int:
    """Test compress select."""
    result: list[int] = compress_select([10, 20, 30, 40, 50], [1, 0, 1, 0, 1])
    total: int = 0
    for x in result:
        total += x
    return total  # 10+30+50 = 90


def test_prime_sieve() -> int:
    """Test prime sieve."""
    primes: list[int] = prime_sieve(30)
    return len(primes)  # 10 primes up to 30


def test_pascal_row() -> int:
    """Test Pascal's triangle row."""
    row: list[int] = pascal_row(5)
    total: int = 0
    for x in row:
        total += x
    return total  # 32 (2^5)


def test_look_and_say() -> int:
    """Test look-and-say step."""
    result: list[int] = look_and_say_step([1, 1, 2, 3, 3])
    return len(result)  # 6: [2,1,1,2,2,3]


def test_catalan() -> int:
    """Test Catalan numbers."""
    result: list[int] = catalan_numbers(6)
    return result[5]  # 42


def run_all_tests() -> int:
    """Run all test functions and sum their results for verification."""
    total: int = 0
    total += test_fibonacci()        # 34
    total += test_lucas()            # 18
    total += test_tribonacci()       # 13
    total += test_naturals()         # 15
    total += test_powers_of_two()    # 32
    total += test_geometric()        # 48
    total += test_arithmetic()       # 19
    total += test_collatz()          # 112
    total += test_take_while()       # 3
    total += test_drop_while()       # 3
    total += test_take_first()       # 60
    total += test_drop_first()       # 120
    total += test_running_sum()      # 15
    total += test_running_product()  # 24
    total += test_running_max()      # 9
    total += test_running_min()      # 2
    total += test_scan()             # 10
    total += test_chain()            # 6
    total += test_flatten()          # 21
    total += test_flatten_filter()   # 3
    total += test_interleave()       # 21
    total += test_roundrobin()       # 6
    total += test_zip_sum()          # 66
    total += test_zip_product()      # 56
    total += test_zip_max()          # 15
    total += test_enumerate()        # 32
    total += test_chunk()            # 3
    total += test_chunk_sum()        # 21
    total += test_pairwise()         # 3
    total += test_triples()          # 3
    total += test_sliding_sum()      # 30
    total += test_sliding_max()      # 8
    total += test_sliding_min()      # 3
    total += test_sliding_avg()      # 20
    total += test_map_filter_pipeline()   # 27
    total += test_filter_map_pipeline()   # 33
    total += test_square_filter_sum()     # 30
    total += test_abs_dedup_sort()        # 3
    total += test_multi_stage()           # 25
    total += test_state_even_odd()        # 3
    total += test_sign_changes()          # 4
    total += test_run_lengths()           # 3
    total += test_bracket_depth()         # 2
    total += test_cartesian_flat()        # 21
    total += test_cartesian_pairs()       # 4
    total += test_triple_sum()            # 6
    total += test_self_cartesian()        # 2
    total += test_step_iterator()         # 7
    total += test_cycle()                 # 13
    total += test_repeat_each()           # 45
    total += test_unique()                # 7
    total += test_compress()              # 90
    total += test_prime_sieve()           # 10
    total += test_pascal_row()            # 32
    total += test_look_and_say()          # 6
    total += test_catalan()               # 42
    return total
