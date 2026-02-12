from typing import List, Tuple

def defunctionalize(closures: List[List[int]], apply_table: List[int]) -> List[int]:
    tags: List[int] = []
    for closure in closures:
        if len(closure) > 0:
            tags.append(closure[0])
    return tags

def apply_defunc(tag: int, env: List[int], arg: int, table: List[List[int]]) -> int:
    if tag < len(table):
        rule: List[int] = table[tag]
        if len(rule) > 0:
            result: int = rule[0]
            for e in env:
                result = result + e
            return result + arg
    return 0

def make_tag(func_id: int) -> int:
    return func_id * 100

def tag_dispatch(tag: int, args: List[int]) -> int:
    base: int = tag // 100
    total: int = base
    for a in args:
        total = total + a
    return total

def convert_closures(funcs: List[Tuple[int, List[int]]]) -> List[Tuple[int, int]]:
    result: List[Tuple[int, int]] = []
    for func in funcs:
        tag: int = make_tag(func[0])
        env_size: int = len(func[1])
        result.append((tag, env_size))
    return result

def count_dispatch_cases(tags: List[int]) -> int:
    unique: List[int] = []
    for t in tags:
        found: bool = False
        for u in unique:
            if u == t:
                found = True
        if not found:
            unique.append(t)
    return len(unique)
