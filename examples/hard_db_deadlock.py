from typing import List, Tuple

def build_wait_graph(waits: List[Tuple[int, int]], n: int) -> List[List[int]]:
    graph: List[List[int]] = []
    for i in range(n):
        graph.append([])
    for w in waits:
        if w[0] < n:
            graph[w[0]].append(w[1])
    return graph

def detect_deadlock(graph: List[List[int]]) -> bool:
    n: int = len(graph)
    visited: List[int] = [0] * n
    in_stack: List[int] = [0] * n
    for i in range(n):
        if visited[i] == 0:
            stack: List[int] = [i]
            while len(stack) > 0:
                curr: int = stack[len(stack) - 1]
                if visited[curr] == 0:
                    visited[curr] = 1
                    in_stack[curr] = 1
                found_next: bool = False
                for nb in graph[curr]:
                    if in_stack[nb] == 1:
                        return True
                    if visited[nb] == 0:
                        stack.append(nb)
                        found_next = True
                        break
                if not found_next:
                    in_stack[curr] = 0
                    stack = stack[0:len(stack) - 1]
    return False

def find_victim(waits: List[Tuple[int, int]], priorities: List[int]) -> int:
    best: int = -1
    best_pri: int = 999999
    for w in waits:
        if w[0] < len(priorities) and priorities[w[0]] < best_pri:
            best_pri = priorities[w[0]]
            best = w[0]
    return best

def abort_txn(waits: List[Tuple[int, int]], txn_id: int) -> List[Tuple[int, int]]:
    result: List[Tuple[int, int]] = []
    for w in waits:
        if w[0] != txn_id and w[1] != txn_id:
            result.append(w)
    return result

def wait_count(waits: List[Tuple[int, int]], txn_id: int) -> int:
    count: int = 0
    for w in waits:
        if w[0] == txn_id:
            count = count + 1
    return count
