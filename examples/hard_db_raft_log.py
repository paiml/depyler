from typing import List, Tuple

def append_entry(log: List[Tuple[int, int, int]], term: int, index: int, command: int) -> List[Tuple[int, int, int]]:
    result: List[Tuple[int, int, int]] = []
    for e in log:
        result.append(e)
    result.append((term, index, command))
    return result

def last_log_index(log: List[Tuple[int, int, int]]) -> int:
    if len(log) == 0:
        return 0
    return log[len(log) - 1][1]

def last_log_term(log: List[Tuple[int, int, int]]) -> int:
    if len(log) == 0:
        return 0
    return log[len(log) - 1][0]

def is_log_up_to_date(log: List[Tuple[int, int, int]], candidate_term: int, candidate_index: int) -> bool:
    my_term: int = last_log_term(log)
    my_index: int = last_log_index(log)
    if candidate_term > my_term:
        return True
    if candidate_term == my_term and candidate_index >= my_index:
        return True
    return False

def truncate_log(log: List[Tuple[int, int, int]], from_index: int) -> List[Tuple[int, int, int]]:
    result: List[Tuple[int, int, int]] = []
    for e in log:
        if e[1] < from_index:
            result.append(e)
    return result

def majority(total: int) -> int:
    return total // 2 + 1
