from typing import List, Tuple

def gossip_merge(state_a: List[int], state_b: List[int]) -> List[int]:
    result: List[int] = []
    for i in range(len(state_a)):
        if i < len(state_b):
            if state_a[i] > state_b[i]:
                result.append(state_a[i])
            else:
                result.append(state_b[i])
        else:
            result.append(state_a[i])
    return result

def gossip_update(state: List[int], node_id: int) -> List[int]:
    result: List[int] = []
    for s in state:
        result.append(s)
    if node_id < len(result):
        result[node_id] = result[node_id] + 1
    return result

def select_peer(node_id: int, num_nodes: int, seed: int) -> int:
    peer: int = (seed * 1103515245 + 12345) % num_nodes
    if peer == node_id:
        peer = (peer + 1) % num_nodes
    return peer

def gossip_round(states: List[List[int]], seed: int) -> List[List[int]]:
    n: int = len(states)
    result: List[List[int]] = []
    for s in states:
        ns: List[int] = []
        for v in s:
            ns.append(v)
        result.append(ns)
    for i in range(n):
        peer: int = select_peer(i, n, seed + i)
        result[i] = gossip_merge(result[i], states[peer])
    return result

def convergence_check(states: List[List[int]]) -> bool:
    if len(states) < 2:
        return True
    for i in range(1, len(states)):
        for j in range(len(states[0])):
            if j < len(states[i]) and states[0][j] != states[i][j]:
                return False
    return True
