from typing import List, Tuple, Dict

def rename_variable(var: int, version: int) -> int:
    return var * 1000 + version

def to_ssa(instrs: List[Tuple[int, int, int]], num_vars: int) -> List[Tuple[int, int, int]]:
    versions: List[int] = [0] * num_vars
    result: List[Tuple[int, int, int]] = []
    for instr in instrs:
        op: int = instr[0]
        dst: int = instr[1]
        src: int = instr[2]
        src_renamed: int = rename_variable(src, versions[src % num_vars])
        versions[dst % num_vars] = versions[dst % num_vars] + 1
        dst_renamed: int = rename_variable(dst, versions[dst % num_vars])
        result.append((op, dst_renamed, src_renamed))
    return result

def insert_phi(preds: List[List[int]], var: int, versions: List[int]) -> List[int]:
    phi: List[int] = [var]
    for v in versions:
        phi.append(rename_variable(var, v))
    return phi

def count_versions(ssa: List[Tuple[int, int, int]]) -> Dict[int, int]:
    counts: Dict[int, int] = {}
    for instr in ssa:
        base: int = instr[1] // 1000
        if base in counts:
            counts[base] = counts[base] + 1
        else:
            counts[base] = 1
    return counts

def is_ssa_form(instrs: List[Tuple[int, int, int]]) -> bool:
    defined: List[int] = []
    for instr in instrs:
        dst: int = instr[1]
        for d in defined:
            if d == dst:
                return False
        defined.append(dst)
    return True
