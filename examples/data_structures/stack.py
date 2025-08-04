# @depyler: thread_safety = "required"
# @depyler: bounds_checking = "explicit"
from typing import List, Optional

class Stack:
    """Thread-safe stack implementation"""
    
    def __init__(self) -> None:
        self._items: List[int] = []
    
    def push(self, item: int) -> None:
        """Push item onto stack"""
        self._items.append(item)
    
    def pop(self) -> Optional[int]:
        """Pop item from stack, return None if empty"""
        if self.is_empty():
            return None
        return self._items.pop()
    
    def peek(self) -> Optional[int]:
        """Look at top item without removing it"""
        if self.is_empty():
            return None
        return self._items[-1]
    
    def is_empty(self) -> bool:
        """Check if stack is empty"""
        return len(self._items) == 0
    
    def size(self) -> int:
        """Get number of items in stack"""
        return len(self._items)

def balanced_parentheses(expression: str) -> bool:
    """Check if parentheses are balanced using a stack"""
    stack = Stack()
    opening = "({["
    closing = ")}]"
    pairs = {"(": ")", "{": "}", "[": "]"}
    
    for char in expression:
        if char in opening:
            stack.push(ord(char))  # Store ASCII value
        elif char in closing:
            if stack.is_empty():
                return False
            
            last = stack.pop()
            if last is None:
                return False
            
            expected = ord(pairs[chr(last)])
            if ord(char) != expected:
                return False
    
    return stack.is_empty()