from typing import List, Tuple

def sort_run(data: List[int]) -> List[int]:
    result: List[int] = []
    for d in data:
        result.append(d)
    n: int = len(result)
    for i in range(n):
        for j in range(i + 1, n):
            if result[j] < result[i]:
                temp: int = result[i]
                result[i] = result[j]
                result[j] = temp
    return result

def merge_two(a: List[int], b: List[int]) -> List[int]:
    result: List[int] = []
    i: int = 0
    j: int = 0
    while i < len(a) and j < len(b):
        if a[i] <= b[j]:
            result.append(a[i])
            i = i + 1
        else:
            result.append(b[j])
            j = j + 1
    while i < len(a):
        result.append(a[i])
        i = i + 1
    while j < len(b):
        result.append(b[j])
        j = j + 1
    return result

def create_runs(data: List[int], run_size: int) -> List[List[int]]:
    runs: List[List[int]] = []
    i: int = 0
    while i < len(data):
        end: int = i + run_size
        if end > len(data):
            end = len(data)
        run: List[int] = []
        for j in range(i, end):
            run.append(data[j])
        n: int = len(run)
        for a in range(n):
            for b in range(a + 1, n):
                if run[b] < run[a]:
                    temp: int = run[a]
                    run[a] = run[b]
                    run[b] = temp
        runs.append(run)
        i = end
    return runs

def external_sort(data: List[int], memory: int) -> List[int]:
    if len(data) <= memory:
        result: List[int] = []
        for d in data:
            result.append(d)
        n: int = len(result)
        for i in range(n):
            for j in range(i + 1, n):
                if result[j] < result[i]:
                    temp: int = result[i]
                    result[i] = result[j]
                    result[j] = temp
        return result
    runs: List[List[int]] = create_runs(data, memory)
    while len(runs) > 1:
        new_runs: List[List[int]] = []
        i: int = 0
        while i < len(runs):
            if i + 1 < len(runs):
                merged: List[int] = merge_two(runs[i], runs[i + 1])
                new_runs.append(merged)
                i = i + 2
            else:
                new_runs.append(runs[i])
                i = i + 1
        runs = new_runs
    return runs[0]

def is_sorted(data: List[int]) -> bool:
    for i in range(1, len(data)):
        if data[i] < data[i - 1]:
            return False
    return True
