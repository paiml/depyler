from typing import List, Tuple

def create_heap(size: int) -> List[int]:
    heap: List[int] = []
    for i in range(size):
        heap.append(0)
    return heap

def allocate(heap: List[int], size: int) -> Tuple[int, List[int]]:
    new_heap: List[int] = []
    for h in heap:
        new_heap.append(h)
    start: int = len(new_heap)
    for i in range(size):
        new_heap.append(0)
    return (start, new_heap)

def set_ref(heap: List[int], obj: int, target: int) -> List[int]:
    new_heap: List[int] = []
    for h in heap:
        new_heap.append(h)
    if obj < len(new_heap):
        new_heap[obj] = target
    return new_heap

def mark(heap: List[int], roots: List[int], obj_size: int) -> List[int]:
    marked: List[int] = [0] * (len(heap) // obj_size + 1)
    worklist: List[int] = []
    for r in roots:
        worklist.append(r)
    while len(worklist) > 0:
        obj: int = worklist[len(worklist) - 1]
        worklist = worklist[0:len(worklist) - 1]
        idx: int = obj // obj_size
        if idx < len(marked) and marked[idx] == 0:
            marked[idx] = 1
            for i in range(obj_size):
                if obj + i < len(heap):
                    ref: int = heap[obj + i]
                    if ref > 0:
                        worklist.append(ref)
    return marked

def count_live(marked: List[int]) -> int:
    count: int = 0
    for m in marked:
        if m == 1:
            count = count + 1
    return count

def count_garbage(marked: List[int], total: int) -> int:
    return total - count_live(marked)
