from typing import List, Tuple

def make_num_node(value: int) -> List[int]:
    return [1, value, 0, 0]

def make_binop_node(op: int, left_idx: int, right_idx: int) -> List[int]:
    return [2, op, left_idx, right_idx]

def make_unary_node(op: int, child_idx: int) -> List[int]:
    return [3, op, child_idx, 0]

def build_ast(tokens: List[int]) -> List[List[int]]:
    nodes: List[List[int]] = []
    i: int = 0
    while i < len(tokens):
        if tokens[i] >= 0 and tokens[i] <= 9:
            nodes.append(make_num_node(tokens[i]))
        i = i + 1
    if len(nodes) >= 2:
        combined: List[int] = make_binop_node(43, 0, 1)
        nodes.append(combined)
    return nodes

def eval_node(nodes: List[List[int]], idx: int) -> int:
    if idx < 0 or idx >= len(nodes):
        return 0
    node: List[int] = nodes[idx]
    if node[0] == 1:
        return node[1]
    if node[0] == 2:
        left: int = eval_node(nodes, node[2])
        right: int = eval_node(nodes, node[3])
        if node[1] == 43:
            return left + right
        if node[1] == 45:
            return left - right
        if node[1] == 42:
            return left * right
        if node[1] == 47 and right != 0:
            return left // right
        return 0
    if node[0] == 3:
        child: int = eval_node(nodes, node[2])
        if node[1] == 45:
            return 0 - child
        return child
    return 0

def tree_depth(nodes: List[List[int]], idx: int) -> int:
    if idx < 0 or idx >= len(nodes):
        return 0
    node: List[int] = nodes[idx]
    if node[0] == 1:
        return 1
    if node[0] == 2:
        ld: int = tree_depth(nodes, node[2])
        rd: int = tree_depth(nodes, node[3])
        if ld > rd:
            return ld + 1
        return rd + 1
    return tree_depth(nodes, node[2]) + 1

def count_nodes(nodes: List[List[int]]) -> int:
    return len(nodes)
