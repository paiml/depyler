"""Pathological control flow patterns for transpiler stress testing.

Tests: deeply nested loops with break/continue, while loops with complex
conditions and early returns, long if/elif/else chains, multiple return
paths, guard clauses, flag-controlled loops, state machines via
if-chains, exception-like error handling via return codes, nested
conditionals with boolean algebra, and complex loop termination logic.
"""


def multi_nested_break(matrix: list[list[int]], target: int) -> list[int]:
    """Search a 2D matrix for a target, returning [row, col] or [-1, -1].

    Uses nested loops with break to exit inner loop on find,
    and a flag to break the outer loop. Tests break propagation
    across multiple loop levels.
    """
    found_row: int = -1
    found_col: int = -1
    found: bool = False

    for i in range(len(matrix)):
        if found:
            break
        for j in range(len(matrix[i])):
            if matrix[i][j] == target:
                found_row = i
                found_col = j
                found = True
                break

    return [found_row, found_col]


def continue_with_accumulator(nums: list[int]) -> int:
    """Sum only numbers that pass multiple filter conditions using continue.

    Skip negative numbers, skip multiples of 3, skip numbers > 100.
    Tests multiple continue statements in one loop with accumulation.
    """
    total: int = 0
    for n in nums:
        if n < 0:
            continue
        if n % 3 == 0:
            continue
        if n > 100:
            continue
        total += n
    return total


def guard_clause_chain(x: int, y: int, z: int) -> int:
    """Function with multiple early-return guard clauses.

    Each guard eliminates an invalid input combination.
    Tests the transpiler's handling of multiple return paths
    with different conditions at function start.
    """
    if x < 0:
        return -1
    if y < 0:
        return -2
    if z < 0:
        return -3
    if x == 0 and y == 0 and z == 0:
        return 0
    if x > 1000 or y > 1000 or z > 1000:
        return -4

    result: int = x * y + z
    if result > 10000:
        return 9999
    return result


def long_elif_classifier(score: int) -> str:
    """Classify a score into a grade using a long if/elif/else chain.

    Tests 8+ branches with various comparison operators and
    compound conditions. Each branch returns a different string.
    """
    if score < 0:
        return "invalid_negative"
    elif score == 0:
        return "zero"
    elif score >= 1 and score <= 10:
        return "minimal"
    elif score > 10 and score <= 30:
        return "low"
    elif score > 30 and score <= 50:
        return "below_average"
    elif score > 50 and score <= 70:
        return "average"
    elif score > 70 and score <= 85:
        return "above_average"
    elif score > 85 and score <= 95:
        return "excellent"
    elif score > 95 and score <= 100:
        return "perfect"
    elif score > 100 and score <= 200:
        return "bonus_range"
    else:
        return "off_scale"


def while_with_complex_condition(data: list[int]) -> int:
    """While loop with compound condition involving multiple variables.

    Loop continues while index is valid AND accumulated sum is under limit
    AND we haven't seen a sentinel value. Tests complex boolean expressions
    in while conditions with multiple exit paths.
    """
    idx: int = 0
    total: int = 0
    limit: int = 100
    sentinel: int = -999

    while idx < len(data) and total < limit and data[idx] != sentinel:
        total += data[idx]
        idx += 1

    return total


def state_machine(commands: list[str]) -> str:
    """Simple state machine using if/elif chains inside a loop.

    States: "idle", "running", "paused", "stopped"
    Commands: "start", "pause", "resume", "stop", "reset"
    Tests complex conditional state transitions with string comparisons.
    """
    state: str = "idle"

    for cmd in commands:
        if state == "idle":
            if cmd == "start":
                state = "running"
            elif cmd == "stop":
                state = "stopped"
            # else: stay idle (ignore invalid commands)
        elif state == "running":
            if cmd == "pause":
                state = "paused"
            elif cmd == "stop":
                state = "stopped"
            # reset from running goes to idle
            elif cmd == "reset":
                state = "idle"
        elif state == "paused":
            if cmd == "resume":
                state = "running"
            elif cmd == "stop":
                state = "stopped"
            elif cmd == "reset":
                state = "idle"
        elif state == "stopped":
            if cmd == "reset":
                state = "idle"
            # stopped is terminal for all other commands

    return state


