from typing import List, Tuple

def resolve_in_env(env: List[Tuple[int, int]], name: int) -> int:
    i: int = len(env) - 1
    while i >= 0:
        if env[i][0] == name:
            return env[i][1]
        i = i - 1
    return -1

def bind(env: List[Tuple[int, int]], name: int, value: int) -> List[Tuple[int, int]]:
    result: List[Tuple[int, int]] = []
    for e in env:
        result.append(e)
    result.append((name, value))
    return result

def resolve_all(names: List[int], env: List[Tuple[int, int]]) -> List[int]:
    result: List[int] = []
    for n in names:
        found: int = -1
        i: int = len(env) - 1
        while i >= 0:
            if env[i][0] == n:
                found = env[i][1]
                break
            i = i - 1
        result.append(found)
    return result

def free_variables(body_refs: List[int], params: List[int]) -> List[int]:
    free: List[int] = []
    for ref in body_refs:
        is_param: bool = False
        for p in params:
            if p == ref:
                is_param = True
        if not is_param:
            found: bool = False
            for f in free:
                if f == ref:
                    found = True
            if not found:
                free.append(ref)
    return free

def env_size(env: List[Tuple[int, int]]) -> int:
    return len(env)
