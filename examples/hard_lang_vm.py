from typing import List, Tuple

def vm_execute(code: List[int], data: List[int]) -> List[int]:
    stack: List[int] = []
    locals_arr: List[int] = []
    for d in data:
        locals_arr.append(d)
    pc: int = 0
    while pc < len(code):
        op: int = code[pc]
        if op == 1 and pc + 1 < len(code):
            stack.append(code[pc + 1])
            pc = pc + 2
        elif op == 2 and len(stack) >= 2:
            b: int = stack[len(stack) - 1]
            a: int = stack[len(stack) - 2]
            stack = stack[0:len(stack) - 2]
            stack.append(a + b)
            pc = pc + 1
        elif op == 3 and len(stack) >= 2:
            b2: int = stack[len(stack) - 1]
            a2: int = stack[len(stack) - 2]
            stack = stack[0:len(stack) - 2]
            stack.append(a2 - b2)
            pc = pc + 1
        elif op == 4 and len(stack) >= 2:
            b3: int = stack[len(stack) - 1]
            a3: int = stack[len(stack) - 2]
            stack = stack[0:len(stack) - 2]
            stack.append(a3 * b3)
            pc = pc + 1
        elif op == 6 and pc + 1 < len(code):
            idx: int = code[pc + 1]
            if idx < len(locals_arr):
                stack.append(locals_arr[idx])
            pc = pc + 2
        elif op == 7 and pc + 1 < len(code) and len(stack) > 0:
            idx2: int = code[pc + 1]
            while idx2 >= len(locals_arr):
                locals_arr.append(0)
            locals_arr[idx2] = stack[len(stack) - 1]
            stack = stack[0:len(stack) - 1]
            pc = pc + 2
        elif op == 8 and pc + 1 < len(code):
            pc = code[pc + 1]
        elif op == 9 and pc + 1 < len(code) and len(stack) > 0:
            val: int = stack[len(stack) - 1]
            stack = stack[0:len(stack) - 1]
            if val == 0:
                pc = code[pc + 1]
            else:
                pc = pc + 2
        else:
            pc = pc + 1
    return stack

def stack_depth(code: List[int]) -> int:
    depth: int = 0
    max_depth: int = 0
    for i in range(0, len(code), 2):
        if code[i] == 1 or code[i] == 6:
            depth = depth + 1
        elif code[i] >= 2 and code[i] <= 5:
            depth = depth - 1
        if depth > max_depth:
            max_depth = depth
    return max_depth
