# Pathological class pattern: Stack and Queue with peek, size tracking
# Tests: two classes, complex pop/push sequences
# Workaround: avoid calling mutable self methods from other methods (transpiler
# generates &self instead of &mut self for methods that call other &mut methods)


class IntStack:
    def __init__(self) -> None:
        self.data: list[int] = []
        self.size: int = 0
        self.push_count: int = 0
        self.pop_count: int = 0

    def push_item(self, val: int) -> None:
        self.data.append(val)
        self.size = self.size + 1
        self.push_count = self.push_count + 1

    def pop_item(self) -> int:
        if self.size == 0:
            return 0 - 1
        self.size = self.size - 1
        self.pop_count = self.pop_count + 1
        return self.data.pop()

    def peek_top(self) -> int:
        if self.size == 0:
            return 0 - 1
        return self.data[self.size - 1]

    def is_empty(self) -> bool:
        return self.size == 0

    def get_size(self) -> int:
        return self.size


class IntQueue:
    def __init__(self) -> None:
        self.data: list[int] = []
        self.size: int = 0

    def enqueue(self, val: int) -> None:
        self.data.append(val)
        self.size = self.size + 1

    def dequeue(self) -> int:
        if self.size == 0:
            return 0 - 1
        self.size = self.size - 1
        front: int = self.data[0]
        new_data: list[int] = []
        i: int = 1
        while i < len(self.data):
            new_data.append(self.data[i])
            i = i + 1
        self.data = new_data
        return front

    def peek_front(self) -> int:
        if self.size == 0:
            return 0 - 1
        return self.data[0]


def test_module() -> int:
    passed: int = 0
    # Test stack
    st: IntStack = IntStack()
    st.push_item(10)
    st.push_item(20)
    st.push_item(30)
    # Test 1: peek
    if st.peek_top() == 30:
        passed = passed + 1
    # Test 2: pop returns LIFO
    if st.pop_item() == 30:
        passed = passed + 1
    # Test 3: size after pop
    if st.size == 2:
        passed = passed + 1
    # Test 4: pop again
    v2: int = st.pop_item()
    if v2 == 20:
        passed = passed + 1
    # Test 5: empty check
    st.pop_item()
    if st.is_empty() == True:
        passed = passed + 1
    # Test queue
    q: IntQueue = IntQueue()
    q.enqueue(100)
    q.enqueue(200)
    q.enqueue(300)
    # Test 6: peek front
    if q.peek_front() == 100:
        passed = passed + 1
    # Test 7: dequeue returns FIFO
    if q.dequeue() == 100:
        passed = passed + 1
    # Test 8: second dequeue
    if q.dequeue() == 200:
        passed = passed + 1
    return passed
