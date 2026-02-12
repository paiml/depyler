from typing import List, Tuple

def is_tail_position(instrs: List[int], pos: int) -> bool:
    return pos == len(instrs) - 1

def identify_tail_calls(instrs: List[Tuple[int, int]], call_op: int) -> List[int]:
    positions: List[int] = []
    for i in range(len(instrs)):
        if instrs[i][0] == call_op and is_tail_position_tuple(instrs, i):
            positions.append(i)
    return positions

def is_tail_position_tuple(instrs: List[Tuple[int, int]], pos: int) -> bool:
    if pos == len(instrs) - 1:
        return True
    if pos + 1 < len(instrs) and instrs[pos + 1][0] == 99:
        return True
    return False

def optimize_tail_call(instrs: List[Tuple[int, int]], call_op: int, jump_op: int) -> List[Tuple[int, int]]:
    tail_positions: List[int] = identify_tail_calls(instrs, call_op)
    result: List[Tuple[int, int]] = []
    for i in range(len(instrs)):
        found: bool = False
        for tp in tail_positions:
            if tp == i:
                found = True
        if found:
            result.append((jump_op, instrs[i][1]))
        else:
            result.append(instrs[i])
    return result

def count_tail_calls(instrs: List[Tuple[int, int]], call_op: int) -> int:
    return len(identify_tail_calls(instrs, call_op))

def tail_call_ratio(instrs: List[Tuple[int, int]], call_op: int) -> float:
    total_calls: int = 0
    for instr in instrs:
        if instr[0] == call_op:
            total_calls = total_calls + 1
    if total_calls == 0:
        return 0.0
    tail_calls: int = count_tail_calls(instrs, call_op)
    return float(tail_calls) / float(total_calls)
