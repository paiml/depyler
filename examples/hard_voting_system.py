"""Voting algorithms: plurality, Borda count, and Condorcet winner detection."""


def plurality_winner(votes: list[int], num_candidates: int) -> int:
    """Find the candidate with the most votes (plurality voting).
    Returns candidate index or -1 if tie."""
    counts: list[int] = []
    i: int = 0
    while i < num_candidates:
        counts.append(0)
        i = i + 1
    j: int = 0
    while j < len(votes):
        candidate: int = votes[j]
        if candidate >= 0 and candidate < num_candidates:
            counts[candidate] = counts[candidate] + 1
        j = j + 1
    max_count: int = 0
    winner: int = -1
    tie: int = 0
    k: int = 0
    while k < num_candidates:
        if counts[k] > max_count:
            max_count = counts[k]
            winner = k
            tie = 0
        elif counts[k] == max_count and max_count > 0:
            tie = 1
        k = k + 1
    if tie == 1:
        return -1
    return winner


def borda_count(rankings: list[int], num_voters: int, num_candidates: int) -> int:
    """Borda count voting. Rankings is a flat array of voter preferences.
    rankings[voter * num_candidates + rank] = candidate at that rank for that voter.
    Returns the winner candidate index."""
    scores: list[int] = []
    i: int = 0
    while i < num_candidates:
        scores.append(0)
        i = i + 1
    v: int = 0
    while v < num_voters:
        r: int = 0
        while r < num_candidates:
            idx: int = v * num_candidates + r
            candidate: int = rankings[idx]
            points: int = num_candidates - 1 - r
            if candidate >= 0 and candidate < num_candidates:
                scores[candidate] = scores[candidate] + points
            r = r + 1
        v = v + 1
    best_score: int = -1
    best_candidate: int = 0
    c: int = 0
    while c < num_candidates:
        if scores[c] > best_score:
            best_score = scores[c]
            best_candidate = c
        c = c + 1
    return best_candidate


def approval_voting(approvals: list[int], num_voters: int, num_candidates: int) -> int:
    """Approval voting: each voter approves multiple candidates.
    approvals[voter * num_candidates + candidate] = 1 if approved.
    Returns winner candidate index."""
    scores: list[int] = []
    i: int = 0
    while i < num_candidates:
        scores.append(0)
        i = i + 1
    v: int = 0
    while v < num_voters:
        c: int = 0
        while c < num_candidates:
            idx: int = v * num_candidates + c
            if approvals[idx] == 1:
                scores[c] = scores[c] + 1
            c = c + 1
        v = v + 1
    best: int = -1
    winner: int = 0
    j: int = 0
    while j < num_candidates:
        if scores[j] > best:
            best = scores[j]
            winner = j
        j = j + 1
    return winner


def test_module() -> int:
    """Test voting system functions."""
    ok: int = 0

    votes: list[int] = [0, 1, 0, 2, 0, 1, 1]
    if plurality_winner(votes, 3) == 0:
        ok = ok + 1

    tie_votes: list[int] = [0, 1, 0, 1]
    if plurality_winner(tie_votes, 2) == -1:
        ok = ok + 1

    # 2 voters, 3 candidates: voter0 ranks [0,1,2], voter1 ranks [1,0,2]
    rankings: list[int] = [0, 1, 2, 1, 0, 2]
    winner: int = borda_count(rankings, 2, 3)
    if winner == 0 or winner == 1:
        ok = ok + 1

    # 3 voters, 2 candidates: approvals
    approvals: list[int] = [1, 0, 1, 1, 0, 1]
    if approval_voting(approvals, 3, 2) == 0:
        ok = ok + 1

    single_vote: list[int] = [2]
    if plurality_winner(single_vote, 3) == 2:
        ok = ok + 1

    empty_votes: list[int] = []
    if plurality_winner(empty_votes, 3) == -1:
        ok = ok + 1

    return ok
