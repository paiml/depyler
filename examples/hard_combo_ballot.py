"""Ballot problem and Catalan number computations."""


def choose(n: int, r: int) -> int:
    """Binomial coefficient C(n,r)."""
    if r < 0 or r > n:
        return 0
    if r == 0 or r == n:
        return 1
    if r > n - r:
        r = n - r
    result: int = 1
    i: int = 0
    while i < r:
        result = result * (n - i)
        result = result // (i + 1)
        i = i + 1
    return result


def catalan(n: int) -> int:
    """Nth Catalan number: C(2n,n)/(n+1)."""
    if n <= 0:
        return 1
    return choose(2 * n, n) // (n + 1)


def ballot_sequences(n: int) -> int:
    """Number of valid ballot sequences of length 2n (Catalan number).
    A wins with n+k votes, B with n-k votes where A always leads."""
    return catalan(n)


def ballot_probability_num(a_votes: int, b_votes: int) -> int:
    """Bertrand's ballot: probability numerator that A strictly leads.
    P = (a-b)/(a+b). Returns numerator = a-b."""
    if a_votes <= b_votes:
        return 0
    return a_votes - b_votes


def ballot_probability_den(a_votes: int, b_votes: int) -> int:
    """Bertrand's ballot: probability denominator = a+b."""
    return a_votes + b_votes


def dyck_paths(n: int) -> int:
    """Number of Dyck paths of length 2n (same as Catalan)."""
    return catalan(n)


def monotone_lattice_paths(m: int, n: int) -> int:
    """Count monotone lattice paths from (0,0) to (m,n) = C(m+n, m)."""
    return choose(m + n, m)


def test_module() -> int:
    """Test ballot and Catalan functions."""
    ok: int = 0
    if catalan(0) == 1:
        ok = ok + 1
    if catalan(3) == 5:
        ok = ok + 1
    if catalan(4) == 14:
        ok = ok + 1
    if ballot_probability_num(5, 3) == 2:
        ok = ok + 1
    if monotone_lattice_paths(2, 2) == 6:
        ok = ok + 1
    return ok
