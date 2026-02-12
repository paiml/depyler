from typing import List, Tuple

def compute_def(instrs: List[Tuple[int, int, int]]) -> List[int]:
    defs: List[int] = []
    for instr in instrs:
        defs.append(instr[1])
    return defs

def compute_use(instrs: List[Tuple[int, int, int]]) -> List[int]:
    uses: List[int] = []
    for instr in instrs:
        uses.append(instr[2])
    return uses

def liveness_step(live_out: List[int], def_var: int, use_var: int) -> List[int]:
    result: List[int] = []
    for v in live_out:
        if v != def_var:
            result.append(v)
    found: bool = False
    for v in result:
        if v == use_var:
            found = True
    if not found:
        result.append(use_var)
    return result

def compute_liveness(instrs: List[Tuple[int, int, int]], num_vars: int) -> List[List[int]]:
    n: int = len(instrs)
    live_in: List[List[int]] = []
    live_out: List[List[int]] = []
    for i in range(n):
        live_in.append([])
        live_out.append([])
    changed: bool = True
    while changed:
        changed = False
        i: int = n - 1
        while i >= 0:
            old_size: int = len(live_in[i])
            if i + 1 < n:
                for v in live_in[i + 1]:
                    found: bool = False
                    for o in live_out[i]:
                        if o == v:
                            found = True
                    if not found:
                        live_out[i].append(v)
            live_in[i] = liveness_step(live_out[i], instrs[i][1], instrs[i][2])
            if len(live_in[i]) != old_size:
                changed = True
            i = i - 1
    return live_in

def interference(live_sets: List[List[int]]) -> List[Tuple[int, int]]:
    edges: List[Tuple[int, int]] = []
    for live_set in live_sets:
        for i in range(len(live_set)):
            for j in range(i + 1, len(live_set)):
                a: int = live_set[i]
                b: int = live_set[j]
                found: bool = False
                for e in edges:
                    if (e[0] == a and e[1] == b) or (e[0] == b and e[1] == a):
                        found = True
                if not found:
                    edges.append((a, b))
    return edges
