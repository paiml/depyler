from typing import List, Tuple

def gset_add(gset: List[int], elem: int) -> List[int]:
    for e in gset:
        if e == elem:
            result: List[int] = []
            for x in gset:
                result.append(x)
            return result
    result2: List[int] = []
    for e in gset:
        result2.append(e)
    result2.append(elem)
    return result2

def gset_contains(gset: List[int], elem: int) -> bool:
    for e in gset:
        if e == elem:
            return True
    return False

def gset_merge(a: List[int], b: List[int]) -> List[int]:
    result: List[int] = []
    for e in a:
        result.append(e)
    for e in b:
        found: bool = False
        for r in result:
            if r == e:
                found = True
        if not found:
            result.append(e)
    return result

def gset_size(gset: List[int]) -> int:
    return len(gset)

def gset_elements(add_set: List[int], rem_set: List[int]) -> List[int]:
    result: List[int] = []
    for e in add_set:
        removed: bool = False
        for r in rem_set:
            if r == e:
                removed = True
        if not removed:
            result.append(e)
    return result
