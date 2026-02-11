# Pathological control flow: Switch-like dispatch via if/elif chains
# Tests: large dispatch tables, multi-condition elsif chains


def opcode_dispatch(opcode: int, operand_a: int, operand_b: int) -> int:
    """Simulate a simple CPU instruction dispatch."""
    if opcode == 0:
        return operand_a + operand_b
    elif opcode == 1:
        return operand_a - operand_b
    elif opcode == 2:
        return operand_a * operand_b
    elif opcode == 3:
        if operand_b == 0:
            return 0 - 1
        return operand_a // operand_b
    elif opcode == 4:
        if operand_b == 0:
            return 0 - 1
        return operand_a % operand_b
    elif opcode == 5:
        if operand_a > operand_b:
            return 1
        elif operand_a == operand_b:
            return 0
        else:
            return 0 - 1
    elif opcode == 6:
        # min
        if operand_a < operand_b:
            return operand_a
        return operand_b
    elif opcode == 7:
        # max
        if operand_a > operand_b:
            return operand_a
        return operand_b
    elif opcode == 8:
        # abs of a
        if operand_a < 0:
            return 0 - operand_a
        return operand_a
    elif opcode == 9:
        # power (simple)
        result: int = 1
        i: int = 0
        while i < operand_b:
            result = result * operand_a
            i = i + 1
        return result
    elif opcode == 10:
        # factorial of a (if small)
        if operand_a < 0:
            return 0 - 1
        result2: int = 1
        j: int = 1
        while j <= operand_a:
            result2 = result2 * j
            j = j + 1
        return result2
    else:
        return 0 - 99


def execute_program(opcodes: list[int], operands_a: list[int], operands_b: list[int]) -> list[int]:
    """Execute a sequence of instructions."""
    results: list[int] = []
    i: int = 0
    while i < len(opcodes):
        r: int = opcode_dispatch(opcodes[i], operands_a[i], operands_b[i])
        results.append(r)
        i = i + 1
    return results


def test_module() -> int:
    passed: int = 0
    # Test 1: add
    if opcode_dispatch(0, 10, 20) == 30:
        passed = passed + 1
    # Test 2: subtract
    if opcode_dispatch(1, 20, 7) == 13:
        passed = passed + 1
    # Test 3: multiply
    if opcode_dispatch(2, 6, 7) == 42:
        passed = passed + 1
    # Test 4: divide
    if opcode_dispatch(3, 20, 4) == 5:
        passed = passed + 1
    # Test 5: compare
    if opcode_dispatch(5, 10, 5) == 1:
        passed = passed + 1
    # Test 6: power
    if opcode_dispatch(9, 2, 10) == 1024:
        passed = passed + 1
    # Test 7: factorial
    if opcode_dispatch(10, 5, 0) == 120:
        passed = passed + 1
    # Test 8: execute program
    ops: list[int] = [0, 2, 1]
    a_vals: list[int] = [3, 4, 10]
    b_vals: list[int] = [5, 6, 3]
    results: list[int] = execute_program(ops, a_vals, b_vals)
    if results[0] == 8 and results[1] == 24 and results[2] == 7:
        passed = passed + 1
    return passed
