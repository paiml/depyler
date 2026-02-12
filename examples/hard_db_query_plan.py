from typing import List, Tuple

def estimate_cost(rows: int, pages: int) -> float:
    return float(pages) + 0.01 * float(rows)

def selectivity(total: int, matching: int) -> float:
    if total == 0:
        return 0.0
    return float(matching) / float(total)

def choose_index(costs: List[float]) -> int:
    best: int = 0
    for i in range(1, len(costs)):
        if costs[i] < costs[best]:
            best = i
    return best

def plan_join_order(tables: List[int], costs: List[List[float]]) -> List[int]:
    n: int = len(tables)
    order: List[int] = []
    used: List[int] = [0] * n
    for step in range(n):
        best: int = -1
        best_cost: float = 999999999.0
        for i in range(n):
            if used[i] == 0:
                total: float = 0.0
                for j in range(n):
                    if used[j] == 1:
                        total = total + costs[i][j]
                if best == -1 or total < best_cost:
                    best_cost = total
                    best = i
        if best >= 0:
            order.append(tables[best])
            used[best] = 1
    return order

def total_plan_cost(plan: List[int], costs: List[float]) -> float:
    total: float = 0.0
    for i in range(len(plan)):
        if i < len(costs):
            total = total + costs[i]
    return total