def nested_loop_with_continue_break(n: int) -> list[int]:
    """Triple-nested loop where inner loops use both break and continue.

    Outer loop iterates i. Middle loop iterates j but skips even j.
    Inner loop iterates k but breaks when k*j > threshold.
    Collects products that pass all filters.
    """
    results: list[int] = []
    threshold: int = n * 2

    for i in range(1, n + 1):
        for j in range(1, n + 1):
            if j % 2 == 0:
                continue
            for k in range(1, n + 1):
                product: int = i * j * k
                if product > threshold:
                    break
                if product % 2 == 1:
                    results.append(product)

    return results


def flag_controlled_search(haystack: list[int], needles: list[int]) -> list[int]:
    """Search for multiple needles in haystack using flag variables.

    For each needle, sets a found flag and searches linearly.
    Collects indices of found needles. Tests flag variable patterns
    with nested loops and conditional accumulation.
    """
    found_indices: list[int] = []

    for needle in needles:
        found: bool = False
        idx: int = -1
        for i in range(len(haystack)):
            if haystack[i] == needle:
                found = True
                idx = i
                break
        if found:
            found_indices.append(idx)
        else:
            found_indices.append(-1)

    return found_indices


def error_code_pipeline(values: list[int]) -> list[int]:
    """Multi-stage processing pipeline using error codes instead of exceptions.

    Stage 1: Validate (return -1 on invalid)
    Stage 2: Transform (return -2 on overflow)
    Stage 3: Aggregate (return -3 on underflow)

    Tests multiple return paths with different error codes,
    simulating exception handling via return values.
    """
    # Stage 1: Validate all values
    for v in values:
        if v < -1000 or v > 1000:
            return [-1]

    # Stage 2: Transform (square each value, check for overflow)
    transformed: list[int] = []
    for v in values:
        squared: int = v * v
        if squared > 500000:
            return [-2]
        transformed.append(squared)

    # Stage 3: Running sum with underflow check
    running: list[int] = []
    total: int = 0
    for t in transformed:
        total += t
        if total < 0:
            return [-3]
        running.append(total)

    return running


def complex_boolean_logic(a: bool, b: bool, c: bool, d: bool) -> int:
    """Evaluate multiple complex boolean expressions and return a code.

    Tests De Morgan's law patterns, nested boolean operations,
    and conditional chains based on boolean algebra.
    """
    # Pattern 1: De Morgan
    if not (a and b):
        if not a or not b:
            result1: int = 1
        else:
            result1 = 0
    else:
        result1 = 0

    # Pattern 2: XOR simulation
    xor_ab: bool = (a and not b) or (not a and b)
    xor_cd: bool = (c and not d) or (not c and d)

    # Pattern 3: Majority vote
    vote_count: int = 0
    if a:
        vote_count += 1
    if b:
        vote_count += 1
    if c:
        vote_count += 1
    if d:
        vote_count += 1

    majority: bool = vote_count >= 3

    # Combine all patterns
    if xor_ab and xor_cd and majority:
        return 100 + result1
    elif xor_ab and not majority:
        return 50 + result1
    elif majority and not xor_ab:
        return 25 + result1
    else:
        return result1


def countdown_with_restarts(initial: int, restart_at: list[int]) -> int:
    """Countdown loop that restarts from specific values.

    When counter hits a value in restart_at, jump back to a higher value.
    Each restart value can only trigger once (tracked via visited set).
    Tests while loop with dynamic condition modification.
    """
    counter: int = initial
    steps: int = 0
    used_restarts: set[int] = set()
    max_steps: int = 1000  # Safety limit

    while counter > 0 and steps < max_steps:
        if counter in restart_at and counter not in used_restarts:
            used_restarts.add(counter)
            counter = counter + 5  # Restart higher
        else:
            counter -= 1
        steps += 1

    return steps


