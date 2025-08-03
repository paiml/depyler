# Test collections module imports
from collections import defaultdict, Counter, deque
from typing import List, Dict

def count_words(text: str) -> Dict[str, int]:
    """Count word frequencies using Counter"""
    words = text.lower().split()
    return dict(Counter(words))

def group_by_length(words: List[str]) -> Dict[int, List[str]]:
    """Group words by their length using defaultdict"""
    groups = defaultdict(list)
    for word in words:
        groups[len(word)].append(word)
    return dict(groups)

def process_queue(items: List[int]) -> List[int]:
    """Process items using a deque"""
    queue = deque(items)
    results = []
    
    while queue:
        if len(queue) % 2 == 0:
            results.append(queue.popleft())
        else:
            results.append(queue.pop())
    
    return results

def sliding_window(data: List[int], window_size: int) -> List[List[int]]:
    """Create sliding windows using deque"""
    if window_size > len(data):
        return []
    
    window = deque(data[:window_size], maxlen=window_size)
    windows = [list(window)]
    
    for item in data[window_size:]:
        window.append(item)
        windows.append(list(window))
    
    return windows