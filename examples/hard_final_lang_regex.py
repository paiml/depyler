"""Simple regex engine supporting ., *, and literal matching.

Implements Thompson's construction approach with backtracking.
Patterns and text represented as integer arrays (character codes).
"""


def match_here(regex: list[int], ri: int, text: list[int], ti: int) -> int:
    """Match regex[ri..] against text[ti..]. Returns 1 if matches."""
    if ri >= len(regex):
        return 1
    rc: int = regex[ri]
    if ri + 1 < len(regex):
        next_rc: int = regex[ri + 1]
        if next_rc == 42:
            star_result: int = match_star(rc, regex, ri + 2, text, ti)
            return star_result
    if rc == 36:
        if ti >= len(text):
            return 1
        return 0
    if ti >= len(text):
        return 0
    tc: int = text[ti]
    if rc == 46:
        dot_result: int = match_here(regex, ri + 1, text, ti + 1)
        return dot_result
    if rc == tc:
        lit_result: int = match_here(regex, ri + 1, text, ti + 1)
        return lit_result
    return 0


def match_star(ch: int, regex: list[int], ri: int, text: list[int], ti: int) -> int:
    """Match ch* followed by regex[ri..] against text[ti..]."""
    pos: int = ti
    while pos <= len(text):
        mh_result: int = match_here(regex, ri, text, pos)
        if mh_result == 1:
            return 1
        if pos >= len(text):
            return 0
        tc: int = text[pos]
        if ch != 46:
            if tc != ch:
                return 0
        pos = pos + 1
    return 0


def regex_match(regex: list[int], text: list[int]) -> int:
    """Match regex against text. ^ anchors start. Returns 1 if match found."""
    if len(regex) == 0:
        return 1
    r0: int = regex[0]
    if r0 == 94:
        anchor_result: int = match_here(regex, 1, text, 0)
        return anchor_result
    ti: int = 0
    while ti <= len(text):
        scan_result: int = match_here(regex, 0, text, ti)
        if scan_result == 1:
            return 1
        ti = ti + 1
    return 0


def count_matches(regex: list[int], text: list[int]) -> int:
    """Count non-overlapping matches of regex in text."""
    cnt: int = 0
    ti: int = 0
    while ti <= len(text):
        cnt_mh: int = match_here(regex, 0, text, ti)
        if cnt_mh == 1:
            cnt = cnt + 1
        ti = ti + 1
    return cnt


def match_length(regex: list[int], text: list[int], start: int) -> int:
    """Find length of match starting at start. Returns 0 if no match."""
    end: int = start
    while end <= len(text):
        sub: list[int] = []
        j: int = start
        while j < end:
            sv: int = text[j]
            sub.append(sv)
            j = j + 1
        rm_result: int = regex_match(regex, sub)
        if rm_result == 1:
            mh_len: int = match_here(regex, 0, text, start)
            if mh_len == 1:
                return end - start
        end = end + 1
    return 0


def test_module() -> int:
    """Test regex engine."""
    ok: int = 0
    if regex_match([97, 98, 99], [97, 98, 99]) == 1:
        ok = ok + 1
    if regex_match([97, 46, 99], [97, 98, 99]) == 1:
        ok = ok + 1
    if regex_match([97, 98, 42], [97, 98, 98, 98]) == 1:
        ok = ok + 1
    if regex_match([94, 97], [98, 97]) == 0:
        ok = ok + 1
    if regex_match([94, 97], [97, 98]) == 1:
        ok = ok + 1
    return ok
