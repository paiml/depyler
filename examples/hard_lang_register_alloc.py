from typing import List, Tuple

def color_graph(adj: List[List[int]], num_colors: int) -> List[int]:
    n: int = len(adj)
    colors: List[int] = [-1] * n
    for node in range(n):
        used: List[int] = [0] * num_colors
        for neighbor in adj[node]:
            if colors[neighbor] >= 0:
                used[colors[neighbor]] = 1
        for c in range(num_colors):
            if used[c] == 0:
                colors[node] = c
                break
    return colors

def build_interference(live_sets: List[List[int]], num_vars: int) -> List[List[int]]:
    adj: List[List[int]] = []
    for i in range(num_vars):
        adj.append([])
    for live_set in live_sets:
        for i in range(len(live_set)):
            for j in range(i + 1, len(live_set)):
                a: int = live_set[i]
                b: int = live_set[j]
                found_ab: bool = False
                for n in adj[a]:
                    if n == b:
                        found_ab = True
                if not found_ab:
                    adj[a].append(b)
                    adj[b].append(a)
    return adj

def spill_cost(var: int, uses: List[int]) -> int:
    count: int = 0
    for u in uses:
        if u == var:
            count = count + 1
    return count

def needs_spill(colors: List[int]) -> List[int]:
    spilled: List[int] = []
    for i in range(len(colors)):
        if colors[i] < 0:
            spilled.append(i)
    return spilled

def allocation_quality(colors: List[int], num_regs: int) -> float:
    total: int = len(colors)
    allocated: int = 0
    for c in colors:
        if c >= 0 and c < num_regs:
            allocated = allocated + 1
    if total == 0:
        return 0.0
    return float(allocated) / float(total)
