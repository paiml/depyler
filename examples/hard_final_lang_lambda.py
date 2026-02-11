"""Lambda calculus evaluator using De Bruijn indices.

Terms encoded as flat integer arrays:
  [0, n] = variable (De Bruijn index n)
  [1, body_start, body_end] = lambda abstraction
  [2, func_start, func_end, arg_start, arg_end] = application

Simplified: uses tagged integer encoding for small expressions.
Tag: 0=var, 1=abs, 2=app. Values stored in tree-like array.
"""


def make_var(idx: int) -> list[int]:
    """Create variable term."""
    return [0, idx]


def make_abs(body: list[int]) -> list[int]:
    """Create lambda abstraction wrapping body."""
    result: list[int] = [1, len(body)]
    i: int = 0
    while i < len(body):
        bv: int = body[i]
        result.append(bv)
        i = i + 1
    return result


def make_app(func_term: list[int], arg_term: list[int]) -> list[int]:
    """Create application term."""
    result: list[int] = [2, len(func_term), len(arg_term)]
    i: int = 0
    while i < len(func_term):
        fv: int = func_term[i]
        result.append(fv)
        i = i + 1
    j: int = 0
    while j < len(arg_term):
        av: int = arg_term[j]
        result.append(av)
        j = j + 1
    return result


def term_tag(term: list[int]) -> int:
    """Get tag of term."""
    return term[0]


def is_value(term: list[int]) -> int:
    """Check if term is a value (variable or abstraction). Returns 1 if value."""
    tag: int = term[0]
    if tag == 0:
        return 1
    if tag == 1:
        return 1
    return 0


def shift_var(term: list[int], cutoff: int, amount: int) -> list[int]:
    """Shift free variables in term by amount above cutoff."""
    tag: int = term[0]
    if tag == 0:
        idx: int = term[1]
        if idx >= cutoff:
            return [0, idx + amount]
        return [0, idx]
    if tag == 1:
        blen: int = term[1]
        body: list[int] = []
        i: int = 0
        while i < blen:
            bv: int = term[2 + i]
            body.append(bv)
            i = i + 1
        shifted_body: list[int] = shift_var(body, cutoff + 1, amount)
        return make_abs(shifted_body)
    return term


def term_size(term: list[int]) -> int:
    """Count number of nodes in term."""
    tag: int = term[0]
    if tag == 0:
        return 1
    if tag == 1:
        blen: int = term[1]
        body: list[int] = []
        i: int = 0
        while i < blen:
            bv: int = term[2 + i]
            body.append(bv)
            i = i + 1
        return 1 + term_size(body)
    if tag == 2:
        flen: int = term[1]
        alen: int = term[2]
        func_term: list[int] = []
        i2: int = 0
        while i2 < flen:
            fv: int = term[3 + i2]
            func_term.append(fv)
            i2 = i2 + 1
        arg_term: list[int] = []
        j: int = 0
        while j < alen:
            av: int = term[3 + flen + j]
            arg_term.append(av)
            j = j + 1
        return 1 + term_size(func_term) + term_size(arg_term)
    return 1


def church_numeral(n: int) -> list[int]:
    """Create Church numeral for n: lambda f. lambda x. f(f(...(x)...))."""
    innermost: list[int] = make_var(0)
    i: int = 0
    while i < n:
        innermost = make_app(make_var(1), innermost)
        i = i + 1
    return make_abs(make_abs(innermost))


def test_module() -> int:
    """Test lambda calculus."""
    ok: int = 0
    v0: list[int] = make_var(0)
    if term_tag(v0) == 0:
        ok = ok + 1
    if is_value(v0) == 1:
        ok = ok + 1
    abs_term: list[int] = make_abs(v0)
    if term_tag(abs_term) == 1:
        ok = ok + 1
    app: list[int] = make_app(abs_term, v0)
    if term_tag(app) == 2:
        ok = ok + 1
    c2: list[int] = church_numeral(2)
    sz: int = term_size(c2)
    if sz > 3:
        ok = ok + 1
    return ok