def zigzag_traverse(matrix: list[list[int]]) -> list[int]:
    """Traverse a matrix in zigzag order (alternating left-right, right-left).

    Even rows go left-to-right, odd rows go right-to-left.
    Tests conditional direction reversal inside nested loops
    with index arithmetic.
    """
    if not matrix:
        return []

    result: list[int] = []
    for i in range(len(matrix)):
        if i % 2 == 0:
            # Left to right
            for j in range(len(matrix[i])):
                result.append(matrix[i][j])
        else:
            # Right to left
            for j in range(len(matrix[i]) - 1, -1, -1):
                result.append(matrix[i][j])

    return result


def two_pointer_partition(arr: list[int], pivot: int) -> list[int]:
    """Dutch national flag partition: elements < pivot, == pivot, > pivot.

    Uses three-way partitioning with index swapping.
    Tests complex index manipulation with conditional swaps
    and multiple pointer variables.
    """
    result: list[int] = []
    for v in arr:
        result.append(v)

    low: int = 0
    mid: int = 0
    high: int = len(result) - 1

    while mid <= high:
        if result[mid] < pivot:
            # Swap result[low] and result[mid]
            temp: int = result[low]
            result[low] = result[mid]
            result[mid] = temp
            low += 1
            mid += 1
        elif result[mid] == pivot:
            mid += 1
        else:
            # Swap result[mid] and result[high]
            temp2: int = result[mid]
            result[mid] = result[high]
            result[high] = temp2
            high -= 1

    return result


def test_multi_nested_break() -> int:
    """Test nested loop break propagation."""
    matrix: list[list[int]] = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ]
    pos: list[int] = multi_nested_break(matrix, 5)
    # 5 is at row 1, col 1
    return pos[0] * 10 + pos[1]  # 11

def test_multi_nested_break_missing() -> int:
    """Test nested loop break when target not found."""
    matrix: list[list[int]] = [[1, 2], [3, 4]]
    pos: list[int] = multi_nested_break(matrix, 99)
    return pos[0] + pos[1]  # -1 + -1 = -2


def test_continue_accumulator() -> int:
    """Test multiple continue conditions in accumulator."""
    nums: list[int] = [-5, 1, 3, 6, 7, 9, 12, 15, 101, 50, 4, -2, 8]
    # Skip: -5 (neg), 3 (mult 3), 6 (mult 3), 9 (mult 3), 12 (mult 3), 15 (mult 3), 101 (>100), -2 (neg)
    # Keep: 1, 7, 50, 4, 8
    return continue_with_accumulator(nums)  # 1 + 7 + 50 + 4 + 8 = 70


def test_guard_clauses() -> int:
    """Test multiple guard clause paths."""
    r1: int = guard_clause_chain(-1, 5, 5)    # -1 (first guard)
    r2: int = guard_clause_chain(5, -1, 5)    # -2 (second guard)
    r3: int = guard_clause_chain(5, 5, -1)    # -3 (third guard)
    r4: int = guard_clause_chain(0, 0, 0)     # 0 (all zero)
    r5: int = guard_clause_chain(5, 3, 2)     # 17 (5*3+2)
    r6: int = guard_clause_chain(2000, 1, 1)  # -4 (overflow guard)
    return r1 + r2 + r3 + r4 + r5 + r6  # -1 + -2 + -3 + 0 + 17 + -4 = 7


def test_long_elif() -> int:
    """Test long if/elif/else chain with various inputs."""
    scores: list[int] = [-5, 0, 5, 25, 45, 65, 80, 90, 98, 150, 999]
    expected: list[str] = [
        "invalid_negative", "zero", "minimal", "low", "below_average",
        "average", "above_average", "excellent", "perfect", "bonus_range", "off_scale"
    ]
    matches: int = 0
    for i in range(len(scores)):
        result: str = long_elif_classifier(scores[i])
        if result == expected[i]:
            matches += 1
    return matches  # 11


def test_while_complex_condition() -> int:
    """Test while loop with multiple termination conditions."""
    # Normal termination: sum hits limit
    data1: list[int] = [20, 30, 40, 50, 60]
    r1: int = while_with_complex_condition(data1)  # 20+30+40+50 = 140 (total exceeds limit after 4th)

    # Sentinel termination
    data2: list[int] = [10, 20, -999, 30, 40]
    r2: int = while_with_complex_condition(data2)  # 10+20 = 30 (hits sentinel)

    # End of data termination
    data3: list[int] = [1, 2, 3]
    r3: int = while_with_complex_condition(data3)  # 1+2+3 = 6

    return r1 + r2 + r3  # 140 + 30 + 6 = 176


