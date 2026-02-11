"""Simple type checker for a tiny typed language.

Types: 0=Int, 1=Bool, 2=Fun(arg_type, ret_type), -1=Error.
Expressions encoded as tagged integers:
  [0, value] = IntLit
  [1, 0/1] = BoolLit
  [2, expr_len, expr...] = Negate
  [3, left_len, right_len, left..., right...] = Add
  [4, cond_len, then_len, else_len, cond..., then..., else...] = IfExpr
  [5, var_index] = Variable reference
"""


def type_int() -> int:
    """Integer type."""
    return 0


def type_bool() -> int:
    """Boolean type."""
    return 1


def type_error() -> int:
    """Type error."""
    return 0 - 1


def check_intlit(expr: list[int]) -> int:
    """Type check integer literal."""
    return type_int()


def check_boollit(expr: list[int]) -> int:
    """Type check boolean literal."""
    return type_bool()


def check_negate(expr: list[int], env: list[int]) -> int:
    """Type check negation: operand must be Int."""
    elen: int = expr[1]
    sub: list[int] = []
    i: int = 0
    while i < elen:
        sv: int = expr[2 + i]
        sub.append(sv)
        i = i + 1
    sub_type: int = typecheck(sub, env)
    if sub_type == type_int():
        return type_int()
    return type_error()


def check_add(expr: list[int], env: list[int]) -> int:
    """Type check addition: both operands must be Int."""
    llen: int = expr[1]
    rlen: int = expr[2]
    left: list[int] = []
    i: int = 0
    while i < llen:
        lv: int = expr[3 + i]
        left.append(lv)
        i = i + 1
    right: list[int] = []
    j: int = 0
    while j < rlen:
        rv: int = expr[3 + llen + j]
        right.append(rv)
        j = j + 1
    lt: int = typecheck(left, env)
    rt: int = typecheck(right, env)
    if lt == type_int():
        if rt == type_int():
            return type_int()
    return type_error()


def check_if(expr: list[int], env: list[int]) -> int:
    """Type check if: condition must be Bool, branches must match."""
    clen: int = expr[1]
    tlen: int = expr[2]
    elen: int = expr[3]
    cond: list[int] = []
    i: int = 0
    while i < clen:
        cv: int = expr[4 + i]
        cond.append(cv)
        i = i + 1
    then_br: list[int] = []
    j: int = 0
    while j < tlen:
        tv: int = expr[4 + clen + j]
        then_br.append(tv)
        j = j + 1
    else_br: list[int] = []
    k: int = 0
    while k < elen:
        ev: int = expr[4 + clen + tlen + k]
        else_br.append(ev)
        k = k + 1
    ct: int = typecheck(cond, env)
    if ct != type_bool():
        return type_error()
    tt: int = typecheck(then_br, env)
    et: int = typecheck(else_br, env)
    if tt == et:
        return tt
    return type_error()


def check_var(expr: list[int], env: list[int]) -> int:
    """Type check variable reference."""
    idx: int = expr[1]
    if idx < len(env):
        return env[idx]
    return type_error()


def typecheck(expr: list[int], env: list[int]) -> int:
    """Main type checking dispatch."""
    if len(expr) == 0:
        return type_error()
    tag: int = expr[0]
    if tag == 0:
        r0: int = check_intlit(expr)
        return r0
    if tag == 1:
        r1: int = check_boollit(expr)
        return r1
    if tag == 2:
        r2: int = check_negate(expr, env)
        return r2
    if tag == 3:
        r3: int = check_add(expr, env)
        return r3
    if tag == 4:
        r4: int = check_if(expr, env)
        return r4
    if tag == 5:
        r5: int = check_var(expr, env)
        return r5
    return type_error()


def test_module() -> int:
    """Test type checker."""
    ok: int = 0
    env: list[int] = [0, 1]
    int_lit: list[int] = [0, 42]
    tc_int: int = typecheck(int_lit, env)
    if tc_int == 0:
        ok = ok + 1
    bool_lit: list[int] = [1, 1]
    tc_bool: int = typecheck(bool_lit, env)
    if tc_bool == 1:
        ok = ok + 1
    add_expr: list[int] = [3, 2, 2, 0, 1, 0, 2]
    tc_add: int = typecheck(add_expr, env)
    if tc_add == 0:
        ok = ok + 1
    bad_add: list[int] = [3, 2, 2, 0, 1, 1, 0]
    tc_bad: int = typecheck(bad_add, env)
    if tc_bad == 0 - 1:
        ok = ok + 1
    var_expr: list[int] = [5, 0]
    tc_var: int = typecheck(var_expr, env)
    if tc_var == 0:
        ok = ok + 1
    return ok
