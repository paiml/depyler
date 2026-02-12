from typing import List, Tuple

def dropout_mask(size: int, seed: int, threshold: int) -> List[int]:
    mask: List[int] = []
    val: int = seed
    for i in range(size):
        val = ((val * 1103515245) + 12345) & 0x7FFFFFFF
        if (val % 1000) < threshold:
            mask.append(0)
        else:
            mask.append(1)
    return mask

def apply_dropout(values: List[float], mask: List[int]) -> List[float]:
    result: List[float] = []
    for i in range(len(values)):
        if mask[i] == 1:
            result.append(values[i])
        else:
            result.append(0.0)
    return result

def dropout_forward_train(values: List[float], seed: int, threshold: int) -> List[float]:
    mask: List[int] = dropout_mask(len(values), seed, threshold)
    return apply_dropout(values, mask)

def count_dropped(mask: List[int]) -> int:
    count: int = 0
    for m in mask:
        if m == 0:
            count = count + 1
    return count

def effective_rate(dropped: int, total: int) -> float:
    return float(dropped) / float(total)
