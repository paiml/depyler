from typing import List, Tuple

def sweep(heap: List[int], marked: List[int], obj_size: int) -> List[int]:
    new_heap: List[int] = []
    for h in heap:
        new_heap.append(h)
    for i in range(len(marked)):
        if marked[i] == 0:
            addr: int = i * obj_size
            for j in range(obj_size):
                if addr + j < len(new_heap):
                    new_heap[addr + j] = 0
    return new_heap

def get_free_list(marked: List[int], obj_size: int) -> List[int]:
    free_list: List[int] = []
    for i in range(len(marked)):
        if marked[i] == 0:
            free_list.append(i * obj_size)
    return free_list

def compact(heap: List[int], marked: List[int], obj_size: int) -> List[int]:
    new_heap: List[int] = []
    for i in range(len(marked)):
        if marked[i] == 1:
            addr: int = i * obj_size
            for j in range(obj_size):
                if addr + j < len(heap):
                    new_heap.append(heap[addr + j])
                else:
                    new_heap.append(0)
    return new_heap

def heap_util(marked: List[int]) -> float:
    total: int = len(marked)
    if total == 0:
        return 0.0
    live: int = 0
    for m in marked:
        if m == 1:
            live = live + 1
    return float(live) / float(total)

def fragmentation(free_list: List[int], obj_size: int) -> int:
    if len(free_list) <= 1:
        return 0
    gaps: int = 0
    for i in range(1, len(free_list)):
        if free_list[i] - free_list[i - 1] > obj_size:
            gaps = gaps + 1
    return gaps
