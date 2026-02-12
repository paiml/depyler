from typing import List, Tuple

def gc_encrypt(pt: int, key: int) -> int:
    return (pt ^ key) & 0xFFFFFFFF

def gc_decrypt(ct: int, key: int) -> int:
    return (ct ^ key) & 0xFFFFFFFF

def gen_wire_labels(seed: int, n: int) -> List[Tuple[int, int]]:
    labels: List[Tuple[int, int]] = []
    val: int = seed
    for i in range(n):
        val = ((val * 1103515245) + 12345) & 0x7FFFFFFF
        l0: int = val & 0xFFFFFFFF
        val = ((val * 1103515245) + 12345) & 0x7FFFFFFF
        l1: int = val & 0xFFFFFFFF
        labels.append((l0, l1))
    return labels

def garble_and(a0: int, a1: int, b0: int, b1: int, o0: int, o1: int) -> List[int]:
    table: List[int] = []
    table.append(gc_encrypt(gc_encrypt(o0, a0), b0))
    table.append(gc_encrypt(gc_encrypt(o0, a0), b1))
    table.append(gc_encrypt(gc_encrypt(o0, a1), b0))
    table.append(gc_encrypt(gc_encrypt(o1, a1), b1))
    return table

def garble_or(a0: int, a1: int, b0: int, b1: int, o0: int, o1: int) -> List[int]:
    table: List[int] = []
    table.append(gc_encrypt(gc_encrypt(o0, a0), b0))
    table.append(gc_encrypt(gc_encrypt(o1, a0), b1))
    table.append(gc_encrypt(gc_encrypt(o1, a1), b0))
    table.append(gc_encrypt(gc_encrypt(o1, a1), b1))
    return table

def garble_circuit(n_gates: int, seed: int) -> List[List[int]]:
    labels: List[Tuple[int, int]] = gen_wire_labels(seed, n_gates * 3)
    tables: List[List[int]] = []
    for i in range(n_gates):
        idx: int = i * 3
        t: List[int] = garble_and(labels[idx][0], labels[idx][1], labels[idx+1][0], labels[idx+1][1], labels[idx+2][0], labels[idx+2][1])
        tables.append(t)
    return tables

def count_gates(circuit: List[List[int]]) -> int:
    return len(circuit)
