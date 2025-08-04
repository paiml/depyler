# @depyler: ownership = "owned"
# @depyler: memory_strategy = "arena"
from typing import List, Optional

class Queue:
    """Simple queue implementation with arena allocation"""
    
    def __init__(self) -> None:
        self._items: List[int] = []
        self._front: int = 0
    
    def enqueue(self, item: int) -> None:
        """Add item to back of queue"""
        self._items.append(item)
    
    def dequeue(self) -> Optional[int]:
        """Remove item from front of queue"""
        if self.is_empty():
            return None
        
        item = self._items[self._front]
        self._front += 1
        
        # Clean up if queue is mostly empty
        if self._front > len(self._items) // 2:
            self._items = self._items[self._front:]
            self._front = 0
        
        return item
    
    def front(self) -> Optional[int]:
        """Look at front item without removing it"""
        if self.is_empty():
            return None
        return self._items[self._front]
    
    def is_empty(self) -> bool:
        """Check if queue is empty"""
        return self._front >= len(self._items)
    
    def size(self) -> int:
        """Get number of items in queue"""
        return len(self._items) - self._front

def level_order_values(tree_values: List[Optional[int]]) -> List[int]:
    """Process binary tree level by level using queue"""
    if not tree_values or tree_values[0] is None:
        return []
    
    queue = Queue()
    result: List[int] = []
    
    # Start with root
    queue.enqueue(0)  # Store indices
    
    while not queue.is_empty():
        index = queue.dequeue()
        if index is None or index >= len(tree_values):
            continue
            
        value = tree_values[index]
        if value is not None:
            result.append(value)
            
            # Add children indices
            left_child = 2 * index + 1
            right_child = 2 * index + 2
            
            if left_child < len(tree_values):
                queue.enqueue(left_child)
            if right_child < len(tree_values):
                queue.enqueue(right_child)
    
    return result