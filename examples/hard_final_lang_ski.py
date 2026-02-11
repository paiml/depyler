"""SKI combinator calculus evaluator.

S, K, I combinators encoded as integers:
  0 = S, 1 = K, 2 = I
  Application encoded as: [3, left_len, right_len, left..., right...]

Reduction rules:
  I x -> x
  K x y -> x
  S x y z -> x z (y z)
"""


def make_s() -> list[int]:
    """Create S combinator."""
    return [0]


def make_k() -> list[int]:
    """Create K combinator."""
    return [1]


def make_i() -> list[int]:
    """Create I combinator."""
    return [2]


def make_ski_app(left: list[int], right: list[int]) -> list[int]:
    """Create application node."""
    result: list[int] = [3, len(left), len(right)]
    i: int = 0
    while i < len(left):
        lv: int = left[i]
        result.append(lv)
        i = i + 1
    j: int = 0
    while j < len(right):
        rv: int = right[j]
        result.append(rv)
        j = j + 1
    return result


def ski_tag(term: list[int]) -> int:
    """Get tag: 0=S, 1=K, 2=I, 3=App."""
    return term[0]


def ski_left(term: list[int]) -> list[int]:
    """Extract left subtree of application."""
    llen: int = term[1]
    result: list[int] = []
    i: int = 0
    while i < llen:
        rv: int = term[3 + i]
        result.append(rv)
        i = i + 1
    return result


def ski_right(term: list[int]) -> list[int]:
    """Extract right subtree of application."""
    llen: int = term[1]
    rlen: int = term[2]
    result: list[int] = []
    i: int = 0
    while i < rlen:
        rv: int = term[3 + llen + i]
        result.append(rv)
        i = i + 1
    return result


def ski_size(term: list[int]) -> int:
    """Count nodes in term."""
    tag: int = ski_tag(term)
    if tag < 3:
        return 1
    left: list[int] = ski_left(term)
    right: list[int] = ski_right(term)
    return 1 + ski_size(left) + ski_size(right)


def ski_equal(a: list[int], b: list[int]) -> int:
    """Check if two terms are structurally equal."""
    if len(a) != len(b):
        return 0
    i: int = 0
    while i < len(a):
        va: int = a[i]
        vb: int = b[i]
        if va != vb:
            return 0
        i = i + 1
    return 1


def ski_reduce_once(term: list[int]) -> list[int]:
    """One step reduction. Returns term unchanged if no reduction possible."""
    tag: int = ski_tag(term)
    if tag < 3:
        return term
    left: list[int] = ski_left(term)
    right: list[int] = ski_right(term)
    lt: int = ski_tag(left)
    if lt == 2:
        return right
    if lt == 3:
        ll: list[int] = ski_left(left)
        lr: list[int] = ski_right(left)
        llt: int = ski_tag(ll)
        if llt == 1:
            return lr
    return term


def test_module() -> int:
    """Test SKI combinator calculus."""
    ok: int = 0
    s_term: list[int] = make_s()
    k_term: list[int] = make_k()
    i_term: list[int] = make_i()
    if ski_tag(s_term) == 0:
        ok = ok + 1
    if ski_tag(k_term) == 1:
        ok = ok + 1
    ix: list[int] = make_ski_app(i_term, k_term)
    reduced: list[int] = ski_reduce_once(ix)
    if ski_equal(reduced, k_term) == 1:
        ok = ok + 1
    kxy: list[int] = make_ski_app(make_ski_app(k_term, s_term), i_term)
    red2: list[int] = ski_reduce_once(kxy)
    if ski_equal(red2, s_term) == 1:
        ok = ok + 1
    sz: int = ski_size(kxy)
    if sz == 5:
        ok = ok + 1
    return ok
