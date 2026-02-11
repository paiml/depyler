def total_pages(total_items: int, page_size: int) -> int:
    if page_size <= 0:
        return 0
    pages: int = total_items // page_size
    rem: int = total_items % page_size
    if rem > 0:
        pages = pages + 1
    return pages

def offset_for_page(page_num: int, page_size: int) -> int:
    return (page_num - 1) * page_size

def get_page(items: list[int], page_num: int, page_size: int) -> list[int]:
    start: int = offset_for_page(page_num, page_size)
    n: int = len(items)
    result: list[int] = []
    i: int = start
    end: int = start + page_size
    if end > n:
        end = n
    while i < end:
        result.append(items[i])
        i = i + 1
    return result

def has_next_page(page_num: int, page_size: int, total_items: int) -> int:
    tp: int = total_pages(total_items, page_size)
    if page_num < tp:
        return 1
    return 0

def has_prev_page(page_num: int) -> int:
    if page_num > 1:
        return 1
    return 0

def page_range(current: int, total: int, window: int) -> list[int]:
    start: int = current - window
    if start < 1:
        start = 1
    end: int = current + window
    if end > total:
        end = total
    result: list[int] = []
    i: int = start
    while i <= end:
        result.append(i)
        i = i + 1
    return result

def cursor_slice(items: list[int], cursor: int, limit: int) -> list[int]:
    n: int = len(items)
    result: list[int] = []
    i: int = cursor
    end: int = cursor + limit
    if end > n:
        end = n
    while i < end:
        result.append(items[i])
        i = i + 1
    return result

def test_module() -> int:
    passed: int = 0
    tp: int = total_pages(25, 10)
    if tp == 3:
        passed = passed + 1
    off: int = offset_for_page(2, 10)
    if off == 10:
        passed = passed + 1
    items: list[int] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    pg: list[int] = get_page(items, 1, 3)
    npg: int = len(pg)
    if npg == 3:
        passed = passed + 1
    hn: int = has_next_page(1, 3, 10)
    if hn == 1:
        passed = passed + 1
    hp: int = has_prev_page(1)
    if hp == 0:
        passed = passed + 1
    return passed