def test_state_machine() -> int:
    """Test state machine transitions."""
    # Test 1: normal lifecycle
    cmds1: list[str] = ["start", "pause", "resume", "stop"]
    s1: str = state_machine(cmds1)
    r1: int = 1 if s1 == "stopped" else 0

    # Test 2: invalid commands ignored
    cmds2: list[str] = ["pause", "resume", "start", "start"]
    s2: str = state_machine(cmds2)
    r2: int = 1 if s2 == "running" else 0

    # Test 3: reset from running
    cmds3: list[str] = ["start", "reset", "start", "pause"]
    s3: str = state_machine(cmds3)
    r3: int = 1 if s3 == "paused" else 0

    # Test 4: stopped is terminal
    cmds4: list[str] = ["start", "stop", "start", "pause"]
    s4: str = state_machine(cmds4)
    r4: int = 1 if s4 == "stopped" else 0

    # Test 5: reset from stopped
    cmds5: list[str] = ["start", "stop", "reset"]
    s5: str = state_machine(cmds5)
    r5: int = 1 if s5 == "idle" else 0

    return r1 + r2 + r3 + r4 + r5  # 5


def test_nested_continue_break() -> int:
    """Test triple-nested loop with mixed continue and break."""
    results: list[int] = nested_loop_with_continue_break(5)
    total: int = 0
    for v in results:
        total += v
    return total


def test_flag_search() -> int:
    """Test flag-controlled multi-needle search."""
    haystack: list[int] = [10, 20, 30, 40, 50, 60, 70, 80]
    needles: list[int] = [30, 70, 99, 10]
    indices: list[int] = flag_controlled_search(haystack, needles)
    # 30 at index 2, 70 at index 6, 99 not found (-1), 10 at index 0
    total: int = 0
    for idx in indices:
        total += idx
    return total  # 2 + 6 + (-1) + 0 = 7


def test_error_pipeline() -> int:
    """Test multi-stage error code pipeline."""
    # Normal case
    r1: list[int] = error_code_pipeline([1, 2, 3, 4, 5])
    # Squared: [1, 4, 9, 16, 25], running: [1, 5, 14, 30, 55]
    result1: int = r1[len(r1) - 1]  # 55

    # Validation failure
    r2: list[int] = error_code_pipeline([1, 2, 9999])
    result2: int = r2[0]  # -1

    # Transform overflow
    r3: list[int] = error_code_pipeline([1, 2, 800])
    # 800^2 = 640000 > 500000
    result3: int = r3[0]  # -2

    return result1 + result2 + result3  # 55 + (-1) + (-2) = 52


def test_boolean_logic() -> int:
    """Test complex boolean expression evaluation."""
    r1: int = complex_boolean_logic(True, False, True, True)
    r2: int = complex_boolean_logic(True, True, True, True)
    r3: int = complex_boolean_logic(False, False, False, False)
    return r1 + r2 + r3


def test_countdown_restarts() -> int:
    """Test while loop with dynamic condition modification."""
    restart_at: list[int] = [5, 3]
    steps: int = countdown_with_restarts(10, restart_at)
    return steps


def test_zigzag() -> int:
    """Test zigzag matrix traversal."""
    matrix: list[list[int]] = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ]
    result: list[int] = zigzag_traverse(matrix)
    # Row 0 L-R: 1,2,3; Row 1 R-L: 6,5,4; Row 2 L-R: 7,8,9
    total: int = 0
    for v in result:
        total += v
    return total  # 1+2+3+6+5+4+7+8+9 = 45


def test_two_pointer() -> int:
    """Test Dutch national flag partitioning."""
    arr: list[int] = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3]
    result: list[int] = two_pointer_partition(arr, 4)
    # All elements < 4 should come first, then 4, then > 4
    # Check: first elements should all be < 4
    below_count: int = 0
    for v in result:
        if v < 4:
            below_count += 1
    return below_count  # 5 elements below 4: {1, 1, 2, 3, 3}
